use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GslError {
    Domain,
    Range,
    Fault,
    Invalid,
    Failed,
    Factor,
    Sanity,
    NoMem,
    BadFunc,
    Runaway,
    MaxIter,
    ZeroDiv,
    BadTol,
    Tol,
    Underflow,
    Overflow,
    Loss,
    Round,
    BadLen,
    NotSquare,
    Singular,
    Diverge,
    Unsupported,
    Unimplemented,
    Cache,
    Table,
    NoProgress,
    NoProgressJ,
    TolF,
    TolX,
    TolG,
    Eof,
    Continue,
    Failure,
    Success,
}

impl fmt::Display for GslError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for GslError {}

pub type GslResult<T> = Result<T, GslError>;

#[derive(Debug, Clone, Copy)]
pub enum MovstatEndType {
    PadZero,
    PadValue,
    Truncate,
}

#[derive(Debug)]
pub struct GslBlock {
    size: usize,
    data: Vec<f64>,
}

impl GslBlock {
    pub fn new(size: usize) -> Self {
        GslBlock {
            size,
            data: vec![0.0; size],
        }
    }
}

#[derive(Debug)]
pub struct GslVector {
    size: usize,
    stride: usize,
    data: Vec<f64>,
    block: GslBlock,
    owner: bool,
}

impl GslVector {
    pub fn new(size: usize) -> Self {
        let block = GslBlock::new(size);
        GslVector {
            size,
            stride: 1,
            data: block.data.clone(),
            block,
            owner: true,
        }
    }

    pub fn get(&self, i: usize) -> f64 {
        self.data[i * self.stride]
    }

    pub fn set(&mut self, i: usize, x: f64) {
        self.data[i * self.stride] = x;
    }
}

pub trait MovstatAccum {
    fn size(&self, k: usize) -> usize;
    fn init(&mut self, k: usize, state: &mut [u8]) -> GslResult<()>;
    fn insert(&mut self, value: f64, state: &mut [u8]) -> GslResult<()>;
    fn delete_oldest(&mut self, state: &mut [u8]) -> GslResult<()>;
    fn get(&self, params: &[u8], state: &[u8], result: &mut [f64]) -> GslResult<()>;
}

pub struct MovstatFunction<F> {
    function: F,
    params: Vec<u8>,
}

impl<F> MovstatFunction<F>
where
    F: Fn(usize, &[f64], &[u8]) -> f64,
{
    pub fn new(function: F, params: Vec<u8>) -> Self {
        MovstatFunction { function, params }
    }
}

pub struct MovstatWorkspace {
    h: usize,
    j: usize,
    k: usize,
    work: Vec<f64>,
    state: Vec<u8>,
}

impl MovstatWorkspace {
    pub fn new(h: usize, j: usize, k: usize, state_size: usize) -> Self {
        MovstatWorkspace {
            h,
            j,
            k,
            work: Vec::new(),
            state: vec![0; state_size],
        }
    }
}

pub fn movstat_apply_accum<A: MovstatAccum>(
    endtype: MovstatEndType,
    x: &GslVector,
    accum: &mut A,
    accum_params: &[u8],
    y: &mut GslVector,
    z: Option<&mut GslVector>,
    w: &mut MovstatWorkspace,
) -> GslResult<()> {
    if x.size != y.size {
        return Err(GslError::BadLen);
    }
    if let Some(z) = z {
        if x.size != z.size {
            return Err(GslError::BadLen);
        }
    }

    let n = x.size;
    let h = w.h;
    let j = w.j;
    let mut result = [0.0; 2];

    accum.init(w.k, &mut w.state)?;

    match endtype {
        MovstatEndType::Truncate => {
            if accum.delete_oldest(&mut w.state).is_err() {
                let idx1 = if n > j + h { n - j - h } else { 0 };
                let idx2 = n - 1;
                w.work.clear();
                for i in idx1..=idx2 {
                    w.work.push(x.get(i));
                }
            }
        }
        MovstatEndType::PadZero => {
            for _ in 0..h {
                accum.insert(0.0, &mut w.state)?;
            }
        }
        MovstatEndType::PadValue => {
            let x1 = x.get(0);
            let xn = x.get(n - 1);
            for _ in 0..h {
                accum.insert(x1, &mut w.state)?;
            }
        }
    }

    for i in 0..n {
        let xi = x.get(i);
        accum.insert(xi, &mut w.state)?;
        let idx = i as isize - j as isize;
        if idx >= 0 {
            accum.get(accum_params, &w.state, &mut result)?;
            y.set(idx as usize, result[0]);
            if let Some(z) = z {
                z.set(idx as usize, result[1]);
            }
        }
    }

    match endtype {
        MovstatEndType::Truncate => {
            let idx1 = if n > j { n - j } else { 0 };
            let idx2 = n - 1;
            if accum.delete_oldest(&mut w.state).is_err() {
                let wsize = n - if n > j + h { n - j - h } else { 0 };
                for i in idx1..=idx2 {
                    let nsamp = n - if i > h { i - h } else { 0 };
                    accum.init(w.k, &mut w.state)?;
                    for j in wsize - nsamp..wsize {
                        accum.insert(w.work[j], &mut w.state)?;
                    }
                    accum.get(accum_params, &w.state, &mut result)?;
                    y.set(i, result[0]);
                    if let Some(z) = z {
                        z.set(i, result[1]);
                    }
                }
            } else {
                for i in idx1..=idx2 {
                    if i > h {
                        accum.delete_oldest(&mut w.state)?;
                    }
                    accum.get(accum_params, &w.state, &mut result)?;
                    y.set(i, result[0]);
                    if let Some(z) = z {
                        z.set(i, result[1]);
                    }
                }
            }
        }
        _ => {
            for i in 0..j {
                let idx = n as isize - j as isize + i as isize;
                if idx >= 0 {
                    let xn = x.get(n - 1);
                    accum.insert(xn, &mut w.state)?;
                    accum.get(accum_params, &w.state, &mut result)?;
                    y.set(idx as usize, result[0]);
                    if let Some(z) = z {
                        z.set(idx as usize, result[1]);
                    }
                }
            }
        }
    }

    Ok(())
}

pub fn movstat_apply<F: Fn(usize, &[f64], &[u8]) -> f64>(
    endtype: MovstatEndType,
    f: &MovstatFunction<F>,
    x: &GslVector,
    y: &mut GslVector,
    w: &mut MovstatWorkspace,
) -> GslResult<()> {
    struct UserFuncAccum<F> {
        func: F,
        params: Vec<u8>,
    }

    impl<F: Fn(usize, &[f64], &[u8]) -> f64> MovstatAccum for UserFuncAccum<F> {
        fn size(&self, _k: usize) -> usize {
            1
        }

        fn init(&mut self, _k: usize, _state: &mut [u8]) -> GslResult<()> {
            Ok(())
        }

        fn insert(&mut self, _value: f64, _state: &mut [u8]) -> GslResult<()> {
            Ok(())
        }

        fn delete_oldest(&mut self, _state: &mut [u8]) -> GslResult<()> {
            Ok(())
        }

        fn get(&self, _params: &[u8], state: &[u8], result: &mut [f64]) -> GslResult<()> {
            result[0] = (self.func)(state.len(), &[], &self.params);
            Ok(())
        }
    }

    let mut accum = UserFuncAccum {
        func: &f.function,
        params: f.params.clone(),
    };

    movstat_apply_accum(endtype, x, &mut accum, &[], y, None, w)
}