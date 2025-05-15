use std::time::{SystemTime, UNIX_EPOCH};
use std::io;

#[derive(Copy, Clone, Debug)]
pub struct Timespec {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}

impl Timespec {
    pub fn now() -> io::Result<Self> {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| {
                Timespec {
                    tv_sec: duration.as_secs() as i64,
                    tv_nsec: duration.subsec_nanos() as i64,
                }
            })
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timespec() {
        let ts = Timespec::now().unwrap();
        assert!(ts.tv_sec > 0);
        assert!(ts.tv_nsec >= 0);
    }
}