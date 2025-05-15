use std::env;
use std::ffi::{CStr, CString};
use std::io::{self, Write};
use std::os::raw::{c_char, c_double, c_int, c_uint};
use std::ptr;

extern "C" {
    fn gsl_isnan(x: c_double) -> c_int;
    fn gsl_isinf(x: c_double) -> c_int;
}

static mut TESTS: c_uint = 0;
static mut PASSED: c_uint = 0;
static mut FAILED: c_uint = 0;
static mut VERBOSE: c_uint = 0;

fn initialise() {
    unsafe {
        if let Ok(val) = env::var("GSL_TEST_VERBOSE") {
            if !val.is_empty() {
                VERBOSE = val.parse().unwrap_or(0);
            }
        }
    }
}

fn update(status: c_int) {
    unsafe {
        TESTS += 1;
        if status == 0 {
            PASSED += 1;
        } else {
            FAILED += 1;
        }
    }
}

pub fn gsl_test(status: c_int, test_description: &str) {
    unsafe {
        if TESTS == 0 {
            initialise();
        }
        update(status);

        if status != 0 || VERBOSE != 0 {
            let prefix = if status != 0 { "FAIL: " } else { "PASS: " };
            print!("{}{}", prefix, test_description);
            
            if status != 0 && VERBOSE == 0 {
                print!(" [{}]", TESTS);
            }
            println!();
            io::stdout().flush().unwrap();
        }
    }
}

pub fn gsl_test_rel(
    result: c_double,
    expected: c_double,
    relative_error: c_double,
    test_description: &str,
) {
    let status = unsafe {
        if TESTS == 0 {
            initialise();
        }

        if gsl_isnan(result) != 0 || gsl_isnan(expected) != 0 {
            (gsl_isnan(result) != gsl_isnan(expected)) as c_int
        } else if gsl_isinf(result) != 0 || gsl_isinf(expected) != 0 {
            (gsl_isinf(result) != gsl_isinf(expected)) as c_int
        } else if (expected > 0.0 && expected < 2.2250738585072014e-308)
            || (expected < 0.0 && expected > -2.2250738585072014e-308)
        {
            -1
        } else if expected != 0.0 {
            ((result - expected).abs() / expected.abs() > relative_error) as c_int
        } else {
            (result.abs() > relative_error) as c_int
        }
    };

    update(status);
    print_test_result(status, test_description, result, expected);
}

pub fn gsl_test_abs(
    result: c_double,
    expected: c_double,
    absolute_error: c_double,
    test_description: &str,
) {
    let status = unsafe {
        if TESTS == 0 {
            initialise();
        }

        if gsl_isnan(result) != 0 || gsl_isnan(expected) != 0 {
            (gsl_isnan(result) != gsl_isnan(expected)) as c_int
        } else if gsl_isinf(result) != 0 || gsl_isinf(expected) != 0 {
            (gsl_isinf(result) != gsl_isinf(expected)) as c_int
        } else if (expected > 0.0 && expected < 2.2250738585072014e-308)
            || (expected < 0.0 && expected > -2.2250738585072014e-308)
        {
            -1
        } else {
            ((result - expected).abs() > absolute_error) as c_int
        }
    };

    update(status);
    print_test_result(status, test_description, result, expected);
}

fn print_test_result(status: c_int, test_description: &str, result: c_double, expected: c_double) {
    unsafe {
        if status != 0 || VERBOSE != 0 {
            let prefix = if status != 0 { "FAIL: " } else { "PASS: " };
            print!("{}{}", prefix, test_description);

            if status == 0 {
                if test_description.len() < 45 {
                    print!(" ({} observed vs {} expected)", result, expected);
                } else {
                    print!(" ({} obs vs {} exp)", result, expected);
                }
            } else {
                print!(
                    " ({:.18} observed vs {:.18} expected)",
                    result, expected
                );
            }

            if status == -1 {
                print!(" [test uses subnormal value]");
            }

            if status != 0 && VERBOSE == 0 {
                print!(" [{}]", TESTS);
            }

            println!();
            io::stdout().flush().unwrap();
        }
    }
}

pub fn gsl_test_int(result: c_int, expected: c_int, test_description: &str) {
    let status = (result != expected) as c_int;
    unsafe {
        if TESTS == 0 {
            initialise();
        }
    }
    update(status);

    unsafe {
        if status != 0 || VERBOSE != 0 {
            let prefix = if status != 0 { "FAIL: " } else { "PASS: " };
            print!("{}{}", prefix, test_description);

            print!(" ({} observed vs {} expected)", result, expected);

            if status != 0 && VERBOSE == 0 {
                print!(" [{}]", TESTS);
            }

            println!();
            io::stdout().flush().unwrap();
        }
    }
}

pub fn gsl_test_str(result: &str, expected: &str, test_description: &str) {
    let status = if result == expected { 0 } else { 1 };
    unsafe {
        if TESTS == 0 {
            initialise();
        }
    }
    update(status);

    unsafe {
        if status != 0 || VERBOSE != 0 {
            let prefix = if status != 0 { "FAIL: " } else { "PASS: " };
            print!("{}{}", prefix, test_description);

            if status != 0 {
                print!(" ({} observed vs {} expected)", result, expected);
            }

            if status != 0 && VERBOSE == 0 {
                print!(" [{}]", TESTS);
            }

            println!();
            io::stdout().flush().unwrap();
        }
    }
}

pub fn gsl_test_verbose(v: c_int) {
    unsafe {
        VERBOSE = v as c_uint;
    }
}

pub fn gsl_test_summary() -> c_int {
    unsafe {
        if VERBOSE != 0 {
            println!("{} tests, passed {}, failed {}.", TESTS, PASSED, FAILED);
        }

        if FAILED != 0 {
            return 1;
        }

        if TESTS != PASSED + FAILED {
            if VERBOSE != 0 {
                println!(
                    "TEST RESULTS DO NOT ADD UP {} != {} + {}",
                    TESTS, PASSED, FAILED
                );
            }
            return 1;
        }

        if PASSED == TESTS {
            if VERBOSE == 0 {
                println!("Completed [{}/{}]", PASSED, TESTS);
            }
            return 0;
        }

        1
    }
}