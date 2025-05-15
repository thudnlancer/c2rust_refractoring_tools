pub fn u8_cpy(dest: &mut [u8], src: &[u8]) -> &mut [u8] {
    let n = src.len().min(dest.len());
    if n > 0 {
        dest[..n].copy_from_slice(&src[..n]);
    }
    dest
}