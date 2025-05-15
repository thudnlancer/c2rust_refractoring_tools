use std::ffi::CStr;
use std::os::raw::{c_char, c_int, c_uchar, c_uint, c_ulong};

pub type size_t = c_ulong;
pub type wchar_t = c_int;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct MbState {
    pub __count: c_int,
    pub __value: MbStateValue,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union MbStateValue {
    pub __wch: c_uint,
    pub __wchb: [c_char; 4],
}

pub type mbstate_t = MbState;

#[no_mangle]
pub extern "C" fn rpl_mbrtowc(
    pwc: Option<&mut wchar_t>,
    s: Option<&CStr>,
    n: size_t,
    ps: Option<&mut mbstate_t>,
) -> size_t {
    let mut wc: wchar_t = 0;
    let pwc = pwc.unwrap_or(&mut wc);
    
    let ret = unsafe {
        let s_ptr = s.map(|s| s.as_ptr()).unwrap_or(std::ptr::null());
        let ps_ptr = ps.map(|ps| ps as *mut mbstate_t).unwrap_or(std::ptr::null_mut());
        libc::mbrtowc(pwc as *mut wchar_t, s_ptr, n, ps_ptr)
    };

    if (-2isize as size_t) <= ret 
        && n != 0 
        && !unsafe { libc::hard_locale(0) }
    {
        if let Some(s) = s {
            if let Some(&c) = s.to_bytes().first() {
                *pwc = c as wchar_t;
                return 1;
            }
        }
    }
    
    ret
}