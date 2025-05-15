use std::cmp::Ordering;
use std::mem;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum GslError {
    Success = 0,
    Failure = -1,
    Continue = -2,
    EDOM = 1,
    ERANGE = 2,
    EFAULT = 3,
    EINVAL = 4,
    EFAILED = 5,
    EFACTOR = 6,
    ESANITY = 7,
    ENOMEM = 8,
    EBADFUNC = 9,
    ERUNAWAY = 10,
    EMAXITER = 11,
    EZERODIV = 12,
    EBADTOL = 13,
    ETOL = 14,
    EUNDRFLW = 15,
    EOVRFLW = 16,
    ELOSS = 17,
    EROUND = 18,
    EBADLEN = 19,
    ENOTSQR = 20,
    ESING = 21,
    EDIVERGE = 22,
    EUNSUP = 23,
    EUNIMPL = 24,
    ECACHE = 25,
    ETABLE = 26,
    ENOPROG = 27,
    ENOPROGJ = 28,
    ETOLF = 29,
    ETOLX = 30,
    ETOLG = 31,
    EOF = 32,
}

pub type SizeT = usize;
pub type QqrAccType = f64;

struct RingBuf {
    array: Vec<QqrAccType>,
    head: i32,
    tail: i32,
    size: i32,
}

impl RingBuf {
    fn new(n: SizeT) -> Self {
        RingBuf {
            array: vec![0.0; n],
            head: -1,
            tail: 0,
            size: n as i32,
        }
    }

    fn is_empty(&self) -> bool {
        self.head == -1
    }

    fn insert(&mut self, x: QqrAccType) -> Result<(), GslError> {
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
            Err(GslError::EBADLEN)
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

    fn copy_to(&self, dest: &mut [f64]) -> SizeT {
        if self.is_empty() || self.tail < 0 {
            0
        } else {
            let n = self.n() as usize;
            for i in 0..n {
                dest[i] = self.array[((self.head + i as i32) % self.size) as usize];
            }
            n
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

struct QqrAccState {
    window: Vec<QqrAccType>,
    rbuf: RingBuf,
}

impl QqrAccState {
    fn new(n: SizeT) -> Self {
        QqrAccState {
            window: vec![0.0; n],
            rbuf: RingBuf::new(n),
        }
    }

    fn insert(&mut self, x: QqrAccType) -> Result<(), GslError> {
        self.rbuf.insert(x)
    }

    fn delete(&mut self) -> Result<(), GslError> {
        if !self.rbuf.is_empty() {
            self.rbuf.pop_back()
        } else {
            Ok(())
        }
    }

    fn get(&mut self, q: f64) -> Result<f64, GslError> {
        let n = self.rbuf.copy_to(&mut self.window);
        if n == 0 {
            return Ok(0.0);
        }

        let window = &mut self.window[..n];
        window.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));

        let quant1 = quantile_from_sorted_data(window, q);
        let quant2 = quantile_from_sorted_data(window, 1.0 - q);
        Ok(quant2 - quant1)
    }
}

fn quantile_from_sorted_data(sorted_data: &[f64], f: f64) -> f64 {
    let n = sorted_data.len();
    if n == 0 {
        return 0.0;
    }

    let index = f * (n - 1) as f64;
    let i = index.floor() as usize;
    let delta = index - i as f64;

    if i == n - 1 {
        sorted_data[i]
    } else {
        sorted_data[i] + delta * (sorted_data[i + 1] - sorted_data[i])
    }
}

pub struct GslMovstatAccum {
    size_fn: fn(SizeT) -> SizeT,
    init_fn: fn(SizeT) -> Result<Box<dyn MovStatAccum>, GslError>,
}

pub trait MovStatAccum {
    fn insert(&mut self, x: f64) -> Result<(), GslError>;
    fn delete(&mut self) -> Result<(), GslError>;
    fn get(&mut self, params: &[f64]) -> Result<f64, GslError>;
}

impl GslMovstatAccum {
    pub const QQR: Self = GslMovstatAccum {
        size_fn: qqracc_size,
        init_fn: qqracc_init,
    };
}

fn qqracc_size(n: SizeT) -> SizeT {
    mem::size_of::<QqrAccState>() + n * mem::size_of::<QqrAccType>() + mem::size_of::<RingBuf>()
}

fn qqracc_init(n: SizeT) -> Result<Box<dyn MovStatAccum>, GslError> {
    Ok(Box::new(QqrAccState::new(n)))
}

impl MovStatAccum for QqrAccState {
    fn insert(&mut self, x: f64) -> Result<(), GslError> {
        self.insert(x)
    }

    fn delete(&mut self) -> Result<(), GslError> {
        self.delete()
    }

    fn get(&mut self, params: &[f64]) -> Result<f64, GslError> {
        if params.is_empty() {
            return Err(GslError::EINVAL);
        }
        self.get(params[0])
    }
}