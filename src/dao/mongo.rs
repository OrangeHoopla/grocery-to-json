use std::env;

use bson::Document;
use mongodb::{Client, Collection};

pub struct MongoConnection {
    connection_string: String,
    collection: String,
    database: String,
}

impl MongoConnection {
    pub async fn client(&self) -> Option<Collection<Document>> {
        let client = Client::with_uri_str(self.connection_string.clone())
            .await
            .unwrap();

        return Some(client.database(&self.database).collection(&self.collection));
    }

    pub fn get_collection(collection: &str) -> Self {
        Self {
            collection: collection.to_owned(),
            connection_string: env::var("MONGODB_URI")
                .expect("You must set the MONGODB_URI environment var!"),
            database: env::var("MONGODB_DB")
            .expect("You must set the MONGODB_DB environment var!"),
        }
    }
}



// GroceryList Client -> groceryList(collection) DAO -> mongodb
//
