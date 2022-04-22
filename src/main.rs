use tracing::{debug, info};

#[tokio::main]
async fn main() {
  tracing_subscriber::fmt::init();

  info!("にゃ～");

  // run with: RUST_LOG=debug cargo run
  debug!("owo hidden");
}
