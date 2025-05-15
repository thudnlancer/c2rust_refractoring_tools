pub fn rotl64(x: u64, n: u32) -> u64 {
    x.rotate_left(n)
}

pub fn rotr8(x: u8, n: u32) -> u8 {
    x.rotate_right(n)
}

pub fn rotr64(x: u64, n: u32) -> u64 {
    x.rotate_right(n)
}

pub fn rotl32(x: u32, n: u32) -> u32 {
    x.rotate_left(n)
}

pub fn rotl8(x: u8, n: u32) -> u8 {
    x.rotate_left(n)
}

pub fn rotr32(x: u32, n: u32) -> u32 {
    x.rotate_right(n)
}

pub fn rotl_sz(x: usize, n: u32) -> usize {
    x.rotate_left(n)
}

pub fn rotr16(x: u16, n: u32) -> u16 {
    x.rotate_right(n)
}

pub fn rotl16(x: u16, n: u32) -> u16 {
    x.rotate_left(n)
}

pub fn rotr_sz(x: usize, n: u32) -> usize {
    x.rotate_right(n)
}