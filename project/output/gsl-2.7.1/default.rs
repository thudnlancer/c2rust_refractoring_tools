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
    fn strtoul(__nptr: *const i8, __endptr: *mut *mut i8, __base: i32) -> u64;
    fn getenv(__name: *const i8) -> *mut i8;
    static mut stderr: *mut _IO_FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn fputc(__c: i32, __stream: *mut FILE) -> i32;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    static mut gsl_rng_mt19937: *const gsl_rng_type;
    fn gsl_rng_types_setup() -> *mut *const gsl_rng_type;
    static mut gsl_rng_default: *const gsl_rng_type;
    static mut gsl_rng_default_seed: u64;
    fn gsl_error(reason: *const i8, file: *const i8, line: i32, gsl_errno: i32);
}
pub type size_t = u64;
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
pub type C2RustUnnamed = i32;
pub const GSL_EOF: C2RustUnnamed = 32;
pub const GSL_ETOLG: C2RustUnnamed = 31;
pub const GSL_ETOLX: C2RustUnnamed = 30;
pub const GSL_ETOLF: C2RustUnnamed = 29;
pub const GSL_ENOPROGJ: C2RustUnnamed = 28;
pub const GSL_ENOPROG: C2RustUnnamed = 27;
pub const GSL_ETABLE: C2RustUnnamed = 26;
pub const GSL_ECACHE: C2RustUnnamed = 25;
pub const GSL_EUNIMPL: C2RustUnnamed = 24;
pub const GSL_EUNSUP: C2RustUnnamed = 23;
pub const GSL_EDIVERGE: C2RustUnnamed = 22;
pub const GSL_ESING: C2RustUnnamed = 21;
pub const GSL_ENOTSQR: C2RustUnnamed = 20;
pub const GSL_EBADLEN: C2RustUnnamed = 19;
pub const GSL_EROUND: C2RustUnnamed = 18;
pub const GSL_ELOSS: C2RustUnnamed = 17;
pub const GSL_EOVRFLW: C2RustUnnamed = 16;
pub const GSL_EUNDRFLW: C2RustUnnamed = 15;
pub const GSL_ETOL: C2RustUnnamed = 14;
pub const GSL_EBADTOL: C2RustUnnamed = 13;
pub const GSL_EZERODIV: C2RustUnnamed = 12;
pub const GSL_EMAXITER: C2RustUnnamed = 11;
pub const GSL_ERUNAWAY: C2RustUnnamed = 10;
pub const GSL_EBADFUNC: C2RustUnnamed = 9;
pub const GSL_ENOMEM: C2RustUnnamed = 8;
pub const GSL_ESANITY: C2RustUnnamed = 7;
pub const GSL_EFACTOR: C2RustUnnamed = 6;
pub const GSL_EFAILED: C2RustUnnamed = 5;
pub const GSL_EINVAL: C2RustUnnamed = 4;
pub const GSL_EFAULT: C2RustUnnamed = 3;
pub const GSL_ERANGE: C2RustUnnamed = 2;
pub const GSL_EDOM: C2RustUnnamed = 1;
pub const GSL_CONTINUE: C2RustUnnamed = -2;
pub const GSL_FAILURE: C2RustUnnamed = -1;
pub const GSL_SUCCESS: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_rng_type {
    pub name: *const i8,
    pub max: u64,
    pub min: u64,
    pub size: size_t,
    pub set: Option<unsafe extern "C" fn(*mut libc::c_void, u64) -> ()>,
    pub get: Option<unsafe extern "C" fn(*mut libc::c_void) -> u64>,
    pub get_double: Option<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_double>,
}
#[no_mangle]
pub unsafe extern "C" fn gsl_rng_env_setup() -> *const gsl_rng_type {
    let mut seed: u64 = 0 as i32 as u64;
    let mut p: *const i8 = getenv(b"GSL_RNG_TYPE\0" as *const u8 as *const i8);
    if !p.is_null() {
        let mut t: *mut *const gsl_rng_type = 0 as *mut *const gsl_rng_type;
        let mut t0: *mut *const gsl_rng_type = gsl_rng_types_setup();
        gsl_rng_default = 0 as *const gsl_rng_type;
        t = t0;
        while !(*t).is_null() {
            if strcmp(p, (**t).name) == 0 as i32 {
                gsl_rng_default = *t;
                break;
            } else {
                t = t.offset(1);
                t;
            }
        }
        if gsl_rng_default.is_null() {
            let mut i: i32 = 0 as i32;
            fprintf(
                stderr,
                b"GSL_RNG_TYPE=%s not recognized\n\0" as *const u8 as *const i8,
                p,
            );
            fprintf(stderr, b"Valid generator types are:\n\0" as *const u8 as *const i8);
            t = t0;
            while !(*t).is_null() {
                fprintf(stderr, b" %18s\0" as *const u8 as *const i8, (**t).name);
                i += 1;
                if i % 4 as i32 == 0 as i32 {
                    fputc('\n' as i32, stderr);
                }
                t = t.offset(1);
                t;
            }
            fputc('\n' as i32, stderr);
            gsl_error(
                b"unknown generator\0" as *const u8 as *const i8,
                b"default.c\0" as *const u8 as *const i8,
                72 as i32,
                GSL_EINVAL as i32,
            );
            return 0 as *const gsl_rng_type;
        }
        fprintf(
            stderr,
            b"GSL_RNG_TYPE=%s\n\0" as *const u8 as *const i8,
            (*gsl_rng_default).name,
        );
    } else {
        gsl_rng_default = gsl_rng_mt19937;
    }
    p = getenv(b"GSL_RNG_SEED\0" as *const u8 as *const i8);
    if !p.is_null() {
        seed = strtoul(p, 0 as *mut *mut i8, 0 as i32);
        fprintf(stderr, b"GSL_RNG_SEED=%lu\n\0" as *const u8 as *const i8, seed);
    }
    gsl_rng_default_seed = seed;
    return gsl_rng_default;
}