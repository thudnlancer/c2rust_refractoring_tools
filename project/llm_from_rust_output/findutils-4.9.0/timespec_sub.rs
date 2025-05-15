use std::time::Duration;

pub type TimeT = i64;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Timespec {
    pub tv_sec: TimeT,
    pub tv_nsec: i64,
}

const TIMESPEC_HZ: u32 = 1_000_000_000;

pub fn make_timespec(s: TimeT, ns: i64) -> Timespec {
    Timespec { tv_sec: s, tv_nsec: ns }
}

pub fn timespec_sub(a: Timespec, b: Timespec) -> Timespec {
    let mut rs = a.tv_sec;
    let mut bs = b.tv_sec;
    let mut ns = a.tv_nsec - b.tv_nsec;
    let mut rns = ns;

    if ns < 0 {
        rns = ns + TIMESPEC_HZ as i64;
        if let Some(new_bs) = bs.checked_add(1) {
            bs = new_bs;
        } else if rs > TimeT::MIN {
            rs -= 1;
        } else {
            return Timespec {
                tv_sec: TimeT::MAX,
                tv_nsec: 0,
            };
        }
    }

    if let Some(new_rs) = rs.checked_sub(bs) {
        rs = new_rs;
    } else {
        if bs > 0 {
            return Timespec {
                tv_sec: TimeT::MAX,
                tv_nsec: 0,
            };
        } else {
            rs = TimeT::MAX;
            rns = TIMESPEC_HZ as i64 - 1;
        }
    }

    make_timespec(rs, rns)
}