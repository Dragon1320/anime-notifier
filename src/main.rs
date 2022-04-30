use std::time::Duration;

use tokio::sync::mpsc;
use tracing::{debug, info};

use error::BoxResult;

mod api;
mod config;
mod error;
mod scheduler;

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

  let rx = s.register_task("rawrxd", 16, |_tx: mpsc::Sender<()>| async {
    info!("rawrxd");
  })?;

  s.spawn_task("rawrxd", scheduler::Timing::Immediate)?;

  // sleep so tasks have time to complete before we exit
  tokio::time::sleep(Duration::from_millis(500)).await;

  Ok(())
}
