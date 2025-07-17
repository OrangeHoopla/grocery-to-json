use std::env;

use chrono::{DateTime, Utc};
use mongodb::Client;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Item {
    pub name: String,
    pub cost: f64,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Reciept {
    pub _id: Uuid,
    pub store: String,
    pub address: String,
    pub items: Vec<Item>,
    pub total: f64,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub transaction_date: DateTime<Utc>,
}

impl Reciept {
    pub async fn save(reciept: Reciept) {
        let client = Client::with_uri_str(env::var("MONGODB_URI").unwrap().as_str())
            .await
            .unwrap();
        let reciept_client: mongodb::Collection<Reciept> =
            client.database("groclog").collection("reciepts");

        let _ = reciept_client.insert_one(reciept).await;
    }
}

