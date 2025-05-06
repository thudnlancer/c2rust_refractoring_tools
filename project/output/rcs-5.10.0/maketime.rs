#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types)]
use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
extern "C" {
    pub type wlink;
    pub type atat;
    pub type fro;
    pub type ephemstuff;
    pub type isr_scratch;
    pub type hash;
    pub type lockdef;
    pub type link;
    fn tzset();
    fn gmtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    fn localtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    static mut top: *mut top;
    fn partime(s: *const i8, t: *mut partime) -> *const i8;
}
pub type __dev_t = u64;
pub type __uid_t = u32;
pub type __gid_t = u32;
pub type __ino_t = u64;
pub type __mode_t = u32;
pub type __nlink_t = u64;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type __time_t = i64;
pub type __blksize_t = i64;
pub type __blkcnt_t = i64;
pub type __syscall_slong_t = i64;
pub type size_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: i32,
    pub _IO_read_ptr: *mut i8,
    pub _IO_read_end: *mut i8,
    pub _IO_read_base: *mut i8,
    pub _IO_write_base: *mut i8,
    pub _IO_write_ptr: *mut i8,
    pub _IO_write_end: *mut i8,
    pub _IO_buf_base: *mut i8,
    pub _IO_buf_end: *mut i8,
    pub _IO_save_base: *mut i8,
    pub _IO_backup_base: *mut i8,
    pub _IO_save_end: *mut i8,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: i32,
    pub _flags2: i32,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [i8; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub __pad1: *mut libc::c_void,
    pub __pad2: *mut libc::c_void,
    pub __pad3: *mut libc::c_void,
    pub __pad4: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: i32,
    pub _unused2: [i8; 20],
}
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: i32,
}
pub type FILE = _IO_FILE;
pub type off_t = __off_t;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: i32,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cbuf {
    pub string: *const i8,
    pub size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct delta {
    pub num: *const i8,
    pub date: *const i8,
    pub author: *const i8,
    pub lockedby: *const i8,
    pub state: *const i8,
    pub log: *mut atat,
    pub text: *mut atat,
    pub name: *const i8,
    pub pretty_log: cbuf,
    pub branches: *mut wlink,
    pub commitid: *const i8,
    pub ilk: *mut delta,
    pub selector: bool,
    pub neck: off_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct program {
    pub invoke: *const i8,
    pub name: *const i8,
    pub desc: *const i8,
    pub help: *const i8,
    pub tyag: i32,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum maker {
    notmade,
    real,
    effective,
}
impl maker {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            maker::notmade => 0,
            maker::real => 1,
            maker::effective => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> maker {
        match value {
            0 => maker::notmade,
            1 => maker::real,
            2 => maker::effective,
            _ => panic!("Invalid value for maker: {}", value),
        }
    }
}
impl AddAssign<u32> for maker {
    fn add_assign(&mut self, rhs: u32) {
        *self = maker::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for maker {
    fn sub_assign(&mut self, rhs: u32) {
        *self = maker::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for maker {
    fn mul_assign(&mut self, rhs: u32) {
        *self = maker::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for maker {
    fn div_assign(&mut self, rhs: u32) {
        *self = maker::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for maker {
    fn rem_assign(&mut self, rhs: u32) {
        *self = maker::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for maker {
    type Output = maker;
    fn add(self, rhs: u32) -> maker {
        maker::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for maker {
    type Output = maker;
    fn sub(self, rhs: u32) -> maker {
        maker::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for maker {
    type Output = maker;
    fn mul(self, rhs: u32) -> maker {
        maker::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for maker {
    type Output = maker;
    fn div(self, rhs: u32) -> maker {
        maker::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for maker {
    type Output = maker;
    fn rem(self, rhs: u32) -> maker {
        maker::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sff {
    pub filename: *const i8,
    pub disposition: maker,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct behavior {
    pub invdir: *const i8,
    pub unbuffered: bool,
    pub quiet: bool,
    pub interactive_valid: bool,
    pub interactive: bool,
    pub inclusive_of_Locker_in_Id_val: bool,
    pub strictly_locking: bool,
    pub version_set: bool,
    pub version: i32,
    pub stick_with_euid: bool,
    pub ruid: i32,
    pub euid: i32,
    pub ruid_cached: bool,
    pub euid_cached: bool,
    pub already_setuid: bool,
    pub kws: i32,
    pub pe: *const i8,
    pub zone_offset: zone_offset,
    pub username: *mut i8,
    pub now: timespec,
    pub fixed_SIGCHLD: bool,
    pub Oerrloop: bool,
    pub cwd: *mut i8,
    pub mem_limit: off_t,
    pub sff: *mut sff,
    pub isr: *mut isr_scratch,
    pub ephemstuff: *mut ephemstuff,
    pub maketimestuff: *mut maketimestuff,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct maketimestuff {
    pub tzset_already_called: bool,
    pub TZ: *const i8,
    pub time2tm_stash: tm,
    pub t_cache: [time_t; 2],
    pub tm_cache: [tm; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zone_offset {
    pub valid: bool,
    pub seconds: i64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct manifestation {
    pub filename: *mut i8,
    pub standard_output: *mut FILE,
    pub prev: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub valid: bool,
    pub author: *mut i8,
    pub date: *mut i8,
    pub name: *mut i8,
    pub rev: *mut i8,
    pub state: *mut i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct repo {
    pub head: *const i8,
    pub branch: *const i8,
    pub access_count: size_t,
    pub access: *mut link,
    pub symbols_count: size_t,
    pub symbols: *mut link,
    pub locks_count: size_t,
    pub locks: *mut link,
    pub strict: bool,
    pub integrity: *mut atat,
    pub comment: *mut atat,
    pub expand: i32,
    pub deltas_count: size_t,
    pub deltas: *mut wlink,
    pub desc: *mut atat,
    pub neck: off_t,
    pub lockdefs: *mut lockdef,
    pub ht: *mut hash,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct repository {
    pub filename: *const i8,
    pub fd_lock: i32,
    pub stat: stat,
    pub r: *mut repo,
    pub tip: *mut delta,
    pub log_lead: cbuf,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct flow {
    pub from: *mut fro,
    pub rewr: *mut FILE,
    pub to: *mut FILE,
    pub res: *mut FILE,
    pub result: *const i8,
    pub erroneous: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct top {
    pub program: *const program,
    pub behavior: behavior,
    pub manifestation: manifestation,
    pub repository: repository,
    pub flow: flow,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct partime {
    pub tm: tm,
    pub ymodulus: i32,
    pub yweek: i32,
    pub zone: i64,
}
#[no_mangle]
pub unsafe extern "C" fn local_tm(
    mut timep: *const time_t,
    mut result: *mut tm,
) -> *mut tm {
    if !(*(*top).behavior.maketimestuff).tzset_already_called {
        tzset();
        (*(*top).behavior.maketimestuff).tzset_already_called = 1 as i32 != 0;
    }
    return localtime_r(timep, result);
}
unsafe extern "C" fn isleap(mut y: i32) -> bool {
    return y & 3 as i32 == 0 as i32
        && (y % 100 as i32 != 0 as i32 || y % 400 as i32 == 0 as i32);
}
static mut month_yday: [i32; 13] = [
    0 as i32,
    31 as i32,
    59 as i32,
    90 as i32,
    120 as i32,
    151 as i32,
    181 as i32,
    212 as i32,
    243 as i32,
    273 as i32,
    304 as i32,
    334 as i32,
    365 as i32,
];
unsafe extern "C" fn month_days(mut tm: *const tm) -> i32 {
    let mut m: i32 = (*tm).tm_mon;
    return month_yday[(m + 1 as i32) as usize] - month_yday[m as usize]
        + (m == 1 as i32 && isleap((*tm).tm_year + 1900 as i32) as i32 != 0) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn time2tm(mut unixtime: time_t, mut localzone: bool) -> *mut tm {
    let mut tm: *mut tm = 0 as *mut tm;
    if localzone as i32 != 0
        || {
            tm = gmtime_r(
                &mut unixtime,
                &mut (*(*top).behavior.maketimestuff).time2tm_stash,
            );
            tm.is_null()
        }
    {
        tm = local_tm(
            &mut unixtime,
            &mut (*(*top).behavior.maketimestuff).time2tm_stash,
        );
    }
    return tm;
}
#[no_mangle]
pub unsafe extern "C" fn difftm(mut a: *const tm, mut b: *const tm) -> time_t {
    let mut ay: i32 = (*a).tm_year + (1900 as i32 - 1 as i32);
    let mut by: i32 = (*b).tm_year + (1900 as i32 - 1 as i32);
    let mut difference_in_day_of_year: i32 = (*a).tm_yday - (*b).tm_yday;
    let mut intervening_leap_days: i32 = (ay >> 2 as i32) - (by >> 2 as i32)
        - (ay / 100 as i32 - by / 100 as i32)
        + ((ay / 100 as i32 >> 2 as i32) - (by / 100 as i32 >> 2 as i32));
    let mut difference_in_years: time_t = (ay - by) as time_t;
    let mut difference_in_days: time_t = difference_in_years * 365 as i32 as i64
        + (intervening_leap_days + difference_in_day_of_year) as i64;
    return ((24 as i32 as i64 * difference_in_days
        + ((*a).tm_hour - (*b).tm_hour) as i64) * 60 as i32 as i64
        + ((*a).tm_min - (*b).tm_min) as i64) * 60 as i32 as i64
        + ((*a).tm_sec - (*b).tm_sec) as i64;
}
#[no_mangle]
pub unsafe extern "C" fn adjzone(mut t: *mut tm, mut seconds: i64) {
    let mut leap_second: i32 = ((*t).tm_sec == 60 as i32) as i32;
    let mut sec: i64 = seconds + ((*t).tm_sec - leap_second) as i64;
    if sec < 0 as i32 as i64 {
        (*t).tm_min = ((*t).tm_min as i64 - (59 as i32 as i64 - sec) / 60 as i32 as i64)
            as i32;
        if (*t).tm_min < 0 as i32 {
            (*t).tm_hour -= (59 as i32 - (*t).tm_min) / 60 as i32;
            if (*t).tm_hour < 0 as i32 {
                (*t).tm_hour += 24 as i32;
                if 0 as i32 <= (*t).tm_wday
                    && {
                        (*t).tm_wday -= 1;
                        (*t).tm_wday < 0 as i32
                    }
                {
                    (*t).tm_wday = 6 as i32;
                }
                (*t).tm_mday -= 1;
                if (*t).tm_mday <= 0 as i32 {
                    (*t).tm_mon -= 1;
                    if (*t).tm_mon < 0 as i32 {
                        (*t).tm_year -= 1;
                        (*t).tm_year;
                        (*t).tm_mon = 11 as i32;
                    }
                    (*t).tm_mday = month_days(t);
                }
            }
            (*t).tm_min += 24 as i32 * 60 as i32;
        }
        sec += 24 as i64 * 60 as i32 as i64 * 60 as i32 as i64;
    } else {
        (*t).tm_min = ((*t).tm_min as i64 + sec / 60 as i32 as i64) as i32;
        if 60 as i32 <= (*t).tm_min {
            (*t).tm_hour += (*t).tm_min / 60 as i32;
            if 24 as i32 <= (*t).tm_hour {
                (*t).tm_hour -= 24 as i32;
                if 0 as i32 <= (*t).tm_wday
                    && {
                        (*t).tm_wday += 1;
                        (*t).tm_wday == 7 as i32
                    }
                {
                    (*t).tm_wday = 0 as i32;
                }
                (*t).tm_mday += 1;
                if month_days(t) < (*t).tm_mday {
                    (*t).tm_mon += 1;
                    if (11 as i32) < (*t).tm_mon {
                        (*t).tm_year += 1;
                        (*t).tm_year;
                        (*t).tm_mon = 0 as i32;
                    }
                    (*t).tm_mday = 1 as i32;
                }
            }
        }
    }
    (*t).tm_min %= 60 as i32;
    (*t).tm_sec = (sec % 60 as i32 as i64) as i32 + leap_second;
}
unsafe extern "C" fn ISO_day_of_week(mut zy: i32, mut mij: i32) -> i32 {
    let mut zd: i32 = (mij + 365 as i32 * zy + zy / 4 as i32 - zy / 100 as i32
        + zy / 400 as i32) % 7 as i32;
    return if zd != 0 { zd } else { 7 as i32 };
}
#[no_mangle]
pub unsafe extern "C" fn tm2time(
    mut tm: *mut tm,
    mut localzone: bool,
    mut yweek: i32,
) -> time_t {
    let mut leap: bool = false;
    let mut d: time_t = 0;
    let mut gt: time_t = 0;
    let mut gtm: *const tm = 0 as *const tm;
    let mut remaining_tries: i32 = 8 as i32;
    if 12 as i32 as u32 <= (*tm).tm_mon as u32 {
        return -(1 as i32) as time_t;
    }
    leap = isleap((*tm).tm_year + 1900 as i32);
    if -(1 as i32) != yweek {
        let mut nyd: i32 = 0;
        let mut doy: i32 = 0;
        let wday: i32 = if (*tm).tm_wday != 0 { (*tm).tm_wday } else { 7 as i32 };
        let mut zy: i32 = (*tm).tm_year + 1900 as i32 - 1 as i32;
        if yweek == 0 {
            zy -= 1;
            zy;
            leap = isleap(1 as i32 + zy);
        }
        nyd = ISO_day_of_week(zy, 1 as i32);
        if yweek == 0 {
            yweek = 52 as i32
                + (4 as i32 == nyd || leap as i32 != 0 && 3 as i32 == nyd) as i32;
        }
        doy = yweek * 7 as i32 + wday - 3 as i32 - ISO_day_of_week(zy, 4 as i32);
        if (365 as i32 + leap as i32) < doy {
            doy -= 365 as i32 + leap as i32;
            zy += 1;
            zy;
            leap = isleap(1 as i32 + zy);
        }
        if 1 as i32 > doy {
            zy -= 1;
            zy;
            leap = isleap(1 as i32 + zy);
            doy += 365 as i32 + leap as i32;
        }
        (*tm).tm_year = zy + 1 as i32 - 1900 as i32;
        (*tm).tm_yday = doy - 1 as i32;
    }
    if 0 as i32 > (*tm).tm_yday || (365 as i32) < (*tm).tm_yday {
        (*tm).tm_yday = month_yday[(*tm).tm_mon as usize] + (*tm).tm_mday
            - !(leap as i32 != 0 && (1 as i32) < (*tm).tm_mon) as i32;
    } else {
        let mut mon: i32 = 1 as i32;
        let mut day: i32 = 1 as i32 + (*tm).tm_yday;
        while day
            > month_yday[mon as usize] + (leap as i32 != 0 && (1 as i32) < mon) as i32
        {
            mon += 1;
            mon;
        }
        mon -= 1;
        mon;
        day -= month_yday[mon as usize] + (leap as i32 != 0 && (1 as i32) < mon) as i32;
        (*tm).tm_mon = mon;
        (*tm).tm_mday = day;
    }
    gt = (*(*top).behavior.maketimestuff).t_cache[localzone as usize];
    gtm = if gt != 0 {
        &mut *((*(*top).behavior.maketimestuff).tm_cache)
            .as_mut_ptr()
            .offset(localzone as isize) as *mut tm
    } else {
        time2tm(gt, localzone)
    };
    loop {
        d = difftm(tm, gtm);
        if !(d != 0 as i32 as i64) {
            break;
        }
        remaining_tries -= 1;
        if remaining_tries == 0 as i32 {
            return -(1 as i32) as time_t;
        }
        gt += d;
        gtm = time2tm(gt, localzone);
    }
    (*(*top).behavior.maketimestuff).t_cache[localzone as usize] = gt;
    (*(*top).behavior.maketimestuff).tm_cache[localzone as usize] = *gtm;
    if (*tm).tm_year ^ (*gtm).tm_year | (*tm).tm_mon ^ (*gtm).tm_mon
        | (*tm).tm_mday ^ (*gtm).tm_mday | (*tm).tm_hour ^ (*gtm).tm_hour
        | (*tm).tm_min ^ (*gtm).tm_min | (*tm).tm_sec ^ (*gtm).tm_sec != 0
    {
        return -(1 as i32) as time_t;
    }
    (*tm).tm_wday = (*gtm).tm_wday;
    return gt;
}
unsafe extern "C" fn maketime(
    mut pt: *const partime,
    mut default_time: time_t,
) -> time_t {
    let mut localzone: bool = (*pt).zone
        == -(24 as i32) as i64 * 60 as i32 as i64 * 60 as i32 as i64 - 1 as i32 as i64;
    let mut wday: i32 = 0;
    let mut tm: tm = tm {
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
    let mut tm0: *mut tm = 0 as *mut tm;
    let mut r: time_t = 0;
    tm0 = 0 as *mut tm;
    tm = (*pt).tm;
    if 0 as i32 <= (*pt).ymodulus || !(0 as i32 <= tm.tm_year) {
        tm0 = time2tm(default_time, localzone);
        if !localzone {
            adjzone(tm0, (*pt).zone);
        }
    }
    if 0 as i32 <= (*pt).ymodulus {
        tm.tm_year += ((*tm0).tm_year + 1900 as i32) / (*pt).ymodulus * (*pt).ymodulus;
    } else if !(0 as i32 <= tm.tm_year) {
        tm.tm_year = (*tm0).tm_year + 1900 as i32;
        if !(0 as i32 <= tm.tm_mon) {
            tm.tm_mon = (*tm0).tm_mon;
            if !(0 as i32 <= tm.tm_mday) {
                tm.tm_mday = (*tm0).tm_mday;
            }
        }
    }
    tm.tm_year -= 1900 as i32;
    if !(0 as i32 <= tm.tm_mon) {
        tm.tm_mon = 0 as i32;
    }
    if !(0 as i32 <= tm.tm_mday) {
        tm.tm_mday = 1 as i32;
    }
    if !(0 as i32 <= tm.tm_hour) {
        tm.tm_hour = 0 as i32;
    }
    if !(0 as i32 <= tm.tm_min) {
        tm.tm_min = 0 as i32;
    }
    if !(0 as i32 <= tm.tm_sec) {
        tm.tm_sec = 0 as i32;
    }
    if !localzone {
        adjzone(&mut tm, -(*pt).zone);
    }
    wday = tm.tm_wday;
    r = tm2time(&mut tm, localzone, (*pt).yweek);
    if r != -(1 as i32) as i64 && 0 as i32 <= wday && wday != tm.tm_wday {
        return -(1 as i32) as time_t;
    }
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn str2time(
    mut source: *const i8,
    mut default_time: time_t,
    mut default_zone: i64,
) -> time_t {
    let mut pt: partime = partime {
        tm: tm {
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
        },
        ymodulus: 0,
        yweek: 0,
        zone: 0,
    };
    if *partime(source, &mut pt) != 0 {
        return -(1 as i32) as time_t;
    }
    if pt.zone == -(24 as i32) as i64 * 60 as i32 as i64 * 60 as i32 as i64 {
        pt.zone = default_zone;
    }
    return maketime(&mut pt, default_time);
}