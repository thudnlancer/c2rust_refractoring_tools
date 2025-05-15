use gsl::{
    cblas::CblasIndex,
    complex::{Complex, ComplexF64},
    eigen::herm::HermitianEigenWorkspace,
    error::{Error, Result},
    linalg::hermtd::{hermtd_decomp, hermtd_unpack_T},
    matrix::MatrixComplexF64,
    vector::VectorF64,
    VectorComplexF64,
};
use libc::{c_double, c_int, c_ulong};
use std::f64;

const GSL_SUCCESS: c_int = 0;
const GSL_ENOTSQR: c_int = 20;
const GSL_EBADLEN: c_int = 19;
const GSL_EINVAL: c_int = 4;
const GSL_ENOMEM: c_int = 8;

#[derive(Debug)]
pub struct HermitianEigensystem {
    workspace: HermitianEigenWorkspace,
}

impl HermitianEigensystem {
    pub fn new(n: usize) -> Result<Self> {
        if n == 0 {
            return Err(Error::new(
                GSL_EINVAL,
                "matrix dimension must be positive integer",
            ));
        }

        let workspace = HermitianEigenWorkspace::new(n)?;
        Ok(Self { workspace })
    }

    pub fn compute(
        &mut self,
        a: &mut MatrixComplexF64,
        eval: &mut VectorF64,
    ) -> Result<()> {
        if a.size1() != a.size2() {
            return Err(Error::new(
                GSL_ENOTSQR,
                "matrix must be square to compute eigenvalues",
            ));
        }

        if eval.len() != a.size1() {
            return Err(Error::new(
                GSL_EBADLEN,
                "eigenvalue vector must match matrix size",
            ));
        }

        let n = a.size1();
        if n == 1 {
            eval.set(0, a.get(0, 0).re());
            return Ok(());
        }

        let (mut d, mut sd) = {
            let (d, sd) = self.workspace.diagonals();
            (d.to_owned(), sd.to_owned())
        };

        let tau = self.workspace.tau();
        hermtd_decomp(a, tau)?;
        hermtd_unpack_T(a, &mut d, &mut sd)?;

        Self::chop_small_elements(&d, &mut sd);
        let mut b = n - 1;

        while b > 0 {
            let sd_b = sd.get(b - 1);
            if sd_b == 0.0 || sd_b.is_nan() {
                b -= 1;
            } else {
                let mut a = b - 1;
                while a > 0 {
                    if sd.get(a - 1) == 0.0 {
                        break;
                    }
                    a -= 1;
                }

                let n_block = b - a + 1;
                let d_block = &mut d.as_slice_mut()[a..=b];
                let sd_block = &mut sd.as_slice_mut()[a..b];

                Self::qr_step(d_block, sd_block, None, None);
                Self::chop_small_elements(&d_block, sd_block);
            }
        }

        eval.copy_from(&d)?;
        Ok(())
    }

    fn chop_small_elements(d: &VectorF64, sd: &mut VectorF64) {
        for i in 0..sd.len() {
            let sd_i = sd.get(i);
            let d_i = d.get(i);
            let d_ip1 = d.get(i + 1);

            if sd_i.abs() < f64::EPSILON * (d_i.abs() + d_ip1.abs()) {
                sd.set(i, 0.0);
            }
        }
    }

    fn qr_step(
        d: &mut [f64],
        sd: &mut [f64],
        gc: Option<&mut [f64]>,
        gs: Option<&mut [f64]>,
    ) {
        let n = d.len();
        let mu = Self::trailing_eigenvalue(d, sd);

        if 2.0 * f64::EPSILON * mu.abs() > d[0].abs() + sd[0].abs() {
            mu = 0.0;
        }

        let mut x = d[0] - mu;
        let mut z = sd[0];
        let mut ak = 0.0;
        let mut bk = 0.0;
        let mut zk = 0.0;
        let mut ap = d[0];
        let mut bp = sd[0];
        let mut aq = d[1];

        if n == 2 {
            let (c, s) = Self::create_givens(x, z);
            if let Some(gc) = gc {
                gc[0] = c;
            }
            if let Some(gs) = gs {
                gs[0] = s;
            }

            let ap1 = c * (c * ap - s * bp) + s * (s * aq - c * bp);
            let bp1 = c * (s * ap + c * bp) - s * (s * bp + c * aq);
            let aq1 = s * (s * ap + c * bp) + c * (s * bp + c * aq);

            d[0] = ap1;
            sd[0] = bp1;
            d[1] = aq1;
            return;
        }

        let mut bq = sd[1];

        for k in 0..n - 1 {
            let (c, s) = Self::create_givens(x, z);
            if let Some(gc) = gc {
                gc[k] = c;
            }
            if let Some(gs) = gs {
                gs[k] = s;
            }

            let bk1 = c * bk - s * zk;
            let ap1 = c * (c * ap - s * bp) + s * (s * aq - c * bp);
            let bp1 = c * (s * ap + c * bp) - s * (s * bp + c * aq);
            let zp1 = -s * bq;
            let aq1 = s * (s * ap + c * bp) + c * (s * bp + c * aq);
            let bq1 = c * bq;

            ak = ap1;
            bk = bp1;
            zk = zp1;
            ap = aq1;
            bp = bq1;

            if k < n - 2 {
                aq = d[k + 2];
            }
            if k < n - 3 {
                bq = sd[k + 2];
            }

            d[k] = ak;
            if k > 0 {
                sd[k - 1] = bk1;
            }
            if k < n - 2 {
                sd[k + 1] = bp;
            }

            x = bk;
            z = zk;
        }

        d[n - 1] = ap;
        sd[n - 2] = bk;
    }

    fn create_givens(a: f64, b: f64) -> (f64, f64) {
        if b == 0.0 {
            (1.0, 0.0)
        } else if b.abs() > a.abs() {
            let t = -a / b;
            let s = 1.0 / (1.0 + t * t).sqrt();
            (s * t, s)
        } else {
            let t = -b / a;
            let c = 1.0 / (1.0 + t * t).sqrt();
            (c, c * t)
        }
    }

    fn trailing_eigenvalue(d: &[f64], sd: &[f64]) -> f64 {
        let n = d.len();
        let ta = d[n - 2];
        let tb = d[n - 1];
        let tab = sd[n - 2];
        let dt = (ta - tb) / 2.0;

        if dt > 0.0 {
            tb - tab * (tab / (dt + (dt * dt + tab * tab).sqrt()))
        } else if dt == 0.0 {
            tb - tab.abs()
        } else {
            tb + tab * (tab / (-dt + (dt * dt + tab * tab).sqrt()))
        }
    }
}

impl Drop for HermitianEigensystem {
    fn drop(&mut self) {
        // Resources are automatically freed by gsl-rs destructors
    }
}