use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
extern "C" {
    fn mktime(__tp: *mut tm) -> time_t;
    fn gmtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    fn localtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    fn tzset();
    fn timegm(__tp: *mut tm) -> time_t;
    fn __errno_location() -> *mut i32;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn getenv(__name: *const i8) -> *mut i8;
    fn setenv(__name: *const i8, __value: *const i8, __replace: i32) -> i32;
    fn unsetenv(__name: *const i8) -> i32;
    fn rpl_free(ptr: *mut libc::c_void);
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strlen(_: *const i8) -> u64;
}
pub type size_t = u64;
pub type __time_t = i64;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
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
pub type ptrdiff_t = i64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm_zone {
    pub next: *mut tm_zone,
    pub tz_is_set: i8,
    pub abbrs: [i8; 0],
}
pub type timezone_t = *mut tm_zone;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed {
    ABBR_SIZE_MIN = 119,
}
impl C2RustUnnamed {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed::ABBR_SIZE_MIN => 119,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed {
        match value {
            119 => C2RustUnnamed::ABBR_SIZE_MIN,
            _ => panic!("Invalid value for C2RustUnnamed: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn add(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn sub(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn mul(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn div(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn rem(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
pub type idx_t = ptrdiff_t;
static mut local_tz: timezone_t = unsafe { 1 as i32 as timezone_t };
unsafe extern "C" fn extend_abbrs(
    mut abbrs: *mut i8,
    mut abbr: *const i8,
    mut abbr_size: size_t,
) {
    memcpy(abbrs as *mut libc::c_void, abbr as *const libc::c_void, abbr_size);
    *abbrs.offset(abbr_size as isize) = '\0' as i32 as i8;
}
#[no_mangle]
pub unsafe extern "C" fn tzalloc(mut name: *const i8) -> timezone_t {
    let mut name_size: size_t = if !name.is_null() {
        (strlen(name)).wrapping_add(1 as i32 as u64)
    } else {
        0 as i32 as u64
    };
    let mut abbr_size: size_t = if name_size < C2RustUnnamed::ABBR_SIZE_MIN as i32 as u64
    {
        C2RustUnnamed::ABBR_SIZE_MIN as i32 as u64
    } else {
        name_size.wrapping_add(1 as i32 as u64)
    };
    let mut tz: timezone_t = malloc(
        (9 as u64)
            .wrapping_add(::core::mem::align_of::<tm_zone>() as u64)
            .wrapping_sub(1 as i32 as u64)
            .wrapping_add(abbr_size)
            & !(::core::mem::align_of::<tm_zone>() as u64).wrapping_sub(1 as i32 as u64),
    ) as timezone_t;
    if !tz.is_null() {
        (*tz).next = 0 as *mut tm_zone;
        (*tz).tz_is_set = !name.is_null() as i32 as i8;
        *((*tz).abbrs).as_mut_ptr().offset(0 as i32 as isize) = '\0' as i32 as i8;
        if !name.is_null() {
            extend_abbrs(((*tz).abbrs).as_mut_ptr(), name, name_size);
        }
    }
    return tz;
}
unsafe extern "C" fn save_abbr(mut tz: timezone_t, mut tm: *mut tm) -> bool {
    let mut zone: *const i8 = 0 as *const i8;
    let mut zone_copy: *mut i8 = b"\0" as *const u8 as *const i8 as *mut i8;
    zone = (*tm).tm_zone;
    if zone.is_null()
        || tm as *mut i8 <= zone as *mut i8
            && zone < tm.offset(1 as i32 as isize) as *mut i8
    {
        return 1 as i32 != 0;
    }
    if *zone != 0 {
        zone_copy = ((*tz).abbrs).as_mut_ptr();
        while strcmp(zone_copy, zone) != 0 as i32 {
            if !(*zone_copy as i32 != 0
                || zone_copy == ((*tz).abbrs).as_mut_ptr()
                    && (*tz).tz_is_set as i32 != 0)
            {
                let mut zone_size: idx_t = (strlen(zone)).wrapping_add(1 as i32 as u64)
                    as idx_t;
                if zone_size
                    < ((*tz).abbrs)
                        .as_mut_ptr()
                        .offset(C2RustUnnamed::ABBR_SIZE_MIN as i32 as isize)
                        .offset_from(zone_copy) as i64
                {
                    extend_abbrs(zone_copy, zone, zone_size as size_t);
                } else {
                    (*tz).next = tzalloc(zone);
                    tz = (*tz).next;
                    if tz.is_null() {
                        return 0 as i32 != 0;
                    }
                    (*tz).tz_is_set = 0 as i32 as i8;
                    zone_copy = ((*tz).abbrs).as_mut_ptr();
                }
                break;
            } else {
                zone_copy = zone_copy
                    .offset((strlen(zone_copy)).wrapping_add(1 as i32 as u64) as isize);
                if *zone_copy == 0 && !((*tz).next).is_null() {
                    tz = (*tz).next;
                    zone_copy = ((*tz).abbrs).as_mut_ptr();
                }
            }
        }
    }
    (*tm).tm_zone = zone_copy;
    return 1 as i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn tzfree(mut tz: timezone_t) {
    if tz != local_tz {
        while !tz.is_null() {
            let mut next: timezone_t = (*tz).next;
            rpl_free(tz as *mut libc::c_void);
            tz = next;
        }
    }
}
unsafe extern "C" fn getenv_TZ() -> *mut i8 {
    return getenv(b"TZ\0" as *const u8 as *const i8);
}
unsafe extern "C" fn setenv_TZ(mut tz: *const i8) -> i32 {
    return if !tz.is_null() {
        setenv(b"TZ\0" as *const u8 as *const i8, tz, 1 as i32)
    } else {
        unsetenv(b"TZ\0" as *const u8 as *const i8)
    };
}
unsafe extern "C" fn change_env(mut tz: timezone_t) -> bool {
    if setenv_TZ(
        (if (*tz).tz_is_set as i32 != 0 {
            ((*tz).abbrs).as_mut_ptr()
        } else {
            0 as *mut i8
        }),
    ) != 0 as i32
    {
        return 0 as i32 != 0;
    }
    tzset();
    return 1 as i32 != 0;
}
unsafe extern "C" fn set_tz(mut tz: timezone_t) -> timezone_t {
    let mut env_tz: *mut i8 = getenv_TZ();
    if if !env_tz.is_null() {
        ((*tz).tz_is_set as i32 != 0
            && strcmp(((*tz).abbrs).as_mut_ptr(), env_tz) == 0 as i32) as i32
    } else {
        ((*tz).tz_is_set == 0) as i32
    } != 0
    {
        return local_tz
    } else {
        let mut old_tz: timezone_t = tzalloc(env_tz);
        if old_tz.is_null() {
            return old_tz;
        }
        if !change_env(tz) {
            let mut saved_errno: i32 = *__errno_location();
            tzfree(old_tz);
            *__errno_location() = saved_errno;
            return 0 as timezone_t;
        }
        return old_tz;
    };
}
unsafe extern "C" fn revert_tz(mut tz: timezone_t) -> bool {
    if tz == local_tz {
        return 1 as i32 != 0
    } else {
        let mut saved_errno: i32 = *__errno_location();
        let mut ok: bool = change_env(tz);
        if !ok {
            saved_errno = *__errno_location();
        }
        tzfree(tz);
        *__errno_location() = saved_errno;
        return ok;
    };
}
#[no_mangle]
pub unsafe extern "C" fn localtime_rz(
    mut tz: timezone_t,
    mut t: *const time_t,
    mut tm: *mut tm,
) -> *mut tm {
    if tz.is_null() {
        return gmtime_r(t, tm)
    } else {
        let mut old_tz: timezone_t = set_tz(tz);
        if !old_tz.is_null() {
            let mut abbr_saved: bool = !(localtime_r(t, tm)).is_null()
                && save_abbr(tz, tm) as i32 != 0;
            if revert_tz(old_tz) as i32 != 0 && abbr_saved as i32 != 0 {
                return tm;
            }
        }
        return 0 as *mut tm;
    };
}
#[no_mangle]
pub unsafe extern "C" fn mktime_z(mut tz: timezone_t, mut tm: *mut tm) -> time_t {
    if tz.is_null() {
        return timegm(tm)
    } else {
        let mut old_tz: timezone_t = set_tz(tz);
        if !old_tz.is_null() {
            let mut tm_1: tm = tm {
                tm_sec: 0,
                tm_min: 0,
                tm_hour: 0,
                tm_mday: 0,
                tm_mon: 0,
                tm_year: 0,
                tm_wday: 0,
                tm_yday: 0,
                tm_isdst: 0,
                tm_gmtoff: 0,
                tm_zone: 0 as *const i8,
            };
            tm_1.tm_sec = (*tm).tm_sec;
            tm_1.tm_min = (*tm).tm_min;
            tm_1.tm_hour = (*tm).tm_hour;
            tm_1.tm_mday = (*tm).tm_mday;
            tm_1.tm_mon = (*tm).tm_mon;
            tm_1.tm_year = (*tm).tm_year;
            tm_1.tm_yday = -(1 as i32);
            tm_1.tm_isdst = (*tm).tm_isdst;
            let mut t: time_t = mktime(&mut tm_1);
            let mut ok: bool = 0 as i32 <= tm_1.tm_yday;
            ok = ok as i32 != 0 && save_abbr(tz, &mut tm_1) as i32 != 0;
            if revert_tz(old_tz) as i32 != 0 && ok as i32 != 0 {
                *tm = tm_1;
                return t;
            }
        }
        return -(1 as i32) as time_t;
    };
}