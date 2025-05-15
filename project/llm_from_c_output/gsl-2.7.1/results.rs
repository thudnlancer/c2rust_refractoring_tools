use std::env;
use std::ffi::CStr;
use std::fmt;
use std::io::{self, Write};
use std::mem;
use std::os::raw::c_char;
use std::ptr;

static mut TESTS: u32 = 0;
static mut PASSED: u32 = 0;
static mut FAILED: u32 = 0;
static mut VERBOSE: u32 = 0;

fn initialise() {
    let verbose = env::var("GSL_TEST_VERBOSE").unwrap_or_default();
    if !verbose.is_empty() {
        unsafe {
            VERBOSE = verbose.parse().unwrap_or(0);
        }
    }
}

fn update(status: i32) {
    unsafe {
        TESTS += 1;
        if status == 0 {
            PASSED += 1;
        } else {
            FAILED += 1;
        }
    }
}

pub fn gsl_test(status: i32, test_description: &str) {
    unsafe {
        if TESTS == 0 {
            initialise();
        }
    }

    update(status);

    unsafe {
        if status != 0 || VERBOSE != 0 {
            print!("{}: {}", if status != 0 { "FAIL" } else { "PASS" }, test_description);

            if status != 0 && VERBOSE == 0 {
                print!(" [{}]", TESTS);
            }

            println!();
            io::stdout().flush().unwrap();
        }
    }
}

pub fn gsl_test_rel(result: f64, expected: f64, relative_error: f64, test_description: &str) {
    let status = if result.is_nan() || expected.is_nan() {
        (result.is_nan() != expected.is_nan()) as i32
    } else if result.is_infinite() || expected.is_infinite() {
        (result.is_infinite() != expected.is_infinite()) as i32
    } else if (expected > 0.0 && expected < f64::MIN_POSITIVE)
        || (expected < 0.0 && expected > -f64::MIN_POSITIVE)
    {
        -1
    } else if expected != 0.0 {
        ((result - expected).abs() / expected.abs() > relative_error) as i32
    } else {
        (result.abs() > relative_error) as i32
    };

    update(status);

    unsafe {
        if status != 0 || VERBOSE != 0 {
            print!("{}: {}", if status != 0 { "FAIL" } else { "PASS" }, test_description);

            if status == 0 {
                if test_description.len() < 45 {
                    print!(" ({} observed vs {} expected)", result, expected);
                } else {
                    print!(" ({} obs vs {} exp)", result, expected);
                }
            } else {
                print!(" ({:.18} observed vs {:.18} expected)", result, expected);
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

pub fn gsl_test_abs(result: f64, expected: f64, absolute_error: f64, test_description: &str) {
    let status = if result.is_nan() || expected.is_nan() {
        (result.is_nan() != expected.is_nan()) as i32
    } else if result.is_infinite() || expected.is_infinite() {
        (result.is_infinite() != expected.is_infinite()) as i32
    } else if (expected > 0.0 && expected < f64::MIN_POSITIVE)
        || (expected < 0.0 && expected > -f64::MIN_POSITIVE)
    {
        -1
    } else {
        ((result - expected).abs() > absolute_error) as i32
    };

    update(status);

    unsafe {
        if status != 0 || VERBOSE != 0 {
            print!("{}: {}", if status != 0 { "FAIL" } else { "PASS" }, test_description);

            if status == 0 {
                if test_description.len() < 45 {
                    print!(" ({} observed vs {} expected)", result, expected);
                } else {
                    print!(" ({} obs vs {} exp)", result, expected);
                }
            } else {
                print!(" ({:.18} observed vs {:.18} expected)", result, expected);
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

pub fn gsl_test_factor(result: f64, expected: f64, factor: f64, test_description: &str) {
    let status = if (expected > 0.0 && expected < f64::MIN_POSITIVE)
        || (expected < 0.0 && expected > -f64::MIN_POSITIVE)
    {
        -1
    } else if result == expected {
        0
    } else if expected == 0.0 {
        (result > expected || result < expected) as i32
    } else {
        let u = result / expected;
        (u > factor || u < 1.0 / factor) as i32
    };

    update(status);

    unsafe {
        if status != 0 || VERBOSE != 0 {
            print!("{}: {}", if status != 0 { "FAIL" } else { "PASS" }, test_description);

            if status == 0 {
                if test_description.len() < 45 {
                    print!(" ({} observed vs {} expected)", result, expected);
                } else {
                    print!(" ({} obs vs {} exp)", result, expected);
                }
            } else {
                print!(" ({:.18} observed vs {:.18} expected)", result, expected);
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

pub fn gsl_test_int(result: i32, expected: i32, test_description: &str) {
    let status = (result != expected) as i32;

    update(status);

    unsafe {
        if status != 0 || VERBOSE != 0 {
            print!("{}: {}", if status != 0 { "FAIL" } else { "PASS" }, test_description);

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
    let status = (result != expected) as i32;

    update(status);

    unsafe {
        if status != 0 || VERBOSE != 0 {
            print!("{}: {}", if status != 0 { "FAIL" } else { "PASS" }, test_description);

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

pub fn gsl_test_verbose(v: i32) {
    unsafe {
        VERBOSE = v as u32;
    }
}

pub fn gsl_test_summary() -> i32 {
    unsafe {
        if VERBOSE != 0 && false {
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