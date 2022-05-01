use std::{collections::HashMap, str::FromStr};

use bson::{doc, oid::ObjectId};
use futures_util::StreamExt;
use mongodb::{bson::Document, options::FindOptions};
use tracing::{debug, info};

use err::BoxResult;

mod api;
mod cfg;
mod database;
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

  // let db = database::Db::new(&config.db.db_str, &config.db.db_name, &config.db.collection_name).await?;
  // let request = database::Request {
  //   id: None,
  //   source: "crunchyroll3".to_string(),
  //   anime: "kaguya-sama-s3".to_string(),
  //   trigger: "available".to_string(),
  //   target: "https://discord.com/api/webhooks/...".to_string(),
  //   payload: HashMap::from([
  //     ("content".to_string(), "new release".to_string()),
  //     ("content2".to_string(), "new release2".to_string()),
  //   ]),
  //   updated_at: chrono::Utc::now(),
  //   created_at: chrono::Utc::now(),
  // };

  // let id = ObjectId::from_str("626d8f3e0493e51c91d7bb65").unwrap();
  // let query = doc! {
  //   "_id": id,
  // };
  // let update_doc = doc! {
  //   "$set": { "source": "c"},
  //   "$currentDate": {
  //     "updated_at": true,
  //   }
  // };
  // db.find_doc_by_source(request).await?;
  // db.create_doc(request).await?;
  // db.find_one_and_update(query, update_doc).await?;
  // let mut c = db.find_doc_by_source(query).await?;
  // while let Some(u) = c.next().await {
  //   println!("{:?}", u);
  // }

  // let mut c = db.delete_doc(query).await?;
  // info!("{:#?}", c);
  Ok(())
}
