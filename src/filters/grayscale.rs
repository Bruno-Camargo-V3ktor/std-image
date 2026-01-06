use super::{Filter, FilterError};

pub struct GrayScale;

impl Filter for GrayScale {
    fn apply(&self, image: &mut impl crate::images::Image) -> Result<(), FilterError> {
        for pixel in image.pixels() {
            let m = (pixel.red() as usize + pixel.green() as usize + pixel.blue() as usize) / 3;
            pixel.set_red(m as u8);
            pixel.set_green(m as u8);
            pixel.set_blue(m as u8);
        }

        Ok(())
    }
}
