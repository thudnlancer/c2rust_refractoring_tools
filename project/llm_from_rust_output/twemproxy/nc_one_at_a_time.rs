pub fn hash_one_at_a_time(key: &[u8]) -> u32 {
    let mut value: u32 = 0;

    for &val in key {
        value = value.wrapping_add(val as u32);
        value = value.wrapping_add(value << 10);
        value ^= value >> 6;
    }

    value = value.wrapping_add(value << 3);
    value ^= value >> 11;
    value = value.wrapping_add(value << 15);
    
    value
}