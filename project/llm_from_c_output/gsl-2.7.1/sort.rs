use std::cmp::Ordering;
use num_complex::Complex64;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum EigenError {
    #[error("eigenvector matrix must be square")]
    NotSquare,
    #[error("eigenvalues must match eigenvector matrix")]
    BadLength,
    #[error("invalid sort type")]
    InvalidSortType,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EigenSortType {
    ValAsc,
    ValDesc,
    AbsAsc,
    AbsDesc,
}

fn complex_less(a: Complex64, b: Complex64) -> bool {
    if (a.re - b.re).abs() < f64::EPSILON {
        a.im < b.im
    } else {
        a.re < b.re
    }
}

pub fn eigen_symmv_sort(
    eval: &mut [f64],
    evec: &mut [Vec<f64>],
    sort_type: EigenSortType,
) -> Result<(), EigenError> {
    if evec.len() != evec[0].len() {
        return Err(EigenError::NotSquare);
    } else if eval.len() != evec.len() {
        return Err(EigenError::BadLength);
    }

    let n = eval.len();
    for i in 0..n - 1 {
        let mut k = i;
        let mut ek = eval[i];

        for j in i + 1..n {
            let ej = eval[j];
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
            eval.swap(i, k);
            for row in evec.iter_mut() {
                row.swap(i, k);
            }
        }
    }

    Ok(())
}

pub fn eigen_hermv_sort(
    eval: &mut [f64],
    evec: &mut [Vec<Complex64>],
    sort_type: EigenSortType,
) -> Result<(), EigenError> {
    if evec.len() != evec[0].len() {
        return Err(EigenError::NotSquare);
    } else if eval.len() != evec.len() {
        return Err(EigenError::BadLength);
    }

    let n = eval.len();
    for i in 0..n - 1 {
        let mut k = i;
        let mut ek = eval[i];

        for j in i + 1..n {
            let ej = eval[j];
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
            eval.swap(i, k);
            for row in evec.iter_mut() {
                row.swap(i, k);
            }
        }
    }

    Ok(())
}

pub fn eigen_nonsymmv_sort(
    eval: &mut [Complex64],
    evec: Option<&mut [Vec<Complex64>]>,
    sort_type: EigenSortType,
) -> Result<(), EigenError> {
    if let Some(evec) = evec {
        if evec.len() != evec[0].len() {
            return Err(EigenError::NotSquare);
        } else if eval.len() != evec.len() {
            return Err(EigenError::BadLength);
        }
    }

    let n = eval.len();
    for i in 0..n - 1 {
        let mut k = i;
        let mut ek = eval[i];

        for j in i + 1..n {
            let ej = eval[j];
            let test = match sort_type {
                EigenSortType::AbsAsc => ej.norm() < ek.norm(),
                EigenSortType::AbsDesc => ej.norm() > ek.norm(),
                EigenSortType::ValAsc => complex_less(ej, ek),
                EigenSortType::ValDesc => complex_less(ek, ej),
            };

            if test {
                k = j;
                ek = ej;
            }
        }

        if k != i {
            eval.swap(i, k);
            if let Some(evec) = evec {
                for row in evec.iter_mut() {
                    row.swap(i, k);
                }
            }
        }
    }

    Ok(())
}

pub fn eigen_gensymmv_sort(
    eval: &mut [f64],
    evec: &mut [Vec<f64>],
    sort_type: EigenSortType,
) -> Result<(), EigenError> {
    eigen_symmv_sort(eval, evec, sort_type)
}

pub fn eigen_genhermv_sort(
    eval: &mut [f64],
    evec: &mut [Vec<Complex64>],
    sort_type: EigenSortType,
) -> Result<(), EigenError> {
    eigen_hermv_sort(eval, evec, sort_type)
}

pub fn eigen_genv_sort(
    alpha: &mut [Complex64],
    beta: &mut [f64],
    evec: &mut [Vec<Complex64>],
    sort_type: EigenSortType,
) -> Result<(), EigenError> {
    if evec.len() != evec[0].len() {
        return Err(EigenError::NotSquare);
    } else if alpha.len() != evec.len() || beta.len() != evec.len() {
        return Err(EigenError::BadLength);
    }

    let n = alpha.len();
    for i in 0..n - 1 {
        let mut k = i;
        let ak = alpha[i];
        let bk = beta[i];
        let mut ek = if bk < f64::EPSILON {
            Complex64::new(
                if ak.re.is_sign_positive() { f64::INFINITY } else { f64::NEG_INFINITY },
                if ak.im.is_sign_positive() { f64::INFINITY } else { f64::NEG_INFINITY },
            )
        } else {
            ak / bk
        };

        for j in i + 1..n {
            let aj = alpha[j];
            let bj = beta[j];
            let ej = if bj < f64::EPSILON {
                Complex64::new(
                    if aj.re.is_sign_positive() { f64::INFINITY } else { f64::NEG_INFINITY },
                    if aj.im.is_sign_positive() { f64::INFINITY } else { f64::NEG_INFINITY },
                )
            } else {
                aj / bj
            };

            let test = match sort_type {
                EigenSortType::AbsAsc => ej.norm() < ek.norm(),
                EigenSortType::AbsDesc => ej.norm() > ek.norm(),
                _ => return Err(EigenError::InvalidSortType),
            };

            if test {
                k = j;
                ek = ej;
            }
        }

        if k != i {
            alpha.swap(i, k);
            beta.swap(i, k);
            for row in evec.iter_mut() {
                row.swap(i, k);
            }
        }
    }

    Ok(())
}