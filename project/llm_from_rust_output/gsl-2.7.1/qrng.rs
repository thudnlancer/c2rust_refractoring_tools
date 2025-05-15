use std::ffi::CString;
use std::mem;
use std::ptr;
use std::slice;

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
    Tolf = 29,
    Tolx = 30,
    Tolg = 31,
    Eof = 32,
}

pub type SizeT = usize;

pub struct GslQrngType {
    pub name: CString,
    pub max_dimension: u32,
    pub state_size: fn(u32) -> SizeT,
    pub init_state: fn(&mut [u8], u32) -> Result<(), GslError>,
    pub get: fn(&mut [u8], u32, &mut [f64]) -> Result<(), GslError>,
}

pub struct GslQrng {
    pub type_: &'static GslQrngType,
    pub dimension: u32,
    pub state: Vec<u8>,
}

impl GslQrng {
    pub fn alloc(type_: &'static GslQrngType, dimension: u32) -> Result<Self, GslError> {
        let state_size = (type_.state_size)(dimension);
        let mut state = vec![0; state_size];
        
        (type_.init_state)(&mut state, dimension)?;
        
        Ok(Self {
            type_,
            dimension,
            state,
        })
    }

    pub fn init(&mut self) -> Result<(), GslError> {
        (self.type_.init_state)(&mut self.state, self.dimension)
    }

    pub fn memcpy(&mut self, src: &Self) -> Result<(), GslError> {
        if self.type_ != src.type_ {
            return Err(GslError::Inval);
        }
        
        self.dimension = src.dimension;
        self.state.copy_from_slice(&src.state);
        
        Ok(())
    }

    pub fn clone(&self) -> Result<Self, GslError> {
        let mut new_state = self.state.clone();
        (self.type_.init_state)(&mut new_state, self.dimension)?;
        
        Ok(Self {
            type_: self.type_,
            dimension: self.dimension,
            state: new_state,
        })
    }

    pub fn name(&self) -> &str {
        self.type_.name.to_str().unwrap()
    }

    pub fn size(&self) -> SizeT {
        self.state.len()
    }

    pub fn state(&mut self) -> &mut [u8] {
        &mut self.state
    }
}

impl Drop for GslQrng {
    fn drop(&mut self) {
        // Vec will be automatically deallocated
    }
}