use std::{future::Future, pin::Pin};

use chrono::{DateTime, Duration, Utc};
use tokio::task::JoinHandle;

pub type BoxFuture = Pin<Box<dyn Future<Output = ()> + Send>>;
pub type TaskFn = Box<dyn Fn() -> BoxFuture>;

#[derive(Debug, Clone)]
pub enum Timing {
  Immediate,
  DateTime(DateTime<Utc>),
  Delayed(Duration),
  Repeating(Duration),
}

#[derive(Debug)]
pub struct TaskHandle {
  handle: JoinHandle<()>,
}

impl TaskHandle {
  pub fn new(handle: JoinHandle<()>) -> Self {
    Self { handle }
  }

  pub fn abort(self) {
    self.handle.abort();
  }
}
