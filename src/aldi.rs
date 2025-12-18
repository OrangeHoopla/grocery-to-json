use std::time::Instant;


use crate::reciept::Reciept;



#[derive(Debug, PartialEq, Clone)]
pub struct Aldi {
    pub location: String,
    pub total: f64,
    pub created: Instant,
    pub updated: Instant,
    pub transaction_date: Instant,

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
            location: todo!(), 
            total: todo!(), 
            created: todo!(), 
            updated: todo!(), 
            transaction_date: todo!() }
    }
}