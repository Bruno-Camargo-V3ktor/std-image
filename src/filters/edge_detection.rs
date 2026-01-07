use super::{Filter, FilterError};

pub struct EdgeDetection {
    gray_scale: bool,
    limit: u8,
}

impl EdgeDetection {
    pub fn grayscale() -> Self {
        Self {
            gray_scale: true,
            limit: 0,
        }
    }

    pub fn limit(value: u8) -> Self {
        Self {
            gray_scale: false,
            limit: value,
        }
    }

    pub fn grayscale_with_limit(value: u8) -> Self {
        Self {
            gray_scale: true,
            limit: value,
        }
    }
}

impl Default for EdgeDetection {
    fn default() -> Self {
        Self {
            gray_scale: false,
            limit: 150,
        }
    }
}

//# # #    //# # #   //# # #
const KERNEL_TABLE: [i16; 9] = [-1, 0, 1, -2, 0, 2, -1, 0, 1];

impl Filter for EdgeDetection {
    fn apply(&self, image: &mut impl crate::images::Image) -> Result<(), FilterError> {
        let size = image.get_pixels().len();
        let widht = image.widht();

        let pixels = image
            .get_pixels()
            .iter()
            .map(|c| c.grayscale())
            .collect::<Vec<_>>();

        for index in 0..size {
            let up_row = index.saturating_sub(widht) / widht;
            let mid_row = index / widht;
            let down_row = (index + widht).clamp(index, size - 1) / widht;

            let pxs = [
                clamp_index_in_row(widht, up_row, index.saturating_sub(widht + 1)), // 0
                clamp_index_in_row(widht, up_row, index.saturating_sub(widht)),     // 1
                clamp_index_in_row(widht, up_row, index.saturating_sub(widht - 1)), // 2
                // # # #
                clamp_index_in_row(widht, mid_row, index.saturating_sub(1)), // 3
                clamp_index_in_row(widht, mid_row, index),                   // 4
                clamp_index_in_row(widht, mid_row, index + 1),               // 5
                // # # #
                clamp_index_in_row(widht, down_row, index + widht - 1), // 6
                clamp_index_in_row(widht, down_row, index + widht),     // 7
                clamp_index_in_row(widht, down_row, index + widht + 1), // 8
            ];

            let gx = (pixels[pxs[0]].red() as i16 * KERNEL_TABLE[0]
                + pixels[pxs[3]].red() as i16 * KERNEL_TABLE[3]
                + pixels[pxs[6]].red() as i16 * KERNEL_TABLE[6])
                + (pixels[pxs[2]].red() as i16 * KERNEL_TABLE[2]
                    + pixels[pxs[5]].red() as i16 * KERNEL_TABLE[5]
                    + pixels[pxs[8]].red() as i16 * KERNEL_TABLE[8]);

            let gy = (pixels[pxs[0]].red() as i16 * KERNEL_TABLE[0]
                + pixels[pxs[2]].red() as i16 * KERNEL_TABLE[3]
                + pixels[pxs[3]].red() as i16 * KERNEL_TABLE[6])
                + (pixels[pxs[6]].red() as i16 * KERNEL_TABLE[2]
                    + pixels[pxs[7]].red() as i16 * KERNEL_TABLE[5]
                    + pixels[pxs[8]].red() as i16 * KERNEL_TABLE[8]);

            let magnitude = (gx.abs() + gy.abs()).clamp(0, 255) as u8;

            let color = &mut image.pixels()[index];

            if self.gray_scale {
                if magnitude > self.limit {
                    color.set_value(magnitude);
                } else {
                    color.set_value(0);
                }
            } else {
                if magnitude > self.limit {
                    color.set_value(255);
                } else {
                    color.set_value(0);
                }
            }
        }

        Ok(())
    }
}

fn clamp_index_in_row(row_size: usize, row_index: usize, index: usize) -> usize {
    let row_start = row_size * row_index;
    let row_end = row_start + row_size - 1;

    index.clamp(row_start, row_end)
}
