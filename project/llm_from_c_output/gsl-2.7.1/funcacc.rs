use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct MovStatError {
    message: String,
}

impl fmt::Display for MovStatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for MovStatError {}

type Result<T> = std::result::Result<T, MovStatError>;

type FuncAccType = f64;
type RingBufType = FuncAccType;

struct RingBuf {
    buffer: Vec<RingBufType>,
    head: usize,
    tail: usize,
    size: usize,
    capacity: usize,
}

impl RingBuf {
    fn new(capacity: usize) -> Self {
        RingBuf {
            buffer: vec![0.0; capacity],
            head: 0,
            tail: 0,
            size: 0,
            capacity,
        }
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn insert(&mut self, x: RingBufType) {
        if self.size == self.capacity {
            self.pop_back();
        }
        self.buffer[self.tail] = x;
        self.tail = (self.tail + 1) % self.capacity;
        self.size += 1;
    }

    fn pop_back(&mut self) -> Option<RingBufType> {
        if self.is_empty() {
            None
        } else {
            self.tail = (self.tail + self.capacity - 1) % self.capacity;
            let val = self.buffer[self.tail];
            self.size -= 1;
            Some(val)
        }
    }

    fn copy_to(&self, dest: &mut [FuncAccType]) -> usize {
        let count = self.size.min(dest.len());
        for i in 0..count {
            let idx = (self.head + i) % self.capacity;
            dest[i] = self.buffer[idx];
        }
        count
    }
}

struct FuncAccState {
    window: Vec<FuncAccType>,
    rbuf: RingBuf,
}

struct MovStatFunction {
    function: Box<dyn Fn(&[FuncAccType]) -> FuncAccType>,
}

impl MovStatFunction {
    fn eval(&self, data: &[FuncAccType]) -> FuncAccType {
        (self.function)(data)
    }
}

struct FuncAccumulator {
    state: FuncAccState,
    function: MovStatFunction,
}

impl FuncAccumulator {
    fn new(n: usize, function: MovStatFunction) -> Self {
        FuncAccumulator {
            state: FuncAccState {
                window: vec![0.0; n],
                rbuf: RingBuf::new(n),
            },
            function,
        }
    }

    fn insert(&mut self, x: FuncAccType) -> Result<()> {
        self.state.rbuf.insert(x);
        Ok(())
    }

    fn delete(&mut self) -> Result<()> {
        if !self.state.rbuf.is_empty() {
            self.state.rbuf.pop_back();
        }
        Ok(())
    }

    fn get(&mut self) -> Result<FuncAccType> {
        let n = self.state.rbuf.copy_to(&mut self.state.window);
        Ok(self.function.eval(&self.state.window[..n]))
    }
}

pub struct MovStatAccumulator {
    func_accum: FuncAccumulator,
}

impl MovStatAccumulator {
    pub fn new(n: usize, function: MovStatFunction) -> Self {
        MovStatAccumulator {
            func_accum: FuncAccumulator::new(n, function),
        }
    }

    pub fn insert(&mut self, x: f64) -> Result<()> {
        self.func_accum.insert(x)
    }

    pub fn delete(&mut self) -> Result<()> {
        self.func_accum.delete()
    }

    pub fn get(&mut self) -> Result<f64> {
        self.func_accum.get()
    }
}