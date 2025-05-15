use std::alloc::{alloc, alloc_zeroed, dealloc, realloc, Layout};
use std::ffi::{CStr, CString};
use std::io::{stderr, Write};
use std::process::exit;

#[derive(Debug)]
pub enum MemoryError {
    AllocationFailed,
    ReallocationFailed,
}

pub fn xmalloc(size: usize) -> Result<*mut u8, MemoryError> {
    if size == 0 {
        return Ok(std::ptr::null_mut());
    }

    let layout = Layout::from_size_align(size, 1).unwrap();
    unsafe {
        let ptr = alloc(layout);
        if ptr.is_null() {
            let _ = writeln!(
                stderr(),
                "xmalloc(): couldn't allocate {} bytes",
                size
            );
            Err(MemoryError::AllocationFailed)
        } else {
            Ok(ptr)
        }
    }
}

pub fn xcalloc(num: usize, size: usize) -> Result<*mut u8, MemoryError> {
    if num == 0 || size == 0 {
        return Ok(std::ptr::null_mut());
    }

    let layout = Layout::from_size_align(num * size, 1).unwrap();
    unsafe {
        let ptr = alloc_zeroed(layout);
        if ptr.is_null() {
            let _ = writeln!(
                stderr(),
                "xcalloc(): couldn't allocate {} bytes",
                size
            );
            Err(MemoryError::AllocationFailed)
        } else {
            Ok(ptr)
        }
    }
}

pub fn xrealloc(ptr: *mut u8, size: usize) -> Result<*mut u8, MemoryError> {
    if ptr.is_null() {
        return xmalloc(size);
    }

    if size == 0 {
        xfree(ptr);
        return Ok(std::ptr::null_mut());
    }

    let layout = Layout::from_size_align(size, 1).unwrap();
    unsafe {
        let new_ptr = realloc(ptr, layout, size);
        if new_ptr.is_null() {
            let _ = writeln!(
                stderr(),
                "xrealloc(): couldn't reallocate {} bytes",
                size
            );
            Err(MemoryError::ReallocationFailed)
        } else {
            Ok(new_ptr)
        }
    }
}

pub fn xfree(ptr: *mut u8) {
    if !ptr.is_null() {
        let layout = Layout::new::<u8>();
        unsafe {
            dealloc(ptr, layout);
        }
    }
}

pub fn xstrdup(s: &CStr) -> Result<CString, MemoryError> {
    let len = s.to_bytes().len();
    let new_ptr = xmalloc(len + 1)?;
    
    unsafe {
        std::ptr::copy_nonoverlapping(s.as_ptr(), new_ptr as *mut i8, len + 1);
        Ok(CString::from_raw(new_ptr as *mut i8))
    }
}

fn handle_error() -> ! {
    exit(1);
}