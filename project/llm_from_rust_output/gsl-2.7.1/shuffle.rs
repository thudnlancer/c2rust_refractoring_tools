use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GslError {
    Dom,
    Range,
    Failure,
    Continue,
    Eof,
    // ... other error variants
    Einval,
    // ... remaining error variants
}

impl fmt::Display for GslError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GSL error: {:?}", self)
    }
}

impl Error for GslError {}

pub type GslResult<T> = Result<T, GslError>;

pub struct GslRngType {
    name: &'static str,
    max: u64,
    min: u64,
    size: usize,
    get_double: fn(&mut ()) -> f64,
    get: fn(&mut ()) -> u64,
}

pub struct GslRng {
    type_: &'static GslRngType,
    state: Box<()>,
}

impl GslRng {
    pub fn uniform(&mut self) -> f64 {
        (self.type_.get_double)(&mut *self.state)
    }

    pub fn uniform_int(&mut self, n: u64) -> GslResult<u64> {
        let offset = self.type_.min;
        let range = self.type_.max - offset;

        if n > range || n == 0 {
            return Err(GslError::Einval);
        }

        let scale = range / n;
        loop {
            let k = ((self.type_.get)(&mut *self.state) - offset) / scale;
            if k < n {
                return Ok(k);
            }
        }
    }
}

fn swap<T>(v: &mut [T], i: usize, j: usize) {
    if i != j {
        v.swap(i, j);
    }
}

fn copy<T: Clone>(dest: &mut [T], dest_idx: usize, src: &[T], src_idx: usize) {
    dest[dest_idx] = src[src_idx].clone();
}

pub fn shuffle<T>(rng: &mut GslRng, v: &mut [T]) -> GslResult<()> {
    for i in (1..v.len()).rev() {
        let j = rng.uniform_int((i + 1) as u64)? as usize;
        swap(v, i, j);
    }
    Ok(())
}

pub fn choose<T: Clone>(
    rng: &mut GslRng,
    dest: &mut [T],
    src: &[T],
) -> GslResult<()> {
    if dest.len() > src.len() {
        return Err(GslError::Einval);
    }

    let mut j = 0;
    for i in 0..src.len() {
        if (src.len() - i) as f64 * rng.uniform() < (dest.len() - j) as f64 {
            copy(dest, j, src, i);
            j += 1;
            if j == dest.len() {
                break;
            }
        }
    }
    Ok(())
}

pub fn sample<T: Clone>(rng: &mut GslRng, dest: &mut [T], src: &[T]) {
    for i in 0..dest.len() {
        let j = rng.uniform_int(src.len() as u64).unwrap() as usize;
        copy(dest, i, src, j);
    }
}