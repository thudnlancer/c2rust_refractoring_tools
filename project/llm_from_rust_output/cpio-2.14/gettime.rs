use std::time::{SystemTime, UNIX_EPOCH};
use std::io;

#[derive(Debug, Clone, Copy)]
pub struct Timespec {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}

impl Timespec {
    pub fn now() -> io::Result<Self> {
        let duration = SystemTime::now().duration_since(UNIX_EPOCH)?;
        Ok(Self {
            tv_sec: duration.as_secs() as i64,
            tv_nsec: duration.subsec_nanos() as i64,
        })
    }
}

pub fn current_timespec() -> io::Result<Timespec> {
    Timespec::now()
}