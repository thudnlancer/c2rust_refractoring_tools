#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(c_variadic, label_break_value)]
extern "C" {
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
    static mut stdout: *mut _IO_FILE;
    fn fclose(__stream: *mut FILE) -> i32;
    fn fflush(__stream: *mut FILE) -> i32;
    fn fopen(__filename: *const i8, __modes: *const i8) -> *mut FILE;
    fn vsprintf(_: *mut i8, _: *const i8, _: ::core::ffi::VaList) -> i32;
    fn fputs(__s: *const i8, __stream: *mut FILE) -> i32;
    fn glp_error_(file: *const i8, line: i32) -> glp_errfunc;
    fn _glp_get_env_ptr() -> *mut ENV;
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
#[no_mangle]
pub unsafe extern "C" fn glp_puts(mut s: *const i8) {
    let mut current_block: u64;
    let mut env: *mut ENV = _glp_get_env_ptr();
    if !((*env).term_out == 0) {
        if ((*env).term_hook).is_some() {
            if ((*env).term_hook)
                .expect("non-null function pointer")((*env).term_info, s) != 0 as i32
            {
                current_block = 10029913920574171561;
            } else {
                current_block = 10680521327981672866;
            }
        } else {
            current_block = 10680521327981672866;
        }
        match current_block {
            10029913920574171561 => {}
            _ => {
                fputs(s, stdout);
                fflush(stdout);
                if !((*env).tee_file).is_null() {
                    fputs(s, (*env).tee_file);
                    fflush((*env).tee_file);
                }
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn glp_printf(mut fmt: *const i8, mut args: ...) {
    let mut env: *mut ENV = _glp_get_env_ptr();
    let mut arg: ::core::ffi::VaListImpl;
    if !((*env).term_out == 0) {
        arg = args.clone();
        vsprintf((*env).term_buf, fmt, arg.as_va_list());
        if strlen((*env).term_buf) < 4096 as i32 as u64 {} else {
            __assert_fail(
                b"strlen(env->term_buf) < TBUF_SIZE\0" as *const u8 as *const i8,
                b"env/stdout.c\0" as *const u8 as *const i8,
                82 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 35],
                    &[i8; 35],
                >(b"void glp_printf(const char *, ...)\0"))
                    .as_ptr(),
            );
        }
        'c_4029: {
            if strlen((*env).term_buf) < 4096 as i32 as u64 {} else {
                __assert_fail(
                    b"strlen(env->term_buf) < TBUF_SIZE\0" as *const u8 as *const i8,
                    b"env/stdout.c\0" as *const u8 as *const i8,
                    82 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 35],
                        &[i8; 35],
                    >(b"void glp_printf(const char *, ...)\0"))
                        .as_ptr(),
                );
            }
        };
        glp_puts((*env).term_buf);
    }
}
#[no_mangle]
pub unsafe extern "C" fn glp_vprintf(mut fmt: *const i8, mut arg: ::core::ffi::VaList) {
    let mut env: *mut ENV = _glp_get_env_ptr();
    if !((*env).term_out == 0) {
        vsprintf((*env).term_buf, fmt, arg.as_va_list());
        if strlen((*env).term_buf) < 4096 as i32 as u64 {} else {
            __assert_fail(
                b"strlen(env->term_buf) < TBUF_SIZE\0" as *const u8 as *const i8,
                b"env/stdout.c\0" as *const u8 as *const i8,
                112 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 55],
                    &[i8; 55],
                >(b"void glp_vprintf(const char *, struct __va_list_tag *)\0"))
                    .as_ptr(),
            );
        }
        'c_4138: {
            if strlen((*env).term_buf) < 4096 as i32 as u64 {} else {
                __assert_fail(
                    b"strlen(env->term_buf) < TBUF_SIZE\0" as *const u8 as *const i8,
                    b"env/stdout.c\0" as *const u8 as *const i8,
                    112 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 55],
                        &[i8; 55],
                    >(b"void glp_vprintf(const char *, struct __va_list_tag *)\0"))
                        .as_ptr(),
                );
            }
        };
        glp_puts((*env).term_buf);
    }
}
#[no_mangle]
pub unsafe extern "C" fn glp_term_out(mut flag: i32) -> i32 {
    let mut env: *mut ENV = _glp_get_env_ptr();
    let mut old: i32 = (*env).term_out;
    if !(flag == 1 as i32 || flag == 0 as i32) {
        (glp_error_(b"env/stdout.c\0" as *const u8 as *const i8, 144 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_term_out: flag = %d; invalid parameter\n\0" as *const u8 as *const i8,
            flag,
        );
    }
    (*env).term_out = flag;
    return old;
}
#[no_mangle]
pub unsafe extern "C" fn glp_term_hook(
    mut func: Option<unsafe extern "C" fn(*mut libc::c_void, *const i8) -> i32>,
    mut info: *mut libc::c_void,
) {
    let mut env: *mut ENV = _glp_get_env_ptr();
    if func.is_none() {
        (*env).term_hook = None;
        (*env).term_info = 0 as *mut libc::c_void;
    } else {
        (*env).term_hook = func;
        (*env).term_info = info;
    };
}
#[no_mangle]
pub unsafe extern "C" fn glp_open_tee(mut name: *const i8) -> i32 {
    let mut env: *mut ENV = _glp_get_env_ptr();
    if !((*env).tee_file).is_null() {
        return 1 as i32;
    }
    (*env).tee_file = fopen(name, b"w\0" as *const u8 as *const i8);
    if ((*env).tee_file).is_null() {
        return 2 as i32;
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn glp_close_tee() -> i32 {
    let mut env: *mut ENV = _glp_get_env_ptr();
    if ((*env).tee_file).is_null() {
        return 1 as i32;
    }
    fclose((*env).tee_file);
    (*env).tee_file = 0 as *mut FILE;
    return 0 as i32;
}