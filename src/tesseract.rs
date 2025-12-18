use rusty_tesseract::{Args, Image};
use crate::reciept::Reciept;



pub trait Tesseract  {
    fn apply(&mut self);

}

impl Tesseract for Reciept {

    fn apply(&mut self) {
        let img = Image::from_dynamic_image(&self.image).unwrap();
        let default_args = Args::default();
        let output = rusty_tesseract::image_to_string(&img, &default_args).unwrap();

        self.text = output;
        
    }
}