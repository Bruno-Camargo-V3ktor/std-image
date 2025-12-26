use super::{i32_from_le_bytes, u32_from_le_bytes};
use std::fs::File;
use std::io::{Read, Result as IOResult};
use std::usize;

pub struct Bitmap {
    file_header: FileHeader,
    dib_header: DIBHeader,
    buffer: Buffer,
}

impl Bitmap {
    pub fn open(path: impl Into<String>) -> IOResult<Bitmap> {
        let path = path.into();
        let mut image = File::open(path)?;

        let file_header = FileHeader::new(&mut image);
        let dib_header = DIBHeader::new(&mut image);
        let buffer = Buffer { pixels: vec![] };

        Ok(Self {
            file_header,
            dib_header,
            buffer,
        })
    }
}

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
}

struct DIBHeader {
    pub size_header: u32,
    pub width: i32,    // 0..3
    pub height: i32,   // 4..7
    pub pixels: u16,   // 10..11
    pub raw_size: u32, // 16..19
}

impl DIBHeader {
    pub fn new(image: &mut File) -> Self {
        let mut bytes = [0_u8; 4];
        image.read_exact(&mut bytes).unwrap();

        let size_header = u32_from_le_bytes(&bytes);

        let mut bytes = vec![0_u8; (size_header - 4) as usize];
        image.read_exact(&mut bytes).unwrap();

        let width = i32_from_le_bytes(&bytes[0..=3]);
        let height = i32_from_le_bytes(&bytes[4..=7]);

        let pixels = u16::from_le_bytes([bytes[10], bytes[11]]);
        let raw_size = u32_from_le_bytes(&bytes[16..=19]);

        DIBHeader {
            size_header,
            width,
            height,
            pixels,
            raw_size,
        }
    }
}

struct Buffer {
    pub pixels: Vec<u32>,
}

#[test]
fn file_header() {
    let mut image = File::open("./images/tower.bmp").unwrap();
    let file_header = FileHeader::new(&mut image);

    assert_eq!("BM", file_header.identify);
    assert_eq!(720056, file_header.size_file);
    assert_eq!(54, file_header.pixel_start_of);
}

#[test]
fn dib_header() {
    let mut image = File::open("./images/tower.bmp").unwrap();
    let _file_header = FileHeader::new(&mut image);
    let dib_header = DIBHeader::new(&mut image);

    assert_eq!(40, dib_header.size_header);
    assert_eq!(600, dib_header.width);
    assert_eq!(-400, dib_header.height);
    assert_eq!(24, dib_header.pixels);
    assert_eq!(720002, dib_header.raw_size);
}
