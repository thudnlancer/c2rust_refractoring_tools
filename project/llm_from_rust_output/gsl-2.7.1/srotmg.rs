use std::f32;

const G: f32 = 4096.0;
const G2: f32 = G * G;

#[derive(Debug, Default)]
pub struct RotmgParameters {
    flag: f32,
    h11: f32,
    h21: f32,
    h12: f32,
    h22: f32,
}

pub fn cblas_srotmg(
    d1: &mut f32,
    d2: &mut f32,
    b1: &mut f32,
    b2: f32,
    p: &mut RotmgParameters,
) {
    let mut params = RotmgParameters::default();
    let mut d1_val = *d1;
    let mut d2_val = *d2;
    let mut x = *b1;
    let y = b2;

    if d1_val < 0.0 {
        *p = RotmgParameters {
            flag: -1.0,
            ..Default::default()
        };
        *d1 = 0.0;
        *d2 = 0.0;
        *b1 = 0.0;
        return;
    }

    if d2_val * y == 0.0 {
        p.flag = -2.0;
        return;
    }

    let c = (d1_val * x * x).abs();
    let s = (d2_val * y * y).abs();

    if c > s {
        params.flag = 0.0;
        params.h11 = 1.0;
        params.h12 = d2_val * y / (d1_val * x);
        params.h21 = -y / x;
        params.h22 = 1.0;
        let u = 1.0 - params.h21 * params.h12;

        if u <= 0.0 {
            *p = RotmgParameters {
                flag: -1.0,
                ..Default::default()
            };
            *d1 = 0.0;
            *d2 = 0.0;
            *b1 = 0.0;
            return;
        }

        d1_val /= u;
        d2_val /= u;
        x *= u;
    } else {
        if d2_val * y * y < 0.0 {
            *p = RotmgParameters {
                flag: -1.0,
                ..Default::default()
            };
            *d1 = 0.0;
            *d2 = 0.0;
            *b1 = 0.0;
            return;
        }

        params.flag = 1.0;
        params.h11 = d1_val * x / (d2_val * y);
        params.h12 = 1.0;
        params.h21 = -1.0;
        params.h22 = x / y;
        let u = 1.0 + params.h11 * params.h22;
        d1_val /= u;
        d2_val /= u;
        std::mem::swap(&mut d1_val, &mut d2_val);
        x = y * u;
    }

    while d1_val <= 1.0 / G2 && d1_val != 0.0 {
        params.flag = -1.0;
        d1_val *= G2;
        x /= G;
        params.h11 /= G;
        params.h12 /= G;
    }

    while d1_val >= G2 {
        params.flag = -1.0;
        d1_val /= G2;
        x *= G;
        params.h11 *= G;
        params.h12 *= G;
    }

    while d2_val.abs() <= 1.0 / G2 && d2_val != 0.0 {
        params.flag = -1.0;
        d2_val *= G2;
        params.h21 /= G;
        params.h22 /= G;
    }

    while d2_val.abs() >= G2 {
        params.flag = -1.0;
        d2_val /= G2;
        params.h21 *= G;
        params.h22 *= G;
    }

    *d1 = d1_val;
    *d2 = d2_val;
    *b1 = x;

    if params.flag == -1.0 {
        *p = RotmgParameters {
            flag: params.flag,
            h11: params.h11,
            h21: params.h21,
            h12: params.h12,
            h22: params.h22,
        };
    } else if params.flag == 0.0 {
        *p = RotmgParameters {
            flag: params.flag,
            h21: params.h21,
            h12: params.h12,
            ..Default::default()
        };
    } else if params.flag == 1.0 {
        *p = RotmgParameters {
            flag: params.flag,
            h11: params.h11,
            h22: params.h22,
            ..Default::default()
        };
    }
}