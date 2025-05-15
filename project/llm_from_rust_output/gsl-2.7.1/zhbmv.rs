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

pub fn cblas_zhbmv(
    order: CBLAS_ORDER,
    uplo: CBLAS_UPLO,
    n: i32,
    k: i32,
    alpha: Complex64,
    a: &[Complex64],
    lda: i32,
    x: &[Complex64],
    inc_x: i32,
    beta: Complex64,
    y: &mut [Complex64],
    inc_y: i32,
) {
    if order != CBLAS_ORDER::RowMajor && order != CBLAS_ORDER::ColMajor {
        panic!("Invalid order parameter");
    }
    if uplo != CBLAS_UPLO::Upper && uplo != CBLAS_UPLO::Lower {
        panic!("Invalid uplo parameter");
    }
    if n < 0 {
        panic!("Invalid n parameter");
    }
    if k < 0 {
        panic!("Invalid k parameter");
    }
    if lda < k + 1 {
        panic!("Invalid lda parameter");
    }
    if inc_x == 0 {
        panic!("Invalid inc_x parameter");
    }
    if inc_y == 0 {
        panic!("Invalid inc_y parameter");
    }

    if n == 0 {
        return;
    }

    if alpha == Complex64::new(0.0, 0.0) && beta == Complex64::new(1.0, 0.0) {
        return;
    }

    // Scale Y by beta
    if beta == Complex64::new(0.0, 0.0) {
        let mut iy = if inc_y > 0 { 0 } else { (n - 1) * -inc_y };
        for _ in 0..n {
            if iy as usize >= y.len() {
                panic!("Y index out of bounds");
            }
            y[iy as usize] = Complex64::new(0.0, 0.0);
            iy += inc_y;
        }
    } else if beta != Complex64::new(1.0, 0.0) {
        let mut iy = if inc_y > 0 { 0 } else { (n - 1) * -inc_y };
        for _ in 0..n {
            if iy as usize >= y.len() {
                panic!("Y index out of bounds");
            }
            y[iy as usize] *= beta;
            iy += inc_y;
        }
    }

    if alpha == Complex64::new(0.0, 0.0) {
        return;
    }

    let conj = if order == CBLAS_ORDER::ColMajor {
        Complex64::new(0.0, -1.0)
    } else {
        Complex64::new(0.0, 1.0)
    };

    match (order, uplo) {
        (CBLAS_ORDER::RowMajor, CBLAS_UPLO::Upper) |
        (CBLAS_ORDER::ColMajor, CBLAS_UPLO::Lower) => {
            let mut ix = if inc_x > 0 { 0 } else { (n - 1) * -inc_x };
            let mut iy = if inc_y > 0 { 0 } else { (n - 1) * -inc_y };
            
            for i in 0..n {
                let x_val = if ix as usize >= x.len() {
                    panic!("X index out of bounds");
                } else {
                    x[ix as usize]
                };
                let temp1 = alpha * x_val;
                
                let aii = if (lda * i) as usize >= a.len() {
                    panic!("A index out of bounds");
                } else {
                    a[(lda * i) as usize].re
                };
                
                if iy as usize >= y.len() {
                    panic!("Y index out of bounds");
                }
                y[iy as usize] += temp1 * aii;
                
                let j_min = i + 1;
                let j_max = n.min(i + k + 1);
                
                let mut jx = if inc_x > 0 { 0 } else { (n - 1) * -inc_x } + j_min * inc_x;
                let mut jy = if inc_y > 0 { 0 } else { (n - 1) * -inc_y } + j_min * inc_y;
                let mut temp2 = Complex64::new(0.0, 0.0);
                
                for j in j_min..j_max {
                    let aij = if (lda * i + (j - i)) as usize >= a.len() {
                        panic!("A index out of bounds");
                    } else {
                        let idx = (lda * i + (j - i)) as usize;
                        Complex64::new(a[idx].re, conj.im * a[idx].im)
                    };
                    
                    if jy as usize >= y.len() {
                        panic!("Y index out of bounds");
                    }
                    y[jy as usize] += temp1 * aij.conj();
                    
                    let x_j = if jx as usize >= x.len() {
                        panic!("X index out of bounds");
                    } else {
                        x[jx as usize]
                    };
                    temp2 += x_j * aij;
                    
                    jx += inc_x;
                    jy += inc_y;
                }
                
                y[iy as usize] += alpha * temp2;
                ix += inc_x;
                iy += inc_y;
            }
        }
        
        (CBLAS_ORDER::RowMajor, CBLAS_UPLO::Lower) |
        (CBLAS_ORDER::ColMajor, CBLAS_UPLO::Upper) => {
            let mut ix = if inc_x > 0 { 0 } else { (n - 1) * -inc_x };
            let mut iy = if inc_y > 0 { 0 } else { (n - 1) * -inc_y };
            
            for i in 0..n {
                let x_val = if ix as usize >= x.len() {
                    panic!("X index out of bounds");
                } else {
                    x[ix as usize]
                };
                let temp1 = alpha * x_val;
                
                let j_min = if k > i { 0 } else { i - k };
                let j_max = i;
                
                let mut jx = if inc_x > 0 { 0 } else { (n - 1) * -inc_x } + j_min * inc_x;
                let mut jy = if inc_y > 0 { 0 } else { (n - 1) * -inc_y } + j_min * inc_y;
                let mut temp2 = Complex64::new(0.0, 0.0);
                
                for j in j_min..j_max {
                    let aij = if (i * lda + (k - i + j)) as usize >= a.len() {
                        panic!("A index out of bounds");
                    } else {
                        let idx = (i * lda + (k - i + j)) as usize;
                        Complex64::new(a[idx].re, conj.im * a[idx].im)
                    };
                    
                    if jy as usize >= y.len() {
                        panic!("Y index out of bounds");
                    }
                    y[jy as usize] += temp1 * aij.conj();
                    
                    let x_j = if jx as usize >= x.len() {
                        panic!("X index out of bounds");
                    } else {
                        x[jx as usize]
                    };
                    temp2 += x_j * aij;
                    
                    jx += inc_x;
                    jy += inc_y;
                }
                
                let aii = if (lda * i + k) as usize >= a.len() {
                    panic!("A index out of bounds");
                } else {
                    a[(lda * i + k) as usize].re
                };
                
                if iy as usize >= y.len() {
                    panic!("Y index out of bounds");
                }
                y[iy as usize] += temp1 * aii;
                y[iy as usize] += alpha * temp2;
                
                ix += inc_x;
                iy += inc_y;
            }
        }
        
        _ => panic!("Unrecognized operation"),
    }
}