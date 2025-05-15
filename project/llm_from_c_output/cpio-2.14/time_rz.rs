use std::ffi::{CString, CStr};
use std::ptr;
use std::mem;
use std::os::raw::{c_char, c_int};
use std::time::{SystemTime, UNIX_EPOCH};
use std::env;
use libc::{time_t, tm, localtime_r, gmtime_r, mktime, tzset, setenv, unsetenv, getenv};

const DEFAULT_MXFAST: usize = 64 * mem::size_of::<usize>() / 4;
const ABBR_SIZE_MIN: usize = DEFAULT_MXFAST - mem::size_of::<TmZone>() + mem::size_of::<*mut c_char>();

static LOCAL_TZ: *mut TmZone = 1 as *mut TmZone;

#[repr(C)]
struct TmZone {
    next: *mut TmZone,
    #[cfg(all(not(HAVE_STRUCT_TM_TM_ZONE), feature = "tzname"))]
    tzname_copy: [*mut c_char; 2],
    tz_is_set: c_int,
    abbrs: [c_char; 1],
}

type TimezoneT = *mut TmZone;

extern "C" {
    fn timegm(tm: *mut tm) -> time_t;
}

fn extend_abbrs(abbrs: *mut c_char, abbr: *const c_char, abbr_size: usize) {
    unsafe {
        libc::memcpy(abbrs as *mut libc::c_void, abbr as *const libc::c_void, abbr_size);
        *abbrs.add(abbr_size) = 0;
    }
}

fn tzalloc(name: Option<&str>) -> Option<TimezoneT> {
    let name_size = name.map(|n| n.len() + 1).unwrap_or(0);
    let abbr_size = if name_size < ABBR_SIZE_MIN {
        ABBR_SIZE_MIN
    } else {
        name_size + 1
    };

    let tz = unsafe {
        libc::malloc(mem::size_of::<TmZone>() + abbr_size - mem::size_of::<c_char>()) as *mut TmZone
    };

    if !tz.is_null() {
        unsafe {
            (*tz).next = ptr::null_mut();
            #[cfg(all(not(HAVE_STRUCT_TM_TM_ZONE), feature = "tzname"))]
            {
                (*tz).tzname_copy = [ptr::null_mut(), ptr::null_mut()];
            }
            (*tz).tz_is_set = if name.is_some() { 1 } else { 0 };
            *(*tz).abbrs.get_unchecked_mut(0) = 0;
            if let Some(name) = name {
                let c_name = CString::new(name).unwrap();
                extend_abbrs((*tz).abbrs.as_mut_ptr(), c_name.as_ptr(), name_size);
            }
        }
        Some(tz)
    } else {
        None
    }
}

fn save_abbr(tz: TimezoneT, tm: *mut tm) -> bool {
    #[cfg(any(HAVE_STRUCT_TM_TM_ZONE, feature = "tzname"))]
    {
        let mut zone: *const c_char = ptr::null();
        let mut zone_copy: *mut c_char = b"\0".as_ptr() as *mut c_char;

        #[cfg(feature = "tzname")]
        let mut tzname_index = -1;

        #[cfg(HAVE_STRUCT_TM_TM_ZONE)]
        unsafe {
            zone = (*tm).tm_zone;
        }

        #[cfg(feature = "tzname")]
        unsafe {
            if zone.is_null() || *zone == 0 && (*tm).tm_isdst >= 0 {
                tzname_index = (*tm).tm_isdst as usize;
                zone = libc::tzname[tzname_index];
            }
        }

        if zone.is_null() || (zone as *mut c_char >= tm as *mut c_char
            && zone as *mut c_char < (tm as *mut c_char).add(1))
        {
            return true;
        }

        unsafe {
            if *zone != 0 {
                zone_copy = (*tz).abbrs.as_mut_ptr();

                while libc::strcmp(zone_copy, zone) != 0 {
                    if *zone_copy == 0 && (zone_copy != (*tz).abbrs.as_ptr() || (*tz).tz_is_set == 0) {
                        let zone_size = libc::strlen(zone) + 1;
                        if zone_size < (*tz).abbrs.as_ptr().add(ABBR_SIZE_MIN).offset_from(zone_copy) as usize {
                            extend_abbrs(zone_copy, zone, zone_size);
                        } else {
                            let new_tz = tzalloc(Some(CStr::from_ptr(zone).to_str().unwrap());
                            if new_tz.is_none() {
                                return false;
                            }
                            (*tz).next = new_tz.unwrap();
                            (*new_tz.unwrap()).tz_is_set = 0;
                            zone_copy = (*new_tz.unwrap()).abbrs.as_mut_ptr();
                        }
                        break;
                    }

                    zone_copy = zone_copy.add(libc::strlen(zone_copy) + 1);
                    if *zone_copy == 0 && !(*tz).next.is_null() {
                        tz = (*tz).next;
                        zone_copy = (*tz).abbrs.as_mut_ptr();
                    }
                }
            }

            #[cfg(HAVE_STRUCT_TM_TM_ZONE)]
            (*tm).tm_zone = zone_copy;
            #[cfg(all(not(HAVE_STRUCT_TM_TM_ZONE), feature = "tzname"))]
            if tzname_index >= 0 {
                (*tz).tzname_copy[tzname_index as usize] = zone_copy;
            }
        }
    }
    true
}

fn tzfree(tz: TimezoneT) {
    if tz != LOCAL_TZ {
        let mut current = tz;
        while !current.is_null() {
            let next = unsafe { (*current).next };
            unsafe { libc::free(current as *mut libc::c_void) };
            current = next;
        }
    }
}

fn getenv_tz() -> Option<String> {
    env::var("TZ").ok()
}

fn setenv_tz(tz: Option<&str>) -> Result<(), ()> {
    match tz {
        Some(tz) => {
            if unsafe { setenv(b"TZ\0".as_ptr() as *const c_char, tz.as_ptr() as *const c_char, 1) } == 0 {
                Ok(())
            } else {
                Err(())
            }
        }
        None => {
            if unsafe { unsetenv(b"TZ\0".as_ptr() as *const c_char) } == 0 {
                Ok(())
            } else {
                Err(())
            }
        }
    }
}

fn change_env(tz: TimezoneT) -> bool {
    unsafe {
        if (*tz).tz_is_set != 0 {
            setenv_tz(Some(CStr::from_ptr((*tz).abbrs.as_ptr()).to_str().unwrap()) == Ok(())
        } else {
            setenv_tz(None) == Ok(())
        }
    }
}

fn set_tz(tz: TimezoneT) -> Option<TimezoneT> {
    let env_tz = getenv_tz();
    let env_tz_cstr = env_tz.as_ref().map(|s| CString::new(s.as_str()).unwrap());

    unsafe {
        if env_tz.is_some() && (*tz).tz_is_set != 0
            && libc::strcmp((*tz).abbrs.as_ptr(), env_tz_cstr.unwrap().as_ptr()) == 0
            || env_tz.is_none() && (*tz).tz_is_set == 0
        {
            Some(LOCAL_TZ)
        } else {
            let old_tz = tzalloc(env_tz.as_ref().map(|s| s.as_str()));
            if old_tz.is_none() {
                return None;
            }
            if !change_env(tz) {
                let saved_errno = std::io::Error::last_os_error().raw_os_error().unwrap();
                tzfree(old_tz.unwrap());
                std::io::Error::from_raw_os_error(saved_errno);
                None
            } else {
                old_tz
            }
        }
    }
}

fn revert_tz(tz: TimezoneT) -> bool {
    if tz == LOCAL_TZ {
        true
    } else {
        let saved_errno = std::io::Error::last_os_error().raw_os_error().unwrap();
        let ok = change_env(tz);
        if !ok {
            std::io::Error::from_raw_os_error(saved_errno);
        }
        tzfree(tz);
        ok
    }
}

pub fn localtime_rz(tz: TimezoneT, t: *const time_t, tm: *mut tm) -> Option<*mut tm> {
    if tz.is_null() {
        unsafe { gmtime_r(t, tm) };
        Some(tm)
    } else {
        let old_tz = set_tz(tz)?;
        unsafe {
            if localtime_r(t, tm).is_null() {
                revert_tz(old_tz);
                None
            } else if !save_abbr(tz, tm) {
                revert_tz(old_tz);
                None
            } else if !revert_tz(old_tz) {
                None
            } else {
                Some(tm)
            }
        }
    }
}

pub fn mktime_z(tz: TimezoneT, tm: *mut tm) -> Option<time_t> {
    if tz.is_null() {
        unsafe { Some(timegm(tm)) }
    } else {
        let old_tz = set_tz(tz)?;
        let mut tm_1 = unsafe { *tm };
        tm_1.tm_yday = -1;
        let t = unsafe { mktime(&mut tm_1) };
        let mut ok = tm_1.tm_yday >= 0;
        #[cfg(any(HAVE_STRUCT_TM_TM_ZONE, feature = "tzname"))]
        {
            ok = ok && save_abbr(tz, &mut tm_1);
        }
        if !revert_tz(old_tz) || !ok {
            None
        } else {
            unsafe { *tm = tm_1 };
            Some(t)
        }
    }
}