use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub enum GlpTimeError {
    TimeError(std::time::SystemTimeError),
    InvalidTimeRange,
}

pub fn glp_time() -> Result<f64, GlpTimeError> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(GlpTimeError::TimeError)?;
    
    let t = now.as_secs() as f64 + now.subsec_micros() as f64 / 1e6;
    
    if !(0.0 <= t && t < 4294967296.0) {
        return Err(GlpTimeError::InvalidTimeRange);
    }
    
    Ok(1000.0 * t)
}

pub fn glp_difftime(t1: f64, t0: f64) -> f64 {
    (t1 - t0) / 1000.0
}