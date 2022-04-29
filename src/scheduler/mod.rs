use std::{collections::HashMap, error, fmt, future::Future};

use chrono::{DateTime, Duration, Utc};
use tokio::sync::watch;

use crate::error::BoxResult;

type SchedulerResult<T> = Result<T, SchedulerError>;

#[derive(Debug)]
pub enum SchedulerError {
  DuplicateTimingId(String),
  TimingNotFound(String),
}

impl fmt::Display for SchedulerError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      SchedulerError::DuplicateTimingId(id) => write!(f, "a timing already exists with the id {}", id),
      SchedulerError::TimingNotFound(id) => write!(f, "could not find a timing with the id {}", id),
    }
  }
}

impl error::Error for SchedulerError {}

#[derive(Debug)]
pub enum Timing {
  Immediate,
  DateTime(DateTime<Utc>),
  Repeating(Duration),
}

pub struct TaskHandle<T> {
  pub receiver: watch::Receiver<T>,

  handle: tokio::task::JoinHandle<T>,
}

impl<T> TaskHandle<T> {
  pub fn new(receiver: watch::Receiver<T>, handle: tokio::task::JoinHandle<T>) -> Self {
    Self { handle, receiver }
  }

  pub fn abort(self) {
    self.handle.abort();
  }
}

#[derive(Debug)]
pub struct Scheduler {
  // we can use tokio::spawn instead but i wanna abstract this further in the future
  // and this is (probably maybe) a good first step towards doing so
  runtime: tokio::runtime::Handle,

  // we currently dont clear useless timings (eg. dates in the past), this will not
  // be an issue post-poc as we will store these in a db where these are cleaned up
  timings: HashMap<String, Timing>,
}

impl Scheduler {
  pub fn new() -> BoxResult<Self> {
    let runtime = tokio::runtime::Handle::try_current()?;

    Ok(Self {
      runtime,
      timings: HashMap::new(),
    })
  }

  pub fn register_timing(&mut self, id: &str, timing: Timing) -> SchedulerResult<()> {
    if self.timings.contains_key(id) {
      return Err(SchedulerError::DuplicateTimingId(id.to_string()));
    }

    self.timings.insert(id.to_string(), timing);

    Ok(())
  }

  pub fn remove_timing(&mut self, id: &str) -> SchedulerResult<()> {
    self
      .timings
      .remove(id)
      .ok_or_else(|| SchedulerError::TimingNotFound(id.to_string()))?;

    Ok(())
  }

  // since we specify the future type here, we can return a channel which will produce *something* *eventually*
  // BUT, we can also store a type-erased future and manage its scheduling internally as we see fit
  pub fn spawn_task<T, F>(&self, id: &str, task_fn: T) -> SchedulerResult<TaskHandle<F::Output>>
  where
    T: Fn(watch::Sender<F::Output>) -> F,
    F: Future + Send + 'static,
    F::Output: Default + Send + 'static,
  {
    let timing = self
      .timings
      .get(id)
      .ok_or_else(|| SchedulerError::TimingNotFound(id.to_string()))?;

    match timing {
      Timing::Immediate => {
        let (tx, rx) = watch::channel::<F::Output>(Default::default());
        let future = task_fn(tx);

        let handle = self.runtime.spawn(future);

        let task_handle = TaskHandle::new(rx, handle);

        Ok(task_handle)
      }
      // Timing::DateTime(date_time) => {}
      // Timing::Repeating(interval_duration) => {}
      _ => todo!("complete the rest of the scheduler issues"),
    }
  }
}
