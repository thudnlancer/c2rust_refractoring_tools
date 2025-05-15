const FNV_64_INIT: u64 = 0xcbf29ce484222325;
const FNV_64_PRIME: u64 = 0x100000001b3;
const FNV_32_INIT: u32 = 2166136261;
const FNV_32_PRIME: u32 = 16777619;

pub fn hash_fnv1_64(key: &[u8]) -> u32 {
    let mut hash = FNV_64_INIT;
    for &byte in key {
        hash = hash.wrapping_mul(FNV_64_PRIME);
        hash ^= byte as u64;
    }
    hash as u32
}

pub fn hash_fnv1a_64(key: &[u8]) -> u32 {
    let mut hash = FNV_64_INIT as u32;
    for &byte in key {
        hash ^= byte as u32;
        hash = hash.wrapping_mul(FNV_64_PRIME as u32);
    }
    hash
}

pub fn hash_fnv1_32(key: &[u8]) -> u32 {
    let mut hash = FNV_32_INIT;
    for &byte in key {
        hash = hash.wrapping_mul(FNV_32_PRIME);
        hash ^= byte as u32;
    }
    hash
}

pub fn hash_fnv1a_32(key: &[u8]) -> u32 {
    let mut hash = FNV_32_INIT;
    for &byte in key {
        hash ^= byte as u32;
        hash = hash.wrapping_mul(FNV_32_PRIME);
    }
    hash
}