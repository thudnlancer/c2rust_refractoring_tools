use std::time::Duration;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Timespec {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}

impl Timespec {
    pub fn new(s: i64, ns: i64) -> Self {
        Timespec { tv_sec: s, tv_nsec: ns }
    }

    pub fn cmp(&self, other: &Timespec) -> i32 {
        2 * (i32::from(self.tv_sec > other.tv_sec) - i32::from(self.tv_sec < other.tv_sec))
            + (i32::from(self.tv_nsec > other.tv_nsec) - i32::from(self.tv_nsec < other.tv_nsec))
    }

    pub fn sign(&self) -> i32 {
        (i32::from(self.tv_sec > 0) - i32::from(self.tv_sec < 0))
            + (i32::from(self.tv_sec == 0) & i32::from(self.tv_nsec != 0))
    }

    pub fn to_seconds(&self) -> f64 {
        self.tv_sec as f64 + (self.tv_nsec as f64) / 1_000_000_000.0
    }
}

impl From<Duration> for Timespec {
    fn from(d: Duration) -> Self {
        Timespec {
            tv_sec: d.as_secs() as i64,
            tv_nsec: d.subsec_nanos() as i64,
        }
    }
}

impl From<Timespec> for Duration {
    fn from(t: Timespec) -> Self {
        Duration::new(t.tv_sec as u64, t.tv_nsec as u32)
    }
}