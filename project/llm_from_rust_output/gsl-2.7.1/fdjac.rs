use libc::{c_double, c_int, c_ulong, c_void};
use std::ffi::CString;
use std::ptr;

pub type size_t = c_ulong;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
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

#[derive(Clone)]
pub struct GslBlock {
    size: size_t,
    data: Vec<c_double>,
}

#[derive(Clone)]
pub struct GslVector {
    size: size_t,
    stride: size_t,
    data: Vec<c_double>,
    block: Option<GslBlock>,
    owner: bool,
}

#[derive(Clone)]
pub struct GslMatrix {
    size1: size_t,
    size2: size_t,
    tda: size_t,
    data: Vec<c_double>,
    block: Option<GslBlock>,
    owner: bool,
}

pub struct GslMultirootFunction {
    f: Box<dyn Fn(&GslVector, &mut c_void, &mut GslVector) -> Result<(), GslError>>,
    n: size_t,
    params: *mut c_void,
}

impl GslVector {
    pub fn new(n: size_t) -> Option<Self> {
        Some(Self {
            size: n,
            stride: 1,
            data: vec![0.0; n],
            block: None,
            owner: true,
        })
    }

    pub fn get(&self, i: size_t) -> Option<c_double> {
        if i >= self.size {
            None
        } else {
            Some(self.data[(i * self.stride) as usize])
        }
    }

    pub fn set(&mut self, i: size_t, x: c_double) -> Result<(), GslError> {
        if i >= self.size {
            Err(GslError::Badlen)
        } else {
            self.data[(i * self.stride) as usize] = x;
            Ok(())
        }
    }

    pub fn is_null(&self) -> bool {
        self.data.iter().all(|&x| x == 0.0)
    }

    pub fn copy_from(&mut self, src: &Self) -> Result<(), GslError> {
        if self.size != src.size {
            Err(GslError::Badlen)
        } else {
            self.data.copy_from_slice(&src.data);
            Ok(())
        }
    }
}

impl GslMatrix {
    pub fn new(size1: size_t, size2: size_t) -> Option<Self> {
        Some(Self {
            size1,
            size2,
            tda: size2,
            data: vec![0.0; (size1 * size2) as usize],
            block: None,
            owner: true,
        })
    }

    pub fn set(&mut self, i: size_t, j: size_t, x: c_double) -> Result<(), GslError> {
        if i >= self.size1 || j >= self.size2 {
            Err(GslError::Badlen)
        } else {
            self.data[(i * self.tda + j) as usize] = x;
            Ok(())
        }
    }

    pub fn column(&mut self, j: size_t) -> Option<GslVector> {
        if j >= self.size2 {
            None
        } else {
            let stride = self.tda;
            let size = self.size1;
            let start = j as usize;
            let end = start + (size * stride) as usize;
            Some(GslVector {
                size,
                stride,
                data: self.data[start..end].to_vec(),
                block: None,
                owner: false,
            })
        }
    }
}

pub fn fd_jacobian(
    f: &GslMultirootFunction,
    x: &GslVector,
    fx: &GslVector,
    epsrel: c_double,
    jacobian: &mut GslMatrix,
) -> Result<(), GslError> {
    let n = x.size;
    let m = fx.size;

    if m != jacobian.size1 || n != jacobian.size2 {
        return Err(GslError::Badlen);
    }

    let mut x1 = GslVector::new(n).ok_or(GslError::Nomem)?;
    let mut f1 = GslVector::new(m).ok_or(GslError::Nomem)?;

    x1.copy_from(x)?;

    for j in 0..n {
        let xj = x.get(j).ok_or(GslError::Badlen)?;
        let mut dx = epsrel * xj.abs();
        if dx == 0.0 {
            dx = epsrel;
        }

        x1.set(j, xj + dx)?;
        (f.f)(&x1, f.params, &mut f1)?;

        x1.set(j, xj)?;

        for i in 0..m {
            let g1 = f1.get(i).ok_or(GslError::Badlen)?;
            let g0 = fx.get(i).ok_or(GslError::Badlen)?;
            jacobian.set(i, j, (g1 - g0) / dx)?;
        }

        if let Some(col) = jacobian.column(j) {
            if col.is_null() {
                return Err(GslError::Sing);
            }
        }
    }

    Ok(())
}