use std::time::{Duration, Instant};
use std::thread;
use std::io;
use std::convert::TryInto;

const BILLION: u64 = 1_000_000_000;
const LIMIT_SECONDS: i64 = 24 * 60 * 60;

#[derive(Debug, Clone, Copy)]
pub struct Timespec {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}

impl Timespec {
    pub fn to_duration(&self) -> Option<Duration> {
        if self.tv_nsec < 0 || self.tv_nsec >= BILLION as i64 {
            return None;
        }
        Some(Duration::new(
            self.tv_sec.try_into().unwrap_or(0),
            self.tv_nsec.try_into().unwrap_or(0),
        ))
    }

    pub fn from_duration(duration: Duration) -> Self {
        Timespec {
            tv_sec: duration.as_secs() as i64,
            tv_nsec: duration.subsec_nanos() as i64,
        }
    }
}

pub fn rpl_nanosleep(
    requested_delay: &Timespec,
    remaining_delay: Option<&mut Timespec>,
) -> Result<(), io::Error> {
    let mut delay = match requested_delay.to_duration() {
        Some(d) => d,
        None => return Err(io::Error::from_raw_os_error(22)),
    };

    let limit = Duration::from_secs(LIMIT_SECONDS as u64);
    let mut total_elapsed = Duration::new(0, 0);

    while delay > limit {
        let start = Instant::now();
        thread::sleep(limit);
        let elapsed = start.elapsed();

        if elapsed < limit {
            // Sleep was interrupted
            let remaining = limit - elapsed;
            if let Some(remaining_delay) = remaining_delay {
                *remaining_delay = Timespec::from_duration(remaining + delay - limit);
            }
            return Err(io::Error::last_os_error());
        }

        delay -= limit;
        total_elapsed += limit;
    }

    let start = Instant::now();
    thread::sleep(delay);
    let elapsed = start.elapsed();

    if elapsed < delay {
        // Sleep was interrupted
        let remaining = delay - elapsed;
        if let Some(remaining_delay) = remaining_delay {
            *remaining_delay = Timespec::from_duration(remaining);
        }
        return Err(io::Error::last_os_error());
    }

    Ok(())
}