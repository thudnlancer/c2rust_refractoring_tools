/* See LICENSE file for copyright and license details. */
use std::alloc::{self, Layout};
use std::ptr;

struct Z {
    chars: Option<Box<[u8]>>,
    alloced: usize,
}

static mut LIBZAHL_POOL: [Option<Vec<Box<[u8]>>>; 64] = [None; 64];
static mut LIBZAHL_POOL_N: [usize; 64] = [0; 64];
static mut LIBZAHL_POOL_ALLOC: [usize; 64] = [0; 64];

fn libzahl_msb_nz_zu(n: usize) -> usize {
    usize::BITS as usize - n.leading_zeros() as usize - 1
}

fn zfree(a: &mut Z) -> Result<(), alloc::LayoutError> {
    if a.chars.is_none() {
        return Ok(());
    }

    let i = libzahl_msb_nz_zu(a.alloced);
    
    unsafe {
        if LIBZAHL_POOL[i].is_none() {
            LIBZAHL_POOL[i] = Some(Vec::new());
        }
        
        let pool = LIBZAHL_POOL[i].as_mut().unwrap();
        let j = LIBZAHL_POOL_N[i];
        LIBZAHL_POOL_N[i] += 1;

        if j == LIBZAHL_POOL_ALLOC[i] {
            let x = if j != 0 { (j * 3) / 2 } else { 128 };
            pool.reserve(x);
            LIBZAHL_POOL_ALLOC[i] = pool.capacity();
        }

        if let Some(chars) = a.chars.take() {
            pool.push(chars);
        }
    }

    Ok(())
}