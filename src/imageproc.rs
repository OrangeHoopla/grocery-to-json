use std::cmp;

use crate::reciept::Reciept;
use image::imageops;
use imageproc::{
    contrast::{otsu_level, threshold, ThresholdType},
    edges::canny,
    hough::{detect_lines, LineDetectionOptions, PolarLine},
    morphology::{grayscale_open, Mask},
};

pub trait ImageProc {
    fn can(&mut self);
    fn crop_gray(&mut self);
    fn otsu(&mut self, kernel_size: u8);
    fn intersection_points(
        line: PolarLine,
        image_width: u32,
        image_height: u32,
    ) -> Option<((f32, f32), (f32, f32))>;
}

impl ImageProc for Reciept {
    fn can(&mut self) {
        let find = canny(&self.image.to_luma8(), 50.0, 100.0);

        let mut inter = image::DynamicImage::ImageLuma8(find);
        inter.invert();
        self.image = inter;
    }

    fn otsu(&mut self, kernel_size: u8) {
        let gray = &self.image.grayscale();
        let otsu = otsu_level(&gray.to_luma8());
        let thres = threshold(&gray.to_luma8(), otsu, ThresholdType::Binary);
        let morph = grayscale_open(&thres, &Mask::square(kernel_size));
        self.image = image::DynamicImage::ImageLuma8(morph);
    }

    fn crop_gray(&mut self) {
        let gray = self.image.grayscale();
        let otsu = otsu_level(&gray.to_luma8());
        let thres = threshold(&gray.to_luma8(), otsu, ThresholdType::Binary);
        let morph = grayscale_open(&thres, &Mask::square(4));

        let edges = canny(&morph, 50.0, 100.0);

        let options = LineDetectionOptions {
            vote_threshold: 250,
            suppression_radius: 90,
        };

        let lines: Vec<PolarLine> = detect_lines(&edges, options);

        // let horizontal_lines: Vec<PolarLine> = lines
        //     .iter()
        //     .filter(|&&x| x.angle_in_degrees.ge(&85) && x.angle_in_degrees.le(&95))
        //     .cloned()
        //     .collect();

        let mut vertical_lines: Vec<PolarLine> = lines
            .iter()
            .filter(|&&x| x.angle_in_degrees.le(&5) || x.angle_in_degrees.ge(&360))
            .cloned()
            .collect();

        vertical_lines.sort_by_key(|x| x.r.round() as i64);
        // vertical_lines.extend(horizontal_lines);

        // will need to error handle a lot here
        let a = Self::intersection_points(
            *vertical_lines.last().unwrap(),
            self.image.width(),
            self.image.height(),
        )
        .unwrap();
        let b = Self::intersection_points(
            *vertical_lines.first().unwrap(),
            self.image.width(),
            self.image.height(),
        )
        .unwrap();
        let right = cmp::max(a.0 .0.round() as u64, a.1 .0.round() as u64);
        let left = cmp::min(b.0 .0.round() as u64, b.1 .0.round() as u64);
        let height = self.image.height();

        let answer = imageops::crop(
            &mut self.image,
            left as u32,
            0,
            (right - left) as u32,
            height,
        );

        self.image = image::DynamicImage::ImageRgba8(answer.to_image());

        //debug stuff
        // let green = Rgb::<u8>([0, 255, 0]);
        // let lines_image = draw_polar_lines(&gray.clone().into_rgb8(), &[*vertical_lines.last().unwrap(),*vertical_lines.first().unwrap()], green);
        // let _ = lines_image.save("temp.jpg");
    }

    // temp
    fn intersection_points(
        line: PolarLine,
        image_width: u32,
        image_height: u32,
    ) -> Option<((f32, f32), (f32, f32))> {
        let r = line.r;
        let m = line.angle_in_degrees;
        let w = image_width as f32;
        let h = image_height as f32;

        // Vertical line
        if m == 0 {
            return if r >= 0.0 && r <= w {
                Some(((r, 0.0), (r, h)))
            } else {
                None
            };
        }

        // Horizontal line
        if m == 90 {
            return if r >= 0.0 && r <= h {
                Some(((0.0, r), (w, r)))
            } else {
                None
            };
        }

        let theta = (m as f32).to_radians();
        let (sin, cos) = theta.sin_cos();

        let right_y = cos.mul_add(-w, r) / sin;
        let left_y = r / sin;
        let bottom_x = sin.mul_add(-h, r) / cos;
        let top_x = r / cos;

        let mut start = None;

        if right_y >= 0.0 && right_y <= h {
            let right_intersect = (w, right_y);
            if let Some(s) = start {
                return Some((s, right_intersect));
            }
            start = Some(right_intersect);
        }

        if left_y >= 0.0 && left_y <= h {
            let left_intersect = (0.0, left_y);
            if let Some(s) = start {
                return Some((s, left_intersect));
            }
            start = Some(left_intersect);
        }

        if bottom_x >= 0.0 && bottom_x <= w {
            let bottom_intersect = (bottom_x, h);
            if let Some(s) = start {
                return Some((s, bottom_intersect));
            }
            start = Some(bottom_intersect);
        }

        if top_x >= 0.0 && top_x <= w {
            let top_intersect = (top_x, 0.0);
            if let Some(s) = start {
                return Some((s, top_intersect));
            }
        }

        None
    }
}
