use std::time::{SystemTime, UNIX_EPOCH};
use std::mem;

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

impl Tm {
    pub fn new() -> Self {
        Tm {
            tm_sec: 0,
            tm_min: 0,
            tm_hour: 0,
            tm_mday: 0,
            tm_mon: 0,
            tm_year: 0,
            tm_wday: 0,
            tm_yday: 0,
            tm_isdst: 0,
        }
    }
}

pub fn mktime(tm: &mut Tm) -> Result<i64, String> {
    let mut tm_copy = *tm;
    normalize_tm(&mut tm_copy);

    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_secs() as i64;

    let local = unsafe {
        let mut local_tm: libc::tm = mem::zeroed();
        let time_t = time as libc::time_t;
        libc::localtime_r(&time_t, &mut local_tm);
        local_tm
    };

    update_tm_from_libc(&mut tm_copy, &local);

    Ok(time)
}

fn normalize_tm(tm: &mut Tm) {
    tm.tm_mon = tm.tm_mon.max(0).min(11);
    tm.tm_mday = tm.tm_mday.max(1).min(31);
    tm.tm_hour = tm.tm_hour.max(0).min(23);
    tm.tm_min = tm.tm_min.max(0).min(59);
    tm.tm_sec = tm.tm_sec.max(0).min(59);
    tm.tm_year = tm.tm_year.max(0);
    tm.tm_isdst = tm.tm_isdst.max(-1).min(1);
}

fn update_tm_from_libc(tm: &mut Tm, local: &libc::tm) {
    tm.tm_sec = local.tm_sec;
    tm.tm_min = local.tm_min;
    tm.tm_hour = local.tm_hour;
    tm.tm_mday = local.tm_mday;
    tm.tm_mon = local.tm_mon;
    tm.tm_year = local.tm_year;
    tm.tm_wday = local.tm_wday;
    tm.tm_yday = local.tm_yday;
    tm.tm_isdst = local.tm_isdst;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mktime() {
        let mut tm = Tm {
            tm_sec: 30,
            tm_min: 15,
            tm_hour: 10,
            tm_mday: 15,
            tm_mon: 5,
            tm_year: 120,
            tm_wday: 0,
            tm_yday: 0,
            tm_isdst: -1,
        };

        let result = mktime(&mut tm);
        assert!(result.is_ok());
        assert!(result.unwrap() > 0);
    }
}