use image::ImageReader;
use grocery_to_json::reciept::Reciept;



fn main() {
    // hough_transform::example("images/sample.jpg");
    // let args: Args = Args::parse();
    // let raw_string = get_raw_string(&args);
    // let answer:Reciept = get_reciept_object(&args,raw_string);
    // let v = serde_json::to_string(&answer).unwrap();
    // fs::write(args.output, v).expect("Failed to write to designated file");
// <ImageReader<std::io::BufReader<fs::File>>, std::io::Error>
    let test: Reciept = ImageReader::open("./test.jpg")
    .unwrap()
    .try_into()
    .unwrap();

    println!("Hello world");
    
}

