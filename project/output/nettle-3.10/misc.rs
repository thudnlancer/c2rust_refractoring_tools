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
    static mut stderr: *mut _IO_FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn vfprintf(_: *mut FILE, _: *const i8, _: ::core::ffi::VaList) -> i32;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn abort() -> !;
    fn exit(_: i32) -> !;
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
pub type va_list = __builtin_va_list;
pub type __off_t = i64;
pub type __off64_t = i64;
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
#[no_mangle]
pub unsafe extern "C" fn die(mut format: *const i8, mut args: ...) -> ! {
    let mut args_0: ::core::ffi::VaListImpl;
    args_0 = args.clone();
    vfprintf(stderr, format, args_0.as_va_list());
    exit(1 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn werror(mut format: *const i8, mut args: ...) {
    let mut args_0: ::core::ffi::VaListImpl;
    args_0 = args.clone();
    vfprintf(stderr, format, args_0.as_va_list());
}
#[no_mangle]
pub unsafe extern "C" fn xalloc(mut size: size_t) -> *mut libc::c_void {
    let mut p: *mut libc::c_void = malloc(size);
    if p.is_null() {
        fprintf(stderr, b"Virtual memory exhausted.\n\0" as *const u8 as *const i8);
        abort();
    }
    return p;
}
#[no_mangle]
pub static mut sexp_token_chars: [i8; 128] = [
    0 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    1 as i32 as i8,
    1 as i32 as i8,
    0 as i32 as i8,
    1 as i32 as i8,
    1 as i32 as i8,
    1 as i32 as i8,
    1 as i32 as i8,
    1 as i32 as i8,
    1 as i32 as i8,
    1 as i32 as i8,
    1 as i32 as i8,
    1 as i32 as i8,
    1 as i32 as i8,
    1 as i32 as i8,
    1 as i32 as i8,
    1 as i32 as i8,
    1 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    1 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    1 as i32 as i8,
    1 as i32 as i8,
    1 as i32 as i8,
    1 as i32 as i8,
    1 as i32 as i8,
    1 as i32 as i8,
    1 as i32 as i8,
    1 as i32 as i8,
    1 as i32 as i8,
    1 as i32 as i8,
    1 as i32 as i8,
    1 as i32 as i8,
    1 as i32 as i8,
    1 as i32 as i8,
    1 as i32 as i8,
    1 as i32 as i8,
    1 as i32 as i8,
    1 as i32 as i8,
    1 as i32 as i8,
    1 as i32 as i8,
    1 as i32 as i8,
    1 as i32 as i8,
    1 as i32 as i8,
    1 as i32 as i8,
    1 as i32 as i8,
    1 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    1 as i32 as i8,
    0 as i32 as i8,
    1 as i32 as i8,
    1 as i32 as i8,
    1 as i32 as i8,
    1 as i32 as i8,
    1 as i32 as i8,
    1 as i32 as i8,
    1 as i32 as i8,
    1 as i32 as i8,
    1 as i32 as i8,
    1 as i32 as i8,
    1 as i32 as i8,
    1 as i32 as i8,
    1 as i32 as i8,
    1 as i32 as i8,
    1 as i32 as i8,
    1 as i32 as i8,
    1 as i32 as i8,
    1 as i32 as i8,
    1 as i32 as i8,
    1 as i32 as i8,
    1 as i32 as i8,
    1 as i32 as i8,
    1 as i32 as i8,
    1 as i32 as i8,
    1 as i32 as i8,
    1 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
    0 as i32 as i8,
];