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
    fn rpl_sprintf(str: *mut i8, format: *const i8, _: ...) -> i32;
    static mut top: *mut top;
    fn strtol(__nptr: *const i8, __endptr: *mut *mut i8, __base: i32) -> i64;
    fn generic_error(who: *const i8, fmt: *const i8, _: ...);
    fn generic_fatal(who: *const i8, fmt: *const i8, _: ...);
    fn parzone(s: *const i8, zone: *mut i64) -> *const i8;
    fn local_tm(timep: *const time_t, result: *mut tm) -> *mut tm;
    fn time2tm(unixtime: time_t, localzone: bool) -> *mut tm;
    fn difftm(a: *const tm, b: *const tm) -> time_t;
    fn str2time(source: *const i8, default_time: time_t, default_zone: i64) -> time_t;
    fn tm2time(tm: *mut tm, localzone: bool, yweek: i32) -> time_t;
    fn adjzone(t: *mut tm, seconds: i64);
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
#[no_mangle]
pub unsafe extern "C" fn time2date(mut unixtime: time_t, mut date: *mut i8) {
    let mut tm: *const tm = time2tm(
        unixtime,
        (*top).behavior.version < 5 as i32 - 5 as i32,
    );
    rpl_sprintf(
        date,
        b"%.2d.%.2d.%.2d.%.2d.%.2d.%.2d\0" as *const u8 as *const i8,
        (*tm).tm_year
            + (if ((*tm).tm_year as u32) < 100 as i32 as u32 {
                0 as i32
            } else {
                1900 as i32
            }),
        (*tm).tm_mon + 1 as i32,
        (*tm).tm_mday,
        (*tm).tm_hour,
        (*tm).tm_min,
        (*tm).tm_sec,
    );
}
unsafe extern "C" fn str2time_checked(
    mut source: *const i8,
    mut default_time: time_t,
    mut default_zone: i64,
) -> time_t {
    let mut t: time_t = str2time(source, default_time, default_zone);
    if t == -(1 as i32) as i64 {
        generic_fatal(
            0 as *const i8,
            b"unknown date/time: %s\0" as *const u8 as *const i8,
            source,
        );
    }
    return t;
}
#[no_mangle]
pub unsafe extern "C" fn str2date(mut source: *const i8, mut target: *mut i8) {
    time2date(
        str2time_checked(
            source,
            (*top).behavior.now.tv_sec,
            if (*top).behavior.zone_offset.valid as i32 != 0 {
                (*top).behavior.zone_offset.seconds
            } else if (*top).behavior.version < 5 as i32 - 5 as i32 {
                -(24 as i32) as i64 * 60 as i32 as i64 * 60 as i32 as i64
                    - 1 as i32 as i64
            } else {
                0 as i32 as i64
            },
        ),
        target,
    );
}
#[no_mangle]
pub unsafe extern "C" fn date2time(mut source: *const i8) -> time_t {
    let mut s: [i8; 31] = [0; 31];
    return str2time_checked(
        date2str(source, s.as_mut_ptr()),
        0 as i32 as time_t,
        0 as i32 as i64,
    );
}
#[no_mangle]
pub unsafe extern "C" fn zone_set(mut s: *const i8) {
    (*top).behavior.zone_offset.valid = *s != 0;
    if (*top).behavior.zone_offset.valid {
        let mut zone: i64 = 0;
        let mut zonetail: *const i8 = parzone(s, &mut zone);
        if zonetail.is_null() || *zonetail as i32 != 0 {
            generic_error(
                0 as *const i8,
                b"%s: not a known time zone\0" as *const u8 as *const i8,
                s,
            );
        } else {
            (*top).behavior.zone_offset.seconds = zone;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn date2str(
    mut date: *const i8,
    mut datebuf: *mut i8,
) -> *const i8 {
    let mut p: *const i8 = date;
    loop {
        let fresh0 = p;
        p = p.offset(1);
        if !(*fresh0 as i32 != '.' as i32) {
            break;
        }
    }
    if !(*top).behavior.zone_offset.valid {
        rpl_sprintf(
            datebuf,
            (b"19%.*s/%.2s/%.2s %.2s:%.2s:%s\0" as *const u8 as *const i8)
                .offset(
                    (if *date.offset(2 as i32 as isize) as i32 == '.' as i32
                        && 5 as i32 - 5 as i32 <= (*top).behavior.version
                    {
                        0 as i32
                    } else {
                        2 as i32
                    }) as isize,
                ),
            (p.offset_from(date) as i64 - 1 as i32 as i64) as i32,
            date,
            p,
            p.offset(3 as i32 as isize),
            p.offset(6 as i32 as isize),
            p.offset(9 as i32 as isize),
            p.offset(12 as i32 as isize),
        );
    } else {
        let mut q: *mut i8 = 0 as *mut i8;
        let mut t: tm = tm {
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
        let mut z: *const tm = 0 as *const tm;
        let mut z_stash: tm = tm {
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
        let mut non_hour: i32 = 0;
        let mut w: i32 = 0;
        let mut zone: i64 = 0;
        let mut c: i8 = 0;
        p = date;
        t.tm_year = strtol(p, &mut q, 10 as i32) as i32;
        p = q.offset(1 as i32 as isize);
        if '.' as i32 != *date.offset(2 as i32 as isize) as i32 {
            t.tm_year -= 1900 as i32;
        }
        t.tm_mon = strtol(p, &mut q, 10 as i32) as i32;
        p = q.offset(1 as i32 as isize);
        t.tm_mon -= 1;
        t.tm_mon;
        t.tm_mday = strtol(p, &mut q, 10 as i32) as i32;
        p = q.offset(1 as i32 as isize);
        t.tm_hour = strtol(p, &mut q, 10 as i32) as i32;
        p = q.offset(1 as i32 as isize);
        t.tm_min = strtol(p, &mut q, 10 as i32) as i32;
        p = q.offset(1 as i32 as isize);
        t.tm_sec = strtol(p, &mut q, 10 as i32) as i32;
        p = q.offset(1 as i32 as isize);
        t.tm_wday = -(1 as i32);
        t.tm_yday = -(1 as i32);
        zone = (*top).behavior.zone_offset.seconds;
        if zone
            == -(24 as i32) as i64 * 60 as i32 as i64 * 60 as i32 as i64
                - 1 as i32 as i64
        {
            let mut u: time_t = tm2time(&mut t, 0 as i32 != 0, -(1 as i32));
            let mut d: time_t = 0;
            z = local_tm(&mut u, &mut z_stash);
            d = difftm(z, &mut t);
            zone = if (-(1 as i32) as time_t) < 0 as i32 as i64 || d < -d {
                d
            } else {
                --d
            };
        } else {
            adjzone(&mut t, zone);
            z = &mut t;
        }
        c = '+' as i32 as i8;
        if zone < 0 as i32 as i64 {
            zone = -zone;
            c = '-' as i32 as i8;
        }
        w = rpl_sprintf(
            datebuf,
            b"%.2d-%.2d-%.2d %.2d:%.2d:%.2d%c%.2d\0" as *const u8 as *const i8,
            (*z).tm_year + 1900 as i32,
            (*z).tm_mon + 1 as i32,
            (*z).tm_mday,
            (*z).tm_hour,
            (*z).tm_min,
            (*z).tm_sec,
            c as i32,
            (zone / (60 as i32 * 60 as i32) as i64) as i32,
        );
        non_hour = (zone % (60 as i32 * 60 as i32) as i64) as i32;
        if non_hour != 0 {
            let mut fmt: *const i8 = b":%.2d\0" as *const u8 as *const i8;
            w += rpl_sprintf(datebuf.offset(w as isize), fmt, non_hour / 60 as i32);
            non_hour %= 60 as i32;
            if non_hour != 0 {
                w += rpl_sprintf(datebuf.offset(w as isize), fmt, non_hour);
            }
        }
    }
    return datebuf as *const i8;
}