use std::slice;

#[derive(Debug, Clone)]
pub struct GslHistogram {
    n: usize,
    range: Vec<f64>,
    bin: Vec<f64>,
}

impl GslHistogram {
    pub fn max_val(&self) -> f64 {
        self.bin.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b))
    }

    pub fn max_bin(&self) -> usize {
        self.bin.iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .map(|(idx, _)| idx)
            .unwrap_or(0)
    }

    pub fn min_val(&self) -> f64 {
        self.bin.iter().fold(f64::INFINITY, |a, &b| a.min(b))
    }

    pub fn min_bin(&self) -> usize {
        self.bin.iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .map(|(idx, _)| idx)
            .unwrap_or(0)
    }
}

#[no_mangle]
pub extern "C" fn gsl_histogram_max_val(h: &GslHistogram) -> f64 {
    h.max_val()
}

#[no_mangle]
pub extern "C" fn gsl_histogram_max_bin(h: &GslHistogram) -> usize {
    h.max_bin()
}

#[no_mangle]
pub extern "C" fn gsl_histogram_min_val(h: &GslHistogram) -> f64 {
    h.min_val()
}

#[no_mangle]
pub extern "C" fn gsl_histogram_min_bin(h: &GslHistogram) -> usize {
    h.min_bin()
}