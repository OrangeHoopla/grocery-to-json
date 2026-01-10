use std::{fmt, fs};

use image::{DynamicImage, ImageDecoder, ImageReader};
use regex::Regex;

#[derive(Debug, PartialEq, Clone)]
pub struct Reciept {
    pub store: Option<Store>,
    pub image: DynamicImage,
    pub text: String,
}

impl TryFrom<ImageReader<std::io::BufReader<fs::File>>> for Reciept {
    type Error = ();

    fn try_from(value: ImageReader<std::io::BufReader<fs::File>>) -> Result<Self, Self::Error> {
        let mut decoder = value.with_guessed_format().unwrap().into_decoder().unwrap();

        let orientation = decoder.orientation().unwrap();

        let mut image = DynamicImage::from_decoder(decoder).unwrap();
        image.apply_orientation(orientation);

        Ok(Reciept {
            store: None,
            image: image,
            text: "".to_owned(),
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Store {
    Aldi,
    Trader_Joes,
    Whole_Foods,
    Giant,
}

impl fmt::Display for Store {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Reciept {
    // should only ever be called when the text field has been placed
    pub fn guess_store(&self) -> Store {
        for variant in vec![
            Store::Aldi,
            Store::Giant,
            Store::Trader_Joes,
            Store::Whole_Foods,
        ] 
        {
            let matcher = variant.to_string().replace("_", " ");
            let mut case_insensitive = "(?i)".to_owned();
            case_insensitive.push_str(&matcher);

            let re = Regex::new(&case_insensitive).unwrap();

            if re.is_match(&self.text) {
                let mut x = "Guessed its ".to_owned();
                x.push_str(&variant.to_string());
                println!("{}", x);
                return variant;
            }
        }
        println!("Failed to Guess Successfully");
        Store::Aldi
    }
}
