use std::ffi::{CStr, CString};
use std::ptr;
use std::time::{SystemTime, UNIX_EPOCH};

pub type time_t = i64;

#[derive(Debug, Clone, Copy, Default)]
pub struct tm {
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
    pub tm_zone: *const i8,
}

pub struct TimeZone {
    tz: CString,
}

impl TimeZone {
    pub fn new(name: Option<&str>) -> Option<Self> {
        name.map(|n| {
            let tz = CString::new(n).unwrap();
            Self { tz }
        })
    }

    pub fn localtime(&self, t: time_t) -> Option<tm> {
        let mut tm = tm::default();
        unsafe {
            if libc::localtime_r(&t, &mut tm).is_null() {
                None
            } else {
                Some(tm)
            }
        }
    }

    pub fn mktime(&self, tm: &mut tm) -> Option<time_t> {
        let old_tz = Self::get_current_tz();
        if self.set_tz().is_err() {
            return None;
        }

        let mut tm_copy = *tm;
        tm_copy.tm_yday = -1;
        let t = unsafe { libc::mktime(&mut tm_copy) };

        if t == -1 || tm_copy.tm_yday < 0 {
            Self::restore_tz(old_tz);
            None
        } else {
            *tm = tm_copy;
            Self::restore_tz(old_tz);
            Some(t)
        }
    }

    fn get_current_tz() -> Option<CString> {
        unsafe {
            let tz = libc::getenv(b"TZ\0".as_ptr() as *const i8);
            if tz.is_null() {
                None
            } else {
                Some(CStr::from_ptr(tz).to_owned())
            }
        }
    }

    fn set_tz(&self) -> Result<(), ()> {
        unsafe {
            if libc::setenv(
                b"TZ\0".as_ptr() as *const i8,
                self.tz.as_ptr(),
                1,
            ) != 0 {
                Err(())
            } else {
                libc::tzset();
                Ok(())
            }
        }
    }

    fn restore_tz(tz: Option<CString>) {
        unsafe {
            if let Some(tz) = tz {
                libc::setenv(b"TZ\0".as_ptr() as *const i8, tz.as_ptr(), 1);
            } else {
                libc::unsetenv(b"TZ\0".as_ptr() as *const i8);
            }
            libc::tzset();
        }
    }
}

pub fn timegm(tm: &mut tm) -> Option<time_t> {
    let mut tm_copy = *tm;
    tm_copy.tm_yday = -1;
    let t = unsafe { libc::timegm(&mut tm_copy) };
    if t == -1 || tm_copy.tm_yday < 0 {
        None
    } else {
        *tm = tm_copy;
        Some(t)
    }
}

pub fn localtime(t: time_t) -> Option<tm> {
    let mut tm = tm::default();
    unsafe {
        if libc::localtime_r(&t, &mut tm).is_null() {
            None
        } else {
            Some(tm)
        }
    }
}

pub fn mktime(tm: &mut tm) -> Option<time_t> {
    let mut tm_copy = *tm;
    tm_copy.tm_yday = -1;
    let t = unsafe { libc::mktime(&mut tm_copy) };
    if t == -1 || tm_copy.tm_yday < 0 {
        None
    } else {
        *tm = tm_copy;
        Some(t)
    }
}