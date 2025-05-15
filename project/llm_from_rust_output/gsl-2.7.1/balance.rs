use gsl::{
    blas::Level1,
    error::{Error, Result},
    matrix::Matrix,
    vector::Vector,
    Finite,
};

pub fn balance_columns(a: &mut Matrix, d: &mut Vector) -> Result<()> {
    let n = a.size2();
    if d.len() != n {
        return Err(Error::InvalidArgument("length of D must match second dimension of A"));
    }

    d.set_all(1.0);

    for j in 0..n {
        let mut a_j = a.column_mut(j);
        let mut s = a_j.asum();
        let mut f = 1.0;

        if s == 0.0 || !s.is_finite() {
            d.set(j, f)?;
        } else {
            while s > 1.0 {
                s /= 2.0;
                f *= 2.0;
            }
            while s < 0.5 {
                s *= 2.0;
                f /= 2.0;
            }

            d.set(j, f)?;
            if f != 1.0 {
                a_j.scale(1.0 / f);
            }
        }
    }

    Ok(())
}