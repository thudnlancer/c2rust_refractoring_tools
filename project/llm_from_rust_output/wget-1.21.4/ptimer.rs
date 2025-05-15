use std::time::{SystemTime, Duration};
use std::sync::Once;
use std::ffi::CStr;
use libc::{self, c_int, c_double, c_void, c_long, c_char};
use std::ptr;

#[derive(Debug, Clone, Copy)]
struct Timespec {
    tv_sec: libc::time_t,
    tv_nsec: libc::c_long,
}

#[derive(Debug)]
pub struct Ptimer {
    start: Timespec,
    elapsed_last: c_double,
    elapsed_pre_start: c_double,
}

static mut POSIX_CLOCK_ID: c_int = 0;
static mut POSIX_CLOCK_RESOLUTION: c_double = 0.0;
static INIT: Once = Once::new();

fn posix_init() {
    unsafe {
        INIT.call_once(|| {
            let clocks = [
                (1, libc::_SC_MONOTONIC_CLOCK),
                (0, -1),
            ];

            for (id, sysconf_name) in clocks.iter() {
                if *sysconf_name != -1 && libc::sysconf(*sysconf_name) < 0 {
                    continue;
                }

                let mut r = Timespec { tv_sec: 0, tv_nsec: 0 };
                if libc::clock_getres(*id, &mut r as *mut Timespec as *mut libc::timespec) >= 0 {
                    POSIX_CLOCK_ID = *id;
                    POSIX_CLOCK_RESOLUTION = r.tv_sec as c_double + r.tv_nsec as c_double / 1e9;
                    if POSIX_CLOCK_RESOLUTION == 0.0 {
                        POSIX_CLOCK_RESOLUTION = 1e-3;
                    }
                    return;
                }
            }

            let errno = *libc::__errno_location();
            let err_msg = CStr::from_ptr(libc::strerror(errno)).to_string_lossy();
            eprintln!("Cannot get REALTIME clock frequency: {}", err_msg);
            POSIX_CLOCK_ID = 0;
            POSIX_CLOCK_RESOLUTION = 1e-3;
        });
    }
}

fn posix_measure(pst: &mut Timespec) {
    unsafe {
        libc::clock_gettime(POSIX_CLOCK_ID, pst as *mut Timespec as *mut libc::timespec);
    }
}

fn posix_diff(pst1: &Timespec, pst2: &Timespec) -> c_double {
    (pst1.tv_sec - pst2.tv_sec) as c_double + 
    (pst1.tv_nsec - pst2.tv_nsec) as c_double / 1e9
}

fn posix_resolution() -> c_double {
    unsafe { POSIX_CLOCK_RESOLUTION }
}

impl Ptimer {
    pub fn new() -> Self {
        posix_init();
        let mut pt = Ptimer {
            start: Timespec { tv_sec: 0, tv_nsec: 0 },
            elapsed_last: 0.0,
            elapsed_pre_start: 0.0,
        };
        pt.reset();
        pt
    }

    pub fn reset(&mut self) {
        posix_measure(&mut self.start);
        self.elapsed_last = 0.0;
        self.elapsed_pre_start = 0.0;
    }

    pub fn measure(&mut self) -> c_double {
        let mut now = Timespec { tv_sec: 0, tv_nsec: 0 };
        posix_measure(&mut now);
        let elapsed = self.elapsed_pre_start + posix_diff(&now, &self.start);
        
        if elapsed < self.elapsed_last {
            self.start = now;
            self.elapsed_pre_start = self.elapsed_last;
            self.elapsed_last = elapsed;
        } else {
            self.elapsed_last = elapsed;
        }
        
        self.elapsed_last
    }

    pub fn read(&self) -> c_double {
        self.elapsed_last
    }

    pub fn resolution() -> c_double {
        posix_resolution()
    }
}

impl Drop for Ptimer {
    fn drop(&mut self) {
        // No special cleanup needed in safe Rust
    }
}