use std::time::Duration;

pub type TimeT = i64;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Timespec {
    pub tv_sec: TimeT,
    pub tv_nsec: i64,
}

const TIMESPEC_HZ: u32 = 1_000_000_000;

fn make_timespec(s: TimeT, ns: i64) -> Timespec {
    Timespec { tv_sec: s, tv_nsec: ns }
}

pub fn timespec_add(a: Timespec, b: Timespec) -> Timespec {
    let mut rs = a.tv_sec;
    let mut bs = b.tv_sec;
    let ns = a.tv_nsec + b.tv_nsec;
    
    let (rns, carry) = if ns >= TIMESPEC_HZ as i64 {
        (ns - TIMESPEC_HZ as i64, 1)
    } else {
        (ns, 0)
    };
    
    bs += carry as TimeT;
    
    match rs.checked_add(bs) {
        Some(sum) => {
            rs = sum;
        }
        None => {
            if bs < 0 {
                rs = TimeT::MAX;
                return make_timespec(rs, 0);
            } else {
                rs = TimeT::MAX;
                return make_timespec(rs, TIMESPEC_HZ as i64 - 1);
            }
        }
    }
    
    make_timespec(rs, rns)
}