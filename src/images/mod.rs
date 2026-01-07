use std::{io::Result as IOResult, ops::Range};

use crate::filters::{Filter, FilterError};

pub mod bitmap;

// Enums...
/// Enums que representa os possivel formatos de imagens
pub enum Format {
    BMP,
}

// Structs...
/// Struct que representa uma cor RGBa, onde alpha é opcional
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct RGB {
    red: u8,
    green: u8,
    blue: u8,
    alpha: Option<u8>,
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

    pub fn red(&self) -> u8 {
        self.red
    }

    pub fn green(&self) -> u8 {
        self.green
    }

    pub fn blue(&self) -> u8 {
        self.blue
    }

    pub fn alpha(&self) -> Option<u8> {
        self.alpha.clone()
    }

    pub fn set_red(&mut self, value: u8) {
        self.red = value;
    }

    pub fn set_green(&mut self, value: u8) {
        self.green = value;
    }

    pub fn set_blue(&mut self, value: u8) {
        self.blue = value;
    }

    pub fn set_alpha(&mut self, value: Option<u8>) {
        self.alpha = value;
    }

    pub fn add_red(&mut self, red: u8) {
        self.red = self.red.checked_add(red).unwrap_or(255);
    }

    pub fn overflowing_add_red(&mut self, value: u8) {
        self.red = self.red.overflowing_add(value).0;
    }

    pub fn add_green(&mut self, green: u8) {
        self.green = self.green.checked_add(green).unwrap_or(255);
    }

    pub fn overflowing_add_green(&mut self, value: u8) {
        self.green = self.green.overflowing_add(value).0;
    }

    pub fn add_blue(&mut self, blue: u8) {
        self.blue = self.blue.checked_add(blue).unwrap_or(255);
    }

    pub fn overflowing_add_blue(&mut self, value: u8) {
        self.blue = self.blue.overflowing_add(value).0;
    }

    pub fn add_aplha(&mut self, alpha: u8) {
        if let Some(value) = &mut self.alpha {
            *value = value.checked_add(alpha).unwrap_or(255);
        }
    }

    pub fn overflowing_add_aplha(&mut self, value: u8) {
        if let Some(alpha) = &mut self.alpha {
            *alpha = alpha.overflowing_add(value).0;
        }
    }

    pub fn sub_red(&mut self, red: u8) {
        self.red = self.red.checked_sub(red).unwrap_or(0);
    }

    pub fn sub_green(&mut self, green: u8) {
        self.green = self.green.checked_sub(green).unwrap_or(0);
    }

    pub fn sub_blue(&mut self, blue: u8) {
        self.blue = self.blue.checked_sub(blue).unwrap_or(0);
    }

    pub fn sub_aplha(&mut self, alpha: u8) {
        if let Some(value) = &mut self.alpha {
            *value = value.checked_sub(alpha).unwrap_or(0);
        }
    }

    pub fn overflowing_sub_red(&mut self, value: u8) {
        self.red = self.red.overflowing_sub(value).0;
    }

    pub fn overflowing_sub_green(&mut self, value: u8) {
        self.green = self.green.overflowing_sub(value).0;
    }

    pub fn overflowing_sub_blue(&mut self, value: u8) {
        self.blue = self.blue.overflowing_sub(value).0;
    }

    pub fn overflowing_sub_aplha(&mut self, value: u8) {
        if let Some(alpha) = &mut self.alpha {
            *alpha = alpha.overflowing_sub(value).0;
        }
    }

    pub fn grayscale(&self) -> Self {
        let m = ((self.red() as usize + self.green() as usize + self.blue() as usize) / 3) as u8;
        Self {
            red: m,
            green: m,
            blue: m,
            alpha: self.alpha.clone(),
        }
    }

    pub fn set_value(&mut self, value: u8) {
        self.red = value;
        self.green = value;
        self.blue = value;
    }
}

// Traits...
/// Trait que representa uma image generica, com os metodos que todo imagem deve ter
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
