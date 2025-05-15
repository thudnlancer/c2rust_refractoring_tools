use std::ops::{Index, IndexMut};

pub type SizeT = usize;

#[derive(Clone)]
pub struct GslHistogram2D {
    nx: SizeT,
    ny: SizeT,
    xrange: Vec<f64>,
    yrange: Vec<f64>,
    bin: Vec<f64>,
}

impl GslHistogram2D {
    pub fn max_val(&self) -> f64 {
        self.bin.iter().fold(f64::NEG_INFINITY, |max, &val| max.max(val))
    }

    pub fn min_val(&self) -> f64 {
        self.bin.iter().fold(f64::INFINITY, |min, &val| min.min(val))
    }

    pub fn max_bin(&self) -> (SizeT, SizeT) {
        let mut max = f64::NEG_INFINITY;
        let mut imax = 0;
        let mut jmax = 0;

        for i in 0..self.nx {
            for j in 0..self.ny {
                let val = self[(i, j)];
                if val > max {
                    max = val;
                    imax = i;
                    jmax = j;
                }
            }
        }

        (imax, jmax)
    }

    pub fn min_bin(&self) -> (SizeT, SizeT) {
        let mut min = f64::INFINITY;
        let mut imin = 0;
        let mut jmin = 0;

        for i in 0..self.nx {
            for j in 0..self.ny {
                let val = self[(i, j)];
                if val < min {
                    min = val;
                    imin = i;
                    jmin = j;
                }
            }
        }

        (imin, jmin)
    }
}

impl Index<(SizeT, SizeT)> for GslHistogram2D {
    type Output = f64;

    fn index(&self, (i, j): (SizeT, SizeT)) -> &f64 {
        &self.bin[i * self.ny + j]
    }
}

impl IndexMut<(SizeT, SizeT)> for GslHistogram2D {
    fn index_mut(&mut self, (i, j): (SizeT, SizeT)) -> &mut f64 {
        &mut self.bin[i * self.ny + j]
    }
}