use std::ffi::{CString, CStr};
use std::ptr;
use std::time::{SystemTime, UNIX_EPOCH};
use std::mem;
use std::os::raw::{c_char, c_int};
use std::env;
use std::str;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    static ref LOCAL_TZ: Mutex<Option<TimeZone>> = Mutex::new(None);
}

const DEFAULT_MXFAST: usize = 64 * mem::size_of::<usize>() / 4;
const ABBR_SIZE_MIN: usize = DEFAULT_MXFAST - mem::size_of::<TimeZone>();

struct TimeZone {
    next: Option<Box<TimeZone>>,
    tzname_copy: [Option<CString>; 2],
    tz_is_set: bool,
    abbrs: Vec<u8>,
}

impl TimeZone {
    fn new(name: Option<&str>) -> Option<Box<Self>> {
        let name_size = name.map(|n| n.len() + 1).unwrap_or(0);
        let abbr_size = if name_size < ABBR_SIZE_MIN {
            ABBR_SIZE_MIN
        } else {
            name_size + 1
        };

        let mut abbrs = vec![0; abbr_size];
        if let Some(name) = name {
            if name.len() + 1 > abbr_size {
                return None;
            }
            abbrs[..name.len()].copy_from_slice(name.as_bytes());
            abbrs[name.len()] = 0;
        } else {
            abbrs[0] = 0;
        }

        Some(Box::new(TimeZone {
            next: None,
            tzname_copy: [None, None],
            tz_is_set: name.is_some(),
            abbrs,
        }))
    }

    fn extend_abbrs(&mut self, abbr: &str) -> bool {
        let abbr_size = abbr.len() + 1;
        if abbr_size > self.abbrs.len() {
            return false;
        }
        self.abbrs[..abbr.len()].copy_from_slice(abbr.as_bytes());
        self.abbrs[abbr.len()] = 0;
        true
    }
}

fn tzalloc(name: Option<&str>) -> Option<Box<TimeZone>> {
    TimeZone::new(name)
}

fn tzfree(tz: Option<Box<TimeZone>>) {
    // Rust's ownership system will automatically free the memory when the Box goes out of scope
}

fn getenv_tz() -> Option<String> {
    env::var("TZ").ok()
}

fn setenv_tz(tz: Option<&str>) -> Result<(), ()> {
    match tz {
        Some(tz) => env::set_var("TZ", tz),
        None => env::remove_var("TZ"),
    }
    Ok(())
}

fn change_env(tz: &TimeZone) -> Result<(), ()> {
    let tz_str = if tz.tz_is_set {
        Some(unsafe { CStr::from_ptr(tz.abbrs.as_ptr() as *const c_char) }.to_str().unwrap())
    } else {
        None
    };
    setenv_tz(tz_str)
}

fn set_tz(tz: &TimeZone) -> Result<Option<Box<TimeZone>>, ()> {
    let env_tz = getenv_tz();
    if env_tz.as_ref().map(|s| s.as_str()) == Some(unsafe { CStr::from_ptr(tz.abbrs.as_ptr() as *const c_char) }.to_str().unwrap()) {
        Ok(None)
    } else {
        let old_tz = tzalloc(env_tz.as_ref().map(|s| s.as_str()));
        if old_tz.is_none() {
            return Err(());
        }
        if change_env(tz).is_err() {
            return Err(());
        }
        Ok(old_tz)
    }
}

fn revert_tz(tz: Option<Box<TimeZone>>) -> Result<(), ()> {
    if tz.is_none() {
        return Ok(());
    }
    let tz = tz.unwrap();
    let result = change_env(&tz);
    tzfree(Some(tz));
    result
}

fn localtime_rz(tz: Option<&TimeZone>, t: i64) -> Option<libc::tm> {
    if tz.is_none() {
        let mut tm: libc::tm = unsafe { mem::zeroed() };
        unsafe {
            libc::gmtime_r(&t, &mut tm);
        }
        return Some(tm);
    }

    let tz = tz.unwrap();
    let old_tz = match set_tz(tz) {
        Ok(t) => t,
        Err(_) => return None,
    };

    let mut tm: libc::tm = unsafe { mem::zeroed() };
    let success = unsafe { libc::localtime_r(&t, &mut tm) != ptr::null_mut() };
    
    if !success || revert_tz(old_tz).is_err() {
        return None;
    }

    Some(tm)
}

fn mktime_z(tz: Option<&TimeZone>, tm: &mut libc::tm) -> Option<i64> {
    if tz.is_none() {
        return Some(unsafe { libc::timegm(tm) });
    }

    let tz = tz.unwrap();
    let old_tz = match set_tz(tz) {
        Ok(t) => t,
        Err(_) => return None,
    };

    let mut tm_1 = *tm;
    tm_1.tm_yday = -1;
    let t = unsafe { libc::mktime(&mut tm_1) };

    if t == -1 || revert_tz(old_tz).is_err() {
        return None;
    }

    *tm = tm_1;
    Some(t)
}