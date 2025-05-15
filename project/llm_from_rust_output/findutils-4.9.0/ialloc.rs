use std::alloc::{alloc, alloc_zeroed, realloc, Layout};
use std::ptr::null_mut;
use std::num::NonZeroUsize;

#[cold]
fn alloc_nomem() -> *mut libc::c_void {
    std::io::Error::last_os_error();
    null_mut()
}

fn imalloc(s: isize) -> *mut libc::c_void {
    if s < 0 {
        return alloc_nomem();
    }
    let size = s as usize;
    if size == 0 {
        return null_mut();
    }
    unsafe {
        let layout = Layout::from_size_align_unchecked(size, 1);
        alloc(layout) as *mut libc::c_void
    }
}

fn irealloc(p: *mut libc::c_void, s: isize) -> *mut libc::c_void {
    if s < 0 {
        return alloc_nomem();
    }
    let size = s as usize;
    if p.is_null() {
        return imalloc(s);
    }
    if size == 0 {
        return null_mut();
    }
    unsafe {
        let layout = Layout::from_size_align_unchecked(size, 1);
        realloc(p as *mut u8, layout, size) as *mut libc::c_void
    }
}

fn icalloc(n: isize, s: isize) -> *mut libc::c_void {
    if n < 0 || s < 0 {
        return alloc_nomem();
    }
    let num = n as usize;
    let size = s as usize;
    if num == 0 || size == 0 {
        return null_mut();
    }
    unsafe {
        let layout = Layout::from_size_align_unchecked(num * size, 1);
        alloc_zeroed(layout) as *mut libc::c_void
    }
}

fn ireallocarray(p: *mut libc::c_void, n: isize, s: isize) -> *mut libc::c_void {
    if n < 0 || s < 0 {
        return alloc_nomem();
    }
    let num = n as usize;
    let size = s as usize;
    if num == 0 || size == 0 {
        return null_mut();
    }
    if let Some(non_zero) = NonZeroUsize::new(num.checked_mul(size).unwrap_or(usize::MAX)) {
        if p.is_null() {
            unsafe {
                let layout = Layout::from_size_align_unchecked(non_zero.get(), 1);
                alloc(layout) as *mut libc::c_void
            }
        } else {
            unsafe {
                let old_layout = Layout::from_size_align_unchecked(1, 1);
                realloc(p as *mut u8, old_layout, non_zero.get()) as *mut libc::c_void
            }
        }
    } else {
        alloc_nomem()
    }
}