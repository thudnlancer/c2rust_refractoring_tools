use std::cmp::Ordering;
use std::f64;

pub type SizeT = usize;
pub type MadAccType = f64;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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

#[derive(Clone)]
pub struct RingBuf {
    array: Vec<MadAccType>,
    head: i32,
    tail: i32,
    size: i32,
}

impl RingBuf {
    pub fn new(n: SizeT) -> Self {
        RingBuf {
            array: vec![0.0; n],
            head: -1,
            tail: 0,
            size: n as i32,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.head == -1
    }

    pub fn insert(&mut self, x: MadAccType) -> Result<(), GslError> {
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

    pub fn len(&self) -> i32 {
        if self.head > self.tail {
            self.size - self.head + self.tail + 1
        } else {
            self.tail - self.head + 1
        }
    }

    pub fn get(&self, index: i32) -> MadAccType {
        let idx = (self.head + index) % self.size;
        self.array[idx as usize]
    }
}

pub struct MadAccState {
    n: SizeT,
    median_state: Vec<u8>, // Placeholder for median state
    rbuf: RingBuf,
    work: Vec<MadAccType>,
}

impl MadAccState {
    pub fn new(n: SizeT) -> Self {
        MadAccState {
            n,
            median_state: Vec::new(), // Initialize properly based on actual median state size
            rbuf: RingBuf::new(n),
            work: vec![0.0; n],
        }
    }

    pub fn insert(&mut self, x: MadAccType) -> Result<(), GslError> {
        // Insert into median state (placeholder)
        self.rbuf.insert(x)
    }

    pub fn medmad(&self, scale: f64) -> Result<(f64, f64), GslError> {
        if self.rbuf.is_empty() {
            return Err(GslError::Invalid);
        }

        let n = self.rbuf.len();
        let median = 0.0; // Calculate median properly
        
        for i in 0..n {
            let xi = self.rbuf.get(i);
            self.work[i as usize] = (xi - median).abs();
        }

        let mut sorted_work = self.work.clone();
        sorted_work.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
        let mad = median(&sorted_work);

        Ok((median, scale * mad))
    }
}

fn median(data: &[f64]) -> f64 {
    let n = data.len();
    if n % 2 == 1 {
        data[n / 2]
    } else {
        (data[n / 2 - 1] + data[n / 2]) / 2.0
    }
}

pub struct GslMovStatAccum {
    pub size: fn(SizeT) -> SizeT,
    pub init: fn(SizeT) -> MadAccState,
    pub insert: fn(&mut MadAccState, MadAccType) -> Result<(), GslError>,
    pub get: fn(&MadAccState, f64) -> Result<(f64, f64), GslError>,
}

pub fn mad_accum_type() -> GslMovStatAccum {
    GslMovStatAccum {
        size: |n| {
            // Calculate total size needed
            std::mem::size_of::<MadAccState>() + n * std::mem::size_of::<MadAccType>()
        },
        init: |n| MadAccState::new(n),
        insert: |state, x| state.insert(x),
        get: |state, scale| state.medmad(scale),
    }
}

pub static GSL_MOVSTAT_ACCUM_MAD: GslMovStatAccum = mad_accum_type();