use super::{Filter, FilterError};
use crate::images::Format;

#[derive(Default)]
pub struct FlipH {
    start_x: usize,
    start_y: usize,
    end_x: usize,
    end_y: usize,
    is_full: bool,
}

impl FlipH {
    pub fn full() -> Self {
        Self {
            is_full: true,
            ..Default::default()
        }
    }

    pub fn rect(start: (usize, usize), end: (usize, usize)) -> Self {
        Self {
            start_x: start.0,
            start_y: start.1,
            end_x: end.0,
            end_y: end.1,
            is_full: false,
        }
    }
}

impl Filter for FlipH {
    fn apply(&self, image: &mut impl crate::images::Image) -> Result<(), super::FilterError> {
        let start = if self.is_full {
            (0, 0)
        } else {
            (self.start_x, self.start_y)
        };

        let end = if self.is_full {
            (image.widht(), image.height())
        } else {
            (self.end_x, self.end_y)
        };

        match image.format() {
            Format::BMP => {
                for i in start.1..end.1 {
                    let start = start.0 + (i * image.widht());
                    let end = end.0 + (i * image.widht());
                    let line = image.slice_pixels(start..end);

                    line.reverse();
                }

                Ok(())
            }

            _ => Err(FilterError::InvalidFormat),
        }
    }
}
