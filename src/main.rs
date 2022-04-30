use std::time::Duration;

use tokio::sync::mpsc;
use tracing::{debug, info};

use util::BoxResult;

use crate::scheduler::{task::Timing, Scheduler};

mod api;
mod config;
mod scheduler;
mod util;

#[tokio::main]
async fn main() -> BoxResult<()> {
  // setup tracing to output logs to stdout
  tracing_subscriber::fmt::init();

  let config = config::Config::load()?;

  // some config/logging examples
  info!("message: {}", config.message);
  info!("cat colour: {}", config.cat.colour);

  // run with: RUST_LOG=debug cargo run
  debug!("{:?}", config);

  // scheduler example
  let mut scheduler = Scheduler::new()?;

  let task_id = "crunchyroll_fetch";

  let owo = "owo".to_string();

  // register a task handler than can spawn tasks later, given the task_id
  let mut rx_chan = scheduler.register_task(task_id, 16, move |tx_chan: mpsc::Sender<String>| {
    // we can capture outside variables
    let message = owo.clone();

    async move {
      // sleep for a second
      tokio::time::sleep(Duration::from_millis(1000)).await;

      // send a message in our channel
      tx_chan.send(message).await.unwrap();
    }
  })?;

  // spawn an instance of the task above to run as soon as possible
  scheduler.spawn_task(task_id, Timing::Immediate)?;

  // de-register the task handler
  scheduler.remove_task(task_id)?;

  // this will only exit when no more messages will be sent (ie. the task handler is removed and all its instances drop tx_chan)
  while let Some(message) = rx_chan.recv().await {
    info!("{}", message);
  }

  Ok(())
}
