use std::f64::consts;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GslResult {
    Success = 0,
    Failure = -1,
    Continue = -2,
    EDOM = 1,
    ERANGE = 2,
    EFAULT = 3,
    EINVAL = 4,
    EFAILED = 5,
    EFACTOR = 6,
    ESANITY = 7,
    ENOMEM = 8,
    EBADFUNC = 9,
    ERUNAWAY = 10,
    EMAXITER = 11,
    EZERODIV = 12,
    EBADTOL = 13,
    ETOL = 14,
    EUNDRFLW = 15,
    EOVRFLW = 16,
    ELOSS = 17,
    EROUND = 18,
    EBADLEN = 19,
    ENOTSQR = 20,
    ESING = 21,
    EDIVERGE = 22,
    EUNSUP = 23,
    EUNIMPL = 24,
    ECACHE = 25,
    ETABLE = 26,
    ENOPROG = 27,
    ENOPROGJ = 28,
    ETOLF = 29,
    ETOLX = 30,
    ETOLG = 31,
    EOF = 32,
}

fn backward_recurse_c(
    aa: f64,
    qq: f64,
    xx: f64,
    ff: &mut [f64],
    gx: &mut f64,
    even_odd: i32,
    ni: i32,
) {
    let mut g1 = *gx;
    ff[ni as usize] = xx;

    if even_odd == 0 {
        for ii in 0..ni {
            let nn = 100 - ii - 1;
            ff[(ni - ii - 1) as usize] = -1.0 / (((4 * nn * nn) as f64 - aa) / qq + ff[(ni - ii) as usize]);
        }
        if ni == 100 - 1 {
            ff[0] *= 2.0;
        }
    } else {
        for ii in 0..ni {
            let nn = 100 - ii - 1;
            ff[(ni - ii - 1) as usize] = -1.0 / ((((2 * nn + 1) * (2 * nn + 1)) as f64 - aa) / qq + ff[(ni - ii) as usize]);
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
    ni: i32,
) {
    let mut g1 = *gx;
    ff[ni as usize] = xx;

    if even_odd == 0 {
        for ii in 0..ni {
            let nn = 100 - ii - 1;
            ff[(ni - ii - 1) as usize] = -1.0 / (((4 * (nn + 1) * (nn + 1)) as f64 - aa) / qq + ff[(ni - ii) as usize]);
        }
    } else {
        for ii in 0..ni {
            let nn = 100 - ii - 1;
            ff[(ni - ii - 1) as usize] = -1.0 / ((((2 * nn + 1) * (2 * nn + 1)) as f64 - aa) / qq + ff[(ni - ii) as usize]);
        }
    }
    *gx = ff[0] - g1;
}

pub fn gsl_sf_mathieu_a_coeff(
    order: i32,
    qq: f64,
    aa: f64,
    coeff: &mut [f64],
) -> GslResult {
    let mut even_odd = 0;
    if order % 2 != 0 {
        even_odd = 1;
    }

    if order > 100 {
        return GslResult::Failure;
    }

    if qq == 0.0 {
        coeff.fill(0.0);
        coeff[(order / 2) as usize] = 1.0;
        return GslResult::Success;
    }

    let eps = 1e-14;
    coeff[0] = 1.0;

    let (mut nn, mut sum, mut ratio) = if order < 5 {
        (0, 0.0, if even_odd == 0 { aa / qq } else { (aa - 1.0 - qq) / qq })
    } else {
        let mut sum = 0.0;
        let mut nn = 0;

        if even_odd == 0 {
            coeff[1] = aa / qq;
            coeff[2] = (aa - 4.0) / qq * coeff[1] - 2.0;
            sum = coeff[0] + coeff[1] + coeff[2];

            for ii in 3..(order / 2 + 1) {
                coeff[ii as usize] = (aa - (4 * (ii - 1) * (ii - 1)) as f64) / qq * coeff[(ii - 1) as usize] - coeff[(ii - 2) as usize];
                sum += coeff[ii as usize];
            }
            nn = order / 2;
        } else {
            coeff[1] = (aa - 1.0) / qq - 1.0;
            sum = coeff[0] + coeff[1];

            for ii in 2..(order / 2 + 1) {
                coeff[ii as usize] = (aa - ((2 * ii - 1) * (2 * ii - 1)) as f64 / qq * coeff[(ii - 1) as usize] - coeff[(ii - 2) as usize];
                sum += coeff[ii as usize];
            }
            nn = order / 2;
        }

        (nn, sum, coeff[nn as usize] / coeff[(nn - 1) as usize])
    };

    let ni = 100 - nn - 1;
    let x1 = if even_odd == 0 {
        -qq / (4.0 * 100.0 * 100.0)
    } else {
        -qq / ((2.0 * 100.0 + 1.0) * (2.0 * 100.0 + 1.0))
    };

    let mut ff = [0.0; 100];
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

    sum += coeff[nn as usize];
    for ii in (nn + 1)..100 {
        coeff[ii as usize] = ff[(ii - nn - 1) as usize] * coeff[(ii - 1) as usize];
        sum += coeff[ii as usize];

        if coeff[ii as usize].abs() < 1e-20 {
            coeff[ii as usize..100].fill(0.0);
            break;
        }
    }

    for val in coeff.iter_mut().take(100) {
        *val /= sum;
    }

    GslResult::Success
}

pub fn gsl_sf_mathieu_b_coeff(
    order: i32,
    qq: f64,
    aa: f64,
    coeff: &mut [f64],
) -> GslResult {
    let mut even_odd = 0;
    if order % 2 != 0 {
        even_odd = 1;
    }

    if order > 100 {
        return GslResult::Failure;
    }

    if qq == 0.0 {
        coeff.fill(0.0);
        coeff[((order - 1) / 2) as usize] = 1.0;
        return GslResult::Success;
    }

    let eps = 1e-10;
    coeff[0] = 1.0;

    let (mut nn, mut sum, mut ratio) = if order < 5 {
        (0, 0.0, if even_odd == 0 { (aa - 4.0) / qq } else { (aa - 1.0 - qq) / qq })
    } else {
        let mut sum = 0.0;
        let mut nn = 0;

        if even_odd == 0 {
            coeff[1] = (aa - 4.0) / qq;
            sum = 2.0 * coeff[0] + 4.0 * coeff[1];

            for ii in 2..(order / 2) {
                coeff[ii as usize] = (aa - (4 * ii * ii) as f64) / qq * coeff[(ii - 1) as usize] - coeff[(ii - 2) as usize];
                sum += (2 * (ii + 1)) as f64 * coeff[ii as usize];
            }
            nn = order / 2 - 1;
        } else {
            coeff[1] = (aa - 1.0) / qq + 1.0;
            sum = coeff[0] + 3.0 * coeff[1];

            for ii in 2..(order / 2 + 1) {
                coeff[ii as usize] = (aa - ((2 * ii - 1) * (2 * ii - 1)) as f64 / qq * coeff[(ii - 1) as usize] - coeff[(ii - 2) as usize];
                sum += (2 * (ii + 1) - 1) as f64 * coeff[ii as usize];
            }
            nn = order / 2;
        }

        (nn, sum, coeff[nn as usize] / coeff[(nn - 1) as usize])
    };

    let ni = 100 - nn - 1;
    let x1 = if even_odd == 0 {
        -qq / (4.0 * (100.0 + 1.0) * (100.0 + 1.0))
    } else {
        -qq / ((2.0 * 100.0 + 1.0) * (2.0 * 100.0 + 1.0))
    };

    let mut ff = [0.0; 100];
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

    sum += (2 * (nn + 1)) as f64 * coeff[nn as usize];
    for ii in (nn + 1)..100 {
        coeff[ii as usize] = ff[(ii - nn - 1) as usize] * coeff[(ii - 1) as usize];
        sum += (2 * (ii + 1)) as f64 * coeff[ii as usize];

        if coeff[ii as usize].abs() < 1e-20 {
            coeff[ii as usize..100].fill(0.0);
            break;
        }
    }

    for val in coeff.iter_mut().take(100) {
        *val /= sum;
    }

    GslResult::Success
}