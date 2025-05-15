use std::error::Error;
use std::fmt;

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
    NoMemory = 8,
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

pub type SizeT = usize;

#[derive(Debug)]
struct RingBuf {
    array: Vec<f64>,
    head: i32,
    tail: i32,
}

impl RingBuf {
    fn new(n: SizeT) -> Self {
        RingBuf {
            array: vec![0.0; n],
            head: -1,
            tail: 0,
        }
    }

    fn is_empty(&self) -> bool {
        self.head == -1
    }

    fn is_full(&self) -> bool {
        (self.head == 0 && self.tail == (self.array.len() as i32) - 1) ||
            (self.head == self.tail + 1)
    }

    fn insert(&mut self, x: f64) -> Result<(), GslError> {
        if self.head == -1 {
            self.head = 0;
            self.tail = 0;
        } else if self.head == 0 {
            self.head = (self.array.len() as i32) - 1;
            if self.tail == self.head && self.array.len() > 1 {
                self.tail -= 1;
            }
        } else {
            self.head -= 1;
            if self.tail == self.head {
                if self.tail == 0 {
                    self.tail = (self.array.len() as i32) - 1;
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
                self.tail = (self.array.len() as i32) - 1;
            } else {
                self.tail -= 1;
            }
            Ok(())
        }
    }

    fn peek_back(&self) -> Result<f64, GslError> {
        if self.is_empty() || self.tail < 0 {
            Err(GslError::BadLen)
        } else {
            Ok(self.array[self.tail as usize])
        }
    }
}

#[derive(Debug)]
struct SumAccState {
    sum: f64,
    rbuf: RingBuf,
}

pub struct GslMovstatAccum {
    size: fn(SizeT) -> SizeT,
    init: fn(SizeT) -> Result<SumAccState, GslError>,
    insert: fn(f64, &mut SumAccState) -> Result<(), GslError>,
    delete_oldest: fn(&mut SumAccState) -> Result<(), GslError>,
    get: fn(&SumAccState) -> Result<f64, GslError>,
}

fn sumacc_size(n: SizeT) -> SizeT {
    std::mem::size_of::<SumAccState>() + std::mem::size_of::<RingBuf>() + n * std::mem::size_of::<f64>()
}

fn sumacc_init(n: SizeT) -> Result<SumAccState, GslError> {
    Ok(SumAccState {
        sum: 0.0,
        rbuf: RingBuf::new(n),
    })
}

fn sumacc_insert(x: f64, state: &mut SumAccState) -> Result<(), GslError> {
    if state.rbuf.is_full() {
        state.sum -= state.rbuf.peek_back()?;
    }
    state.sum += x;
    state.rbuf.insert(x)
}

fn sumacc_delete(state: &mut SumAccState) -> Result<(), GslError> {
    if !state.rbuf.is_empty() {
        state.sum -= state.rbuf.peek_back()?;
        state.rbuf.pop_back()?;
    }
    Ok(())
}

fn sumacc_get(state: &SumAccState) -> Result<f64, GslError> {
    Ok(state.sum)
}

pub const GSL_MOVSTAT_ACCUM_SUM: GslMovstatAccum = GslMovstatAccum {
    size: sumacc_size,
    init: sumacc_init,
    insert: sumacc_insert,
    delete_oldest: sumacc_delete,
    get: sumacc_get,
};