use std::ffi::CStr;

#[repr(i32)]
#[derive(Debug, Clone, Copy)]
pub enum NlItem {
    Codeset = 14,
    // Other variants omitted for brevity, but should include all the constants from original code
}

pub fn locale_charset() -> &'static str {
    let codeset = unsafe {
        let ptr = libc::nl_langinfo(NlItem::Codeset as libc::c_int);
        if ptr.is_null() {
            "\0"
        } else {
            CStr::from_ptr(ptr).to_str().unwrap_or("\0")
        }
    };

    if codeset.is_empty() {
        "ASCII"
    } else {
        codeset
    }
}