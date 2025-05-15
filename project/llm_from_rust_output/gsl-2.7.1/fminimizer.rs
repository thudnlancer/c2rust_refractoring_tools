use std::ffi::CString;
use std::os::raw::{c_char, c_double, c_int, c_void};
use std::ptr;

pub type size_t = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum GslError {
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
    Tolf = 29,
    Tolx = 30,
    Tolg = 31,
    Eof = 32,
    Continue = -2,
    Failure = -1,
    Success = 0,
}

#[derive(Debug)]
pub struct GslBlock {
    size: size_t,
    data: Vec<f64>,
}

#[derive(Debug)]
pub struct GslVector {
    size: size_t,
    stride: size_t,
    data: Vec<f64>,
    block: Option<Box<GslBlock>>,
    owner: bool,
}

#[derive(Debug)]
pub struct GslMultiminFunction {
    f: Box<dyn Fn(&GslVector) -> f64>,
    n: size_t,
}

#[derive(Debug)]
pub struct GslMultiminFminimizerType {
    name: String,
    size: size_t,
    alloc: Box<dyn Fn(size_t) -> Result<Box<dyn std::any::Any>, GslError>>,
    set: Box<dyn Fn(&mut dyn std::any::Any, &GslMultiminFunction, &GslVector, &GslVector) -> Result<(), GslError>>,
    iterate: Box<dyn Fn(&mut dyn std::any::Any, &GslMultiminFunction, &mut GslVector) -> Result<(f64, f64), GslError>>,
}

#[derive(Debug)]
pub struct GslMultiminFminimizer {
    type_: Box<GslMultiminFminimizerType>,
    f: Option<GslMultiminFunction>,
    fval: f64,
    x: GslVector,
    size: f64,
    state: Box<dyn std::any::Any>,
}

impl GslMultiminFminimizer {
    pub fn new(type_: Box<GslMultiminFminimizerType>, n: size_t) -> Result<Self, GslError> {
        let x = GslVector::new(n)?;
        let state = (type_.alloc)(n)?;

        Ok(Self {
            type_,
            f: None,
            fval: 0.0,
            x,
            size: 0.0,
            state,
        })
    }

    pub fn set(
        &mut self,
        f: GslMultiminFunction,
        x: &GslVector,
        step_size: &GslVector,
    ) -> Result<(), GslError> {
        if self.x.size != f.n {
            return Err(GslError::Badlen);
        }
        if x.size != f.n || step_size.size != f.n {
            return Err(GslError::Badlen);
        }

        self.f = Some(f);
        self.x.copy_from(x)?;
        (self.type_.set)(&mut *self.state, self.f.as_ref().unwrap(), &self.x, step_size)
    }

    pub fn iterate(&mut self) -> Result<(), GslError> {
        let (size, fval) = (self.type_.iterate)(
            &mut *self.state,
            self.f.as_ref().unwrap(),
            &mut self.x,
        )?;
        self.size = size;
        self.fval = fval;
        Ok(())
    }

    pub fn name(&self) -> &str {
        &self.type_.name
    }

    pub fn x(&self) -> &GslVector {
        &self.x
    }

    pub fn minimum(&self) -> f64 {
        self.fval
    }

    pub fn size(&self) -> f64 {
        self.size
    }
}

impl GslVector {
    pub fn new(n: size_t) -> Result<Self, GslError> {
        Ok(Self {
            size: n,
            stride: 1,
            data: vec![0.0; n],
            block: None,
            owner: true,
        })
    }

    pub fn copy_from(&mut self, src: &Self) -> Result<(), GslError> {
        if self.size != src.size {
            return Err(GslError::Badlen);
        }
        self.data.copy_from_slice(&src.data);
        Ok(())
    }
}

impl Drop for GslMultiminFminimizer {
    fn drop(&mut self) {
        // Resources are automatically cleaned up by Rust's ownership system
    }
}