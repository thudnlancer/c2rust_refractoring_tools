use std::alloc::{alloc, alloc_zeroed, dealloc, handle_alloc_error, Layout};
use std::cmp::{max, min};
use std::mem;
use std::ptr::{copy_nonoverlapping, null_mut};
use std::slice;

const DEFAULT_MXFAST: usize = 64 * mem::size_of::<usize>() / 4;

fn nonnull<T>(ptr: *mut T) -> *mut T {
    if ptr.is_null() {
        xalloc_die();
    }
    ptr
}

pub fn xmalloc(size: usize) -> *mut u8 {
    unsafe { nonnull(alloc(Layout::from_size_align(size, 1).unwrap())) }
}

pub fn xcharalloc(n: usize) -> *mut u8 {
    xmalloc(n)
}

pub fn xrealloc(ptr: *mut u8, size: usize) -> *mut u8 {
    if size == 0 {
        if !ptr.is_null() {
            unsafe { dealloc(ptr, Layout::from_size_align(1, 1).unwrap()) };
        }
        return xmalloc(1);
    }

    let new_ptr = unsafe {
        std::alloc::realloc(
            ptr,
            Layout::from_size_align(1, 1).unwrap(),
            size
        )
    };

    if new_ptr.is_null() && (!ptr.is_null() || size != 0) {
        xalloc_die();
    }
    new_ptr
}

pub fn xreallocarray(ptr: *mut u8, n: usize, size: usize) -> *mut u8 {
    match n.checked_mul(size) {
        Some(bytes) => xrealloc(ptr, bytes),
        None => {
            xalloc_die();
        }
    }
}

pub fn xnmalloc(n: usize, size: usize) -> *mut u8 {
    xreallocarray(null_mut(), n, size)
}

pub fn x2realloc(ptr: *mut u8, ps: &mut usize) -> *mut u8 {
    x2nrealloc(ptr, ps, 1)
}

pub fn x2nrealloc(ptr: *mut u8, pn: &mut usize, size: usize) -> *mut u8 {
    let mut n = *pn;

    if ptr.is_null() {
        if n == 0 {
            n = DEFAULT_MXFAST / size;
            if n == 0 {
                n = 1;
            }
        }
    } else {
        n = n + (n >> 1) + 1;
    }

    let new_ptr = xreallocarray(ptr, n, size);
    *pn = n;
    new_ptr
}

pub fn xpalloc(
    pa: *mut u8,
    pn: &mut usize,
    n_incr_min: usize,
    n_max: isize,
    size: usize,
) -> *mut u8 {
    let n0 = *pn;
    let mut n = n0 + (n0 >> 1);

    if n_max >= 0 && (n_max as usize) < n {
        n = n_max as usize;
    }

    let mut nbytes = match n.checked_mul(size) {
        Some(bytes) => bytes,
        None => {
            if size == 0 {
                0
            } else {
                xalloc_die();
            }
        }
    };

    if nbytes < DEFAULT_MXFAST {
        nbytes = DEFAULT_MXFAST;
        n = nbytes / size;
        nbytes = n * size;
    }

    if pa.is_null() {
        *pn = 0;
    }

    if n - n0 < n_incr_min {
        n = match n0.checked_add(n_incr_min) {
            Some(val) => val,
            None => {
                xalloc_die();
            }
        };

        if n_max >= 0 && (n_max as usize) < n {
            n = n_max as usize;
        }

        nbytes = match n.checked_mul(size) {
            Some(bytes) => bytes,
            None => {
                xalloc_die();
            }
        };
    }

    let new_ptr = xrealloc(pa, nbytes);
    *pn = n;
    new_ptr
}

pub fn xzalloc(size: usize) -> *mut u8 {
    xcalloc(size, 1)
}

pub fn xcalloc(n: usize, size: usize) -> *mut u8 {
    let layout = Layout::from_size_align(n * size, 1).unwrap();
    unsafe { nonnull(alloc_zeroed(layout)) }
}

pub fn xmemdup(p: *const u8, size: usize) -> *mut u8 {
    let new_ptr = xmalloc(size);
    unsafe {
        copy_nonoverlapping(p, new_ptr, size);
    }
    new_ptr
}

pub fn ximemdup0(p: *const u8, size: usize) -> *mut u8 {
    let new_ptr = xmalloc(size + 1);
    unsafe {
        copy_nonoverlapping(p, new_ptr, size);
        *new_ptr.add(size) = 0;
    }
    new_ptr
}

pub fn xstrdup(string: &str) -> *mut u8 {
    let len = string.len();
    let ptr = xmalloc(len + 1);
    unsafe {
        copy_nonoverlapping(string.as_ptr(), ptr, len);
        *ptr.add(len) = 0;
    }
    ptr
}

fn xalloc_die() -> ! {
    panic!("Memory allocation failed");
}