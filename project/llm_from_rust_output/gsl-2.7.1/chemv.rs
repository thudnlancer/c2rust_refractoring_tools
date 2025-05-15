use num_complex::Complex32;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CBLAS_ORDER {
    RowMajor = 101,
    ColMajor = 102,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CBLAS_UPLO {
    Upper = 121,
    Lower = 122,
}

fn cblas_xerbla(p: i32, rout: &str, form: &str) {
    eprintln!("Parameter {} was incorrect in {}: {}", p, rout, form);
}

pub fn cblas_chemv(
    order: CBLAS_ORDER,
    uplo: CBLAS_UPLO,
    n: i32,
    alpha: Complex32,
    a: &[Complex32],
    lda: i32,
    x: &[Complex32],
    inc_x: i32,
    beta: Complex32,
    y: &mut [Complex32],
    inc_y: i32,
) {
    let conj = if order == CBLAS_ORDER::ColMajor { -1 } else { 1 };

    let mut pos = 0;
    if order != CBLAS_ORDER::RowMajor && order != CBLAS_ORDER::ColMajor {
        pos = 1;
    }
    if uplo != CBLAS_UPLO::Upper && uplo != CBLAS_UPLO::Lower {
        pos = 2;
    }
    if n < 0 {
        pos = 3;
    }
    if lda < 1.max(n) {
        pos = 6;
    }
    if inc_x == 0 {
        pos = 8;
    }
    if inc_y == 0 {
        pos = 11;
    }
    if pos != 0 {
        cblas_xerbla(pos, "cblas_chemv", "");
        return;
    }

    if alpha == Complex32::new(0.0, 0.0) && beta == Complex32::new(1.0, 0.0) {
        return;
    }

    if beta == Complex32::new(0.0, 0.0) {
        let mut iy = if inc_y > 0 { 0 } else { (n - 1) * -inc_y };
        for _ in 0..n {
            let idx = (iy * 2) as usize;
            y[idx] = Complex32::new(0.0, 0.0);
            y[idx + 1] = Complex32::new(0.0, 0.0);
            iy += inc_y;
        }
    } else if beta != Complex32::new(1.0, 0.0) {
        let mut iy = if inc_y > 0 { 0 } else { (n - 1) * -inc_y };
        for _ in 0..n {
            let idx = (iy * 2) as usize;
            let y_val = Complex32::new(y[idx].re, y[idx + 1].re);
            let new_val = y_val * beta;
            y[idx] = Complex32::new(new_val.re, 0.0);
            y[idx + 1] = Complex32::new(new_val.im, 0.0);
            iy += inc_y;
        }
    }

    if alpha == Complex32::new(0.0, 0.0) {
        return;
    }

    match (order, uplo) {
        (CBLAS_ORDER::RowMajor, CBLAS_UPLO::Upper) | (CBLAS_ORDER::ColMajor, CBLAS_UPLO::Lower) => {
            let mut ix = if inc_x > 0 { 0 } else { (n - 1) * -inc_x };
            let mut iy = if inc_y > 0 { 0 } else { (n - 1) * -inc_y };
            
            for i in 0..n {
                let x_idx = (ix * 2) as usize;
                let x_val = Complex32::new(x[x_idx].re, x[x_idx + 1].re);
                let temp1 = alpha * x_val;
                
                let aii_idx = (lda * i + i) as usize * 2;
                let aii = Complex32::new(a[aii_idx].re, 0.0);
                
                let y_idx = (iy * 2) as usize;
                y[y_idx] += (temp1 * aii).re;
                y[y_idx + 1] += (temp1 * aii).im;
                
                let mut jx = (if inc_x > 0 { 0 } else { (n - 1) * -inc_x }) + (i + 1) * inc_x;
                let mut jy = (if inc_y > 0 { 0 } else { (n - 1) * -inc_y }) + (i + 1) * inc_y;
                let mut temp2 = Complex32::new(0.0, 0.0);
                
                for j in (i + 1)..n {
                    let a_idx = (lda * i + j) as usize * 2;
                    let a_val = Complex32::new(a[a_idx].re, conj as f32 * a[a_idx + 1].re);
                    
                    let y_j_idx = (jy * 2) as usize;
                    let update = temp1 * a_val.conj();
                    y[y_j_idx] += update.re;
                    y[y_j_idx + 1] += update.im;
                    
                    let x_j_idx = (jx * 2) as usize;
                    let x_j_val = Complex32::new(x[x_j_idx].re, x[x_j_idx + 1].re);
                    temp2 += x_j_val * a_val;
                    
                    jx += inc_x;
                    jy += inc_y;
                }
                
                let update = alpha * temp2;
                y[y_idx] += update.re;
                y[y_idx + 1] += update.im;
                
                ix += inc_x;
                iy += inc_y;
            }
        }
        
        (CBLAS_ORDER::RowMajor, CBLAS_UPLO::Lower) | (CBLAS_ORDER::ColMajor, CBLAS_UPLO::Upper) => {
            let mut ix = (if inc_x > 0 { 0 } else { (n - 1) * -inc_x }) + (n - 1) * inc_x;
            let mut iy = (if inc_y > 0 { 0 } else { (n - 1) * -inc_y }) + (n - 1) * inc_y;
            
            for i in (0..n).rev() {
                let x_idx = (ix * 2) as usize;
                let x_val = Complex32::new(x[x_idx].re, x[x_idx + 1].re);
                let temp1 = alpha * x_val;
                
                let aii_idx = (lda * i + i) as usize * 2;
                let aii = Complex32::new(a[aii_idx].re, 0.0);
                
                let y_idx = (iy * 2) as usize;
                y[y_idx] += (temp1 * aii).re;
                y[y_idx + 1] += (temp1 * aii).im;
                
                let mut jx = (if inc_x > 0 { 0 } else { (n - 1) * -inc_x });
                let mut jy = (if inc_y > 0 { 0 } else { (n - 1) * -inc_y });
                let mut temp2 = Complex32::new(0.0, 0.0);
                
                for j in 0..i {
                    let a_idx = (lda * i + j) as usize * 2;
                    let a_val = Complex32::new(a[a_idx].re, conj as f32 * a[a_idx + 1].re);
                    
                    let y_j_idx = (jy * 2) as usize;
                    let update = temp1 * a_val.conj();
                    y[y_j_idx] += update.re;
                    y[y_j_idx + 1] += update.im;
                    
                    let x_j_idx = (jx * 2) as usize;
                    let x_j_val = Complex32::new(x[x_j_idx].re, x[x_j_idx + 1].re);
                    temp2 += x_j_val * a_val;
                    
                    jx += inc_x;
                    jy += inc_y;
                }
                
                let update = alpha * temp2;
                y[y_idx] += update.re;
                y[y_idx + 1] += update.im;
                
                ix -= inc_x;
                iy -= inc_y;
            }
        }
        
        _ => {
            cblas_xerbla(0, "cblas_chemv", "unrecognized operation");
        }
    }
}