use grocery_to_json::{
    aldi::Aldi, imageproc::ImageProc, reciept::Reciept, tesseract::Tesseract, whole_foods::WholeFoods,
};
use image::ImageReader;
// ./molly.JPEG
// ./test2.jpg
fn main() {
    let mut test: Reciept = ImageReader::open("./wf1.jpg")
        .unwrap()
        .try_into()
        .unwrap();

    test.crop_gray();
    test.otsu(1);
    let _ = test.image.save("sample.png");

    test.apply();
    println!("{}", test.text);

    let wow: WholeFoods = test.try_into().unwrap();
    let res = serde_json::to_string_pretty(&wow).unwrap();
    println!("{}", res);
}
