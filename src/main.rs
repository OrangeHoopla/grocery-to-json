use image::ImageReader;
use grocery_to_json::{ reciept::Reciept, tesseract::Tesseract, aldi::Aldi};



fn main() {

    let mut test: Reciept = ImageReader::open("./test.jpg")
    .unwrap()
    .try_into()
    .unwrap();

    test.apply();
    println!("{}", test.text);

    let wow: Aldi = test.try_into().unwrap();
    println!("{:?}",wow);


    let j = serde_json::to_string(&wow);
    println!("{}",j.unwrap());
    
}

