use std::convert::TryFrom;

pub fn write_be32(dst: &mut [u8], src: &[u32]) {
    let length = dst.len();
    let words = length / 4;
    let leftover = length % 4;

    for i in 0..words {
        let word = src[i];
        dst[i * 4] = (word >> 24) as u8;
        dst[i * 4 + 1] = (word >> 16) as u8;
        dst[i * 4 + 2] = (word >> 8) as u8;
        dst[i * 4 + 3] = word as u8;
    }

    if leftover != 0 {
        let word = src[words];
        let mut j = leftover;
        
        match leftover {
            3 => {
                j -= 1;
                dst[words * 4 + j] = (word >> 8) as u8;
            }
            2 => {
                j -= 1;
                dst[words * 4 + j] = (word >> 16) as u8;
            }
            1 => {
                j -= 1;
                dst[words * 4 + j] = (word >> 24) as u8;
            }
            _ => unreachable!(),
        }
    }
}