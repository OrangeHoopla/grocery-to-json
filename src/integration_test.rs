

#[cfg(test)]
mod tests {
    use std::fs;

    use image::{DynamicImage, ImageDecoder, ImageReader};

    use crate::image_processors::sobel_transform;


    #[test]
    fn test_add() {
        assert_eq!(4,4);
    }

    // #[test]
    fn test_bad_add() {

        
        let result = sobel_transform::process_frame("images/IMG_5722.jpg".to_string(),"images/result.jpeg".to_string(),1);

        let _ = result.save("result.jpeg");
        let fer = rusty_tesseract::Image::from_dynamic_image(&result).unwrap();
        let default_args = rusty_tesseract::Args::default();
        let output = rusty_tesseract::image_to_string(&fer, &default_args).unwrap();
        fs::write("edit.txt", output).expect("Should be able to write to `/foo/tmp`");
        

        // //original
        let mut decoder = ImageReader::open("images/IMG_5722.jpg").unwrap()
        .with_guessed_format().unwrap().into_decoder().unwrap();
        let orientation = decoder.orientation().unwrap();
        let mut dynamic_image = DynamicImage::from_decoder(decoder).unwrap();
        dynamic_image.apply_orientation(orientation);

        let img = rusty_tesseract::Image::from_dynamic_image(&dynamic_image).unwrap();
        let default_args = rusty_tesseract::Args::default();
        let output = rusty_tesseract::image_to_string(&img, &default_args).unwrap();
        fs::write("original.txt", output).expect("Should be able to write to `/foo/tmp`");
    }
}
