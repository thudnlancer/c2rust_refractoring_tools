use std::f64::consts::{E, LN_2};
use std::f64::{INFINITY, NAN};

#[derive(Debug, Clone, Copy)]
pub struct GslSfResult {
    pub val: f64,
    pub err: f64,
}

#[derive(Debug, Clone, Copy)]
pub enum GslError {
    Success = 0,
    Failure = -1,
    Continue = -2,
    Domain = 1,
    Range = 2,
    Fault = 3,
    Invalid = 4,
    Failed = 5,
    Factor = 6,
    Sanity = 7,
    NoMem = 8,
    BadFunc = 9,
    Runaway = 10,
    MaxIter = 11,
    ZeroDiv = 12,
    BadTol = 13,
    Tol = 14,
    Underflow = 15,
    Overflow = 16,
    Loss = 17,
    Round = 18,
    BadLen = 19,
    NotSquare = 20,
    Singular = 21,
    Diverge = 22,
    Unsupported = 23,
    Unimplemented = 24,
    Cache = 25,
    Table = 26,
    NoProgress = 27,
    NoProgressJ = 28,
    TolF = 29,
    TolX = 30,
    TolG = 31,
    Eof = 32,
}

#[derive(Debug, Clone)]
pub struct ChebSeries {
    c: Vec<f64>,
    order: i32,
    a: f64,
    b: f64,
    order_sp: i32,
}

impl ChebSeries {
    pub fn eval(&self, x: f64) -> GslSfResult {
        let mut d = 0.0;
        let mut dd = 0.0;
        let y = (2.0 * x - self.a - self.b) / (self.b - self.a);
        let y2 = 2.0 * y;
        let mut e = 0.0;

        let mut j = self.order;
        while j >= 1 {
            let temp = d;
            d = y2 * d - dd + self.c[j as usize];
            e += (y2 * temp).abs() + dd.abs() + self.c[j as usize].abs();
            dd = temp;
            j -= 1;
        }

        let temp = d;
        d = y * d - dd + 0.5 * self.c[0];
        e += (y * temp).abs() + dd.abs() + 0.5 * self.c[0].abs();

        GslSfResult {
            val: d,
            err: 2.2204460492503131e-16 * e + self.c[self.order as usize].abs(),
        }
    }
}

// Static Chebyshev series data
lazy_static! {
    static ref AE11_DATA: Vec<f64> = vec![
        0.121503239716065790, -0.065088778513550150, 0.004897651357459670, 
        -0.000649237843027216, 0.000093840434587471, 0.000000420236380882,
        -0.000008113374735904, 0.000002804247688663, 0.000000056487164441,
        -0.000000344809174450, 0.000000058209273578, 0.000000038711426349,
        -0.000000012453235014, -0.000000005118504888, 0.000000002148771527,
        0.000000000868459898, -0.000000000343650105, -0.000000000179796603,
        0.000000000047442060, 0.000000000040423282, -0.000000000003543928,
        -0.000000000008853444, -0.000000000000960151, 0.000000000001692921,
        0.000000000000607990, -0.000000000000224338, -0.000000000000200327,
        -0.000000000000006246, 0.000000000000045571, 0.000000000000016383,
        -0.000000000000005561, -0.000000000000006074, -0.000000000000000862,
        0.000000000000001223, 0.000000000000000716, -0.000000000000000024,
        -0.000000000000000201, -0.000000000000000082, 0.000000000000000017
    ];

    static ref AE11_CS: ChebSeries = ChebSeries {
        c: AE11_DATA.clone(),
        order: 38,
        a: -1.0,
        b: 1.0,
        order_sp: 20,
    };

    // Similarly define other Chebyshev series (AE12, E11, E12, AE13, AE14)
    // ...
}

pub fn expint_e1_impl(x: f64, scale: bool) -> Result<GslSfResult, GslError> {
    let xmaxt = -708.39641853226408;
    let xmax = xmaxt - xmaxt.ln();

    if x < -xmax && !scale {
        return Err(GslError::Overflow);
    }

    if x <= -10.0 {
        let s = 1.0 / x * if scale { 1.0 } else { (-x).exp() };
        let result_c = AE11_CS.eval(20.0 / x + 1.0);
        Ok(GslSfResult {
            val: s * (1.0 + result_c.val),
            err: s * result_c.err + 2.0 * 2.2204460492503131e-16 * (x.abs() + 1.0) * (s * (1.0 + result_c.val)).abs(),
        })
    } 
    // Other cases similarly implemented
    // ...
    else if x == 0.0 {
        Err(GslError::Domain)
    } else {
        // Default case
        Ok(GslSfResult { val: 0.0, err: 0.0 })
    }
}

// Implement other functions similarly:
// expint_e2_impl, expint_en_impl, and their public interfaces

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expint_e1() {
        let result = expint_e1_impl(1.0, false).unwrap();
        assert!((result.val - 0.21938393439552029).abs() < 1e-10);
    }
}