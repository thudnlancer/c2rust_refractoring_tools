use std::alloc::{alloc, alloc_zeroed, realloc, Layout};
use std::ptr::{copy_nonoverlapping, null_mut};
use std::mem::{size_of, MaybeUninit};
use std::os::raw::c_void;
use std::panic;

#[derive(Debug)]
pub enum XallocError {
    OutOfMemory,
    InvalidInput,
}

pub type XallocResult<T> = Result<T, XallocError>;

fn xalloc_die() -> ! {
    panic!("Memory allocation failed");
}

fn nonnull<T>(ptr: *mut T) -> XallocResult<*mut T> {
    if ptr.is_null() {
        xalloc_die();
    }
    Ok(ptr)
}

pub fn xmalloc(size: usize) -> XallocResult<*mut c_void> {
    let layout = Layout::from_size_align(size, 1).map_err(|_| XallocError::InvalidInput)?;
    unsafe { nonnull(alloc(layout)) }
}

pub fn xcharalloc(n: usize) -> XallocResult<*mut u8> {
    let layout = Layout::array::<u8>(n).map_err(|_| XallocError::InvalidInput)?;
    unsafe { nonnull(alloc(layout)) }
}

pub fn xrealloc(ptr: *mut c_void, size: usize) -> XallocResult<*mut c_void> {
    if size == 0 {
        return Ok(ptr);
    }
    
    let layout = Layout::from_size_align(size, 1).map_err(|_| XallocError::InvalidInput)?;
    let new_ptr = unsafe { realloc(ptr, layout, size) };
    
    if new_ptr.is_null() && !ptr.is_null() {
        xalloc_die();
    }
    Ok(new_ptr)
}

pub fn xreallocarray(ptr: *mut c_void, n: usize, size: usize) -> XallocResult<*mut c_void> {
    let total_size = n.checked_mul(size).ok_or(XallocError::InvalidInput)?;
    xrealloc(ptr, total_size)
}

pub fn xnmalloc(n: usize, size: usize) -> XallocResult<*mut c_void> {
    xreallocarray(null_mut(), n, size)
}

pub fn x2nrealloc(ptr: *mut c_void, pn: &mut usize, size: usize) -> XallocResult<*mut c_void> {
    let mut n = *pn;
    
    if ptr.is_null() {
        if n == 0 {
            const DEFAULT_MXFAST: usize = 64 * size_of::<usize>() / 4;
            n = DEFAULT_MXFAST / size;
            if n == 0 {
                n = 1;
            }
        }
    } else {
        n = n.checked_add(n >> 1).and_then(|v| v.checked_add(1))
            .ok_or(XallocError::InvalidInput)?;
    }
    
    let new_ptr = xreallocarray(ptr, n, size)?;
    *pn = n;
    Ok(new_ptr)
}

pub fn xzalloc(size: usize) -> XallocResult<*mut c_void> {
    let layout = Layout::from_size_align(size, 1).map_err(|_| XallocError::InvalidInput)?;
    unsafe { nonnull(alloc_zeroed(layout)) }
}

pub fn xcalloc(n: usize, size: usize) -> XallocResult<*mut c_void> {
    let total_size = n.checked_mul(size).ok_or(XallocError::InvalidInput)?;
    xzalloc(total_size)
}

pub fn xmemdup(src: *const c_void, size: usize) -> XallocResult<*mut c_void> {
    let dest = xmalloc(size)?;
    unsafe {
        copy_nonoverlapping(src as *const u8, dest as *mut u8, size);
    }
    Ok(dest)
}

pub fn xstrdup(string: &str) -> XallocResult<*mut u8> {
    let len = string.len();
    let ptr = xcharalloc(len + 1)?;
    unsafe {
        copy_nonoverlapping(string.as_ptr(), ptr, len);
        *ptr.add(len) = 0;
    }
    Ok(ptr)
}