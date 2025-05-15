use std::cmp::Ordering;
use std::ptr;
use std::slice;

struct RingBuf {
    data: Vec<f64>,
    head: usize,
    tail: usize,
    size: usize,
    capacity: usize,
}

impl RingBuf {
    fn new(capacity: usize) -> Self {
        Self {
            data: vec![0.0; capacity],
            head: 0,
            tail: 0,
            size: 0,
            capacity,
        }
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn is_full(&self) -> bool {
        self.size == self.capacity
    }

    fn insert(&mut self, x: f64) {
        if self.is_full() {
            self.pop_back();
        }

        self.data[self.tail] = x;
        self.tail = (self.tail + 1) % self.capacity;
        self.size += 1;
    }

    fn pop_back(&mut self) -> Option<f64> {
        if self.is_empty() {
            return None;
        }

        self.tail = (self.tail + self.capacity - 1) % self.capacity;
        let val = self.data[self.tail];
        self.size -= 1;
        Some(val)
    }

    fn copy_to_slice(&self, dest: &mut [f64]) -> usize {
        let n = self.size;
        if n == 0 {
            return 0;
        }

        if self.head < self.tail {
            dest[..n].copy_from_slice(&self.data[self.head..self.tail]);
        } else {
            let first_part = self.capacity - self.head;
            dest[..first_part].copy_from_slice(&self.data[self.head..]);
            dest[first_part..n].copy_from_slice(&self.data[..self.tail]);
        }

        n
    }
}

struct QQRState {
    window: Vec<f64>,
    rbuf: RingBuf,
}

impl QQRState {
    fn new(n: usize) -> Self {
        Self {
            window: vec![0.0; n],
            rbuf: RingBuf::new(n),
        }
    }

    fn size(n: usize) -> usize {
        std::mem::size_of::<Self>() + n * std::mem::size_of::<f64>() + n * std::mem::size_of::<f64>()
    }

    fn insert(&mut self, x: f64) -> Result<(), &'static str> {
        self.rbuf.insert(x);
        Ok(())
    }

    fn delete(&mut self) -> Result<(), &'static str> {
        if !self.rbuf.is_empty() {
            self.rbuf.pop_back();
        }
        Ok(())
    }

    fn get(&self, q: f64) -> Result<f64, &'static str> {
        let n = self.rbuf.copy_to_slice(&mut self.window[..]);
        if n == 0 {
            return Err("empty window");
        }

        let window = &mut self.window[..n];
        window.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));

        let quant1 = quantile(window, q)?;
        let quant2 = quantile(window, 1.0 - q)?;

        Ok(quant2 - quant1)
    }
}

fn quantile(sorted_data: &[f64], q: f64) -> Result<f64, &'static str> {
    if q < 0.0 || q > 1.0 {
        return Err("q must be between 0 and 1");
    }

    let n = sorted_data.len();
    if n == 0 {
        return Err("empty data");
    }

    let index = q * (n - 1) as f64;
    let lower = index.floor() as usize;
    let upper = index.ceil() as usize;

    if lower == upper {
        Ok(sorted_data[lower])
    } else {
        let weight = index - lower as f64;
        Ok(sorted_data[lower] + weight * (sorted_data[upper] - sorted_data[lower]))
    }
}

pub struct QQRAccumulator {
    state: QQRState,
}

impl QQRAccumulator {
    pub fn new(n: usize) -> Self {
        Self {
            state: QQRState::new(n),
        }
    }

    pub fn size(n: usize) -> usize {
        QQRState::size(n)
    }

    pub fn insert(&mut self, x: f64) -> Result<(), &'static str> {
        self.state.insert(x)
    }

    pub fn delete(&mut self) -> Result<(), &'static str> {
        self.state.delete()
    }

    pub fn get(&self, q: f64) -> Result<f64, &'static str> {
        self.state.get(q)
    }
}