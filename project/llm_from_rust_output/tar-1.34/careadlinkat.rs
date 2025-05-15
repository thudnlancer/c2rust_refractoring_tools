use std::ffi::CString;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;
use std::ptr;
use std::slice;
use std::io::{Error, ErrorKind};

const STACK_BUF_SIZE: usize = 1024;

pub struct Allocator {
    pub allocate: Option<fn(usize) -> Option<Box<[u8]>>>,
    pub reallocate: Option<fn(Box<[u8]>, usize) -> Option<Box<[u8]>>>,
    pub free: Option<fn(Box<[u8]>)>,
    pub die: Option<fn(usize)>,
}

impl Default for Allocator {
    fn default() -> Self {
        Self {
            allocate: Some(|size| Some(vec![0; size].into_boxed_slice())),
            reallocate: None,
            free: Some(|_| {}),
            die: None,
        }
    }
}

fn readlink_stk(
    fd: i32,
    filename: &Path,
    buffer: Option<&mut [u8]>,
    alloc: &Allocator,
    preadlinkat: impl Fn(i32, &Path, &mut [u8]) -> Result<usize, Error>,
) -> Result<Box<[u8]>, Error> {
    let mut stack_buf = [0u8; STACK_BUF_SIZE];
    let mut buf = if let Some(b) = buffer {
        b
    } else {
        &mut stack_buf
    };

    loop {
        match preadlinkat(fd, filename, buf) {
            Ok(link_length) => {
                if link_length < buf.len() {
                    let mut result = if buf.as_ptr() == stack_buf.as_ptr() {
                        let new_size = link_length + 1;
                        if let Some(alloc_fn) = alloc.allocate {
                            if let Some(mut new_buf) = alloc_fn(new_size) {
                                new_buf[..link_length].copy_from_slice(&buf[..link_length]);
                                new_buf[link_length] = 0;
                                new_buf
                            } else {
                                return Err(Error::new(ErrorKind::Other, "Allocation failed"));
                            }
                        } else {
                            let mut new_buf = vec![0u8; new_size];
                            new_buf[..link_length].copy_from_slice(&buf[..link_length]);
                            new_buf[link_length] = 0;
                            new_buf.into_boxed_slice()
                        }
                    } else {
                        if link_length + 1 < buf.len() {
                            if let Some(realloc_fn) = alloc.reallocate {
                                let old_buf = unsafe {
                                    Box::from_raw(slice::from_raw_parts_mut(
                                        buf.as_mut_ptr(),
                                        buf.len(),
                                    ))
                                };
                                if let Some(new_buf) = realloc_fn(old_buf, link_length + 1) {
                                    new_buf
                                } else {
                                    return Err(Error::new(ErrorKind::Other, "Reallocation failed"));
                                }
                            } else {
                                buf[link_length] = 0;
                                return Ok(unsafe {
                                    Box::from_raw(slice::from_raw_parts_mut(
                                        buf.as_mut_ptr(),
                                        link_length + 1,
                                    ))
                                });
                            }
                        } else {
                            buf[link_length] = 0;
                            return Ok(unsafe {
                                Box::from_raw(slice::from_raw_parts_mut(
                                    buf.as_mut_ptr(),
                                    link_length + 1,
                                ))
                            });
                        }
                    };
                    result[link_length] = 0;
                    return Ok(result);
                }
            }
            Err(e) if e.kind() == ErrorKind::InvalidInput => {
                if buf.as_ptr() != stack_buf.as_ptr() {
                    if let Some(free_fn) = alloc.free {
                        free_fn(unsafe {
                            Box::from_raw(slice::from_raw_parts_mut(
                                buf.as_mut_ptr(),
                                buf.len(),
                            ))
                        });
                    }
                }
                return Err(e);
            }
            Err(e) => return Err(e),
        }

        let new_size = if buf.len() < usize::MAX / 2 {
            buf.len() * 2 + 1
        } else if buf.len() < usize::MAX {
            usize::MAX
        } else {
            return Err(Error::new(ErrorKind::Other, "Buffer size too large"));
        };

        if buf.as_ptr() != stack_buf.as_ptr() {
            if let Some(free_fn) = alloc.free {
                free_fn(unsafe {
                    Box::from_raw(slice::from_raw_parts_mut(
                        buf.as_mut_ptr(),
                        buf.len(),
                    ))
                });
            }
        }

        buf = if let Some(alloc_fn) = alloc.allocate {
            if let Some(new_buf) = alloc_fn(new_size) {
                new_buf
            } else {
                if let Some(die_fn) = alloc.die {
                    die_fn(new_size);
                }
                return Err(Error::new(ErrorKind::Other, "Allocation failed"));
            }
        } else {
            vec![0u8; new_size].into_boxed_slice()
        };
    }
}

pub fn careadlinkat(
    fd: i32,
    filename: &Path,
    buffer: Option<&mut [u8]>,
    alloc: &Allocator,
    preadlinkat: impl Fn(i32, &Path, &mut [u8]) -> Result<usize, Error>,
) -> Result<Box<[u8]>, Error> {
    readlink_stk(fd, filename, buffer, alloc, preadlinkat)
}