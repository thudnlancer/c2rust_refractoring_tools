use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr;

pub fn mfile_name_concat(
    dir: &str,
    base: &str,
    base_in_result: Option<&mut *mut c_char>,
) -> Option<Box<[u8]>> {
    let dir_c = CString::new(dir).ok()?;
    let base_c = CString::new(base).ok()?;
    
    let dir_bytes = dir_c.as_bytes_with_nul();
    let base_bytes = base_c.as_bytes_with_nul();
    
    let last_slash = dir.rfind('/').map(|i| i + 1).unwrap_or(0);
    let dirbase = &dir[last_slash..];
    
    let mut sep = b'\0';
    if !dirbase.is_empty() {
        let dir_ends_with_slash = dir.ends_with('/');
        let base_starts_with_slash = base.starts_with('/');
        
        if !dir_ends_with_slash && !base_starts_with_slash {
            sep = b'/';
        }
    } else if base.starts_with('/') {
        sep = b'.';
    }
    
    let total_len = dir.len() + (sep != b'\0') as usize + base.len() + 1;
    let mut result = Vec::with_capacity(total_len);
    
    result.extend_from_slice(dir.as_bytes());
    if sep != b'\0' {
        result.push(sep);
    }
    
    let base_ptr = if sep != b'\0' {
        result.len() - base.len() - 1
    } else {
        result.len() - base.len()
    };
    
    result.extend_from_slice(base.as_bytes());
    result.push(b'\0');
    
    if let Some(ptr_out) = base_in_result {
        let slice = &result[base_ptr..];
        let c_str = CString::from_vec_with_nul(slice.to_vec()).ok()?;
        *ptr_out = c_str.into_raw();
    }
    
    Some(result.into_boxed_slice())
}