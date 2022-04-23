use tracing::{debug, info};

mod cfg;
mod err;

#[tokio::main]
async fn main() -> err::BoxResult<()> {
  // setup tracing to output logs to stdout
  tracing_subscriber::fmt::init();

  // some example code...
  let config = cfg::Config::load()?;

  info!("message: {}", config.message);
  info!("cat colour: {}", config.cat.colour);

  // run with: RUST_LOG=debug cargo run
  debug!("{:?}", config);

  Ok(())
}
