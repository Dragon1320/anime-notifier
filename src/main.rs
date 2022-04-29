use std::{future::Future, marker::PhantomData, ops::Add, sync::Arc, time::Duration};

use chrono::{DateTime, Utc};
use error::BoxResult;
use scheduler::{Scheduler, Timing};
use tokio::{sync::oneshot, task::JoinHandle};
use tracing::info;

mod api;
mod config;
mod error;
mod scheduler;

#[tokio::main]
async fn main() -> BoxResult<()> {
  // setup tracing to output logs to stdout
  tracing_subscriber::fmt::init();

  let mut scheduler = Scheduler::new();

  let date_time = Utc::now() + chrono::Duration::seconds(2);

  scheduler.schedule("rawrxd", Timing::Delayed(date_time));

  scheduler.spawn(
    "rawrxd",
    Box::new(|| {
      Box::pin(async {
        info!("rawrxd");
      })
    }),
  );

  tokio::time::sleep(Duration::from_secs(4)).await;

  Ok(())
}
