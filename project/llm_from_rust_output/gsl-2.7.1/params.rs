use std::ops::Deref;

pub type SizeT = usize;

#[derive(Clone)]
pub struct GslHistogram {
    n: SizeT,
    range: Vec<f64>,
    bin: Vec<f64>,
}

impl GslHistogram {
    pub fn max(&self) -> f64 {
        self.range[self.n]
    }

    pub fn min(&self) -> f64 {
        self.range[0]
    }

    pub fn bins(&self) -> SizeT {
        self.n
    }
}

impl Deref for GslHistogram {
    type Target = [f64];

    fn deref(&self) -> &[f64] {
        &self.bin
    }
}