use std::time::{Duration, Instant};
use std::thread;
use std::io;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Once;
use libc::{c_int, timespec};

const BILLION: i64 = 1_000_000_000;

#[cfg(target_os = "windows")]
mod windows {
    use super::*;
    use winapi::um::profileapi::{QueryPerformanceCounter, QueryPerformanceFrequency};
    use winapi::um::synchapi::Sleep;
    use winapi::shared::minwindef::DWORD;
    use std::ptr;

    pub fn nanosleep(requested_delay: &timespec, remaining_delay: Option<&mut timespec>) -> c_int {
        if requested_delay.tv_nsec < 0 || requested_delay.tv_nsec >= BILLION {
            errno::set_errno(errno::Errno(libc::EINVAL));
            return -1;
        }

        static INIT: Once = Once::new();
        static mut TICKS_PER_NANOSECOND: f64 = 0.0;

        unsafe {
            INIT.call_once(|| {
                let mut freq = 0;
                if QueryPerformanceFrequency(&mut freq) != 0 {
                    TICKS_PER_NANOSECOND = freq as f64 / 1_000_000_000.0;
                }
            });
        }

        if requested_delay.tv_sec == 0 {
            unsafe {
                if TICKS_PER_NANOSECOND != 0.0 {
                    let sleep_millis = (requested_delay.tv_nsec / 1_000_000) as DWORD - 10;
                    let wait_ticks = (requested_delay.tv_nsec as f64 * TICKS_PER_NANOSECOND) as i64;
                    let mut counter_before = 0;
                    if QueryPerformanceCounter(&mut counter_before) != 0 {
                        let wait_until = counter_before + wait_ticks;
                        if sleep_millis > 0 {
                            Sleep(sleep_millis);
                        }
                        loop {
                            let mut counter_after = 0;
                            if QueryPerformanceCounter(&mut counter_after) == 0 {
                                break;
                            }
                            if counter_after >= wait_until {
                                break;
                            }
                        }
                        if let Some(remaining) = remaining_delay {
                            remaining.tv_sec = 0;
                            remaining.tv_nsec = 0;
                        }
                        return 0;
                    }
                }
            }
        }

        let total_millis = requested_delay.tv_sec * 1000 + requested_delay.tv_nsec / 1_000_000;
        unsafe {
            Sleep(total_millis as DWORD);
        }
        if let Some(remaining) = remaining_delay {
            remaining.tv_sec = 0;
            remaining.tv_nsec = 0;
        }
        0
    }
}

#[cfg(not(target_os = "windows"))]
mod unix {
    use super::*;
    use libc::{sigaction, sigemptyset, SIGCONT, SIGTERM, SA_RESTART};
    use std::mem;
    use std::ptr;

    static SUSPENDED: AtomicBool = AtomicBool::new(false);
    static INIT: Once = Once::new();

    extern "C" fn sighandler(_: c_int) {
        SUSPENDED.store(true, Ordering::SeqCst);
    }

    fn my_usleep(ts_delay: &timespec) -> c_int {
        let mut delay = libc::timeval {
            tv_sec: ts_delay.tv_sec,
            tv_usec: (ts_delay.tv_nsec + 999) / 1000,
        };
        if delay.tv_usec == 1_000_000 {
            if delay.tv_sec == i64::MAX {
                delay.tv_usec = 999_999;
            } else {
                delay.tv_sec += 1;
                delay.tv_usec = 0;
            }
        }
        unsafe { libc::select(0, ptr::null_mut(), ptr::null_mut(), ptr::null_mut(), &delay) }
    }

    pub fn nanosleep(requested_delay: &timespec, remaining_delay: Option<&mut timespec>) -> c_int {
        if requested_delay.tv_nsec < 0 || requested_delay.tv_nsec >= BILLION {
            errno::set_errno(errno::Errno(libc::EINVAL));
            return -1;
        }

        INIT.call_once(|| {
            let mut oldact: sigaction = unsafe { mem::zeroed() };
            unsafe { sigaction(SIGCONT, ptr::null(), &mut oldact) };
            if oldact.sa_sigaction != libc::SIG_IGN {
                let mut newact: sigaction = unsafe { mem::zeroed() };
                newact.sa_sigaction = sighandler as usize;
                unsafe { sigemptyset(&mut newact.sa_mask) };
                newact.sa_flags = SA_RESTART as i32;
                unsafe { sigaction(SIGCONT, &newact, ptr::null_mut()) };
            }
        });

        SUSPENDED.store(false, Ordering::SeqCst);

        if my_usleep(requested_delay) == -1 {
            if SUSPENDED.load(Ordering::SeqCst) {
                errno::set_errno(errno::Errno(libc::EINTR));
            }
            return -1;
        }

        0
    }
}

#[cfg(target_os = "windows")]
pub fn nanosleep(requested_delay: &timespec, remaining_delay: Option<&mut timespec>) -> c_int {
    windows::nanosleep(requested_delay, remaining_delay)
}

#[cfg(not(target_os = "windows"))]
pub fn nanosleep(requested_delay: &timespec, remaining_delay: Option<&mut timespec>) -> c_int {
    unix::nanosleep(requested_delay, remaining_delay)
}

#[cfg(test)]
mod tests {
    use super::*;
    use libc::{timespec, EINVAL};

    #[test]
    fn test_invalid_nanos() {
        let req = timespec {
            tv_sec: 0,
            tv_nsec: -1,
        };
        assert_eq!(nanosleep(&req, None), -1);
        assert_eq!(errno::errno().0, EINVAL);
    }

    #[test]
    fn test_valid_sleep() {
        let req = timespec {
            tv_sec: 0,
            tv_nsec: 100_000,
        };
        assert_eq!(nanosleep(&req, None), 0);
    }
}