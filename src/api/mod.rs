use std::net;

use axum::{body::Body, routing::get, Router, Server};
use tracing::info;

use crate::err::BoxResult;

pub struct Api {
  router: Router<Body>,
}

impl Api {
  pub fn new() -> Self {
    let router = Router::new().route("/", get(|| async {}));

    Self { router }
  }

  pub async fn serve(self, ip: net::Ipv4Addr, port: u16) -> BoxResult<()> {
    let addr = net::SocketAddr::from((ip, port));

    info!("server listening on {}:{}", ip, port);

    Server::bind(&addr).serve(self.router.into_make_service()).await?;

    Ok(())
  }
}
