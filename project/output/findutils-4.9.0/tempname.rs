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
    fn __errno_location() -> *mut i32;
    fn strlen(_: *const i8) -> u64;
    fn strspn(_: *const i8, _: *const i8) -> u64;
    fn open(__file: *const i8, __oflag: i32, _: ...) -> i32;
    fn getrandom(__buffer: *mut libc::c_void, __length: size_t, __flags: u32) -> ssize_t;
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> i32;
    fn mkdir(__path: *const i8, __mode: __mode_t) -> i32;
    fn __lxstat(__ver: i32, __filename: *const i8, __stat_buf: *mut stat) -> i32;
}
pub type size_t = u64;
pub type __dev_t = u64;
pub type __uid_t = u32;
pub type __gid_t = u32;
pub type __ino_t = u64;
pub type __mode_t = u32;
pub type __nlink_t = u64;
pub type __off_t = i64;
pub type __time_t = i64;
pub type __clockid_t = i32;
pub type __blksize_t = i64;
pub type __blkcnt_t = i64;
pub type __ssize_t = i64;
pub type __syscall_slong_t = i64;
pub type ssize_t = __ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct max_align_t {
    pub __clang_max_align_nonce1: libc::c_longlong,
    pub __clang_max_align_nonce2: f128::f128,
}
pub type clockid_t = __clockid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
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
pub type random_value = uint_fast64_t;
pub type uint_fast64_t = u64;
pub type uintptr_t = u64;
#[inline]
unsafe extern "C" fn lstat(mut __path: *const i8, mut __statbuf: *mut stat) -> i32 {
    return __lxstat(1 as i32, __path, __statbuf);
}
unsafe extern "C" fn random_bits(
    mut var: random_value,
    mut use_getrandom: bool,
) -> random_value {
    let mut r: random_value = 0;
    if use_getrandom as i32 != 0
        && getrandom(
            &mut r as *mut random_value as *mut libc::c_void,
            ::core::mem::size_of::<random_value>() as u64,
            0x1 as i32 as u32,
        ) as u64 == ::core::mem::size_of::<random_value>() as u64
    {
        return r;
    }
    let mut tv: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    clock_gettime(1 as i32, &mut tv);
    var ^= tv.tv_nsec as u64;
    return (2862933555777941757 as i64 as u64)
        .wrapping_mul(var)
        .wrapping_add(3037000493 as i64 as u64);
}
unsafe extern "C" fn try_file(mut tmpl: *mut i8, mut flags: *mut libc::c_void) -> i32 {
    let mut openflags: *mut i32 = flags as *mut i32;
    return open(
        tmpl,
        *openflags & !(0o3 as i32) | 0o2 as i32 | 0o100 as i32 | 0o200 as i32,
        0o400 as i32 | 0o200 as i32,
    );
}
unsafe extern "C" fn try_dir(mut tmpl: *mut i8, mut flags: *mut libc::c_void) -> i32 {
    return mkdir(tmpl, (0o400 as i32 | 0o200 as i32 | 0o100 as i32) as __mode_t);
}
unsafe extern "C" fn try_nocreate(
    mut tmpl: *mut i8,
    mut flags: *mut libc::c_void,
) -> i32 {
    let mut st: stat = stat {
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
    if lstat(tmpl, &mut st) == 0 as i32 || *__errno_location() == 75 as i32 {
        *__errno_location() = 17 as i32;
    }
    return if *__errno_location() == 2 as i32 { 0 as i32 } else { -(1 as i32) };
}
static mut letters: [i8; 63] = unsafe {
    *::core::mem::transmute::<
        &[u8; 63],
        &[i8; 63],
    >(b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789\0")
};
#[no_mangle]
pub unsafe extern "C" fn gen_tempname_len(
    mut tmpl: *mut i8,
    mut suffixlen: i32,
    mut flags: i32,
    mut kind: i32,
    mut x_suffix_len: size_t,
) -> i32 {
    static mut tryfunc: [Option<
        unsafe extern "C" fn(*mut i8, *mut libc::c_void) -> i32,
    >; 3] = unsafe {
        [
            Some(try_file as unsafe extern "C" fn(*mut i8, *mut libc::c_void) -> i32),
            Some(try_dir as unsafe extern "C" fn(*mut i8, *mut libc::c_void) -> i32),
            Some(try_nocreate as unsafe extern "C" fn(*mut i8, *mut libc::c_void) -> i32),
        ]
    };
    return try_tempname_len(
        tmpl,
        suffixlen,
        &mut flags as *mut i32 as *mut libc::c_void,
        tryfunc[kind as usize],
        x_suffix_len,
    );
}
#[no_mangle]
pub unsafe extern "C" fn try_tempname_len(
    mut tmpl: *mut i8,
    mut suffixlen: i32,
    mut args: *mut libc::c_void,
    mut tryfunc: Option<unsafe extern "C" fn(*mut i8, *mut libc::c_void) -> i32>,
    mut x_suffix_len: size_t,
) -> i32 {
    let mut len: size_t = 0;
    let mut XXXXXX: *mut i8 = 0 as *mut i8;
    let mut count: u32 = 0;
    let mut fd: i32 = -(1 as i32);
    let mut save_errno: i32 = *__errno_location();
    let mut attempts: u32 = (62 as i32 * 62 as i32 * 62 as i32) as u32;
    let mut v: random_value = 0;
    v = (&mut v as *mut random_value as uintptr_t)
        .wrapping_div(::core::mem::align_of::<max_align_t>() as u64);
    let mut vdigits: i32 = 0 as i32;
    let mut use_getrandom: bool = tryfunc
        == Some(try_nocreate as unsafe extern "C" fn(*mut i8, *mut libc::c_void) -> i32);
    let unfair_min: random_value = (18446744073709551615 as u64 as libc::c_ulonglong)
        .wrapping_sub(
            (18446744073709551615 as u64 as libc::c_ulonglong)
                .wrapping_rem(
                    (62 as libc::c_longlong * 62 as i32 as libc::c_longlong
                        * 62 as i32 as libc::c_longlong * 62 as i32 as libc::c_longlong
                        * 62 as i32 as libc::c_longlong * 62 as i32 as libc::c_longlong
                        * 62 as i32 as libc::c_longlong * 62 as i32 as libc::c_longlong
                        * 62 as i32 as libc::c_longlong * 62 as i32 as libc::c_longlong)
                        as libc::c_ulonglong,
                ),
        ) as random_value;
    len = strlen(tmpl);
    if len < x_suffix_len.wrapping_add(suffixlen as u64)
        || strspn(
            &mut *tmpl
                .offset(
                    len.wrapping_sub(x_suffix_len).wrapping_sub(suffixlen as u64)
                        as isize,
                ),
            b"X\0" as *const u8 as *const i8,
        ) < x_suffix_len
    {
        *__errno_location() = 22 as i32;
        return -(1 as i32);
    }
    XXXXXX = &mut *tmpl
        .offset(len.wrapping_sub(x_suffix_len).wrapping_sub(suffixlen as u64) as isize)
        as *mut i8;
    count = 0 as i32 as u32;
    while count < attempts {
        let mut i: size_t = 0 as i32 as size_t;
        while i < x_suffix_len {
            if vdigits == 0 as i32 {
                loop {
                    v = random_bits(v, use_getrandom);
                    use_getrandom = 1 as i32 != 0;
                    if !(unfair_min <= v) {
                        break;
                    }
                }
                vdigits = 10 as i32;
            }
            *XXXXXX.offset(i as isize) = letters[v.wrapping_rem(62 as i32 as u64)
                as usize];
            v = (v as u64).wrapping_div(62 as i32 as u64) as random_value
                as random_value;
            vdigits -= 1;
            vdigits;
            i = i.wrapping_add(1);
            i;
        }
        fd = tryfunc.expect("non-null function pointer")(tmpl, args);
        if fd >= 0 as i32 {
            *__errno_location() = save_errno;
            return fd;
        } else if *__errno_location() != 17 as i32 {
            return -(1 as i32)
        }
        count = count.wrapping_add(1);
        count;
    }
    *__errno_location() = 17 as i32;
    return -(1 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn gen_tempname(
    mut tmpl: *mut i8,
    mut suffixlen: i32,
    mut flags: i32,
    mut kind: i32,
) -> i32 {
    return gen_tempname_len(tmpl, suffixlen, flags, kind, 6 as i32 as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn try_tempname(
    mut tmpl: *mut i8,
    mut suffixlen: i32,
    mut args: *mut libc::c_void,
    mut tryfunc: Option<unsafe extern "C" fn(*mut i8, *mut libc::c_void) -> i32>,
) -> i32 {
    return try_tempname_len(tmpl, suffixlen, args, tryfunc, 6 as i32 as size_t);
}