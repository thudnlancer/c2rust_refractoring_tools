use num_complex::Complex64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CBLAS_ORDER {
    RowMajor = 101,
    ColMajor = 102,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CBLAS_TRANSPOSE {
    NoTrans = 111,
    Trans = 112,
    ConjTrans = 113,
}

fn cblas_xerbla(p: i32, rout: &str, form: &str) {
    eprintln!("Parameter {} was incorrect in {}: {}", p, rout, form);
}

pub fn cblas_zgemv(
    order: CBLAS_ORDER,
    trans_a: CBLAS_TRANSPOSE,
    m: i32,
    n: i32,
    alpha: Complex64,
    a: &[Complex64],
    lda: i32,
    x: &[Complex64],
    inc_x: i32,
    beta: Complex64,
    y: &mut [Complex64],
    inc_y: i32,
) {
    let mut pos = 0;
    
    if m < 0 {
        pos = 3;
    }
    if n < 0 {
        pos = 4;
    }
    
    match order {
        CBLAS_ORDER::RowMajor => {
            if lda < 1.max(n) {
                pos = 7;
            }
        }
        CBLAS_ORDER::ColMajor => {
            if lda < 1.max(m) {
                pos = 7;
            }
        }
    }
    
    if inc_x == 0 {
        pos = 9;
    }
    if inc_y == 0 {
        pos = 12;
    }
    
    if pos != 0 {
        cblas_xerbla(pos, "cblas_zgemv", "");
        return;
    }
    
    if m == 0 || n == 0 {
        return;
    }
    
    if alpha == Complex64::new(0.0, 0.0) && beta == Complex64::new(1.0, 0.0) {
        return;
    }
    
    let (len_x, len_y) = if trans_a == CBLAS_TRANSPOSE::NoTrans {
        (n, m)
    } else {
        (m, n)
    };
    
    // Apply beta to y
    if beta == Complex64::new(0.0, 0.0) {
        for i in 0..len_y as usize {
            let idx = if inc_y > 0 { i } else { (len_y as usize - 1) - i };
            y[idx] = Complex64::new(0.0, 0.0);
        }
    } else if beta != Complex64::new(1.0, 0.0) {
        for i in 0..len_y as usize {
            let idx = if inc_y > 0 { i } else { (len_y as usize - 1) - i };
            y[idx] *= beta;
        }
    }
    
    if alpha == Complex64::new(0.0, 0.0) {
        return;
    }
    
    match (order, trans_a) {
        (CBLAS_ORDER::RowMajor, CBLAS_TRANSPOSE::NoTrans) |
        (CBLAS_ORDER::ColMajor, CBLAS_TRANSPOSE::Trans) => {
            for i in 0..len_y as usize {
                let mut dot = Complex64::new(0.0, 0.0);
                for j in 0..len_x as usize {
                    let a_idx = if order == CBLAS_ORDER::RowMajor {
                        i * lda as usize + j
                    } else {
                        j * lda as usize + i
                    };
                    dot += a[a_idx] * x[j];
                }
                let y_idx = if inc_y > 0 { i } else { (len_y as usize - 1) - i };
                y[y_idx] += alpha * dot;
            }
        }
        
        (CBLAS_ORDER::RowMajor, CBLAS_TRANSPOSE::Trans) |
        (CBLAS_ORDER::ColMajor, CBLAS_TRANSPOSE::NoTrans) => {
            for j in 0..len_x as usize {
                let tmp = alpha * x[j];
                for i in 0..len_y as usize {
                    let a_idx = if order == CBLAS_ORDER::RowMajor {
                        j * lda as usize + i
                    } else {
                        i * lda as usize + j
                    };
                    let y_idx = if inc_y > 0 { i } else { (len_y as usize - 1) - i };
                    y[y_idx] += a[a_idx] * tmp;
                }
            }
        }
        
        (CBLAS_ORDER::RowMajor, CBLAS_TRANSPOSE::ConjTrans) => {
            for j in 0..len_x as usize {
                let tmp = alpha * x[j];
                for i in 0..len_y as usize {
                    let a_idx = j * lda as usize + i;
                    let y_idx = if inc_y > 0 { i } else { (len_y as usize - 1) - i };
                    y[y_idx] += a[a_idx].conj() * tmp;
                }
            }
        }
        
        (CBLAS_ORDER::ColMajor, CBLAS_TRANSPOSE::ConjTrans) => {
            for i in 0..len_y as usize {
                let mut dot = Complex64::new(0.0, 0.0);
                for j in 0..len_x as usize {
                    let a_idx = i * lda as usize + j;
                    dot += a[a_idx].conj() * x[j];
                }
                let y_idx = if inc_y > 0 { i } else { (len_y as usize - 1) - i };
                y[y_idx] += alpha * dot;
            }
        }
        
        _ => {
            cblas_xerbla(0, "cblas_zgemv", "unrecognized operation");
        }
    }
}