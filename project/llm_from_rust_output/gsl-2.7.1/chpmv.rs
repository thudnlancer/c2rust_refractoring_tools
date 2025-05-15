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

pub fn cblas_chpmv(
    order: CBLAS_ORDER,
    uplo: CBLAS_UPLO,
    n: i32,
    alpha: Complex32,
    ap: &[Complex32],
    x: &[Complex32],
    inc_x: i32,
    beta: Complex32,
    y: &mut [Complex32],
    inc_y: i32,
) {
    // Parameter validation
    if order != CBLAS_ORDER::RowMajor && order != CBLAS_ORDER::ColMajor {
        panic!("Invalid order parameter");
    }
    if uplo != CBLAS_UPLO::Upper && uplo != CBLAS_UPLO::Lower {
        panic!("Invalid uplo parameter");
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

    // Early return if alpha is zero and beta is one
    if alpha == Complex32::new(0.0, 0.0) && beta == Complex32::new(1.0, 0.0) {
        return;
    }

    // Scale Y by beta if beta is not one
    if beta != Complex32::new(1.0, 0.0) {
        let mut iy = if inc_y > 0 { 0 } else { (n - 1) * -inc_y };
        for _ in 0..n {
            if iy as usize >= y.len() {
                panic!("Y index out of bounds");
            }
            y[iy as usize] *= beta;
            iy += inc_y;
        }
    }

    // Early return if alpha is zero
    if alpha == Complex32::new(0.0, 0.0) {
        return;
    }

    // Main computation
    if (order == CBLAS_ORDER::RowMajor && uplo == CBLAS_UPLO::Upper)
        || (order == CBLAS_ORDER::ColMajor && uplo == CBLAS_UPLO::Lower)
    {
        let mut ix = if inc_x > 0 { 0 } else { (n - 1) * -inc_x };
        let mut iy = if inc_y > 0 { 0 } else { (n - 1) * -inc_y };
        
        for i in 0..n {
            let x_i = if ix as usize >= x.len() {
                panic!("X index out of bounds");
            } else {
                x[ix as usize]
            };
            let temp1 = alpha * x_i;
            
            let mut temp2 = Complex32::new(0.0, 0.0);
            let j_min = i + 1;
            let j_max = n;
            
            let mut jx = ix + j_min * inc_x;
            let mut jy = iy + j_min * inc_y;
            
            let a_ii = if (i as usize) >= ap.len() {
                panic!("Ap index out of bounds");
            } else {
                ap[(i * (2 * n - i + 1) / 2) as usize].re
            };
            
            if iy as usize >= y.len() {
                panic!("Y index out of bounds");
            }
            y[iy as usize] += temp1 * a_ii;
            
            for j in j_min..j_max {
                let a_ij = if (j as usize) >= ap.len() {
                    panic!("Ap index out of bounds");
                } else {
                    ap[(i * (2 * n - i + 1) / 2 + j - i) as usize]
                };
                
                if jy as usize >= y.len() {
                    panic!("Y index out of bounds");
                }
                y[jy as usize] += temp1 * a_ij.conj();
                
                let x_j = if jx as usize >= x.len() {
                    panic!("X index out of bounds");
                } else {
                    x[jx as usize]
                };
                temp2 += x_j * a_ij;
                
                jx += inc_x;
                jy += inc_y;
            }
            
            if iy as usize >= y.len() {
                panic!("Y index out of bounds");
            }
            y[iy as usize] += alpha * temp2;
            
            ix += inc_x;
            iy += inc_y;
        }
    } else if (order == CBLAS_ORDER::RowMajor && uplo == CBLAS_UPLO::Lower)
        || (order == CBLAS_ORDER::ColMajor && uplo == CBLAS_UPLO::Upper)
    {
        let mut ix = if inc_x > 0 { 0 } else { (n - 1) * -inc_x };
        let mut iy = if inc_y > 0 { 0 } else { (n - 1) * -inc_y };
        
        for i in 0..n {
            let x_i = if ix as usize >= x.len() {
                panic!("X index out of bounds");
            } else {
                x[ix as usize]
            };
            let temp1 = alpha * x_i;
            
            let mut temp2 = Complex32::new(0.0, 0.0);
            let j_min = 0;
            let j_max = i;
            
            let mut jx = ix + j_min * inc_x;
            let mut jy = iy + j_min * inc_y;
            
            let a_ii = if (i as usize) >= ap.len() {
                panic!("Ap index out of bounds");
            } else {
                ap[(i * (i + 1) / 2 + i) as usize].re
            };
            
            if iy as usize >= y.len() {
                panic!("Y index out of bounds");
            }
            y[iy as usize] += temp1 * a_ii;
            
            for j in j_min..j_max {
                let a_ij = if (j as usize) >= ap.len() {
                    panic!("Ap index out of bounds");
                } else {
                    ap[(i * (i + 1) / 2 + j) as usize]
                };
                
                if jy as usize >= y.len() {
                    panic!("Y index out of bounds");
                }
                y[jy as usize] += temp1 * a_ij.conj();
                
                let x_j = if jx as usize >= x.len() {
                    panic!("X index out of bounds");
                } else {
                    x[jx as usize]
                };
                temp2 += x_j * a_ij;
                
                jx += inc_x;
                jy += inc_y;
            }
            
            if iy as usize >= y.len() {
                panic!("Y index out of bounds");
            }
            y[iy as usize] += alpha * temp2;
            
            ix += inc_x;
            iy += inc_y;
        }
    } else {
        panic!("Unrecognized operation");
    }
}