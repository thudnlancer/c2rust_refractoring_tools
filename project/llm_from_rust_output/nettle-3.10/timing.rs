use std::ffi::CStr;
use std::io::{self, Write};
use std::time::{Duration, Instant};
use std::process;

struct TimeSpec {
    tv_sec: i64,
    tv_nsec: i64,
}

static mut CGT_START: Option<TimeSpec> = None;
static mut CLOCK_START: Option<Instant> = None;

fn die(message: &str) -> ! {
    writeln!(io::stderr(), "{}", message).unwrap();
    process::exit(1);
}

fn cgt_works() -> bool {
    unsafe {
        let mut now = TimeSpec { tv_sec: 0, tv_nsec: 0 };
        clock_gettime(2, &mut now) == 0
    }
}

fn cgt_time_start() {
    unsafe {
        let mut start = TimeSpec { tv_sec: 0, tv_nsec: 0 };
        if clock_gettime(2, &mut start) < 0 {
            die(&format!("clock_gettime failed: {}", io::Error::last_os_error()));
        }
        CGT_START = Some(start);
    }
}

fn cgt_time_end() -> f64 {
    unsafe {
        let mut end = TimeSpec { tv_sec: 0, tv_nsec: 0 };
        if clock_gettime(2, &mut end) < 0 {
            die(&format!("clock_gettime failed: {}", io::Error::last_os_error()));
        }
        
        let start = CGT_START.unwrap();
        let sec_diff = end.tv_sec - start.tv_sec;
        let nsec_diff = end.tv_nsec - start.tv_nsec;
        sec_diff as f64 + 1e-9 * nsec_diff as f64
    }
}

fn clock_time_start() {
    unsafe {
        CLOCK_START = Some(Instant::now());
    }
}

fn clock_time_end() -> f64 {
    unsafe {
        let duration = CLOCK_START.unwrap().elapsed();
        duration.as_secs() as f64 + duration.subsec_nanos() as f64 * 1e-9
    }
}

pub static mut TIME_START: Option<fn()> = None;
pub static mut TIME_END: Option<fn() -> f64> = None;

pub fn time_init() {
    if cgt_works() {
        unsafe {
            TIME_START = Some(cgt_time_start);
            TIME_END = Some(cgt_time_end);
        }
    } else {
        writeln!(io::stderr(), "clock_gettime not working, falling back to clock").unwrap();
        unsafe {
            TIME_START = Some(clock_time_start);
            TIME_END = Some(clock_time_end);
        }
    }
}

// External FFI functions
extern "C" {
    fn clock_gettime(clock_id: i32, tp: *mut TimeSpec) -> i32;
}