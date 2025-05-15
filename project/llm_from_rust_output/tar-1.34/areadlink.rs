use std::ffi::CString;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;

fn careadlinkatcwd(filename: &Path, buffer: &mut [u8]) -> Option<usize> {
    std::fs::read_link(filename)
        .ok()
        .and_then(|link_path| {
            let os_str = link_path.as_os_str();
            let bytes = os_str.as_bytes();
            if bytes.len() <= buffer.len() {
                buffer[..bytes.len()].copy_from_slice(bytes);
                Some(bytes.len())
            } else {
                None
            }
        })
}

pub fn areadlink(filename: &Path) -> Option<CString> {
    let mut buf = Vec::new();
    let mut len = 128;
    
    loop {
        buf.resize(len, 0);
        match careadlinkatcwd(filename, &mut buf) {
            Some(read_len) => {
                buf.truncate(read_len);
                return CString::new(buf).ok();
            }
            None => {
                if len > 4096 {
                    return None;
                }
                len *= 2;
            }
        }
    }
}