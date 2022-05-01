use bson::doc;
use chrono::Utc;
// use futures_util::stream::stream::StreamExt;
use mongodb::{
  bson::{self, oid::ObjectId, Document},
  options::{ClientOptions, FindOneOptions, FindOptions},
  results::DeleteResult,
  Client, Collection, Cursor,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error};
use tracing::info;

pub struct Db {
  pub client: Client,
  pub db_name: String,
  pub collection_name: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
  #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
  pub id: Option<ObjectId>,
  pub source: String,
  pub anime: String,
  pub trigger: String, // TODO: create a struct for planned available and actually available
  pub target: String,
  pub(crate) payload: HashMap<String, String>,
  #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
  pub updated_at: chrono::DateTime<Utc>,
  #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
  pub created_at: chrono::DateTime<Utc>,
}

impl Db {
  pub async fn new(db_str: &str, db_name: &str, collection_name: &str) -> Result<Self, Box<dyn Error + Send + Sync>> {
    let mut client_options = ClientOptions::parse(db_str).await?;

    client_options.app_name = Some("anime-notifier".to_string());
    Ok(Self {
      client: Client::with_options(client_options)?,
      db_name: db_name.to_string(),
      collection_name: collection_name.to_string(),
    })
  }

  pub async fn create_doc(&self, request: Request) -> Result<(), Box<dyn Error + Send + Sync>> {
    let serialized_request = bson::to_bson(&request)?;
    let request_content = serialized_request.as_document().unwrap();
    let requests = self.client.database(&self.db_name).collection(&self.collection_name); //TODO: handle invalid config
    requests.insert_one(request_content.to_owned(), None).await?;
    Ok(())
  }

  pub async fn update_doc(&self, id: ObjectId, request: Document) -> Result<(), Box<dyn Error + Send + Sync>> {
    let requests: Collection<Document> = self.client.database(&self.db_name).collection(&self.collection_name); //TODO: handle invalid config
    let query = doc! {
      "_id": id,
    };
    requests.update_one(query, request, None).await?;
    Ok(())
  }

  pub async fn find_doc_by_id(&self, id: ObjectId) -> Result<Option<Document>, Box<dyn Error + Send + Sync>> {
    let requests: Collection<Document> = self.client.database(&self.db_name).collection(&self.collection_name); //TODO: handle invalid config
    let query = doc! {
      "_id": id,
    };
    Ok(requests.find_one(query, None).await?)
  }

  pub async fn find_one_and_update(
    &self,
    filter: Document,
    request: Document,
  ) -> Result<(), Box<dyn Error + Send + Sync>> {
    let requests: Collection<Document> = self.client.database(&self.db_name).collection(&self.collection_name); //TODO: handle invalid config
    requests.find_one_and_update(filter, request, None).await?;
    Ok(())
  }

  pub async fn find_doc_by_filter(&self, filter: Document) -> Result<Cursor<Document>, Box<dyn Error + Send + Sync>> {
    let requests: Collection<Document> = self.client.database(&self.db_name).collection(&self.collection_name); //TODO: handle invalid config
                                                                                                                //TODO: put the collections into struct
                                                                                                                //TODO: sort and pagination
    Ok(requests.find(filter, None).await?)
  }

  pub async fn count_doc(&self, filter: Document) -> Result<u64, Box<dyn Error + Send + Sync>> {
    let requests: Collection<Document> = self.client.database(&self.db_name).collection(&self.collection_name); //TODO: handle invalid config
    Ok(requests.count_documents(filter, None).await?)
  }

  pub async fn delete_doc(&self, filter: Document) -> Result<DeleteResult, Box<dyn Error + Send + Sync>> {
    let requests: Collection<Document> = self.client.database(&self.db_name).collection(&self.collection_name); //TODO: handle invalid config
    Ok(requests.delete_many(filter, None).await?)
  }
}
