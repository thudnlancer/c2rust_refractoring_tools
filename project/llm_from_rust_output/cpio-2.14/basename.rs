use std::ffi::{CStr, CString};
use std::path::{Path, Component};

pub fn base_name(name: &str) -> Option<CString> {
    let path = Path::new(name);
    let last_component = path.components().last().unwrap_or(Component::RootDir);
    
    let (base_str, length) = if last_component != Component::RootDir {
        let base = last_component.as_os_str().to_str()?;
        let len = base.len();
        let additional_slash = if base.ends_with('/') { 1 } else { 0 };
        (base, len + additional_slash)
    } else {
        let base = path.to_str()?;
        let len = base.len();
        (base, len)
    };

    let dotslash_len = if cfg!(windows) { 2 } else { 0 };
    let total_len = dotslash_len + length + 1;

    let mut result = Vec::with_capacity(total_len);
    if dotslash_len > 0 {
        result.push(b'.');
        result.push(b'/');
    }

    result.extend_from_slice(base_str[..length].as_bytes());
    result.push(b'\0');

    Some(unsafe { CString::from_vec_unchecked(result) })
}