use std::error::Error;
use std::fmt;
use std::ptr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

impl fmt::Display for GslError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for GslError {}

pub struct GslRngType {
    name: &'static str,
    max: u64,
    min: u64,
    size: usize,
    set: Option<fn(&mut [u8], u64)>,
    get: Option<fn(&[u8]) -> u64>,
    get_double: Option<fn(&[u8]) -> f64>,
}

pub struct GslRng {
    type_: &'static GslRngType,
    state: Vec<u8>,
}

pub struct GslRanDiscrete {
    k: usize,
    a: Vec<usize>,
    f: Vec<f64>,
}

struct Stack {
    data: Vec<usize>,
}

impl Stack {
    fn new(capacity: usize) -> Self {
        Stack {
            data: Vec::with_capacity(capacity),
        }
    }

    fn push(&mut self, value: usize) -> Result<(), GslError> {
        if self.data.len() >= self.data.capacity() {
            return Err(GslError::Failure);
        }
        self.data.push(value);
        Ok(())
    }

    fn pop(&mut self) -> Result<usize, GslError> {
        self.data.pop().ok_or(GslError::Sanity)
    }

    fn size(&self) -> usize {
        self.data.len()
    }
}

pub fn gsl_ran_discrete_preproc(
    k_events: usize,
    prob_array: &[f64],
) -> Result<GslRanDiscrete, GslError> {
    if k_events == 0 {
        return Err(GslError::Invalid);
    }

    if prob_array.len() < k_events {
        return Err(GslError::Invalid);
    }

    if prob_array.iter().any(|&p| p < 0.0) {
        return Err(GslError::Invalid);
    }

    let p_total: f64 = prob_array.iter().take(k_events).sum();
    if p_total <= 0.0 {
        return Err(GslError::Invalid);
    }

    let mut e: Vec<f64> = prob_array.iter().take(k_events).map(|&p| p / p_total).collect();
    let mean = 1.0 / k_events as f64;

    let mut which = vec![0; k_events];
    let mut n_bigs = 0;
    let mut n_smalls = 0;

    for k in 0..k_events {
        if e[k] < mean {
            n_smalls += 1;
            which[k] = 0;
        } else {
            n_bigs += 1;
            which[k] = 1;
        }
    }

    let mut bigs = Stack::new(n_bigs);
    let mut smalls = Stack::new(n_smalls);

    for k in 0..k_events {
        let dest = if which[k] != 0 { &mut bigs } else { &mut smalls };
        dest.push(k)?;
    }

    let mut a = vec![0; k_events];
    let mut f = vec![0.0; k_events];

    while smalls.size() > 0 {
        let s = smalls.pop()?;
        if bigs.size() == 0 {
            a[s] = s;
            f[s] = 1.0;
        } else {
            let b = bigs.pop()?;
            a[s] = b;
            f[s] = k_events as f64 * e[s];
            let d = mean - e[s];
            e[s] += d;
            e[b] -= d;

            if e[b] < mean {
                smalls.push(b)?;
            } else if e[b] > mean {
                bigs.push(b)?;
            } else {
                a[b] = b;
                f[b] = 1.0;
            }
        }
    }

    while bigs.size() > 0 {
        let b = bigs.pop()?;
        a[b] = b;
        f[b] = 1.0;
    }

    if smalls.size() != 0 {
        return Err(GslError::Sanity);
    }

    for k in 0..k_events {
        f[k] = (f[k] + k as f64) / k_events as f64;
    }

    Ok(GslRanDiscrete { k: k_events, a, f })
}

pub fn gsl_ran_discrete(rng: &GslRng, g: &GslRanDiscrete) -> usize {
    let u = rng.uniform();
    let c = (u * g.k as f64) as usize;
    let f = g.f[c];
    if f == 1.0 || u < f {
        c
    } else {
        g.a[c]
    }
}

pub fn gsl_ran_discrete_pdf(k: usize, g: &GslRanDiscrete) -> f64 {
    if k >= g.k {
        return 0.0;
    }

    let mut p = 0.0;
    for i in 0..g.k {
        let mut f = g.f[i];
        f = g.k as f64 * f - i as f64;
        if i == k {
            p += f;
        } else if k == g.a[i] {
            p += 1.0 - f;
        }
    }
    p / g.k as f64
}

impl GslRng {
    fn uniform(&self) -> f64 {
        if let Some(get_double) = self.type_.get_double {
            get_double(&self.state)
        } else {
            panic!("No get_double function defined");
        }
    }
}