use tracing::{debug, info};

mod api;

#[tokio::main]
async fn main() {
  tracing_subscriber::fmt::init();

  info!("にゃ～");

  // run with: RUST_LOG=debug cargo run
  debug!("owo hidden")
}
