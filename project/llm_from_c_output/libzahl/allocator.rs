/* See LICENSE file for copyright and license details. */
use std::alloc::{self, Layout};
use std::ptr;
use std::mem;

/// Represents a large integer type
pub struct Z {
    chars: Vec<zahl_char_t>,
    alloced: usize,
}

type zahl_char_t = u64; // Assuming this is the type based on common bignum implementations

static mut libzahl_pool: [[*mut zahl_char_t; POOL_SIZE]; POOL_LEVELS] = [[ptr::null_mut(); POOL_SIZE]; POOL_LEVELS];
static mut libzahl_pool_n: [usize; POOL_LEVELS] = [0; POOL_LEVELS];
const POOL_SIZE: usize = 32; // Example pool size
const POOL_LEVELS: usize = 64; // Enough for all possible allocation sizes
const ZAHL_FLUFF: usize = 1; // Additional allocation space

fn libzahl_msb_nz_zu(n: usize) -> usize {
    (usize::BITS - n.leading_zeros()) as usize - 1
}

fn likely(condition: bool) -> bool {
    condition
}

fn check(condition: bool) -> bool {
    condition
}

fn libzahl_memfailure() -> ! {
    panic!("Memory allocation failure");
}

fn zmemcpy(dest: &mut [zahl_char_t], src: &[zahl_char_t], n: usize) {
    dest[..n].copy_from_slice(&src[..n]);
}

fn libzahl_realloc(a: &mut Z, need: usize) {
    let mut new_size = 1;
    let mut i = libzahl_msb_nz_zu(need);
    new_size <<= i;
    
    if likely(new_size != need) {
        i += 1;
        new_size <<= 1;
    }

    unsafe {
        if likely(libzahl_pool_n[i] != 0) {
            libzahl_pool_n[i] -= 1;
            let new_ptr = libzahl_pool[i][libzahl_pool_n[i]];
            let new_slice = std::slice::from_raw_parts_mut(new_ptr, new_size + ZAHL_FLUFF);
            zmemcpy(new_slice, &a.chars, a.alloced);
            a.chars = Vec::from_raw_parts(new_ptr, new_size + ZAHL_FLUFF, new_size + ZAHL_FLUFF);
        } else {
            let new_layout = Layout::array::<zahl_char_t>(new_size + ZAHL_FLUFF).unwrap();
            let new_ptr = alloc::alloc(new_layout) as *mut zahl_char_t;
            
            if check(new_ptr.is_null()) {
                libzahl_memfailure();
            }
            
            let new_slice = std::slice::from_raw_parts_mut(new_ptr, new_size + ZAHL_FLUFF);
            zmemcpy(new_slice, &a.chars, a.alloced);
            a.chars = Vec::from_raw_parts(new_ptr, new_size + ZAHL_FLUFF, new_size + ZAHL_FLUFF);
        }
    }
    
    a.alloced = new_size;
}