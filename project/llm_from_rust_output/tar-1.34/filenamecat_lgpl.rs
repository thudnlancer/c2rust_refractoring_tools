use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr;

pub fn mfile_name_concat(
    dir: &str,
    base: &str,
    base_in_result: Option<&mut *mut c_char>,
) -> Option<Box<c_char>> {
    let dir_c = CString::new(dir).ok()?;
    let base_c = CString::new(base).ok()?;
    
    let dir_ptr = dir_c.as_ptr();
    let base_ptr = base_c.as_ptr();
    
    let dirbase = unsafe { last_component(dir_ptr) };
    let dirbaselen = unsafe { base_len(dir_ptr) };
    
    let dirlen = unsafe { dirbase.offset_from(dir_ptr) as usize + dirbaselen };
    let baselen = unsafe { strlen(base_ptr) };
    
    let sep = if dirbaselen != 0 {
        if !dir.is_empty() 
            && !dir.ends_with('/') 
            && !base.starts_with('/') 
        {
            Some('/')
        } else {
            None
        }
    } else if base.starts_with('/') {
        Some('.')
    } else {
        None
    };
    
    let total_len = dirlen 
        + sep.map_or(0, |_| 1) 
        + baselen 
        + 1;
    
    let mut buffer = Vec::with_capacity(total_len);
    
    // Copy dir
    buffer.extend_from_slice(unsafe { std::slice::from_raw_parts(dir_ptr as *const u8, dirlen) });
    
    // Add separator if needed
    if let Some(c) = sep {
        buffer.push(c as u8);
    }
    
    // Store base position if requested
    let base_pos = buffer.len();
    if let Some(ptr) = base_in_result {
        *ptr = buffer.as_mut_ptr() as *mut c_char;
    }
    
    // Copy base
    buffer.extend_from_slice(unsafe { std::slice::from_raw_parts(base_ptr as *const u8, baselen) });
    
    // Null terminator
    buffer.push(0);
    
    Some(unsafe { Box::from_raw(buffer.leak().as_mut_ptr() as *mut c_char) })
}

// These would need to be implemented safely or wrapped from unsafe externs
unsafe fn last_component(filename: *const c_char) -> *const c_char {
    // Implementation would wrap the C function
    extern "C" {
        fn last_component(filename: *const c_char) -> *mut c_char;
    }
    last_component(filename)
}

unsafe fn base_len(filename: *const c_char) -> usize {
    // Implementation would wrap the C function
    extern "C" {
        fn base_len(filename: *const c_char) -> usize;
    }
    base_len(filename)
}

unsafe fn strlen(s: *const c_char) -> usize {
    // Implementation would wrap the C function
    extern "C" {
        fn strlen(s: *const c_char) -> usize;
    }
    strlen(s)
}