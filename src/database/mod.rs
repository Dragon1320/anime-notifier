use mongodb::{options::ClientOptions, Client};
use std::error::Error;

pub struct Db {
  pub client: Client,
}

impl Db {
  pub async fn new(db_str: &str) -> Result<Self, Box<dyn Error + Send + Sync>> {
    let mut client_options = ClientOptions::parse(db_str).await?;

    client_options.app_name = Some("anime-notifier".to_string());
    Ok(Self {
      client: Client::with_options(client_options)?,
    })
  }
}
