use std::alloc::{self, Layout};
use std::ptr::NonNull;

pub type idx_t = isize;

#[cold]
fn alloc_nomem() -> Option<NonNull<u8>> {
    std::io::Error::last_os_error().raw_os_error().and_then(|_| None)
}

pub fn ireallocarray(p: Option<NonNull<u8>>, n: idx_t, s: idx_t) -> Option<NonNull<u8>> {
    if n == 0 || s == 0 {
        return imalloc(1);
    }

    let n = n as usize;
    let s = s as usize;

    if n.checked_mul(s).is_none() {
        return alloc_nomem();
    }

    let new_size = n * s;
    if new_size == 0 {
        return imalloc(1);
    }

    let layout = Layout::from_size_align(new_size, 1).ok()?;

    unsafe {
        if let Some(ptr) = p {
            alloc::realloc(ptr.as_ptr(), layout, new_size).into()
        } else {
            alloc::alloc(layout).into()
        }
    }
}

pub fn icalloc(n: idx_t, s: idx_t) -> Option<NonNull<u8>> {
    let n = n as usize;
    let s = s as usize;

    if n.checked_mul(s).is_none() {
        return alloc_nomem();
    }

    let size = n * s;
    if size == 0 {
        return imalloc(1);
    }

    let layout = Layout::from_size_align(size, 1).ok()?;
    unsafe { alloc::alloc_zeroed(layout).into() }
}

pub fn irealloc(p: Option<NonNull<u8>>, s: idx_t) -> Option<NonNull<u8>> {
    let s = if s == 0 { 1 } else { s } as usize;

    let layout = Layout::from_size_align(s, 1).ok()?;

    unsafe {
        if let Some(ptr) = p {
            alloc::realloc(ptr.as_ptr(), layout, s).into()
        } else {
            alloc::alloc(layout).into()
        }
    }
}

pub fn imalloc(s: idx_t) -> Option<NonNull<u8>> {
    let s = if s == 0 { 1 } else { s } as usize;

    let layout = Layout::from_size_align(s, 1).ok()?;
    unsafe { alloc::alloc(layout).into() }
}