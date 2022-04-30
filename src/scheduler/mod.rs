use std::{collections::HashMap, error, fmt, future::Future, pin::Pin};

use chrono::{DateTime, Duration, Utc};
use tokio::{
  sync::{mpsc, watch},
  task,
};

use crate::error::BoxResult;

type SchedulerResult<T> = Result<T, SchedulerError>;

type BoxFuture = Pin<Box<dyn Future<Output = ()> + Send + 'static>>;
type TaskFn = Box<dyn Fn() -> BoxFuture>;

#[derive(Debug)]
pub enum SchedulerError {
  DuplicateTaskId(String),
  TaskNotFound(String),
}

impl fmt::Display for SchedulerError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      SchedulerError::DuplicateTaskId(id) => write!(f, "a task already exists with the id {}", id),
      SchedulerError::TaskNotFound(id) => write!(f, "could not find a task with the id {}", id),
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

// #[derive(Debug)]
pub struct Scheduler {
  // we can use tokio::spawn instead but i wanna abstract this further in the future
  // and this is (probably maybe) a good first step towards doing so
  runtime: tokio::runtime::Handle,

  // TODO: ...
  tasks: HashMap<String, TaskFn>,
}

// TODO: ...
// we currently dont clear useless timings (eg. dates in the past), this will not
// be an issue post-poc as we will store these in a db where these are cleaned up

impl Scheduler {
  pub fn new() -> BoxResult<Self> {
    let runtime = tokio::runtime::Handle::try_current()?;

    Ok(Self {
      runtime,
      tasks: HashMap::new(),
    })
  }

  // TODO: should make sure these generics can always be inferred from input
  // we only need type info for this to return a correctly typed channel
  // then we can store the future-producing fn in type-erased form
  pub fn register_task<T, F, R>(&mut self, id: &str, buffer: usize, task_fn: T) -> SchedulerResult<mpsc::Receiver<R>>
  where
    T: Fn(mpsc::Sender<R>) -> F + Send + 'static,
    F: Future<Output = ()> + Send + 'static,
    R: 'static,
  {
    if self.tasks.contains_key(id) {
      return Err(SchedulerError::DuplicateTaskId(id.to_string()));
    }

    let (tx, rx) = mpsc::channel::<R>(buffer);

    // we need to do some closure magic to pass a clone of sender into each instance of the task
    self.tasks.insert(
      id.to_string(),
      Box::new(move || {
        let tx = tx.clone();

        Box::pin(task_fn(tx))
      }),
    );

    Ok(rx)
  }

  // TODO: validate these claims!
  // this only removes the task handler, it does not stop any currently running futures
  // abort should be called on all scheduled task handles to stop all running futures
  // the associated channel receiver will return none when all senders are dropped (ie. all futures are finished)
  pub fn remove_task(&mut self, id: &str) -> SchedulerResult<()> {
    // immediately drop the task handler
    let _ = self
      .tasks
      .remove(id)
      .ok_or_else(|| SchedulerError::TaskNotFound(id.to_string()))?;

    Ok(())
  }

  // TODO: rewrite
  // since we specify the future type here, we can return a channel which will produce *something* *eventually*
  // BUT, we can also store a type-erased future and manage its scheduling internally as we see fit
  pub fn spawn_task(&self, id: &str, timing: Timing) -> SchedulerResult<task::JoinHandle<()>> {
    let task_fn = self
      .tasks
      .get(id)
      .ok_or_else(|| SchedulerError::TaskNotFound(id.to_string()))?;

    match timing {
      Timing::Immediate => {
        let future = task_fn();
        let handle = self.runtime.spawn(future);

        Ok(handle)
      }
      // Timing::DateTime(date_time) => {}
      // Timing::Repeating(interval_duration) => {}
      _ => todo!("complete the rest of the scheduler issues"),
    }
  }
}
