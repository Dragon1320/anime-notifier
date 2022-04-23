use tracing::{debug, info};

use err::BoxResult;

mod api;
mod cfg;
mod err;

#[tokio::main]
async fn main() -> BoxResult<()> {
  // setup tracing to output logs to stdout
  tracing_subscriber::fmt::init();

  // TODO: add logging to config
  let config = cfg::Config::load()?;

  // some config/logging examples
  info!("message: {}", config.message);
  info!("cat colour: {}", config.cat.colour);

  // run with: RUST_LOG=debug cargo run
  debug!("{:?}", config);

  let anime_api = api::Api::new();
  anime_api.serve(config.api.ip, config.api.port).await?;

  Ok(())
}
