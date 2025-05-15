use std::alloc::{alloc, alloc_zeroed, dealloc, handle_alloc_error, Layout};
use std::ptr::{copy_nonoverlapping, null_mut};
use std::os::raw::c_char;
use std::ffi::CStr;

const DEFAULT_MXFAST: usize = 128;

fn xalloc_die() -> ! {
    panic!("Memory allocation failed");
}

fn xmalloc(n: usize) -> *mut u8 {
    if n == 0 {
        return null_mut();
    }
    unsafe {
        let layout = Layout::from_size_align(n, 1).unwrap_or_else(|_| xalloc_die());
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
            unsafe {
                dealloc(p, Layout::from_size_align(1, 1).unwrap());
            }
        }
        return null_mut();
    }
    
    unsafe {
        let new_layout = Layout::from_size_align(n, 1).unwrap_or_else(|_| xalloc_die());
        let ptr = if p.is_null() {
            alloc(new_layout)
        } else {
            let old_layout = Layout::from_size_align(1, 1).unwrap();
            std::alloc::realloc(p, old_layout, new_layout.size())
        };
        
        if ptr.is_null() {
            xalloc_die();
        }
        ptr
    }
}

fn x2nrealloc(p: *mut u8, pn: &mut usize, s: usize) -> *mut u8 {
    let mut n = *pn;
    
    if p.is_null() {
        if n == 0 {
            n = DEFAULT_MXFAST / s;
            n = if n == 0 { 1 } else { n };
        }
        
        if usize::MAX / s < n {
            xalloc_die();
        }
    } else {
        if usize::MAX / 3 * 2 / s <= n {
            xalloc_die();
        }
        n = n + n / 2 + 1;
    }
    
    *pn = n;
    xrealloc(p, n * s)
}

fn x2realloc(p: *mut u8, pn: &mut usize) -> *mut u8 {
    x2nrealloc(p, pn, 1)
}

fn xcalloc(n: usize, s: usize) -> *mut u8 {
    if n == 0 || s == 0 {
        return null_mut();
    }
    
    if usize::MAX / s < n {
        xalloc_die();
    }
    
    unsafe {
        let layout = Layout::from_size_align(n * s, 1).unwrap_or_else(|_| xalloc_die());
        let ptr = alloc_zeroed(layout);
        if ptr.is_null() {
            xalloc_die();
        }
        ptr
    }
}

fn xzalloc(n: usize) -> *mut u8 {
    xcalloc(n, 1)
}

fn xmemdup(p: *const u8, s: usize) -> *mut u8 {
    if s == 0 {
        return null_mut();
    }
    
    let new_ptr = xmalloc(s);
    unsafe {
        copy_nonoverlapping(p, new_ptr, s);
    }
    new_ptr
}

fn xstrdup(string: *const c_char) -> *mut c_char {
    if string.is_null() {
        return null_mut();
    }
    
    unsafe {
        let len = CStr::from_ptr(string).to_bytes().len();
        xmemdup(string as *const u8, len + 1) as *mut c_char
    }
}