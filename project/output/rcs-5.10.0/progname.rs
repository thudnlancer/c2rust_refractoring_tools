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
    static mut program_invocation_name: *mut i8;
    static mut program_invocation_short_name: *mut i8;
    static mut stderr: *mut _IO_FILE;
    fn fputs(__s: *const i8, __stream: *mut FILE) -> i32;
    fn abort() -> !;
    fn strncmp(_: *const i8, _: *const i8, _: u64) -> i32;
    fn strrchr(_: *const i8, _: i32) -> *mut i8;
}
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
pub type size_t = u64;
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
#[no_mangle]
pub static mut program_name: *const i8 = 0 as *const i8;
#[no_mangle]
pub unsafe extern "C" fn set_program_name(mut argv0: *const i8) {
    let mut slash: *const i8 = 0 as *const i8;
    let mut base: *const i8 = 0 as *const i8;
    if argv0.is_null() {
        fputs(
            b"A NULL argv[0] was passed through an exec system call.\n\0" as *const u8
                as *const i8,
            stderr,
        );
        abort();
    }
    slash = strrchr(argv0, '/' as i32);
    base = if !slash.is_null() { slash.offset(1 as i32 as isize) } else { argv0 };
    if base.offset_from(argv0) as i64 >= 7 as i32 as i64
        && strncmp(
            base.offset(-(7 as i32 as isize)),
            b"/.libs/\0" as *const u8 as *const i8,
            7 as i32 as u64,
        ) == 0 as i32
    {
        argv0 = base;
        if strncmp(base, b"lt-\0" as *const u8 as *const i8, 3 as i32 as u64) == 0 as i32
        {
            argv0 = base.offset(3 as i32 as isize);
            program_invocation_short_name = argv0 as *mut i8;
        }
    }
    program_name = argv0;
    program_invocation_name = argv0 as *mut i8;
}