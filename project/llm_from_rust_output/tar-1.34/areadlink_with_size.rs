use std::ffi::CString;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;
use std::io;
use std::ptr;

const STACKBUF_SIZE: usize = 128;
const INITIAL_SYMLINK_MAX: usize = 1024;
const INITIAL_LIMIT_BOUND: usize = 8 * 1024;

pub fn areadlink_with_size(file: &Path, size: Option<usize>) -> io::Result<Vec<u8>> {
    let initial_limit = if INITIAL_SYMLINK_MAX < INITIAL_LIMIT_BOUND {
        INITIAL_SYMLINK_MAX + 1
    } else {
        INITIAL_LIMIT_BOUND
    };

    let mut buf_size = match size {
        Some(0) => STACKBUF_SIZE,
        Some(s) if s < initial_limit => s + 1,
        Some(_) => initial_limit,
        None => STACKBUF_SIZE,
    };

    let file_cstr = CString::new(file.as_os_str().as_bytes())?;

    loop {
        let mut stackbuf = [0u8; STACKBUF_SIZE];
        let mut buffer = Vec::new();
        let buf: &mut [u8] = if size.is_none() && buf_size == STACKBUF_SIZE {
            &mut stackbuf[..]
        } else {
            buffer.resize(buf_size, 0);
            &mut buffer[..]
        };

        let bytes_read = unsafe {
            libc::readlink(
                file_cstr.as_ptr(),
                buf.as_mut_ptr() as *mut libc::c_char,
                buf_size,
            )
        };

        if bytes_read < 0 {
            return Err(io::Error::last_os_error());
        }

        let link_length = bytes_read as usize;
        if link_length < buf_size {
            let mut result = if buffer.is_empty() {
                let mut v = Vec::with_capacity(link_length + 1);
                v.extend_from_slice(&buf[..link_length]);
                v
            } else {
                buffer.truncate(link_length);
                buffer
            };
            result.push(0);
            return Ok(result);
        }

        if buf_size <= (usize::MAX / 2) {
            buf_size *= 2;
        } else if buf_size < usize::MAX {
            buf_size = usize::MAX;
        } else {
            return Err(io::Error::new(io::ErrorKind::Other, "buffer size limit reached"));
        }
    }
}