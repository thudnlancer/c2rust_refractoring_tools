use libm::{fabs, pow, sqrt};
use std::f64::consts::PI;

#[derive(Debug, Copy, Clone)]
pub struct GslSfResult {
    pub val: f64,
    pub err: f64,
}

#[derive(Debug)]
pub enum GslError {
    Dom,
    Einval,
    Eunimpl,
    Success,
    // Add other error variants as needed
}

impl GslError {
    fn as_int(&self) -> i32 {
        match self {
            GslError::Dom => 1,
            GslError::Einval => 4,
            GslError::Eunimpl => 24,
            GslError::Success => 0,
        }
    }
}

const COEF_JNU1_A: [f64; 24] = [
    3.801775243633476,
    1.360704737511120,
    -0.030707710261106,
    0.004526823746202,
    -0.000808682832134,
    0.000159218792489,
    -0.000033225189761,
    0.000007205599763,
    -0.000001606110397,
    0.000000365439424,
    -0.000000084498039,
    0.000000019793815,
    -0.000000004687054,
    0.000000001120052,
    -0.000000000269767,
    0.000000000065420,
    -0.000000000015961,
    0.000000000003914,
    -0.000000000000965,
    0.000000000000239,
    -0.000000000000059,
    0.000000000000015,
    -0.000000000000004,
    0.000000000000001,
];

const COEF_JNU1_B: [f64; 15] = [
    1.735063412537096,
    0.784478100951978,
    0.048881473180370,
    -0.000578279783021,
    -0.000038984957864,
    0.000005758297879,
    -0.000000327583229,
    -0.000000003853878,
    0.000000002284653,
    -0.000000000153079,
    -0.000000000000895,
    0.000000000000283,
    0.000000000000043,
    0.000000000000010,
    -0.000000000000003,
];

// Similarly define other COEF_JNU* constants...

fn clenshaw(c: &[f64], n: i32, u: f64) -> f64 {
    let mut b_np1 = 0.0;
    let mut b_n = c[n as usize];
    let mut b_nm1;
    
    for i in (1..=n).rev() {
        b_nm1 = 2.0 * (2.0 * u - 1.0) * b_n - b_np1 + c[(i - 1) as usize];
        b_np1 = b_n;
        b_n = b_nm1;
    }
    
    b_n - (2.0 * u - 1.0) * b_np1
}

fn mcmahon_correction(mu: f64, beta: f64) -> f64 {
    let eb = 8.0 * beta;
    let ebsq = eb * eb;
    
    if mu < 2.2204460492503131e-16 {
        let term1 = 1.0 / ebsq;
        let term2 = -4.0 * 31.0 / (3.0 * ebsq * ebsq);
        let term3 = 32.0 * 3779.0 / (15.0 * ebsq * ebsq * ebsq);
        let term4 = -64.0 * 6277237.0 / (105.0 * ebsq * ebsq * ebsq * ebsq);
        let term5 = 512.0 * 2092163573.0 / (315.0 * ebsq * ebsq * ebsq * ebsq * ebsq);
        1.0 + 8.0 * (term1 + term2 + term3 + term4 + term5)
    } else {
        let mi = 1.0 / mu;
        let r = mu / ebsq;
        let n2 = 4.0 / 3.0 * (7.0 - 31.0 * mi);
        let n3 = 32.0 / 15.0 * (83.0 + (-982.0 + 3779.0 * mi) * mi);
        let n4 = 64.0 / 105.0 * (6949.0 + (-153855.0 + (1585743.0 - 6277237.0 * mi) * mi) * mi);
        let n5 = 512.0 / 315.0 * (70197.0 + (-2479316.0 + (48010494.0 + (-512062548.0 + 2092163573.0 * mi) * mi) * mi);
        let n6 = 2048.0 / 3465.0 * (5592657.0 + (-287149133.0 + (8903961290.0 + (-179289628602.0 + (1982611456181.0 - 8249725736393.0 * mi) * mi) * mi) * mi);
        
        let term1_0 = (1.0 - mi) * r;
        let term2_0 = term1_0 * n2 * r;
        let term3_0 = term1_0 * n3 * r * r;
        let term4_0 = term1_0 * n4 * r * r * r;
        let term5_0 = term1_0 * n5 * r * r * r * r;
        let term6 = term1_0 * n6 * r * r * r * r * r;
        
        1.0 - 8.0 * (term1_0 + term2_0 + term3_0 + term4_0 + term5_0 + term6)
    }
}

fn olver_b0(z: f64, minus_zeta: f64) -> f64 {
    if z < 1.02 {
        let a = 1.0 - z;
        let c0 = 0.0179988721413553309252458658183;
        let c1 = 0.0111992982212877614645974276203;
        let c2 = 0.0059404069786014304317781160605;
        let c3 = 0.0028676724516390040844556450173;
        let c4 = 0.0012339189052567271708525111185;
        let c5 = 0.0004169250674535178764734660248;
        let c6 = 0.0000330173385085949806952777365;
        let c7 = -0.0001318076238578203009990106425;
        let c8 = -0.0001906870370050847239813945647;
        
        c0 + a * (c1 + a * (c2 + a * (c3 + a * (c4 + a * (c5 + a * (c6 + a * (c7 + a * c8))))))
    } else {
        let abs_zeta = minus_zeta;
        let t = 1.0 / (z * sqrt(1.0 - 1.0 / (z * z)));
        -5.0 / (48.0 * abs_zeta * abs_zeta) + t * (3.0 + 5.0 * t * t) / (24.0 * sqrt(abs_zeta))
    }
}

fn olver_f1(z: f64, minus_zeta: f64) -> f64 {
    let b0 = olver_b0(z, minus_zeta);
    let h2 = sqrt(4.0 * minus_zeta / (z * z - 1.0));
    0.5 * z * h2 * b0
}

pub fn gsl_sf_bessel_zero_j0_e(s: u32, result: &mut GslSfResult) -> Result<(), GslError> {
    if s == 0 {
        result.val = 0.0;
        result.err = 0.0;
        return Err(GslError::Einval);
    }

    let p = [
        1567450796.0 / 12539606369.0,
        8903660.0 / 2365861.0,
        10747040.0 / 536751.0,
        17590991.0 / 1696654.0,
    ];
    
    let q = [
        1.0,
        29354255.0 / 954518.0,
        76900001.0 / 431847.0,
        67237052.0 / 442411.0,
    ];
    
    let beta = (s as f64 - 0.25) * PI;
    let bi2 = 1.0 / (beta * beta);
    
    let r33num = p[0] + bi2 * (p[1] + bi2 * (p[2] + p[3] * bi2));
    let r33den = q[0] + bi2 * (q[1] + bi2 * (q[2] + q[3] * bi2));
    let r33 = r33num / r33den;
    
    result.val = beta + r33 / beta;
    result.err = fabs(3.0e-15 * result.val);
    
    Ok(())
}

// Similarly implement other functions...

pub fn gsl_sf_bessel_zero_j0(s: u32) -> f64 {
    let mut result = GslSfResult { val: 0.0, err: 0.0 };
    match gsl_sf_bessel_zero_j0_e(s, &mut result) {
        Ok(_) => result.val,
        Err(e) => {
            // Handle error appropriately
            result.val
        }
    }
}

// Similarly implement other wrapper functions...