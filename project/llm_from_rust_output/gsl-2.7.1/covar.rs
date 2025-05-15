use gsl::{
    ffi::{
        gsl_error, gsl_linalg_QRPT_decomp, gsl_matrix, gsl_matrix_alloc, gsl_matrix_free,
        gsl_matrix_get, gsl_matrix_memcpy, gsl_matrix_set, gsl_permutation, gsl_permutation_alloc,
        gsl_permutation_free, gsl_permutation_get, gsl_vector, gsl_vector_alloc, gsl_vector_free,
    },
    types::{Matrix, Permutation, Vector},
};
use libc::{c_double, c_int, size_t};
use std::ffi::CString;

const GSL_SUCCESS: c_int = 0;
const GSL_EBADLEN: c_int = 19;

fn gsl_multifit_nlinear_covar(
    J: &Matrix,
    epsrel: c_double,
    covar: &mut Matrix,
) -> Result<(), i32> {
    let m = J.size1();
    let n = J.size2();

    if m < n {
        let reason = CString::new("Jacobian be rectangular M x N with M >= N").unwrap();
        let file = CString::new("covar.c").unwrap();
        gsl_error(
            reason.as_ptr(),
            file.as_ptr(),
            51,
            GSL_EBADLEN,
        );
        return Err(GSL_EBADLEN);
    }

    if covar.size1() != covar.size2() || covar.size1() != n {
        let reason = CString::new("covariance matrix must be square and match second dimension of jacobian").unwrap();
        let file = CString::new("covar.c").unwrap();
        gsl_error(
            reason.as_ptr(),
            file.as_ptr(),
            56,
            GSL_EBADLEN,
        );
        return Err(GSL_EBADLEN);
    }

    let mut r = Matrix::new(m, n).ok_or(GSL_EBADLEN)?;
    let mut tau = Vector::new(n).ok_or(GSL_EBADLEN)?;
    let mut perm = Permutation::new(n).ok_or(GSL_EBADLEN)?;
    let mut norm = Vector::new(n).ok_or(GSL_EBADLEN)?;
    let mut signum: c_int = 0;

    gsl_matrix_memcpy(r.as_mut_ptr(), J.as_ptr());
    gsl_linalg_QRPT_decomp(
        r.as_mut_ptr(),
        tau.as_mut_ptr(),
        perm.as_mut_ptr(),
        &mut signum,
        norm.as_mut_ptr(),
    );

    let status = covar_QRPT(&mut r, &mut perm, epsrel, covar);

    Ok(status?)
}

fn covar_QRPT(
    r: &mut Matrix,
    perm: &mut Permutation,
    epsrel: c_double,
    covar: &mut Matrix,
) -> Result<(), i32> {
    let tolr = epsrel * r.get(0, 0).abs();
    let n = r.size2();
    let mut kmax = 0;

    for k in 0..n {
        let rkk = r.get(k, k);
        if rkk.abs() <= tolr {
            break;
        }

        r.set(k, k, 1.0 / rkk);

        for j in 0..k {
            let t = r.get(j, k) / rkk;
            r.set(j, k, 0.0);

            for i in 0..=j {
                let rik = r.get(i, k);
                let rij = r.get(i, j);
                r.set(i, k, rik - t * rij);
            }
        }

        kmax = k;
    }

    for k in 0..=kmax {
        for j in 0..k {
            let rjk = r.get(j, k);

            for i in 0..=j {
                let rij = r.get(i, j);
                let rik = r.get(i, k);
                r.set(i, j, rij + rjk * rik);
            }
        }

        let t = r.get(k, k);
        for i in 0..=k {
            let rik = r.get(i, k);
            r.set(i, k, t * rik);
        }
    }

    for j in 0..n {
        let pj = perm.get(j);

        for i in 0..=j {
            let pi = perm.get(i);
            let rij = if j > kmax {
                r.set(i, j, 0.0);
                0.0
            } else {
                r.get(i, j)
            };

            if pi > pj {
                r.set(pi, pj, rij);
            } else if pi < pj {
                r.set(pj, pi, rij);
            }
        }

        let rjj = r.get(j, j);
        covar.set(pj, pj, rjj);
    }

    for j in 0..n {
        for i in 0..j {
            let rji = r.get(j, i);
            covar.set(j, i, rji);
            covar.set(i, j, rji);
        }
    }

    Ok(())
}

mod gsl {
    pub mod ffi {
        // FFI bindings would be defined here
    }

    pub mod types {
        // Safe wrappers would be defined here
    }
}