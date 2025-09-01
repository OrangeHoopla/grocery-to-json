use std::fs;
use image::{DynamicImage, ImageDecoder, ImageReader};
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short='i', long, default_value_t = String::from("images/sample.jpg"))]
    image: String,

    /// Number of times to greet
    #[arg(short='p', long, default_value_t = String::from("GenericProcessor"))]
    textprocessor: String,

    #[arg(short='p', long, default_value_t = String::from("GenericProcessor"))]
    imageprocessor: String,

    #[arg(short='o', long, default_value_t = String::from("result.jpg"))]
    output: String,
}

fn main() {
        let args: Args = Args::parse();

        let mut decoder = ImageReader::open(args.image)
        .expect("Could not find image")
        .with_guessed_format().unwrap()
        .into_decoder().unwrap();

        let orientation = decoder.orientation().unwrap();

        let mut dynamic_image = DynamicImage::
        from_decoder(decoder).unwrap();
        dynamic_image.apply_orientation(orientation);

        let img = rusty_tesseract::Image::from_dynamic_image(&dynamic_image).unwrap();
        let default_args = rusty_tesseract::Args::default();
        
        let output = rusty_tesseract::image_to_string(&img, &default_args).unwrap();
        fs::write(args.output, output).expect("Failed to write to designated file");


    
}
