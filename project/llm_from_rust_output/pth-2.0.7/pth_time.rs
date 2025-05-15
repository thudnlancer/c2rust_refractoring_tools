use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Timeval {
    pub tv_sec: i64,
    pub tv_usec: i64,
}

impl Timeval {
    pub fn zero() -> Self {
        Timeval {
            tv_sec: 0,
            tv_usec: 0,
        }
    }

    pub fn new(sec: i64, usec: i64) -> Self {
        Timeval { tv_sec: sec, tv_usec: usec }
    }

    pub fn from_system_time(time: SystemTime) -> Self {
        match time.duration_since(UNIX_EPOCH) {
            Ok(duration) => Timeval {
                tv_sec: duration.as_secs() as i64,
                tv_usec: duration.subsec_micros() as i64,
            },
            Err(_) => Timeval::zero(),
        }
    }

    pub fn now() -> Self {
        Self::from_system_time(SystemTime::now())
    }

    pub fn add(&self, other: &Timeval) -> Timeval {
        let mut sec = self.tv_sec + other.tv_sec;
        let mut usec = self.tv_usec + other.tv_usec;

        if usec > 1_000_000 {
            sec += 1;
            usec -= 1_000_000;
        }

        Timeval { tv_sec: sec, tv_usec: usec }
    }

    pub fn cmp(&self, other: &Timeval) -> std::cmp::Ordering {
        self.tv_sec.cmp(&other.tv_sec)
            .then(self.tv_usec.cmp(&other.tv_usec))
    }

    pub fn div(&mut self, n: i32) {
        let n = n as i64;
        let q = self.tv_sec / n;
        let r = (self.tv_sec % n * 1_000_000 / n) + (self.tv_usec / n);

        let (q, r) = if r > 1_000_000 {
            (q + 1, r - 1_000_000)
        } else {
            (q, r)
        };

        self.tv_sec = q;
        self.tv_usec = r;
    }

    pub fn mul(&mut self, n: i32) {
        let n = n as i64;
        self.tv_sec *= n;
        self.tv_usec *= n;

        self.tv_sec += self.tv_usec / 1_000_000;
        self.tv_usec %= 1_000_000;
    }

    pub fn to_seconds_f64(&self) -> f64 {
        (self.tv_sec as f64 * 1_000_000.0 + self.tv_usec as f64) / 1_000_000.0
    }

    pub fn to_seconds_i32(&self) -> i32 {
        ((self.tv_sec * 1_000_000 + self.tv_usec) / 1_000_000) as i32
    }

    pub fn is_positive(&self) -> bool {
        self.tv_sec > 0 && self.tv_usec > 0
    }
}

pub fn usleep(usec: u64) {
    std::thread::sleep(Duration::from_micros(usec));
}

pub fn pth_time(sec: i64, usec: i64) -> Timeval {
    Timeval::new(sec, usec)
}

pub fn pth_timeout(sec: i64, usec: i64) -> Timeval {
    let current = Timeval::now();
    let duration = Timeval::new(sec, usec);
    current.add(&duration)
}