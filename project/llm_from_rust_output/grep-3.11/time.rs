use std::time::{SystemTime, UNIX_EPOCH};
use std::ptr;

#[derive(Copy, Clone, Debug)]
pub struct Timeval {
    pub tv_sec: i64,
    pub tv_usec: i64,
}

#[derive(Copy, Clone, Debug)]
pub struct Timezone {
    pub tz_minuteswest: i32,
    pub tz_dsttime: i32,
}

pub fn rpl_time(tp: Option<&mut i64>) -> i64 {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => {
            let tt = duration.as_secs() as i64;
            if let Some(t) = tp {
                *t = tt;
            }
            tt
        }
        Err(_) => {
            panic!("Failed to get current time");
        }
    }
}