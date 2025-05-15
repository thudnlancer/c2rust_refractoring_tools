use ndarray::{Array2, Array1, Axis, s};
use ndarray_linalg::{QR, Givens, Rotate};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LinAlgError {
    #[error("Hessenberg-triangular reduction requires square matrices")]
    NotSquare,
    #[error("length of workspace must match matrix dimension")]
    BadLength,
}

pub fn hesstri_decomp(
    a: &mut Array2<f64>,
    b: &mut Array2<f64>,
    u: Option<&mut Array2<f64>>,
    v: Option<&mut Array2<f64>>,
    work: &mut Array1<f64>,
) -> Result<(), LinAlgError> {
    let n = a.shape()[0];

    if a.shape()[1] != n || b.shape()[0] != n || b.shape()[1] != n {
        return Err(LinAlgError::NotSquare);
    } else if work.len() != n {
        return Err(LinAlgError::BadLength);
    }

    // B -> Q^T B = R (upper triangular)
    let qr = b.qr()?;
    *b = qr.r();

    // A -> Q^T A
    let q = qr.q();
    *a = q.t().dot(a);

    // Initialize U and V if desired
    if let Some(u_mat) = u {
        *u_mat = q;
        // Reset B to R since we unpacked Q into U
        *b = qr.r();
    } else {
        // Zero out lower triangle of B
        for j in 0..n - 1 {
            for i in j + 1..n {
                b[(i, j)] = 0.0;
            }
        }
    }

    if let Some(v_mat) = v {
        v_mat.fill(0.0);
        for i in 0..n {
            v_mat[(i, i)] = 1.0;
        }
    }

    if n < 3 {
        return Ok(());
    }

    // Reduce A and B
    for j in 0..n - 2 {
        for i in (j + 2..n).rev() {
            // Step 1: rotate rows i-1, i to kill A(i,j)
            let a_ij = a[(i - 1, j)];
            let a_i1j = a[(i, j)];
            let g = Givens::new(a_ij, a_i1j).unwrap();
            let (cs, sn) = (g.c(), -g.s()); // Invert sn for transpose

            // Apply G^t to A(i-1:i, j:n)
            {
                let mut a_row1 = a.slice_mut(s![i - 1, j..]);
                let mut a_row2 = a.slice_mut(s![i, j..]);
                g.rotate_rows(&mut a_row1, &mut a_row2);
            }

            // Apply G^t to B(i-1:i, i-1:n)
            {
                let mut b_row1 = b.slice_mut(s![i - 1, i - 1..]);
                let mut b_row2 = b.slice_mut(s![i, i - 1..]);
                g.rotate_rows(&mut b_row1, &mut b_row2);
            }

            if let Some(u_mat) = u {
                // Accumulate U: U -> U G
                let mut u_col1 = u_mat.slice_mut(s![.., i - 1]);
                let mut u_col2 = u_mat.slice_mut(s![.., i]);
                g.rotate_cols(&mut u_col1, &mut u_col2);
            }

            // Step 2: rotate columns i, i-1 to kill B(i, i-1)
            let b_ii = -b[(i, i)];
            let b_ii1 = b[(i, i - 1)];
            let g = Givens::new(b_ii, b_ii1).unwrap();
            let (cs, sn) = (g.c(), -g.s()); // Invert sn for transpose

            // Apply G to B(0..=i, i-1:i)
            {
                let mut b_col1 = b.slice_mut(s![..=i, i - 1]);
                let mut b_col2 = b.slice_mut(s![..=i, i]);
                g.rotate_cols(&mut b_col1, &mut b_col2);
            }

            // Apply to A(.., i-1:i)
            {
                let mut a_col1 = a.slice_mut(s![.., i - 1]);
                let mut a_col2 = a.slice_mut(s![.., i]);
                g.rotate_cols(&mut a_col1, &mut a_col2);
            }

            if let Some(v_mat) = v {
                // Accumulate V: V -> V G
                let mut v_col1 = v_mat.slice_mut(s![.., i - 1]);
                let mut v_col2 = v_mat.slice_mut(s![.., i]);
                g.rotate_cols(&mut v_col1, &mut v_col2);
            }
        }
    }

    Ok(())
}