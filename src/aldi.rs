use std::time::Instant;
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::reciept::Reciept;



#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq)]
pub struct Item {
    pub name: String,
    pub cost: f64,
}

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct Aldi {
    pub location: String,
    pub total: f64,
    #[serde(with = "approx_instant")]
    pub created: Instant,
    #[serde(with = "approx_instant")]
    pub updated: Instant,
    #[serde(with = "approx_instant")]
    pub transaction_date: Instant,
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
            created: Instant::now(), 
            updated: Instant::now(), 
            transaction_date: Instant::now(),
            items: Self::get_items(value)
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
                name: cap[1].to_owned(),
                cost: 0.0,
            });
        }

        items
    }
}






mod approx_instant {
    use std::time::{Instant, SystemTime};
    use serde::{Serialize, Serializer, Deserialize, Deserializer, de::Error};

    pub fn serialize<S>(instant: &Instant, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let system_now = SystemTime::now();
        let instant_now = Instant::now();
        let approx = system_now - (instant_now - *instant);
        approx.serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Instant, D::Error>
    where
        D: Deserializer<'de>,
    {
        let de = SystemTime::deserialize(deserializer)?;
        let system_now = SystemTime::now();
        let instant_now = Instant::now();
        let duration = system_now.duration_since(de).map_err(Error::custom)?;
        let approx = instant_now - duration;
        Ok(approx)
    }
}