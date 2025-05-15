use std::f64;

#[derive(Debug, Clone, Copy, PartialEq)]
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

pub type SizeT = usize;
pub type RingbufType = f64;

#[derive(Debug)]
struct Ringbuf {
    array: Vec<RingbufType>,
    head: i32,
    tail: i32,
    size: i32,
}

impl Ringbuf {
    fn new(n: SizeT) -> Self {
        Self {
            array: vec![0.0; n],
            head: -1,
            tail: 0,
            size: n as i32,
        }
    }

    fn is_empty(&self) -> bool {
        self.head == -1
    }

    fn is_full(&self) -> bool {
        (self.head == 0 && self.tail == self.size - 1) || (self.head == self.tail + 1)
    }

    fn insert(&mut self, x: RingbufType) -> Result<(), GslError> {
        if self.head == -1 {
            self.head = 0;
            self.tail = 0;
        } else if self.head == 0 {
            self.head = self.size - 1;
            if self.tail == self.head && self.size > 1 {
                self.tail -= 1;
            }
        } else {
            self.head -= 1;
            if self.tail == self.head {
                if self.tail == 0 {
                    self.tail = self.size - 1;
                } else {
                    self.tail -= 1;
                }
            }
        }
        self.array[self.head as usize] = x;
        Ok(())
    }

    fn pop_back(&mut self) -> Result<(), GslError> {
        if self.is_empty() || self.tail < 0 {
            Err(GslError::BadLen)
        } else {
            if self.head == self.tail {
                self.head = -1;
                self.tail = -1;
            } else if self.tail == 0 {
                self.tail = self.size - 1;
            } else {
                self.tail -= 1;
            }
            Ok(())
        }
    }

    fn peek_back(&self) -> Result<RingbufType, GslError> {
        if self.is_empty() || self.tail < 0 {
            Err(GslError::BadLen)
        } else {
            Ok(self.array[self.tail as usize])
        }
    }
}

#[derive(Debug)]
struct MvaccState {
    n: SizeT,
    k: SizeT,
    mean: f64,
    m2: f64,
    rbuf: Ringbuf,
}

impl MvaccState {
    fn new(n: SizeT) -> Self {
        Self {
            n,
            k: 0,
            mean: 0.0,
            m2: 0.0,
            rbuf: Ringbuf::new(n),
        }
    }

    fn insert(&mut self, x: f64) -> Result<(), GslError> {
        if self.rbuf.is_full() {
            let old = self.rbuf.peek_back()?;
            let prev_mean = self.mean;
            self.mean += (x - old) / self.n as f64;
            self.m2 += (old - prev_mean + (x - self.mean)) * (x - old);
        } else {
            let delta = x - self.mean;
            self.k += 1;
            self.mean += delta / self.k as f64;
            self.m2 += delta * (x - self.mean);
        }
        self.rbuf.insert(x)
    }

    fn delete(&mut self) -> Result<(), GslError> {
        if !self.rbuf.is_empty() {
            if self.k > 1 {
                let old = self.rbuf.peek_back()?;
                let prev_mean = self.mean;
                let delta = prev_mean - old;
                self.mean += delta / (self.k as f64 - 1.0);
                self.m2 -= delta * (self.mean - old);
            } else if self.k == 1 {
                self.mean = 0.0;
                self.m2 = 0.0;
            }
            self.rbuf.pop_back()?;
            self.k -= 1;
        }
        Ok(())
    }

    fn mean(&self) -> f64 {
        self.mean
    }

    fn variance(&self) -> f64 {
        if self.k < 2 {
            0.0
        } else {
            self.m2 / (self.k as f64 - 1.0)
        }
    }

    fn sd(&self) -> f64 {
        self.variance().sqrt()
    }
}

pub struct GslMovstatAccum {
    state: MvaccState,
}

impl GslMovstatAccum {
    pub fn new(n: SizeT) -> Self {
        Self {
            state: MvaccState::new(n),
        }
    }

    pub fn insert(&mut self, x: f64) -> Result<(), GslError> {
        self.state.insert(x)
    }

    pub fn delete(&mut self) -> Result<(), GslError> {
        self.state.delete()
    }

    pub fn mean(&self) -> f64 {
        self.state.mean()
    }

    pub fn variance(&self) -> f64 {
        self.state.variance()
    }

    pub fn sd(&self) -> f64 {
        self.state.sd()
    }
}

pub fn gsl_movstat_accum_mean(n: SizeT) -> GslMovstatAccum {
    GslMovstatAccum::new(n)
}

pub fn gsl_movstat_accum_variance(n: SizeT) -> GslMovstatAccum {
    GslMovstatAccum::new(n)
}

pub fn gsl_movstat_accum_sd(n: SizeT) -> GslMovstatAccum {
    GslMovstatAccum::new(n)
}