use grocery_to_json::{
    aldi::Aldi, imageproc::ImageProc, reciept::Reciept, tesseract::Tesseract,
};
use image::ImageReader;
// ./molly.JPEG
// ./test2.jpg
fn main() {
    let mut test: Reciept = ImageReader::open("./molly.JPEG")
        .unwrap()
        .try_into()
        .unwrap();

    test.crop_gray();
    test.otsu(1);
    let _ = test.image.save("sample.png");

    test.apply();
    println!("{}", test.text);

    let wow: Aldi = test.try_into().unwrap();
    let res = serde_json::to_string_pretty(&wow).unwrap();
    println!("{}", res);
}
