use std::env;

use chrono::{DateTime, Utc};
use mongodb::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct RawText {
    pub raw_text: String,
    pub created: DateTime<Utc>,
    pub image_processor: String
}

impl RawText {
    pub async fn save(text: RawText) {
        let client = Client::with_uri_str(env::var("MONGODB_URI").unwrap().as_str())
            .await
            .unwrap();
        let image_client: mongodb::Collection<RawText> =
            client.database("groclog").collection("rawtext");

        let _ = image_client.insert_one(text).await;
    }
}