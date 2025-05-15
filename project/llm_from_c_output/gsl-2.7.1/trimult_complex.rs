use ndarray::{Array2, ArrayView2, ArrayViewMut2, Axis};
use num_complex::Complex64;
use std::f64;

const CROSSOVER_TRIMULT: usize = 64;

#[derive(Debug, PartialEq)]
pub enum CBLAS_UPLO {
    Upper,
    Lower,
}

#[derive(Debug)]
pub enum LinalgError {
    NotSquare,
    Other(&'static str),
}

pub fn gsl_linalg_complex_tri_lhl(l: &mut Array2<Complex64>) -> Result<(), LinalgError> {
    triangular_multherm_l3(CBLAS_UPLO::Lower, l)
}

pub fn gsl_linalg_complex_tri_ul(lu: &mut Array2<Complex64>) -> Result<(), LinalgError> {
    triangular_mult_l3(CBLAS_UPLO::Upper, lu)
}

fn triangular_multherm_l2(uplo: CBLAS_UPLO, t: &mut Array2<Complex64>) -> Result<(), LinalgError> {
    let n = t.nrows();

    if n != t.ncols() {
        return Err(LinalgError::NotSquare);
    }

    if uplo == CBLAS_UPLO::Upper {
        // Upper case not implemented
    } else {
        for i in 0..n {
            let tii = t[(i, i)];
            let z0 = tii;

            if i < n - 1 {
                let v = t.slice(s![i+1.., i]);
                let norm = v.mapv(|x| x.norm_sqr()).sum().sqrt();

                t[(i, i)] = Complex64::new(tii.norm_sqr() + norm * norm, 0.0);

                if i > 0 {
                    let mut w = t.slice_mut(s![i, ..i]);
                    let m = t.slice(s![i+1.., ..i]);
                    
                    complex_conj_vector(&mut w);
                    
                    // Compute w = z0 * w + m^H * v
                    let mut temp = Array2::zeros((1, i));
                    ndarray::linalg::general_mat_vec_mul(
                        Complex64::new(1.0, 0.0),
                        &m.t(),
                        &v,
                        Complex64::new(0.0, 0.0),
                        &mut temp,
                    );
                    w += &temp;
                    
                    complex_conj_vector(&mut w);
                }
            } else {
                let mut w = t.slice_mut(s![i, ..]);
                w *= z0.re;
            }

            t[(i, i)].im = 0.0;
        }
    }

    Ok(())
}

fn triangular_multherm_l3(uplo: CBLAS_UPLO, t: &mut Array2<Complex64>) -> Result<(), LinalgError> {
    let n = t.nrows();

    if n != t.ncols() {
        return Err(LinalgError::NotSquare);
    } else if n <= CROSSOVER_TRIMULT {
        return triangular_multherm_l2(uplo, t);
    } else {
        let n1 = split_complex(n);
        let n2 = n - n1;
        
        let (mut t11, mut t12, mut t21, mut t22) = (
            t.slice_mut(s![..n1, ..n1]),
            t.slice_mut(s![..n1, n1..]),
            t.slice_mut(s![n1.., ..n1]),
            t.slice_mut(s![n1.., n1..]),
        );

        triangular_multherm_l3(uplo, &mut t11.to_owned())?;

        if uplo == CBLAS_UPLO::Lower {
            // t11 += t21^H * t21
            let t21_t = t21.t().to_owned();
            t11 += &t21_t.dot(&t21);

            // t21 = t22^H * t21
            let t22_t = t22.t().to_owned();
            t21.assign(&t22_t.dot(&t21));
        } else {
            // t11 += t12 * t12^H
            let t12_h = t12.t().mapv(|x| x.conj());
            t11 += &t12.dot(&t12_h);

            // t12 = t12 * t22^H
            let t22_h = t22.t().mapv(|x| x.conj());
            t12.assign(&t12.dot(&t22_h));
        }

        triangular_multherm_l3(uplo, &mut t22.to_owned())?;

        Ok(())
    }
}

fn triangular_mult_l2(uplo: CBLAS_UPLO, lu: &mut Array2<Complex64>) -> Result<(), LinalgError> {
    let n = lu.nrows();

    if n != lu.ncols() {
        return Err(LinalgError::NotSquare);
    } else if n == 1 {
        return Ok(());
    }

    if uplo == CBLAS_UPLO::Upper {
        for i in 0..n {
            let aii = lu[(i, i)];
            let uii = aii;

            if i < n - 1 {
                let lb = lu.slice(s![i+1.., i]);
                let ur = lu.slice(s![i, i+1..]);
                let dot = lb.dot(&ur);

                lu[(i, i)] = aii + dot;

                if i > 0 {
                    let u_tr = lu.slice(s![..i, i+1..]);
                    let l_bl = lu.slice(s![i+1.., ..i]);
                    let mut ut = lu.slice_mut(s![i, ..i]);
                    let mut ll = lu.slice_mut(s![i, ..i]);

                    // ll = uii * ll + l_bl^T * ur
                    let temp = l_bl.t().dot(&ur);
                    ll *= uii;
                    ll += &temp;

                    // ut = ut + u_tr * lb
                    let temp = u_tr.dot(&lb);
                    ut += &temp;
                }
            } else {
                let mut v = lu.slice_mut(s![i, ..i]);
                v *= uii;
            }
        }
    } else {
        // Lower case not implemented
    }

    Ok(())
}

fn triangular_mult_l3(uplo: CBLAS_UPLO, a: &mut Array2<Complex64>) -> Result<(), LinalgError> {
    let n = a.nrows();

    if n != a.ncols() {
        return Err(LinalgError::NotSquare);
    } else if n <= CROSSOVER_TRIMULT {
        return triangular_mult_l2(uplo, a);
    } else {
        let n1 = split_complex(n);
        let n2 = n - n1;
        
        let (mut a11, mut a12, mut a21, mut a22) = (
            a.slice_mut(s![..n1, ..n1]),
            a.slice_mut(s![..n1, n1..]),
            a.slice_mut(s![n1.., ..n1]),
            a.slice_mut(s![n1.., n1..]),
        );

        triangular_mult_l3(uplo, &mut a11.to_owned())?;

        if uplo == CBLAS_UPLO::Lower {
            // Lower case not implemented
        } else {
            // a11 += a12 * a21
            a11 += &a12.dot(&a21);

            // a12 = a12 * l22
            let l22 = a22.triangular_lower();
            a12.assign(&a12.dot(&l22));

            // a21 = u22 * a21
            let u22 = a22.triangular_upper();
            a21.assign(&u22.dot(&a21));
        }

        triangular_mult_l3(uplo, &mut a22.to_owned())?;

        Ok(())
    }
}

fn complex_conj_vector(v: &mut ArrayViewMut1<Complex64>) {
    v.mapv_inplace(|x| x.conj());
}

fn split_complex(n: usize) -> usize {
    n / 2
}

// Helper functions for triangular matrices
trait Triangular {
    fn triangular_lower(&self) -> Array2<Complex64>;
    fn triangular_upper(&self) -> Array2<Complex64>;
}

impl Triangular for Array2<Complex64> {
    fn triangular_lower(&self) -> Array2<Complex64> {
        let mut res = self.clone();
        for i in 0..res.nrows() {
            for j in i+1..res.ncols() {
                res[(i, j)] = Complex64::new(0.0, 0.0);
            }
            res[(i, i)] = Complex64::new(1.0, 0.0);
        }
        res
    }

    fn triangular_upper(&self) -> Array2<Complex64> {
        let mut res = self.clone();
        for i in 0..res.nrows() {
            for j in 0..i {
                res[(i, j)] = Complex64::new(0.0, 0.0);
            }
        }
        res
    }
}