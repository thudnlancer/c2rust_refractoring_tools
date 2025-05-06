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
    fn getenv(__name: *const i8) -> *mut i8;
    static mut stderr: *mut _IO_FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn gsl_ieee_read_mode_string(
        description: *const i8,
        precision: *mut i32,
        rounding: *mut i32,
        exception_mask: *mut i32,
    ) -> i32;
    fn gsl_ieee_set_mode(precision: i32, rounding: i32, exception_mask: i32) -> i32;
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
pub const GSL_IEEE_EXTENDED_PRECISION: C2RustUnnamed = 3;
pub const GSL_IEEE_DOUBLE_PRECISION: C2RustUnnamed = 2;
pub const GSL_IEEE_SINGLE_PRECISION: C2RustUnnamed = 1;
pub type C2RustUnnamed_0 = u32;
pub const GSL_IEEE_ROUND_TO_ZERO: C2RustUnnamed_0 = 4;
pub const GSL_IEEE_ROUND_UP: C2RustUnnamed_0 = 3;
pub const GSL_IEEE_ROUND_DOWN: C2RustUnnamed_0 = 2;
pub const GSL_IEEE_ROUND_TO_NEAREST: C2RustUnnamed_0 = 1;
pub type C2RustUnnamed_1 = u32;
pub const GSL_IEEE_TRAP_INEXACT: C2RustUnnamed_1 = 32;
pub const GSL_IEEE_MASK_ALL: C2RustUnnamed_1 = 31;
pub const GSL_IEEE_MASK_UNDERFLOW: C2RustUnnamed_1 = 16;
pub const GSL_IEEE_MASK_OVERFLOW: C2RustUnnamed_1 = 8;
pub const GSL_IEEE_MASK_DIVISION_BY_ZERO: C2RustUnnamed_1 = 4;
pub const GSL_IEEE_MASK_DENORMALIZED: C2RustUnnamed_1 = 2;
pub const GSL_IEEE_MASK_INVALID: C2RustUnnamed_1 = 1;
#[no_mangle]
pub unsafe extern "C" fn gsl_ieee_env_setup() {
    let mut p: *const i8 = getenv(b"GSL_IEEE_MODE\0" as *const u8 as *const i8);
    let mut precision: i32 = 0 as i32;
    let mut rounding: i32 = 0 as i32;
    let mut exception_mask: i32 = 0 as i32;
    let mut comma: i32 = 0 as i32;
    if p.is_null() {
        return;
    }
    if *p as i32 == '\0' as i32 {
        return;
    }
    gsl_ieee_read_mode_string(p, &mut precision, &mut rounding, &mut exception_mask);
    gsl_ieee_set_mode(precision, rounding, exception_mask);
    fprintf(stderr, b"GSL_IEEE_MODE=\"\0" as *const u8 as *const i8);
    match precision {
        1 => {
            if comma != 0 {
                fprintf(stderr, b",\0" as *const u8 as *const i8);
            }
            fprintf(stderr, b"single-precision\0" as *const u8 as *const i8);
            comma += 1;
            comma;
        }
        2 => {
            if comma != 0 {
                fprintf(stderr, b",\0" as *const u8 as *const i8);
            }
            fprintf(stderr, b"double-precision\0" as *const u8 as *const i8);
            comma += 1;
            comma;
        }
        3 => {
            if comma != 0 {
                fprintf(stderr, b",\0" as *const u8 as *const i8);
            }
            fprintf(stderr, b"extended-precision\0" as *const u8 as *const i8);
            comma += 1;
            comma;
        }
        _ => {}
    }
    match rounding {
        1 => {
            if comma != 0 {
                fprintf(stderr, b",\0" as *const u8 as *const i8);
            }
            fprintf(stderr, b"round-to-nearest\0" as *const u8 as *const i8);
            comma += 1;
            comma;
        }
        2 => {
            if comma != 0 {
                fprintf(stderr, b",\0" as *const u8 as *const i8);
            }
            fprintf(stderr, b"round-down\0" as *const u8 as *const i8);
            comma += 1;
            comma;
        }
        3 => {
            if comma != 0 {
                fprintf(stderr, b",\0" as *const u8 as *const i8);
            }
            fprintf(stderr, b"round-up\0" as *const u8 as *const i8);
            comma += 1;
            comma;
        }
        4 => {
            if comma != 0 {
                fprintf(stderr, b",\0" as *const u8 as *const i8);
            }
            fprintf(stderr, b"round-to-zero\0" as *const u8 as *const i8);
            comma += 1;
            comma;
        }
        _ => {}
    }
    if exception_mask & GSL_IEEE_MASK_ALL as i32 == GSL_IEEE_MASK_ALL as i32 {
        if comma != 0 {
            fprintf(stderr, b",\0" as *const u8 as *const i8);
        }
        fprintf(stderr, b"mask-all\0" as *const u8 as *const i8);
        comma += 1;
        comma;
    } else if exception_mask & GSL_IEEE_MASK_ALL as i32 == 0 as i32 {
        if comma != 0 {
            fprintf(stderr, b",\0" as *const u8 as *const i8);
        }
        fprintf(stderr, b"trap-common\0" as *const u8 as *const i8);
        comma += 1;
        comma;
    } else {
        if exception_mask & GSL_IEEE_MASK_INVALID as i32 != 0 {
            if comma != 0 {
                fprintf(stderr, b",\0" as *const u8 as *const i8);
            }
            fprintf(stderr, b"mask-invalid\0" as *const u8 as *const i8);
            comma += 1;
            comma;
        }
        if exception_mask & GSL_IEEE_MASK_DENORMALIZED as i32 != 0 {
            if comma != 0 {
                fprintf(stderr, b",\0" as *const u8 as *const i8);
            }
            fprintf(stderr, b"mask-denormalized\0" as *const u8 as *const i8);
            comma += 1;
            comma;
        }
        if exception_mask & GSL_IEEE_MASK_DIVISION_BY_ZERO as i32 != 0 {
            if comma != 0 {
                fprintf(stderr, b",\0" as *const u8 as *const i8);
            }
            fprintf(stderr, b"mask-division-by-zero\0" as *const u8 as *const i8);
            comma += 1;
            comma;
        }
        if exception_mask & GSL_IEEE_MASK_OVERFLOW as i32 != 0 {
            if comma != 0 {
                fprintf(stderr, b",\0" as *const u8 as *const i8);
            }
            fprintf(stderr, b"mask-overflow\0" as *const u8 as *const i8);
            comma += 1;
            comma;
        }
        if exception_mask & GSL_IEEE_MASK_UNDERFLOW as i32 != 0 {
            if comma != 0 {
                fprintf(stderr, b",\0" as *const u8 as *const i8);
            }
            fprintf(stderr, b"mask-underflow\0" as *const u8 as *const i8);
            comma += 1;
            comma;
        }
    }
    if exception_mask & GSL_IEEE_TRAP_INEXACT as i32 != 0 {
        if comma != 0 {
            fprintf(stderr, b",\0" as *const u8 as *const i8);
        }
        fprintf(stderr, b"trap-inexact\0" as *const u8 as *const i8);
        comma += 1;
        comma;
    }
    fprintf(stderr, b"\"\n\0" as *const u8 as *const i8);
}