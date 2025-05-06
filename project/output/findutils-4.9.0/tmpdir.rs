#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn secure_getenv(__name: *const i8) -> *mut i8;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strlen(_: *const i8) -> u64;
    fn __errno_location() -> *mut i32;
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn __xstat(__ver: i32, __filename: *const i8, __stat_buf: *mut stat) -> i32;
}
pub type size_t = u64;
pub type __mode_t = u32;
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
pub type __syscall_slong_t = i64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type __time_t = i64;
pub type __blkcnt_t = i64;
pub type __blksize_t = i64;
pub type __off_t = i64;
pub type __dev_t = u64;
pub type __gid_t = u32;
pub type __uid_t = u32;
pub type __nlink_t = u64;
pub type __ino_t = u64;
#[inline]
unsafe extern "C" fn stat(mut __path: *const i8, mut __statbuf: *mut stat) -> i32 {
    return __xstat(1 as i32, __path, __statbuf);
}
unsafe extern "C" fn direxists(mut dir: *const i8) -> bool {
    let mut buf: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    return stat(dir, &mut buf) == 0 as i32
        && buf.st_mode & 0o170000 as i32 as u32 == 0o40000 as i32 as u32;
}
#[no_mangle]
pub unsafe extern "C" fn path_search(
    mut tmpl: *mut i8,
    mut tmpl_len: size_t,
    mut dir: *const i8,
    mut pfx: *const i8,
    mut try_tmpdir: bool,
) -> i32 {
    let mut d: *const i8 = 0 as *const i8;
    let mut dlen: size_t = 0;
    let mut plen: size_t = 0;
    let mut add_slash: bool = false;
    if pfx.is_null() || *pfx.offset(0 as i32 as isize) == 0 {
        pfx = b"file\0" as *const u8 as *const i8;
        plen = 4 as i32 as size_t;
    } else {
        plen = strlen(pfx);
        if plen > 5 as i32 as u64 {
            plen = 5 as i32 as size_t;
        }
    }
    if try_tmpdir {
        d = secure_getenv(b"TMPDIR\0" as *const u8 as *const i8);
        if !d.is_null() && direxists(d) as i32 != 0 {
            dir = d;
        } else if !(!dir.is_null() && direxists(dir) as i32 != 0) {
            dir = 0 as *const i8;
        }
    }
    if dir.is_null() {
        if direxists(b"/tmp\0" as *const u8 as *const i8) {
            dir = b"/tmp\0" as *const u8 as *const i8;
        } else if strcmp(
            b"/tmp\0" as *const u8 as *const i8,
            b"/tmp\0" as *const u8 as *const i8,
        ) != 0 as i32 && direxists(b"/tmp\0" as *const u8 as *const i8) as i32 != 0
        {
            dir = b"/tmp\0" as *const u8 as *const i8;
        } else {
            *__errno_location() = 2 as i32;
            return -(1 as i32);
        }
    }
    dlen = strlen(dir);
    add_slash = dlen != 0 as i32 as u64
        && !(*dir.offset(dlen.wrapping_sub(1 as i32 as u64) as isize) as i32
            == '/' as i32);
    if tmpl_len
        < dlen
            .wrapping_add(add_slash as u64)
            .wrapping_add(plen)
            .wrapping_add(6 as i32 as u64)
            .wrapping_add(1 as i32 as u64)
    {
        *__errno_location() = 22 as i32;
        return -(1 as i32);
    }
    memcpy(tmpl as *mut libc::c_void, dir as *const libc::c_void, dlen);
    sprintf(
        tmpl.offset(dlen as isize),
        &*(b"/%.*sXXXXXX\0" as *const u8 as *const i8).offset(!add_slash as i32 as isize)
            as *const i8,
        plen as i32,
        pfx,
    );
    return 0 as i32;
}