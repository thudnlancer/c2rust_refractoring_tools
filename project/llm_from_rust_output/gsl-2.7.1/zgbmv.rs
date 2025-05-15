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

pub fn cblas_zgbmv(
    order: CBLAS_ORDER,
    trans_a: CBLAS_TRANSPOSE,
    m: i32,
    n: i32,
    kl: i32,
    ku: i32,
    alpha: Complex64,
    a: &[Complex64],
    lda: i32,
    x: &[Complex64],
    inc_x: i32,
    beta: Complex64,
    y: &mut [Complex64],
    inc_y: i32,
) {
    // Parameter validation
    if m == 0 || n == 0 {
        return;
    }

    if alpha == Complex64::new(0.0, 0.0) && beta == Complex64::new(1.0, 0.0) {
        return;
    }

    let (len_x, len_y, l, u) = match trans_a {
        CBLAS_TRANSPOSE::NoTrans => (n, m, kl, ku),
        _ => (m, n, ku, kl),
    };

    // Apply beta to Y
    if beta == Complex64::new(0.0, 0.0) {
        for i in 0..len_y as usize {
            let idx = if inc_y > 0 { i } else { (len_y - 1 - i as i32) as usize };
            y[idx * inc_y.abs() as usize] = Complex64::new(0.0, 0.0);
        }
    } else if beta != Complex64::new(1.0, 0.0) {
        for i in 0..len_y as usize {
            let idx = if inc_y > 0 { i } else { (len_y - 1 - i as i32) as usize };
            y[idx * inc_y.abs() as usize] *= beta;
        }
    }

    if alpha == Complex64::new(0.0, 0.0) {
        return;
    }

    match (order, trans_a) {
        (CBLAS_ORDER::RowMajor, CBLAS_TRANSPOSE::NoTrans) |
        (CBLAS_ORDER::ColMajor, CBLAS_TRANSPOSE::Trans) => {
            for i in 0..len_y as usize {
                let j_min = if i as i32 > l { i as i32 - l } else { 0 };
                let j_max = (len_x.min(i as i32 + u + 1)) as usize;
                
                let mut dot = Complex64::new(0.0, 0.0);
                for j in j_min as usize..j_max {
                    let a_idx = lda * i as i32 + (l + j as i32 - i as i32);
                    dot += a[a_idx as usize] * x[j * inc_x.abs() as usize];
                }
                
                let y_idx = if inc_y > 0 { i } else { (len_y - 1 - i as i32) as usize };
                y[y_idx * inc_y.abs() as usize] += alpha * dot;
            }
        }
        
        (CBLAS_ORDER::RowMajor, CBLAS_TRANSPOSE::Trans) |
        (CBLAS_ORDER::ColMajor, CBLAS_TRANSPOSE::NoTrans) => {
            for j in 0..len_x as usize {
                let tmp = alpha * x[j * inc_x.abs() as usize];
                if tmp != Complex64::new(0.0, 0.0) {
                    let i_min = if j as i32 > u { j as i32 - u } else { 0 };
                    let i_max = (len_y.min(j as i32 + l + 1)) as usize;
                    
                    for i in i_min as usize..i_max {
                        let a_idx = lda * j as i32 + (u + i as i32 - j as i32);
                        let y_idx = if inc_y > 0 { i } else { (len_y - 1 - i as i32) as usize };
                        y[y_idx * inc_y.abs() as usize] += a[a_idx as usize] * tmp;
                    }
                }
            }
        }
        
        (CBLAS_ORDER::RowMajor, CBLAS_TRANSPOSE::ConjTrans) => {
            for j in 0..len_x as usize {
                let tmp = alpha * x[j * inc_x.abs() as usize];
                if tmp != Complex64::new(0.0, 0.0) {
                    let i_min = if j as i32 > u { j as i32 - u } else { 0 };
                    let i_max = (len_y.min(j as i32 + l + 1)) as usize;
                    
                    for i in i_min as usize..i_max {
                        let a_idx = lda * j as i32 + (u + i as i32 - j as i32);
                        let y_idx = if inc_y > 0 { i } else { (len_y - 1 - i as i32) as usize };
                        y[y_idx * inc_y.abs() as usize] += a[a_idx as usize].conj() * tmp;
                    }
                }
            }
        }
        
        (CBLAS_ORDER::ColMajor, CBLAS_TRANSPOSE::ConjTrans) => {
            for i in 0..len_y as usize {
                let j_min = if i as i32 > l { i as i32 - l } else { 0 };
                let j_max = (len_x.min(i as i32 + u + 1)) as usize;
                
                let mut dot = Complex64::new(0.0, 0.0);
                for j in j_min as usize..j_max {
                    let a_idx = lda * i as i32 + (l + j as i32 - i as i32);
                    dot += a[a_idx as usize].conj() * x[j * inc_x.abs() as usize];
                }
                
                let y_idx = if inc_y > 0 { i } else { (len_y - 1 - i as i32) as usize };
                y[y_idx * inc_y.abs() as usize] += alpha * dot;
            }
        }
        
        _ => panic!("unrecognized operation"),
    }
}