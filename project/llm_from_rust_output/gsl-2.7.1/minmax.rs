use num_traits::{Float, Num, NumCast, ToPrimitive};
use std::cmp::Ordering;

pub type size_t = usize;

#[derive(Debug, Clone)]
pub struct Matrix<T> {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: Vec<T>,
}

impl<T: Num + PartialOrd + Copy> Matrix<T> {
    pub fn max(&self) -> T {
        let mut max = self.data[0];
        for i in 0..self.size1 {
            for j in 0..self.size2 {
                let idx = i * self.tda + j;
                let x = self.data[idx];
                if x > max {
                    max = x;
                }
            }
        }
        max
    }

    pub fn min(&self) -> T {
        let mut min = self.data[0];
        for i in 0..self.size1 {
            for j in 0..self.size2 {
                let idx = i * self.tda + j;
                let x = self.data[idx];
                if x < min {
                    min = x;
                }
            }
        }
        min
    }

    pub fn minmax(&self) -> (T, T) {
        let mut min = self.data[0];
        let mut max = self.data[0];
        for i in 0..self.size1 {
            for j in 0..self.size2 {
                let idx = i * self.tda + j;
                let x = self.data[idx];
                if x < min {
                    min = x;
                }
                if x > max {
                    max = x;
                }
            }
        }
        (min, max)
    }

    pub fn max_index(&self) -> (size_t, size_t) {
        let mut max = self.data[0];
        let mut imax = 0;
        let mut jmax = 0;
        for i in 0..self.size1 {
            for j in 0..self.size2 {
                let idx = i * self.tda + j;
                let x = self.data[idx];
                if x > max {
                    max = x;
                    imax = i;
                    jmax = j;
                }
            }
        }
        (imax, jmax)
    }

    pub fn min_index(&self) -> (size_t, size_t) {
        let mut min = self.data[0];
        let mut imin = 0;
        let mut jmin = 0;
        for i in 0..self.size1 {
            for j in 0..self.size2 {
                let idx = i * self.tda + j;
                let x = self.data[idx];
                if x < min {
                    min = x;
                    imin = i;
                    jmin = j;
                }
            }
        }
        (imin, jmin)
    }

    pub fn minmax_index(&self) -> ((size_t, size_t), (size_t, size_t)) {
        let mut min = self.data[0];
        let mut max = self.data[0];
        let mut imin = 0;
        let mut jmin = 0;
        let mut imax = 0;
        let mut jmax = 0;
        for i in 0..self.size1 {
            for j in 0..self.size2 {
                let idx = i * self.tda + j;
                let x = self.data[idx];
                if x < min {
                    min = x;
                    imin = i;
                    jmin = j;
                }
                if x > max {
                    max = x;
                    imax = i;
                    jmax = j;
                }
            }
        }
        ((imin, jmin), (imax, jmax))
    }
}

impl<T: Float> Matrix<T> {
    pub fn max_with_nan(&self) -> T {
        let mut max = self.data[0];
        for i in 0..self.size1 {
            for j in 0..self.size2 {
                let idx = i * self.tda + j;
                let x = self.data[idx];
                if x.is_nan() {
                    return x;
                }
                if x > max {
                    max = x;
                }
            }
        }
        max
    }

    pub fn min_with_nan(&self) -> T {
        let mut min = self.data[0];
        for i in 0..self.size1 {
            for j in 0..self.size2 {
                let idx = i * self.tda + j;
                let x = self.data[idx];
                if x.is_nan() {
                    return x;
                }
                if x < min {
                    min = x;
                }
            }
        }
        min
    }

    pub fn minmax_with_nan(&self) -> (T, T) {
        let mut min = self.data[0];
        let mut max = self.data[0];
        for i in 0..self.size1 {
            for j in 0..self.size2 {
                let idx = i * self.tda + j;
                let x = self.data[idx];
                if x.is_nan() {
                    return (x, x);
                }
                if x < min {
                    min = x;
                }
                if x > max {
                    max = x;
                }
            }
        }
        (min, max)
    }

    pub fn max_index_with_nan(&self) -> (size_t, size_t) {
        let mut max = self.data[0];
        let mut imax = 0;
        let mut jmax = 0;
        for i in 0..self.size1 {
            for j in 0..self.size2 {
                let idx = i * self.tda + j;
                let x = self.data[idx];
                if x.is_nan() {
                    return (i, j);
                }
                if x > max {
                    max = x;
                    imax = i;
                    jmax = j;
                }
            }
        }
        (imax, jmax)
    }

    pub fn min_index_with_nan(&self) -> (size_t, size_t) {
        let mut min = self.data[0];
        let mut imin = 0;
        let mut jmin = 0;
        for i in 0..self.size1 {
            for j in 0..self.size2 {
                let idx = i * self.tda + j;
                let x = self.data[idx];
                if x.is_nan() {
                    return (i, j);
                }
                if x < min {
                    min = x;
                    imin = i;
                    jmin = j;
                }
            }
        }
        (imin, jmin)
    }

    pub fn minmax_index_with_nan(&self) -> ((size_t, size_t), (size_t, size_t)) {
        let mut min = self.data[0];
        let mut max = self.data[0];
        let mut imin = 0;
        let mut jmin = 0;
        let mut imax = 0;
        let mut jmax = 0;
        for i in 0..self.size1 {
            for j in 0..self.size2 {
                let idx = i * self.tda + j;
                let x = self.data[idx];
                if x.is_nan() {
                    return ((i, j), (i, j));
                }
                if x < min {
                    min = x;
                    imin = i;
                    jmin = j;
                }
                if x > max {
                    max = x;
                    imax = i;
                    jmax = j;
                }
            }
        }
        ((imin, jmin), (imax, jmax))
    }
}