use num_complex::Complex32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CBLAS_ORDER {
    RowMajor = 101,
    ColMajor = 102,
}

#[no_mangle]
pub fn cblas_cgerc(
    order: CBLAS_ORDER,
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
    // Parameter validation
    if m < 0 {
        panic!("M must be >= 0");
    }
    if n < 0 {
        panic!("N must be >= 0");
    }
    if inc_x == 0 {
        panic!("incX must not be zero");
    }
    if inc_y == 0 {
        panic!("incY must not be zero");
    }

    let min_lda = match order {
        CBLAS_ORDER::RowMajor => n.max(1),
        CBLAS_ORDER::ColMajor => m.max(1),
    };
    if lda < min_lda {
        panic!("lda must be >= {}", min_lda);
    }

    // Check buffer sizes
    let x_len = if m == 0 {
        0
    } else {
        1 + (m - 1) * inc_x.abs()
    } as usize;
    let y_len = if n == 0 {
        0
    } else {
        1 + (n - 1) * inc_y.abs()
    } as usize;
    let a_len = match order {
        CBLAS_ORDER::RowMajor => (lda * m) as usize,
        CBLAS_ORDER::ColMajor => (lda * n) as usize,
    };

    if x.len() < x_len {
        panic!("X buffer too small");
    }
    if y.len() < y_len {
        panic!("Y buffer too small");
    }
    if a.len() < a_len {
        panic!("A buffer too small");
    }

    match order {
        CBLAS_ORDER::RowMajor => {
            let mut ix = if inc_x > 0 { 0 } else { (m - 1) * -inc_x };
            for i in 0..m {
                let x_val = x[ix as usize];
                let tmp = alpha * x_val;

                let mut jy = if inc_y > 0 { 0 } else { (n - 1) * -inc_y };
                for j in 0..n {
                    let y_val = y[jy as usize].conj();
                    let idx = (lda * i + j) as usize;
                    a[idx] += y_val * tmp;

                    jy += inc_y;
                }
                ix += inc_x;
            }
        }
        CBLAS_ORDER::ColMajor => {
            let mut jy = if inc_y > 0 { 0 } else { (n - 1) * -inc_y };
            for j in 0..n {
                let y_val = y[jy as usize].conj();
                let tmp = alpha * y_val;

                let mut ix = if inc_x > 0 { 0 } else { (m - 1) * -inc_x };
                for i in 0..m {
                    let x_val = x[ix as usize];
                    let idx = (i + lda * j) as usize;
                    a[idx] += x_val * tmp;

                    ix += inc_x;
                }
                jy += inc_y;
            }
        }
    }
}