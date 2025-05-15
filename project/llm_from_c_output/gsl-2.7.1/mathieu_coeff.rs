use std::f64::consts::EPSILON;

const GSL_SF_MATHIEU_COEFF: usize = 100;
const GSL_SUCCESS: i32 = 0;
const GSL_FAILURE: i32 = -1;

fn backward_recurse_c(
    aa: f64,
    qq: f64,
    xx: f64,
    ff: &mut [f64],
    gx: &mut f64,
    even_odd: i32,
    ni: usize,
) {
    let g1 = *gx;
    ff[ni] = xx;

    if even_odd == 0 {
        for ii in 0..ni {
            let nn = GSL_SF_MATHIEU_COEFF - ii - 1;
            ff[ni - ii - 1] = -1.0 / ((4.0 * (nn * nn) as f64 - aa) / qq + ff[ni - ii]);
        }
        if ni == GSL_SF_MATHIEU_COEFF - 1 {
            ff[0] *= 2.0;
        }
    } else {
        for ii in 0..ni {
            let nn = GSL_SF_MATHIEU_COEFF - ii - 1;
            ff[ni - ii - 1] = -1.0 / ((((2 * nn + 1) * (2 * nn + 1)) as f64 - aa) / qq + ff[ni - ii]);
        }
    }

    *gx = ff[0] - g1;
}

fn backward_recurse_s(
    aa: f64,
    qq: f64,
    xx: f64,
    ff: &mut [f64],
    gx: &mut f64,
    even_odd: i32,
    ni: usize,
) {
    let g1 = *gx;
    ff[ni] = xx;

    if even_odd == 0 {
        for ii in 0..ni {
            let nn = GSL_SF_MATHIEU_COEFF - ii - 1;
            ff[ni - ii - 1] = -1.0 / ((4.0 * ((nn + 1) * (nn + 1)) as f64 - aa) / qq + ff[ni - ii];
        }
    } else {
        for ii in 0..ni {
            let nn = GSL_SF_MATHIEU_COEFF - ii - 1;
            ff[ni - ii - 1] = -1.0 / ((((2 * nn + 1) * (2 * nn + 1)) as f64 - aa) / qq + ff[ni - ii]);
        }
    }

    *gx = ff[0] - g1;
}

pub fn gsl_sf_mathieu_a_coeff(
    order: i32,
    qq: f64,
    aa: f64,
    coeff: &mut [f64],
) -> i32 {
    let eps = 1e-14;
    coeff[0] = 1.0;

    let even_odd = if order % 2 != 0 { 1 } else { 0 };

    if order as usize > GSL_SF_MATHIEU_COEFF {
        return GSL_FAILURE;
    }

    if qq == 0.0 {
        for ii in 0..GSL_SF_MATHIEU_COEFF {
            coeff[ii] = 0.0;
        }
        coeff[(order / 2) as usize] = 1.0;
        return GSL_SUCCESS;
    }

    let (mut nn, mut sum, mut ratio) = if order < 5 {
        let ratio = if even_odd == 0 {
            aa / qq
        } else {
            (aa - 1.0 - qq) / qq
        };
        (0, 0.0, ratio)
    } else {
        let mut sum = 0.0;
        let mut ii = 1;
        if even_odd == 0 {
            coeff[1] = aa / qq;
            coeff[2] = (aa - 4.0) / qq * coeff[1] - 2.0;
            sum = coeff[0] + coeff[1] + coeff[2];
            for ii in 3..(order / 2 + 1) as usize {
                coeff[ii] = (aa - 4.0 * ((ii - 1) * (ii - 1)) as f64) / qq * coeff[ii - 1] - coeff[ii - 2];
                sum += coeff[ii];
            }
        } else {
            coeff[1] = (aa - 1.0) / qq - 1.0;
            sum = coeff[0] + coeff[1];
            for ii in 2..(order / 2 + 1) as usize {
                coeff[ii] = (aa - ((2 * ii - 1) * (2 * ii - 1)) as f64) / qq * coeff[ii - 1] - coeff[ii - 2];
                sum += coeff[ii];
            }
        }
        let nn = ii - 1;
        let ratio = coeff[nn] / coeff[nn - 1];
        (nn, sum, ratio)
    };

    let ni = GSL_SF_MATHIEU_COEFF - nn - 1;
    let mut ff = [0.0; GSL_SF_MATHIEU_COEFF];

    let x1 = if even_odd == 0 {
        -qq / (4.0 * (GSL_SF_MATHIEU_COEFF * GSL_SF_MATHIEU_COEFF) as f64)
    } else {
        -qq / ((2.0 * GSL_SF_MATHIEU_COEFF as f64 + 1.0).powi(2))
    };
    let mut g1 = ratio;
    backward_recurse_c(aa, qq, x1, &mut ff, &mut g1, even_odd, ni);
    let mut x2 = g1;
    let mut g2 = ratio;
    backward_recurse_c(aa, qq, x2, &mut ff, &mut g2, even_odd, ni);

    loop {
        let e1 = g1 - x1;
        let e2 = g2 - x2;
        let de = e1 - e2;

        if de.abs() < eps {
            break;
        }

        let xh = (e1 * x2 - e2 * x1) / de;
        x1 = x2;
        g1 = g2;
        x2 = xh;
        g2 = ratio;
        backward_recurse_c(aa, qq, x2, &mut ff, &mut g2, even_odd, ni);
    }

    sum += coeff[nn];
    for ii in nn + 1..GSL_SF_MATHIEU_COEFF {
        coeff[ii] = ff[ii - nn - 1] * coeff[ii - 1];
        sum += coeff[ii];

        if coeff[ii].abs() < 1e-20 {
            for j in ii..GSL_SF_MATHIEU_COEFF {
                coeff[j] = 0.0;
            }
            break;
        }
    }

    for ii in 0..GSL_SF_MATHIEU_COEFF {
        coeff[ii] /= sum;
    }

    GSL_SUCCESS
}

pub fn gsl_sf_mathieu_b_coeff(
    order: i32,
    qq: f64,
    aa: f64,
    coeff: &mut [f64],
) -> i32 {
    let eps = 1e-10;
    coeff[0] = 1.0;

    let even_odd = if order % 2 != 0 { 1 } else { 0 };

    if order as usize > GSL_SF_MATHIEU_COEFF {
        return GSL_FAILURE;
    }

    if qq == 0.0 {
        for ii in 0..GSL_SF_MATHIEU_COEFF {
            coeff[ii] = 0.0;
        }
        coeff[((order - 1) / 2) as usize] = 1.0;
        return GSL_SUCCESS;
    }

    let (mut nn, mut sum, mut ratio) = if order < 5 {
        let ratio = if even_odd == 0 {
            (aa - 4.0) / qq
        } else {
            (aa - 1.0 - qq) / qq
        };
        (0, 0.0, ratio)
    } else {
        let mut sum = 0.0;
        let mut ii = 1;
        if even_odd == 0 {
            coeff[1] = (aa - 4.0) / qq;
            sum = 2.0 * coeff[0] + 4.0 * coeff[1];
            for ii in 2..(order / 2) as usize {
                coeff[ii] = (aa - 4.0 * (ii * ii) as f64) / qq * coeff[ii - 1] - coeff[ii - 2];
                sum += 2.0 * (ii + 1) as f64 * coeff[ii];
            }
        } else {
            coeff[1] = (aa - 1.0) / qq + 1.0;
            sum = coeff[0] + 3.0 * coeff[1];
            for ii in 2..(order / 2 + 1) as usize {
                coeff[ii] = (aa - ((2 * ii - 1) * (2 * ii - 1)) as f64) / qq * coeff[ii - 1] - coeff[ii - 2];
                sum += (2.0 * (ii + 1) as f64 - 1.0) * coeff[ii];
            }
        }
        let nn = ii - 1;
        let ratio = coeff[nn] / coeff[nn - 1];
        (nn, sum, ratio)
    };

    let ni = GSL_SF_MATHIEU_COEFF - nn - 1;
    let mut ff = [0.0; GSL_SF_MATHIEU_COEFF];

    let x1 = if even_odd == 0 {
        -qq / (4.0 * ((GSL_SF_MATHIEU_COEFF + 1) * (GSL_SF_MATHIEU_COEFF + 1)) as f64)
    } else {
        -qq / ((2.0 * GSL_SF_MATHIEU_COEFF as f64 + 1.0).powi(2))
    };
    let mut g1 = ratio;
    backward_recurse_s(aa, qq, x1, &mut ff, &mut g1, even_odd, ni);
    let mut x2 = g1;
    let mut g2 = ratio;
    backward_recurse_s(aa, qq, x2, &mut ff, &mut g2, even_odd, ni);

    loop {
        let e1 = g1 - x1;
        let e2 = g2 - x2;
        let de = e1 - e2;

        if de.abs() < eps {
            break;
        }

        let xh = (e1 * x2 - e2 * x1) / de;
        x1 = x2;
        g1 = g2;
        x2 = xh;
        g2 = ratio;
        backward_recurse_s(aa, qq, x2, &mut ff, &mut g2, even_odd, ni);
    }

    sum += 2.0 * (nn + 1) as f64 * coeff[nn];
    for ii in nn + 1..GSL_SF_MATHIEU_COEFF {
        coeff[ii] = ff[ii - nn - 1] * coeff[ii - 1];
        sum += 2.0 * (ii + 1) as f64 * coeff[ii];

        if coeff[ii].abs() < 1e-20 {
            for j in ii..GSL_SF_MATHIEU_COEFF {
                coeff[j] = 0.0;
            }
            break;
        }
    }

    for ii in 0..GSL_SF_MATHIEU_COEFF {
        coeff[ii] /= sum;
    }

    GSL_SUCCESS
}