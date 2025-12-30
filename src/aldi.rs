use chrono::{DateTime, Utc};
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::reciept::Reciept;

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq)]
pub struct Item {
    pub id: u64,
    pub name: String,
    pub cost: f64,
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct Aldi {
    pub location: String,
    pub total: f64,
    pub transaction_date: Option<DateTime<Utc>>,
    pub items: Vec<Item>,
}

impl TryFrom<Reciept> for Aldi {
    type Error = ();

    fn try_from(value: Reciept) -> Result<Self, Self::Error> {
        Ok(Aldi::convert(value.text))
    }
}

impl Aldi {
    fn convert(value: String) -> Aldi {
        Aldi {
            location: Self::get_store_name(),
            total: Self::get_total_cost(),
            transaction_date: Some(Utc::now()),
            items: Self::get_items(value),
        }
    }

    fn get_store_name() -> String {
        "Aldi".to_owned()
    }

    fn get_total_cost() -> f64 {
        12.0
    }

    fn get_items(raw_text: String) -> Vec<Item> {
        let re = Regex::new(r"[0-9]{6,7} (.*)").unwrap();
        let mut items: Vec<Item> = Vec::new();
        for cap in re.captures_iter(&raw_text) {
            items.push(Item {
                id: 0,
                name: cap[1].to_owned(),
                cost: 0.0,
            });
        }

        items
    }
}
