use gsl::{
    blas::drot,
    linalg::{QR_decomp, QR_QTmat, QR_unpack},
    Matrix, Vector, BLAS, Givens, QR,
};
use std::f64::consts;

pub fn hesstri_decomp(
    a: &mut Matrix,
    b: &mut Matrix,
    u: Option<&mut Matrix>,
    v: Option<&mut Matrix>,
    work: &mut Vector,
) -> Result<(), &'static str> {
    let n = a.size1();
    if n != a.size2() || n != b.size1() || n != b.size2() {
        return Err("Hessenberg-triangular reduction requires square matrices");
    } else if n != work.size() {
        return Err("length of workspace must match matrix dimension");
    }

    let mut cs = 0.0;
    let mut sn = 0.0;

    QR_decomp(b, work)?;
    QR_QTmat(b, work, a)?;

    match u {
        Some(u_mat) => {
            QR_unpack(b, work, u_mat, b)?;
        }
        None => {
            for j in 0..n - 1 {
                for i in j + 1..n {
                    b.set(i, j, 0.0);
                }
            }
        }
    }

    if let Some(v_mat) = v {
        v_mat.set_identity();
    }

    if n < 3 {
        return Ok(());
    }

    for j in 0..n - 2 {
        let mut i = n - 1;
        while i >= j + 2 {
            let (a_im1_j, a_i_j) = (a.get(i - 1, j), a.get(i, j);
            let g = Givens::new(a_im1_j, a_i_j);
            cs = g.c();
            sn = -g.s();

            drot(
                &mut a.subrow_mut(i - 1, j, n - j),
                &mut a.subrow_mut(i, j, n - j),
                cs,
                sn,
            )?;

            drot(
                &mut b.subrow_mut(i - 1, i - 1, n - i + 1),
                &mut b.subrow_mut(i, i - 1, n - i + 1),
                cs,
                sn,
            )?;

            if let Some(u_mat) = u {
                drot(
                    &mut u_mat.column_mut(i - 1),
                    &mut u_mat.column_mut(i),
                    cs,
                    sn,
                )?;
            }

            let (b_i_i, b_i_im1) = (-b.get(i, i), b.get(i, i - 1);
            let g = Givens::new(b_i_i, b_i_im1);
            cs = g.c();
            sn = -g.s();

            drot(
                &mut b.subcolumn_mut(i - 1, 0, i + 1),
                &mut b.subcolumn_mut(i, 0, i + 1),
                cs,
                sn,
            )?;

            drot(
                &mut a.column_mut(i - 1),
                &mut a.column_mut(i),
                cs,
                sn,
            )?;

            if let Some(v_mat) = v {
                drot(
                    &mut v_mat.column_mut(i - 1),
                    &mut v_mat.column_mut(i),
                    cs,
                    sn,
                )?;
            }

            i -= 1;
        }
    }

    Ok(())
}