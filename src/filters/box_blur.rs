use crate::images::RGB;

use super::{Filter, FilterError};

pub struct BoxBlur(pub usize);

impl Default for BoxBlur {
    fn default() -> Self {
        Self(1)
    }
}

impl Filter for BoxBlur {
    fn apply(&self, image: &mut impl crate::images::Image) -> Result<(), FilterError> {
        let widht = image.widht();
        let height = image.height();
        let size = widht * height;
        let step = self.0 * 2 + 1;

        let mut buffer = vec![RGB::default(); size];

        for index in 0..size {
            let mut counter = 0;
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;

            let start_index = (index as isize) - (self.0 * widht) as isize;

            for i in 0..step {
                let index = start_index + (i as isize * widht as isize);
                if index < 0 || index >= size as isize {
                    continue;
                }

                let index = index as usize;
                let row_start = (index / widht) * widht;
                let row_end = row_start + widht;

                let start =
                    (index.checked_sub(self.0).unwrap_or_default()).clamp(row_start, row_end);
                let end = (index.checked_add(self.0).unwrap_or(usize::max_value()))
                    .clamp(row_start, row_end);

                if end >= size {
                    continue;
                }

                for color in image.get_slice_pixels(start..end + 1) {
                    red += color.red() as u32;
                    green += color.green() as u32;
                    blue += color.blue() as u32;
                    counter += 1;
                }
            }

            let color = &mut buffer[index];
            color.set_red((red.checked_div(counter).unwrap_or_default()) as u8);
            color.set_green((green.checked_div(counter).unwrap_or_default()) as u8);
            color.set_blue((blue.checked_div(counter).unwrap_or_default()) as u8);
        }

        for color in image.pixels() {
            *color = buffer.remove(0);
        }

        Ok(())
    }
}
