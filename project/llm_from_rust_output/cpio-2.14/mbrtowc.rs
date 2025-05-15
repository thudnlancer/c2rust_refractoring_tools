use std::ffi::CStr;
use std::os::raw::{c_char, c_int, c_uchar, c_uint, c_ulong};

pub type size_t = c_ulong;
pub type wchar_t = c_int;

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

extern "C" {
    fn mbrtowc(
        __pwc: *mut wchar_t,
        __s: *const c_char,
        __n: size_t,
        __p: *mut mbstate_t,
    ) -> size_t;
    fn hard_locale(category: c_int) -> bool;
}

pub fn rpl_mbrtowc(
    pwc: Option<&mut wchar_t>,
    s: Option<&CStr>,
    n: size_t,
    ps: Option<&mut mbstate_t>,
) -> Result<size_t, ()> {
    let mut wc: wchar_t = 0;
    let pwc_ptr = match pwc {
        Some(p) => p as *mut wchar_t,
        None => &mut wc as *mut wchar_t,
    };

    let s_ptr = match s {
        Some(s) => s.as_ptr(),
        None => std::ptr::null(),
    };

    let ps_ptr = match ps {
        Some(p) => p as *mut mbstate_t,
        None => std::ptr::null_mut(),
    };

    let ret = unsafe { mbrtowc(pwc_ptr, s_ptr, n, ps_ptr) };

    if (-2isize as size_t) <= ret 
        && n != 0 
        && unsafe { !hard_locale(0) } 
    {
        if let Some(s) = s {
            if let Some(&uc) = s.to_bytes().first() {
                unsafe { *pwc_ptr = uc as wchar_t };
                return Ok(1);
            }
        }
    }

    Ok(ret)
}