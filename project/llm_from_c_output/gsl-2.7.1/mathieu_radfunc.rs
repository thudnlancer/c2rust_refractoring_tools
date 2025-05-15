use std::f64::consts::PI;
use std::f64::EPSILON as DBL_EPSILON;
use std::cmp::max;

#[derive(Debug, Clone, Copy)]
pub struct SfResult {
    pub val: f64,
    pub err: f64,
}

#[derive(Debug)]
pub struct MathieuWorkspace {
    pub aa: Vec<f64>,
    pub bb: Vec<f64>,
}

impl MathieuWorkspace {
    pub fn new(size: usize) -> Self {
        Self {
            aa: vec![0.0; size],
            bb: vec![0.0; size],
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum MathieuError {
    InvalidInput(String),
    ComputationFailed,
}

pub fn mathieu_mc_e(
    kind: i32,
    order: i32,
    qq: f64,
    zz: f64,
) -> Result<SfResult, MathieuError> {
    if qq <= 0.0 {
        return Err(MathieuError::InvalidInput("q must be greater than zero".to_string()));
    }
    if kind < 1 || kind > 2 {
        return Err(MathieuError::InvalidInput("kind must be 1 or 2".to_string()));
    }

    let maxerr = 1e-14;
    let mut amax = 0.0;
    let mut fn = 0.0;
    let u1 = qq.sqrt() * (-zz).exp();
    let u2 = qq.sqrt() * zz.exp();
    
    let even_odd = if order % 2 != 0 { 1 } else { 0 };

    let aa = mathieu_a_e(order, qq)?;
    
    let coeff = mathieu_a_coeff(order, qq, aa.val)?;

    if even_odd == 0 {
        for (kk, &ck) in coeff.iter().enumerate() {
            amax = max(amax, ck.abs());
            if ck.abs() / amax < maxerr {
                break;
            }

            let j1c = bessel_jn(kk as i32, u1);
            let z2c = if kind == 1 {
                bessel_jn(kk as i32, u2)
            } else {
                bessel_yn(kk as i32, u2)
            };
            
            let fc = (-1.0f64).powf(0.5 * order as f64 + kk as f64) * ck;
            fn += fc * j1c * z2c;
        }

        fn *= (PI / 2.0).sqrt() / coeff[0];
    } else {
        for (kk, &ck) in coeff.iter().enumerate() {
            amax = max(amax, ck.abs());
            if ck.abs() / amax < maxerr {
                break;
            }

            let j1c = bessel_jn(kk as i32, u1);
            let j1pc = bessel_jn(kk as i32 + 1, u1);
            let z2c = if kind == 1 {
                bessel_jn(kk as i32, u2)
            } else {
                bessel_yn(kk as i32, u2)
            };
            let z2pc = if kind == 1 {
                bessel_jn(kk as i32 + 1, u2)
            } else {
                bessel_yn(kk as i32 + 1, u2)
            };
            
            let fc = (-1.0f64).powf(0.5 * (order - 1) as f64 + kk as f64) * ck;
            fn += fc * (j1c * z2pc + j1pc * z2c);
        }

        fn *= (PI / 2.0).sqrt() / coeff[0];
    }

    let mut result = SfResult {
        val: fn,
        err: 2.0 * DBL_EPSILON,
    };
    
    let factor = fn.abs();
    if factor > 1.0 {
        result.err *= factor;
    }
    
    Ok(result)
}

pub fn mathieu_ms_e(
    kind: i32,
    order: i32,
    qq: f64,
    zz: f64,
) -> Result<SfResult, MathieuError> {
    if qq <= 0.0 {
        return Err(MathieuError::InvalidInput("q must be greater than zero".to_string()));
    }
    if kind < 1 || kind > 2 {
        return Err(MathieuError::InvalidInput("kind must be 1 or 2".to_string()));
    }

    if order == 0 {
        return Ok(SfResult { val: 0.0, err: 0.0 });
    }
    
    let maxerr = 1e-14;
    let mut amax = 0.0;
    let mut fn = 0.0;
    let u1 = qq.sqrt() * (-zz).exp();
    let u2 = qq.sqrt() * zz.exp();
    
    let even_odd = if order % 2 != 0 { 1 } else { 0 };

    let aa = mathieu_b_e(order, qq)?;
    
    let coeff = mathieu_b_coeff(order, qq, aa.val)?;

    if even_odd == 0 {
        for (kk, &ck) in coeff.iter().enumerate() {
            amax = max(amax, ck.abs());
            if ck.abs() / amax < maxerr {
                break;
            }

            let j1mc = bessel_jn(kk as i32, u1);
            let j1pc = bessel_jn(kk as i32 + 2, u1);
            let z2mc = if kind == 1 {
                bessel_jn(kk as i32, u2)
            } else {
                bessel_yn(kk as i32, u2)
            };
            let z2pc = if kind == 1 {
                bessel_jn(kk as i32 + 2, u2)
            } else {
                bessel_yn(kk as i32 + 2, u2)
            };
            
            let fc = (-1.0f64).powf(0.5 * order as f64 + kk as f64 + 1.0) * ck;
            fn += fc * (j1mc * z2pc - j1pc * z2mc);
        }

        fn *= (PI / 2.0).sqrt() / coeff[0];
    } else {
        for (kk, &ck) in coeff.iter().enumerate() {
            amax = max(amax, ck.abs());
            if ck.abs() / amax < maxerr {
                break;
            }

            let j1c = bessel_jn(kk as i32, u1);
            let j1pc = bessel_jn(kk as i32 + 1, u1);
            let z2c = if kind == 1 {
                bessel_jn(kk as i32, u2)
            } else {
                bessel_yn(kk as i32, u2)
            };
            let z2pc = if kind == 1 {
                bessel_jn(kk as i32 + 1, u2)
            } else {
                bessel_yn(kk as i32 + 1, u2)
            };
            
            let fc = (-1.0f64).powf(0.5 * (order - 1) as f64 + kk as f64) * ck;
            fn += fc * (j1c * z2pc - j1pc * z2c);
        }

        fn *= (PI / 2.0).sqrt() / coeff[0];
    }

    let mut result = SfResult {
        val: fn,
        err: 2.0 * DBL_EPSILON,
    };
    
    let factor = fn.abs();
    if factor > 1.0 {
        result.err *= factor;
    }
    
    Ok(result)
}

pub fn mathieu_mc_array(
    kind: i32,
    nmin: i32,
    nmax: i32,
    qq: f64,
    zz: f64,
    work: &mut MathieuWorkspace,
    result_array: &mut [f64],
) -> Result<(), MathieuError> {
    if qq <= 0.0 {
        return Err(MathieuError::InvalidInput("q must be greater than zero".to_string()));
    }
    if kind < 1 || kind > 2 {
        return Err(MathieuError::InvalidInput("kind must be 1 or 2".to_string()));
    }

    let maxerr = 1e-14;
    let mut amax = 0.0;
    let u1 = qq.sqrt() * (-zz).exp();
    let u2 = qq.sqrt() * zz.exp();
    
    mathieu_a_array(0, nmax, qq, work)?;
    
    for (ii, order) in (nmin..=nmax).enumerate() {
        let mut fn = 0.0;
        let even_odd = if order % 2 != 0 { 1 } else { 0 };

        let coeff = mathieu_a_coeff(order, qq, work.aa[order as usize])?;

        if even_odd == 0 {
            for (kk, &ck) in coeff.iter().enumerate() {
                amax = max(amax, ck.abs());
                if ck.abs() / amax < maxerr {
                    break;
                }

                let j1c = bessel_jn(kk as i32, u1);
                let z2c = if kind == 1 {
                    bessel_jn(kk as i32, u2)
                } else {
                    bessel_yn(kk as i32, u2)
                };
                
                let fc = (-1.0f64).powf(0.5 * order as f64 + kk as f64) * ck;
                fn += fc * j1c * z2c;
            }

            fn *= (PI / 2.0).sqrt() / coeff[0];
        } else {
            for (kk, &ck) in coeff.iter().enumerate() {
                amax = max(amax, ck.abs());
                if ck.abs() / amax < maxerr {
                    break;
                }

                let j1c = bessel_jn(kk as i32, u1);
                let j1pc = bessel_jn(kk as i32 + 1, u1);
                let z2c = if kind == 1 {
                    bessel_jn(kk as i32, u2)
                } else {
                    bessel_yn(kk as i32, u2)
                };
                let z2pc = if kind == 1 {
                    bessel_jn(kk as i32 + 1, u2)
                } else {
                    bessel_yn(kk as i32 + 1, u2)
                };
                
                let fc = (-1.0f64).powf(0.5 * (order - 1) as f64 + kk as f64) * ck;
                fn += fc * (j1c * z2pc + j1pc * z2c);
            }

            fn *= (PI / 2.0).sqrt() / coeff[0];
        }

        result_array[ii] = fn;
    }
    
    Ok(())
}

pub fn mathieu_ms_array(
    kind: i32,
    nmin: i32,
    nmax: i32,
    qq: f64,
    zz: f64,
    work: &mut MathieuWorkspace,
    result_array: &mut [f64],
) -> Result<(), MathieuError> {
    if qq <= 0.0 {
        return Err(MathieuError::InvalidInput("q must be greater than zero".to_string()));
    }
    if kind < 1 || kind > 2 {
        return Err(MathieuError::InvalidInput("kind must be 1 or 2".to_string()));
    }

    let maxerr = 1e-14;
    let mut amax = 0.0;
    let u1 = qq.sqrt() * (-zz).exp();
    let u2 = qq.sqrt() * zz.exp();
    
    mathieu_b_array(0, nmax, qq, work)?;
    
    for (ii, order) in (nmin..=nmax).enumerate() {
        if order == 0 {
            result_array[ii] = 0.0;
            continue;
        }

        let mut fn = 0.0;
        let even_odd = if order % 2 != 0 { 1 } else { 0 };

        let coeff = mathieu_b_coeff(order, qq, work.bb[order as usize])?;

        if even_odd == 0 {
            for (kk, &ck) in coeff.iter().enumerate() {
                amax = max(amax, ck.abs());
                if ck.abs() / amax < maxerr {
                    break;
                }

                let j1mc = bessel_jn(kk as i32, u1);
                let j1pc = bessel_jn(kk as i32 + 2, u1);
                let z2mc = if kind == 1 {
                    bessel_jn(kk as i32, u2)
                } else {
                    bessel_yn(kk as i32, u2)
                };
                let z2pc = if kind == 1 {
                    bessel_jn(kk as i32 + 2, u2)
                } else {
                    bessel_yn(kk as i32 + 2, u2)
                };
                
                let fc = (-1.0f64).powf(0.5 * order as f64 + kk as f64 + 1.0) * ck;
                fn += fc * (j1mc * z2pc - j1pc * z2mc);
            }

            fn *= (PI / 2.0).sqrt() / coeff[0];
        } else {
            for (kk, &ck) in coeff.iter().enumerate() {
                amax = max(amax, ck.abs());
                if ck.abs() / amax < maxerr {
                    break;
                }

                let j1c = bessel_jn(kk as i32, u1);
                let j1pc = bessel_jn(kk as i32 + 1, u1);
                let z2c = if kind == 1 {
                    bessel_jn(kk as i32, u2)
                } else {
                    bessel_yn(kk as i32, u2)
                };
                let z2pc = if kind == 1 {
                    bessel_jn(kk as i32 + 1, u2)
                } else {
                    bessel_yn(kk as i32 + 1, u2)
                };
                
                let fc = (-1.0f64).powf(0.5 * (order - 1) as f64 + kk as f64) * ck;
                fn += fc * (j1c * z2pc - j1pc * z2c);
            }

            fn *= (PI / 2.0).sqrt() / coeff[0];
        }

        result_array[ii] = fn;
    }
    
    Ok(())
}

// Placeholder functions for the GSL functions used
fn mathieu_a_e(_order: i32, _qq: f64) -> Result<SfResult, MathieuError> {
    // Implementation would depend on actual mathieu function implementation
    unimplemented!()
}

fn mathieu_b_e(_order: i32, _qq: f64) -> Result<SfResult, MathieuError> {
    // Implementation would depend on actual mathieu function implementation
    unimplemented!()
}

fn mathieu_a_coeff(_order: i32, _qq: f64, _aa: f64) -> Result<Vec<f64>, MathieuError> {
    // Implementation would depend on actual mathieu function implementation
    unimplemented!()
}

fn mathieu_b_coeff(_order: i32, _qq: f64, _bb: f64) -> Result<Vec<f64>, MathieuError> {
    // Implementation would depend on actual mathieu function implementation
    unimplemented!()
}

fn mathieu_a_array(_nmin: i32, _nmax: i32, _qq: f64, _work: &mut MathieuWorkspace) -> Result<(), MathieuError> {
    // Implementation would depend on actual mathieu function implementation
    unimplemented!()
}

fn mathieu_b_array(_nmin: i32, _nmax: i32, _qq: f64, _work: &mut MathieuWorkspace) -> Result<(), MathieuError> {
    // Implementation would depend on actual mathieu function implementation
    unimplemented!()
}

fn bessel_jn(_n: i32, _x: f64) -> f64 {
    // Implementation would depend on Bessel function implementation
    unimplemented!()
}

fn bessel_yn(_n: i32, _x: f64) -> f64 {
    // Implementation would depend on Bessel function implementation
    unimplemented!()
}

pub fn mathieu_mc(kind: i32, order: i32, qq: f64, zz: f64) -> f64 {
    mathieu_mc_e(kind, order, qq, zz).unwrap().val
}

pub fn mathieu_ms(kind: i32, order: i32, qq: f64, zz: f64) -> f64 {
    mathieu_ms_e(kind, order, qq, zz).unwrap().val
}