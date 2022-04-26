use std::time::Duration;

use error::BoxResult;
use scheduler::Scheduler;
use tracing::info;

mod api;
mod config;
mod error;
mod scheduler;

#[tokio::main]
async fn main() -> BoxResult<()> {
  // setup tracing to output logs to stdout
  tracing_subscriber::fmt::init();

  let sc = Scheduler::new()?;

  let fut_closure = || async {
    info!("rawrxd");
  };

  sc.repeating(fut_closure, Duration::from_secs(1));

  tokio::time::sleep(Duration::from_secs(5)).await;

  Ok(())
}
