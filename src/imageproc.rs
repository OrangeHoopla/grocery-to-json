use crate::reciept::Reciept;
use image::Rgb;
use imageproc::hough::draw_polar_lines;
use imageproc::{
    contrast::{otsu_level, threshold, ThresholdType},
    edges::canny,
    hough::{detect_lines, LineDetectionOptions, PolarLine},
    morphology::{grayscale_open, Mask},
};

pub trait ImageProc {
    fn can(&mut self);
    fn crop_gray(&mut self);
    fn otsu(&mut self);
}

impl ImageProc for Reciept {
    fn can(&mut self) {
        let find = canny(&self.image.to_luma8(), 50.0, 100.0);

        let mut inter = image::DynamicImage::ImageLuma8(find);
        inter.invert();
        self.image = inter;
    }

    fn otsu(&mut self) {
        let gray = &self.image.grayscale();
        let otsu = otsu_level(&gray.to_luma8());
        let thres = threshold(&gray.to_luma8(), otsu, ThresholdType::Binary);
        let morph = grayscale_open(&thres, &Mask::square(1));
        self.image = image::DynamicImage::ImageLuma8(morph);
    }

    fn crop_gray(&mut self) {
        let gray = &self.image.grayscale();

        // let otsu = otsu_level(gray);
        // println!("{}", otsu);
        // let thres = threshold(gray, 200, ThresholdType::Binary);

        // let morph = grayscale_open(&thres, &Mask::square(3));
        let edges = canny(&self.image.to_luma8(), 50.0, 100.0);

        let options = LineDetectionOptions {
            vote_threshold: 170,
            suppression_radius: 90,
        };

        let lines: Vec<PolarLine> = detect_lines(&edges, options);

        let horizontal_lines: Vec<PolarLine> = lines
            .iter()
            .filter(|&&x| x.angle_in_degrees.ge(&85) && x.angle_in_degrees.le(&95))
            .cloned()
            .collect();

        let mut vertical_lines: Vec<PolarLine> = lines
            .iter()
            .filter(|&&x| x.angle_in_degrees.le(&5) || x.angle_in_degrees.ge(&360))
            .cloned()
            .collect();

        vertical_lines.extend(horizontal_lines);

        println!("{:?}", vertical_lines);

        let green = Rgb::<u8>([0, 255, 0]);
        // Draw lines on top of edge image
        let lines_image = draw_polar_lines(&gray.clone().into_rgb8(), &vertical_lines, green);

        self.image = image::DynamicImage::ImageRgb8(lines_image);
    }
}
