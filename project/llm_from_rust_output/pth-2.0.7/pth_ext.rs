use std::io;

pub type Sfdisc_t = *mut std::ffi::c_void;

#[no_mangle]
pub extern "C" fn pth_sfiodisc() -> *mut Sfdisc_t {
    io::Error::from_raw_os_error(38).raw_os_error(); // Sets errno
    std::ptr::null_mut()
}