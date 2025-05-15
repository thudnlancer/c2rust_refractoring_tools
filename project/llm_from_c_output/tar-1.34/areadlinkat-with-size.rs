use std::fs;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::os::unix::ffi::OsStrExt;
use std::ffi::OsStr;
use std::io;

const SYMLINK_MAX: usize = 1024;
const INITIAL_LIMIT_BOUND: usize = 8 * 1024;
const STACKBUF_SIZE: usize = 128;

fn areadlinkat_with_size(fd: i32, file: &Path, size: usize) -> io::Result<PathBuf> {
    let symlink_max = SYMLINK_MAX;
    let initial_limit = if symlink_max < INITIAL_LIMIT_BOUND {
        symlink_max + 1
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

    loop {
        let mut stack_buf = [0u8; STACKBUF_SIZE];
        let mut heap_buf = Vec::new();
        let buf = if size == 0 && buf_size == STACKBUF_SIZE {
            &mut stack_buf[..]
        } else {
            heap_buf.resize(buf_size, 0);
            &mut heap_buf[..]
        };

        let link_length = match fs::read_link_at(fd, file, buf) {
            Ok(len) => len,
            Err(e) if e.kind() == io::ErrorKind::InvalidInput => {
                // Buffer too small, try again with larger buffer
                if buf_size <= usize::MAX / 2 {
                    buf_size *= 2;
                } else if buf_size < usize::MAX {
                    buf_size = usize::MAX;
                } else {
                    return Err(io::Error::new(io::ErrorKind::OutOfMemory, "link too large"));
                }
                continue;
            }
            Err(e) => return Err(e),
        };

        if link_length < buf.len() {
            let result = if size == 0 && buf_size == STACKBUF_SIZE {
                PathBuf::from(OsStr::from_bytes(&buf[..link_length]))
            } else {
                PathBuf::from(OsStr::from_bytes(&buf[..link_length]))
            };
            return Ok(result);
        }
    }
}