use std::ffi::CString;
use std::os::unix::prelude::*;
use std::path::Path;
use std::ptr;
use std::mem;
use std::io;
use libc::{c_char, c_int, size_t, ssize_t};

pub const AT_FDCWD: c_int = -100; // Standard value in Linux

pub struct Allocator {
    pub allocate: fn(size: usize) -> *mut c_char,
    pub reallocate: Option<fn(ptr: *mut c_char, size: usize) -> *mut c_char>,
    pub free: fn(ptr: *mut c_char),
    pub die: Option<fn(size: usize)>,
}

pub fn stdlib_allocator() -> Allocator {
    Allocator {
        allocate: |size| unsafe { libc::malloc(size) as *mut c_char },
        reallocate: Some(|ptr, size| unsafe { libc::realloc(ptr as *mut libc::c_void, size) as *mut c_char }),
        free: |ptr| unsafe { libc::free(ptr as *mut libc::c_void) },
        die: None,
    }
}

const STACK_BUF_SIZE: usize = 1024;

fn readlink_stk(
    fd: c_int,
    filename: &Path,
    buffer: Option<&mut [u8]>,
    alloc: Option<&Allocator>,
    preadlinkat: fn(c_int, *const c_char, *mut c_char, size_t) -> ssize_t,
    stack_buf: &mut [u8; STACK_BUF_SIZE],
) -> io::Result<Vec<u8>> {
    let alloc = alloc.unwrap_or(&stdlib_allocator());
    
    let mut buf = match buffer {
        Some(b) => b.to_vec(),
        None => stack_buf.to_vec(),
    };
    
    let mut buf_size = buf.len().min(ssize_t::MAX as usize).min(size_t::MAX as usize);
    
    loop {
        let c_filename = CString::new(filename.as_os_str().as_bytes())?;
        let link_length = unsafe {
            preadlinkat(
                fd,
                c_filename.as_ptr(),
                buf.as_mut_ptr() as *mut c_char,
                buf_size,
            )
        };
        
        if link_length < 0 {
            return Err(io::Error::last_os_error());
        }
        
        let link_size = link_length as usize;
        
        if link_size < buf_size {
            if link_size >= buf.len() {
                buf.resize(link_size + 1, 0);
            }
            buf[link_size] = b'\0';
            
            if buffer.is_none() && buf.as_ptr() == stack_buf.as_ptr() {
                let mut new_buf = Vec::with_capacity(link_size + 1);
                new_buf.extend_from_slice(&buf[..link_size + 1]);
                return Ok(new_buf);
            }
            
            if link_size < buf_size && buffer.is_none() {
                if let Some(reallocate) = alloc.reallocate {
                    let new_ptr = unsafe {
                        reallocate(buf.as_mut_ptr() as *mut c_char, link_size + 1)
                    };
                    if !new_ptr.is_null() {
                        let new_buf = unsafe {
                            Vec::from_raw_parts(new_ptr as *mut u8, link_size + 1, link_size + 1)
                        };
                        return Ok(new_buf);
                    }
                }
            }
            
            buf.truncate(link_size + 1);
            return Ok(buf);
        }
        
        if buffer.is_none() {
            buf = Vec::new();
        }
        
        if buf_size > (ssize_t::MAX as usize) / 2 {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "symbolic link too long"));
        }
        
        buf_size = buf_size * 2 + 1;
        buf = vec![0; buf_size];
    }
}

pub fn careadlinkat(
    fd: c_int,
    filename: &Path,
    buffer: Option<&mut [u8]>,
    alloc: Option<&Allocator>,
    preadlinkat: fn(c_int, *const c_char, *mut c_char, size_t) -> ssize_t,
) -> io::Result<Vec<u8>> {
    let mut stack_buf = [0u8; STACK_BUF_SIZE];
    readlink_stk(fd, filename, buffer, alloc, preadlinkat, &mut stack_buf)
}