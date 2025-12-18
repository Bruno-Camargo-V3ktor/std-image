pub mod bitmap;

// Utils Functions
pub fn u32_from_le_bytes(bytes: &[u8]) -> u32 {
    if bytes.len() == 0 {
        return 0;
    }

    let value: u32 = ((bytes[3] as u32) << 24)
        | ((bytes[2] as u32) << 16)
        | ((bytes[1] as u32) << 8)
        | (bytes[0] as u32);

    value
}

pub fn i32_from_le_bytes(bytes: &[u8]) -> i32 {
    if bytes.len() == 0 {
        return 0;
    }

    let value: i32 = ((bytes[3] as i32) << 24)
        | ((bytes[2] as i32) << 16)
        | ((bytes[1] as i32) << 8)
        | (bytes[0] as i32);

    value
}
