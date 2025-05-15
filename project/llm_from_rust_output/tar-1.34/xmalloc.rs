use std::alloc::{alloc, alloc_zeroed, dealloc, handle_alloc_error, Layout};
use std::ptr::{copy_nonoverlapping, null_mut};
use std::mem;

const DEFAULT_MXFAST: usize = 128;

pub fn x2nrealloc<T>(p: Option<&mut Vec<T>>, pn: &mut usize, s: usize) -> Vec<T> {
    let mut n = *pn;
    
    if p.is_none() {
        if n == 0 {
            n = DEFAULT_MXFAST / s;
            n = if n == 0 { 1 } else { n };
        }
        
        if (isize::MAX as usize) / s < n {
            handle_alloc_error(Layout::array::<u8>(n * s).unwrap());
        }
    } else {
        if (usize::MAX / 3 * 2) / s <= n {
            handle_alloc_error(Layout::array::<u8>(n * s).unwrap());
        }
        n = n + (n / 2 + 1);
    }
    
    *pn = n;
    let new_size = n * s;
    
    if let Some(vec) = p {
        let mut new_vec = Vec::with_capacity(new_size);
        new_vec.extend_from_slice(vec);
        *vec = new_vec;
        vec.clone()
    } else {
        Vec::with_capacity(new_size)
    }
}

pub fn xmalloc(n: usize) -> *mut u8 {
    if n == 0 {
        return null_mut();
    }
    
    let layout = Layout::array::<u8>(n).unwrap();
    unsafe {
        let ptr = alloc(layout);
        if ptr.is_null() {
            handle_alloc_error(layout);
        }
        ptr
    }
}

pub fn xrealloc(p: *mut u8, old_size: usize, new_size: usize) -> *mut u8 {
    if new_size == 0 {
        if !p.is_null() {
            unsafe {
                dealloc(p, Layout::array::<u8>(old_size).unwrap());
            }
        }
        return null_mut();
    }
    
    let new_layout = Layout::array::<u8>(new_size).unwrap();
    let ptr = if p.is_null() {
        unsafe { alloc(new_layout) }
    } else {
        let old_layout = Layout::array::<u8>(old_size).unwrap();
        unsafe {
            let new_ptr = alloc(new_layout);
            if !new_ptr.is_null() {
                copy_nonoverlapping(p, new_ptr, old_size.min(new_size));
                dealloc(p, old_layout);
            }
            new_ptr
        }
    };
    
    if ptr.is_null() {
        handle_alloc_error(new_layout);
    }
    ptr
}

pub fn x2realloc<T>(p: Option<&mut Vec<T>>, pn: &mut usize) -> Vec<T> {
    x2nrealloc(p, pn, mem::size_of::<T>())
}

pub fn xzalloc(n: usize) -> *mut u8 {
    if n == 0 {
        return null_mut();
    }
    
    let layout = Layout::array::<u8>(n).unwrap();
    unsafe {
        let ptr = alloc_zeroed(layout);
        if ptr.is_null() {
            handle_alloc_error(layout);
        }
        ptr
    }
}

pub fn xcalloc(n: usize, s: usize) -> *mut u8 {
    if n == 0 || s == 0 {
        return null_mut();
    }
    
    let total_size = n * s;
    if (isize::MAX as usize) / s < n {
        handle_alloc_error(Layout::array::<u8>(total_size).unwrap());
    }
    
    let layout = Layout::array::<u8>(total_size).unwrap();
    unsafe {
        let ptr = alloc_zeroed(layout);
        if ptr.is_null() {
            handle_alloc_error(layout);
        }
        ptr
    }
}

pub fn xmemdup(p: *const u8, s: usize) -> *mut u8 {
    if s == 0 {
        return null_mut();
    }
    
    let new_ptr = xmalloc(s);
    unsafe {
        copy_nonoverlapping(p, new_ptr, s);
    }
    new_ptr
}

pub fn xstrdup(string: &str) -> *mut u8 {
    let len = string.len() + 1;
    let new_ptr = xmalloc(len);
    unsafe {
        copy_nonoverlapping(string.as_ptr(), new_ptr, len);
    }
    new_ptr
}