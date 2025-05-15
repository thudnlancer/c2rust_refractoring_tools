use std::error::Error;
use std::fmt;
use std::ptr;

#[derive(Debug)]
pub enum InterpError {
    InsufficientPoints,
    AllocationFailed,
    InvalidData,
    OutOfRange,
    DomainError,
    InterpolationError,
}

impl fmt::Display for InterpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            InterpError::InsufficientPoints => write!(f, "insufficient number of points for interpolation type"),
            InterpError::AllocationFailed => write!(f, "failed to allocate space"),
            InterpError::InvalidData => write!(f, "data must be strictly increasing"),
            InterpError::OutOfRange => write!(f, "index out of range"),
            InterpError::DomainError => write!(f, "value out of range"),
            InterpError::InterpolationError => write!(f, "interpolation error"),
        }
    }
}

impl Error for InterpError {}

pub type Result<T> = std::result::Result<T, InterpError>;

pub trait Interp2DType {
    fn min_size(&self) -> usize;
    fn name(&self) -> &str;
    fn alloc(&self, xsize: usize, ysize: usize) -> Option<Box<dyn std::any::Any>>;
    fn free(&self, state: &mut Box<dyn std::any::Any>);
    fn init(&self, state: &mut Box<dyn std::any::Any>, xarr: &[f64], yarr: &[f64], zarr: &[f64], xsize: usize, ysize: usize) -> Result<()>;
    fn eval(&self, state: &Box<dyn std::any::Any>, xarr: &[f64], yarr: &[f64], zarr: &[f64], x: f64, y: f64, xa: &mut usize, ya: &mut usize) -> Result<f64>;
    fn eval_deriv_x(&self, state: &Box<dyn std::any::Any>, xarr: &[f64], yarr: &[f64], zarr: &[f64], x: f64, y: f64, xa: &mut usize, ya: &mut usize) -> Result<f64>;
    fn eval_deriv_y(&self, state: &Box<dyn std::any::Any>, xarr: &[f64], yarr: &[f64], zarr: &[f64], x: f64, y: f64, xa: &mut usize, ya: &mut usize) -> Result<f64>;
    fn eval_deriv_xx(&self, state: &Box<dyn std::any::Any>, xarr: &[f64], yarr: &[f64], zarr: &[f64], x: f64, y: f64, xa: &mut usize, ya: &mut usize) -> Result<f64>;
    fn eval_deriv_yy(&self, state: &Box<dyn std::any::Any>, xarr: &[f64], yarr: &[f64], zarr: &[f64], x: f64, y: f64, xa: &mut usize, ya: &mut usize) -> Result<f64>;
    fn eval_deriv_xy(&self, state: &Box<dyn std::any::Any>, xarr: &[f64], yarr: &[f64], zarr: &[f64], x: f64, y: f64, xa: &mut usize, ya: &mut usize) -> Result<f64>;
}

pub struct Interp2D {
    interp_type: Box<dyn Interp2DType>,
    xsize: usize,
    ysize: usize,
    xmin: f64,
    xmax: f64,
    ymin: f64,
    ymax: f64,
    state: Option<Box<dyn std::any::Any>>,
}

impl Interp2D {
    pub fn new(interp_type: Box<dyn Interp2DType>, xsize: usize, ysize: usize) -> Result<Self> {
        if xsize < interp_type.min_size() || ysize < interp_type.min_size() {
            return Err(InterpError::InsufficientPoints);
        }

        let state = if interp_type.alloc(xsize, ysize).is_some() {
            interp_type.alloc(xsize, ysize)
        } else {
            return Err(InterpError::AllocationFailed);
        };

        Ok(Interp2D {
            interp_type,
            xsize,
            ysize,
            xmin: 0.0,
            xmax: 0.0,
            ymin: 0.0,
            ymax: 0.0,
            state,
        })
    }

    pub fn init(&mut self, xarr: &[f64], yarr: &[f64], zarr: &[f64]) -> Result<()> {
        if xarr.len() != self.xsize || yarr.len() != self.ysize {
            return Err(InterpError::InvalidData);
        }

        for i in 1..self.xsize {
            if xarr[i-1] >= xarr[i] {
                return Err(InterpError::InvalidData);
            }
        }

        for i in 1..self.ysize {
            if yarr[i-1] >= yarr[i] {
                return Err(InterpError::InvalidData);
            }
        }

        self.xmin = xarr[0];
        self.xmax = xarr[self.xsize - 1];
        self.ymin = yarr[0];
        self.ymax = yarr[self.ysize - 1];

        self.interp_type.init(self.state.as_mut().unwrap(), xarr, yarr, zarr, self.xsize, self.ysize)
    }

    pub fn eval(&self, xarr: &[f64], yarr: &[f64], zarr: &[f64], x: f64, y: f64, xa: &mut usize, ya: &mut usize) -> Result<f64> {
        if x < self.xmin || x > self.xmax {
            return Err(InterpError::DomainError);
        }
        if y < self.ymin || y > self.ymax {
            return Err(InterpError::DomainError);
        }

        self.interp_type.eval(self.state.as_ref().unwrap(), xarr, yarr, zarr, x, y, xa, ya)
    }

    pub fn eval_extrap(&self, xarr: &[f64], yarr: &[f64], zarr: &[f64], x: f64, y: f64, xa: &mut usize, ya: &mut usize) -> Result<f64> {
        self.interp_type.eval(self.state.as_ref().unwrap(), xarr, yarr, zarr, x, y, xa, ya)
    }

    pub fn eval_deriv_x(&self, xarr: &[f64], yarr: &[f64], zarr: &[f64], x: f64, y: f64, xa: &mut usize, ya: &mut usize) -> Result<f64> {
        if x < self.xmin || x > self.xmax {
            return Err(InterpError::DomainError);
        }
        if y < self.ymin || y > self.ymax {
            return Err(InterpError::DomainError);
        }

        self.interp_type.eval_deriv_x(self.state.as_ref().unwrap(), xarr, yarr, zarr, x, y, xa, ya)
    }

    pub fn eval_deriv_y(&self, xarr: &[f64], yarr: &[f64], zarr: &[f64], x: f64, y: f64, xa: &mut usize, ya: &mut usize) -> Result<f64> {
        if x < self.xmin || x > self.xmax {
            return Err(InterpError::DomainError);
        }
        if y < self.ymin || y > self.ymax {
            return Err(InterpError::DomainError);
        }

        self.interp_type.eval_deriv_y(self.state.as_ref().unwrap(), xarr, yarr, zarr, x, y, xa, ya)
    }

    pub fn eval_deriv_xx(&self, xarr: &[f64], yarr: &[f64], zarr: &[f64], x: f64, y: f64, xa: &mut usize, ya: &mut usize) -> Result<f64> {
        if x < self.xmin || x > self.xmax {
            return Err(InterpError::DomainError);
        }
        if y < self.ymin || y > self.ymax {
            return Err(InterpError::DomainError);
        }

        self.interp_type.eval_deriv_xx(self.state.as_ref().unwrap(), xarr, yarr, zarr, x, y, xa, ya)
    }

    pub fn eval_deriv_yy(&self, xarr: &[f64], yarr: &[f64], zarr: &[f64], x: f64, y: f64, xa: &mut usize, ya: &mut usize) -> Result<f64> {
        if x < self.xmin || x > self.xmax {
            return Err(InterpError::DomainError);
        }
        if y < self.ymin || y > self.ymax {
            return Err(InterpError::DomainError);
        }

        self.interp_type.eval_deriv_yy(self.state.as_ref().unwrap(), xarr, yarr, zarr, x, y, xa, ya)
    }

    pub fn eval_deriv_xy(&self, xarr: &[f64], yarr: &[f64], zarr: &[f64], x: f64, y: f64, xa: &mut usize, ya: &mut usize) -> Result<f64> {
        if x < self.xmin || x > self.xmax {
            return Err(InterpError::DomainError);
        }
        if y < self.ymin || y > self.ymax {
            return Err(InterpError::DomainError);
        }

        self.interp_type.eval_deriv_xy(self.state.as_ref().unwrap(), xarr, yarr, zarr, x, y, xa, ya)
    }

    pub fn min_size(&self) -> usize {
        self.interp_type.min_size()
    }

    pub fn name(&self) -> &str {
        self.interp_type.name()
    }

    pub fn idx(&self, i: usize, j: usize) -> Result<usize> {
        if i >= self.xsize {
            return Err(InterpError::OutOfRange);
        }
        if j >= self.ysize {
            return Err(InterpError::OutOfRange);
        }

        Ok(j * self.xsize + i)
    }

    pub fn set(&self, zarr: &mut [f64], i: usize, j: usize, z: f64) -> Result<()> {
        if i >= self.xsize {
            return Err(InterpError::OutOfRange);
        }
        if j >= self.ysize {
            return Err(InterpError::OutOfRange);
        }

        let idx = j * self.xsize + i;
        zarr[idx] = z;
        Ok(())
    }

    pub fn get(&self, zarr: &[f64], i: usize, j: usize) -> Result<f64> {
        if i >= self.xsize {
            return Err(InterpError::OutOfRange);
        }
        if j >= self.ysize {
            return Err(InterpError::OutOfRange);
        }

        let idx = j * self.xsize + i;
        Ok(zarr[idx])
    }
}

impl Drop for Interp2D {
    fn drop(&mut self) {
        if let Some(state) = &mut self.state {
            self.interp_type.free(state);
        }
    }
}