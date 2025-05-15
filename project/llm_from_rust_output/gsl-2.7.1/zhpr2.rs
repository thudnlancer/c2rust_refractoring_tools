use num_traits::Zero;
use std::f64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CBLAS_ORDER {
    RowMajor = 101,
    ColMajor = 102,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CBLAS_UPLO {
    Upper = 121,
    Lower = 122,
}

#[derive(Debug)]
pub struct Complex {
    pub real: f64,
    pub imag: f64,
}

impl Complex {
    pub fn new(real: f64, imag: f64) -> Self {
        Self { real, imag }
    }

    pub fn is_zero(&self) -> bool {
        self.real.is_zero() && self.imag.is_zero()
    }
}

impl Zero for Complex {
    fn zero() -> Self {
        Self::new(0.0, 0.0)
    }

    fn is_zero(&self) -> bool {
        Complex::is_zero(self)
    }
}

pub fn cblas_zhpr2(
    order: CBLAS_ORDER,
    uplo: CBLAS_UPLO,
    n: i32,
    alpha: Complex,
    x: &[Complex],
    inc_x: i32,
    y: &[Complex],
    inc_y: i32,
    ap: &mut [Complex],
) -> Result<(), &'static str> {
    if n < 0 {
        return Err("Invalid n");
    }
    if inc_x == 0 {
        return Err("Invalid inc_x");
    }
    if inc_y == 0 {
        return Err("Invalid inc_y");
    }

    let conj = if order == CBLAS_ORDER::ColMajor { -1 } else { 1 };

    if alpha.is_zero() {
        return Ok(());
    }

    match (order, uplo) {
        (CBLAS_ORDER::RowMajor, CBLAS_UPLO::Upper) | (CBLAS_ORDER::ColMajor, CBLAS_UPLO::Lower) => {
            let mut ix = if inc_x > 0 { 0 } else { (n - 1) * -inc_x };
            let mut iy = if inc_y > 0 { 0 } else { (n - 1) * -inc_y };

            for i in 0..n as usize {
                let xi = &x[ix as usize];
                let tmp1 = Complex::new(
                    alpha.real * xi.real - alpha.imag * xi.imag,
                    alpha.imag * xi.real + alpha.real * xi.imag,
                );

                let yi = &y[iy as usize];
                let tmp2 = Complex::new(
                    alpha.real * yi.real + alpha.imag * yi.imag,
                    -alpha.imag * yi.real + alpha.real * yi.imag,
                );

                let mut jx = ix + inc_x;
                let mut jy = iy + inc_y;

                let diag_index = (i * (2 * n as usize - i + 1)) / 2;
                ap[diag_index].real += 2.0 * (tmp1.real * yi.real + tmp1.imag * yi.imag);
                ap[diag_index].imag = 0.0;

                for j in (i + 1)..n as usize {
                    let xj = &x[jx as usize];
                    let yj = &y[jy as usize];

                    let index = diag_index + j - i;
                    ap[index].real += tmp1.real * yj.real + tmp1.imag * yj.imag
                        + (tmp2.real * xj.real + tmp2.imag * xj.imag);
                    ap[index].imag += conj as f64
                        * (tmp1.imag * yj.real - tmp1.real * yj.imag
                            + (tmp2.imag * xj.real - tmp2.real * xj.imag));

                    jx += inc_x;
                    jy += inc_y;
                }

                ix += inc_x;
                iy += inc_y;
            }
        }
        (CBLAS_ORDER::RowMajor, CBLAS_UPLO::Lower) | (CBLAS_ORDER::ColMajor, CBLAS_UPLO::Upper) => {
            let mut ix = if inc_x > 0 { 0 } else { (n - 1) * -inc_x };
            let mut iy = if inc_y > 0 { 0 } else { (n - 1) * -inc_y };

            for i in 0..n as usize {
                let xi = &x[ix as usize];
                let tmp1 = Complex::new(
                    alpha.real * xi.real - alpha.imag * xi.imag,
                    alpha.imag * xi.real + alpha.real * xi.imag,
                );

                let yi = &y[iy as usize];
                let tmp2 = Complex::new(
                    alpha.real * yi.real + alpha.imag * yi.imag,
                    -alpha.imag * yi.real + alpha.real * yi.imag,
                );

                let mut jx = if inc_x > 0 { 0 } else { (n - 1) * -inc_x };
                let mut jy = if inc_y > 0 { 0 } else { (n - 1) * -inc_y };

                for j in 0..i {
                    let xj = &x[jx as usize];
                    let yj = &y[jy as usize];

                    let index = i * (i + 1) / 2 + j;
                    ap[index].real += tmp1.real * yj.real + tmp1.imag * yj.imag
                        + (tmp2.real * xj.real + tmp2.imag * xj.imag);
                    ap[index].imag += conj as f64
                        * (tmp1.imag * yj.real - tmp1.real * yj.imag
                            + (tmp2.imag * xj.real - tmp2.real * xj.imag));

                    jx += inc_x;
                    jy += inc_y;
                }

                let diag_index = i * (i + 1) / 2 + i;
                ap[diag_index].real += 2.0 * (tmp1.real * yi.real + tmp1.imag * yi.imag);
                ap[diag_index].imag = 0.0;

                ix += inc_x;
                iy += inc_y;
            }
        }
    }

    Ok(())
}