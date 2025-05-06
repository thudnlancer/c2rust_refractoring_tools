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
    fn malloc(_: u64) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn gsl_error(reason: *const i8, file: *const i8, line: i32, gsl_errno: i32);
}
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
pub type C2RustUnnamed_0 = u32;
pub const GSL_IEEE_EXTENDED_PRECISION: C2RustUnnamed_0 = 3;
pub const GSL_IEEE_DOUBLE_PRECISION: C2RustUnnamed_0 = 2;
pub const GSL_IEEE_SINGLE_PRECISION: C2RustUnnamed_0 = 1;
pub type C2RustUnnamed_1 = u32;
pub const GSL_IEEE_ROUND_TO_ZERO: C2RustUnnamed_1 = 4;
pub const GSL_IEEE_ROUND_UP: C2RustUnnamed_1 = 3;
pub const GSL_IEEE_ROUND_DOWN: C2RustUnnamed_1 = 2;
pub const GSL_IEEE_ROUND_TO_NEAREST: C2RustUnnamed_1 = 1;
pub type C2RustUnnamed_2 = u32;
pub const GSL_IEEE_TRAP_INEXACT: C2RustUnnamed_2 = 32;
pub const GSL_IEEE_MASK_ALL: C2RustUnnamed_2 = 31;
pub const GSL_IEEE_MASK_UNDERFLOW: C2RustUnnamed_2 = 16;
pub const GSL_IEEE_MASK_OVERFLOW: C2RustUnnamed_2 = 8;
pub const GSL_IEEE_MASK_DIVISION_BY_ZERO: C2RustUnnamed_2 = 4;
pub const GSL_IEEE_MASK_DENORMALIZED: C2RustUnnamed_2 = 2;
pub const GSL_IEEE_MASK_INVALID: C2RustUnnamed_2 = 1;
#[no_mangle]
pub unsafe extern "C" fn gsl_ieee_read_mode_string(
    mut description: *const i8,
    mut precision: *mut i32,
    mut rounding: *mut i32,
    mut exception_mask: *mut i32,
) -> i32 {
    let mut start: *mut i8 = 0 as *mut i8;
    let mut end: *mut i8 = 0 as *mut i8;
    let mut p: *mut i8 = 0 as *mut i8;
    let mut precision_count: i32 = 0 as i32;
    let mut rounding_count: i32 = 0 as i32;
    let mut exception_count: i32 = 0 as i32;
    start = malloc((strlen(description)).wrapping_add(1 as i32 as u64)) as *mut i8;
    if start.is_null() {
        gsl_error(
            b"no memory to parse mode string\0" as *const u8 as *const i8,
            b"read.c\0" as *const u8 as *const i8,
            48 as i32,
            GSL_ENOMEM as i32,
        );
        return GSL_ENOMEM as i32;
    }
    strcpy(start, description);
    p = start;
    *precision = 0 as i32;
    *rounding = 0 as i32;
    *exception_mask = 0 as i32;
    loop {
        let mut status: i32 = 0;
        let mut new_precision: i32 = 0;
        let mut new_rounding: i32 = 0;
        let mut new_exception: i32 = 0;
        end = strchr(p, ',' as i32);
        if !end.is_null() {
            *end = '\0' as i32 as i8;
            loop {
                end = end.offset(1);
                end;
                if !(*end as i32 == ' ' as i32 || *end as i32 == ',' as i32) {
                    break;
                }
            }
        }
        new_precision = 0 as i32;
        new_rounding = 0 as i32;
        new_exception = 0 as i32;
        status = lookup_string(
            p,
            &mut new_precision,
            &mut new_rounding,
            &mut new_exception,
        );
        if status != 0 {
            free(start as *mut libc::c_void);
            gsl_error(
                b"unrecognized GSL_IEEE_MODE string.\nValid settings are:\n\n  single-precision double-precision extended-precision\n  round-to-nearest round-down round-up round-to-zero\n  mask-invalid mask-denormalized mask-division-by-zero\n  mask-overflow mask-underflow mask-all\n  trap-common trap-inexact\n\nseparated by commas. (e.g. GSL_IEEE_MODE=\"round-down,mask-underflow\")\0"
                    as *const u8 as *const i8,
                b"read.c\0" as *const u8 as *const i8,
                93 as i32,
                GSL_EINVAL as i32,
            );
            return GSL_EINVAL as i32;
        }
        if new_precision != 0 {
            *precision = new_precision;
            precision_count += 1;
            precision_count;
            if precision_count > 1 as i32 {
                free(start as *mut libc::c_void);
                gsl_error(
                    b"attempted to set IEEE precision twice\0" as *const u8 as *const i8,
                    b"read.c\0" as *const u8 as *const i8,
                    103 as i32,
                    GSL_EINVAL as i32,
                );
                return GSL_EINVAL as i32;
            }
        }
        if new_rounding != 0 {
            *rounding = new_rounding;
            rounding_count += 1;
            rounding_count;
            if rounding_count > 1 as i32 {
                free(start as *mut libc::c_void);
                gsl_error(
                    b"attempted to set IEEE rounding mode twice\0" as *const u8
                        as *const i8,
                    b"read.c\0" as *const u8 as *const i8,
                    114 as i32,
                    GSL_EINVAL as i32,
                );
                return GSL_EINVAL as i32;
            }
        }
        if new_exception != 0 {
            *exception_mask |= new_exception;
            exception_count += 1;
            exception_count;
        }
        p = end;
        if !(!end.is_null() && *p as i32 != '\0' as i32) {
            break;
        }
    }
    free(start as *mut libc::c_void);
    return GSL_SUCCESS as i32;
}
unsafe extern "C" fn lookup_string(
    mut p: *const i8,
    mut precision: *mut i32,
    mut rounding: *mut i32,
    mut exception_mask: *mut i32,
) -> i32 {
    if strcmp(p, b"single-precision\0" as *const u8 as *const i8) == 0 as i32 {
        *precision = GSL_IEEE_SINGLE_PRECISION as i32;
    } else if strcmp(p, b"double-precision\0" as *const u8 as *const i8) == 0 as i32 {
        *precision = GSL_IEEE_DOUBLE_PRECISION as i32;
    } else if strcmp(p, b"extended-precision\0" as *const u8 as *const i8) == 0 as i32 {
        *precision = GSL_IEEE_EXTENDED_PRECISION as i32;
    } else if strcmp(p, b"round-to-nearest\0" as *const u8 as *const i8) == 0 as i32 {
        *rounding = GSL_IEEE_ROUND_TO_NEAREST as i32;
    } else if strcmp(p, b"round-down\0" as *const u8 as *const i8) == 0 as i32 {
        *rounding = GSL_IEEE_ROUND_DOWN as i32;
    } else if strcmp(p, b"round-up\0" as *const u8 as *const i8) == 0 as i32 {
        *rounding = GSL_IEEE_ROUND_UP as i32;
    } else if strcmp(p, b"round-to-zero\0" as *const u8 as *const i8) == 0 as i32 {
        *rounding = GSL_IEEE_ROUND_TO_ZERO as i32;
    } else if strcmp(p, b"mask-all\0" as *const u8 as *const i8) == 0 as i32 {
        *exception_mask = GSL_IEEE_MASK_ALL as i32;
    } else if strcmp(p, b"mask-invalid\0" as *const u8 as *const i8) == 0 as i32 {
        *exception_mask = GSL_IEEE_MASK_INVALID as i32;
    } else if strcmp(p, b"mask-denormalized\0" as *const u8 as *const i8) == 0 as i32 {
        *exception_mask = GSL_IEEE_MASK_DENORMALIZED as i32;
    } else if strcmp(p, b"mask-division-by-zero\0" as *const u8 as *const i8) == 0 as i32
    {
        *exception_mask = GSL_IEEE_MASK_DIVISION_BY_ZERO as i32;
    } else if strcmp(p, b"mask-overflow\0" as *const u8 as *const i8) == 0 as i32 {
        *exception_mask = GSL_IEEE_MASK_OVERFLOW as i32;
    } else if strcmp(p, b"mask-underflow\0" as *const u8 as *const i8) == 0 as i32 {
        *exception_mask = GSL_IEEE_MASK_UNDERFLOW as i32;
    } else if strcmp(p, b"trap-inexact\0" as *const u8 as *const i8) == 0 as i32 {
        *exception_mask = GSL_IEEE_TRAP_INEXACT as i32;
    } else if strcmp(p, b"trap-common\0" as *const u8 as *const i8) == 0 as i32 {
        return 0 as i32
    } else {
        return 1 as i32
    }
    return 0 as i32;
}