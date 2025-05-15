use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GslError {
    Domain,
    Range,
    Fault,
    Invalid,
    Failed,
    Factor,
    Sanity,
    NoMem,
    BadFunc,
    Runaway,
    MaxIter,
    ZeroDiv,
    BadTol,
    Tol,
    Underflow,
    Overflow,
    Loss,
    Round,
    BadLen,
    NotSquare,
    Singular,
    Diverge,
    Unsupported,
    Unimplemented,
    Cache,
    Table,
    NoProg,
    NoProgJ,
    TolF,
    TolX,
    TolG,
    Eof,
    Continue,
    Failure,
    Success,
}

impl fmt::Display for GslError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for GslError {}

type SizeT = usize;
type MmaccType = f64;
type RingbufType = MmaccType;
type DequeType = i32;

#[derive(Debug)]
struct Deque {
    head: i32,
    tail: i32,
    size: i32,
    array: Vec<DequeType>,
}

impl Deque {
    fn new(n: SizeT) -> Self {
        Self {
            head: -1,
            tail: 0,
            size: n as i32,
            array: vec![0; n],
        }
    }

    fn is_empty(&self) -> bool {
        self.head == -1
    }

    fn is_full(&self) -> bool {
        (self.head == 0 && self.tail == self.size - 1) || (self.head == self.tail + 1)
    }

    fn push_back(&mut self, x: DequeType) -> Result<(), GslError> {
        if self.is_full() {
            return Err(GslError::Overflow);
        }

        if self.head == -1 {
            self.head = 0;
            self.tail = 0;
        } else if self.tail == self.size - 1 {
            self.tail = 0;
        } else {
            self.tail += 1;
        }

        self.array[self.tail as usize] = x;
        Ok(())
    }

    fn pop_front(&mut self) -> Result<(), GslError> {
        if self.is_empty() {
            return Err(GslError::Overflow);
        }

        if self.head == self.tail {
            self.head = -1;
            self.tail = -1;
        } else if self.head == self.size - 1 {
            self.head = 0;
        } else {
            self.head += 1;
        }

        Ok(())
    }

    fn pop_back(&mut self) -> Result<(), GslError> {
        if self.is_empty() {
            return Err(GslError::Overflow);
        }

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

    fn peek_front(&self) -> Result<DequeType, GslError> {
        if self.is_empty() {
            Err(GslError::BadLen)
        } else {
            Ok(self.array[self.head as usize])
        }
    }

    fn peek_back(&self) -> Result<DequeType, GslError> {
        if self.is_empty() || self.tail < 0 {
            Err(GslError::BadLen)
        } else {
            Ok(self.array[self.tail as usize])
        }
    }
}

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
            return Err(GslError::BadLen);
        }

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

#[derive(Debug)]
struct MmaccState {
    n: SizeT,
    k: SizeT,
    xprev: MmaccType,
    rbuf: Ringbuf,
    minque: Deque,
    maxque: Deque,
}

impl MmaccState {
    fn new(n: SizeT) -> Self {
        Self {
            n,
            k: 0,
            xprev: 0.0,
            rbuf: Ringbuf::new(n),
            minque: Deque::new(n + 1),
            maxque: Deque::new(n + 1),
        }
    }

    fn insert(&mut self, x: MmaccType) -> Result<(), GslError> {
        if self.k == 0 {
            self.rbuf.insert(x)?;
            let head = self.rbuf.head;
            self.maxque.push_back(head)?;
            self.minque.push_back(head)?;
        } else {
            if x > self.xprev {
                self.maxque.pop_back()?;
                while !self.maxque.is_empty() {
                    if x <= self.rbuf.array[self.maxque.peek_back()? as usize] {
                        break;
                    }
                    self.maxque.pop_back()?;
                }
            } else {
                self.minque.pop_back()?;
                while !self.minque.is_empty() {
                    if x >= self.rbuf.array[self.minque.peek_back()? as usize] {
                        break;
                    }
                    self.minque.pop_back()?;
                }
            }

            let tail = self.rbuf.tail;
            self.rbuf.insert(x)?;
            let head = self.rbuf.head;
            self.maxque.push_back(head)?;
            self.minque.push_back(head)?;

            if self.k == self.n {
                if self.maxque.head != self.maxque.tail && tail == self.maxque.peek_front()? {
                    self.maxque.pop_front()?;
                } else if self.minque.head != self.minque.tail && tail == self.minque.peek_front()? {
                    self.minque.pop_front()?;
                }
            }
        }

        if self.k < self.n {
            self.k += 1;
        }

        self.xprev = x;
        Ok(())
    }

    fn delete(&mut self) -> Result<(), GslError> {
        if self.k > 0 {
            if self.rbuf.tail == self.maxque.peek_front()? {
                self.maxque.pop_front()?;
            } else if self.rbuf.tail == self.minque.peek_front()? {
                self.minque.pop_front()?;
            }

            self.rbuf.pop_back()?;
            self.k -= 1;
        }

        Ok(())
    }

    fn min(&self) -> Result<MmaccType, GslError> {
        if self.k == 0 {
            Err(GslError::Invalid)
        } else {
            Ok(self.rbuf.array[self.minque.peek_front()? as usize])
        }
    }

    fn max(&self) -> Result<MmaccType, GslError> {
        if self.k == 0 {
            Err(GslError::Invalid)
        } else {
            Ok(self.rbuf.array[self.maxque.peek_front()? as usize])
        }
    }

    fn minmax(&self) -> Result<(MmaccType, MmaccType), GslError> {
        if self.k == 0 {
            Err(GslError::Invalid)
        } else {
            Ok((
                self.rbuf.array[self.minque.peek_front()? as usize],
                self.rbuf.array[self.maxque.peek_front()? as usize],
            ))
        }
    }
}

pub struct GslMovstatAccum {
    state: MmaccState,
}

impl GslMovstatAccum {
    pub fn new(n: SizeT) -> Self {
        Self {
            state: MmaccState::new(n),
        }
    }

    pub fn insert(&mut self, x: MmaccType) -> Result<(), GslError> {
        self.state.insert(x)
    }

    pub fn delete(&mut self) -> Result<(), GslError> {
        self.state.delete()
    }

    pub fn min(&self) -> Result<MmaccType, GslError> {
        self.state.min()
    }

    pub fn max(&self) -> Result<MmaccType, GslError> {
        self.state.max()
    }

    pub fn minmax(&self) -> Result<(MmaccType, MmaccType), GslError> {
        self.state.minmax()
    }
}

pub fn gsl_movstat_accum_min(n: SizeT) -> GslMovstatAccum {
    GslMovstatAccum::new(n)
}

pub fn gsl_movstat_accum_max(n: SizeT) -> GslMovstatAccum {
    GslMovstatAccum::new(n)
}

pub fn gsl_movstat_accum_minmax(n: SizeT) -> GslMovstatAccum {
    GslMovstatAccum::new(n)
}