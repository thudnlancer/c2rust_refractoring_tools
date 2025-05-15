pub fn write_le64(dst: &mut [u8], src: &[u64]) {
    let words = dst.len() / 8;
    let leftover = dst.len() % 8;

    for i in 0..words {
        let word = src[i];
        let offset = i * 8;
        dst[offset] = (word & 0xff) as u8;
        dst[offset + 1] = ((word >> 8) & 0xff) as u8;
        dst[offset + 2] = ((word >> 16) & 0xff) as u8;
        dst[offset + 3] = ((word >> 24) & 0xff) as u8;
        dst[offset + 4] = ((word >> 32) & 0xff) as u8;
        dst[offset + 5] = ((word >> 40) & 0xff) as u8;
        dst[offset + 6] = ((word >> 48) & 0xff) as u8;
        dst[offset + 7] = ((word >> 56) & 0xff) as u8;
    }

    if leftover > 0 {
        let word = src[words];
        let offset = words * 8;
        for i in 0..leftover {
            dst[offset + i] = ((word >> (8 * i)) & 0xff) as u8;
        }
    }
}