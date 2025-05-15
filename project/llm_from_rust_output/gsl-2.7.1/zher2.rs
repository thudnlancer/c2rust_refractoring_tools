use num_complex::Complex64;

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

pub fn cblas_zher2(
    order: CBLAS_ORDER,
    uplo: CBLAS_UPLO,
    n: i32,
    alpha: Complex64,
    x: &[Complex64],
    inc_x: i32,
    y: &[Complex64],
    inc_y: i32,
    a: &mut [Complex64],
    lda: i32,
) {
    // Parameter validation
    if n < 0 {
        panic!("n must be non-negative");
    }
    if inc_x == 0 {
        panic!("inc_x must not be zero");
    }
    if inc_y == 0 {
        panic!("inc_y must not be zero");
    }
    if lda < 1.max(n) {
        panic!("lda must be at least max(1, n)");
    }

    if alpha == Complex64::new(0.0, 0.0) {
        return;
    }

    let conj = if order == CBLAS_ORDER::ColMajor { -1.0 } else { 1.0 };

    match (order, uplo) {
        (CBLAS_ORDER::RowMajor, CBLAS_UPLO::Upper) | (CBLAS_ORDER::ColMajor, CBLAS_UPLO::Lower) => {
            let mut ix = if inc_x > 0 { 0 } else { (n - 1) * -inc_x };
            let mut iy = if inc_y > 0 { 0 } else { (n - 1) * -inc_y };

            for i in 0..n {
                let xi = x[(ix / inc_x) as usize];
                let tmp1 = alpha * xi;
                let yi = y[(iy / inc_y) as usize];
                let tmp2 = alpha.conj() * yi;

                let mut jx = ix + inc_x;
                let mut jy = iy + inc_y;

                a[(i * lda + i) as usize].re += 2.0 * (tmp1 * yi.conj()).re;
                a[(i * lda + i) as usize].im = 0.0;

                for j in (i + 1)..n {
                    let xj = x[(jx / inc_x) as usize];
                    let yj = y[(jy / inc_y) as usize];

                    let a_ij = &mut a[(i * lda + j) as usize];
                    *a_ij += tmp1 * yj.conj() + tmp2 * xj.conj();
                    a_ij.im += conj * (tmp1 * yj.conj() + tmp2 * xj.conj()).im;

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

            for i in 0..n {
                let xi = x[(ix / inc_x) as usize];
                let tmp1 = alpha * xi;
                let yi = y[(iy / inc_y) as usize];
                let tmp2 = alpha.conj() * yi;

                let mut jx = if inc_x > 0 { 0 } else { (n - 1) * -inc_x };
                let mut jy = if inc_y > 0 { 0 } else { (n - 1) * -inc_y };

                for j in 0..i {
                    let xj = x[(jx / inc_x) as usize];
                    let yj = y[(jy / inc_y) as usize];

                    let a_ij = &mut a[(i * lda + j) as usize];
                    *a_ij += tmp1 * yj.conj() + tmp2 * xj.conj();
                    a_ij.im += conj * (tmp1 * yj.conj() + tmp2 * xj.conj()).im;

                    jx += inc_x;
                    jy += inc_y;
                }

                a[(i * lda + i) as usize].re += 2.0 * (tmp1 * yi.conj()).re;
                a[(i * lda + i) as usize].im = 0.0;

                ix += inc_x;
                iy += inc_y;
            }
        }
        _ => panic!("unrecognized operation"),
    }
}