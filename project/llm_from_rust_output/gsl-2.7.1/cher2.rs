use std::ffi::CString;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CBLAS_ORDER {
    RowMajor = 101,
    ColMajor = 102,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CBLAS_UPLO {
    Upper = 121,
    Lower = 122,
}

#[derive(Debug, Clone, Copy)]
pub struct Complex32 {
    pub real: f32,
    pub imag: f32,
}

impl Complex32 {
    pub fn new(real: f32, imag: f32) -> Self {
        Self { real, imag }
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0)
    }
}

pub fn cblas_cher2(
    order: CBLAS_ORDER,
    uplo: CBLAS_UPLO,
    n: i32,
    alpha: Complex32,
    x: &[Complex32],
    inc_x: i32,
    y: &[Complex32],
    inc_y: i32,
    a: &mut [Complex32],
    lda: i32,
) {
    // Parameter validation
    if n < 0 {
        panic!("Invalid n: {}", n);
    }
    if inc_x == 0 {
        panic!("Invalid inc_x: {}", inc_x);
    }
    if inc_y == 0 {
        panic!("Invalid inc_y: {}", inc_y);
    }
    if lda < 1.max(n) {
        panic!("Invalid lda: {}", lda);
    }

    if alpha.real == 0.0 && alpha.imag == 0.0 {
        return;
    }

    let conj = if order == CBLAS_ORDER::ColMajor { -1 } else { 1 };

    match (order, uplo) {
        (CBLAS_ORDER::RowMajor, CBLAS_UPLO::Upper) |
        (CBLAS_ORDER::ColMajor, CBLAS_UPLO::Lower) => {
            let mut ix = if inc_x > 0 { 0 } else { (n - 1) * -inc_x };
            let mut iy = if inc_y > 0 { 0 } else { (n - 1) * -inc_y };

            for i in 0..n {
                let xi = x[(ix * 2) as usize];
                let tmp1 = Complex32::new(
                    alpha.real * xi.real - alpha.imag * xi.imag,
                    alpha.imag * xi.real + alpha.real * xi.imag,
                );

                let yi = y[(iy * 2) as usize];
                let tmp2 = Complex32::new(
                    alpha.real * yi.real + alpha.imag * yi.imag,
                    -alpha.imag * yi.real + alpha.real * yi.imag,
                );

                let mut jx = ix + inc_x;
                let mut jy = iy + inc_y;

                let diag_index = (lda * i + i) as usize;
                a[diag_index * 2].real += 2.0 * (tmp1.real * yi.real + tmp1.imag * yi.imag);
                a[diag_index * 2 + 1] = Complex32::zero();

                for j in (i + 1)..n {
                    let xj = x[(jx * 2) as usize];
                    let yj = y[(jy * 2) as usize];

                    let index = (lda * i + j) as usize;
                    a[index * 2].real += tmp1.real * yj.real + tmp1.imag * yj.imag
                        + (tmp2.real * xj.real + tmp2.imag * xj.imag);
                    a[index * 2 + 1].real += conj as f32 * (
                        tmp1.imag * yj.real - tmp1.real * yj.imag
                        + (tmp2.imag * xj.real - tmp2.real * xj.imag)
                    );

                    jx += inc_x;
                    jy += inc_y;
                }

                ix += inc_x;
                iy += inc_y;
            }
        }

        (CBLAS_ORDER::RowMajor, CBLAS_UPLO::Lower) |
        (CBLAS_ORDER::ColMajor, CBLAS_UPLO::Upper) => {
            let mut ix = if inc_x > 0 { 0 } else { (n - 1) * -inc_x };
            let mut iy = if inc_y > 0 { 0 } else { (n - 1) * -inc_y };

            for i in 0..n {
                let xi = x[(ix * 2) as usize];
                let tmp1 = Complex32::new(
                    alpha.real * xi.real - alpha.imag * xi.imag,
                    alpha.imag * xi.real + alpha.real * xi.imag,
                );

                let yi = y[(iy * 2) as usize];
                let tmp2 = Complex32::new(
                    alpha.real * yi.real + alpha.imag * yi.imag,
                    -alpha.imag * yi.real + alpha.real * yi.imag,
                );

                let mut jx = if inc_x > 0 { 0 } else { (n - 1) * -inc_x };
                let mut jy = if inc_y > 0 { 0 } else { (n - 1) * -inc_y };

                for j in 0..i {
                    let xj = x[(jx * 2) as usize];
                    let yj = y[(jy * 2) as usize];

                    let index = (lda * i + j) as usize;
                    a[index * 2].real += tmp1.real * yj.real + tmp1.imag * yj.imag
                        + (tmp2.real * xj.real + tmp2.imag * xj.imag);
                    a[index * 2 + 1].real += conj as f32 * (
                        tmp1.imag * yj.real - tmp1.real * yj.imag
                        + (tmp2.imag * xj.real - tmp2.real * xj.imag)
                    );

                    jx += inc_x;
                    jy += inc_y;
                }

                let diag_index = (lda * i + i) as usize;
                a[diag_index * 2].real += 2.0 * (tmp1.real * yi.real + tmp1.imag * yi.imag);
                a[diag_index * 2 + 1] = Complex32::zero();

                ix += inc_x;
                iy += inc_y;
            }
        }

        _ => panic!("Unrecognized operation: order={:?}, uplo={:?}", order, uplo),
    }
}