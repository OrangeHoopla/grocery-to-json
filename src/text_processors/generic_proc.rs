use crate::dao::{RawText, Reciept};


pub struct GenericProcessor {
    pub raw_text: RawText,
}

trait Store {
    fn new(name: &'static str) -> Self;
    fn get_store_name(&self) -> &'static str;
    fn get_address(&self) -> &'static str;
    fn get_items(&self) -> &'static str;
    fn get_reciept(&self) -> Reciept;
}