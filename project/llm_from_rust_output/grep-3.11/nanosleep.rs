use std::time::{Duration, Instant};
use std::thread;
use std::io::{Error, ErrorKind};

const BILLION: u32 = 1_000_000_000;
const LIMIT_SECONDS: i64 = 24 * 60 * 60;

#[derive(Debug, Clone, Copy)]
pub struct Timespec {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}

impl Timespec {
    pub fn new(seconds: i64, nanoseconds: i64) -> Result<Self, Error> {
        if nanoseconds < 0 || nanoseconds >= BILLION as i64 {
            return Err(Error::new(ErrorKind::InvalidInput, "invalid nanoseconds value"));
        }
        Ok(Self {
            tv_sec: seconds,
            tv_nsec: nanoseconds,
        })
    }

    pub fn to_duration(&self) -> Duration {
        Duration::new(self.tv_sec as u64, self.tv_nsec as u32)
    }
}

pub fn rpl_nanosleep(requested_delay: &Timespec) -> Result<Option<Timespec>, Error> {
    let mut remaining_seconds = requested_delay.tv_sec;
    let mut remaining_nanos = requested_delay.tv_nsec;
    let mut total_remaining = None;

    while remaining_seconds > LIMIT_SECONDS {
        let partial_delay = Timespec::new(LIMIT_SECONDS, remaining_nanos)?;
        let start = Instant::now();
        thread::sleep(partial_delay.to_duration());
        
        remaining_seconds -= LIMIT_SECONDS;
        remaining_nanos = 0;
        
        if start.elapsed() < partial_delay.to_duration() {
            let remaining = Timespec::new(remaining_seconds, remaining_nanos)?;
            total_remaining = Some(remaining);
            break;
        }
    }

    if remaining_seconds > 0 || remaining_nanos > 0 {
        let final_delay = Timespec::new(remaining_seconds, remaining_nanos)?;
        let start = Instant::now();
        thread::sleep(final_delay.to_duration());
        
        if start.elapsed() < final_delay.to_duration() {
            let elapsed = start.elapsed();
            let remaining = final_delay.to_duration() - elapsed;
            let remaining_secs = remaining.as_secs() as i64;
            let remaining_ns = remaining.subsec_nanos() as i64;
            total_remaining = Some(Timespec::new(remaining_secs, remaining_ns)?);
        }
    }

    Ok(total_remaining)
}