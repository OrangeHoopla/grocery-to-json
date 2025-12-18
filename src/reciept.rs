use std::fs;

use image::{DynamicImage, ImageDecoder, ImageReader};

#[derive(Debug, PartialEq, Clone)]
pub struct Reciept {

    pub image: DynamicImage,
    pub text: String, 

}

impl TryFrom<ImageReader<std::io::BufReader<fs::File>>> for Reciept {
    type Error = ();

    fn try_from(value: ImageReader<std::io::BufReader<fs::File>>) -> Result<Self, Self::Error> {
        
        let mut decoder = value
        .with_guessed_format()
        .unwrap()
        .into_decoder()
        .unwrap();

        let orientation = decoder.orientation().unwrap();

        let mut image = DynamicImage::from_decoder(decoder).unwrap();
        image.apply_orientation(orientation);


        Ok(Reciept{ image: image, text: "".to_owned() })

    }
}