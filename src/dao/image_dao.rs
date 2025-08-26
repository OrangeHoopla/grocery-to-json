use std::env;

use chrono::{DateTime, Utc};
use mongodb::Client;
use serde::{Deserialize, Serialize};


#[allow(dead_code)]
enum DataAccessError {
    NotFound,
    ParsingError,
    UnknownError
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Image {
    pub file_name: String,
    pub created: DateTime<Utc>,
}

impl Image {
    pub async fn save(image: Image) {
        let client = Client::with_uri_str(env::var("MONGODB_URI").unwrap().as_str())
            .await
            .unwrap();
        let image_client: mongodb::Collection<Image> =
            client.database("groclog").collection("images");

        let _ = image_client.insert_one(image).await;
    }
}

struct ImagesRepository ();

impl ImagesRepository {
    async fn _save(&self, _id: &str) {
        todo!();
    }

    async fn _load(&self, _id: &str) {
        todo!();
    }
}
