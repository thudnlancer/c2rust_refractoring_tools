use std::ffi::CStr;
use std::sync::OnceLock;

static PROGRAM_NAME: OnceLock<&'static CStr> = OnceLock::new();

pub fn getprogname() -> &'static CStr {
    PROGRAM_NAME.get_or_init(|| {
        unsafe {
            CStr::from_ptr(libc::program_invocation_short_name)
        }
    })
}