use std::f64;

pub type SizeT = usize;

#[derive(Debug, Clone, Copy)]
pub enum GslError {
    Success = 0,
    Failure = -1,
    Continue = -2,
    Dom = 1,
    Range = 2,
    Fault = 3,
    Inval = 4,
    Failed = 5,
    Factor = 6,
    Sanity = 7,
    Nomem = 8,
    Badfunc = 9,
    Runaway = 10,
    Maxiter = 11,
    Zerodiv = 12,
    Badtol = 13,
    Tol = 14,
    Undrflw = 15,
    Ovrflw = 16,
    Loss = 17,
    Round = 18,
    Badlen = 19,
    Notsqr = 20,
    Sing = 21,
    Diverge = 22,
    Unsup = 23,
    Unimpl = 24,
    Cache = 25,
    Table = 26,
    Noprog = 27,
    Noprogj = 28,
    Etolf = 29,
    Etolx = 30,
    Etolg = 31,
    Eof = 32,
}

#[derive(Debug)]
pub struct GslBlock {
    pub size: SizeT,
    pub data: Vec<f64>,
}

#[derive(Debug)]
pub struct GslVector {
    pub size: SizeT,
    pub stride: SizeT,
    pub data: Vec<f64>,
    pub block: GslBlock,
    pub owner: i32,
}

impl GslVector {
    pub fn new(size: SizeT) -> Self {
        let data = vec![0.0; size];
        let block = GslBlock {
            size,
            data: data.clone(),
        };
        GslVector {
            size,
            stride: 1,
            data,
            block,
            owner: 1,
        }
    }

    pub fn get(&self, i: SizeT) -> f64 {
        self.data[i * self.stride]
    }

    pub fn set(&mut self, i: SizeT, x: f64) {
        self.data[i * self.stride] = x;
    }

    pub fn copy_from(&mut self, src: &GslVector) -> Result<(), GslError> {
        if self.size != src.size {
            return Err(GslError::Badlen);
        }
        self.data.copy_from_slice(&src.data);
        Ok(())
    }
}

pub struct GslMultiminFunction {
    pub f: Box<dyn Fn(&GslVector) -> f64>,
    pub n: SizeT,
}

pub fn gsl_multimin_diff(
    f: &GslMultiminFunction,
    x: &GslVector,
    g: &mut GslVector,
) -> Result<(), GslError> {
    let n = f.n;
    let h = 1.4901161193847656e-08f64;
    let mut x1 = GslVector::new(n);

    x1.copy_from(x)?;

    for i in 0..n {
        let xi = x.get(i);
        let dx = xi.abs() * h;
        let dx = if dx == 0.0 { h } else { dx };

        x1.set(i, xi + dx);
        let fh = (f.f)(&x1);

        x1.set(i, xi - dx);
        let fl = (f.f)(&x1);

        x1.set(i, xi);
        g.set(i, (fh - fl) / (2.0 * dx));
    }

    Ok(())
}