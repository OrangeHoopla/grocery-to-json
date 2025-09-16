use clap::Parser;
use grocery_to_json::dao::ImageProcessor;
use grocery_to_json::dao::Reciept;
use grocery_to_json::dao::TextProcessor;
use grocery_to_json::image_processors;
use grocery_to_json::image_processors::hough_transform;
use grocery_to_json::text_processors;
use std::fs;

///
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short='i', long, default_value_t = String::from("images/sample.jpg"))]
    image: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t, value_enum)]
    textprocessor: TextProcessor,

    #[arg(short, long, default_value_t, value_enum)]
    imageprocessor: ImageProcessor,

    #[arg(short='o', long, default_value_t = String::from("result.jpg"))]
    output: String,
}

fn main() {
    hough_transform::example("images/sample.jpg");
    // let args: Args = Args::parse();
    // let raw_string = get_raw_string(&args);
    // let answer:Reciept = get_reciept_object(&args,raw_string);
    // let v = serde_json::to_string(&answer).unwrap();
    // fs::write(args.output, v).expect("Failed to write to designated file");
}

fn get_raw_string(args: &Args) -> String {
    match args.imageprocessor {
        ImageProcessor::GenericProcessor => {
            image_processors::tesseract_processor::result(&args.image)
        }
        ImageProcessor::SobelProcessor => todo!(),
    }
}

fn get_reciept_object(args: &Args, raw_string: String) -> Reciept {
    match args.textprocessor {
        TextProcessor::GenericProcessor => {
            text_processors::generic_processor::get_reciept(raw_string)
        }
        TextProcessor::AldiProcessor => todo!(),
        TextProcessor::WholeFoodsProcessor => todo!(),
    }
}
