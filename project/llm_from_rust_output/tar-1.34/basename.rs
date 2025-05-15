use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::path::Path;

fn base_name(name: &CStr) -> CString {
    let path = Path::new(name.to_str().unwrap());
    let base = path.file_name().unwrap_or_else(|| path.as_os_str());
    
    let base_cstr = CStr::from_bytes_with_nul(base.to_str().unwrap().as_bytes()).unwrap();
    let length = base_cstr.to_bytes().len();
    
    if base_cstr.to_bytes().is_empty() {
        let len = path.to_str().unwrap().len();
        return CString::new(path.to_str().unwrap()[..len].as_bytes()).unwrap();
    }
    
    if base_cstr.to_bytes_with_nul().last() == Some(&b'/') {
        let mut new_base = base_cstr.to_bytes().to_vec();
        new_base.push(b'/');
        return CString::new(new_base).unwrap();
    }
    
    CString::new(base_cstr.to_bytes()).unwrap()
}