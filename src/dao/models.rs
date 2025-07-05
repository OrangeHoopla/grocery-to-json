use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct GroceryList {
    pub store: String,
    pub address: String,
    pub items: Vec<(String, f64)>,
    pub total: f64,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub transaction_date: DateTime<Utc>,
}
