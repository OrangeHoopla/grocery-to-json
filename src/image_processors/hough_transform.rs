use std::path::Path;

use image::{open, DynamicImage, ImageDecoder, ImageReader, Luma, Rgb};
use imageproc::{edges::canny, hough::{detect_lines, draw_polar_lines, LineDetectionOptions, PolarLine}, map::map_pixels};




pub fn write_lines(image: &str) {

    let mut source = ImageReader::open(image)
    .unwrap()
    .with_guessed_format()
    .unwrap().into_decoder().unwrap();
    let orientation = source.orientation().unwrap();
    let mut dynamic_image = DynamicImage::from_decoder(source).unwrap();
    dynamic_image.apply_orientation(orientation);

    let gray = dynamic_image.into_luma8();
    let option = LineDetectionOptions{vote_threshold: 50, suppression_radius: 2 };

    let res = imageproc::hough::detect_lines(&gray, option);

    let pixel = Luma([0 as u8]);
    let im = draw_polar_lines(&gray, &res, pixel);

    let _ = im.save("wow.jpeg");

    println!("{:?}", res);

}

pub fn example(image: &str) {

    let mut decoder = ImageReader::open(format!("{}",image)).unwrap()
        .with_guessed_format().unwrap().into_decoder().unwrap();
        let orientation = decoder.orientation().unwrap();
        let mut dynamic_image = DynamicImage::from_decoder(decoder).unwrap();
        dynamic_image.apply_orientation(orientation);

        let input_image = dynamic_image.into_luma8();

    

    let output_dir = Path::new("images");

    // let input_image = open(image)
    //     .unwrap_or_else(|_| panic!("Could not load image at {:?}", image))
    //     .to_luma8();
    // Detect edges using Canny algorithm
    let edges = canny(&input_image, 50.0, 100.0);
    let canny_path = output_dir.join("canny.png");
    edges.save(&canny_path).unwrap();

    // Detect lines using Hough transform
    let options = LineDetectionOptions {
        vote_threshold: 100,
        suppression_radius: 35,
    };
    let lines: Vec<PolarLine> = detect_lines(&edges, options);

    let mut filtered_lines: Vec<PolarLine> = vec![]; 
    lines.clone().iter().for_each(|x | 
        if x.angle_in_degrees.le(&100) && x.angle_in_degrees.ge(&80) {
            filtered_lines.push(x.clone())
        });
        

    

    

    let white = Rgb::<u8>([255, 255, 255]);
    let green = Rgb::<u8>([0, 255, 0]);
    // let black = Rgb::<u8>([0, 0, 0]);
    let black = Luma([255 as u8]);
    // Convert edge image to colour

    // Draw lines on top of edge image
    let lines_image = draw_polar_lines(&edges, &filtered_lines, black);
    let lines_path = output_dir.join("lines.png");
    lines_image.save(&lines_path).unwrap();
}