use std::{collections::HashMap, future::Future, pin::Pin, time::Duration};

use chrono::{DateTime, Utc};
use tokio::{
  sync::{mpsc, oneshot},
  task::JoinHandle,
};

use crate::error::BoxResult;

type BoxFuture<T> = Pin<Box<dyn Future<Output = T> + Send + 'static>>;
type FutureFn<T> = Box<dyn Fn() -> BoxFuture<T>>;

// pub fn create_task<F>(future: F) -> FutureFn<F::Output>
// where
//   F: Future + Send + 'static,
//   F::Output: Send + 'static,
// {
//   let closure = || Box::pin(future);

//   Box::new(closure)
// }

pub enum Timing {
  Immediate,
  Delayed(DateTime<Utc>),
  Repeating(Duration),
}

pub struct Scheduler {
  // if this was some threadpool, it would need to be owned
  // eventually i wanna (possibly) abstract this to work with anything that can spawn tasks
  runtime: tokio::runtime::Handle,
  // these can instead be fetched from a database, in which case we would delete records if are no longer needed
  timings: HashMap<String, Timing>,
}

impl Scheduler {
  pub fn new() -> Self {
    let runtime = tokio::runtime::Handle::current();

    Self {
      runtime,
      timings: HashMap::new(),
    }
  }

  pub fn schedule(&mut self, id: &str, timing: Timing) {
    self.timings.insert(id.to_string(), timing);
  }

  // the id is used for fetching scheduling info
  pub fn spawn<T>(&self, id: &str, future_fn: FutureFn<T>)
  where
    T: Send + 'static,
  {
    let timing = self.timings.get(id).unwrap();

    match timing {
      Timing::Immediate => {
        let future = future_fn();

        self.runtime.spawn(future);
      }
      // we may run into a potential issue here - if we call schedule()
      // and only call this after the set date, this will error
      Timing::Delayed(date_time) => {
        let delay = date_time.signed_duration_since(Utc::now());
        let future = future_fn();

        self.runtime.spawn(async move {
          tokio::time::sleep(delay.to_std().unwrap()).await;

          future.await;
        });
      }
      Timing::Repeating(duration) => {
        // let start = std::time::Instant::now() + *duration;
        // let mut interval = tokio::time::interval_at(start.into(), *duration);

        // self.runtime.spawn(async move {
        //   loop {
        //     interval.tick().await;

        //     let future = future_fn();
        //     future.await;
        //   }
        // });

        todo!();
      }
    }

    // since we specify the return type here, we can return a channel which will produce *something* *eventually*
    // BUT, we can also store a type-erased future type and manage its scheduling internally as we see fit

    // todo!();
  }

  pub fn abort(&self, id: &str) {
    todo!();
  }
}
