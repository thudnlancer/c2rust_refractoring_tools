use std::ops::{Deref, DerefMut};

pub type SizeT = usize;

#[derive(Clone)]
pub struct GslHistogram2d {
    nx: SizeT,
    ny: SizeT,
    xrange: Vec<f64>,
    yrange: Vec<f64>,
    bin: Vec<f64>,
}

impl GslHistogram2d {
    pub fn reset(&mut self) {
        self.bin.iter_mut().for_each(|x| *x = 0.0);
    }
}

impl Deref for GslHistogram2d {
    type Target = [f64];
    fn deref(&self) -> &Self::Target {
        &self.bin
    }
}

impl DerefMut for GslHistogram2d {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.bin
    }
}