use chrono::{DateTime, NaiveDate, NaiveTime, TimeZone, Utc};
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::reciept::Reciept;

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq)]
pub struct Item {
    pub name: String,
    pub cost: f64,
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct Giant {
    pub location: String,
    pub total: f64,
    pub transaction_date: DateTime<Utc>,
    pub items: Vec<Item>,
}

impl TryFrom<Reciept> for Giant {
    type Error = ();

    fn try_from(value: Reciept) -> Result<Self, Self::Error> {
        Ok(Giant::convert(value.text))
    }
}

impl Giant {
    fn convert(value: String) -> Giant {
        Giant {
            location: Self::get_store_name(),
            total: Self::get_total_cost(&value),
            transaction_date: Self::get_transaction_date(&value),
            items: Self::get_items(&value),
        }
    }

    fn get_store_name() -> String {
        "Giant".to_owned()
    }

    fn get_transaction_date(raw_text: &String) -> DateTime<Utc> {
        let re = Regex::new(r"([0-9]+\/[0-9]+\/[0-9]+)").unwrap();
        let caps = re.captures(&raw_text);

        match caps {
            Some(part) => match part.get(1) {
                Some(part) => match NaiveDate::parse_from_str(&part.as_str(), "%d/%m/%y") {
                    Ok(res) => Utc.from_utc_datetime(&res.and_time(NaiveTime::default())),
                    Err(_) => Utc::now(),
                },
                None => Utc::now(),
            },
            None => Utc::now(),
        }
    }

    fn get_total_cost(raw_text: &String) -> f64 {
        let re = Regex::new(r"BALANCE (.*)").unwrap();
        let caps = re.captures(&raw_text);

        match caps {
            Some(part) => match part.get(1) {
                Some(part) => match part.as_str().parse::<f64>() {
                    Ok(res) => res,
                    Err(_) => 0.0,
                },
                None => 0.0,
            },
            None => 0.0,
        }
    }

    fn get_items(raw_text: &String) -> Vec<Item> {
        let re = Regex::new(r"(.*) ([0-9,\.]+) [F,T]").unwrap();
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
                name: item_name.to_owned(),
                cost: item_cost,
            });
        }

        items
    }
}
