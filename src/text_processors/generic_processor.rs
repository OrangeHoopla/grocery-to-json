use crate::dao::models::Reciept;



struct _GenericProcessor {
    result: Reciept,
}

impl _GenericProcessor {
    fn get_store_name(_name: &'static str) -> String {
        todo!()
    }

    fn get_address(&self) -> String {
        todo!()
    }

    fn get_time_of_purchase(&self) -> chrono::DateTime<chrono::Utc> {
        todo!()
    }

    fn get_total_cost(&self) -> f64 {
        todo!()
    }

    fn get_items(&self) -> Vec<(String, f64)> {
        todo!()
    }

    fn get_reciept(_raw_text: String) -> Reciept {
        todo!()
    }
}
