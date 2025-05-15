//! Time handling functionality for GLPK (GNU Linear Programming Kit).
//! Ported from C to Rust with safety and idiomatic practices in mind.

use std::time::{SystemTime, UNIX_EPOCH, Duration};
use std::convert::TryInto;

const EPOCH: i32 = 2440588; // = jday(1, 1, 1970)

/// Determines current universal time (UTC) in milliseconds since Unix epoch
pub fn glp_time() -> f64 {
    #[cfg(unix)]
    {
        match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(duration) => {
                let secs = duration.as_secs() as f64;
                let micros = duration.subsec_micros() as f64;
                (secs * 1000.0) + (micros / 1000.0)
            }
            Err(_) => panic!("SystemTime before UNIX EPOCH!"),
        }
    }

    #[cfg(windows)]
    {
        use winapi::um::sysinfoapi::GetSystemTime;
        use winapi::um::minwinbase::SYSTEMTIME;

        unsafe {
            let mut st: SYSTEMTIME = std::mem::zeroed();
            GetSystemTime(&mut st);

            let j = jday(st.wDay as i32, st.wMonth as i32, st.wYear as i32);
            assert!(j >= 0);

            ((((f64::from(j - EPOCH) * 24.0 + f64::from(st.wHour)) * 60.0
                + f64::from(st.wMinute)) * 60.0
                + f64::from(st.wSecond)) * 1000.0
                + f64::from(st.wMilliseconds)
        }
    }

    #[cfg(not(any(unix, windows)))]
    {
        let now = SystemTime::now();
        let duration = now.duration_since(UNIX_EPOCH)
            .expect("SystemTime before UNIX EPOCH!");
        
        let secs = duration.as_secs() as f64;
        secs * 1000.0
    }
}

/// Computes difference between two time values in seconds
pub fn glp_difftime(t1: f64, t0: f64) -> f64 {
    (t1 - t0) / 1000.0
}

/// Helper function to calculate Julian day
fn jday(day: i32, month: i32, year: i32) -> i32 {
    // Implementation of jday function would go here
    // This is a placeholder - actual implementation would need to be ported
    // from the original C code or use an existing Julian day calculation crate
    0
}