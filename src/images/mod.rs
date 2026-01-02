use std::{io::Result as IOResult, ops::Range};

use crate::filters::{Filter, FilterError};

pub mod bitmap;

// Enums...
pub enum Format {
    BMP,
}

// Structs...
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct RGB {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: Option<u8>,
}

impl RGB {
    pub fn new(red: u8, green: u8, blue: u8, alpha: Option<u8>) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
    }

    pub fn add_red(&mut self, red: u8) {
        self.red = self.red.checked_add(red).unwrap_or(255);
    }

    pub fn add_green(&mut self, green: u8) {
        self.green = self.green.checked_add(green).unwrap_or(255);
    }

    pub fn add_blue(&mut self, blue: u8) {
        self.blue = self.blue.checked_add(blue).unwrap_or(255);
    }

    pub fn add_aplha(&mut self, alpha: u8) {
        if let Some(value) = &mut self.alpha {
            *value = value.checked_add(alpha).unwrap_or(255);
        }
    }
}

// Traits...
pub trait Image {
    fn open(path: impl Into<String>) -> IOResult<Self>
    where
        Self: Sized;

    fn save(&mut self, path: impl Into<String>) -> IOResult<()>;

    fn filter(&mut self, filter: impl Filter) -> Result<(), FilterError>;

    fn widht(&self) -> usize;

    fn height(&self) -> usize;

    fn format(&self) -> Format;

    fn bytes_per_pixels(&self) -> u16;

    fn pixels(&mut self) -> &mut [RGB];

    fn get_pixels(&self) -> &[RGB];

    fn pixel(&mut self, x: usize, y: usize) -> Option<&mut RGB>;

    fn get_pixel(&self, x: usize, y: usize) -> Option<&RGB>;

    fn slice_pixels(&mut self, range: Range<usize>) -> &mut [RGB];

    fn get_slice_pixels(&self, range: Range<usize>) -> &[RGB];
}

// Utils Functions
pub fn u32_from_le_bytes(bytes: &[u8]) -> u32 {
    if bytes.len() < 4 {
        return 0;
    }

    let value: u32 = ((bytes[3] as u32) << 24)
        | ((bytes[2] as u32) << 16)
        | ((bytes[1] as u32) << 8)
        | (bytes[0] as u32);

    value
}

pub fn i32_from_le_bytes(bytes: &[u8]) -> i32 {
    if bytes.len() < 4 {
        return 0;
    }

    let value: i32 = ((bytes[3] as i32) << 24)
        | ((bytes[2] as i32) << 16)
        | ((bytes[1] as i32) << 8)
        | (bytes[0] as i32);

    value
}
