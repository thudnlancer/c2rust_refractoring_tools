use num_traits::Zero;
use std::cmp::max;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CblasOrder {
    RowMajor = 101,
    ColMajor = 102,
}

#[derive(Debug)]
pub struct Complex32 {
    pub real: f32,
    pub imag: f32,
}

impl Complex32 {
    pub fn new(real: f32, imag: f32) -> Self {
        Self { real, imag }
    }
}

impl Zero for Complex32 {
    fn zero() -> Self {
        Self::new(0.0, 0.0)
    }

    fn is_zero(&self) -> bool {
        self.real == 0.0 && self.imag == 0.0
    }
}

pub fn cblas_cgeru(
    order: CblasOrder,
    m: i32,
    n: i32,
    alpha: Complex32,
    x: &[Complex32],
    inc_x: i32,
    y: &[Complex32],
    inc_y: i32,
    a: &mut [Complex32],
    lda: i32,
) {
    if order != CblasOrder::RowMajor && order != CblasOrder::ColMajor {
        panic!("Invalid order parameter");
    }
    if m < 0 {
        panic!("Invalid m parameter");
    }
    if n < 0 {
        panic!("Invalid n parameter");
    }
    if inc_x == 0 {
        panic!("Invalid inc_x parameter");
    }
    if inc_y == 0 {
        panic!("Invalid inc_y parameter");
    }

    let min_lda = match order {
        CblasOrder::RowMajor => max(1, n),
        CblasOrder::ColMajor => max(1, m),
    };
    if lda < min_lda {
        panic!("Invalid lda parameter");
    }

    if order == CblasOrder::RowMajor {
        let mut ix = if inc_x > 0 { 0 } else { (m - 1) * -inc_x };
        for i in 0..m {
            let x_val = &x[(ix as usize)];
            let tmp = Complex32::new(
                alpha.real * x_val.real - alpha.imag * x_val.imag,
                alpha.imag * x_val.real + alpha.real * x_val.imag,
            );

            let mut jy = if inc_y > 0 { 0 } else { (n - 1) * -inc_y };
            for j in 0..n {
                let y_val = &y[(jy as usize)];
                let a_idx = (lda * i + j) as usize;
                a[a_idx].real += y_val.real * tmp.real - y_val.imag * tmp.imag;
                a[a_idx].imag += y_val.imag * tmp.real + y_val.real * tmp.imag;
                jy += inc_y;
            }
            ix += inc_x;
        }
    } else {
        let mut jy = if inc_y > 0 { 0 } else { (n - 1) * -inc_y };
        for j in 0..n {
            let y_val = &y[(jy as usize)];
            let tmp = Complex32::new(
                alpha.real * y_val.real - alpha.imag * y_val.imag,
                alpha.imag * y_val.real + alpha.real * y_val.imag,
            );

            let mut ix = if inc_x > 0 { 0 } else { (m - 1) * -inc_x };
            for i in 0..m {
                let x_val = &x[(ix as usize)];
                let a_idx = (i + lda * j) as usize;
                a[a_idx].real += x_val.real * tmp.real - x_val.imag * tmp.imag;
                a[a_idx].imag += x_val.imag * tmp.real + x_val.real * tmp.imag;
                ix += inc_x;
            }
            jy += inc_y;
        }
    }
}