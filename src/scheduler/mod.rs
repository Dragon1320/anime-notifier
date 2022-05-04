use std::{collections::HashMap, future::Future, sync::Arc};

use serde::ser::Error;
use tokio::sync::mpsc;

use crate::util::BoxResult;

use self::{
  error::{SchedulerError, SchedulerResult},
  task::{TaskFn, TaskHandle, Timing},
};

pub mod error;
pub mod task;

pub struct Scheduler {
  // we can use tokio::spawn() instead, but i wanna tie our ability to schedule tasks to the lifetime
  // of scheduler, this will be useful later if we choose to switch this out to something that does
  // require to be owned/alive to schedule tasks
  runtime: tokio::runtime::Handle,

  // all tasks are stored as handler functions which can later be called to produce a future
  // these can continuously produce futures and so can be scheduled multiple times
  tasks: HashMap<String, TaskFn>,
}

// TODO: some helper fns (eg. to see if a handler with some id exists)
// TODO: add logging (tracing stuff will probably come in useful here)
impl Scheduler {
  pub fn new() -> BoxResult<Self> {
    // for now just try to get a handle to the current tokio executor, this will probably be expanded post-poc
    let runtime = tokio::runtime::Handle::try_current()?;

    Ok(Self {
      runtime,
      tasks: HashMap::new(),
    })
  }

  // return a channel that may produce *something* *eventually*
  // after creating the channel, we no longer need type info - we can store the future-producing fn in type-erased form

  // why generics over trait objects:
  // - we can handle the boxing and pinning here, so we have a more ergonomic api
  // - if we wanna box the future, it needs to be pinned and so cannot be moved

  // TODO: we could always wrap the future such that any value it returns gets sent over the channel?
  // if we did this we could probably remove a layer of closures (would be fine with some more boxed types)
  pub fn register_task<T, F, R>(&mut self, id: &str, buffer: usize, task_fn: T) -> SchedulerResult<mpsc::Receiver<R>>
  where
    T: Fn(mpsc::Sender<R>) -> F + 'static,
    F: Future<Output = ()> + Send + 'static,
    R: 'static,
  {
    if self.tasks.contains_key(id) {
      return Err(SchedulerError::DuplicateTaskId(id.to_string()));
    }

    let (tx, rx) = mpsc::channel::<R>(buffer);

    // we need to do some closure magic to pass a clone of sender into each instance of the task
    // the mpsc sender will live as long as 1. a task is using it and 2. the task handler is registered in the executor
    self.tasks.insert(
      id.to_string(),
      Box::new(move || {
        let tx = tx.clone();

        Box::pin(task_fn(tx))
      }),
    );

    Ok(rx)
  }

  // this only removes the task handler, it does not abort any currently running futures
  // to ensure no instances of this task are running, we need to call abort() on all its task handles
  // once the task is removed and all its instances drop the mpsc sender, its associated channel will close
  pub fn remove_task(&mut self, id: &str) -> SchedulerResult<()> {
    // immediately drop the task handler
    let _ = self
      .tasks
      .remove(id)
      .ok_or_else(|| SchedulerError::TaskNotFound(id.to_string()))?;

    Ok(())
  }

  // schedule a task that has had its handler fn already registered
  // post-poc these timings can come from a database for tasks which need to persist between restarts - we would first
  // register all required handlers with register_task() and then call some restore() to restore the scheduler state
  pub fn spawn_task(&self, id: &str, timing: Timing) -> SchedulerResult<TaskHandle> {
    let task_fn = self
      .tasks
      .get(id)
      .ok_or_else(|| SchedulerError::TaskNotFound(id.to_string()))?;

    match timing {
      Timing::Immediate => {
        let future = task_fn();
        let handle = self.runtime.spawn(future);

        let task_handle = TaskHandle::new(handle);

        Ok(task_handle)
      }
      Timing::DateTime(date_time) => {
        // TODO: better error handling
        let duration = (date_time - chrono::Utc::now()).to_std().unwrap();

        let future = task_fn();
        let handle = self.runtime.spawn(async move {
          tokio::time::sleep(duration).await;

          future.await;
        });

        let task_handle = TaskHandle::new(handle);

        Ok(task_handle)
      }
      Timing::Delayed(duration) => {
        // TODO: better error handling
        let duration = duration.to_std().unwrap();

        let future = task_fn();
        let handle = self.runtime.spawn(async move {
          tokio::time::sleep(duration).await;

          future.await;
        });

        let task_handle = TaskHandle::new(handle);

        Ok(task_handle)
      }
      Timing::Repeating(interval_duration) => {
        // TODO: better error handling
        let duration = interval_duration.to_std().unwrap();
        // we set an interval <duration> in the future, since otherwise the event would fire instantly
        let mut interval = tokio::time::interval_at(tokio::time::Instant::now() + duration, duration);

        // let a = Arc::new(task_fn);

        let handle = self.runtime.spawn(async move {
          // let a = a.clone();

          loop {
            interval.tick().await;

            // let future = task_fn();
            // future.into().await;
          }
        });

        let task_handle = TaskHandle::new(handle);

        Ok(task_handle)
      }
    }
  }
}
