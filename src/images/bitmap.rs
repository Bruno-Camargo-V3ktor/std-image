use super::{i32_from_le_bytes, u32_from_le_bytes};
use std::fs::File;
use std::io::{Read, Result as IOResult};
use std::usize;

pub struct Bitmap<const N: usize> {
    file_header: FileHeader,
    dib_header: DIBHeader,
    buffer: Buffer<N>,
}

impl<const N: usize> Bitmap<N> {
    pub fn open(path: impl Into<String>) -> IOResult<Bitmap<N>> {
        let path = path.into();
        let mut image = File::open(path)?;

        let file_header = FileHeader::new(&mut image);

        //let mut bitmap = Bitmap {
        //    file_header: [0; 14],
        //    dib_header: [0; 40],
        //    width: 0,
        //    height: 0,
        //};

        //let _ = image.read_exact(&mut bitmap.file_header);
        //let _ = image.read_exact(&mut bitmap.dib_header);

        //bitmap.width = i32_from_le_bytes(&bitmap.dib_header[4..8]);
        //bitmap.height = i32_from_le_bytes(&bitmap.dib_header[8..12]);

        todo!()
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

        image.read_exact(&mut extract);

        let mut identify = String::new();
        identify.push(extract[1] as char);
        identify.push(extract[0] as char);

        let size_file = u32_from_le_bytes(&mut extract[2..6]);
        let pixel_start_of = u32_from_le_bytes(&mut extract[10..]);

        Self {
            identify,
            size_file,
            pixel_start_of,
        }
    }
}

struct DIBHeader {
    pub size_header: u32,
    pub width: i32,
    pub height: i32,
    pub pixels: u16,
    pub raw_size: u32,
}

struct Buffer<const N: usize> {
    pub pixels: [u32; N],
}
