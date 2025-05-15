use std::alloc::{self, Layout};
use std::ptr::NonNull;

pub type idx_t = isize;

#[cold]
fn alloc_nomem() -> Option<NonNull<u8>> {
    std::io::Error::last_os_error().raw_os_error().and_then(|e| {
        if e == 12 { // ENOMEM
            None
        } else {
            unsafe { std::ptr::NonNull::new(alloc::alloc(Layout::new::<u8>())) }
        }
    })
}

pub fn ireallocarray(p: Option<NonNull<u8>>, n: idx_t, s: idx_t) -> Option<NonNull<u8>> {
    if n == 0 || s == 0 {
        return ireallocarray(p, 1, 1);
    }

    let n = n as usize;
    let s = s as usize;
    
    if n.checked_mul(s).is_none() {
        return alloc_nomem();
    }

    let new_size = n * s;
    let layout = Layout::from_size_align(new_size, 1).ok()?;

    unsafe {
        match p {
            Some(ptr) => NonNull::new(alloc::realloc(ptr.as_ptr(), layout, new_size)),
            None => NonNull::new(alloc::alloc(layout)),
        }
    }
}

pub fn icalloc(n: idx_t, s: idx_t) -> Option<NonNull<u8>> {
    let n = n as usize;
    let s = s as usize;
    
    if n.checked_mul(s).is_none() {
        return alloc_nomem();
    }

    let layout = Layout::from_size_align(n * s, 1).ok()?;
    unsafe { NonNull::new(alloc::alloc_zeroed(layout)) }
}

pub fn irealloc(p: Option<NonNull<u8>>, s: idx_t) -> Option<NonNull<u8>> {
    let s = if s == 0 { 1 } else { s as usize };
    
    let layout = Layout::from_size_align(s, 1).ok()?;
    unsafe {
        match p {
            Some(ptr) => NonNull::new(alloc::realloc(ptr.as_ptr(), layout, s)),
            None => NonNull::new(alloc::alloc(layout)),
        }
    }
}

pub fn imalloc(s: idx_t) -> Option<NonNull<u8>> {
    let s = if s == 0 { 1 } else { s as usize };
    
    let layout = Layout::from_size_align(s, 1).ok()?;
    unsafe { NonNull::new(alloc::alloc(layout)) }
}