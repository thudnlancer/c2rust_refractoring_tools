//! Portable timer implementation in Rust.

use std::time::{SystemTime, UNIX_EPOCH, Duration};
use std::sync::Once;
use std::fmt;

#[cfg(windows)]
use winapi::um::profileapi::{QueryPerformanceCounter, QueryPerformanceFrequency};
#[cfg(windows)]
use winapi::um::sysinfoapi::GetTickCount;

/// Timer implementation that uses the most appropriate timing mechanism
/// for the current platform.
pub struct PTimer {
    start: SystemTime,
    elapsed_last: f64,
    elapsed_pre_start: f64,
}

impl PTimer {
    /// Create a new timer and reset it.
    pub fn new() -> Self {
        static INIT: Once = Once::new();
        
        #[cfg(any(windows, target_os = "cygwin"))]
        INIT.call_once(|| {
            // Windows initialization if needed
        });
        
        #[cfg(all(unix, not(target_os = "cygwin")))]
        INIT.call_once(|| {
            // POSIX initialization if needed
        });

        let mut timer = PTimer {
            start: SystemTime::now(),
            elapsed_last: 0.0,
            elapsed_pre_start: 0.0,
        };
        timer.reset();
        timer
    }

    /// Reset the timer to zero.
    pub fn reset(&mut self) {
        self.start = SystemTime::now();
        self.elapsed_last = 0.0;
        self.elapsed_pre_start = 0.0;
    }

    /// Measure the elapsed time since creation or last reset.
    pub fn measure(&mut self) -> f64 {
        let now = SystemTime::now();
        let elapsed = self.elapsed_pre_start + duration_to_secs(now.duration_since(self.start).unwrap_or(Duration::from_secs(0)));

        // Handle clock skew
        if elapsed < self.elapsed_last {
            self.start = now;
            self.elapsed_pre_start = self.elapsed_last;
            self.elapsed_last
        } else {
            self.elapsed_last = elapsed;
            elapsed
        }
    }

    /// Read the last measured elapsed time.
    pub fn read(&self) -> f64 {
        self.elapsed_last
    }

    /// Get the timer resolution in seconds.
    pub fn resolution() -> f64 {
        // Typical resolution for SystemTime on modern systems
        1e-6
    }
}

impl Drop for PTimer {
    fn drop(&mut self) {
        // No special cleanup needed
    }
}

fn duration_to_secs(d: Duration) -> Option<f64> {
    Some(d.as_secs() as f64 + (d.subsec_nanos() as f64 / 1e9))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_timer() {
        let mut timer = PTimer::new();
        thread::sleep(Duration::from_millis(100));
        let elapsed = timer.measure();
        assert!(elapsed >= 0.1 && elapsed < 0.2);
        
        timer.reset();
        thread::sleep(Duration::from_millis(50));
        let elapsed = timer.measure();
        assert!(elapsed >= 0.05 && elapsed < 0.15);
        
        assert_eq!(timer.read(), elapsed);
    }

    #[test]
    fn test_resolution() {
        let res = PTimer::resolution();
        assert!(res > 0.0 && res < 0.01);
    }
}