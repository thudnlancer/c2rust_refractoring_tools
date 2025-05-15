use std::alloc::{alloc, alloc_zeroed, dealloc, handle_alloc_error, Layout};
use std::ptr::{copy_nonoverlapping, null_mut};
use std::mem;

const DEFAULT_MXFAST: usize = 64;

fn xalloc_die() -> ! {
    handle_alloc_error(Layout::new::<u8>());
}

fn xmalloc(n: usize) -> *mut u8 {
    if n == 0 {
        return null_mut();
    }
    unsafe {
        let layout = Layout::from_size_align_unchecked(n, 1);
        let ptr = alloc(layout);
        if ptr.is_null() {
            xalloc_die();
        }
        ptr
    }
}

fn xrealloc(p: *mut u8, n: usize) -> *mut u8 {
    if n == 0 {
        if !p.is_null() {
            unsafe { dealloc(p, Layout::from_size_align_unchecked(1, 1)) };
        }
        return null_mut();
    }
    
    unsafe {
        let new_layout = Layout::from_size_align_unchecked(n, 1);
        let new_ptr = if p.is_null() {
            alloc(new_layout)
        } else {
            let old_layout = Layout::from_size_align_unchecked(1, 1);
            std::alloc::realloc(p, old_layout, n)
        };
        
        if new_ptr.is_null() {
            xalloc_die();
        }
        new_ptr
    }
}

fn x2nrealloc(p: Option<&mut Vec<u8>>, s: usize) -> Vec<u8> {
    let mut n = if let Some(v) = &p {
        v.len()
    } else {
        0
    };

    if p.is_none() {
        if n == 0 {
            n = DEFAULT_MXFAST / s;
            n = if n == 0 { 1 } else { n };
        }
    } else {
        if (usize::MAX / 3 * 2) / s <= n {
            xalloc_die();
        }
        n += (n + 1) / 2;
    }

    let mut vec = p.map_or_else(|| Vec::with_capacity(n * s), |v| {
        let mut new = Vec::with_capacity(n * s);
        new.extend_from_slice(v);
        new
    });
    
    vec.resize(n * s, 0);
    vec
}

fn x2realloc(p: Option<&mut Vec<u8>>) -> Vec<u8> {
    x2nrealloc(p, 1)
}

fn xzalloc(s: usize) -> Vec<u8> {
    if s == 0 {
        return Vec::new();
    }
    vec![0; s]
}

fn xcalloc(n: usize, s: usize) -> Vec<u8> {
    if n == 0 || s == 0 {
        return Vec::new();
    }
    vec![0; n * s]
}

fn xmemdup(p: &[u8]) -> Vec<u8> {
    p.to_vec()
}

fn xstrdup(string: &str) -> String {
    string.to_string()
}