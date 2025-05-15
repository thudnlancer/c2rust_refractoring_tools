use std::time::{Duration, Instant};
use std::thread;
use std::io::Error;

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
            return Err(Error::from_raw_os_error(22));
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
    let mut remaining = requested_delay.clone();
    let mut total_slept = Duration::new(0, 0);

    while remaining.tv_sec > LIMIT_SECONDS {
        let partial_sleep = Timespec::new(LIMIT_SECONDS, remaining.tv_nsec)?;
        let start = Instant::now();
        
        thread::sleep(partial_sleep.to_duration());
        total_slept += start.elapsed();

        remaining.tv_sec -= LIMIT_SECONDS;
        remaining.tv_nsec = 0;
    }

    let final_sleep = Timespec::new(remaining.tv_sec, remaining.tv_nsec)?;
    let start = Instant::now();
    thread::sleep(final_sleep.to_duration());
    total_slept += start.elapsed();

    let actual_delay = requested_delay.to_duration();
    if total_slept < actual_delay {
        let remaining_nanos = (actual_delay - total_slept).as_nanos();
        let remaining_secs = remaining_nanos / 1_000_000_000;
        let remaining_nsecs = remaining_nanos % 1_000_000_000;
        Ok(Some(Timespec::new(remaining_secs as i64, remaining_nsecs as i64)?))
    } else {
        Ok(None)
    }
}