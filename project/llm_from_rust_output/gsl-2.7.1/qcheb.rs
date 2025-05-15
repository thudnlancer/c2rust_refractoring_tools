use std::f64;

pub type SizeT = usize;

#[derive(Copy, Clone)]
pub struct GslFunction {
    pub function: fn(f64) -> f64,
}

pub fn gsl_integration_qcheb(
    f: GslFunction,
    a: f64,
    b: f64,
    cheb12: &mut [f64; 13],
    cheb24: &mut [f64; 25],
) {
    let mut fval = [0.0; 25];
    let mut v = [0.0; 12];
    let x = [
        0.9914448613738104,
        0.9659258262890683,
        0.9238795325112868,
        0.8660254037844386,
        0.7933533402912352,
        0.7071067811865475,
        0.6087614290087206,
        0.5000000000000000,
        0.3826834323650898,
        0.2588190451025208,
        0.1305261922200516,
    ];

    let center = 0.5 * (b + a);
    let half_length = 0.5 * (b - a);

    fval[0] = 0.5 * (f.function)(b);
    fval[12] = (f.function)(center);
    fval[24] = 0.5 * (f.function)(a);

    for i in 1..12 {
        let j = 24 - i;
        let u = half_length * x[i - 1];
        fval[i] = (f.function)(center + u);
        fval[j] = (f.function)(center - u);
    }

    for i in 0..12 {
        let j = 24 - i;
        v[i] = fval[i] - fval[j];
        fval[i] = fval[i] + fval[j];
    }

    let alam1 = v[0] - v[8];
    let alam2 = x[5] * (v[2] - v[6] - v[10]);
    cheb12[3] = alam1 + alam2;
    cheb12[9] = alam1 - alam2;

    let alam1 = v[1] - v[7] - v[9];
    let alam2 = v[3] - v[5] - v[11];
    let alam = x[2] * alam1 + x[8] * alam2;
    cheb24[3] = cheb12[3] + alam;
    cheb24[21] = cheb12[3] - alam;
    let alam = x[8] * alam1 - x[2] * alam2;
    cheb24[9] = cheb12[9] + alam;
    cheb24[15] = cheb12[9] - alam;

    let part1 = x[3] * v[4];
    let part2 = x[7] * v[8];
    let part3 = x[5] * v[6];
    let alam1 = v[0] + part1 + part2;
    let alam2 = x[1] * v[2] + part3 + x[9] * v[10];
    cheb12[1] = alam1 + alam2;
    cheb12[11] = alam1 - alam2;
    let alam1 = v[0] - part1 + part2;
    let alam2 = x[9] * v[2] - part3 + x[1] * v[10];
    cheb12[5] = alam1 + alam2;
    cheb12[7] = alam1 - alam2;

    let alam = x[0] * v[1] + x[2] * v[3] + x[4] * v[5] + x[6] * v[7] + x[8] * v[9] + x[10] * v[11];
    cheb24[1] = cheb12[1] + alam;
    cheb24[23] = cheb12[1] - alam;
    let alam = x[10] * v[1] - x[8] * v[3] + x[6] * v[5] - x[4] * v[7] + x[2] * v[9] - x[0] * v[11];
    cheb24[11] = cheb12[11] + alam;
    cheb24[13] = cheb12[11] - alam;
    let alam = x[4] * v[1] - x[8] * v[3] - x[0] * v[5] - x[10] * v[7] + x[2] * v[9] + x[6] * v[11];
    cheb24[5] = cheb12[5] + alam;
    cheb24[19] = cheb12[5] - alam;
    let alam = x[6] * v[1] - x[2] * v[3] - x[10] * v[5] + x[0] * v[7] - x[8] * v[9] - x[4] * v[11];
    cheb24[7] = cheb12[7] + alam;
    cheb24[17] = cheb12[7] - alam;

    for i in 0..6 {
        let j = 12 - i;
        v[i] = fval[i] - fval[j];
        fval[i] = fval[i] + fval[j];
    }

    let alam1 = v[0] + x[7] * v[4];
    let alam2 = x[3] * v[2];
    cheb12[2] = alam1 + alam2;
    cheb12[10] = alam1 - alam2;
    cheb12[6] = v[0] - v[4];

    let alam = x[1] * v[1] + x[5] * v[3] + x[9] * v[5];
    cheb24[2] = cheb12[2] + alam;
    cheb24[22] = cheb12[2] - alam;
    let alam = x[5] * (v[1] - v[3] - v[5]);
    cheb24[6] = cheb12[6] + alam;
    cheb24[18] = cheb12[6] - alam;
    let alam = x[9] * v[1] - x[5] * v[3] + x[1] * v[5];
    cheb24[10] = cheb12[10] + alam;
    cheb24[14] = cheb12[10] - alam;

    for i in 0..3 {
        let j = 6 - i;
        v[i] = fval[i] - fval[j];
        fval[i] = fval[i] + fval[j];
    }

    cheb12[4] = v[0] + x[7] * v[2];
    cheb12[8] = fval[0] - x[7] * fval[2];
    let alam = x[3] * v[1];
    cheb24[4] = cheb12[4] + alam;
    cheb24[20] = cheb12[4] - alam;
    let alam = x[7] * fval[1] - fval[3];
    cheb24[8] = cheb12[8] + alam;
    cheb24[16] = cheb12[8] - alam;

    cheb12[0] = fval[0] + fval[2];
    let alam = fval[1] + fval[3];
    cheb24[0] = cheb12[0] + alam;
    cheb24[24] = cheb12[0] - alam;
    cheb12[12] = v[0] - v[2];
    cheb24[12] = cheb12[12];

    for i in 1..12 {
        cheb12[i] /= 6.0;
    }
    cheb12[0] /= 12.0;
    cheb12[12] /= 12.0;

    for i in 1..24 {
        cheb24[i] /= 12.0;
    }
    cheb24[0] /= 24.0;
    cheb24[24] /= 24.0;
}