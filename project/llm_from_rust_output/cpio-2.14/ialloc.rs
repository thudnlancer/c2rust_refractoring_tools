use std::alloc::{self, Layout};
use std::ptr;

pub type idx_t = isize;

#[cold]
fn alloc_nomem() -> *mut std::ffi::c_void {
    std::io::Error::last_os_error().raw_os_error().unwrap();
    ptr::null_mut()
}

pub fn ireallocarray(p: *mut std::ffi::c_void, n: idx_t, s: idx_t) -> *mut std::ffi::c_void {
    if n == 0 || s == 0 {
        return ireallocarray(p, 1, 1);
    }

    if n as u64 > usize::MAX as u64 || s as u64 > usize::MAX as u64 {
        return alloc_nomem();
    }

    let new_size = n as usize * s as usize;
    if new_size == 0 {
        return ireallocarray(p, 1, 1);
    }

    if p.is_null() {
        let layout = Layout::from_size_align(new_size, 1).unwrap();
        unsafe { alloc::alloc(layout) as *mut std::ffi::c_void }
    } else {
        let old_layout = Layout::from_size_align(1, 1).unwrap();
        unsafe { alloc::realloc(p as *mut u8, old_layout, new_size) as *mut std::ffi::c_void }
    }
}

pub fn icalloc(n: idx_t, s: idx_t) -> *mut std::ffi::c_void {
    if n as u64 > usize::MAX as u64 {
        if s != 0 {
            return alloc_nomem();
        }
        return icalloc(0, s);
    }

    if s as u64 > usize::MAX as u64 {
        if n != 0 {
            return alloc_nomem();
        }
        return icalloc(n, 0);
    }

    let size = n as usize * s as usize;
    if size == 0 {
        return icalloc(1, 1);
    }

    let layout = Layout::from_size_align(size, 1).unwrap();
    unsafe { alloc::alloc_zeroed(layout) as *mut std::ffi::c_void }
}

pub fn irealloc(p: *mut std::ffi::c_void, s: idx_t) -> *mut std::ffi::c_void {
    if s as u64 > usize::MAX as u64 {
        return alloc_nomem();
    }

    let new_size = if s == 0 { 1 } else { s as usize };
    if p.is_null() {
        let layout = Layout::from_size_align(new_size, 1).unwrap();
        unsafe { alloc::alloc(layout) as *mut std::ffi::c_void }
    } else {
        let old_layout = Layout::from_size_align(1, 1).unwrap();
        unsafe { alloc::realloc(p as *mut u8, old_layout, new_size) as *mut std::ffi::c_void }
    }
}

pub fn imalloc(s: idx_t) -> *mut std::ffi::c_void {
    if s as u64 > usize::MAX as u64 {
        return alloc_nomem();
    }

    let size = if s == 0 { 1 } else { s as usize };
    let layout = Layout::from_size_align(size, 1).unwrap();
    unsafe { alloc::alloc(layout) as *mut std::ffi::c_void }
}