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
) -> Option<Box<Path>> {
    let dir_path = Path::new(dir);
    let base_path = Path::new(base);
    
    let dirbase = last_component(dir_path);
    let dirbaselen = base_len(dirbase);
    let dirlen = dir.as_ptr().offset_from(dirbase.as_os_str().as_encoded_bytes().as_ptr()).unsigned_abs() + dirbaselen;
    
    let baselen = base.len();
    let mut sep = None;
    
    if dirbaselen != 0 {
        if !dir.as_bytes().get(dirlen.wrapping_sub(1)).map_or(false, |&b| b == b'/') 
            && !base.starts_with('/') 
        {
            sep = Some('/');
        }
    } else if base.starts_with('/') {
        sep = Some('.');
    }
    
    let total_len = dirlen + sep.map_or(0, |_| 1) + baselen + 1;
    let mut result = PathBuf::with_capacity(total_len);
    
    result.push(dir);
    if let Some(s) = sep {
        result.push(s.to_string());
    }
    result.push(base);
    
    let c_path = CString::new(result.to_string_lossy().into_owned()).ok()?;
    let ptr = c_path.into_raw();
    
    if let Some(base_ptr) = base_in_result {
        let base_start = result.as_os_str().len() - baselen;
        unsafe {
            *base_ptr = ptr.offset(base_start as isize);
        }
    }
    
    Some(unsafe { Box::from_raw(ptr as *mut Path) })
}