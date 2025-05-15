use std::ffi::{CStr, CString};
use std::ptr;
use std::time::{SystemTime, UNIX_EPOCH};
use libc::{c_char, c_int, c_long, c_ulong, c_void, time_t, tm};
use std::mem::{size_of, align_of};
use std::os::raw::c_uint;

const ABBR_SIZE_MIN: c_uint = 119;

#[derive(Debug, Clone)]
struct TmZone {
    next: Option<Box<TmZone>>,
    tz_is_set: bool,
    abbrs: Vec<c_char>,
}

type Timezone = Option<Box<TmZone>>;

static mut LOCAL_TZ: Timezone = None;

fn extend_abbrs(abbrs: &mut [c_char], abbr: &[c_char]) {
    let len = abbr.len().min(abbrs.len() - 1);
    abbrs[..len].copy_from_slice(&abbr[..len]);
    abbrs[len] = 0;
}

fn tzalloc(name: Option<&CStr>) -> Timezone {
    let name_size = name.map_or(0, |n| n.to_bytes_with_nul().len());
    let abbr_size = if name_size < ABBR_SIZE_MIN as usize {
        ABBR_SIZE_MIN as usize
    } else {
        name_size + 1
    };

    let mut tz = Box::new(TmZone {
        next: None,
        tz_is_set: name.is_some(),
        abbrs: vec![0; abbr_size],
    });

    if let Some(n) = name {
        extend_abbrs(&mut tz.abbrs, n.to_bytes_with_nul());
    }

    Some(tz)
}

fn save_abbr(tz: &mut TmZone, tm: &mut tm) -> bool {
    let zone = unsafe { tm.tm_zone };
    if zone.is_null() {
        return true;
    }

    let zone_cstr = unsafe { CStr::from_ptr(zone) };
    let zone_bytes = zone_cstr.to_bytes_with_nul();

    if !zone_bytes.is_empty() {
        let mut current = tz;
        loop {
            if current.abbrs.starts_with(zone_bytes) {
                tm.tm_zone = current.abbrs.as_ptr();
                return true;
            }

            if current.next.is_none() {
                if zone_bytes.len() < ABBR_SIZE_MIN as usize {
                    extend_abbrs(&mut current.abbrs, zone_bytes);
                    tm.tm_zone = current.abbrs.as_ptr();
                    return true;
                } else {
                    let new_tz = tzalloc(Some(zone_cstr));
                    if new_tz.is_none() {
                        return false;
                    }
                    current.next = new_tz;
                    current = current.next.as_mut().unwrap();
                    tm.tm_zone = current.abbrs.as_ptr();
                    return true;
                }
            }

            current = current.next.as_mut().unwrap();
        }
    }

    true
}

fn tzfree(tz: &mut Timezone) {
    if unsafe { &LOCAL_TZ } != tz {
        while let Some(mut current) = tz.take() {
            *tz = current.next.take();
        }
    }
}

fn getenv_tz() -> Option<CString> {
    std::env::var("TZ").ok().map(|s| CString::new(s).unwrap())
}

fn setenv_tz(tz: Option<&CStr>) -> std::io::Result<()> {
    if let Some(t) = tz {
        std::env::set_var("TZ", t.to_str().unwrap());
    } else {
        std::env::remove_var("TZ");
    }
    Ok(())
}

fn change_env(tz: &TmZone) -> bool {
    let tz_str = if tz.tz_is_set {
        Some(unsafe { CStr::from_ptr(tz.abbrs.as_ptr()) })
    } else {
        None
    };
    setenv_tz(tz_str).is_ok()
}

fn set_tz(tz: &mut TmZone) -> Option<Box<TmZone>> {
    let env_tz = getenv_tz();
    let env_tz_ptr = env_tz.as_ref().map(|s| s.as_ptr());

    if (tz.tz_is_set && env_tz_ptr == Some(tz.abbrs.as_ptr())) || (!tz.tz_is_set && env_tz.is_none()) {
        unsafe { LOCAL_TZ.clone() }
    } else {
        let old_tz = tzalloc(env_tz.as_ref().map(|s| s.as_c_str()));
        if old_tz.is_none() {
            return None;
        }
        if !change_env(tz) {
            return None;
        }
        old_tz
    }
}

fn revert_tz(tz: &mut Timezone) -> bool {
    if unsafe { &LOCAL_TZ } == tz {
        true
    } else {
        let ok = change_env(tz.as_ref().unwrap());
        tzfree(tz);
        ok
    }
}

pub fn localtime_rz(tz: &mut Timezone, t: &time_t, tm: &mut tm) -> Option<&mut tm> {
    if tz.is_none() {
        unsafe { gmtime_r(t, tm) };
        Some(tm)
    } else {
        let old_tz = set_tz(tz.as_mut().unwrap());
        if old_tz.is_some() {
            let abbr_saved = unsafe { localtime_r(t, tm) }.is_null() && save_abbr(tz.as_mut().unwrap(), tm);
            if revert_tz(&mut old_tz) && abbr_saved {
                Some(tm)
            } else {
                None
            }
        } else {
            None
        }
    }
}

pub fn mktime_z(tz: &mut Timezone, tm: &mut tm) -> Option<time_t> {
    if tz.is_none() {
        Some(unsafe { timegm(tm) })
    } else {
        let old_tz = set_tz(tz.as_mut().unwrap());
        if old_tz.is_some() {
            let mut tm_copy = *tm;
            tm_copy.tm_yday = -1;
            let t = unsafe { mktime(&mut tm_copy) };
            let ok = tm_copy.tm_yday >= 0 && save_abbr(tz.as_mut().unwrap(), &mut tm_copy);
            if revert_tz(&mut old_tz) && ok {
                *tm = tm_copy;
                Some(t)
            } else {
                None
            }
        } else {
            None
        }
    }
}