use super::Filter;

pub struct Negative;

impl Filter for Negative {
    fn apply(&self, image: &mut impl crate::images::Image) -> Result<(), super::FilterError> {
        for pixel in image.pixels() {
            pixel.set_red(255 - pixel.red());
            pixel.set_green(255 - pixel.green());
            pixel.set_blue(255 - pixel.blue());
        }

        Ok(())
    }
}
