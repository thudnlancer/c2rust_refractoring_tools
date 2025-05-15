pub fn hash_murmur(key: &[u8]) -> u32 {
    const M: u32 = 0x5bd1e995;
    const R: i32 = 24;
    
    let length = key.len();
    let seed = 0xdeadbeef_u32.wrapping_mul(length as u32);
    let mut h = seed ^ (length as u32);
    let mut i = 0;

    while i + 4 <= length {
        let mut k = u32::from_le_bytes([key[i], key[i+1], key[i+2], key[i+3]]);
        k = k.wrapping_mul(M);
        k ^= k >> R;
        k = k.wrapping_mul(M);
        h = h.wrapping_mul(M);
        h ^= k;
        i += 4;
    }

    match length - i {
        3 => {
            h ^= (key[i+2] as u32) << 16;
            h ^= (key[i+1] as u32) << 8;
            h ^= key[i] as u32;
            h = h.wrapping_mul(M);
        }
        2 => {
            h ^= (key[i+1] as u32) << 8;
            h ^= key[i] as u32;
            h = h.wrapping_mul(M);
        }
        1 => {
            h ^= key[i] as u32;
            h = h.wrapping_mul(M);
        }
        _ => {}
    }

    h ^= h >> 13;
    h = h.wrapping_mul(M);
    h ^= h >> 15;
    h
}