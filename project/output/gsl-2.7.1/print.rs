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
    static mut stdout: *mut _IO_FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn gsl_ieee_float_to_rep(x: *const libc::c_float, r: *mut gsl_ieee_float_rep);
    fn gsl_ieee_double_to_rep(x: *const libc::c_double, r: *mut gsl_ieee_double_rep);
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
pub type C2RustUnnamed = u32;
pub const GSL_IEEE_TYPE_ZERO: C2RustUnnamed = 5;
pub const GSL_IEEE_TYPE_DENORMAL: C2RustUnnamed = 4;
pub const GSL_IEEE_TYPE_NORMAL: C2RustUnnamed = 3;
pub const GSL_IEEE_TYPE_INF: C2RustUnnamed = 2;
pub const GSL_IEEE_TYPE_NAN: C2RustUnnamed = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_ieee_float_rep {
    pub sign: i32,
    pub mantissa: [i8; 24],
    pub exponent: i32,
    pub type_0: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_ieee_double_rep {
    pub sign: i32,
    pub mantissa: [i8; 53],
    pub exponent: i32,
    pub type_0: i32,
}
static mut signs: [i8; 2] = [' ' as i32 as i8, '-' as i32 as i8];
#[no_mangle]
pub unsafe extern "C" fn gsl_ieee_fprintf_float(
    mut stream: *mut FILE,
    mut x: *const libc::c_float,
) {
    let mut r: gsl_ieee_float_rep = gsl_ieee_float_rep {
        sign: 0,
        mantissa: [0; 24],
        exponent: 0,
        type_0: 0,
    };
    gsl_ieee_float_to_rep(x, &mut r);
    match r.type_0 {
        1 => {
            fprintf(stream, b"NaN\0" as *const u8 as *const i8);
        }
        2 => {
            fprintf(
                stream,
                b"%cInf\0" as *const u8 as *const i8,
                signs[r.sign as usize] as i32,
            );
        }
        3 => {
            fprintf(
                stream,
                b"%c1.%s*2^%d\0" as *const u8 as *const i8,
                signs[r.sign as usize] as i32,
                (r.mantissa).as_mut_ptr(),
                r.exponent,
            );
        }
        4 => {
            fprintf(
                stream,
                b"%c0.%s*2^%d\0" as *const u8 as *const i8,
                signs[r.sign as usize] as i32,
                (r.mantissa).as_mut_ptr(),
                r.exponent + 1 as i32,
            );
        }
        5 => {
            fprintf(
                stream,
                b"%c0\0" as *const u8 as *const i8,
                signs[r.sign as usize] as i32,
            );
        }
        _ => {
            fprintf(stream, b"[non-standard IEEE float]\0" as *const u8 as *const i8);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn gsl_ieee_printf_float(mut x: *const libc::c_float) {
    gsl_ieee_fprintf_float(stdout, x);
}
#[no_mangle]
pub unsafe extern "C" fn gsl_ieee_fprintf_double(
    mut stream: *mut FILE,
    mut x: *const libc::c_double,
) {
    let mut r: gsl_ieee_double_rep = gsl_ieee_double_rep {
        sign: 0,
        mantissa: [0; 53],
        exponent: 0,
        type_0: 0,
    };
    gsl_ieee_double_to_rep(x, &mut r);
    match r.type_0 {
        1 => {
            fprintf(stream, b"NaN\0" as *const u8 as *const i8);
        }
        2 => {
            fprintf(
                stream,
                b"%cInf\0" as *const u8 as *const i8,
                signs[r.sign as usize] as i32,
            );
        }
        3 => {
            fprintf(
                stream,
                b"%c1.%s*2^%d\0" as *const u8 as *const i8,
                signs[r.sign as usize] as i32,
                (r.mantissa).as_mut_ptr(),
                r.exponent,
            );
        }
        4 => {
            fprintf(
                stream,
                b"%c0.%s*2^%d\0" as *const u8 as *const i8,
                signs[r.sign as usize] as i32,
                (r.mantissa).as_mut_ptr(),
                r.exponent + 1 as i32,
            );
        }
        5 => {
            fprintf(
                stream,
                b"%c0\0" as *const u8 as *const i8,
                signs[r.sign as usize] as i32,
            );
        }
        _ => {
            fprintf(stream, b"[non-standard IEEE double]\0" as *const u8 as *const i8);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn gsl_ieee_printf_double(mut x: *const libc::c_double) {
    gsl_ieee_fprintf_double(stdout, x);
}