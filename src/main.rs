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
mod accuracy {
    use std::fs;

    use grocery_to_json::grocery_list::Item;

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

    items_test(answer_key.items,test_grocery_list.items,"wf2");
        
    assert!(test_grocery_list.location.eq(&answer_key.location));
    assert!((test_grocery_list.total == answer_key.total));

    

    }

    #[test]
    fn aldi_1() {
    let mut test: Reciept = load_image("./test/molly.JPEG");
    let answer_key_raw: String = fs::read_to_string("./test/molly.json").unwrap();

    test.crop_gray();
    test.otsu(1);
    test.apply();

    let test_grocery_list: GroceryList = test.try_into().unwrap();
    let answer_key : GroceryList = serde_json::from_str(&answer_key_raw).unwrap();

    items_test(answer_key.items,test_grocery_list.items,"aldi");
        
    assert!(test_grocery_list.location.eq(&answer_key.location));
    assert!((test_grocery_list.total == answer_key.total));
    // ICRA, IROS, CVPR, ECCV, ICCV, CoRL, or RSS
    }

    #[test]
    fn giant_1() {
    let mut test: Reciept = load_image("./test/giant.jpg");
    let answer_key_raw: String = fs::read_to_string("./test/giant.json").unwrap();

    test.crop_gray();
    test.otsu(1);
    test.apply();

    let test_grocery_list: GroceryList = test.try_into().unwrap();
    // eprintln!("{}", serde_json::to_string_pretty(&test_grocery_list).unwrap());
    let answer_key : GroceryList = serde_json::from_str(&answer_key_raw).unwrap();

    items_test(answer_key.items,test_grocery_list.items,"giant");
        
    assert!(test_grocery_list.location.eq(&answer_key.location));
    assert!((test_grocery_list.total == answer_key.total));
    // ICRA, IROS, CVPR, ECCV, ICCV, CoRL, or RSS
    }


    fn items_test(original: Vec<Item>,test: Vec<Item>, list_name: &str) {
        
        let mut found: i32 = 0;
        let mut price_match: i32 = 0;

        for item in test.iter() {
            let res = original.iter().find(|predicate| predicate.name == item.name);
            if res.is_some() {
                found +=1;
                if res.unwrap().cost == item.cost {
                    price_match +=1;
                }
            }
        }

        eprintln!("   \x1b[92m{}/{}\x1b[0m Items Found {}",found,original.len(),list_name);
        eprintln!("   \x1b[92m{}/{}\x1b[0m Costs Match {}",price_match,original.len(),list_name);

    }



}

