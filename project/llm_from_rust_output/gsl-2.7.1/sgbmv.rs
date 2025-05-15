use std::ffi::CString;

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

pub fn cblas_sgbmv(
    order: CBLAS_ORDER,
    trans_a: CBLAS_TRANSPOSE,
    m: i32,
    n: i32,
    kl: i32,
    ku: i32,
    alpha: f32,
    a: &[f32],
    lda: i32,
    x: &[f32],
    inc_x: i32,
    beta: f32,
    y: &mut [f32],
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
        panic!("Negative dimension parameter");
    }
    if lda < std::cmp::max(1, kl + ku + 1) {
        panic!("Invalid lda parameter");
    }
    if inc_x == 0 || inc_y == 0 {
        panic!("Zero increment parameter");
    }

    // Early returns for edge cases
    if m == 0 || n == 0 {
        return;
    }
    if alpha == 0.0 && beta == 1.0 {
        return;
    }

    let trans = if trans_a == CBLAS_TRANSPOSE::ConjTrans {
        CBLAS_TRANSPOSE::Trans
    } else {
        trans_a
    };

    let (len_x, len_y, l, u) = if trans == CBLAS_TRANSPOSE::NoTrans {
        (n, m, kl, ku)
    } else {
        (m, n, ku, kl)
    };

    // Handle beta scaling
    if beta == 0.0 {
        for i in 0..len_y {
            let idx = if inc_y > 0 { i } else { (len_y - 1 - i) * (-inc_y) as usize };
            y[idx as usize] = 0.0;
        }
    } else if beta != 1.0 {
        for i in 0..len_y {
            let idx = if inc_y > 0 { i } else { (len_y - 1 - i) * (-inc_y) as usize };
            y[idx as usize] *= beta;
        }
    }

    if alpha == 0.0 {
        return;
    }

    match (order, trans) {
        (CBLAS_ORDER::RowMajor, CBLAS_TRANSPOSE::NoTrans) |
        (CBLAS_ORDER::ColMajor, CBLAS_TRANSPOSE::Trans) => {
            for i in 0..len_y {
                let iy = if inc_y > 0 { i } else { (len_y - 1 - i) * (-inc_y) as usize };
                let mut temp = 0.0;
                
                let j_min = std::cmp::max(0, i - l);
                let j_max = std::cmp::min(len_x, i + u + 1);
                
                let mut jx = if inc_x > 0 { 0 } else { (len_x - 1) * (-inc_x) } + j_min * inc_x;
                
                for j in j_min..j_max {
                    temp += x[jx as usize] * a[(l - i + j + i * lda) as usize];
                    jx += inc_x;
                }
                
                y[iy as usize] += alpha * temp;
            }
        }
        
        (CBLAS_ORDER::RowMajor, CBLAS_TRANSPOSE::Trans) |
        (CBLAS_ORDER::ColMajor, CBLAS_TRANSPOSE::NoTrans) => {
            for j in 0..len_x {
                let jx = if inc_x > 0 { j } else { (len_x - 1 - j) * (-inc_x) as usize };
                let temp = alpha * x[jx as usize];
                
                if temp != 0.0 {
                    let i_min = std::cmp::max(0, j - u);
                    let i_max = std::cmp::min(len_y, j + l + 1);
                    
                    for i in i_min..i_max {
                        let iy = if inc_y > 0 { i } else { (len_y - 1 - i) * (-inc_y) as usize };
                        y[iy as usize] += temp * a[(lda * j + (u + i - j)) as usize];
                    }
                }
            }
        }
        
        _ => panic!("Unrecognized operation"),
    }
}