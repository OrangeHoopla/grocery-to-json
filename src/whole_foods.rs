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
pub struct WholeFoods {
    pub location: String,
    pub total: f64,
    pub transaction_date: Option<DateTime<Utc>>,
    pub items: Vec<Item>,
}

impl TryFrom<Reciept> for WholeFoods {
    type Error = ();

    fn try_from(value: Reciept) -> Result<Self, Self::Error> {
        Ok(WholeFoods::convert(value.text))
    }
}

impl WholeFoods {
    fn convert(value: String) -> WholeFoods {
        WholeFoods {
            location: Self::get_store_name(),
            total: Self::get_total_cost(&value),
            transaction_date: Some(Utc::now()),
            items: Self::get_items(&value),
        }
    }

    fn get_store_name() -> String {
        "Whole Foods".to_owned()
    }

    fn get_total_cost(raw_text: &String) -> f64 {
        let re = Regex::new(r"Total: \$(.*)").unwrap();
        let caps = re.captures(&raw_text);

        match caps {
            Some(part) => match part.get(1) {
                Some(part) => match part
                .as_str()
                .replace(" ", "")
                .parse::<f64>() {
                    Ok(res) => res,
                    Err(_) => 0.0,
                },
                None => 0.0,
            },
            None => 0.0,
        }
    }

    fn get_items(raw_text: &String) -> Vec<Item> {
        let re = Regex::new(r"(.*) \$(.*) [A-Z]\b").unwrap();
        let mut items: Vec<Item> = Vec::new();

        for cap in re.captures_iter(raw_text) {

            let item_name = match cap.get(1) {
                Some(res) => res.as_str(),
                None => " ",
            };

            let item_cost = match cap.get(2) {
                Some(part) => match part.as_str().parse::<f64>() {
                    Ok(res) => res,
                    Err(_) => 0.0,
                },
                None => 0.0,
            };

            items.push(Item {
                id: 0,
                name: item_name.to_owned(),
                cost: item_cost,
            });
        }

        items
    }
}