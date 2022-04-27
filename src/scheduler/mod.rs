use std::future::Future;

use crate::error::BoxResult;

// i do plan to abstract this at some point so it can be used with executors
// other than tokio, but i feel like doing it now will severely slow down the poc
pub struct Scheduler {
  handle: tokio::runtime::Handle,
}

// abstractions:
// - something that can spawn tasks
// - something that can sleep
// - some task that can be spawned

// NOTES:
// when scheduling tasks, we do not care about any values produced
// a scheduled task can be split into 2 stages: waiting + executing
// - a task must be safe to cancel when waiting
// - a task might not be safe to cancel when executing (eg. if it produces side-effects)
// - a task is always safe to cancel when it does not produce side-effects when executing

// for now, all tasks will be transient (ie. they wont persist after a restart)
impl Scheduler {
  pub fn new() -> BoxResult<Self> {
    // any clones of handle are ok - its reference counted
    let handle = tokio::runtime::Handle::try_current()?;

    Ok(Self { handle })
  }

  pub fn spawn_immediate<T, F>(&self, task_fn: F)
  where
    T: Future + Send + 'static,
    T::Output: Send + 'static,
    F: Fn() -> T,
  {
    let task = task_fn();

    self.handle.spawn(task);
  }

  pub fn spawn_delayed<T, F>(&self, task_fn: F, delay: std::time::Duration) -> tokio::task::JoinHandle<()>
  where
    T: Future + Send + 'static,
    T::Output: Send + 'static,
    F: Fn() -> T,
  {
    let task = task_fn();
    let handle = self.handle.clone();

    self.handle.spawn(async move {
      tokio::time::sleep(delay).await;

      handle.spawn(task);
    })
  }

  pub fn spawn_datetime<T, F>(&self, task_fn: F, instant: std::time::Instant) -> tokio::task::JoinHandle<()>
  where
    T: Future + Send + 'static,
    T::Output: Send + 'static,
    F: Fn() -> T,
  {
    let task = task_fn();
    let handle = self.handle.clone();

    self.handle.spawn(async move {
      tokio::time::sleep_until(instant.into()).await;

      handle.spawn(task);
    })
  }

  pub fn spawn_repeating<T, F>(&self, task_fn: F, duration: std::time::Duration) -> tokio::task::JoinHandle<()>
  where
    T: Future + Send + 'static,
    T::Output: Send + 'static,
    F: Fn() -> T + Send + 'static,
  {
    let handle = self.handle.clone();

    // the first tick of interval happens immediately, delay this to interval duration
    let start = std::time::Instant::now() + duration;
    let mut interval = tokio::time::interval_at(start.into(), duration);

    self.handle.spawn(async move {
      loop {
        interval.tick().await;

        handle.spawn(task_fn());
      }
    })
  }
}
