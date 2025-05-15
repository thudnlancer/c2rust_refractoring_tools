/* sum/levin_utrunc.rs
 * 
 * Copyright (C) 1996, 1997, 1998, 1999, 2000, 2007 Gerard Jungman, Brian Gough
 * 
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or (at
 * your option) any later version.
 * 
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * General Public License for more details.
 * 
 * You should have received a copy of the GNU General Public License
 * along with this program; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301, USA.
 */

/* Author:  G. Jungman */

use std::f64;

pub const GSL_SUCCESS: i32 = 0;
pub const GSL_EZERODIV: i32 = 1;
pub const GSL_POSINF: f64 = f64::INFINITY;
pub const GSL_DBL_MAX: f64 = f64::MAX;
pub const GSL_MACH_EPS: f64 = f64::EPSILON;

pub struct LevinUtruncWorkspace {
    pub sum_plain: f64,
    pub terms_used: usize,
    pub q_den: Vec<f64>,
    pub q_num: Vec<f64>,
}

impl LevinUtruncWorkspace {
    pub fn new(size: usize) -> Self {
        LevinUtruncWorkspace {
            sum_plain: 0.0,
            terms_used: 0,
            q_den: vec![0.0; size],
            q_num: vec![0.0; size],
        }
    }
}

pub fn gsl_sum_levin_utrunc_accel(
    array: &[f64],
    w: &mut LevinUtruncWorkspace,
) -> Result<(f64, f64), i32> {
    gsl_sum_levin_utrunc_minmax(array, 0, array.len() - 1, w)
}

pub fn gsl_sum_levin_utrunc_minmax(
    array: &[f64],
    min_terms: usize,
    max_terms: usize,
    w: &mut LevinUtruncWorkspace,
) -> Result<(f64, f64), i32> {
    let array_size = array.len();

    if array_size == 0 {
        w.sum_plain = 0.0;
        w.terms_used = 0;
        Ok((0.0, 0.0))
    } else if array_size == 1 {
        w.sum_plain = array[0];
        w.terms_used = 1;
        Ok((array[0], GSL_POSINF))
    } else {
        const SMALL: f64 = 0.01;
        let nmax = std::cmp::max(max_terms, array_size) - 1;
        let mut trunc_n = 0.0;
        let mut trunc_nm1 = 0.0;
        let mut actual_trunc_n = 0.0;
        let mut actual_trunc_nm1 = 0.0;
        let mut result_n = 0.0;
        let mut result_nm1 = 0.0;
        let mut n = 0;
        let mut better = false;
        let mut before = false;
        let mut converging = false;
        let mut least_trunc = GSL_DBL_MAX;
        let mut result_least_trunc = 0.0;

        for i in 0..min_terms {
            let t = array[i];
            result_nm1 = result_n;
            gsl_sum_levin_utrunc_step(t, i, w, &mut result_n)?;
            n = i + 1;
        }

        result_least_trunc = result_n;

        for i in n..=nmax {
            let t = array[i];
            result_nm1 = result_n;
            gsl_sum_levin_utrunc_step(t, i, w, &mut result_n)?;

            actual_trunc_nm1 = actual_trunc_n;
            actual_trunc_n = (result_n - result_nm1).abs();

            trunc_nm1 = trunc_n;
            trunc_n = 0.5 * (actual_trunc_n + actual_trunc_nm1);

            better = trunc_n < trunc_nm1 || trunc_n < SMALL * result_n.abs();
            converging = converging || (better && before);
            before = better;

            if converging {
                if trunc_n < least_trunc {
                    least_trunc = trunc_n;
                    result_least_trunc = result_n;
                }

                if (trunc_n / result_n).abs() < 10.0 * GSL_MACH_EPS {
                    break;
                }
            }
            n = i + 1;
        }

        if converging {
            w.terms_used = n;
            Ok((result_least_trunc, least_trunc))
        } else {
            w.terms_used = n;
            Ok((result_n, trunc_n))
        }
    }
}

pub fn gsl_sum_levin_utrunc_step(
    term: f64,
    n: usize,
    w: &mut LevinUtruncWorkspace,
    sum_accel: &mut f64,
) -> Result<(), i32> {
    if term == 0.0 {
        Err(GSL_EZERODIV)
    } else if n == 0 {
        *sum_accel = term;
        w.sum_plain = term;
        w.q_den[0] = 1.0 / term;
        w.q_num[0] = 1.0;
        Ok(())
    } else {
        let mut factor = 1.0;
        let ratio = n as f64 / (n as f64 + 1.0);

        w.sum_plain += term;
        w.q_den[n] = 1.0 / (term * (n as f64 + 1.0).powi(2));
        w.q_num[n] = w.sum_plain * w.q_den[n];

        for j in (0..n).rev() {
            let c = factor * (j as f64 + 1.0) / (n as f64 + 1.0);
            factor *= ratio;
            w.q_den[j] = w.q_den[j + 1] - c * w.q_den[j];
            w.q_num[j] = w.q_num[j + 1] - c * w.q_num[j];
        }

        *sum_accel = w.q_num[0] / w.q_den[0];
        Ok(())
    }
}