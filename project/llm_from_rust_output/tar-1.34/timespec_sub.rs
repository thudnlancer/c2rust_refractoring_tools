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

pub fn timespec_sub(a: Timespec, b: Timespec) -> Timespec {
    let mut rs = a.tv_sec;
    let mut bs = b.tv_sec;
    let mut ns = (a.tv_nsec - b.tv_nsec) as i32;
    let mut rns = ns;

    if ns < 0 {
        rns = ns + TIMESPEC_HZ as i32;
        bs = match bs.checked_add(1) {
            Some(new_bs) => new_bs,
            None => {
                if rs > 0 {
                    rs -= 1;
                }
                bs
            }
        };
    }

    rs = match rs.checked_sub(bs) {
        Some(new_rs) => new_rs,
        None => {
            if bs > 0 {
                return Timespec {
                    tv_sec: TimeT::MIN,
                    tv_nsec: 0,
                };
            } else {
                rns = TIMESPEC_HZ as i32 - 1;
                TimeT::MAX
            }
        }
    };

    make_timespec(rs, rns as i64)
}