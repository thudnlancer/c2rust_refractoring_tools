use ndarray::{Array1, Array2, ArrayView1, ArrayView2, ArrayViewMut1, ArrayViewMut2, Axis};
use ndarray_linalg::{Scalar, SVD};
use num_traits::float::Float;
use std::cmp::{min, max};
use std::f64;

pub struct SVDResult {
    pub u: Array2<f64>,
    pub v: Array2<f64>,
    pub s: Array1<f64>,
}

pub fn svd_decomp(
    a: &Array2<f64>,
    v: &mut Array2<f64>,
    s: &mut Array1<f64>,
    work: &mut Array1<f64>,
) -> Result<(), String> {
    let (m, n) = a.dim();
    let k = min(m, n);

    if m < n {
        return Err("svd of MxN matrix, M<N, is not implemented".to_string());
    } else if v.dim().0 != n {
        return Err("square matrix V must match second dimension of matrix A".to_string());
    } else if v.dim().0 != v.dim().1 {
        return Err("matrix V must be square".to_string());
    } else if s.len() != n {
        return Err("length of vector S must match second dimension of matrix A".to_string());
    } else if work.len() != n {
        return Err("length of workspace must match second dimension of matrix A".to_string());
    }

    if n == 1 {
        let column = a.column(0);
        let norm = column.dot(&column).sqrt();

        s[0] = norm;
        v[(0, 0)] = 1.0;

        if norm != 0.0 {
            let mut column_mut = a.column_mut(0);
            column_mut /= norm;
        }

        return Ok(());
    }

    let mut a = a.clone();
    let mut f = work.slice_mut(s![0..k - 1]);

    // TODO: Implement bidiagonal decomposition and unpacking
    // gsl_linalg_bidiag_decomp(&mut a, s, &mut f);
    // gsl_linalg_bidiag_unpack2(&mut a, s, &mut f, v);

    chop_small_elements(s, &mut f);

    let mut b = n - 1;
    let mut iter = 0;

    while b > 0 {
        let fbm1 = f[b - 1];

        if fbm1 == 0.0 || fbm1.is_nan() {
            b -= 1;
            continue;
        }

        let mut a_idx = b - 1;
        while a_idx > 0 {
            let fam1 = f[a_idx - 1];
            if fam1 == 0.0 || fam1.is_nan() {
                break;
            }
            a_idx -= 1;
        }

        iter += 1;
        if iter > 100 * n {
            return Err("SVD decomposition failed to converge".to_string());
        }

        let n_block = b - a_idx + 1;
        let mut s_block = s.slice_mut(s![a_idx..a_idx + n_block]);
        let mut f_block = f.slice_mut(s![a_idx..a_idx + n_block - 1]);

        let mut u_block = a.slice_mut(s![.., a_idx..a_idx + n_block]);
        let mut v_block = v.slice_mut(s![.., a_idx..a_idx + n_block]);

        let mut rescale = false;
        let mut scale = 1.0;
        let mut norm = 0.0;

        for i in 0..n_block {
            let s_i = s_block[i].abs();
            if s_i > norm {
                norm = s_i;
            }
        }

        for i in 0..n_block - 1 {
            let f_i = f_block[i].abs();
            if f_i > norm {
                norm = f_i;
            }
        }

        if norm > f64::MAX.sqrt() {
            scale = norm / f64::MAX.sqrt();
            rescale = true;
        } else if norm < f64::MIN_POSITIVE.sqrt() && norm > 0.0 {
            scale = norm / f64::MIN_POSITIVE.sqrt();
            rescale = true;
        }

        if rescale {
            s_block /= scale;
            f_block /= scale;
        }

        // TODO: Implement QR step
        // qrstep(&mut s_block, &mut f_block, &mut u_block, &mut v_block);
        chop_small_elements(&mut s_block, &mut f_block);

        if rescale {
            s_block *= scale;
            f_block *= scale;
        }
    }

    for j in 0..k {
        let sj = s[j];
        if sj < 0.0 {
            for i in 0..n {
                v[(i, j)] = -v[(i, j)];
            }
            s[j] = -sj;
        }
    }

    for i in 0..k {
        let mut s_max = s[i];
        let mut i_max = i;

        for j in i + 1..k {
            let sj = s[j];
            if sj > s_max {
                s_max = sj;
                i_max = j;
            }
        }

        if i_max != i {
            s.swap(i, i_max);
            swap_columns(&mut a, i, i_max);
            swap_columns(v, i, i_max);
        }
    }

    Ok(())
}

fn chop_small_elements(s: &mut ArrayViewMut1<f64>, f: &mut ArrayViewMut1<f64>) {
    let n = s.len();
    for i in 0..n - 1 {
        let si = s[i];
        let fi = f[i];

        let r = (si * si + fi * fi).sqrt();
        if r == 0.0 {
            continue;
        }

        let cs = si / r;
        let sn = fi / r;

        s[i] = r;
        f[i] = 0.0;

        if i < n - 1 {
            let next_s = s[i + 1];
            s[i + 1] = next_s * cs;
            f[i + 1] = next_s * sn;
        }
    }
}

fn swap_columns(m: &mut ArrayViewMut2<f64>, i: usize, j: usize) {
    for row in m.rows_mut() {
        row.swap(i, j);
    }
}

// TODO: Implement remaining functions (SV_decomp_mod, SV_solve, SV_leverage, SV_decomp_jacobi)