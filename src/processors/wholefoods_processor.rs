
use bson::{doc, Document};
use chrono::Utc;
use mongodb::{Client, Collection};
use regex::Regex;
use crate::dao::models::GroceryList;



// this will need a sub function as to seperate different regex and additional fields
pub fn parse(raw_reciept: String) -> GroceryList {
// get items
let re = Regex::new(r"(?m)^(.*) \$([0-9\.]+).*[F\|JT]\s").unwrap();
let re_total = Regex::new(r"Total.*:.*\$([0-9\.]+)").unwrap();
let re_address = Regex::new(r"MARKET[\r\n]+([^\r\n]+)[\r\n]+([^\r\n]+)[\r\n]+([^\r\n]+)").unwrap();
let mut items: Vec<(String, f64)> = Vec::new();
for cap in re.captures_iter(&raw_reciept) {
    items.push((
        cap[1].to_owned(), 
        cap[2].parse::<f64>().unwrap_or(0.0)));
    // println!("Item: {} \nCost:{}\n************", &cap[1], &cap[2]); 
}
let total = re_total.captures(&raw_reciept);


let real_total = match total {
    Some(x) => x,
    None => re_address.captures(&raw_reciept).unwrap(),
  };
  
let addr = re_address.captures(&raw_reciept).unwrap();
let mut address = addr[1].to_owned();
address.push(' ');
address.push_str(&addr[2]);
address.push(' ');
address.push_str(&addr[3]);


let reciept: GroceryList = GroceryList {
    store: "Whole Foods".to_owned(),
    address: address,
    total: real_total[1].parse::<f64>().unwrap_or(0.0),
    items: items,
    created: todo!(),
    updated: todo!(),
    transaction_date: todo!(),
};

return reciept;
}


pub async fn _mongo_test(_raw_reciept: String) -> mongodb::error::Result<()> {

    let client_uri = "mongodb://mongo:27017";
    let client = Client::with_uri_str(&client_uri).await?;
    let my_coll: Collection<Document> = client
        .database("test")
        .collection("Entries");

    let result = my_coll.find_one(doc! {}).await?;
    

    println!("{:#?}", result.unwrap());


    Ok(())
}
