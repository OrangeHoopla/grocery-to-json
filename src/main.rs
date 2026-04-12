use grocery_to_json::{
    grocery_list::GroceryList,
    imageproc::ImageProc,
    reciept::Reciept,
    tesseract::Tesseract,
};
use image::ImageReader;

fn main() {
    let mut test: Reciept = ImageReader::open("./wf2.jpg")
        .unwrap()
        .try_into()
        .unwrap();

    test.crop_gray();
    test.otsu(1);
    let _ = test.image.save("sample.png");
    test.apply();
    // test.store = Some(Store::Aldi);

    println!("{}", test.text);

    let wow: GroceryList = test.try_into().unwrap();
    let res = serde_json::to_string_pretty(&wow).unwrap();
    println!("{}", res);
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    fn load_image(input_file: &str) -> Reciept {
        let test: Reciept = ImageReader::open(input_file)
        .unwrap()
        .try_into()
        .unwrap();

        test

    }

    #[test]
    fn whole_foods_2() {
    let mut test: Reciept = load_image("./test/wf2.jpg");
    let answer_key: String = fs::read_to_string("./test/wf2.json").unwrap();

    test.crop_gray();
    test.otsu(1);
    test.apply();

    let wow: GroceryList = test.try_into().unwrap();
    let result_parse = serde_json::to_string_pretty(&wow).unwrap();
    assert_eq!(answer_key, result_parse);
        
    // println!("{}", answer_key);
    // println!("-------------------------------");
    // println!("{}", result_parse);
    }

    #[test]
    fn aldi_1() {
    let mut test: Reciept = load_image("./test/molly.JPEG");
    // let answer_key: String = fs::read_to_string("./test/wf2.json").unwrap();

    test.crop_gray();
    test.otsu(1);
    test.apply();

    let wow: GroceryList = test.try_into().unwrap();
    let result_parse = serde_json::to_string_pretty(&wow).unwrap();
    // assert_eq!(answer_key, result_parse);
        
    // println!("{}", answer_key);
    // println!("-------------------------------");
    println!("{}", result_parse);
    }
}

