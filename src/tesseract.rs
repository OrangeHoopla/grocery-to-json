use crate::reciept::Reciept;



trait Tesseract  {
    fn quade(&self) -> Self;
}

impl Tesseract for Reciept {
    fn quade(&self) -> Self {
        println!("Hellow");

        Reciept { image: self.image.clone(), text: "New".to_owned() }
    }
}