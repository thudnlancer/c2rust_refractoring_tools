use std::f64::consts::LN_10;
use libm::{fabs, exp};

#[derive(Debug, Clone, Copy)]
pub struct GslSfResult {
    pub val: f64,
    pub err: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct GslSfResultE10 {
    pub val: f64,
    pub err: f64,
    pub e10: i32,
}

pub const GSL_SUCCESS: i32 = 0;
const GSL_SQRT_DBL_MIN: f64 = f64::MIN_POSITIVE.sqrt();
const GSL_SQRT_DBL_MAX: f64 = f64::MAX.sqrt();
const GSL_LOG_DBL_MIN: f64 = f64::MIN_POSITIVE.ln();
const GSL_LOG_DBL_MAX: f64 = f64::MAX.ln();

pub fn gsl_sf_result_smash_e(re: &GslSfResultE10, r: &mut GslSfResult) -> i32 {
    if re.e10 == 0 {
        // nothing to smash
        r.val = re.val;
        r.err = re.err;
        GSL_SUCCESS
    } else {
        let av = fabs(re.val);
        let ae = fabs(re.err);

        if GSL_SQRT_DBL_MIN < av && av < GSL_SQRT_DBL_MAX &&
           GSL_SQRT_DBL_MIN < ae && ae < GSL_SQRT_DBL_MAX &&
           0.49 * GSL_LOG_DBL_MIN < re.e10 as f64 && re.e10 as f64 < 0.49 * GSL_LOG_DBL_MAX
        {
            let scale = exp(re.e10 as f64 * LN_10);
            r.val = re.val * scale;
            r.err = re.err * scale;
            GSL_SUCCESS
        } else {
            // TODO: Implement gsl_sf_exp_mult_err_e equivalent in Rust
            // For now, we'll return an error code indicating this case isn't handled
            // In a complete implementation, this would call the equivalent function
            -1
        }
    }
}

/*
  The commented out section in the original C code shows an alternative implementation
  that handles edge cases differently. This could be implemented in Rust as follows:

  let stat_v;
  let stat_e;

  if re.val == 0.0 {
      r.val = 0.0;
      stat_v = GSL_SUCCESS;
  } else {
      let mut r_val = GslSfResult { val: 0.0, err: 0.0 };
      let s = re.val.signum();
      let x_v = re.e10 as f64 * LN_10 + fabs(re.val).ln();
      stat_v = gsl_sf_exp_e(x_v, &mut r_val);
      r.val = s * r_val.val;
  }

  if re.err == 0.0 {
      r.err = 0.0;
      stat_e = GSL_SUCCESS;
  } else if re.val != 0.0 {
      r.err = fabs(r.val * re.err / re.val);
      stat_e = GSL_SUCCESS;
  } else {
      let mut r_err = GslSfResult { val: 0.0, err: 0.0 };
      let x_e = re.e10 as f64 * LN_10 + fabs(re.err).ln();
      stat_e = gsl_sf_exp_e(x_e, &mut r_err);
      r.err = r_err.val;
  }

  // GSL_ERROR_SELECT_2 equivalent
  if stat_v != GSL_SUCCESS { stat_v } else { stat_e }
*/