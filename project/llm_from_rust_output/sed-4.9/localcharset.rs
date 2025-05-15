use std::ffi::CStr;

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum NlItem {
    Codeset = 14,
    // Other variants omitted for brevity
    // ...
}

pub fn locale_charset() -> &'static str {
    let codeset = unsafe {
        let ptr = libc::nl_langinfo(NlItem::Codeset as libc::c_int);
        if ptr.is_null() {
            ""
        } else {
            CStr::from_ptr(ptr).to_str().unwrap_or("")
        }
    };

    if codeset.is_empty() {
        "ASCII"
    } else {
        codeset
    }
}