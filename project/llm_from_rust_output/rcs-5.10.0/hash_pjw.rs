use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

pub type SizeT = usize;

pub fn hash_pjw(x: &[u8], tablesize: SizeT) -> SizeT {
    let mut h = 0 as SizeT;
    
    for &byte in x {
        if byte == 0 {
            break;
        }
        h = (byte as SizeT)
            .wrapping_add(
                h << 9 | h >> (std::mem::size_of::<SizeT>() * 8).wrapping_sub(9)
            );
    }
    
    h.wrapping_rem(tablesize)
}