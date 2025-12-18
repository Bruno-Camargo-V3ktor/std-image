pub mod bitmap;

// Utils Functions
pub fn u32_from_le_bytes(bytes: &[u8]) -> u32 {
    let value: u32 = ((bytes[3] as u32) << 24)
        | ((bytes[2] as u32) << 16)
        | ((bytes[1] as u32) << 8)
        | (bytes[0] as u32);

    value
}
