use image::ImageReader;
use grocery_to_json::{ aldi::Aldi, imageproc::ImageProc, reciept::Reciept, tesseract::Tesseract};



fn main() {

    let mut test: Reciept = ImageReader::open("./test.jpg")
    .unwrap()
    .try_into()
    .unwrap();

    test.can();
    let _ = test.image.save("sample.png");
    test.apply();
    
    println!("{}", test.text);

    let wow: Aldi = test.try_into().unwrap();
    println!("{:?}",wow);


    let j = serde_json::to_string(&wow);
    println!("{}",j.unwrap());
    
}

