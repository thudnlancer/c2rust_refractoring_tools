use std::f64;
use std::ptr;
use std::mem;

struct RingBuf {
    buffer: Vec<f64>,
    head: usize,
    tail: usize,
    size: usize,
    capacity: usize,
}

impl RingBuf {
    fn new(n: usize) -> Self {
        RingBuf {
            buffer: vec![0.0; n],
            head: 0,
            tail: 0,
            size: 0,
            capacity: n,
        }
    }

    fn is_full(&self) -> bool {
        self.size == self.capacity
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn insert(&mut self, x: f64) {
        if self.is_full() {
            self.pop_back();
        }

        self.buffer[self.head] = x;
        self.head = (self.head + 1) % self.capacity;
        self.size += 1;
    }

    fn pop_back(&mut self) -> Option<f64> {
        if self.is_empty() {
            None
        } else {
            let val = self.buffer[self.tail];
            self.tail = (self.tail + 1) % self.capacity;
            self.size -= 1;
            Some(val)
        }
    }

    fn peek_back(&self) -> f64 {
        self.buffer[self.tail]
    }
}

struct MvaccState {
    n: usize,        // window size
    k: usize,        // number of samples currently in window
    mean: f64,       // current window mean
    m2: f64,         // current window M2
    rbuf: RingBuf,   // ring buffer storing current window
}

impl MvaccState {
    fn new(n: usize) -> Self {
        MvaccState {
            n,
            k: 0,
            mean: 0.0,
            m2: 0.0,
            rbuf: RingBuf::new(n),
        }
    }

    fn insert(&mut self, x: f64) {
        if self.rbuf.is_full() {
            // remove oldest window element and add new one
            let old = self.rbuf.peek_back();
            let prev_mean = self.mean;

            self.mean += (x - old) / self.n as f64;
            self.m2 += ((old - prev_mean) + (x - self.mean)) * (x - old);
        } else {
            let delta = x - self.mean;

            // Welford algorithm:
            // mu_new = mu_old + (x - mu_old) / n
            // M2_new = M2_old + (x - mu_old) * (x - mu_new)
            self.k += 1;
            self.mean += delta / self.k as f64;
            self.m2 += delta * (x - self.mean);
        }

        // add new element to ring buffer
        self.rbuf.insert(x);
    }

    fn delete(&mut self) {
        if !self.rbuf.is_empty() {
            if self.k > 1 {
                // mu_new = mu_old + (mu_old - x_old) / (n - 1)
                // M2_new = M2_old - (mu_old - x_old) * (mu_new - x_old)
                let old = self.rbuf.peek_back();
                let prev_mean = self.mean;
                let delta = prev_mean - old;

                self.mean += delta / (self.k - 1) as f64;
                self.m2 -= delta * (self.mean - old);
            } else if self.k == 1 {
                self.mean = 0.0;
                self.m2 = 0.0;
            }

            self.rbuf.pop_back();
            self.k -= 1;
        }
    }

    fn mean(&self) -> f64 {
        self.mean
    }

    fn variance(&self) -> f64 {
        if self.k < 2 {
            0.0
        } else {
            self.m2 / (self.k - 1) as f64
        }
    }

    fn sd(&self) -> f64 {
        self.variance().sqrt()
    }
}

pub struct MovStatAccum {
    state: MvaccState,
}

impl MovStatAccum {
    pub fn mean(n: usize) -> Self {
        MovStatAccum {
            state: MvaccState::new(n),
        }
    }

    pub fn variance(n: usize) -> Self {
        MovStatAccum {
            state: MvaccState::new(n),
        }
    }

    pub fn sd(n: usize) -> Self {
        MovStatAccum {
            state: MvaccState::new(n),
        }
    }

    pub fn insert(&mut self, x: f64) {
        self.state.insert(x);
    }

    pub fn delete(&mut self) {
        self.state.delete();
    }

    pub fn get_mean(&self) -> f64 {
        self.state.mean()
    }

    pub fn get_variance(&self) -> f64 {
        self.state.variance()
    }

    pub fn get_sd(&self) -> f64 {
        self.state.sd()
    }
}