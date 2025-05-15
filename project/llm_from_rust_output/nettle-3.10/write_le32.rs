pub fn write_le32(dst: &mut [u8], src: &[u32]) {
    let words = dst.len() / 4;
    let leftover = dst.len() % 4;

    for i in 0..words {
        let word = src[i];
        dst[i * 4] = (word & 0xFF) as u8;
        dst[i * 4 + 1] = ((word >> 8) & 0xFF) as u8;
        dst[i * 4 + 2] = ((word >> 16) & 0xFF) as u8;
        dst[i * 4 + 3] = ((word >> 24) & 0xFF) as u8;
    }

    if leftover != 0 {
        let mut word = src[words];
        for byte in &mut dst[words * 4..] {
            *byte = (word & 0xFF) as u8;
            word >>= 8;
        }
    }
}