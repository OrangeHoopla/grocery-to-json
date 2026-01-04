use crate::reciept::Reciept;
use rusty_tesseract::{Args, Image};

pub trait Tesseract {
    fn apply(&mut self);
}

impl Tesseract for Reciept {
    fn apply(&mut self) {
        let img = Image::from_dynamic_image(&self.image).unwrap();
        let mut default_args = Args::default();
        default_args.psm = Some(4);
        // default_args.dpi = Some(550); // should grab tesseracts estimation
        let output = rusty_tesseract::image_to_string(&img, &default_args).unwrap();

        self.text = output;
    }
}
