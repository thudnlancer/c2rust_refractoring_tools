use std::ptr;
use std::slice;

pub fn xmemdup0(p: &[u8]) -> Box<[u8]> {
    let mut result = vec![0; p.len() + 1];
    if !p.is_empty() {
        result[..p.len()].copy_from_slice(p);
    }
    result.into_boxed_slice()
}