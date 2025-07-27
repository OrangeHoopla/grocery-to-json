use chrono::Utc;
use regex::Regex;
use uuid::Uuid;

use crate::dao::{raw_text_dao::RawText, reciept_dao::{Item, Reciept}};



pub struct GenericProcessor {
    pub raw_text: RawText,
}

impl GenericProcessor {
    fn get_store_name(&self) -> String {
        "Whole Foods".to_owned()
    }

    fn get_address(&self,raw_reciept: String) -> String {
        let re_address =
        Regex::new(r"MARKET[\r\n]+([^\r\n]+)[\r\n]+([^\r\n]+)[\r\n]+([^\r\n]+)").unwrap();
        let addr = re_address.captures(&raw_reciept);

        if addr.is_some() {
        let safe_addr = addr.unwrap();
        let mut address = safe_addr[1].to_owned();
        address.push(' ');
        address.push_str(&safe_addr[2]);
        address.push(' ');
        address.push_str(&safe_addr[3]);
        return address;
        }
        else {
            return "".to_owned();
        }
    }

    fn _get_time_of_purchase(&self) -> chrono::DateTime<chrono::Utc> {
        todo!()
    }

    fn get_total_cost(&self, raw_text: String) -> f64 {
        let re_total = Regex::new(r"Total.*:.*\$([0-9\.]+)").unwrap();
        let re_address =
        Regex::new(r"MARKET[\r\n]+([^\r\n]+)[\r\n]+([^\r\n]+)[\r\n]+([^\r\n]+)").unwrap(); // this needs to be fixed
        let total = re_total.captures(&raw_text);

        let real_total = match total {
            Some(x) => x,
            None => re_address.captures(&raw_text).unwrap(),
        };
        return real_total[1].parse::<f64>().unwrap_or(0.0);
    }

    fn get_items(&self, raw_text: String) -> Vec<Item> {
        let re = Regex::new(r"(?m)^(.*) \$([0-9\.]+).*[F\|JT]\s").unwrap();
        let mut items: Vec<Item> = Vec::new();
        for cap in re.captures_iter(&raw_text) {
            items.push(Item { name: cap[1].to_owned(), 
                              cost: cap[2].parse::<f64>().unwrap_or(0.0) });
            
        }

        items
    }

    pub fn get_reciept(&self) -> Reciept {
        
        return Reciept {
             _id: Uuid::new_v4(),
             store: self.get_store_name(),
             address: self.get_address(self.raw_text.raw_text.clone()),
             items: self.get_items(self.raw_text.raw_text.clone()),
             total: self.get_total_cost(self.raw_text.raw_text.clone()),
             created: Utc::now(),
             updated: Utc::now(),
             transaction_date: Utc::now(),
        }
    }

}
