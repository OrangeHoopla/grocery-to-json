use image::ImageReader;
use grocery_to_json::{reciept::Reciept, tesseract::Tesseract};



fn main() {

    let mut test: Reciept = ImageReader::open("./test.jpg")
    .unwrap()
    .try_into()
    .unwrap();

    test.apply();


    println!("{}", test.text);


    println!("Hello world");
    
}

