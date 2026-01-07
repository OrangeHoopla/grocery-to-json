use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{aldi::Aldi, giant::Giant, reciept::Reciept, whole_foods::WholeFoods};


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
    type Error = ();

    fn try_from(value: Reciept) -> Result<Self, Self::Error> {
        Ok(<GroceryList as Aldi>::convert(value.text))
    }

}