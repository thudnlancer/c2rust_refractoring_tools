use std::ffi::CString;
use std::os::raw::c_char;
use std::path::{Path, PathBuf};

fn last_component(path: &Path) -> &Path {
    path.components().last().map(|c| c.as_ref()).unwrap_or(path)
}

fn base_len(path: &Path) -> usize {
    last_component(path).to_string_lossy().len()
}

pub fn mfile_name_concat(
    dir: &str,
    base: &str,
    base_in_result: Option<&mut *mut c_char>,
) -> Option<CString> {
    let dir_path = Path::new(dir);
    let dirbase = last_component(dir_path);
    let dirbaselen = base_len(dirbase);
    
    let dir_str = dir_path.to_string_lossy();
    let dirbase_str = dirbase.to_string_lossy();
    let dirbase_pos = dir_str.rfind(dirbase_str.as_ref()).unwrap_or(0);
    let dirlen = dirbase_pos + dirbaselen;
    
    let baselen = base.len();
    
    let mut sep = '\0';
    if dirbaselen != 0 {
        if !dir_str.ends_with('/') && !base.starts_with('/') {
            sep = '/';
        }
    } else if base.starts_with('/') {
        sep = '.';
    }
    
    let total_len = dirlen + (if sep != '\0' { 1 } else { 0 }) + baselen + 1;
    let mut result = Vec::with_capacity(total_len);
    
    result.extend_from_slice(&dir_str.as_bytes()[..dirlen]);
    if sep != '\0' {
        result.push(sep as u8);
    }
    
    let base_start = result.len();
    result.extend_from_slice(base.as_bytes());
    result.push(b'\0');
    
    if let Some(base_ptr) = base_in_result {
        let c_str = CString::new(&result[base_start..base_start + baselen]).ok()?;
        *base_ptr = c_str.into_raw();
    }
    
    CString::new(result).ok()
}