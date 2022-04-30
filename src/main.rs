use std::time::Duration;

use tokio::sync::mpsc;
use tracing::{debug, info};

use util::BoxResult;

use crate::scheduler::task::Timing;

mod api;
mod config;
mod scheduler;
mod util;

#[tokio::main]
async fn main() -> BoxResult<()> {
  // setup tracing to output logs to stdout
  tracing_subscriber::fmt::init();

  // TODO: add logging to config
  let config = config::Config::load()?;

  // some config/logging examples
  info!("message: {}", config.message);
  info!("cat colour: {}", config.cat.colour);

  // run with: RUST_LOG=debug cargo run
  debug!("{:?}", config);

  // let anime_api = api::Api::new();
  // anime_api.serve(config.api.ip, config.api.port).await?;

  let mut s = scheduler::Scheduler::new()?;

  // TODO: why is async move allowed here?
  // TODO: move ||?
  // TODO: || async move?
  let mut rx = s.register_task("rawrxd", 16, |tx: mpsc::Sender<()>| async move {
    tokio::time::sleep(Duration::from_millis(1000)).await;

    info!("rawrxd");

    tx.send(()).await;
  })?;

  s.spawn_task("rawrxd", Timing::Immediate)?;
  s.remove_task("rawrxd")?;

  info!("{:?}", rx.recv().await);
  info!("{:?}", rx.recv().await);

  // sleep so tasks have time to complete before we exit
  tokio::time::sleep(Duration::from_millis(2000)).await;

  Ok(())
}
