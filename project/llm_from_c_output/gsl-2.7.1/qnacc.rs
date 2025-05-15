use std::cmp::Ordering;
use std::ptr::NonNull;

struct RingBuf {
    buffer: Vec<f64>,
    head: usize,
    tail: usize,
    size: usize,
    capacity: usize,
}

impl RingBuf {
    fn new(capacity: usize) -> Self {
        Self {
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

    fn insert(&mut self, x: f64) {
        if self.size == self.capacity {
            self.pop_back();
        }

        self.buffer[self.tail] = x;
        self.tail = (self.tail + 1) % self.capacity;
        self.size += 1;
    }

    fn pop_back(&mut self) -> Option<f64> {
        if self.is_empty() {
            return None;
        }

        self.tail = (self.tail + self.capacity - 1) % self.capacity;
        let val = self.buffer[self.tail];
        self.size -= 1;
        Some(val)
    }

    fn copy_to(&self, dest: &mut [f64]) -> usize {
        let n = self.size.min(dest.len());
        if n == 0 {
            return 0;
        }

        let mut idx = 0;
        let mut pos = (self.tail + self.capacity - self.size) % self.capacity;

        while idx < n {
            dest[idx] = self.buffer[pos];
            pos = (pos + 1) % self.capacity;
            idx += 1;
        }

        n
    }
}

struct QnAccState {
    window: Vec<f64>,
    work: Vec<f64>,
    work_int: Vec<i32>,
    rbuf: RingBuf,
}

impl QnAccState {
    fn new(n: usize) -> Self {
        Self {
            window: vec![0.0; n],
            work: vec![0.0; 3 * n],
            work_int: vec![0; 5 * n],
            rbuf: RingBuf::new(n),
        }
    }

    fn insert(&mut self, x: f64) {
        self.rbuf.insert(x);
    }

    fn delete(&mut self) {
        if !self.rbuf.is_empty() {
            self.rbuf.pop_back();
        }
    }

    fn get(&mut self) -> Result<f64, &'static str> {
        let n = self.rbuf.copy_to(&mut self.window);
        if n == 0 {
            return Err("Empty window");
        }

        self.window[..n].sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
        Ok(compute_qn(&self.window[..n], &mut self.work, &mut self.work_int))
    }
}

fn compute_qn(sorted_data: &[f64], work: &mut [f64], work_int: &mut [i32]) -> f64 {
    // Simplified Qn computation - replace with actual implementation
    // This is a placeholder for the actual GSL Qn algorithm
    let n = sorted_data.len();
    if n < 2 {
        return 0.0;
    }

    let k = n / 2;
    let mut diffs = Vec::with_capacity(k * (n - 1));
    for i in 0..n {
        for j in (i + 1)..n {
            diffs.push((sorted_data[j] - sorted_data[i]).abs());
        }
    }

    diffs.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
    let result = if k < diffs.len() {
        diffs[k]
    } else {
        0.0
    };

    2.219 * result
}

pub struct QnAccumulator {
    state: QnAccState,
}

impl QnAccumulator {
    pub fn new(window_size: usize) -> Self {
        Self {
            state: QnAccState::new(window_size),
        }
    }

    pub fn insert(&mut self, x: f64) {
        self.state.insert(x);
    }

    pub fn delete(&mut self) {
        self.state.delete();
    }

    pub fn get(&mut self) -> Result<f64, &'static str> {
        self.state.get()
    }
}