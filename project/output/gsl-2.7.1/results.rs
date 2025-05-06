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
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strlen(_: *const i8) -> u64;
    static mut stdout: *mut _IO_FILE;
    fn fflush(__stream: *mut FILE) -> i32;
    fn printf(_: *const i8, _: ...) -> i32;
    fn vfprintf(_: *mut FILE, _: *const i8, _: ::core::ffi::VaList) -> i32;
    fn strtoul(__nptr: *const i8, __endptr: *mut *mut i8, __base: i32) -> u64;
    fn getenv(__name: *const i8) -> *mut i8;
    fn gsl_isnan(x: libc::c_double) -> i32;
    fn gsl_isinf(x: libc::c_double) -> i32;
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
pub type va_list = __builtin_va_list;
#[inline]
unsafe extern "C" fn vprintf(
    mut __fmt: *const i8,
    mut __arg: ::core::ffi::VaList,
) -> i32 {
    return vfprintf(stdout, __fmt, __arg.as_va_list());
}
static mut tests: u32 = 0 as i32 as u32;
static mut passed: u32 = 0 as i32 as u32;
static mut failed: u32 = 0 as i32 as u32;
static mut verbose: u32 = 0 as i32 as u32;
unsafe extern "C" fn initialise() {
    let mut p: *const i8 = getenv(b"GSL_TEST_VERBOSE\0" as *const u8 as *const i8);
    if p.is_null() {
        return;
    }
    if *p as i32 == '\0' as i32 {
        return;
    }
    verbose = strtoul(p, 0 as *mut *mut i8, 0 as i32) as u32;
}
unsafe extern "C" fn update(mut s: i32) {
    tests = tests.wrapping_add(1);
    tests;
    if s == 0 as i32 {
        passed = passed.wrapping_add(1);
        passed;
    } else {
        failed = failed.wrapping_add(1);
        failed;
    };
}
#[no_mangle]
pub unsafe extern "C" fn gsl_test(
    mut status: i32,
    mut test_description: *const i8,
    mut args: ...
) {
    if tests == 0 {
        initialise();
    }
    update(status);
    if status != 0 || verbose != 0 {
        printf(
            if status != 0 {
                b"FAIL: \0" as *const u8 as *const i8
            } else {
                b"PASS: \0" as *const u8 as *const i8
            },
        );
        let mut ap: ::core::ffi::VaListImpl;
        ap = args.clone();
        vprintf(test_description, ap.as_va_list());
        if status != 0 && verbose == 0 {
            printf(b" [%u]\0" as *const u8 as *const i8, tests);
        }
        printf(b"\n\0" as *const u8 as *const i8);
        fflush(stdout);
    }
}
#[no_mangle]
pub unsafe extern "C" fn gsl_test_rel(
    mut result: libc::c_double,
    mut expected: libc::c_double,
    mut relative_error: libc::c_double,
    mut test_description: *const i8,
    mut args: ...
) {
    let mut status: i32 = 0;
    if tests == 0 {
        initialise();
    }
    if gsl_isnan(result) != 0 || gsl_isnan(expected) != 0 {
        status = (gsl_isnan(result) != gsl_isnan(expected)) as i32;
    } else if gsl_isinf(result) != 0 || gsl_isinf(expected) != 0 {
        status = (gsl_isinf(result) != gsl_isinf(expected)) as i32;
    } else if expected > 0 as i32 as libc::c_double
        && expected < 2.2250738585072014e-308f64
        || expected < 0 as i32 as libc::c_double
            && expected > -2.2250738585072014e-308f64
    {
        status = -(1 as i32);
    } else if expected != 0 as i32 as libc::c_double {
        status = (fabs(result - expected) / fabs(expected) > relative_error) as i32;
    } else {
        status = (fabs(result) > relative_error) as i32;
    }
    update(status);
    if status != 0 || verbose != 0 {
        printf(
            if status != 0 {
                b"FAIL: \0" as *const u8 as *const i8
            } else {
                b"PASS: \0" as *const u8 as *const i8
            },
        );
        let mut ap: ::core::ffi::VaListImpl;
        ap = args.clone();
        vprintf(test_description, ap.as_va_list());
        if status == 0 as i32 {
            if strlen(test_description) < 45 as i32 as u64 {
                printf(
                    b" (%g observed vs %g expected)\0" as *const u8 as *const i8,
                    result,
                    expected,
                );
            } else {
                printf(
                    b" (%g obs vs %g exp)\0" as *const u8 as *const i8,
                    result,
                    expected,
                );
            }
        } else {
            printf(
                b" (%.18g observed vs %.18g expected)\0" as *const u8 as *const i8,
                result,
                expected,
            );
        }
        if status == -(1 as i32) {
            printf(b" [test uses subnormal value]\0" as *const u8 as *const i8);
        }
        if status != 0 && verbose == 0 {
            printf(b" [%u]\0" as *const u8 as *const i8, tests);
        }
        printf(b"\n\0" as *const u8 as *const i8);
        fflush(stdout);
    }
}
#[no_mangle]
pub unsafe extern "C" fn gsl_test_abs(
    mut result: libc::c_double,
    mut expected: libc::c_double,
    mut absolute_error: libc::c_double,
    mut test_description: *const i8,
    mut args: ...
) {
    let mut status: i32 = 0;
    if tests == 0 {
        initialise();
    }
    if gsl_isnan(result) != 0 || gsl_isnan(expected) != 0 {
        status = (gsl_isnan(result) != gsl_isnan(expected)) as i32;
    } else if gsl_isinf(result) != 0 || gsl_isinf(expected) != 0 {
        status = (gsl_isinf(result) != gsl_isinf(expected)) as i32;
    } else if expected > 0 as i32 as libc::c_double
        && expected < 2.2250738585072014e-308f64
        || expected < 0 as i32 as libc::c_double
            && expected > -2.2250738585072014e-308f64
    {
        status = -(1 as i32);
    } else {
        status = (fabs(result - expected) > absolute_error) as i32;
    }
    update(status);
    if status != 0 || verbose != 0 {
        printf(
            if status != 0 {
                b"FAIL: \0" as *const u8 as *const i8
            } else {
                b"PASS: \0" as *const u8 as *const i8
            },
        );
        let mut ap: ::core::ffi::VaListImpl;
        ap = args.clone();
        vprintf(test_description, ap.as_va_list());
        if status == 0 as i32 {
            if strlen(test_description) < 45 as i32 as u64 {
                printf(
                    b" (%g observed vs %g expected)\0" as *const u8 as *const i8,
                    result,
                    expected,
                );
            } else {
                printf(
                    b" (%g obs vs %g exp)\0" as *const u8 as *const i8,
                    result,
                    expected,
                );
            }
        } else {
            printf(
                b" (%.18g observed vs %.18g expected)\0" as *const u8 as *const i8,
                result,
                expected,
            );
        }
        if status == -(1 as i32) {
            printf(b" [test uses subnormal value]\0" as *const u8 as *const i8);
        }
        if status != 0 && verbose == 0 {
            printf(b" [%u]\0" as *const u8 as *const i8, tests);
        }
        printf(b"\n\0" as *const u8 as *const i8);
        fflush(stdout);
    }
}
#[no_mangle]
pub unsafe extern "C" fn gsl_test_factor(
    mut result: libc::c_double,
    mut expected: libc::c_double,
    mut factor: libc::c_double,
    mut test_description: *const i8,
    mut args: ...
) {
    let mut status: i32 = 0;
    if tests == 0 {
        initialise();
    }
    if expected > 0 as i32 as libc::c_double && expected < 2.2250738585072014e-308f64
        || expected < 0 as i32 as libc::c_double
            && expected > -2.2250738585072014e-308f64
    {
        status = -(1 as i32);
    } else if result == expected {
        status = 0 as i32;
    } else if expected == 0.0f64 {
        status = (result > expected || result < expected) as i32;
    } else {
        let mut u: libc::c_double = result / expected;
        status = (u > factor || u < 1.0f64 / factor) as i32;
    }
    update(status);
    if status != 0 || verbose != 0 {
        printf(
            if status != 0 {
                b"FAIL: \0" as *const u8 as *const i8
            } else {
                b"PASS: \0" as *const u8 as *const i8
            },
        );
        let mut ap: ::core::ffi::VaListImpl;
        ap = args.clone();
        vprintf(test_description, ap.as_va_list());
        if status == 0 as i32 {
            if strlen(test_description) < 45 as i32 as u64 {
                printf(
                    b" (%g observed vs %g expected)\0" as *const u8 as *const i8,
                    result,
                    expected,
                );
            } else {
                printf(
                    b" (%g obs vs %g exp)\0" as *const u8 as *const i8,
                    result,
                    expected,
                );
            }
        } else {
            printf(
                b" (%.18g observed vs %.18g expected)\0" as *const u8 as *const i8,
                result,
                expected,
            );
        }
        if status == -(1 as i32) {
            printf(b" [test uses subnormal value]\0" as *const u8 as *const i8);
        }
        if status != 0 && verbose == 0 {
            printf(b" [%u]\0" as *const u8 as *const i8, tests);
        }
        printf(b"\n\0" as *const u8 as *const i8);
        fflush(stdout);
    }
}
#[no_mangle]
pub unsafe extern "C" fn gsl_test_int(
    mut result: i32,
    mut expected: i32,
    mut test_description: *const i8,
    mut args: ...
) {
    let mut status: i32 = (result != expected) as i32;
    if tests == 0 {
        initialise();
    }
    update(status);
    if status != 0 || verbose != 0 {
        printf(
            if status != 0 {
                b"FAIL: \0" as *const u8 as *const i8
            } else {
                b"PASS: \0" as *const u8 as *const i8
            },
        );
        let mut ap: ::core::ffi::VaListImpl;
        ap = args.clone();
        vprintf(test_description, ap.as_va_list());
        if status == 0 as i32 {
            printf(
                b" (%d observed vs %d expected)\0" as *const u8 as *const i8,
                result,
                expected,
            );
        } else {
            printf(
                b" (%d observed vs %d expected)\0" as *const u8 as *const i8,
                result,
                expected,
            );
        }
        if status != 0 && verbose == 0 {
            printf(b" [%u]\0" as *const u8 as *const i8, tests);
        }
        printf(b"\n\0" as *const u8 as *const i8);
        fflush(stdout);
    }
}
#[no_mangle]
pub unsafe extern "C" fn gsl_test_str(
    mut result: *const i8,
    mut expected: *const i8,
    mut test_description: *const i8,
    mut args: ...
) {
    let mut status: i32 = strcmp(result, expected);
    if tests == 0 {
        initialise();
    }
    update(status);
    if status != 0 || verbose != 0 {
        printf(
            if status != 0 {
                b"FAIL: \0" as *const u8 as *const i8
            } else {
                b"PASS: \0" as *const u8 as *const i8
            },
        );
        let mut ap: ::core::ffi::VaListImpl;
        ap = args.clone();
        vprintf(test_description, ap.as_va_list());
        if status != 0 {
            printf(
                b" (%s observed vs %s expected)\0" as *const u8 as *const i8,
                result,
                expected,
            );
        }
        if status != 0 && verbose == 0 {
            printf(b" [%u]\0" as *const u8 as *const i8, tests);
        }
        printf(b"\n\0" as *const u8 as *const i8);
        fflush(stdout);
    }
}
#[no_mangle]
pub unsafe extern "C" fn gsl_test_verbose(mut v: i32) {
    verbose = v as u32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_test_summary() -> i32 {
    if verbose != 0 && 0 as i32 != 0 {
        printf(
            b"%d tests, passed %d, failed %d.\n\0" as *const u8 as *const i8,
            tests,
            passed,
            failed,
        );
    }
    if failed != 0 as i32 as u32 {
        return 1 as i32;
    }
    if tests != passed.wrapping_add(failed) {
        if verbose != 0 {
            printf(
                b"TEST RESULTS DO NOT ADD UP %d != %d + %d\n\0" as *const u8
                    as *const i8,
                tests,
                passed,
                failed,
            );
        }
        return 1 as i32;
    }
    if passed == tests {
        if verbose == 0 {
            printf(b"Completed [%d/%d]\n\0" as *const u8 as *const i8, passed, tests);
        }
        return 0 as i32;
    }
    return 1 as i32;
}