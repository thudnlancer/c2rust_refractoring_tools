pub fn camellia_invert_key(nkeys: u32, dst: &mut [u64], src: &[u64]) {
    if dst.as_ptr() == src.as_ptr() {
        let last = nkeys as usize - 1;
        for i in 0..(nkeys as usize / 2) {
            dst.swap(i, last - i);
        }
    } else {
        let last = nkeys as usize - 1;
        for i in 0..nkeys as usize {
            dst[i] = src[last - i];
        }
    }
}