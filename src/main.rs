use grocery_to_json::{aldi::Aldi, imageproc::ImageProc, reciept::Reciept, tesseract::Tesseract};
use image::ImageReader;

fn main() {
    let mut test: Reciept = ImageReader::open("./test6.png")
        .unwrap()
        .try_into()
        .unwrap();

    test.otsu();

    // let mut binding = test.image.clone();
    // let answer = imageops::crop(&mut binding, 0, 0, test.image.width()/2, test.image.height()/2);
    // let _ = answer.to_image().save("crop.png");

    let _ = test.image.save("sample.png");
    test.apply();

    println!("{}", test.text);

    let wow: Aldi = test.try_into().unwrap();
    let res = serde_json::to_string_pretty(&wow).unwrap();
    println!("{}", res);
}
