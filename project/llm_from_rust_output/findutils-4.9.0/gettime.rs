use std::time::{SystemTime, UNIX_EPOCH};
use std::mem::MaybeUninit;

#[derive(Copy, Clone, Debug)]
pub struct Timespec {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}

impl Timespec {
    pub fn now() -> Result<Self, std::time::SystemTimeError> {
        let duration = SystemTime::now().duration_since(UNIX_EPOCH)?;
        Ok(Timespec {
            tv_sec: duration.as_secs() as i64,
            tv_nsec: duration.subsec_nanos() as i64,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timespec_now() {
        let ts = Timespec::now().unwrap();
        assert!(ts.tv_sec > 0);
        assert!(ts.tv_nsec >= 0);
    }
}