use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

pub type SizeT = usize;

pub fn hash_pjw(x: &[u8], tablesize: SizeT) -> SizeT {
    let mut hasher = DefaultHasher::new();
    x.hash(&mut hasher);
    (hasher.finish() as SizeT) % tablesize
}