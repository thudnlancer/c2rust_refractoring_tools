use std::time::{Duration, Instant};
use std::thread;
use std::io::Error;
use std::convert::TryInto;

const BILLION: u64 = 1_000_000_000;

#[cfg(target_os = "windows")]
mod windows {
    use super::*;
    use winapi::um::profileapi::{QueryPerformanceCounter, QueryPerformanceFrequency};
    use winapi::um::synchapi::Sleep;
    use std::mem::MaybeUninit;
    use std::ptr;

    pub fn nanosleep(requested_delay: &Duration) -> Result<(), Error> {
        if requested_delay.subsec_nanos() >= BILLION as u32 {
            return Err(Error::new(std::io::ErrorKind::InvalidInput, "Invalid nanoseconds value"));
        }

        // For delays >= 1 second, use Sleep
        if requested_delay.as_secs() > 0 {
            let total_millis = requested_delay.as_secs() * 1000 + u64::from(requested_delay.subsec_millis());
            unsafe { Sleep(total_millis as u32) };
            return Ok(());
        }

        static INIT: std::sync::Once = std::sync::Once::new();
        static mut TICKS_PER_NANOSECOND: f64 = 0.0;
        static mut INITIALIZED: bool = false;

        INIT.call_once(|| {
            unsafe {
                let mut freq = MaybeUninit::uninit();
                if QueryPerformanceFrequency(freq.as_mut_ptr()) != 0 {
                    TICKS_PER_NANOSECOND = (*freq.as_ptr()) as f64 / 1_000_000_000.0;
                }
                INITIALIZED = true;
            }
        });

        unsafe {
            if !INITIALIZED || TICKS_PER_NANOSECOND == 0.0 {
                Sleep(requested_delay.subsec_nanos() as u32 / 1_000_000);
                return Ok(());
            }

            let wait_ticks = (requested_delay.subsec_nanos() as f64 * TICKS_PER_NANOSECOND) as i64;
            let mut counter_before = MaybeUninit::uninit();
            if QueryPerformanceCounter(counter_before.as_mut_ptr()) == 0 {
                return Err(Error::last_os_error());
            }

            let wait_until = counter_before.assume_init().QuadPart + wait_ticks;
            let sleep_millis = requested_delay.subsec_nanos() / 1_000_000 - 10;

            if sleep_millis > 0 {
                Sleep(sleep_millis as u32);
            }

            loop {
                let mut counter_after = MaybeUninit::uninit();
                if QueryPerformanceCounter(counter_after.as_mut_ptr()) == 0 {
                    break;
                }
                if counter_after.assume_init().QuadPart >= wait_until {
                    break;
                }
            }
        }

        Ok(())
    }
}

#[cfg(not(target_os = "windows"))]
mod unix {
    use super::*;
    use libc::{timespec, pselect, fd_set};
    use std::mem::MaybeUninit;
    use std::ptr;

    pub fn nanosleep(requested_delay: &Duration) -> Result<(), Error> {
        if requested_delay.subsec_nanos() >= BILLION as u32 {
            return Err(Error::new(std::io::ErrorKind::InvalidInput, "Invalid nanoseconds value"));
        }

        let mut req = timespec {
            tv_sec: requested_delay.as_secs().try_into().unwrap_or(libc::time_t::MAX),
            tv_nsec: requested_delay.subsec_nanos() as libc::c_long,
        };

        // Handle large delays by breaking into smaller chunks
        const LIMIT: libc::time_t = 24 * 24 * 60 * 60;
        while req.tv_sec > LIMIT {
            let mut rem = MaybeUninit::uninit();
            let mut chunk = timespec {
                tv_sec: LIMIT,
                tv_nsec: req.tv_nsec,
            };

            unsafe {
                if libc::nanosleep(&chunk, rem.as_mut_ptr()) != 0 {
                    let err = Error::last_os_error();
                    if err.raw_os_error() == Some(libc::EINTR) {
                        req.tv_sec -= LIMIT - rem.assume_init().tv_sec;
                        req.tv_nsec = rem.assume_init().tv_nsec;
                        continue;
                    }
                    return Err(err);
                }
            }

            req.tv_sec -= LIMIT;
            req.tv_nsec = 0;
        }

        unsafe {
            if libc::nanosleep(&req, ptr::null_mut()) != 0 {
                return Err(Error::last_os_error());
            }
        }

        Ok(())
    }
}

pub fn nanosleep(requested_delay: &Duration) -> Result<(), Error> {
    #[cfg(target_os = "windows")]
    return windows::nanosleep(requested_delay);
    
    #[cfg(not(target_os = "windows"))]
    return unix::nanosleep(requested_delay);
}