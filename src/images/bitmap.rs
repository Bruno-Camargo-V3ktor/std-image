use std::fs::File;
use std::io::{Read, Result as IOResult};

use super::i32_from_le_bytes;

pub struct Bitmap {
    file_header: [u8; 14],
    dib_header: [u8; 40],
    pub width: i32,
    pub height: i32,
}

impl Bitmap {
    pub fn open(path: impl Into<String>) -> IOResult<Bitmap> {
        let path = path.into();
        let mut image = File::open(path)?;

        let mut bitmap = Bitmap {
            file_header: [0; 14],
            dib_header: [0; 40],
            width: 0,
            height: 0,
        };

        let _ = image.read_exact(&mut bitmap.file_header);
        let _ = image.read_exact(&mut bitmap.dib_header);

        bitmap.width = i32_from_le_bytes(&bitmap.dib_header[4..8]);
        bitmap.height = i32_from_le_bytes(&bitmap.dib_header[8..12]);

        Ok(bitmap)
    }
}
