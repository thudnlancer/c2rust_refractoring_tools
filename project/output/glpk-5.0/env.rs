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
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn fflush(__stream: *mut FILE) -> i32;
    static mut stderr: *mut _IO_FILE;
    fn abort() -> !;
    fn _glp_dlclose(h: *mut libc::c_void);
    fn fclose(__stream: *mut FILE) -> i32;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn _glp_tls_get_ptr() -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn _glp_tls_set_ptr(ptr: *mut libc::c_void);
}
pub type size_t = u64;
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
pub type FILE = _IO_FILE;
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
#[no_mangle]
pub unsafe extern "C" fn glp_init_env() -> i32 {
    let mut env: *mut ENV = 0 as *mut ENV;
    let mut ok: i32 = 0;
    ok = (8 as i32 == 8 as i32 && ::core::mem::size_of::<i8>() as u64 == 1 as i32 as u64
        && ::core::mem::size_of::<libc::c_short>() as u64 == 2 as i32 as u64
        && ::core::mem::size_of::<i32>() as u64 == 4 as i32 as u64
        && (::core::mem::size_of::<*mut libc::c_void>() as u64 == 4 as i32 as u64
            || ::core::mem::size_of::<*mut libc::c_void>() as u64 == 8 as i32 as u64))
        as i32;
    if ok == 0 {
        return 3 as i32;
    }
    if !(_glp_tls_get_ptr()).is_null() {
        return 1 as i32;
    }
    env = malloc(::core::mem::size_of::<ENV>() as u64) as *mut ENV;
    if env.is_null() {
        return 2 as i32;
    }
    memset(env as *mut libc::c_void, 0 as i32, ::core::mem::size_of::<ENV>() as u64);
    (*env).self_0 = env;
    (*env).term_buf = malloc(4096 as i32 as u64) as *mut i8;
    if ((*env).term_buf).is_null() {
        free(env as *mut libc::c_void);
        return 2 as i32;
    }
    (*env).term_out = 1 as i32;
    (*env).term_hook = None;
    (*env).term_info = 0 as *mut libc::c_void;
    (*env).tee_file = 0 as *mut FILE;
    (*env).err_st = 0 as i32;
    (*env).err_file = 0 as *const i8;
    (*env).err_line = 0 as i32;
    (*env).err_hook = None;
    (*env).err_info = 0 as *mut libc::c_void;
    (*env).err_buf = malloc(1024 as i32 as u64) as *mut i8;
    if ((*env).err_buf).is_null() {
        free((*env).term_buf as *mut libc::c_void);
        free(env as *mut libc::c_void);
        return 2 as i32;
    }
    *((*env).err_buf).offset(0 as i32 as isize) = '\0' as i32 as i8;
    (*env).mem_limit = !(0 as i32 as size_t);
    (*env).mem_ptr = 0 as *mut MBD;
    (*env).mem_cpeak = 0 as i32;
    (*env).mem_count = (*env).mem_cpeak;
    (*env).mem_tpeak = 0 as i32 as size_t;
    (*env).mem_total = (*env).mem_tpeak;
    (*env).gmp_pool = 0 as *mut libc::c_void;
    (*env).gmp_size = 0 as i32;
    (*env).gmp_work = 0 as *mut libc::c_ushort;
    (*env).h_mysql = 0 as *mut libc::c_void;
    (*env).h_odbc = (*env).h_mysql;
    _glp_tls_set_ptr(env as *mut libc::c_void);
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_get_env_ptr() -> *mut ENV {
    let mut env: *mut ENV = _glp_tls_get_ptr() as *mut ENV;
    if env.is_null() {
        if glp_init_env() != 0 as i32 {
            fprintf(stderr, b"GLPK initialization failed\n\0" as *const u8 as *const i8);
            fflush(stderr);
            abort();
        }
        env = _glp_tls_get_ptr() as *mut ENV;
    }
    if (*env).self_0 != env {
        fprintf(stderr, b"Invalid GLPK environment\n\0" as *const u8 as *const i8);
        fflush(stderr);
        abort();
    }
    return env;
}
#[no_mangle]
pub unsafe extern "C" fn glp_version() -> *const i8 {
    return b"5.0\0" as *const u8 as *const i8;
}
#[no_mangle]
pub unsafe extern "C" fn glp_config(mut option: *const i8) -> *const i8 {
    let mut s: *const i8 = 0 as *const i8;
    if strcmp(option, b"TLS\0" as *const u8 as *const i8) == 0 as i32 {
        s = b"_Thread_local\0" as *const u8 as *const i8;
    } else if strcmp(option, b"ODBC_DLNAME\0" as *const u8 as *const i8) == 0 as i32 {
        s = 0 as *const i8;
    } else if strcmp(option, b"MYSQL_DLNAME\0" as *const u8 as *const i8) == 0 as i32 {
        s = 0 as *const i8;
    } else {
        s = 0 as *const i8;
    }
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn glp_free_env() -> i32 {
    let mut env: *mut ENV = _glp_tls_get_ptr() as *mut ENV;
    let mut desc: *mut MBD = 0 as *mut MBD;
    if env.is_null() {
        return 1 as i32;
    }
    if (*env).self_0 != env {
        fprintf(stderr, b"Invalid GLPK environment\n\0" as *const u8 as *const i8);
        fflush(stderr);
        abort();
    }
    if !((*env).h_odbc).is_null() {
        _glp_dlclose((*env).h_odbc);
    }
    if !((*env).h_mysql).is_null() {
        _glp_dlclose((*env).h_mysql);
    }
    while !((*env).mem_ptr).is_null() {
        desc = (*env).mem_ptr;
        (*env).mem_ptr = (*desc).next;
        free(desc as *mut libc::c_void);
    }
    if !((*env).tee_file).is_null() {
        fclose((*env).tee_file);
    }
    (*env).self_0 = 0 as *mut ENV;
    free((*env).term_buf as *mut libc::c_void);
    free((*env).err_buf as *mut libc::c_void);
    free(env as *mut libc::c_void);
    _glp_tls_set_ptr(0 as *mut libc::c_void);
    return 0 as i32;
}