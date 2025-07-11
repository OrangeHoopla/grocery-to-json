use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::dao::raw_text_dao::RawText;

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Reciept {
    pub store: String,
    pub address: String,
    pub items: Vec<(String, f64)>,
    pub total: f64,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub transaction_date: DateTime<Utc>,
}

// pub trait RecieptDAO {
//     // Associated function signature; `Self` refers to the implementor type.
//     fn get_store_name(name: &'static str) -> String;
//     fn get_address(&self) -> String;
//     fn get_time_of_purchase(&self) -> DateTime<Utc>;
//     fn get_total_cost(&self) -> f64;
//     fn get_items(&self) -> Vec<(String, f64)>;
//     fn get_reciept(raw_text: RawText) -> Reciept;
// }

// #[derive(Debug, Deserialize, Serialize, Default)]
// pub struct RawText {
//     pub image_processor: String,
//     pub created: DateTime<Utc>,
//     pub result: String,
// }
