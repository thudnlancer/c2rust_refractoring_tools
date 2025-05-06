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
    fn glp_vprintf(fmt: *const i8, arg: ::core::ffi::VaList);
    fn glp_printf(fmt: *const i8, _: ...);
    fn _glp_get_env_ptr() -> *mut ENV;
    fn abort() -> !;
    fn exit(_: i32) -> !;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn strlen(_: *const i8) -> u64;
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
pub type __off_t = i64;
pub type __off64_t = i64;
pub type va_list = __builtin_va_list;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ENV {
    pub self_0: *mut ENV,
    pub term_buf: *mut i8,
    pub term_out: i32,
    pub term_hook: Option<unsafe extern "C" fn(*mut libc::c_void, *const i8) -> i32>,
    pub term_info: *mut libc::c_void,
    pub tee_file: *mut FILE,
    pub err_st: i32,
    pub err_file: *const i8,
    pub err_line: i32,
    pub err_hook: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub err_info: *mut libc::c_void,
    pub err_buf: *mut i8,
    pub mem_limit: size_t,
    pub mem_ptr: *mut MBD,
    pub mem_count: i32,
    pub mem_cpeak: i32,
    pub mem_total: size_t,
    pub mem_tpeak: size_t,
    pub gmp_pool: *mut libc::c_void,
    pub gmp_size: i32,
    pub gmp_work: *mut libc::c_ushort,
    pub h_odbc: *mut libc::c_void,
    pub h_mysql: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MBD {
    pub size: size_t,
    pub self_0: *mut MBD,
    pub prev: *mut MBD,
    pub next: *mut MBD,
}
pub type glp_errfunc = Option<unsafe extern "C" fn(*const i8, ...) -> ()>;
unsafe extern "C" fn errfunc(mut fmt: *const i8, mut args: ...) {
    let mut env: *mut ENV = _glp_get_env_ptr();
    let mut arg: ::core::ffi::VaListImpl;
    (*env).err_st = 1 as i32;
    (*env).term_out = 1 as i32;
    arg = args.clone();
    glp_vprintf(fmt, arg.as_va_list());
    glp_printf(
        b"Error detected in file %s at line %d\n\0" as *const u8 as *const i8,
        (*env).err_file,
        (*env).err_line,
    );
    if ((*env).err_hook).is_some() {
        ((*env).err_hook).expect("non-null function pointer")((*env).err_info);
    }
    abort();
}
#[no_mangle]
pub unsafe extern "C" fn glp_error_(mut file: *const i8, mut line: i32) -> glp_errfunc {
    let mut env: *mut ENV = _glp_get_env_ptr();
    (*env).err_file = file;
    (*env).err_line = line;
    return Some(errfunc as unsafe extern "C" fn(*const i8, ...) -> ());
}
#[no_mangle]
pub unsafe extern "C" fn glp_at_error() -> i32 {
    let mut env: *mut ENV = _glp_get_env_ptr();
    return (*env).err_st;
}
#[no_mangle]
pub unsafe extern "C" fn glp_assert_(
    mut expr: *const i8,
    mut file: *const i8,
    mut line: i32,
) {
    (glp_error_(file, line))
        .expect(
            "non-null function pointer",
        )(b"Assertion failed: %s\n\0" as *const u8 as *const i8, expr);
}
#[no_mangle]
pub unsafe extern "C" fn glp_error_hook(
    mut func: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    mut info: *mut libc::c_void,
) {
    let mut env: *mut ENV = _glp_get_env_ptr();
    if func.is_none() {
        (*env).err_hook = None;
        (*env).err_info = 0 as *mut libc::c_void;
    } else {
        (*env).err_hook = func;
        (*env).err_info = info;
    };
}
#[no_mangle]
pub unsafe extern "C" fn _glp_put_err_msg(mut msg: *const i8) {
    let mut env: *mut ENV = _glp_get_env_ptr();
    let mut len: i32 = 0;
    len = strlen(msg) as i32;
    if len >= 1024 as i32 {
        len = 1024 as i32 - 1 as i32;
    }
    memcpy((*env).err_buf as *mut libc::c_void, msg as *const libc::c_void, len as u64);
    if len > 0 as i32
        && *((*env).err_buf).offset((len - 1 as i32) as isize) as i32 == '\n' as i32
    {
        len -= 1;
        len;
    }
    *((*env).err_buf).offset(len as isize) = '\0' as i32 as i8;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_get_err_msg() -> *const i8 {
    let mut env: *mut ENV = _glp_get_env_ptr();
    return (*env).err_buf;
}