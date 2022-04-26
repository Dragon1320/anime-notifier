// define cancellable intervals:
// - pre work started
// - pre x checkpoint
// - post x checkpoint

// define guarantees
// - at least once
// - at most once
// - exactly once

// needs:
// - not use too many resources -> only schedule whats required
// - persistent/transient timers -> persistent survive a reboot
// - be a wrapper - we dont interact with tokio outside of the module (and could switch out for different internals without affecting anything)
// - predictable - any action (such as cancelling a task) should have well defined consequences

// also:
// - tasks may need to be transactional in nature

// optimisation: instead of scheduling a task, schedule the result processing of that task
// so for example, we schedule a http request earlier and processed the response at the scheduled time

use std::{future::Future, sync::Arc};

use crate::error::BoxResult;

trait Stateful {
  fn init() -> Self;
  fn update(&self);
  fn subscribe(&self);
}

pub trait Spawn {
  fn spawn<F>(&self, future: F)
  where
    F: Future + Send + 'static,
    F::Output: Send + 'static;
}

impl Spawn for tokio::runtime::Handle {
  fn spawn<F>(&self, future: F)
  where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
  {
    self.spawn(future);
  }
}

pub struct Scheduler<H>
where
  H: Spawn,
{
  handle: H,
}

// for now, all tasks will be transient (ie. they wont persist after a restart)
impl Scheduler<tokio::runtime::Handle> {
  pub fn new() -> BoxResult<Self> {
    let handle = tokio::runtime::Handle::try_current()?;

    Ok(Self { handle })
  }

  pub fn immediate<F>(&self, future: F)
  where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
  {
    self.handle.spawn(future);
  }

  pub fn delayed<F>(&self, future: F, delay: std::time::Duration)
  where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
  {
    // handle is reference counted so this is ok
    let handle = self.handle.clone();

    self.handle.spawn(async move {
      tokio::time::sleep(delay).await;

      handle.spawn(future);
    });
  }

  pub fn datetime<F>(&self, future: F, instant: std::time::Instant)
  where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
  {
    let handle = self.handle.clone();

    self.handle.spawn(async move {
      tokio::time::sleep_until(instant.into()).await;

      handle.spawn(future);
    });
  }

  pub fn repeating<C, F>(&self, future: C, duration: std::time::Duration)
  where
    C: Fn() -> F + Send + 'static,
    F: Future + Send + 'static,
    F::Output: Send + 'static,
  {
    let handle = self.handle.clone();

    // we dont want it to tick instantly (or we could make that a param)
    let start = std::time::Instant::now() + duration;
    let mut interval = tokio::time::interval_at(start.into(), duration);

    self.handle.spawn(async move {
      loop {
        interval.tick().await;

        handle.spawn(future());
      }
    });
  }
}
