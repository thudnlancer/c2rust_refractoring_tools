use std::f64;
use std::mem;
use std::ptr;
use std::slice;

use gsl::blas::dnrm2;
use gsl::error::{GSLResult, GSL_ERROR};
use gsl::odeiv2::{Driver, System, StepType};

const MSADAMS_MAX_ORD: usize = 12;

struct MSAdamsState {
    z: Vec<f64>,
    zbackup: Vec<f64>,
    ytmp: Vec<f64>,
    ytmp2: Vec<f64>,
    pc: Vec<f64>,
    l: Vec<f64>,
    hprev: Vec<f64>,
    hprevbackup: Vec<f64>,
    errlev: Vec<f64>,
    abscor: Vec<f64>,
    relcor: Vec<f64>,
    svec: Vec<f64>,
    tempvec: Vec<f64>,
    driver: Option<Box<Driver>>,
    
    ni: i64,
    ord: usize,
    ordprev: usize,
    ordprevbackup: usize,
    tprev: f64,
    ordwait: usize,
    ordwaitbackup: usize,
    failord: usize,
    failt: f64,
    ordm1coeff: f64,
    ordp1coeffprev: f64,
    failcount: usize,
}

impl MSAdamsState {
    fn new(dim: usize) -> GSLResult<Box<Self>> {
        let state = Box::new(MSAdamsState {
            z: vec![0.0; (MSADAMS_MAX_ORD + 1) * dim],
            zbackup: vec![0.0; (MSADAMS_MAX_ORD + 1) * dim],
            ytmp: vec![0.0; dim],
            ytmp2: vec![0.0; dim],
            pc: vec![0.0; MSADAMS_MAX_ORD + 1],
            l: vec![0.0; MSADAMS_MAX_ORD + 1],
            hprev: vec![0.0; MSADAMS_MAX_ORD],
            hprevbackup: vec![0.0; MSADAMS_MAX_ORD],
            errlev: vec![0.0; dim],
            abscor: vec![0.0; dim],
            relcor: vec![0.0; dim],
            svec: vec![0.0; dim],
            tempvec: vec![0.0; dim],
            driver: None,
            
            ni: 0,
            ord: 1,
            ordprev: 1,
            ordprevbackup: 1,
            tprev: f64::NAN,
            ordwait: 2,
            ordwaitbackup: 2,
            failord: 0,
            failt: f64::NAN,
            ordm1coeff: 0.0,
            ordp1coeffprev: 0.0,
            failcount: 0,
        });
        
        Ok(state)
    }
    
    fn reset(&mut self, dim: usize) -> GSLResult<()> {
        self.ni = 0;
        self.ord = 1;
        self.ordprev = 1;
        self.ordprevbackup = 1;
        self.ordwait = 2;
        self.ordwaitbackup = 2;
        self.failord = 0;
        self.failt = f64::NAN;
        self.failcount = 0;
        
        self.hprev.iter_mut().for_each(|x| *x = 0.0);
        self.z.iter_mut().for_each(|x| *x = 0.0);
        
        Ok(())
    }
    
    fn failure_handler(&mut self, dim: usize, t: f64) -> GSLResult<()> {
        let ord = self.ord;
        
        if ord > 1 && (ord - self.ordprev == 0) && 
           ord == self.failord && t == self.failt 
        {
            self.ord -= 1;
        }
        
        self.failord = ord;
        self.failt = t;
        self.ni += 1;
        
        if ord == 1 {
            self.reset(dim)?;
        }
        
        Ok(())
    }
    
    fn calc_coeffs(
        ord: usize,
        ordwait: usize,
        h: f64,
        hprev: &[f64],
        pc: &mut [f64],
        l: &mut [f64],
        errcoeff: &mut f64,
        ordm1coeff: &mut f64,
        ordp1coeff: &mut f64,
        ordp2coeff: &mut f64,
    ) -> GSLResult<()> {
        if ord == 1 {
            l[0] = 1.0;
            l[1] = 1.0;
            *errcoeff = 0.5;
            *ordp1coeff = 1.0;
            *ordp2coeff = 12.0;
        } else {
            let mut hsum = h;
            let mut st1 = 0.0;
            let mut st2 = 0.0;
            
            pc.iter_mut().for_each(|x| *x = 0.0);
            pc[0] = 1.0;
            
            for i in 1..ord {
                if i == ord - 1 && ordwait == 1 {
                    let mut s = 1;
                    *ordm1coeff = 0.0;
                    
                    for j in 0..ord-1 {
                        *ordm1coeff += s as f64 * pc[j] / (j + 2) as f64;
                        s = -s;
                    }
                    
                    *ordm1coeff = pc[ord - 2] / (ord as f64 * *ordm1coeff);
                }
                
                for j in (1..=i).rev() {
                    pc[j] += pc[j - 1] * h / hsum;
                }
                
                hsum += hprev[i - 1];
            }
            
            let mut s = 1;
            for i in 0..ord {
                st1 += s as f64 * pc[i] / (i + 1) as f64;
                s = -s;
            }
            
            let mut s = 1;
            for i in 0..ord {
                st2 += s as f64 * pc[i] / (i + 2) as f64;
                s = -s;
            }
            
            l.iter_mut().for_each(|x| *x = 0.0);
            l[0] = 1.0;
            
            for i in 1..=ord {
                l[i] = pc[i - 1] / (i as f64 * st1);
            }
            
            *errcoeff = (h / hsum) * (st2 / st1);
            
            if ordwait < 2 {
                let mut s = 1;
                *ordp1coeff = hsum / (h * l[ord]);
                
                *ordp2coeff = 0.0;
                
                for i in (1..=ord).rev() {
                    pc[i] += pc[i - 1] * (h / hsum);
                }
                
                for i in 0..=ord {
                    *ordp2coeff += s as f64 * pc[i] / (i + 2) as f64;
                    s = -s;
                }
                
                *ordp2coeff = (ord + 1) as f64 * st1 / *ordp2coeff;
            }
        }
        
        Ok(())
    }
    
    fn corrector(
        &mut self,
        sys: &System,
        t: f64,
        h: f64,
        dim: usize,
        z: &[f64],
        errlev: &[f64],
        l: &[f64],
        errcoeff: f64,
        abscor: &mut [f64],
        relcor: &mut [f64],
        ytmp: &mut [f64],
        ytmp2: &mut [f64],
    ) -> GSLResult<()> {
        const MAX_ITER: usize = 3;
        let mut convrate = 1.0;
        let mut stepnorm = 0.0;
        let mut stepnormprev = 0.0;
        
        sys.function(t + h, z, ytmp)?;
        
        abscor.iter_mut().for_each(|x| *x = 0.0);
        
        for mi in 0..MAX_ITER {
            const SAFETY: f64 = 0.3;
            const SAFETY2: f64 = 0.1;
            
            for i in 0..dim {
                ytmp[i] = ytmp[i] * h - z[1 * dim + i];
                ytmp[i] /= l[1];
                ytmp2[i] = z[0 * dim + i] + ytmp[i];
            }
            
            for i in 0..dim {
                relcor[i] = (ytmp[i] - abscor[i]) / errlev[i];
                abscor[i] = ytmp[i];
            }
            
            stepnorm = dnrm2(relcor) / (dim as f64).sqrt();
            
            if mi > 0 {
                convrate = convrate.max(stepnorm / stepnormprev);
            } else {
                convrate = 1.0;
            }
            
            let convtest = convrate.min(1.0) * stepnorm * errcoeff / SAFETY2;
            
            if convtest <= 1.0 {
                break;
            }
            
            if mi > 1 && stepnorm > 2.0 * stepnormprev {
                self.failure_handler(dim, t)?;
                return Err(GSL_ERROR::FAILURE);
            }
            
            sys.function(t + h, ytmp2, ytmp)?;
            
            stepnormprev = stepnorm;
        }
        
        if stepnorm > 0.0 {
            self.failure_handler(dim, t)?;
            return Err(GSL_ERROR::FAILURE);
        }
        
        Ok(())
    }
    
    fn eval_order(
        abscor: &[f64],
        tempvec: &mut [f64],
        svec: &mut [f64],
        errcoeff: f64,
        dim: usize,
        errlev: &[f64],
        ordm1coeff: f64,
        ordp1coeff: f64,
        ordp1coeffprev: f64,
        ordp2coeff: f64,
        hprev: &[f64],
        h: f64,
        z: &[f64],
        ord: &mut usize,
        ordwait: &mut usize,
    ) -> GSLResult<()> {
        const SAFETY: f64 = 1e-6;
        const BIAS: f64 = 6.0;
        const BIAS2: f64 = 10.0;
        
        let ordest = 1.0 / ((BIAS * dnrm2(abscor) / (dim as f64).sqrt() * errcoeff)
            .powf(1.0 / (*ord + 1) as f64 + SAFETY);
        
        let mut ordm1est = 0.0;
        if *ord > 1 {
            for i in 0..dim {
                tempvec[i] = z[*ord * dim + i] / errlev[i];
            }
            ordm1est = 1.0 / ((BIAS * dnrm2(tempvec) / (dim as f64).sqrt() / ordm1coeff)
                .powf(1.0 / *ord as f64) + SAFETY);
        }
        
        let mut ordp1est = 0.0;
        if *ord < MSADAMS_MAX_ORD {
            let c = -ordp1coeff / ordp1coeffprev * (h / hprev[1]).powi(*ord as i32 + 1);
            for i in 0..dim {
                svec[i] = svec[i] * c + abscor[i];
            }
            ordp1est = 1.0 / ((BIAS2 * dnrm2(svec) / (dim as f64).sqrt() / ordp2coeff)
                .powf(1.0 / (*ord + 2) as f64) + SAFETY);
        }
        
        const MIN_INCR: f64 = 1.5;
        if ordm1est > ordest && ordm1est > ordp1est && ordm1est > MIN_INCR {
            *ord -= 1;
        } else if ordp1est > ordest && ordp1est > ordm1est && ordp1est > MIN_INCR {
            *ord += 1;
        }
        
        *ordwait = *ord + 2;
        
        Ok(())
    }
    
    fn apply(
        &mut self,
        dim: usize,
        t: f64,
        h: f64,
        y: &mut [f64],
        yerr: &mut [f64],
        dydt_in: Option<&[f64]>,
        dydt_out: Option<&mut [f64]>,
        sys: &System,
    ) -> GSLResult<()> {
        let ord = self.ord;
        let mut ordm1coeff = 0.0;
        let mut ordp1coeff = 0.0;
        let mut ordp2coeff = 0.0;
        let mut errcoeff = 0.0;
        
        if self.ni > 0 && (t == self.tprev || t == self.failt) {
            if self.ni == 1 {
                self.reset(dim)?;
            } else {
                if ord > self.ordprev {
                    self.ord = self.ordprev;
                }
                
                self.z.copy_from_slice(&self.zbackup);
                self.hprev.copy_from_slice(&self.hprevbackup);
                self.ordprev = self.ordprevbackup;
                self.ordwait = self.ordwaitbackup;
            }
            
            self.failcount += 1;
            
            const MAX_FAILCOUNT: usize = 3;
            if self.failcount > MAX_FAILCOUNT && self.ni > 1 {
                self.reset(dim)?;
            } else if (self.ordprev as isize - ord as isize) >= 2 {
                self.reset(dim)?;
            }
        } else {
            self.zbackup.copy_from_slice(&self.z);
            self.hprevbackup.copy_from_slice(&self.hprev);
            self.ordprevbackup = self.ordprev;
            self.ordwaitbackup = self.ordwait;
            
            self.failcount = 0;
        }
        
        if let Some(driver) = &self.driver {
            for i in 0..dim {
                let dydt = dydt_in.map(|d| d[i]).unwrap_or(0.0);
                self.errlev[i] = driver.control.errlevel(y[i], dydt, h, i)?;
            }
        } else {
            return Err(GSL_ERROR::EFAULT);
        }
        
        if self.ni == 0 {
            self.z.iter_mut().for_each(|x| *x = 0.0);
            
            if let Some(dydt_in) = dydt_in {
                self.ytmp.copy_from_slice(dydt_in);
            } else {
                sys.function(t, y, &mut self.ytmp)?;
            }
            
            self.z[0..dim].copy_from_slice(y);
            self.z[dim..2*dim].copy_from_slice(&self.ytmp);
            
            for i in 0..dim {
                self.z[dim + i] *= h;
            }
        }
        
        let deltaord = ord as isize - self.ordprev as isize;
        if deltaord > 1 || deltaord < -1 {
            return Err(GSL_ERROR::ESANITY);
        }
        
        if deltaord == 1 {
            self.z[ord*dim..(ord+1)*dim].iter_mut().for_each(|x| *x = 0.0);
        }
        
        if deltaord == -1 {
            let mut hsum = 0.0;
            self.l.iter_mut().for_each(|x| *x = 0.0);
            self.l[1] = 1.0;
            
            for i in 1..ord {
                hsum += self.hprev[i - 1];
                
                for j in (1..=i+1).rev() {
                    self.l[j] *= hsum / self.hprev[0];
                    self.l[j] += self.l[j - 1];
                }
            }
            
            for i in 1..ord {
                self.l[i + 1] = (ord + 1) as f64 * self.l[i] / (i + 1) as f64;
            }
            
            for i in 2..=ord {
                for j in 0..dim {
                    self.z[i*dim + j] += -self.l[i] * self.z[(ord+1)*dim + j];
                }
            }
        }
        
        if self.ni > 0 && h != self.hprev[0] {
            let hrel = h / self.hprev[0];
            let mut coeff = hrel;
            
            for i in 1..=ord {
                for j in 0..dim {
                    self.z[i*dim + j] *= coeff;
                }
                coeff *= hrel;
            }
        }
        
        Self::calc_coeffs(
            ord,
            self.ordwait,
            h,
            &self.hprev,
            &mut self.pc,
            &mut self.l,
            &mut errcoeff,
            &mut ordm1coeff,
            &mut ordp1coeff,
            &mut ordp2coeff,
        )?;
        
        for i in 1..=ord {
            for j in (i..=ord).rev() {
                for k in 0..dim {
                    self.z[(j-1)*dim + k] += self.z[j*dim + k];
                }
            }
        }
        
        self.corrector