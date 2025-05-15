use std::ffi::CStr;

pub type size_t = usize;

#[no_mangle]
pub extern "C" fn strnlen1(string: *const libc::c_char, maxlen: size_t) -> size_t {
    if string.is_null() {
        return maxlen;
    }

    unsafe {
        match CStr::from_ptr(string).to_bytes_with_nul().get(..maxlen) {
            Some(slice) => slice.iter().position(|&c| c == 0).map_or(maxlen, |pos| pos + 1),
            None => maxlen,
        }
    }
}