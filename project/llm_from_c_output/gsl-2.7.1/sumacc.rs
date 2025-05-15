use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct RingBuffer {
    buffer: Vec<f64>,
    head: usize,
    tail: usize,
    count: usize,
    capacity: usize,
}

impl RingBuffer {
    fn new(capacity: usize) -> Self {
        RingBuffer {
            buffer: vec![0.0; capacity],
            head: 0,
            tail: 0,
            count: 0,
            capacity,
        }
    }

    fn is_full(&self) -> bool {
        self.count == self.capacity
    }

    fn is_empty(&self) -> bool {
        self.count == 0
    }

    fn insert(&mut self, x: f64) {
        if self.is_full() {
            self.tail = (self.tail + 1) % self.capacity;
        } else {
            self.count += 1;
        }
        self.buffer[self.head] = x;
        self.head = (self.head + 1) % self.capacity;
    }

    fn pop_back(&mut self) -> Option<f64> {
        if self.is_empty() {
            None
        } else {
            let val = self.buffer[self.tail];
            self.tail = (self.tail + 1) % self.capacity;
            self.count -= 1;
            Some(val)
        }
    }

    fn peek_back(&self) -> f64 {
        if self.is_empty() {
            0.0
        } else {
            self.buffer[self.tail]
        }
    }
}

#[derive(Debug)]
struct SumAccState {
    sum: f64,
    rbuf: RingBuffer,
}

impl SumAccState {
    fn new(n: usize) -> Self {
        SumAccState {
            sum: 0.0,
            rbuf: RingBuffer::new(n),
        }
    }

    fn insert(&mut self, x: f64) -> Result<(), Box<dyn Error>> {
        if self.rbuf.is_full() {
            self.sum -= self.rbuf.peek_back();
        }

        self.sum += x;
        self.rbuf.insert(x);
        Ok(())
    }

    fn delete(&mut self) -> Result<(), Box<dyn Error>> {
        if !self.rbuf.is_empty() {
            self.sum -= self.rbuf.peek_back();
            self.rbuf.pop_back();
        }
        Ok(())
    }

    fn get(&self) -> f64 {
        self.sum
    }
}

#[derive(Debug)]
struct SumAccumulator;

impl SumAccumulator {
    fn size(n: usize) -> usize {
        std::mem::size_of::<SumAccState>() + std::mem::size_of::<RingBuffer>() * n
    }

    fn init(n: usize) -> Result<SumAccState, Box<dyn Error>> {
        Ok(SumAccState::new(n))
    }
}

#[derive(Debug)]
struct GslError;

impl fmt::Display for GslError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GSL error occurred")
    }
}

impl Error for GslError {}

fn gsl_movstat_accum_sum() -> Result<(), Box<dyn Error>> {
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    gsl_movstat_accum_sum()
}