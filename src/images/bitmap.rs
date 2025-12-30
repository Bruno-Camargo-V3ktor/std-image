use super::{Image, RGB, i32_from_le_bytes, u32_from_le_bytes};
use std::fs::File;
use std::io::{Read, Result as IOResult, Write};
use std::usize;

pub struct Bitmap {
    file_header: FileHeader,
    dib_header: DIBHeader,
    surface: Surface,
}

impl Bitmap {
    fn size_in_bytes(&self) -> u32 {
        self.file_header.size_file
    }

    pub fn identify(&self) -> &str {
        &self.file_header.identify
    }
}

impl Image for Bitmap {
    fn open(path: impl Into<String>) -> IOResult<Bitmap> {
        let path = path.into();
        let mut image = File::open(path)?;

        let file_header = FileHeader::new(&mut image);
        let dib_header = DIBHeader::new(&mut image);
        let surface = Surface::new(&mut image, &dib_header);

        Ok(Self {
            file_header,
            dib_header,
            surface,
        })
    }

    fn save(&mut self, path: impl Into<String>) -> IOResult<()> {
        let mut file = File::create_new(path.into())?;

        file.write_all(&mut self.file_header.to_bytes())?;
        file.write_all(&mut self.dib_header.to_bytes())?;
        file.write_all(&mut self.surface.to_bytes())?;

        Ok(())
    }

    fn filter(
        &mut self,
        filter: impl crate::filters::Filter,
    ) -> Result<(), crate::filters::FilterError> {
        filter.apply(self)
    }

    fn widht(&self) -> usize {
        self.dib_header.width.abs() as usize
    }

    fn height(&self) -> usize {
        self.dib_header.height.abs() as usize
    }

    fn format(&self) -> super::Format {
        super::Format::BMP
    }

    fn bytes_per_pixels(&self) -> u16 {
        self.dib_header.pixels
    }

    fn pixels(&mut self) -> &mut [RGB] {
        &mut self.surface.pixels
    }

    fn get_pixels(&self) -> &[RGB] {
        &self.surface.pixels
    }

    fn pixel(&mut self, x: usize, y: usize) -> Option<&mut RGB> {
        let width = self.dib_header.width.abs() as usize;
        let height = self.dib_header.height.abs() as usize;

        if x >= width && y >= height {
            return None;
        }

        let index = (self.surface.pixels.len() - 1) - ((y * width + x) as usize);
        self.surface.pixels.get_mut(index)
    }

    fn get_pixel(&self, x: usize, y: usize) -> Option<&RGB> {
        let width = self.dib_header.width.abs() as usize;
        let height = self.dib_header.height.abs() as usize;

        if x >= width && y >= height {
            return None;
        }

        let index = (self.surface.pixels.len() - 1) - ((y * width + x) as usize);
        self.surface.pixels.get(index)
    }

    fn slice_pixels(&mut self, range: std::ops::Range<usize>) -> &mut [RGB] {
        &mut self.surface.pixels[range]
    }

    fn get_slice_pixels(&self, range: std::ops::Range<usize>) -> &[RGB] {
        &self.surface.pixels[range]
    }
}

#[derive(Debug)]
struct FileHeader {
    pub identify: String,
    pub size_file: u32,
    pub pixel_start_of: u32,
}
impl FileHeader {
    pub fn new(image: &mut File) -> Self {
        let mut extract = [0_u8; 14];

        image.read_exact(&mut extract).unwrap();

        let mut identify = String::new();
        identify.push(extract[0] as char);
        identify.push(extract[1] as char);

        let size_file = u32_from_le_bytes(&extract[2..6]);
        let pixel_start_of = u32_from_le_bytes(&extract[10..]);

        Self {
            identify,
            size_file,
            pixel_start_of,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![0; 14];

        bytes[0] = self.identify.as_bytes()[0];
        bytes[1] = self.identify.as_bytes()[1];

        let size_file_bytes = self.size_file.to_le_bytes();
        bytes[2] = size_file_bytes[0];
        bytes[3] = size_file_bytes[1];
        bytes[4] = size_file_bytes[2];
        bytes[5] = size_file_bytes[3];

        let pixel_start_of_bytes = self.pixel_start_of.to_le_bytes();
        bytes[10] = pixel_start_of_bytes[0];
        bytes[11] = pixel_start_of_bytes[1];
        bytes[12] = pixel_start_of_bytes[2];
        bytes[13] = pixel_start_of_bytes[3];

        bytes
    }
}

#[derive(Debug)]
struct DIBHeader {
    bytes: Vec<u8>,
    pub size_header: u32,
    pub width: i32,  // 0..3
    pub height: i32, // 4..7
    pub pixels: u16, // 10..11
                     //pub raw_size: u32, // 16..19
}
impl DIBHeader {
    pub fn new(image: &mut File) -> Self {
        let mut bytes: Vec<u8> = vec![0_u8; (4) as usize];
        image.read_exact(&mut bytes).unwrap();

        let size_header = u32_from_le_bytes(&bytes);

        let mut extract = vec![0_u8; (size_header - 4) as usize];
        image.read_exact(&mut extract).unwrap();

        let width = i32_from_le_bytes(&extract[0..=3]);
        let height = i32_from_le_bytes(&extract[4..=7]);

        let pixels = u16::from_le_bytes([extract[10], extract[11]]);

        bytes.append(&mut extract);
        DIBHeader {
            bytes: bytes,
            size_header,
            width,
            height,
            pixels,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = self.bytes.clone();

        let size_header_bytes = self.size_header.to_le_bytes();
        bytes[0] = size_header_bytes[0];
        bytes[1] = size_header_bytes[1];
        bytes[2] = size_header_bytes[2];
        bytes[3] = size_header_bytes[3];

        let width_bytes = self.width.to_le_bytes();
        bytes[4] = width_bytes[0];
        bytes[5] = width_bytes[1];
        bytes[6] = width_bytes[2];
        bytes[7] = width_bytes[3];

        let height_bytes = self.height.to_le_bytes();
        bytes[8] = height_bytes[0];
        bytes[9] = height_bytes[1];
        bytes[10] = height_bytes[2];
        bytes[11] = height_bytes[3];

        let pixels_bytes = self.pixels.to_le_bytes();
        bytes[14] = pixels_bytes[0];
        bytes[15] = pixels_bytes[1];

        bytes
    }
}

#[derive(Debug)]
struct Surface {
    pub padding: usize,
    pub row_size: u32,
    pub column_size: u32,
    pub alpha_channel: bool,
    pub pixels: Vec<RGB>,
}
impl Surface {
    pub fn new(image: &mut File, dib: &DIBHeader) -> Self {
        let padding = {
            let i = (dib.width * (dib.pixels as i32)) % 4;
            if i == 0 { 0 } else { (4 - i) as usize }
        };

        let total_pixels = i32::abs(dib.width * dib.height);
        let bytes_per_pixels = dib.pixels / 8;

        let mut bytes = vec![0_u8; (total_pixels * (bytes_per_pixels) as i32) as usize];
        let mut pixels = vec![RGB::default(); total_pixels as usize];

        image.read_exact(&mut bytes).unwrap();

        let mut index = bytes.len() - 1;

        for _ in 0..i32::abs(dib.height) {
            index -= padding;
            for _ in 0..i32::abs(dib.width) {
                let pixel = pixels.get_mut(index / bytes_per_pixels as usize).unwrap();
                for i in 0..bytes_per_pixels {
                    match i {
                        0 => pixel.red = bytes[index],
                        1 => pixel.green = bytes[index],
                        2 => pixel.blue = bytes[index],
                        _ => pixel.alpha = Some(bytes[index]),
                    }

                    index = index.checked_sub(1).unwrap_or(0);
                }
            }
        }

        Self {
            alpha_channel: if bytes_per_pixels == 32 { true } else { false },
            padding,
            pixels,
            row_size: dib.width.abs() as u32,
            column_size: dib.height.abs() as u32,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let total_pixels = (self.row_size * self.column_size) as usize;
        let bytes_per_pixels = if self.alpha_channel { 4 } else { 3 };

        let mut bytes = vec![0_u8; total_pixels * bytes_per_pixels];

        let mut index = bytes.len() - 1;

        for x in 0..self.row_size {
            index -= self.padding;
            for y in 0..self.column_size {
                let pixel: &RGB = self.pixels.get(index / bytes_per_pixels).unwrap();
                for i in 0..bytes_per_pixels {
                    match i {
                        0 => bytes[index] = pixel.red,
                        1 => bytes[index] = pixel.green,
                        2 => bytes[index] = pixel.blue,
                        _ => bytes[index] = pixel.alpha.unwrap_or(0),
                    }

                    index = index.checked_sub(1).unwrap_or(0);
                }
            }
        }

        bytes
    }
}
