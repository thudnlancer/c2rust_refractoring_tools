use gsl::{
    blas::dscal,
    error::{Error, Result},
    matrix::Matrix,
    vector::Vector,
};
use libc::c_double;
use std::f64;

pub fn balance_matrix(a: &mut Matrix, d: &mut Vector) -> Result<()> {
    let n = a.size1();
    if n != d.size() {
        return Err(Error::BadLen);
    }

    d.set_all(1.0);

    let mut not_converged = true;
    while not_converged {
        not_converged = false;

        for i in 0..n {
            let mut row_norm = 0.0;
            let mut col_norm = 0.0;

            for j in 0..n {
                if j != i {
                    col_norm += f64::abs(a.get(j, i));
                    row_norm += f64::abs(a.get(i, j));
                }
            }

            if col_norm != 0.0 && row_norm != 0.0 {
                let mut g = row_norm / 2.0;
                let mut f = 1.0;
                let s = col_norm + row_norm;

                while col_norm < g {
                    f *= 2.0;
                    col_norm *= 4.0;
                }

                g = row_norm * 2.0;
                while col_norm > g {
                    f /= 2.0;
                    col_norm /= 4.0;
                }

                if row_norm + col_norm < 0.95 * s * f {
                    not_converged = true;
                    g = 1.0 / f;

                    let mut row = a.row_mut(i);
                    dscal(g, &mut row)?;

                    let mut col = a.column_mut(i);
                    dscal(f, &mut col)?;

                    d.set(i, d.get(i) * f);
                }
            }
        }
    }

    Ok(())
}

pub fn balance_accum(a: &mut Matrix, d: &Vector) -> Result<()> {
    let n = a.size1();
    if n != d.size() {
        return Err(Error::BadLen);
    }

    for i in 0..n {
        let s = d.get(i);
        let mut row = a.row_mut(i);
        dscal(s, &mut row)?;
    }

    Ok(())
}