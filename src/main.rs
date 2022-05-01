use util::BoxResult;

mod api;
mod config;
mod scheduler;
mod util;

#[tokio::main]
async fn main() -> BoxResult<()> {
  // setup tracing to output logs to stdout
  tracing_subscriber::fmt::init();

  let config = config::Config::load()?;

  let anime_api = api::Api::new();
  anime_api.serve(config.api.ip, config.api.port).await?;

  Ok(())
}
