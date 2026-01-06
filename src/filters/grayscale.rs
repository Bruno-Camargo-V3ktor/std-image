use super::{Filter, FilterError};

pub struct GrayScale;

impl Filter for GrayScale {
    fn apply(&self, image: &mut impl crate::images::Image) -> Result<(), FilterError> {
        for pixel in image.pixels() {
            let new_color = pixel.grayscale();
            *pixel = new_color;
        }

        Ok(())
    }
}
