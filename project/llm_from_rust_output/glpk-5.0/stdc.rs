use std::ffi::CStr;
use std::ptr;
use std::time::{SystemTime, UNIX_EPOCH};
use std::mem::MaybeUninit;
use std::thread_local;

#[derive(Debug, Clone, Copy)]
pub struct Tm {
    pub tm_sec: i32,
    pub tm_min: i32,
    pub tm_hour: i32,
    pub tm_mday: i32,
    pub tm_mon: i32,
    pub tm_year: i32,
    pub tm_wday: i32,
    pub tm_yday: i32,
    pub tm_isdst: i32,
    pub tm_gmtoff: i64,
    pub tm_zone: *const libc::c_char,
}

pub fn glp_xgmtime(timer: &SystemTime) -> Tm {
    let duration = timer.duration_since(UNIX_EPOCH).unwrap();
    let time = duration.as_secs() as libc::time_t;
    
    let mut result = MaybeUninit::<libc::tm>::uninit();
    unsafe {
        libc::gmtime_r(&time, result.as_mut_ptr());
        let tm = result.assume_init();
        Tm {
            tm_sec: tm.tm_sec,
            tm_min: tm.tm_min,
            tm_hour: tm.tm_hour,
            tm_mday: tm.tm_mday,
            tm_mon: tm.tm_mon,
            tm_year: tm.tm_year,
            tm_wday: tm.tm_wday,
            tm_yday: tm.tm_yday,
            tm_isdst: tm.tm_isdst,
            tm_gmtoff: tm.tm_gmtoff,
            tm_zone: tm.tm_zone,
        }
    }
}

pub fn glp_xstrerr(errnum: i32) -> String {
    let mut buf = [0u8; 1024];
    unsafe {
        let ret = libc::strerror_r(
            errnum,
            buf.as_mut_ptr() as *mut libc::c_char,
            buf.len() as libc::size_t,
        );
        if ret != 0 {
            return "Unknown error".to_string();
        }
        CStr::from_ptr(buf.as_ptr() as *const libc::c_char)
            .to_string_lossy()
            .into_owned()
    }
}

thread_local! {
    static STRTOK_PTR: std::cell::RefCell<*mut libc::c_char> = std::cell::RefCell::new(ptr::null_mut());
}

pub fn glp_xstrtok(s1: Option<&mut [u8]>, delim: &[u8]) -> Option<&mut [u8]> {
    STRTOK_PTR.with(|ptr| unsafe {
        let s1_ptr = s1.map_or(ptr::null_mut(), |s| s.as_mut_ptr() as *mut libc::c_char);
        let delim_ptr = delim.as_ptr() as *const libc::c_char;
        let mut save_ptr = ptr.borrow_mut();
        
        let result = libc::strtok_r(s1_ptr, delim_ptr, &mut *save_ptr);
        if result.is_null() {
            None
        } else {
            let len = libc::strlen(result);
            Some(std::slice::from_raw_parts_mut(result as *mut u8, len))
        }
    })
}