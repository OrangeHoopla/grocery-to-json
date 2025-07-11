use image::{DynamicImage, ImageReader};
use rusty_tesseract::{Args, Image};


pub fn hello() {
    println!("hello");
}

pub fn result(image: String) -> String {

    let dynamic_image: DynamicImage = ImageReader::open(format!("images/{}",image))
    .unwrap()
    .with_guessed_format().unwrap().decode().unwrap();

    let img = Image::from_dynamic_image(&dynamic_image).unwrap();
    let default_args = Args::default();
    let output = rusty_tesseract::image_to_string(&img, &default_args).unwrap();

    
    println!("{}",output);
    return output;
}