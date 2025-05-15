use std::ffi::{CString, CStr};
use std::ptr;

fn concatenated_filename(
    directory: &CStr,
    filename: &CStr,
    suffix: &CStr,
) -> Option<CString> {
    // This would be implemented by a safe wrapper around the original C function
    // For the purpose of this translation, we'll assume it returns an Option<CString>
    None
}

fn xalloc_die() -> ! {
    panic!("Memory allocation failed");
}

pub fn xconcatenated_filename(
    directory: &CStr,
    filename: &CStr,
    suffix: &CStr,
) -> CString {
    concatenated_filename(directory, filename, suffix)
        .unwrap_or_else(|| xalloc_die())
}