use crate::reciept::Reciept;
use imageproc::edges::canny;



pub trait ImageProc  {
    fn can(&mut self);

}

impl ImageProc for Reciept {

    fn can(&mut self) {
        let find = canny(&self.image.to_luma8(), 50.0, 100.0);

        self.image = image::DynamicImage::ImageLuma8(find);
        
    }
}