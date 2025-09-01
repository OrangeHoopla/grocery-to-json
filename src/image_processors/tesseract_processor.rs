use image::{DynamicImage, ImageDecoder, ImageReader};
use rusty_tesseract::{Args, Image};


pub fn hello() {
    println!("hello");
}

pub fn result(image: &str) -> String {

    //this should go to generic preproc and pass dynamic image in
    let mut decoder = ImageReader::open(format!("images/{}",image)).unwrap()
        .with_guessed_format().unwrap().into_decoder().unwrap();
        let orientation = decoder.orientation().unwrap();
        let mut dynamic_image = DynamicImage::from_decoder(decoder).unwrap();
        dynamic_image.apply_orientation(orientation);

    let img = Image::from_dynamic_image(&dynamic_image).unwrap();
    
    let default_args = Args::default();
    let output = rusty_tesseract::image_to_string(&img, &default_args).unwrap();

    
    println!("{}",output);
    return output;
}