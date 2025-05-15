use std::mem;
use std::ptr;
use std::slice;

const ABAR: [f64; 13] = [
    14005451.0 / 335480064.0,
    0.0,
    0.0,
    0.0,
    0.0,
    -59238493.0 / 1068277825.0,
    181606767.0 / 758867731.0,
    561292985.0 / 797845732.0,
    -1041891430.0 / 1371343529.0,
    760417239.0 / 1151165299.0,
    118820643.0 / 751138087.0,
    -528747749.0 / 2220607170.0,
    1.0 / 4.0,
];

const A: [f64; 12] = [
    13451932.0 / 455176623.0,
    0.0,
    0.0,
    0.0,
    0.0,
    -808719846.0 / 976000145.0,
    1757004468.0 / 5645159321.0,
    656045339.0 / 265891186.0,
    -3867574721.0 / 1518517206.0,
    465885868.0 / 322736535.0,
    53011238.0 / 667516719.0,
    2.0 / 45.0,
];

const AH: [f64; 10] = [
    1.0 / 18.0,
    1.0 / 12.0,
    1.0 / 8.0,
    5.0 / 16.0,
    3.0 / 8.0,
    59.0 / 400.0,
    93.0 / 200.0,
    5490023248.0 / 9719169821.0,
    13.0 / 20.0,
    1201146811.0 / 1299019798.0,
];

const B21: f64 = 1.0 / 18.0;
const B3: [f64; 2] = [1.0 / 48.0, 1.0 / 16.0];
const B4: [f64; 3] = [1.0 / 32.0, 0.0, 3.0 / 32.0];
const B5: [f64; 4] = [5.0 / 16.0, 0.0, -75.0 / 64.0, 75.0 / 64.0];
const B6: [f64; 5] = [3.0 / 80.0, 0.0, 0.0, 3.0 / 16.0, 3.0 / 20.0];

const B7: [f64; 6] = [
    29443841.0 / 614563906.0,
    0.0,
    0.0,
    77736538.0 / 692538347.0,
    -28693883.0 / 1125000000.0,
    23124283.0 / 1800000000.0,
];

const B8: [f64; 7] = [
    16016141.0 / 946692911.0,
    0.0,
    0.0,
    61564180.0 / 158732637.0,
    22789713.0 / 633445777.0,
    545815736.0 / 2771057229.0,
    -180193667.0 / 1043307555.0,
];

const B9: [f64; 8] = [
    39632708.0 / 573591083.0,
    0.0,
    0.0,
    -433636366.0 / 683701615.0,
    -421739975.0 / 2616292301.0,
    100302831.0 / 723423059.0,
    790204164.0 / 839813087.0,
    800635310.0 / 3783071287.0,
];

const B10: [f64; 9] = [
    246121993.0 / 1340847787.0,
    0.0,
    0.0,
    -37695042795.0 / 15268766246.0,
    -309121744.0 / 1061227803.0,
    -12992083.0 / 490766935.0,
    6005943493.0 / 2108947869.0,
    393006217.0 / 1396673457.0,
    123872331.0 / 1001029789.0,
];

const B11: [f64; 10] = [
    -1028468189.0 / 846180014.0,
    0.0,
    0.0,
    8478235783.0 / 508512852.0,
    1311729495.0 / 1432422823.0,
    -10304129995.0 / 1701304382.0,
    -48777925059.0 / 3047939560.0,
    15336726248.0 / 1032824649.0,
    -45442868181.0 / 3398467696.0,
    3065993473.0 / 597172653.0,
];

const B12: [f64; 11] = [
    185892177.0 / 718116043.0,
    0.0,
    0.0,
    -3185094517.0 / 667107341.0,
    -477755414.0 / 1098053517.0,
    -703635378.0 / 230739211.0,
    5731566787.0 / 1027545527.0,
    5232866602.0 / 850066563.0,
    -4093664535.0 / 808688257.0,
    3962137247.0 / 1805957418.0,
    65686358.0 / 487910083.0,
];

const B13: [f64; 12] = [
    403863854.0 / 491063109.0,
    0.0,
    0.0,
    -5068492393.0 / 434740067.0,
    -411421997.0 / 543043805.0,
    652783627.0 / 914296604.0,
    11173962825.0 / 925320556.0,
    -13158990841.0 / 6184727034.0,
    3936647629.0 / 1978049680.0,
    -160528059.0 / 685178525.0,
    248638103.0 / 1413531060.0,
    0.0,
];

struct Rk8pdState {
    k: [Vec<f64>; 13],
    ytmp: Vec<f64>,
    y0: Vec<f64>,
}

impl Rk8pdState {
    fn new(dim: usize) -> Option<Self> {
        let mut k = [
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
        ];

        for k_vec in k.iter_mut() {
            *k_vec = vec![0.0; dim];
            if k_vec.len() != dim {
                return None;
            }
        }

        let ytmp = vec![0.0; dim];
        let y0 = vec![0.0; dim];

        if ytmp.len() != dim || y0.len() != dim {
            return None;
        }

        Some(Rk8pdState { k, ytmp, y0 })
    }

    fn reset(&mut self, dim: usize) {
        for k_vec in self.k.iter_mut() {
            k_vec.fill(0.0);
        }
        self.ytmp.fill(0.0);
        self.y0.fill(0.0);
    }
}

struct Odeiv2System {
    function: fn(f64, &[f64], &mut [f64]) -> Result<(), ()>,
    jacobian: Option<fn(f64, &[f64], &mut [f64], &mut [f64]) -> Result<(), ()>>,
    dimension: usize,
    params: (),
}

fn rk8pd_apply(
    state: &mut Rk8pdState,
    dim: usize,
    t: f64,
    h: f64,
    y: &mut [f64],
    yerr: &mut [f64],
    dydt_in: Option<&[f64]>,
    dydt_out: Option<&mut [f64]>,
    sys: &Odeiv2System,
) -> Result<(), ()> {
    let ytmp = &mut state.ytmp;
    let y0 = &mut state.y0;
    let k = &mut state.k;

    y0.copy_from_slice(y);

    // k1 step
    if let Some(dydt_in) = dydt_in {
        k[0].copy_from_slice(dydt_in);
    } else {
        (sys.function)(t, y, &mut k[0])?;
    }

    for i in 0..dim {
        ytmp[i] = y[i] + B21 * h * k[0][i];
    }

    // k2 step
    (sys.function)(t + AH[0] * h, ytmp, &mut k[1])?;

    for i in 0..dim {
        ytmp[i] = y[i] + h * (B3[0] * k[0][i] + B3[1] * k[1][i]);
    }

    // k3 step
    (sys.function)(t + AH[1] * h, ytmp, &mut k[2])?;

    for i in 0..dim {
        ytmp[i] = y[i] + h * (B4[0] * k[0][i] + B4[2] * k[2][i]);
    }

    // k4 step
    (sys.function)(t + AH[2] * h, ytmp, &mut k[3])?;

    for i in 0..dim {
        ytmp[i] = y[i] + h * (B5[0] * k[0][i] + B5[2] * k[2][i] + B5[3] * k[3][i]);
    }

    // k5 step
    (sys.function)(t + AH[3] * h, ytmp, &mut k[4])?;

    for i in 0..dim {
        ytmp[i] = y[i] + h * (B6[0] * k[0][i] + B6[3] * k[3][i] + B6[4] * k[4][i]);
    }

    // k6 step
    (sys.function)(t + AH[4] * h, ytmp, &mut k[5])?;

    for i in 0..dim {
        ytmp[i] = y[i] + h * (B7[0] * k[0][i] + B7[3] * k[3][i] + B7[4] * k[4][i] + B7[5] * k[5][i]);
    }

    // k7 step
    (sys.function)(t + AH[5] * h, ytmp, &mut k[6])?;

    for i in 0..dim {
        ytmp[i] = y[i]
            + h * (B8[0] * k[0][i]
                + B8[3] * k[3][i]
                + B8[4] * k[4][i]
                + B8[5] * k[5][i]
                + B8[6] * k[6][i]);
    }

    // k8 step
    (sys.function)(t + AH[6] * h, ytmp, &mut k[7])?;

    for i in 0..dim {
        ytmp[i] = y[i]
            + h * (B9[0] * k[0][i]
                + B9[3] * k[3][i]
                + B9[4] * k[4][i]
                + B9[5] * k[5][i]
                + B9[6] * k[6][i]
                + B9[7] * k[7][i]);
    }

    // k9 step
    (sys.function)(t + AH[7] * h, ytmp, &mut k[8])?;

    for i in 0..dim {
        ytmp[i] = y[i]
            + h * (B10[0] * k[0][i]
                + B10[3] * k[3][i]
                + B10[4] * k[4][i]
                + B10[5] * k[5][i]
                + B10[6] * k[6][i]
                + B10[7] * k[7][i]
                + B10[8] * k[8][i]);
    }

    // k10 step
    (sys.function)(t + AH[8] * h, ytmp, &mut k[9])?;

    for i in 0..dim {
        ytmp[i] = y[i]
            + h * (B11[0] * k[0][i]
                + B11[3] * k[3][i]
                + B11[4] * k[4][i]
                + B11[5] * k[5][i]
                + B11[6] * k[6][i]
                + B11[7] * k[7][i]
                + B11[8] * k[8][i]
                + B11[9] * k[9][i]);
    }

    // k11 step
    (sys.function)(t + AH[9] * h, ytmp, &mut k[10])?;

    for i in 0..dim {
        ytmp[i] = y[i]
            + h * (B12[0] * k[0][i]
                + B12[3] * k[3][i]
                + B12[4] * k[4][i]
                + B12[5] * k[5][i]
                + B12[6] * k[6][i]
                + B12[7] * k[7][i]
                + B12[8] * k[8][i]
                + B12[9] * k[9][i]
                + B12[10] * k[10][i]);
    }

    // k12 step
    (sys.function)(t + h, ytmp, &mut k[11])?;

    for i in 0..dim {
        ytmp[i] = y[i]
            + h * (B13[0] * k[0][i]
                + B13[3] * k[3][i]
                + B13[4] * k[4][i]
                + B13[5] * k[5][i]
                + B13[6] * k[6][i]
                + B13[7] * k[7][i]
                + B13[8] * k[8][i]
                + B13[9] * k[9][i]
                + B13[10] * k[10][i]
                + B13[11] * k[11][i]);
    }

    // k13 step
    (sys.function)(t + h, ytmp, &mut k[12])?;

    // final sum
    for i in 0..dim {
        let ksum8 = ABAR[0] * k[0][i]
            + ABAR[5] * k[5][i]
            + ABAR[6] * k[6][i]
            + ABAR[7] * k[7][i]
            + ABAR[8] * k[8][i]
            + ABAR[9] * k[9][i]
            + ABAR[10] * k[10][i]
            + ABAR[11] * k[11][i]
            + ABAR[12] * k[12][i];
        y[i] += h * ksum8;
    }

    // Evaluate dydt_out
    if let Some(dydt_out) = dydt_out {
        (sys.function)(t + h, y, dydt_out)?;
    }

    // error estimate
    for i in 0..dim {
        let ksum8 = ABAR[0] * k[0][i]
            + ABAR[5] * k[5][i]
            + ABAR[6] * k[6][i]
            + ABAR[7] * k[7][i]
            + ABAR[8] * k[8][i]
            + ABAR[9] * k[9][i]
            + ABAR[10] * k[10][i]
            + ABAR[11] * k[11][i]
            + ABAR[12] * k[12][i];
        let ksum7 = A[0] * k[0][i]
            + A[5] * k[5][i]
            + A[6] * k[6][i]
            + A[7] * k[7][i]
            + A[8] * k[8][i]
            + A[9] * k[9][i]
            + A[10] * k[10][i]
            + A[11] * k[11][i];
        yerr[i] = h * (ksum7 - ksum8);
    }

    Ok(())
}

fn rk8pd_order() -> u32 {
    8
}