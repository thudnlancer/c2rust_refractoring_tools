use std::ffi::{CString, CStr};
use std::ptr;
use std::mem;
use std::time::{SystemTime, UNIX_EPOCH};
use std::os::raw::{c_char, c_int};
use std::env;
use std::str;
use std::io::{Error, ErrorKind};

const DEFAULT_MXFAST: usize = 64 * mem::size_of::<usize>() / 4;
const ABBR_SIZE_MIN: usize = DEFAULT_MXFAST - mem::size_of::<usize>() * 3;

struct TmZone {
    next: Option<Box<TmZone>>,
    tzname_copy: [Option<CString>; 2],
    tz_is_set: bool,
    abbrs: Vec<u8>,
}

type TimezoneT = Option<Box<TmZone>>;

static LOCAL_TZ: TimezoneT = None;

fn extend_abbrs(abbrs: &mut Vec<u8>, abbr: &[u8]) {
    abbrs.extend_from_slice(abbr);
    abbrs.push(0);
}

fn tzalloc(name: Option<&str>) -> TimezoneT {
    let name_size = name.map_or(0, |n| n.len() + 1);
    let abbr_size = if name_size < ABBR_SIZE_MIN {
        ABBR_SIZE_MIN
    } else {
        name_size + 1
    };

    let mut tz = Box::new(TmZone {
        next: None,
        tzname_copy: [None, None],
        tz_is_set: name.is_some(),
        abbrs: Vec::with_capacity(abbr_size),
    });

    tz.abbrs.push(0);
    if let Some(name) = name {
        extend_abbrs(&mut tz.abbrs, name.as_bytes());
    }

    Some(tz)
}

fn save_abbr(tz: &mut TmZone, tm: &mut libc::tm) -> Result<(), Error> {
    let zone = unsafe {
        if !tm.tm_zone.is_null() {
            Some(CStr::from_ptr(tm.tm_zone).to_bytes())
        } else {
            None
        }
    };

    if let Some(zone) = zone {
        if !zone.is_empty() {
            let mut zone_copy = &tz.abbrs[..];
            loop {
                let current = unsafe { CStr::from_ptr(zone_copy.as_ptr() as *const c_char) };
                if current.to_bytes() == zone {
                    break;
                }

                if zone_copy[0] == 0 && (zone_copy.as_ptr() == tz.abbrs.as_ptr() && !tz.tz_is_set) {
                    let zone_size = zone.len() + 1;
                    if zone_size < ABBR_SIZE_MIN - (zone_copy.as_ptr() as usize - tz.abbrs.as_ptr() as usize) {
                        extend_abbrs(&mut tz.abbrs, zone);
                    } else {
                        let new_tz = tzalloc(Some(str::from_utf8(zone).unwrap()));
                        if new_tz.is_none() {
                            return Err(Error::new(ErrorKind::Other, "tzalloc failed"));
                        }
                        tz.next = new_tz;
                        zone_copy = &tz.next.as_ref().unwrap().abbrs[..];
                    }
                    break;
                }

                let len = current.to_bytes().len() + 1;
                zone_copy = &zone_copy[len..];
                if zone_copy.is_empty() && tz.next.is_some() {
                    tz = tz.next.as_mut().unwrap();
                    zone_copy = &tz.abbrs[..];
                }
            }
        }
    }

    Ok(())
}

fn tzfree(tz: TimezoneT) {
    if tz.is_some() {
        let mut current = tz;
        while let Some(mut t) = current {
            current = t.next.take();
        }
    }
}

fn getenv_tz() -> Option<String> {
    env::var("TZ").ok()
}

fn setenv_tz(tz: Option<&str>) -> Result<(), Error> {
    match tz {
        Some(tz) => env::set_var("TZ", tz),
        None => env::remove_var("TZ"),
    }
    Ok(())
}

fn change_env(tz: &TmZone) -> Result<(), Error> {
    let tz_str = if tz.tz_is_set {
        Some(unsafe { CStr::from_ptr(tz.abbrs.as_ptr() as *const c_char).to_str().unwrap() })
    } else {
        None
    };
    setenv_tz(tz_str)
}

fn set_tz(tz: &TmZone) -> Result<TimezoneT, Error> {
    let env_tz = getenv_tz();
    if (env_tz.is_some() && tz.tz_is_set && env_tz.unwrap() == unsafe { CStr::from_ptr(tz.abbrs.as_ptr() as *const c_char).to_str().unwrap() })
        || (env_tz.is_none() && !tz.tz_is_set)
    {
        Ok(None)
    } else {
        let old_tz = tzalloc(env_tz.as_deref());
        if old_tz.is_none() {
            return Err(Error::new(ErrorKind::Other, "tzalloc failed"));
        }
        if let Err(e) = change_env(tz) {
            tzfree(old_tz);
            return Err(e);
        }
        Ok(old_tz)
    }
}

fn revert_tz(tz: TimezoneT) -> Result<(), Error> {
    if tz.is_none() {
        return Ok(());
    }
    let result = change_env(&tz.as_ref().unwrap());
    tzfree(tz);
    result
}

fn localtime_rz(tz: TimezoneT, t: &libc::time_t, tm: &mut libc::tm) -> Result<(), Error> {
    if tz.is_none() {
        unsafe {
            if libc::gmtime_r(t, tm).is_null() {
                return Err(Error::last_os_error());
            }
        }
        return Ok(());
    }

    let old_tz = set_tz(&tz.as_ref().unwrap())?;
    unsafe {
        if libc::localtime_r(t, tm).is_null() {
            revert_tz(old_tz)?;
            return Err(Error::last_os_error());
        }
    }
    if !save_abbr(tz.as_ref().unwrap(), tm)? {
        revert_tz(old_tz)?;
        return Err(Error::new(ErrorKind::Other, "save_abbr failed"));
    }
    revert_tz(old_tz)
}

fn mktime_z(tz: TimezoneT, tm: &mut libc::tm) -> Result<libc::time_t, Error> {
    if tz.is_none() {
        unsafe {
            let t = libc::timegm(tm);
            if t == -1 {
                return Err(Error::last_os_error());
            }
            return Ok(t);
        }
    }

    let old_tz = set_tz(&tz.as_ref().unwrap())?;
    let mut tm_1 = *tm;
    tm_1.tm_yday = -1;
    unsafe {
        let t = libc::mktime(&mut tm_1);
        if t == -1 {
            revert_tz(old_tz)?;
            return Err(Error::last_os_error());
        }
    }
    if tm_1.tm_yday < 0 {
        revert_tz(old_tz)?;
        return Err(Error::new(ErrorKind::Other, "invalid tm_yday"));
    }
    if !save_abbr(tz.as_ref().unwrap(), &mut tm_1)? {
        revert_tz(old_tz)?;
        return Err(Error::new(ErrorKind::Other, "save_abbr failed"));
    }
    *tm = tm_1;
    revert_tz(old_tz)?;
    Ok(unsafe { libc::mktime(tm) })
}