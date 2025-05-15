use gsl::{
    bspline::BSpLineWorkspace,
    error::{Error, Result},
    linalg::{QRDecomp, QRLSSolve},
    matrix::Matrix,
    vector::Vector,
};
use libc::c_double;
use std::f64;

pub fn bspline_greville_abscissa(i: usize, w: &BSpLineWorkspace) -> c_double {
    let stride = w.knots().stride();
    let km1 = w.km1();
    let mut data = w.knots().data().as_ptr();

    unsafe {
        data = data.add((i + 1) * stride);
    }

    let km1 = if km1 == 0 { 2 } else { km1 };
    let data = if km1 == 0 {
        unsafe { data.sub(stride) }
    } else {
        data
    };

    let slice = unsafe { std::slice::from_raw_parts(data, km1) };
    slice.iter().sum::<f64>() / km1 as f64
}

pub fn bspline_knots_greville(
    abscissae: &Vector,
    w: &mut BSpLineWorkspace,
    abserr: Option<&mut f64>,
) -> Result<()> {
    if w.k() < 2 {
        return Err(Error::EINVAL("w->k must be at least 2"));
    } else if abscissae.size() < 2 {
        return Err(Error::EINVAL("abscissae->size must be at least 2"));
    } else if w.nbreak() != abscissae.size() - w.k() + 2 {
        return Err(Error::EINVAL(
            "w->nbreak must equal abscissae->size - w->k + 2",
        ));
    }

    let s = if w.nbreak() == 2 {
        let a = abscissae.get(0)?;
        let b = abscissae.get(abscissae.size() - 1)?;
        w.knots_uniform(a, b)?
    } else {
        let km2 = w.k() - 2;
        let m = abscissae.size() - 2;
        let n = w.nbreak() - 2;
        let invkm1 = 1.0 / w.km1() as f64;

        let mut storage = vec![0.0; m * n + 2 * n + 2 * m];
        let mut a = Matrix::view_array(&mut storage, m, n)?;
        let mut tau = Vector::view_array(&mut storage[m * n..m * n + n])?;
        let mut b = Vector::view_array(&mut storage[m * n + n..m * n + n + m])?;
        let mut x = Vector::view_array(&mut storage[m * n + n + m..m * n + n + m + n])?;
        let mut r = Vector::view_array(&mut storage[m * n + n + m + n..])?;

        for j in 0..n {
            for i in 0..=km2 {
                a.set(i + j, j, invkm1)?;
            }
        }

        for i in 0..m {
            b.set(i, abscissae.get(i + 1)?)?;
        }

        for i in 0..km2 {
            let v = b.get_mut(i)?;
            *v -= (1.0 - (i + 1) as f64 * invkm1) * abscissae.get(0)?;
        }

        for i in 0..km2 {
            let v = b.get_mut(m - km2 + i)?;
            *v -= (i + 1) as f64 * invkm1 * abscissae.get(abscissae.size() - 1)?;
        }

        QRDecomp::new(&mut a, &mut tau)?;
        QRLSSolve::new(&a, &tau, &b, &mut x, &mut r)?;

        let mut x_full = Vector::new(x.size() + 2)?;
        x_full.set(0, abscissae.get(0)?)?;
        x_full.set(x_full.size() - 1, abscissae.get(abscissae.size() - 1)?)?;
        for i in 0..x.size() {
            x_full.set(i + 1, x.get(i)?)?;
        }

        w.knots(&x_full)?
    };

    if s.is_ok() && abserr.is_some() {
        let mut err = 0.0;
        for i in 1..abscissae.size() - 1 {
            err += f64::abs(bspline_greville_abscissa(i, w) - abscissae.get(i)?);
        }
        *abserr.unwrap() = err;
    }

    s
}