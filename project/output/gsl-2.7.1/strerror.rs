#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
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
#[no_mangle]
pub unsafe extern "C" fn gsl_strerror(gsl_errno: i32) -> *const i8 {
    match gsl_errno {
        0 => return b"success\0" as *const u8 as *const i8,
        -1 => return b"failure\0" as *const u8 as *const i8,
        -2 => {
            return b"the iteration has not converged yet\0" as *const u8 as *const i8;
        }
        1 => return b"input domain error\0" as *const u8 as *const i8,
        2 => return b"output range error\0" as *const u8 as *const i8,
        3 => return b"invalid pointer\0" as *const u8 as *const i8,
        4 => {
            return b"invalid argument supplied by user\0" as *const u8 as *const i8;
        }
        5 => return b"generic failure\0" as *const u8 as *const i8,
        6 => return b"factorization failed\0" as *const u8 as *const i8,
        7 => {
            return b"sanity check failed - shouldn't happen\0" as *const u8 as *const i8;
        }
        8 => return b"malloc failed\0" as *const u8 as *const i8,
        9 => {
            return b"problem with user-supplied function\0" as *const u8 as *const i8;
        }
        10 => {
            return b"iterative process is out of control\0" as *const u8 as *const i8;
        }
        11 => {
            return b"exceeded max number of iterations\0" as *const u8 as *const i8;
        }
        12 => return b"tried to divide by zero\0" as *const u8 as *const i8,
        13 => {
            return b"specified tolerance is invalid or theoretically unattainable\0"
                as *const u8 as *const i8;
        }
        14 => {
            return b"failed to reach the specified tolerance\0" as *const u8
                as *const i8;
        }
        15 => return b"underflow\0" as *const u8 as *const i8,
        16 => return b"overflow\0" as *const u8 as *const i8,
        17 => return b"loss of accuracy\0" as *const u8 as *const i8,
        18 => return b"roundoff error\0" as *const u8 as *const i8,
        19 => {
            return b"matrix/vector sizes are not conformant\0" as *const u8 as *const i8;
        }
        20 => return b"matrix not square\0" as *const u8 as *const i8,
        21 => {
            return b"singularity or extremely bad function behavior detected\0"
                as *const u8 as *const i8;
        }
        22 => {
            return b"integral or series is divergent\0" as *const u8 as *const i8;
        }
        23 => {
            return b"the required feature is not supported by this hardware platform\0"
                as *const u8 as *const i8;
        }
        24 => {
            return b"the requested feature is not (yet) implemented\0" as *const u8
                as *const i8;
        }
        25 => return b"cache limit exceeded\0" as *const u8 as *const i8,
        26 => return b"table limit exceeded\0" as *const u8 as *const i8,
        27 => {
            return b"iteration is not making progress towards solution\0" as *const u8
                as *const i8;
        }
        28 => {
            return b"jacobian evaluations are not improving the solution\0" as *const u8
                as *const i8;
        }
        29 => {
            return b"cannot reach the specified tolerance in F\0" as *const u8
                as *const i8;
        }
        30 => {
            return b"cannot reach the specified tolerance in X\0" as *const u8
                as *const i8;
        }
        31 => {
            return b"cannot reach the specified tolerance in gradient\0" as *const u8
                as *const i8;
        }
        32 => return b"end of file\0" as *const u8 as *const i8,
        _ => return b"unknown error code\0" as *const u8 as *const i8,
    };
}