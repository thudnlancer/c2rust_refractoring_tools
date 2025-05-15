use std::ffi::CString;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;
use std::{io, mem, ptr};

const STACKBUF_SIZE: usize = 128;
const INITIAL_LIMIT_BOUND: usize = 8 * 1024;
const SYMLINK_MAX: usize = 1024;

pub fn areadlinkat_with_size(fd: i32, file: &Path, size: usize) -> io::Result<CString> {
    let initial_limit = if SYMLINK_MAX < INITIAL_LIMIT_BOUND {
        SYMLINK_MAX + 1
    } else {
        INITIAL_LIMIT_BOUND
    };

    let mut buf_size = if size == 0 {
        STACKBUF_SIZE
    } else if size < initial_limit {
        size + 1
    } else {
        initial_limit
    };

    let file_cstr = CString::new(file.as_os_str().as_bytes())?;

    loop {
        let mut stackbuf = [0; STACKBUF_SIZE];
        let mut buffer: Vec<u8> = if size == 0 && buf_size == STACKBUF_SIZE {
            stackbuf.to_vec()
        } else {
            vec![0; buf_size]
        };

        let r = unsafe {
            libc::readlinkat(
                fd,
                file_cstr.as_ptr(),
                buffer.as_mut_ptr() as *mut libc::c_char,
                buf_size,
            )
        };

        if r < 0 {
            return Err(io::Error::last_os_error());
        }

        let link_length = r as usize;
        if link_length < buf_size {
            buffer.truncate(link_length);
            buffer.push(0);
            return Ok(unsafe { CString::from_vec_unchecked(buffer) });
        }

        if buf_size <= isize::MAX as usize / 2 {
            buf_size *= 2;
        } else if buf_size < isize::MAX as usize {
            buf_size = isize::MAX as usize;
        } else {
            return Err(io::Error::from_raw_os_error(libc::ENOMEM));
        }
    }
}