use chrono::{DateTime, TimeZone, Utc};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Copy)]
pub struct Tm {
    pub tm_sec: i32,
    pub tm_min: i32,
    pub tm_hour: i32,
    pub tm_mday: i32,
    pub tm_mon: i32,
    pub tm_year: i32,
    pub tm_wday: i32,
    pub tm_yday: i32,
    pub tm_isdst: i32,
}

pub fn rpl_timegm(tmp: &mut Tm) -> i64 {
    let datetime = Utc
        .ymd(tmp.tm_year + 1900, (tmp.tm_mon + 1) as u32, tmp.tm_mday as u32)
        .and_hms(tmp.tm_hour as u32, tmp.tm_min as u32, tmp.tm_sec as u32);

    datetime.timestamp()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rpl_timegm() {
        let mut tm = Tm {
            tm_sec: 0,
            tm_min: 0,
            tm_hour: 0,
            tm_mday: 1,
            tm_mon: 0,
            tm_year: 70,
            tm_wday: 0,
            tm_yday: 0,
            tm_isdst: 0,
        };

        let timestamp = rpl_timegm(&mut tm);
        assert_eq!(timestamp, 0);
    }
}