use grocery_to_json::{aldi::Aldi, imageproc::ImageProc, reciept::Reciept, tesseract::Tesseract};
use image::ImageReader;

fn main() {
    let mut test: Reciept = ImageReader::open("./test.jpg")
        .unwrap()
        .try_into()
        .unwrap();

    test.otsu();

    let _ = test.image.save("sample.png");
    test.apply();

    // println!("{}", test.text);

    let wow: Aldi = test.try_into().unwrap();
    let res = serde_json::to_string(&wow).unwrap();
    println!("{}", res);

    // let j = serde_json::to_string(&wow);
    // println!("{}",j.unwrap());
}
