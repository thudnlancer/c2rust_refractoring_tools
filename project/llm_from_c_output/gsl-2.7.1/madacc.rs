use std::cmp::Ordering;
use std::error::Error;
use std::fmt;
use std::ptr::NonNull;

#[derive(Debug)]
struct MadError {
    message: String,
}

impl fmt::Display for MadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for MadError {}

type Result<T> = std::result::Result<T, MadError>;

struct RingBuffer {
    array: Vec<f64>,
    head: usize,
    tail: usize,
    size: usize,
}

impl RingBuffer {
    fn new(n: usize) -> Self {
        RingBuffer {
            array: vec![0.0; n],
            head: 0,
            tail: 0,
            size: n,
        }
    }

    fn is_empty(&self) -> bool {
        self.head == self.tail
    }

    fn insert(&mut self, x: f64) {
        self.array[self.tail] = x;
        self.tail = (self.tail + 1) % self.size;
    }

    fn pop_back(&mut self) -> Option<f64> {
        if self.is_empty() {
            None
        } else {
            self.tail = (self.tail + self.size - 1) % self.size;
            Some(self.array[self.tail])
        }
    }

    fn len(&self) -> usize {
        (self.tail + self.size - self.head) % self.size
    }

    fn get(&self, index: usize) -> Option<f64> {
        if index >= self.len() {
            None
        } else {
            Some(self.array[(self.head + index) % self.size])
        }
    }
}

struct MedianAccumulator {
    window: Vec<f64>,
    sorted: Vec<f64>,
}

impl MedianAccumulator {
    fn new(n: usize) -> Self {
        MedianAccumulator {
            window: Vec::with_capacity(n),
            sorted: Vec::with_capacity(n),
        }
    }

    fn insert(&mut self, x: f64) {
        self.window.push(x);
        let pos = self.sorted.binary_search_by(|v| {
            v.partial_cmp(&x).unwrap_or(Ordering::Equal)
        }).unwrap_or_else(|e| e);
        self.sorted.insert(pos, x);
    }

    fn get_median(&self) -> Option<f64> {
        if self.sorted.is_empty() {
            None
        } else {
            let n = self.sorted.len();
            if n % 2 == 1 {
                Some(self.sorted[n / 2])
            } else {
                Some((self.sorted[n / 2 - 1] + self.sorted[n / 2]) / 2.0)
            }
        }
    }
}

struct MadAccumulator {
    n: usize,
    median_acc: MedianAccumulator,
    rbuf: RingBuffer,
    work: Vec<f64>,
}

impl MadAccumulator {
    fn new(n: usize) -> Self {
        MadAccumulator {
            n,
            median_acc: MedianAccumulator::new(n),
            rbuf: RingBuffer::new(n),
            work: vec![0.0; n],
        }
    }

    fn insert(&mut self, x: f64) {
        self.median_acc.insert(x);
        self.rbuf.insert(x);
    }

    fn delete(&mut self) {
        if !self.rbuf.is_empty() {
            if let Some(x) = self.rbuf.pop_back() {
                if let Ok(pos) = self.median_acc.sorted.binary_search_by(|v| {
                    v.partial_cmp(&x).unwrap_or(Ordering::Equal)
                }) {
                    self.median_acc.sorted.remove(pos);
                }
            }
        }
    }

    fn compute_medmad(&self, scale: f64) -> Result<(f64, f64)> {
        if self.rbuf.is_empty() {
            Err(MadError {
                message: "no samples yet added to workspace".to_string(),
            })
        } else {
            let median = self.median_acc.get_median().ok_or_else(|| MadError {
                message: "failed to compute median".to_string(),
            })?;

            let n = self.rbuf.len();
            for i in 0..n {
                self.work[i] = f64::abs(self.rbuf.get(i).unwrap() - median);
            }

            let mut work_copy = self.work[..n].to_vec();
            work_copy.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
            let mad = if n % 2 == 1 {
                work_copy[n / 2]
            } else {
                (work_copy[n / 2 - 1] + work_copy[n / 2]) / 2.0
            };

            Ok((median, scale * mad))
        }
    }
}

pub struct MovStatAccumulator {
    mad_acc: MadAccumulator,
}

impl MovStatAccumulator {
    pub fn new(n: usize) -> Self {
        MovStatAccumulator {
            mad_acc: MadAccumulator::new(n),
        }
    }

    pub fn insert(&mut self, x: f64) {
        self.mad_acc.insert(x);
    }

    pub fn delete(&mut self) {
        self.mad_acc.delete();
    }

    pub fn compute_medmad(&self, scale: f64) -> Result<(f64, f64)> {
        self.mad_acc.compute_medmad(scale)
    }
}