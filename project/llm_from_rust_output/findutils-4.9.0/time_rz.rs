use std::ffi::{CStr, CString};
use std::ptr;
use std::time::{SystemTime, UNIX_EPOCH};
use libc::{c_char, c_int, c_long, c_void, time_t, tm};
use std::mem;

const ABBR_SIZE_MIN: usize = 119;

struct TmZone {
    next: Option<Box<TmZone>>,
    tz_is_set: bool,
    abbrs: CString,
}

type Timezone = Option<Box<TmZone>>;

static mut LOCAL_TZ: Timezone = None;

fn extend_abbrs(abbrs: &mut CString, abbr: &CStr) {
    *abbrs = abbr.to_owned();
}

fn tzalloc(name: Option<&CStr>) -> Timezone {
    let name_size = name.map_or(0, |n| n.to_bytes_with_nul().len());
    let abbr_size = if name_size < ABBR_SIZE_MIN {
        ABBR_SIZE_MIN
    } else {
        name_size + 1
    };

    let abbrs = name.map_or_else(
        || CString::new("").unwrap(),
        |n| n.to_owned(),
    );

    Some(Box::new(TmZone {
        next: None,
        tz_is_set: name.is_some(),
        abbrs,
    }))
}

fn save_abbr(tz: &mut TmZone, tm: &mut tm) -> bool {
    let zone = unsafe { tm.tm_zone };
    if zone.is_null() {
        return true;
    }

    let zone_cstr = unsafe { CStr::from_ptr(zone) };
    if !zone_cstr.to_bytes().is_empty() {
        let mut current = tz;
        loop {
            if current.abbrs.as_ptr() == zone {
                unsafe { tm.tm_zone = current.abbrs.as_ptr(); }
                return true;
            }

            if current.abbrs.to_bytes().is_empty() && !current.tz_is_set {
                let zone_size = zone_cstr.to_bytes_with_nul().len();
                if zone_size < ABBR_SIZE_MIN {
                    extend_abbrs(&mut current.abbrs, zone_cstr);
                    unsafe { tm.tm_zone = current.abbrs.as_ptr(); }
                    return true;
                } else {
                    let new_zone = tzalloc(Some(zone_cstr));
                    if new_zone.is_none() {
                        return false;
                    }
                    current.next = new_zone;
                    current = current.next.as_mut().unwrap();
                }
            } else {
                if let Some(next) = &mut current.next {
                    current = next;
                } else {
                    break;
                }
            }
        }
    }
    true
}

fn tzfree(tz: Timezone) {
    if tz.is_some() && !ptr::eq(tz.as_ref().unwrap(), unsafe { &LOCAL_TZ }) {
        let mut current = tz;
        while let Some(mut zone) = current {
            current = zone.next.take();
        }
    }
}

fn getenv_tz() -> Option<CString> {
    unsafe {
        let tz = libc::getenv(b"TZ\0".as_ptr() as *const c_char);
        if tz.is_null() {
            None
        } else {
            Some(CStr::from_ptr(tz).to_owned())
        }
    }
}

fn setenv_tz(tz: Option<&CStr>) -> Result<(), ()> {
    let res = unsafe {
        if let Some(tz) = tz {
            libc::setenv(
                b"TZ\0".as_ptr() as *const c_char,
                tz.as_ptr(),
                1,
            )
        } else {
            libc::unsetenv(b"TZ\0".as_ptr() as *const c_char)
        }
    };
    if res == 0 {
        unsafe { libc::tzset(); }
        Ok(())
    } else {
        Err(())
    }
}

fn change_env(tz: &TmZone) -> bool {
    let tz_str = if tz.tz_is_set {
        Some(tz.abbrs.as_c_str())
    } else {
        None
    };
    setenv_tz(tz_str).is_ok()
}

fn set_tz(tz: &mut TmZone) -> Option<Box<TmZone>> {
    let env_tz = getenv_tz();
    let env_matches = if let Some(env_tz) = &env_tz {
        tz.tz_is_set && tz.abbrs.as_c_str() == env_tz.as_c_str()
    } else {
        !tz.tz_is_set
    };

    if env_matches {
        None
    } else {
        let old_tz = tzalloc(env_tz.as_ref().map(|s| s.as_c_str()));
        if old_tz.is_none() {
            return None;
        }
        if !change_env(tz) {
            let saved_errno = unsafe { *libc::__errno_location() };
            tzfree(old_tz);
            unsafe { *libc::__errno_location() = saved_errno; }
            None
        } else {
            old_tz
        }
    }
}

fn revert_tz(tz: Option<Box<TmZone>>) -> bool {
    if tz.is_none() {
        true
    } else {
        let saved_errno = unsafe { *libc::__errno_location() };
        let ok = change_env(tz.as_ref().unwrap());
        if !ok {
            unsafe { *libc::__errno_location() = saved_errno; }
        }
        tzfree(tz);
        ok
    }
}

pub fn localtime_rz(tz: Option<&mut TmZone>, t: &time_t, tm: &mut tm) -> Option<&mut tm> {
    if tz.is_none() {
        unsafe {
            if libc::gmtime_r(t, tm).is_null() {
                None
            } else {
                Some(tm)
            }
        }
    } else {
        let tz = tz.unwrap();
        let old_tz = set_tz(tz);
        if old_tz.is_none() {
            return None;
        }

        unsafe {
            let abbr_saved = !libc::localtime_r(t, tm).is_null() && save_abbr(tz, tm);
            if revert_tz(old_tz) && abbr_saved {
                Some(tm)
            } else {
                None
            }
        }
    }
}

pub fn mktime_z(tz: Option<&mut TmZone>, tm: &mut tm) -> Option<time_t> {
    if tz.is_none() {
        unsafe {
            let t = libc::timegm(tm);
            if t == -1 {
                None
            } else {
                Some(t)
            }
        }
    } else {
        let tz = tz.unwrap();
        let old_tz = set_tz(tz);
        if old_tz.is_none() {
            return None;
        }

        let mut tm_copy = unsafe { mem::zeroed::<tm>() };
        unsafe {
            tm_copy.tm_sec = tm.tm_sec;
            tm_copy.tm_min = tm.tm_min;
            tm_copy.tm_hour = tm.tm_hour;
            tm_copy.tm_mday = tm.tm_mday;
            tm_copy.tm_mon = tm.tm_mon;
            tm_copy.tm_year = tm.tm_year;
            tm_copy.tm_yday = -1;
            tm_copy.tm_isdst = tm.tm_isdst;
        }

        let t = unsafe { libc::mktime(&mut tm_copy) };
        let ok = tm_copy.tm_yday >= 0 && save_abbr(tz, &mut tm_copy);

        if revert_tz(old_tz) && ok {
            unsafe { *tm = tm_copy; }
            Some(t)
        } else {
            None
        }
    }
}