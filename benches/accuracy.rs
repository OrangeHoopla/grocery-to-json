use std::fs;

use grocery_to_json::{grocery_list::GroceryList, imageproc::ImageProc, reciept::Reciept, tesseract::Tesseract};
use image::ImageReader;

fn main() {
    // Run registered benchmarks.
    divan::main();
}


#[divan::bench(sample_size = 1,
    sample_count = 1,
    args = ["./test/molly.JPEG"])]
fn default(input_file: &str) {

let answer_key_raw: String = fs::read_to_string("./test/wf2.json").unwrap();
let answer_key : GroceryList = serde_json::from_str(&answer_key_raw).unwrap();
print!("{:?}",answer_key);

let mut test: Reciept = load_image(input_file);
test.crop_gray();
test.otsu(1);
test.apply();

let _wow: GroceryList = test.try_into().unwrap();
    
}

fn load_image(input_file: &str) -> Reciept 
    {
        let test: Reciept = ImageReader::open(input_file)
        .unwrap()
        .try_into()
        .unwrap();

        test

    }