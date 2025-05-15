use num_complex::Complex32;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CBLAS_ORDER {
    RowMajor = 101,
    ColMajor = 102,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CBLAS_TRANSPOSE {
    NoTrans = 111,
    Trans = 112,
    ConjTrans = 113,
}

pub fn cblas_cgbmv(
    order: CBLAS_ORDER,
    trans_a: CBLAS_TRANSPOSE,
    m: i32,
    n: i32,
    kl: i32,
    ku: i32,
    alpha: Complex32,
    a: &[Complex32],
    lda: i32,
    x: &[Complex32],
    inc_x: i32,
    beta: Complex32,
    y: &mut [Complex32],
    inc_y: i32,
) {
    // Parameter validation
    if ![CBLAS_ORDER::RowMajor, CBLAS_ORDER::ColMajor].contains(&order) {
        panic!("Invalid order parameter");
    }
    if ![CBLAS_TRANSPOSE::NoTrans, CBLAS_TRANSPOSE::Trans, CBLAS_TRANSPOSE::ConjTrans].contains(&trans_a) {
        panic!("Invalid trans_a parameter");
    }
    if m < 0 || n < 0 || kl < 0 || ku < 0 {
        panic!("Negative matrix dimension");
    }
    if lda < (kl + ku + 1).max(1) {
        panic!("Invalid lda parameter");
    }
    if inc_x == 0 || inc_y == 0 {
        panic!("Zero increment");
    }

    // Early return if empty operation
    if m == 0 || n == 0 {
        return;
    }
    if alpha == Complex32::new(0.0, 0.0) && beta == Complex32::new(1.0, 0.0) {
        return;
    }

    // Determine lengths and bounds
    let (len_x, len_y, l, u) = if trans_a == CBLAS_TRANSPOSE::NoTrans {
        (n, m, kl, ku)
    } else {
        (m, n, ku, kl)
    };

    // Scale Y by beta if needed
    if beta == Complex32::new(0.0, 0.0) {
        for i in 0..len_y {
            let idx = if inc_y > 0 { i } else { (len_y - 1 - i) * (-inc_y) as usize };
            y[idx] = Complex32::new(0.0, 0.0);
        }
    } else if beta != Complex32::new(1.0, 0.0) {
        for i in 0..len_y {
            let idx = if inc_y > 0 { i } else { (len_y - 1 - i) * (-inc_y) as usize };
            y[idx] = y[idx] * beta;
        }
    }

    // Early return if alpha is zero
    if alpha == Complex32::new(0.0, 0.0) {
        return;
    }

    // Main computation
    match (order, trans_a) {
        (CBLAS_ORDER::RowMajor, CBLAS_TRANSPOSE::NoTrans) |
        (CBLAS_ORDER::ColMajor, CBLAS_TRANSPOSE::Trans) => {
            for i in 0..len_y {
                let j_min = (i as i32 - l).max(0) as usize;
                let j_max = (i as i32 + u + 1).min(len_x as i32) as usize;
                
                let mut dot = Complex32::new(0.0, 0.0);
                for j in j_min..j_max {
                    let a_idx = lda as usize * i + (l as usize + j - i);
                    let x_idx = if inc_x > 0 { j } else { (len_x - 1 - j) * (-inc_x) as usize };
                    dot += a[a_idx] * x[x_idx];
                }
                
                let y_idx = if inc_y > 0 { i } else { (len_y - 1 - i) * (-inc_y) as usize };
                y[y_idx] += alpha * dot;
            }
        },
        (CBLAS_ORDER::RowMajor, CBLAS_TRANSPOSE::Trans) |
        (CBLAS_ORDER::ColMajor, CBLAS_TRANSPOSE::NoTrans) => {
            for j in 0..len_x {
                let i_min = (j as i32 - u).max(0) as usize;
                let i_max = (j as i32 + l + 1).min(len_y as i32) as usize;
                
                let tmp = alpha * x[if inc_x > 0 { j } else { (len_x - 1 - j) * (-inc_x) as usize }];
                if tmp != Complex32::new(0.0, 0.0) {
                    for i in i_min..i_max {
                        let a_idx = lda as usize * j + (u as usize + i - j);
                        let y_idx = if inc_y > 0 { i } else { (len_y - 1 - i) * (-inc_y) as usize };
                        y[y_idx] += a[a_idx] * tmp;
                    }
                }
            }
        },
        (CBLAS_ORDER::RowMajor, CBLAS_TRANSPOSE::ConjTrans) => {
            for j in 0..len_x {
                let i_min = (j as i32 - u).max(0) as usize;
                let i_max = (j as i32 + l + 1).min(len_y as i32) as usize;
                
                let tmp = alpha * x[if inc_x > 0 { j } else { (len_x - 1 - j) * (-inc_x) as usize }];
                if tmp != Complex32::new(0.0, 0.0) {
                    for i in i_min..i_max {
                        let a_idx = lda as usize * j + (u as usize + i - j);
                        let y_idx = if inc_y > 0 { i } else { (len_y - 1 - i) * (-inc_y) as usize };
                        y[y_idx] += a[a_idx].conj() * tmp;
                    }
                }
            }
        },
        (CBLAS_ORDER::ColMajor, CBLAS_TRANSPOSE::ConjTrans) => {
            for i in 0..len_y {
                let j_min = (i as i32 - l).max(0) as usize;
                let j_max = (i as i32 + u + 1).min(len_x as i32) as usize;
                
                let mut dot = Complex32::new(0.0, 0.0);
                for j in j_min..j_max {
                    let a_idx = lda as usize * i + (l as usize + j - i);
                    let x_idx = if inc_x > 0 { j } else { (len_x - 1 - j) * (-inc_x) as usize };
                    dot += a[a_idx].conj() * x[x_idx];
                }
                
                let y_idx = if inc_y > 0 { i } else { (len_y - 1 - i) * (-inc_y) as usize };
                y[y_idx] += alpha * dot;
            }
        },
        _ => panic!("Unrecognized operation"),
    }
}