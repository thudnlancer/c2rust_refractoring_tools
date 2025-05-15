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

fn cblas_xerbla(p: i32, rout: &str, form: &str) {
    eprintln!("Parameter {} to routine {} was incorrect: {}", p, rout, form);
}

pub fn cblas_zhpmv(
    order: CBLAS_ORDER,
    uplo: CBLAS_UPLO,
    n: i32,
    alpha: Complex64,
    ap: &[Complex64],
    x: &[Complex64],
    inc_x: i32,
    beta: Complex64,
    y: &mut [Complex64],
    inc_y: i32,
) {
    if n <= 0 {
        return;
    }

    if order != CBLAS_ORDER::RowMajor && order != CBLAS_ORDER::ColMajor {
        cblas_xerbla(1, "cblas_zhpmv", "invalid order");
        return;
    }

    if uplo != CBLAS_UPLO::Upper && uplo != CBLAS_UPLO::Lower {
        cblas_xerbla(2, "cblas_zhpmv", "invalid uplo");
        return;
    }

    if inc_x == 0 {
        cblas_xerbla(7, "cblas_zhpmv", "invalid inc_x");
        return;
    }

    if inc_y == 0 {
        cblas_xerbla(10, "cblas_zhpmv", "invalid inc_y");
        return;
    }

    let conj = if order == CBLAS_ORDER::ColMajor { -1.0 } else { 1.0 };

    if alpha == Complex64::new(0.0, 0.0) && beta == Complex64::new(1.0, 0.0) {
        return;
    }

    if beta == Complex64::new(0.0, 0.0) {
        let mut iy = if inc_y > 0 { 0 } else { (n - 1) * -inc_y } as usize;
        for _ in 0..n {
            y[iy] = Complex64::new(0.0, 0.0);
            iy = (iy as i32 + inc_y) as usize;
        }
    } else if beta != Complex64::new(1.0, 0.0) {
        let mut iy = if inc_y > 0 { 0 } else { (n - 1) * -inc_y } as usize;
        for _ in 0..n {
            y[iy] = y[iy] * beta;
            iy = (iy as i32 + inc_y) as usize;
        }
    }

    if alpha == Complex64::new(0.0, 0.0) {
        return;
    }

    let upper = uplo == CBLAS_UPLO::Upper;
    let row_major = order == CBLAS_ORDER::RowMajor;

    if (row_major && upper) || (!row_major && !upper) {
        let mut ix = if inc_x > 0 { 0 } else { (n - 1) * -inc_x } as usize;
        let mut iy = if inc_y > 0 { 0 } else { (n - 1) * -inc_y } as usize;
        for i in 0..n {
            let xi = x[ix];
            let temp1 = alpha * xi;
            let mut temp2 = Complex64::new(0.0, 0.0);
            
            let aii_index = (i * (2 * n - i + 1) / 2) as usize;
            let aii = Complex64::new(ap[aii_index].re, 0.0);
            y[iy] += temp1 * aii;

            let mut jx = (if inc_x > 0 { 0 } else { (n - 1) * -inc_x } + (i + 1) * inc_x) as usize;
            let mut jy = (if inc_y > 0 { 0 } else { (n - 1) * -inc_y } + (i + 1) * inc_y) as usize;
            
            for j in (i + 1)..n {
                let aij_index = (i * (2 * n - i + 1) / 2 + j - i) as usize;
                let aij = Complex64::new(ap[aij_index].re, conj * ap[aij_index].im);
                
                y[jy] += temp1 * aij.conj();
                
                let xj = x[jx];
                temp2 += xj * aij;
                
                jx = (jx as i32 + inc_x) as usize;
                jy = (jy as i32 + inc_y) as usize;
            }
            
            y[iy] += alpha * temp2;
            ix = (ix as i32 + inc_x) as usize;
            iy = (iy as i32 + inc_y) as usize;
        }
    } else if (row_major && !upper) || (!row_major && upper) {
        let mut ix = if inc_x > 0 { 0 } else { (n - 1) * -inc_x } as usize;
        let mut iy = if inc_y > 0 { 0 } else { (n - 1) * -inc_y } as usize;
        for i in 0..n {
            let xi = x[ix];
            let temp1 = alpha * xi;
            let mut temp2 = Complex64::new(0.0, 0.0);
            
            let aii_index = (i * (i + 1) / 2 + i) as usize;
            let aii = Complex64::new(ap[aii_index].re, 0.0);
            y[iy] += temp1 * aii;

            let mut jx = (if inc_x > 0 { 0 } else { (n - 1) * -inc_x }) as usize;
            let mut jy = (if inc_y > 0 { 0 } else { (n - 1) * -inc_y }) as usize;
            
            for j in 0..i {
                let aij_index = (i * (i + 1) / 2 + j) as usize;
                let aij = Complex64::new(ap[aij_index].re, conj * ap[aij_index].im);
                
                y[jy] += temp1 * aij.conj();
                
                let xj = x[jx];
                temp2 += xj * aij;
                
                jx = (jx as i32 + inc_x) as usize;
                jy = (jy as i32 + inc_y) as usize;
            }
            
            y[iy] += alpha * temp2;
            ix = (ix as i32 + inc_x) as usize;
            iy = (iy as i32 + inc_y) as usize;
        }
    } else {
        cblas_xerbla(0, "cblas_zhpmv", "unrecognized operation");
    }
}