use std::ops::Range;

pub struct GslHistogram {
    n: usize,
    range: Vec<f64>,
    bin: Vec<f64>,
}

impl GslHistogram {
    pub fn reset(&mut self) {
        self.bin.iter_mut().for_each(|x| *x = 0.0);
    }
}