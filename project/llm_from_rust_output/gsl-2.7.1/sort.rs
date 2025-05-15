use gsl::{Vector, Matrix, VectorComplex, MatrixComplex, Complex, EigenSortType};
use gsl::error::{Result, Error};
use gsl::value::Value;
use std::f64;

fn complex_less(a: Complex, b: Complex) -> bool {
    if gsl::fcmp(a.dat[0], b.dat[0], f64::EPSILON) == 0 {
        a.dat[1] < b.dat[1]
    } else {
        a.dat[0] < b.dat[0]
    }
}

pub fn gsl_eigen_symmv_sort(
    eval: &mut Vector,
    evec: &mut Matrix,
    sort_type: EigenSortType,
) -> Result<()> {
    if evec.size1() != evec.size2() {
        return Err(Error::new(Value::NotSquare));
    }
    if eval.size() != evec.size1() {
        return Err(Error::new(Value::BadLength));
    }

    let n = eval.size();
    for i in 0..n-1 {
        let mut k = i;
        let mut ek = eval.get(i)?;
        for j in i+1..n {
            let ej = eval.get(j)?;
            let test = match sort_type {
                EigenSortType::ValAsc => ej < ek,
                EigenSortType::ValDesc => ej > ek,
                EigenSortType::AbsAsc => ej.abs() < ek.abs(),
                EigenSortType::AbsDesc => ej.abs() > ek.abs(),
            };
            if test {
                k = j;
                ek = ej;
            }
        }
        if k != i {
            eval.swap_elements(i, k)?;
            }
    }
    Ok(())
}

pub fn gsl_eigen_hermv_sort(
    eval: &mut Vector,
    evec: &mut MatrixComplex,
    sort_type: EigenSortType,
) -> Result<()> {
    if evec.size1() != evec.size2() {
        return Err(Error::new(Value::NotSquare));
    }
    if eval.size() != evec.size1() {
        return Err(Error::new(Value::BadLength));
    }

    let n = eval.size();
    for i in 0..n-1 {
        let mut k = i;
        let mut ek = eval.get(i)?;
        for j in i+1..n {
            let ej = eval.get(j)?;
            let test = match sort_type {
                EigenSortType::ValAsc => ej < ek,
                EigenSortType::ValDesc => ej > ek,
                EigenSortType::AbsAsc => ej.abs() < ek.abs(),
                EigenSortType::AbsDesc => ej.abs() > ek.abs(),
            };
            if test {
                k = j;
                ek = ej;
            }
        }
        if k != i {
            eval.swap_elements(i, k)?;
            evec.swap_columns(i, k)?;
        }
    }
    Ok(())
}

pub fn gsl_eigen_nonsymmv_sort(
    eval: &mut VectorComplex,
    evec: Option<&mut MatrixComplex>,
    sort_type: EigenSortType,
) -> Result<()> {
    if let Some(evec) = evec {
        if evec.size1() != evec.size2() {
            return Err(Error::new(Value::NotSquare));
        }
        if eval.size() != evec.size1() {
            return Err(Error::new(Value::BadLength));
        }
    }

    let n = eval.size();
    for i in 0..n-1 {
        let mut k = i;
        let mut ek = eval.get(i)?;
        for j in i+1..n {
            let ej = eval.get(j)?;
            let test = match sort_type {
                EigenSortType::AbsAsc => ej.abs() < ek.abs(),
                EigenSortType::AbsDesc => ej.abs() > ek.abs(),
                EigenSortType::ValAsc => complex_less(ej, ek),
                EigenSortType::ValDesc => complex_less(ek, ej),
            };
            if test {
                k = j;
                ek = ej;
            }
        }
        if k != i {
            eval.swap_elements(i, k)?;
            if let Some(evec) = evec {
                evec.swap_columns(i, k)?;
            }
        }
    }
    Ok(())
}

pub fn gsl_eigen_gensymmv_sort(
    eval: &mut Vector,
    evec: &mut Matrix,
    sort_type: EigenSortType,
) -> Result<()> {
    gsl_eigen_symmv_sort(eval, evec, sort_type)
}

pub fn gsl_eigen_genhermv_sort(
    eval: &mut Vector,
    evec: &mut MatrixComplex,
    sort_type: EigenSortType,
) -> Result<()> {
    gsl_eigen_hermv_sort(eval, evec, sort_type)
}

pub fn gsl_eigen_genv_sort(
    alpha: &mut VectorComplex,
    beta: &mut Vector,
    evec: &mut MatrixComplex,
    sort_type: EigenSortType,
) -> Result<()> {
    if evec.size1() != evec.size2() {
        return Err(Error::new(Value::NotSquare));
    }
    if alpha.size() != evec.size1() || beta.size() != evec.size1() {
        return Err(Error::new(Value::BadLength));
    }

    let n = alpha.size();
    for i in 0..n-1 {
        let mut k = i;
        let ak = alpha.get(i)?;
        let bk = beta.get(i)?;
        let ek = if bk < f64::EPSILON {
            Complex::new(
                if ak.re() >= 0.0 { f64::INFINITY } else { f64::NEG_INFINITY },
                if ak.im() >= 0.0 { f64::INFINITY } else { f64::NEG_INFINITY },
            )
        } else {
            ak / bk
        };

        for j in i+1..n {
            let aj = alpha.get(j)?;
            let bj = beta.get(j)?;
            let ej = if bj < f64::EPSILON {
                Complex::new(
                    if aj.re() >= 0.0 { f64::INFINITY } else { f64::NEG_INFINITY },
                    if aj.im() >= 0.0 { f64::INFINITY } else { f64::NEG_INFINITY },
                )
            } else {
                aj / bj
            };

            let test = match sort_type {
                EigenSortType::AbsAsc => ej.abs() < ek.abs(),
                EigenSortType::AbsDesc => ej.abs() > ek.abs(),
                _ => return Err(Error::new(Value::Invalid)),
            };

            if test {
                k = j;
            }
        }

        if k != i {
            alpha.swap_elements(i, k)?;
            beta.swap_elements(i, k)?;
            evec.swap_columns(i, k)?;
        }
    }
    Ok(())
}