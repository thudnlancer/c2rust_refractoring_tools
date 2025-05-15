use std::cmp::min;

pub fn nettle_memeql_sec(a: &[u8], b: &[u8]) -> i32 {
    let n = min(a.len(), b.len());
    let mut diff = 0u8;
    
    for i in 0..n {
        diff |= a[i] ^ b[i];
    }
    
    ((diff as u32).wrapping_sub(1) >> 31) as i32
}