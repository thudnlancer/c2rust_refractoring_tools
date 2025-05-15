use std::mem;
use std::ffi::CStr;
use std::os::raw::{c_char, c_int, c_uint, c_void};
use std::ptr;

type size_t = usize;
type wchar_t = c_int;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct __mbstate_t {
    pub __count: c_int,
    pub __value: C2RustUnnamed,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __wch: c_uint,
    pub __wchb: [c_char; 4],
}

pub type mbstate_t = __mbstate_t;

pub static mut mb_cur_max: c_int = 0;
pub static mut is_utf8: bool = false;

pub fn is_mb_char(ch: c_int, cur_stat: &mut mbstate_t) -> c_int {
    let c = ch as c_char;
    let mb_pending = (!mbsinit(cur_stat)) as c_int;
    
    let result = unsafe {
        rpl_mbrtowc(
            ptr::null_mut(),
            &c,
            1,
            cur_stat,
        )
    } as c_int;

    match result {
        -2 => 1,
        -1 => {
            unsafe {
                ptr::write_bytes(cur_stat, 0, 1);
            }
            0
        }
        1 => mb_pending,
        0 => 1,
        _ => panic!("is_mb_char: mbrtowc (0x{:x}) returned {}", ch as c_uint, result),
    }
}

pub fn initialize_mbcs() {
    let codeset_name = unsafe { CStr::from_ptr(locale_charset()) };
    unsafe {
        is_utf8 = codeset_name.to_str().unwrap_or("") == "UTF-8";
        mb_cur_max = __ctype_get_mb_cur_max() as c_int;
    }
}

extern "C" {
    fn mbsinit(__ps: *const mbstate_t) -> c_int;
    fn rpl_mbrtowc(
        pwc: *mut wchar_t,
        s: *const c_char,
        n: size_t,
        ps: *mut mbstate_t,
    ) -> size_t;
    fn __ctype_get_mb_cur_max() -> size_t;
    fn locale_charset() -> *const c_char;
}