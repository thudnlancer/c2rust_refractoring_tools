use std::ffi::CString;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;
use std::ptr;
use std::slice;
use std::io;

const STACK_BUF_SIZE: usize = 1024;

pub struct Allocator {
    pub allocate: Option<fn(usize) -> *mut u8>,
    pub reallocate: Option<fn(*mut u8, usize) -> *mut u8>,
    pub free: Option<fn(*mut u8)>,
    pub die: Option<fn(usize)>,
}

impl Default for Allocator {
    fn default() -> Self {
        Self {
            allocate: None,
            reallocate: None,
            free: None,
            die: None,
        }
    }
}

pub fn careadlinkat(
    fd: i32,
    filename: &Path,
    buffer: Option<&mut [u8]>,
    alloc: Option<&Allocator>,
    preadlinkat: impl Fn(i32, &Path, &mut [u8]) -> io::Result<usize>,
) -> io::Result<Vec<u8>> {
    let mut stack_buf = [0u8; STACK_BUF_SIZE];
    readlink_stk(fd, filename, buffer, alloc, preadlinkat, &mut stack_buf)
}

fn readlink_stk(
    fd: i32,
    filename: &Path,
    mut buffer: Option<&mut [u8]>,
    alloc: Option<&Allocator>,
    preadlinkat: impl Fn(i32, &Path, &mut [u8]) -> io::Result<usize>,
    stack_buf: &mut [u8],
) -> io::Result<Vec<u8>> {
    let alloc = alloc.unwrap_or(&Allocator::default());
    let mut buf = buffer.take().unwrap_or(stack_buf);
    let mut buf_size = buf.len();
    let buf_size_max = isize::MAX as usize;

    loop {
        match preadlinkat(fd, filename, buf) {
            Ok(link_length) => {
                if link_length < buf.len() {
                    let mut result = Vec::with_capacity(link_length + 1);
                    result.extend_from_slice(&buf[..link_length]);
                    result.push(b'\0');
                    
                    if buf.as_ptr() == stack_buf.as_ptr() {
                        if let Some(allocate) = alloc.allocate {
                            let allocated = unsafe {
                                slice::from_raw_parts_mut(allocate(result.len()), result.len())
                            };
                            allocated.copy_from_slice(&result);
                            return Ok(allocated.to_vec());
                        }
                    } else {
                        if link_length < buf_size && buf.as_ptr() != stack_buf.as_ptr() {
                            if let Some(reallocate) = alloc.reallocate {
                                let reallocated = unsafe {
                                    slice::from_raw_parts_mut(
                                        reallocate(buf.as_mut_ptr(), link_length + 1),
                                        link_length + 1,
                                    )
                                };
                                reallocated[..link_length].copy_from_slice(&buf[..link_length]);
                                reallocated[link_length] = b'\0';
                                return Ok(reallocated.to_vec());
                            }
                        }
                        buf[link_length] = b'\0';
                        return Ok(buf[..=link_length].to_vec());
                    }
                } else {
                    if buf.as_ptr() != stack_buf.as_ptr() {
                        if let Some(free) = alloc.free {
                            unsafe { free(buf.as_mut_ptr()) };
                        }
                    }
                    if buf_size_max / 2 <= buf_size {
                        return Err(io::Error::new(io::ErrorKind::Other, "File name too long"));
                    }
                    buf_size = 2 * buf_size + 1;
                    if let Some(allocate) = alloc.allocate {
                        buf = unsafe { slice::from_raw_parts_mut(allocate(buf_size), buf_size) };
                    } else {
                        return Err(io::Error::new(io::ErrorKind::Other, "Allocation failed"));
                    }
                }
            }
            Err(e) => {
                if buf.as_ptr() != stack_buf.as_ptr() {
                    if let Some(free) = alloc.free {
                        unsafe { free(buf.as_mut_ptr()) };
                    }
                }
                return Err(e);
            }
        }
    }
}