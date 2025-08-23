fn main() {
    // Run registered benchmarks.
    divan::main();
}

// Register a `fibonacci` function and benchmark it over multiple cases.
#[divan::bench(args = [1, 2, 4, 8, 16, 32])]
fn fibonacci(n: u64) -> u64 {
    if n <= 1 {
        1
    } else {
        fibonacci(n - 2) + fibonacci(n - 1)
    }
}

fn it_adds_two() {
    // let result = sobel_transform::process_frame("IMG_5722.jpg".to_string(),"result.jpeg".to_string(),1);

    // let _ = result.save("result.jpeg");
    // let fer = rusty_tesseract::Image::from_dynamic_image(&result).unwrap();
    // let default_args = rusty_tesseract::Args::default();
    // let output = rusty_tesseract::image_to_string(&fer, &default_args).unwrap();
    // fs::write("edit.txt", output).expect("Should be able to write to `/foo/tmp`");
    assert_eq!(1,1);
}