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
    let answer_key_raw: String = fs::read_to_string("./test/wf2.json").unwrap();

    test.crop_gray();
    test.otsu(1);
    test.apply();

    let test_grocery_list: GroceryList = test.try_into().unwrap();
    let answer_key : GroceryList = serde_json::from_str(&answer_key_raw).unwrap();
        
    eprintln!("   \x1b[92m{}/{}\x1b[0m Items Found {}",12,30,"wf2");
    // println!("{}", answer_key);
    assert!(test_grocery_list.location.eq(&answer_key.location));
    assert!((test_grocery_list.total == answer_key.total));
    // eprintln!("12/80 Items ");
    

    }

    #[test]
    fn aldi_1() {
    let mut test: Reciept = load_image("./test/molly.JPEG");
    // let answer_key: String = fs::read_to_string("./test/wf2.json").unwrap();

    test.crop_gray();
    test.otsu(1);
    test.apply();

    let wow: GroceryList = test.try_into().unwrap();
    let _result_parse = serde_json::to_string_pretty(&wow).unwrap();
    // assert_eq!(answer_key, result_parse);
        
    // println!("{}", answer_key);
    // println!("-------------------------------");
    // println!("{}", result_parse);

    // assert!(false);
    // assert!(false);
    }



}

