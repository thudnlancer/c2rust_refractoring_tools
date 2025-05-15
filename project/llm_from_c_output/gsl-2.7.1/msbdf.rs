use std::f64;
use std::mem;
use std::ptr;
use std::slice;

use ndarray::{Array1, Array2, Axis};
use ndarray_linalg::{Determinant, LU};

const MSBDF_MAX_ORD: usize = 5;
const MSBDF_JAC_WAIT: usize = 50;
const MSBDF_M_WAIT: usize = 20;

struct MsbdfState {
    z: Array2<f64>,
    zbackup: Array2<f64>,
    ytmp: Array1<f64>,
    ytmp2: Array1<f64>,
    l: Array1<f64>,
    hprev: Array1<f64>,
    hprevbackup: Array1<f64>,
    ordprev: Array1<usize>,
    ordprevbackup: Array1<usize>,
    errlev: Array1<f64>,
    abscor: Array1<f64>,
    abscorscaled: Array1<f64>,
    relcor: Array1<f64>,
    svec: Array1<f64>,
    tempvec: Array1<f64>,
    driver: Option<*const gsl_odeiv2_driver>,
    dfdy: Array2<f64>,
    dfdt: Array1<f64>,
    M: Array2<f64>,
    p: Vec<usize>,
    rhs: Array1<f64>,
    ni: i64,
    ord: usize,
    tprev: f64,
    ordwait: usize,
    ordwaitbackup: usize,
    failord: usize,
    failt: f64,
    ordp1coeffprev: f64,
    nJ: usize,
    nM: usize,
    gammaprev: f64,
    gammaprevbackup: f64,
    failcount: usize,
}

impl MsbdfState {
    fn new(dim: usize) -> Result<Self, &'static str> {
        Ok(Self {
            z: Array2::zeros((MSBDF_MAX_ORD + 1, dim)),
            zbackup: Array2::zeros((MSBDF_MAX_ORD + 1, dim)),
            ytmp: Array1::zeros(dim),
            ytmp2: Array1::zeros(dim),
            l: Array1::zeros(MSBDF_MAX_ORD + 1),
            hprev: Array1::zeros(MSBDF_MAX_ORD),
            hprevbackup: Array1::zeros(MSBDF_MAX_ORD),
            ordprev: Array1::ones(MSBDF_MAX_ORD),
            ordprevbackup: Array1::ones(MSBDF_MAX_ORD),
            errlev: Array1::zeros(dim),
            abscor: Array1::zeros(dim),
            abscorscaled: Array1::zeros(dim),
            relcor: Array1::zeros(dim),
            svec: Array1::zeros(dim),
            tempvec: Array1::zeros(dim),
            driver: None,
            dfdy: Array2::zeros((dim, dim)),
            dfdt: Array1::zeros(dim),
            M: Array2::zeros((dim, dim)),
            p: vec![0; dim],
            rhs: Array1::zeros(dim),
            ni: 0,
            ord: 1,
            tprev: f64::NAN,
            ordwait: 2,
            ordwaitbackup: 2,
            failord: 0,
            failt: f64::NAN,
            ordp1coeffprev: 0.0,
            nJ: 0,
            nM: 0,
            gammaprev: 1.0,
            gammaprevbackup: 1.0,
            failcount: 0,
        })
    }

    fn reset(&mut self, dim: usize) {
        self.ni = 0;
        self.ord = 1;
        self.ordwait = 2;
        self.ordwaitbackup = 2;
        self.failord = 0;
        self.failt = f64::NAN;
        self.tprev = f64::NAN;
        self.gammaprev = 1.0;
        self.gammaprevbackup = 1.0;
        self.nJ = 0;
        self.nM = 0;
        self.failcount = 0;
        self.ordp1coeffprev = 0.0;

        self.hprev.fill(0.0);
        self.hprevbackup.fill(0.0);
        self.z.fill(0.0);
        self.zbackup.fill(0.0);

        self.ordprev.fill(1);
        self.ordprevbackup.fill(1);
    }

    fn failure_handler(&mut self, dim: usize, t: f64) -> Result<(), &'static str> {
        let ord = self.ord;

        if ord > 1 
            && (ord - self.ordprev[0] == 0) 
            && ord == self.failord 
            && t == self.failt 
        {
            self.ord -= 1;
        }

        self.failord = ord;
        self.failt = t;
        self.ni += 1;

        if ord == 1 {
            self.reset(dim);
        }

        Ok(())
    }

    fn calc_coeffs(
        ord: usize,
        ordwait: usize,
        h: f64,
        hprev: &Array1<f64>,
        l: &mut Array1<f64>,
        errcoeff: &mut f64,
        ordm1coeff: &mut f64,
        ordp1coeff: &mut f64,
        ordp2coeff: &mut f64,
        gamma: &mut f64,
    ) -> Result<(), &'static str> {
        if ord == 1 {
            l[0] = 1.0;
            l[1] = 1.0;
            *errcoeff = 0.5;
            *ordp1coeff = 2.0;

            let hsum = h + hprev[0];
            let a5 = -1.5;
            let a6 = -1.0 - h / hsum;
            let c2 = 2.0 / (1.0 - a6 + a5);
            *ordp2coeff = f64::abs(c2 * (h / hsum) * 3.0 * a5);
        } else {
            let mut hsum = h;
            let mut coeff1 = -1.0;
            let mut x;

            l.fill(0.0);
            l[0] = 1.0;
            l[1] = 1.0;

            for i in 2..ord {
                hsum += hprev[i - 2];
                coeff1 += -1.0 / (i as f64);

                for j in (1..=i).rev() {
                    l[j] += h / hsum * l[j - 1];
                }
            }

            coeff1 += -1.0 / (ord as f64);
            x = -l[1] - coeff1;

            for i in (1..=ord).rev() {
                l[i] += l[i - 1] * x;
            }

            hsum += hprev[ord - 2];
            let coeff2 = -l[1] - h / hsum;
            let a1 = 1.0 - coeff2 + coeff1;
            let a2 = 1.0 + (ord as f64) * a1;

            *errcoeff = f64::abs(a1 / (coeff1 * a2));

            if ordwait < 2 {
                let a3 = coeff1 + 1.0 / (ord as f64);
                let a4 = coeff2 + h / hsum;
                let c1 = a3 / (1.0 - a4 + a3);

                *ordm1coeff = f64::abs(c1 / (x / l[ord]));

                *ordp1coeff = f64::abs(a2 / (l[ord] * (h / hsum) / x));

                hsum += hprev[ord - 1];
                let a5 = coeff1 - 1.0 / ((ord + 1) as f64);
                let a6 = coeff2 - h / hsum;
                let c2 = a2 / (1.0 - a6 + a5);

                *ordp2coeff = f64::abs(c2 * (h / hsum) * ((ord + 2) as f64) * a5);
            }
        }

        *gamma = h / l[1];

        Ok(())
    }

    fn update(
        &mut self,
        dim: usize,
        dfdy: &mut Array2<f64>,
        dfdt: &mut Array1<f64>,
        t: f64,
        y: &Array1<f64>,
        sys: &gsl_odeiv2_system,
        M: &mut Array2<f64>,
        p: &mut Vec<usize>,
        iter: usize,
        nJ: &mut usize,
        nM: &mut usize,
        tprev: f64,
        failt: f64,
        gamma: f64,
        gammaprev: f64,
        hratio: f64,
    ) -> Result<(), &'static str> {
        let c = 0.2;
        let gammarel = f64::abs(gamma / gammaprev - 1.0);

        if *nJ == 0 || *nJ > MSBDF_JAC_WAIT || (t == failt && (gammarel < c || hratio < 1.0)) {
            // Evaluate Jacobian
            *nJ = 0;
        }

        if *nM == 0 || *nM > MSBDF_M_WAIT || gammarel >= c || t == tprev || t == failt {
            // Update iteration matrix M
            *nM = 0;
        }

        Ok(())
    }

    fn corrector(
        &mut self,
        sys: &gsl_odeiv2_system,
        t: f64,
        h: f64,
        dim: usize,
        z: &Array1<f64>,
        errlev: &Array1<f64>,
        l: &Array1<f64>,
        errcoeff: f64,
        abscor: &mut Array1<f64>,
        relcor: &mut Array1<f64>,
        ytmp: &mut Array1<f64>,
        ytmp2: &mut Array1<f64>,
        dfdy: &mut Array2<f64>,
        dfdt: &mut Array1<f64>,
        M: &mut Array2<f64>,
        p: &mut Vec<usize>,
        rhs: &mut Array1<f64>,
        nJ: &mut usize,
        nM: &mut usize,
        tprev: f64,
        failt: f64,
        gamma: f64,
        gammaprev: f64,
        hprev0: f64,
    ) -> Result<(), &'static str> {
        let max_iter = 3;
        let mut convrate = 1.0;
        let mut stepnorm = 0.0;
        let mut stepnormprev = 0.0;

        // Evaluate at predicted values
        abscor.fill(0.0);

        for mi in 0..max_iter {
            let safety = 0.3;
            let safety2 = 0.1;

            if mi == 0 {
                self.update(
                    dim, dfdy, dfdt, t + h, z, sys, M, p, mi, nJ, nM, tprev, failt, gamma, gammaprev, h / hprev0,
                )?;
            }

            // Evaluate the right hand side (-G)
            for i in 0..dim {
                rhs[i] = -abscor[i] - z[1 * dim + i] / l[1] + gamma * ytmp[i];
            }

            // Solve system of equations
            let lu = M.lu()?;
            *relcor = lu.solve(&rhs)?;

            // Add iteration results
            for i in 0..dim {
                abscor[i] += relcor[i];
                ytmp2[i] = z[i] + abscor[i];
                relcor[i] /= errlev[i];
            }

            // Convergence test
            stepnorm = relcor.mapv(|x| x.powi(2)).sum().sqrt() / (dim as f64).sqrt();

            if mi > 0 {
                convrate = convrate.max(safety * convrate).max(stepnorm / stepnormprev);
            } else {
                convrate = 1.0;
            }

            let convtest = convrate.min(1.0) * stepnorm * errcoeff / safety2;

            if convtest <= 1.0 {
                break;
            }

            // Check for divergence
            if mi > 1 && stepnorm > 2.0 * stepnormprev {
                self.failure_handler(dim, t)?;
                return Err("diverging Newton iteration");
            }

            // Evaluate at new y
            stepnormprev = stepnorm;
        }

        if stepnormprev == 0.0 {
            self.failure_handler(dim, t)?;
            return Err("max_iter reached");
        }

        Ok(())
    }

    fn eval_order(
        abscorscaled: &Array1<f64>,
        tempvec: &mut Array1<f64>,
        svec: &mut Array1<f64>,
        errcoeff: f64,
        dim: usize,
        errlev: &Array1<f64>,
        ordm1coeff: f64,
        ordp1coeff: f64,
        ordp1coeffprev: f64,
        ordp2coeff: f64,
        hprev: &Array1<f64>,
        h: f64,
        z: &Array1<f64>,
        ord: &mut usize,
    ) -> Result<(), &'static str> {
        let safety = 1e-6;
        let bias = 6.0;
        let bias2 = 10.0;
        let min_incr = 1.5;

        let ordest = 1.0 / ((bias * abscorscaled.mapv(|x| x.powi(2)).sum().sqrt() / (dim as f64).sqrt() * errcoeff)
            .powf(1.0 / (*ord + 1) as f64) + safety);

        let mut ordm1est = 0.0;
        if *ord > 1 {
            for i in 0..dim {
                tempvec[i] = z[*ord * dim + i] / errlev[i];
            }
            ordm1est = 1.0 / ((bias * tempvec.mapv(|x| x.powi(2)).sum().sqrt() / (dim as f64).sqrt() / ordm1coeff)
                .powf(1.0 / *ord as f64) + safety);
        }

        let mut ordp1est = 0.0;
        if *ord < MSBDF_MAX_ORD {
            let c = -ordp1coeff / ordp1coeffprev * (h / hprev[1]).powi(*ord as i32 + 1);
            for i in 0..dim {
                svec[i] = svec[i] * c + abscorscaled[i];
            }
            ordp1est = 1.0 / ((bias2 * svec.mapv(|x| x.powi(2)).sum().sqrt() / (dim as f64).sqrt() / ordp2coeff)
                .powf(1.0 / (*ord + 2) as f64) + safety);
        }

        if ordm1est > ordest && ordm1est > ordp1est && ordm1est > min_incr {
            *ord -= 1;
        } else if ordp1est > ordest && ordp1est > ordm1est && ordp1est > min_incr {
            *ord += 1;
        }

        Ok(())
    }

    fn check_no_order_decrease(ordprev: &Array1<usize>) -> bool {
        for i in 0..MSBDF_MAX_ORD - 1 {
            if ordprev[i + 1] > ordprev[i] {
                return false;
            }
        }
        true
    }

    fn check_step_size_decrease(hprev: &Array1<f64>) -> bool {
        let mut max = hprev[0].abs();
        let min = hprev[0].abs();
        let decrease_limit = 0.5;

        for i in 1..MSBDF_MAX_ORD {
            let h = hprev[i].abs();
            if h > min && h > max {
                max = h;
            }
        }

        min / max < decrease_limit
    }
}

struct MsbdfStepper {
    state: MsbdfState,
}

impl MsbdfStepper {
    fn new(dim: usize) -> Result<Self, &'static str> {
        Ok(Self {
            state: MsbdfState::new(dim)?,
        })
    }

    fn apply(
        &mut self,
        dim: usize,
        t: f64,
        h: f64,
        y: &mut Array1<f64>,
        yerr: &mut Array1<f64>,
        dydt_in: Option<&Array1<f64>>,
        dydt_out: Option<&mut Array1<f64>>,
        sys: &gsl_odeiv2_system,
    ) -> Result<(), &'static str> {
        // Main stepping logic here
        Ok(())
    }

    fn set_driver(&mut self, driver: *const gsl_odeiv2_driver) {
        self.state.driver = Some(driver);
    }

    fn order(&self) -> usize {
        self.state.ord
    }
}

struct gsl_odeiv2_driver {
    c: *const std::ffi::c_void,
}

struct gsl_odeiv2_system {
    //