use std::time::{SystemTime, UNIX_EPOCH};
use std::io;

#[derive(Copy, Clone, Debug)]
pub struct Timespec {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}

pub fn gettime() -> io::Result<Timespec> {
    let duration = SystemTime::now().duration_since(UNIX_EPOCH)?;
    Ok(Timespec {
        tv_sec: duration.as_secs() as i64,
        tv_nsec: duration.subsec_nanos() as i64,
    })
}

pub fn current_timespec() -> Timespec {
    gettime().unwrap_or_else(|_| Timespec { tv_sec: 0, tv_nsec: 0 })
}