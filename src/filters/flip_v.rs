use super::{Filter, FilterError};
use crate::images::Format;

#[derive(Default)]
pub struct FlipV {
    start_x: usize,
    start_y: usize,
    end_x: usize,
    end_y: usize,
    full: bool,
}

impl FlipV {
    pub fn full() -> Self {
        Self {
            full: true,
            ..Default::default()
        }
    }

    pub fn rect(start: (usize, usize), end: (usize, usize)) -> Self {
        Self {
            start_x: start.0,
            start_y: start.1,

            end_x: end.0,
            end_y: end.1,

            full: false,
        }
    }
}

impl Filter for FlipV {
    fn apply(&self, image: &mut impl crate::images::Image) -> Result<(), FilterError> {
        let start = if self.full {
            (0, 0)
        } else {
            (self.start_x, self.start_y)
        };

        let end = if self.full {
            (image.widht(), image.height())
        } else {
            (self.end_x, self.end_y)
        };

        match image.format() {
            Format::BMP => {
                for x in start.0..end.0 {
                    let mut column = Vec::with_capacity(end.1 - start.1);

                    for y in start.1..end.1 {
                        let index = (y * image.widht()) + x;
                        column.push(image.get_pixels()[index].clone());
                    }

                    column.reverse();

                    for y in start.1..end.1 {
                        let index = (y * image.widht()) + x;
                        image.pixels()[index] = column.remove(0);
                    }
                }
            }

            _ => {
                return Err(FilterError::InvalidFormat);
            }
        }

        Ok(())
    }
}
