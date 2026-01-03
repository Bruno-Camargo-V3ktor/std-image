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
        let divisor = (self.0 * 2 + 1) as isize;

        let pixels = image.pixels();
        let mut buffer = vec![RGB::default(); size];

        for index in 0..size {
            let mut counter = 0;
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;

            let mut new_index = (index as isize) - (self.0 as isize) - (self.0 * widht) as isize;

            for i in 0..divisor {
                new_index += i * (widht as isize);
                for j in 0..divisor {
                    new_index += j;
                    if new_index >= 0 && new_index < (size as isize) {
                        let color = &pixels[new_index as usize];
                        red += color.red() as u32;
                        green += color.green() as u32;
                        blue += color.blue() as u32;
                        counter += 1;
                    }
                }
            }

            let color = &mut buffer[index];
            color.set_red((red / counter) as u8);
            color.set_green((green / counter) as u8);
            color.set_blue((blue / counter) as u8);
        }

        for color in pixels {
            *color = buffer.remove(0);
        }

        Ok(())
    }
}
