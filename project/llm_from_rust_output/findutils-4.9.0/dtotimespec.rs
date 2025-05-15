use std::time::Duration;

pub type TimeT = i64;

#[derive(Copy, Clone, Debug)]
pub struct Timespec {
    pub tv_sec: TimeT,
    pub tv_nsec: i64,
}

const TIMESPEC_HZ: u32 = 1_000_000_000;
const TIME_MAX: TimeT = TimeT::MAX;
const TIME_MIN: TimeT = TimeT::MIN;

fn make_timespec(s: TimeT, ns: i64) -> Timespec {
    Timespec { tv_sec: s, tv_nsec: ns }
}

pub fn dtotimespec(sec: f64) -> Timespec {
    if sec >= TIME_MAX as f64 {
        make_timespec(TIME_MAX, 0)
    } else if sec <= TIME_MIN as f64 + 1.0 {
        make_timespec(TIME_MIN, (TIMESPEC_HZ as i64) - 1)
    } else {
        let s = sec as TimeT;
        let frac = TIMESPEC_HZ as f64 * (sec - s as f64);
        let mut ns = frac as i64;
        ns += ((ns as f64) < frac) as i64;
        
        let s = s + ns / TIMESPEC_HZ as i64;
        ns %= TIMESPEC_HZ as i64;
        
        if ns < 0 {
            let s = s - 1;
            let ns = ns + TIMESPEC_HZ as i64;
            make_timespec(s, ns)
        } else {
            make_timespec(s, ns)
        }
    }
}