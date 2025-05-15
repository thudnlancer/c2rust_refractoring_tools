use std::ffi::CString;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;
use std::os::unix::io::RawFd;
use std::ptr;
use std::mem;
use std::io;

const AT_FDCWD: RawFd = -3041965;
const STACK_BUF_SIZE: usize = 1024;

pub struct Allocator {
    pub allocate: fn(usize) -> *mut u8,
    pub reallocate: Option<fn(*mut u8, usize) -> *mut u8>,
    pub free: fn(*mut u8),
    pub die: Option<fn(usize)>,
}

pub fn careadlinkat(
    fd: RawFd,
    filename: &Path,
    buffer: Option<&mut [u8]>,
    alloc: Option<&Allocator>,
    preadlinkat: fn(RawFd, &Path, &mut [u8]) -> io::Result<usize>,
) -> io::Result<Vec<u8>> {
    let mut stack_buf = [0u8; STACK_BUF_SIZE];
    readlink_stk(fd, filename, buffer, alloc, preadlinkat, &mut stack_buf)
}

fn readlink_stk(
    fd: RawFd,
    filename: &Path,
    buffer: Option<&mut [u8]>,
    alloc: Option<&Allocator>,
    preadlinkat: fn(RawFd, &Path, &mut [u8]) -> io::Result<usize>,
    stack_buf: &mut [u8; STACK_BUF_SIZE],
) -> io::Result<Vec<u8>> {
    let alloc = alloc.unwrap_or(&stdlib_allocator());
    let buf_size_max = if usize::MAX > isize::MAX as usize {
        isize::MAX as usize + 1
    } else {
        usize::MAX
    };

    let (mut buf, mut buf_size) = if let Some(buffer) = buffer {
        (buffer.as_mut_ptr(), buffer.len())
    } else {
        (stack_buf.as_mut_ptr(), stack_buf.len())
    };

    let mut owned_buf: Option<Vec<u8>> = None;

    loop {
        let slice = unsafe { std::slice::from_raw_parts_mut(buf, buf_size) };
        match preadlinkat(fd, filename, slice) {
            Ok(link_length) => {
                let link_size = link_length;
                if link_size < buf_size {
                    unsafe { *buf.add(link_size) = 0 };

                    if buf == stack_buf.as_mut_ptr() {
                        let mut new_buf = vec![0u8; link_size + 1];
                        new_buf[..link_size].copy_from_slice(&stack_buf[..link_size]);
                        new_buf[link_size] = 0;
                        return Ok(new_buf);
                    }

                    if link_size < buf_size && owned_buf.is_some() {
                        if let Some(reallocate) = alloc.reallocate {
                            let new_ptr = unsafe { reallocate(buf, link_size + 1) };
                            if !new_ptr.is_null() {
                                return Ok(unsafe { Vec::from_raw_parts(new_ptr, link_size + 1, link_size + 1) });
                            }
                        }
                    }

                    if owned_buf.is_none() && buf != stack_buf.as_mut_ptr() {
                        let mut vec = unsafe { Vec::from_raw_parts(buf, link_size + 1, buf_size) };
                        vec.truncate(link_size + 1);
                        return Ok(vec);
                    }

                    let mut result = Vec::with_capacity(link_size + 1);
                    result.extend_from_slice(unsafe { std::slice::from_raw_parts(buf, link_size) });
                    result.push(0);
                    return Ok(result);
                }
            }
            Err(e) => {
                if owned_buf.is_some() {
                    return Err(e);
                } else {
                    return Err(e);
                }
            }
        }

        if owned_buf.is_some() {
            owned_buf = None;
        }

        if buf_size < buf_size_max / 2 {
            buf_size = 2 * buf_size + 1;
        } else if buf_size < buf_size_max {
            buf_size = buf_size_max;
        } else if buf_size_max < usize::MAX {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "name too long"));
        } else {
            break;
        }

        let new_buf = unsafe { (alloc.allocate)(buf_size) };
        if new_buf.is_null() {
            if let Some(die) = alloc.die {
                unsafe { die(buf_size) };
            }
            return Err(io::Error::new(io::ErrorKind::OutOfMemory, "allocation failed"));
        }
        buf = new_buf;
        owned_buf = Some(unsafe { Vec::from_raw_parts(buf, 0, buf_size) });
    }

    if let Some(die) = alloc.die {
        unsafe { die(buf_size) };
    }
    Err(io::Error::new(io::ErrorKind::OutOfMemory, "allocation failed"))
}

fn stdlib_allocator() -> Allocator {
    Allocator {
        allocate: |size| unsafe { libc::malloc(size) as *mut u8 },
        reallocate: Some(|ptr, size| unsafe { libc::realloc(ptr as *mut libc::c_void, size) as *mut u8 }),
        free: |ptr| unsafe { libc::free(ptr as *mut libc::c_void) },
        die: None,
    }
}