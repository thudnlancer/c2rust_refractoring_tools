use std::alloc::{alloc, alloc_zeroed, realloc, Layout};
use std::ptr::null_mut;
use std::io::Error;

#[cold]
fn alloc_nomem() -> *mut libc::c_void {
    std::io::Error::last_os_error().raw_os_error().unwrap_or(0);
    null_mut()
}

fn imalloc(s: isize) -> *mut libc::c_void {
    if s < 0 {
        return alloc_nomem();
    }
    let size = s as usize;
    unsafe {
        let layout = match Layout::from_size_align(size, 1) {
            Ok(l) => l,
            Err(_) => return alloc_nomem(),
        };
        alloc(layout) as *mut libc::c_void
    }
}

fn irealloc(p: *mut libc::c_void, s: isize) -> *mut libc::c_void {
    if s < 0 {
        return alloc_nomem();
    }
    let size = if s == 0 { 1 } else { s as usize };
    unsafe {
        if p.is_null() {
            let layout = match Layout::from_size_align(size, 1) {
                Ok(l) => l,
                Err(_) => return alloc_nomem(),
            };
            alloc(layout) as *mut libc::c_void
        } else {
            realloc(p as *mut u8, Layout::new::<u8>(), size) as *mut libc::c_void
        }
    }
}

fn icalloc(n: isize, s: isize) -> *mut libc::c_void {
    if n < 0 || s < 0 {
        return alloc_nomem();
    }
    let num = n as usize;
    let size = s as usize;
    unsafe {
        let layout = match Layout::array::<u8>(num * size) {
            Ok(l) => l,
            Err(_) => return alloc_nomem(),
        };
        alloc_zeroed(layout) as *mut libc::c_void
    }
}

fn ireallocarray(p: *mut libc::c_void, n: isize, s: isize) -> *mut libc::c_void {
    if n < 0 || s < 0 {
        return alloc_nomem();
    }
    if n == 0 || s == 0 {
        return irealloc(p, 1);
    }
    let num = n as usize;
    let size = s as usize;
    unsafe {
        if p.is_null() {
            let layout = match Layout::array::<u8>(num * size) {
                Ok(l) => l,
                Err(_) => return alloc_nomem(),
            };
            alloc(layout) as *mut libc::c_void
        } else {
            let new_size = num * size;
            realloc(p as *mut u8, Layout::new::<u8>(), new_size) as *mut libc::c_void
        }
    }
}