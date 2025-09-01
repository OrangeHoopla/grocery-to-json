use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct RawText {
    pub raw_text: String,
    pub created: DateTime<Utc>,
    pub image_processor: String
}

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

enum TextProcessor {
    GenericProcessor,
    AldiProcessor,
    WholeFoodsProcessor
}

enum ImageProcessor {
    GenericProcessor,
    SobelProcessor
}