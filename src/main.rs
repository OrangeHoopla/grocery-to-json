use image::ImageReader;
use grocery_to_json::reciept::Reciept;



fn main() {

    let test: Reciept = ImageReader::open("./test.jpg")
    .unwrap()
    .try_into()
    .unwrap();




    println!("Hello world");
    
}

