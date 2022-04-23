use std::net;

use axum::{body::Body, routing::get, Router, Server};

// TODO: temp - remove
type BoxResult<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

pub struct Api {
  router: Router<Body>,
}

impl Api {
  pub fn new() -> Self {
    let router = Router::new().route("/healthz", get(|| async {}));

    Self { router }
  }

  pub async fn serve(self) -> BoxResult<()> {
    // TODO: add this to config
    // TODO: use some interesting port number?
    let addr = net::SocketAddr::from(([127, 0, 0, 1], 3000));

    Server::bind(&addr).serve(self.router.into_make_service()).await?;

    Ok(())
  }
}
