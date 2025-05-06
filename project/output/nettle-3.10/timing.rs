#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(c_variadic)]
extern "C" {
    fn __errno_location() -> *mut i32;
    static mut stderr: *mut _IO_FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn vfprintf(_: *mut FILE, _: *const i8, _: ::core::ffi::VaList) -> i32;
    fn exit(_: i32) -> !;
    fn strerror(_: i32) -> *mut i8;
    fn clock() -> clock_t;
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> i32;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: u32,
    pub fp_offset: u32,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type size_t = u64;
pub type __clock_t = i64;
pub type clock_t = __clock_t;
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
pub type __off64_t = i64;
pub type _IO_lock_t = ();
pub type __off_t = i64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: i32,
}
pub type FILE = _IO_FILE;
pub type __syscall_slong_t = i64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type __time_t = i64;
pub type va_list = __builtin_va_list;
pub type clockid_t = __clockid_t;
pub type __clockid_t = i32;
#[no_mangle]
pub static mut cgt_start: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
unsafe extern "C" fn die(mut format: *const i8, mut args: ...) -> ! {
    let mut args_0: ::core::ffi::VaListImpl;
    args_0 = args.clone();
    vfprintf(stderr, format, args_0.as_va_list());
    exit(1 as i32);
}
unsafe extern "C" fn cgt_works_p() -> i32 {
    let mut now: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    return (clock_gettime(2 as i32, &mut now) == 0 as i32) as i32;
}
unsafe extern "C" fn cgt_time_start() {
    if clock_gettime(2 as i32, &mut cgt_start) < 0 as i32 {
        die(
            b"clock_gettime failed: %s\n\0" as *const u8 as *const i8,
            strerror(*__errno_location()),
        );
    }
}
unsafe extern "C" fn cgt_time_end() -> libc::c_double {
    let mut end: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    if clock_gettime(2 as i32, &mut end) < 0 as i32 {
        die(
            b"clock_gettime failed: %s\n\0" as *const u8 as *const i8,
            strerror(*__errno_location()),
        );
    }
    return (end.tv_sec - cgt_start.tv_sec) as libc::c_double
        + 1e-9f64 * (end.tv_nsec - cgt_start.tv_nsec) as libc::c_double;
}
static mut clock_start: clock_t = 0;
unsafe extern "C" fn clock_time_start() {
    clock_start = clock();
}
unsafe extern "C" fn clock_time_end() -> libc::c_double {
    return (clock() - clock_start) as libc::c_double
        / 1000000 as i32 as __clock_t as libc::c_double;
}
#[no_mangle]
pub static mut time_start: Option<unsafe extern "C" fn() -> ()> = unsafe {
    Some(clock_time_start as unsafe extern "C" fn() -> ())
};
#[no_mangle]
pub static mut time_end: Option<unsafe extern "C" fn() -> libc::c_double> = unsafe {
    Some(clock_time_end as unsafe extern "C" fn() -> libc::c_double)
};
#[no_mangle]
pub unsafe extern "C" fn time_init() {
    if cgt_works_p() != 0 {
        time_start = Some(cgt_time_start as unsafe extern "C" fn() -> ());
        time_end = Some(cgt_time_end as unsafe extern "C" fn() -> libc::c_double);
    } else {
        fprintf(
            stderr,
            b"clock_gettime not working, falling back to clock\n\0" as *const u8
                as *const i8,
        );
        time_start = Some(clock_time_start as unsafe extern "C" fn() -> ());
        time_end = Some(clock_time_end as unsafe extern "C" fn() -> libc::c_double);
    };
}