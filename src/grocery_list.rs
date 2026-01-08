use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    aldi::Aldi,
    giant::Giant,
    reciept::{Reciept, Store},
    whole_foods::WholeFoods,
};

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq)]
pub struct Item {
    pub id: u64,
    pub name: String,
    pub cost: f64,
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct GroceryList {
    pub location: String,
    pub total: f64,
    pub transaction_date: Option<DateTime<Utc>>,
    pub items: Vec<Item>,
}

impl TryFrom<Reciept> for GroceryList {
    type Error = String;

    fn try_from(mut value: Reciept) -> Result<Self, Self::Error> {
        // Future spot for guessing store type
        if value.store.is_none() {
            value.store = Some(Store::Aldi);
        }

        match value.store {
            Some(Store::Aldi) => Ok(<GroceryList as Aldi>::convert(value.text)),
            Some(Store::WholeFoods) => Ok(<GroceryList as WholeFoods>::convert(value.text)),
            Some(Store::Giant) => Ok(<GroceryList as Giant>::convert(value.text)),
            None => Err("Unknown store type".to_owned()),
        }
    }
}
