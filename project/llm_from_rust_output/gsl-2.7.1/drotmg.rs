use std::f64;

#[derive(Debug, Clone, Copy)]
pub struct RotmgParameters {
    pub flag: f64,
    pub h11: f64,
    pub h21: f64,
    pub h12: f64,
    pub h22: f64,
}

impl Default for RotmgParameters {
    fn default() -> Self {
        RotmgParameters {
            flag: -2.0,
            h11: 0.0,
            h21: 0.0,
            h12: 0.0,
            h22: 0.0,
        }
    }
}

pub fn cblas_drotmg(
    d1: &mut f64,
    d2: &mut f64,
    b1: &mut f64,
    b2: f64,
) -> RotmgParameters {
    const G: f64 = 4096.0;
    const G2: f64 = G * G;
    
    let mut params = RotmgParameters::default();
    let mut d1_val = *d1;
    let mut d2_val = *d2;
    let mut x = *b1;
    let y = b2;

    if d1_val < 0.0 {
        *d1 = 0.0;
        *d2 = 0.0;
        *b1 = 0.0;
        params.flag = -1.0;
        return params;
    }

    if d2_val * y == 0.0 {
        return params;
    }

    let c = (d1_val * x * x).abs();
    let s = (d2_val * y * y).abs();

    let (h11, h12, h21, h22, u) = if c > s {
        params.flag = 0.0;
        let h11 = 1.0;
        let h12 = d2_val * y / (d1_val * x);
        let h21 = -y / x;
        let h22 = 1.0;
        let u = 1.0 - h21 * h12;

        if u <= 0.0 {
            *d1 = 0.0;
            *d2 = 0.0;
            *b1 = 0.0;
            params.flag = -1.0;
            return params;
        }

        d1_val /= u;
        d2_val /= u;
        x *= u;
        (h11, h12, h21, h22, u)
    } else {
        if d2_val * y * y < 0.0 {
            *d1 = 0.0;
            *d2 = 0.0;
            *b1 = 0.0;
            params.flag = -1.0;
            return params;
        }

        params.flag = 1.0;
        let h11 = d1_val * x / (d2_val * y);
        let h12 = 1.0;
        let h21 = -1.0;
        let h22 = x / y;
        let u = 1.0 + h11 * h22;

        d1_val /= u;
        d2_val /= u;
        let tmp = d2_val;
        d2_val = d1_val;
        d1_val = tmp;
        (h11, h12, h21, h22, u)
    };

    let (mut h11, mut h12, mut h21, mut h22) = (h11, h12, h21, h22);

    while d1_val <= 1.0 / G2 && d1_val != 0.0 {
        params.flag = -1.0;
        d1_val *= G2;
        x /= G;
        h11 /= G;
        h12 /= G;
    }

    while d1_val >= G2 {
        params.flag = -1.0;
        d1_val /= G2;
        x *= G;
        h11 *= G;
        h12 *= G;
    }

    while d2_val.abs() <= 1.0 / G2 && d2_val != 0.0 {
        params.flag = -1.0;
        d2_val *= G2;
        h21 /= G;
        h22 /= G;
    }

    while d2_val.abs() >= G2 {
        params.flag = -1.0;
        d2_val /= G2;
        h21 *= G;
        h22 *= G;
    }

    *d1 = d1_val;
    *d2 = d2_val;
    *b1 = x;

    match params.flag {
        -1.0 => {
            params.h11 = h11;
            params.h21 = h21;
            params.h12 = h12;
            params.h22 = h22;
        },
        0.0 => {
            params.h21 = h21;
            params.h12 = h12;
        },
        1.0 => {
            params.h11 = h11;
            params.h22 = h22;
        },
        _ => {}
    }

    params
}