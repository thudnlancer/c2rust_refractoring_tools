use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::thread;
use std::io;

/// Represents time with seconds and microseconds components
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PthTime {
    pub tv_sec: i64,
    pub tv_usec: i64,
}

/// Global zero time constant
pub const PTH_TIME_ZERO: PthTime = PthTime { tv_sec: 0, tv_usec: 0 };

/// Sleep for a specified amount of microseconds
pub fn pth_time_usleep(usec: u64) -> io::Result<()> {
    thread::sleep(Duration::from_micros(usec));
    Ok(())
}

/// Set time value t1 to t2 or current time if t2 is None
pub fn pth_time_set(t1: &mut PthTime, t2: Option<&PthTime>) -> io::Result<()> {
    match t2 {
        None => {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
            t1.tv_sec = now.as_secs() as i64;
            t1.tv_usec = now.subsec_micros() as i64;
        }
        Some(t2) => {
            t1.tv_sec = t2.tv_sec;
            t1.tv_usec = t2.tv_usec;
        }
    }
    Ok(())
}

/// Time value constructor
pub fn pth_time(sec: i64, usec: i64) -> PthTime {
    PthTime { tv_sec: sec, tv_usec: usec }
}

/// Timeout value constructor
pub fn pth_timeout(sec: i64, usec: i64) -> io::Result<PthTime> {
    let mut tv = PthTime { tv_sec: 0, tv_usec: 0 };
    pth_time_set(&mut tv, None)?;
    let tvd = PthTime { tv_sec: sec, tv_usec: usec };
    pth_time_add(&mut tv, &tvd);
    Ok(tv)
}

/// Compare two time values
pub fn pth_time_cmp(t1: &PthTime, t2: &PthTime) -> i64 {
    let rc = t1.tv_sec - t2.tv_sec;
    if rc == 0 {
        t1.tv_usec - t2.tv_usec
    } else {
        rc
    }
}

/// Add t2 to t1
pub fn pth_time_add(t1: &mut PthTime, t2: &PthTime) {
    t1.tv_sec += t2.tv_sec;
    t1.tv_usec += t2.tv_usec;
    if t1.tv_usec > 1_000_000 {
        t1.tv_sec += 1;
        t1.tv_usec -= 1_000_000;
    }
}

/// Subtract t2 from t1
pub fn pth_time_sub(t1: &mut PthTime, t2: &PthTime) {
    t1.tv_sec -= t2.tv_sec;
    t1.tv_usec -= t2.tv_usec;
    if t1.tv_usec < 0 {
        t1.tv_sec -= 1;
        t1.tv_usec += 1_000_000;
    }
}

/// Divide time by n
pub fn pth_time_div(t1: &mut PthTime, n: i32) {
    let n = n as i64;
    let q = t1.tv_sec / n;
    let r = (((t1.tv_sec % n) * 1_000_000) / n) + (t1.tv_usec / n);
    let (q, r) = if r > 1_000_000 {
        (q + 1, r - 1_000_000)
    } else {
        (q, r)
    };
    t1.tv_sec = q;
    t1.tv_usec = r;
}

/// Multiply time by n
pub fn pth_time_mul(t1: &mut PthTime, n: i32) {
    let n = n as i64;
    t1.tv_sec *= n;
    t1.tv_usec *= n;
    t1.tv_sec += t1.tv_usec / 1_000_000;
    t1.tv_usec %= 1_000_000;
}

/// Convert time to double value (seconds)
pub fn pth_time_t2d(t: &PthTime) -> f64 {
    (t.tv_sec as f64 * 1_000_000.0 + t.tv_usec as f64) / 1_000_000.0
}

/// Convert time to integer value (seconds)
pub fn pth_time_t2i(t: &PthTime) -> i64 {
    (t.tv_sec * 1_000_000 + t.tv_usec) / 1_000_000
}

/// Check if time is positive
pub fn pth_time_pos(t: &PthTime) -> bool {
    t.tv_sec > 0 && t.tv_usec > 0
}