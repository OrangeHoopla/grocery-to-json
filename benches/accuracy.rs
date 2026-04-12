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
let mut test: Reciept = load_image(input_file);
// let answer_key: String = fs::read_to_string("./test/wf2.json").unwrap();
// print!("hello ");
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