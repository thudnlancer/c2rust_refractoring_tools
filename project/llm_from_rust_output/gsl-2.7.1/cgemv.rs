use num_complex::Complex32;

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

pub fn cblas_cgemv(
    order: CBLAS_ORDER,
    trans_a: CBLAS_TRANSPOSE,
    m: i32,
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
        cblas_xerbla(pos, "cblas_cgemv", "");
        return;
    }
    
    if m == 0 || n == 0 {
        return;
    }
    
    if alpha == Complex32::new(0.0, 0.0) && beta == Complex32::new(1.0, 0.0) {
        return;
    }
    
    let (len_x, len_y) = if trans_a == CBLAS_TRANSPOSE::NoTrans {
        (n, m)
    } else {
        (m, n)
    };
    
    if beta == Complex32::new(0.0, 0.0) {
        for i in 0..len_y {
            let idx = if inc_y > 0 { i } else { (len_y - 1 - i) * (-inc_y) as usize };
            y[idx as usize] = Complex32::new(0.0, 0.0);
        }
    } else if beta != Complex32::new(1.0, 0.0) {
        for i in 0..len_y {
            let idx = if inc_y > 0 { i } else { (len_y - 1 - i) * (-inc_y) as usize };
            y[idx as usize] = y[idx as usize] * beta;
        }
    }
    
    if alpha == Complex32::new(0.0, 0.0) {
        return;
    }
    
    match (order, trans_a) {
        (CBLAS_ORDER::RowMajor, CBLAS_TRANSPOSE::NoTrans) |
        (CBLAS_ORDER::ColMajor, CBLAS_TRANSPOSE::Trans) => {
            for i in 0..len_y {
                let y_idx = if inc_y > 0 { i } else { (len_y - 1 - i) * (-inc_y) as usize };
                let mut dot = Complex32::new(0.0, 0.0);
                
                for j in 0..len_x {
                    let x_idx = if inc_x > 0 { j } else { (len_x - 1 - j) * (-inc_x) as usize };
                    let a_idx = (i * lda + j) as usize;
                    
                    dot += a[a_idx] * x[x_idx as usize];
                }
                
                y[y_idx as usize] += alpha * dot;
            }
        }
        
        (CBLAS_ORDER::RowMajor, CBLAS_TRANSPOSE::Trans) |
        (CBLAS_ORDER::ColMajor, CBLAS_TRANSPOSE::NoTrans) => {
            for j in 0..len_x {
                let x_idx = if inc_x > 0 { j } else { (len_x - 1 - j) * (-inc_x) as usize };
                let tmp = alpha * x[x_idx as usize];
                
                for i in 0..len_y {
                    let y_idx = if inc_y > 0 { i } else { (len_y - 1 - i) * (-inc_y) as usize };
                    let a_idx = (j * lda + i) as usize;
                    
                    y[y_idx as usize] += a[a_idx] * tmp;
                }
            }
        }
        
        (CBLAS_ORDER::RowMajor, CBLAS_TRANSPOSE::ConjTrans) => {
            for j in 0..len_x {
                let x_idx = if inc_x > 0 { j } else { (len_x - 1 - j) * (-inc_x) as usize };
                let tmp = alpha * x[x_idx as usize];
                
                for i in 0..len_y {
                    let y_idx = if inc_y > 0 { i } else { (len_y - 1 - i) * (-inc_y) as usize };
                    let a_idx = (j * lda + i) as usize;
                    let a_conj = Complex32::new(a[a_idx].re, -a[a_idx].im);
                    
                    y[y_idx as usize] += a_conj * tmp;
                }
            }
        }
        
        (CBLAS_ORDER::ColMajor, CBLAS_TRANSPOSE::ConjTrans) => {
            for i in 0..len_y {
                let y_idx = if inc_y > 0 { i } else { (len_y - 1 - i) * (-inc_y) as usize };
                let mut dot = Complex32::new(0.0, 0.0);
                
                for j in 0..len_x {
                    let x_idx = if inc_x > 0 { j } else { (len_x - 1 - j) * (-inc_x) as usize };
                    let a_idx = (i * lda + j) as usize;
                    let a_conj = Complex32::new(a[a_idx].re, -a[a_idx].im);
                    
                    dot += a_conj * x[x_idx as usize];
                }
                
                y[y_idx as usize] += alpha * dot;
            }
        }
        
        _ => {
            cblas_xerbla(0, "cblas_cgemv", "unrecognized operation");
        }
    }
}