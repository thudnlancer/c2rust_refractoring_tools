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

pub type SizeT = usize;
pub type FuncAccType = f64;

struct RingBuf {
    array: Vec<FuncAccType>,
    head: i32,
    tail: i32,
    size: i32,
}

impl RingBuf {
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

    fn insert(&mut self, x: FuncAccType) -> Result<(), GslError> {
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

    fn copy_to(&self, dest: &mut [FuncAccType]) -> SizeT {
        if self.is_empty() || self.tail < 0 {
            0
        } else {
            let n = self.n();
            for i in 0..n {
                dest[i as usize] = self.array[((self.head + i) % self.size) as usize];
            }
            n as SizeT
        }
    }

    fn n(&self) -> i32 {
        if self.head > self.tail {
            self.size - self.head + self.tail + 1
        } else {
            self.tail - self.head + 1
        }
    }
}

struct FuncAccState {
    window: Vec<FuncAccType>,
    rbuf: RingBuf,
}

pub struct GslMovstatFunction {
    function: Box<dyn Fn(SizeT, &[FuncAccType]) -> FuncAccType>,
}

pub struct GslMovstatAccum {
    size: Box<dyn Fn(SizeT) -> SizeT>,
    init: Box<dyn Fn(SizeT) -> Result<FuncAccState, GslError>>,
    insert: Box<dyn Fn(FuncAccType, &mut FuncAccState) -> Result<(), GslError>>,
    delete_oldest: Box<dyn Fn(&mut FuncAccState) -> Result<(), GslError>>,
    get: Box<dyn Fn(&GslMovstatFunction, &FuncAccState) -> Result<FuncAccType, GslError>>,
}

fn funcacc_size(n: SizeT) -> SizeT {
    std::mem::size_of::<FuncAccState>() + n * std::mem::size_of::<FuncAccType>() + RingBuf::new(n).array.len() * std::mem::size_of::<FuncAccType>()
}

fn funcacc_init(n: SizeT) -> Result<FuncAccState, GslError> {
    Ok(FuncAccState {
        window: vec![0.0; n],
        rbuf: RingBuf::new(n),
    })
}

fn funcacc_insert(x: FuncAccType, state: &mut FuncAccState) -> Result<(), GslError> {
    state.rbuf.insert(x)
}

fn funcacc_delete(state: &mut FuncAccState) -> Result<(), GslError> {
    if !state.rbuf.is_empty() {
        state.rbuf.pop_back()?;
    }
    Ok(())
}

fn funcacc_get(f: &GslMovstatFunction, state: &FuncAccState) -> Result<FuncAccType, GslError> {
    let n = state.rbuf.copy_to(&mut state.window.clone());
    Ok((f.function)(n, &state.window[..n]))
}

pub static GSL_MOVSTAT_ACCUM_USERFUNC: GslMovstatAccum = GslMovstatAccum {
    size: Box::new(funcacc_size),
    init: Box::new(funcacc_init),
    insert: Box::new(funcacc_insert),
    delete_oldest: Box::new(funcacc_delete),
    get: Box::new(funcacc_get),
};