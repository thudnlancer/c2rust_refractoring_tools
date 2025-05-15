use std::time::{Duration, Instant};
use std::thread;
use std::num::NonZeroI32;
use std::io::Error;

const BILLION: u64 = 1_000_000_000;
const LIMIT_SECONDS: i64 = 24 * 24 * 60 * 60; // 24 days in seconds

#[derive(Debug, Clone, Copy)]
pub struct Timespec {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}

impl Timespec {
    pub fn new(sec: i64, nsec: i64) -> Result<Self, Error> {
        if nsec < 0 || nsec >= BILLION as i64 {
            return Err(Error::from_raw_os_error(22)); // EINVAL
        }
        Ok(Timespec { tv_sec: sec, tv_nsec: nsec })
    }

    pub fn to_duration(&self) -> Duration {
        Duration::new(self.tv_sec as u64, self.tv_nsec as u32)
    }

    pub fn from_duration(d: Duration) -> Self {
        Timespec {
            tv_sec: d.as_secs() as i64,
            tv_nsec: d.subsec_nanos() as i64,
        }
    }
}

pub fn rpl_nanosleep(requested_delay: &Timespec) -> Result<Option<Timespec>, Error> {
    let mut remaining = requested_delay.tv_sec;
    let mut intermediate = Timespec {
        tv_sec: 0,
        tv_nsec: requested_delay.tv_nsec,
    };

    while LIMIT_SECONDS < remaining {
        intermediate.tv_sec = LIMIT_SECONDS;
        match thread::sleep_future(intermediate.to_duration()) {
            Ok(_) => {
                remaining -= LIMIT_SECONDS;
                intermediate.tv_nsec = 0;
            }
            Err(e) => {
                if let Some(raw) = e.raw_os_error() {
                    if raw == libc::EINTR {
                        let mut rem = Timespec::from_duration(e.duration());
                        rem.tv_sec += remaining;
                        return Ok(Some(rem));
                    }
                }
                return Err(e);
            }
        }
    }

    intermediate.tv_sec = remaining;
    match thread::sleep_future(intermediate.to_duration()) {
        Ok(_) => Ok(None),
        Err(e) => {
            if let Some(raw) = e.raw_os_error() {
                if raw == libc::EINTR {
                    Ok(Some(Timespec::from_duration(e.duration())))
                } else {
                    Err(e)
                }
            } else {
                Err(e)
            }
        }
    }
}

// Helper function to simulate sleep with remaining time
fn sleep_future(duration: Duration) -> Result<(), Error> {
    let start = Instant::now();
    thread::sleep(duration);
    if start.elapsed() < duration {
        Err(Error::new(
            std::io::ErrorKind::Interrupted,
            "Sleep interrupted",
        ))
    } else {
        Ok(())
    }
}