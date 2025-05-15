// write-be32.rs

/// Writes big-endian 32-bit words to a byte buffer.
///
/// # Arguments
/// * `dst` - The destination byte buffer to write to
/// * `src` - The source 32-bit words to read from
///
/// # Panics
/// Panics if the length is not divisible by 4 and there are leftover bytes.
pub fn write_be32(dst: &mut [u8], src: &[u32]) {
    let length = dst.len();
    let words = length / 4;
    let leftover = length % 4;

    for (i, chunk) in dst.chunks_exact_mut(4).take(words).enumerate() {
        chunk.copy_from_slice(&src[i].to_be_bytes());
    }

    if leftover != 0 {
        let word = src[words];
        let last_chunk = &mut dst[words * 4..];
        
        match leftover {
            3 => {
                last_chunk[2] = (word >> 8) as u8;
                last_chunk[1] = (word >> 16) as u8;
                last_chunk[0] = (word >> 24) as u8;
            },
            2 => {
                last_chunk[1] = (word >> 16) as u8;
                last_chunk[0] = (word >> 24) as u8;
            },
            1 => {
                last_chunk[0] = (word >> 24) as u8;
            },
            _ => panic!("Invalid leftover bytes"),
        }
    }
}