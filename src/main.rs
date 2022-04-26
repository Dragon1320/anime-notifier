use error::BoxResult;

mod api;
mod config;
mod error;
mod scheduler;

#[tokio::main]
async fn main() -> BoxResult<()> {
  // setup tracing to output logs to stdout
  tracing_subscriber::fmt::init();

  Ok(())
}
