use grocery_to_json::{
    grocery_list::GroceryList,
    imageproc::ImageProc,
    reciept::{Reciept, Store},
    tesseract::Tesseract,
};
use clap::Parser;
use image::ImageReader;

fn main() {

    let args = Args::parse();


    let mut reciept: Reciept = ImageReader::open(args.file)
        .unwrap()
        .try_into()
        .unwrap();

    if args.store.is_some() {
        reciept.store = args.store;
    }
    
    reciept.crop_gray();
    reciept.otsu(1);
    reciept.apply();

    

    let grocery_list: GroceryList = reciept.try_into().unwrap();

    println!("{}", serde_json::to_string_pretty(&grocery_list).unwrap());
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// file to parse(has to be a png or jpg)
    #[arg(short, long)]
    file: String,

    /// If you want to use a single store parser instead of in house guess
    #[arg(short, long,value_enum)]
    store: Option<Store>,
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
        let mut priced = original.len();

        for item in test.iter() {
            let res = original.iter().find(|predicate| predicate.name == item.name);
            if res.is_some() {
                found +=1;
                if res.unwrap().cost == item.cost {
                    price_match +=1;
                }
            }
            else { priced -= 1;}
        }

        eprintln!("   \x1b[92m{}/{}\x1b[0m Items Found {}",found, original.len(), list_name);
        eprintln!("   \x1b[92m{}/{}\x1b[0m Costs Match {}",price_match, priced, list_name);

    }



}

