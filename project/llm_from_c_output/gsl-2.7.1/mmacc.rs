use std::collections::VecDeque;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct MovStatError {
    message: String,
}

impl fmt::Display for MovStatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for MovStatError {}

impl MovStatError {
    fn new(msg: &str) -> Self {
        MovStatError {
            message: msg.to_string(),
        }
    }
}

pub struct RingBuffer {
    data: Vec<f64>,
    head: usize,
    tail: usize,
    size: usize,
    capacity: usize,
}

impl RingBuffer {
    pub fn new(capacity: usize) -> Self {
        RingBuffer {
            data: vec![0.0; capacity],
            head: 0,
            tail: 0,
            size: 0,
            capacity,
        }
    }

    pub fn insert(&mut self, x: f64) {
        if self.size == self.capacity {
            self.tail = (self.tail + 1) % self.capacity;
        } else {
            self.size += 1;
        }
        self.data[self.head] = x;
        self.head = (self.head + 1) % self.capacity;
    }

    pub fn pop_back(&mut self) -> Option<f64> {
        if self.size == 0 {
            None
        } else {
            let val = self.data[self.tail];
            self.tail = (self.tail + 1) % self.capacity;
            self.size -= 1;
            Some(val)
        }
    }

    pub fn peek_tail(&self) -> Option<f64> {
        if self.size == 0 {
            None
        } else {
            Some(self.data[self.tail])
        }
    }

    pub fn get(&self, index: usize) -> Option<f64> {
        if index < self.size {
            Some(self.data[(self.tail + index) % self.capacity])
        } else {
            None
        }
    }
}

pub struct MmAccState {
    n: usize,
    k: usize,
    xprev: f64,
    rbuf: RingBuffer,
    minque: VecDeque<usize>,
    maxque: VecDeque<usize>,
}

impl MmAccState {
    pub fn new(n: usize) -> Self {
        MmAccState {
            n,
            k: 0,
            xprev: 0.0,
            rbuf: RingBuffer::new(n),
            minque: VecDeque::with_capacity(n + 1),
            maxque: VecDeque::with_capacity(n + 1),
        }
    }

    pub fn insert(&mut self, x: f64) -> Result<(), MovStatError> {
        if self.k == 0 {
            self.rbuf.insert(x);
            self.maxque.push_back(self.rbuf.head);
            self.minque.push_back(self.rbuf.head);
        } else {
            if x > self.xprev {
                self.maxque.pop_back();

                while !self.maxque.is_empty() {
                    if let Some(&back) = self.maxque.back() {
                        if x <= self.rbuf.data[back] {
                            break;
                        }
                        self.maxque.pop_back();
                    }
                }
            } else {
                self.minque.pop_back();

                while !self.minque.is_empty() {
                    if let Some(&back) = self.minque.back() {
                        if x >= self.rbuf.data[back] {
                            break;
                        }
                        self.minque.pop_back();
                    }
                }
            }

            let tail = self.rbuf.tail;
            self.rbuf.insert(x);
            let head = self.rbuf.head;

            self.maxque.push_back(head);
            self.minque.push_back(head);

            if self.k == self.n {
                if self.maxque.front() != self.maxque.back() && tail == *self.maxque.front().unwrap() {
                    self.maxque.pop_front();
                } else if self.minque.front() != self.minque.back() && tail == *self.minque.front().unwrap() {
                    self.minque.pop_front();
                }
            }
        }

        if self.k < self.n {
            self.k += 1;
        }

        self.xprev = x;
        Ok(())
    }

    pub fn delete(&mut self) -> Result<(), MovStatError> {
        if self.k > 0 {
            if let Some(&front_max) = self.maxque.front() {
                if self.rbuf.tail == front_max {
                    self.maxque.pop_front();
                }
            }

            if let Some(&front_min) = self.minque.front() {
                if self.rbuf.tail == front_min {
                    self.minque.pop_front();
                }
            }

            self.rbuf.pop_back();
            self.k -= 1;
        }
        Ok(())
    }

    pub fn min(&self) -> Result<f64, MovStatError> {
        if self.k == 0 {
            Err(MovStatError::new("no samples yet added to workspace"))
        } else {
            Ok(self.rbuf.data[*self.minque.front().unwrap()])
        }
    }

    pub fn max(&self) -> Result<f64, MovStatError> {
        if self.k == 0 {
            Err(MovStatError::new("no samples yet added to workspace"))
        } else {
            Ok(self.rbuf.data[*self.maxque.front().unwrap()])
        }
    }

    pub fn minmax(&self) -> Result<(f64, f64), MovStatError> {
        if self.k == 0 {
            Err(MovStatError::new("no samples yet added to workspace"))
        } else {
            Ok((
                self.rbuf.data[*self.minque.front().unwrap()],
                self.rbuf.data[*self.maxque.front().unwrap()],
            ))
        }
    }
}

pub struct MinAccumulator;
pub struct MaxAccumulator;
pub struct MinMaxAccumulator;

impl MinAccumulator {
    pub fn new(n: usize) -> MmAccState {
        MmAccState::new(n)
    }
}

impl MaxAccumulator {
    pub fn new(n: usize) -> MmAccState {
        MmAccState::new(n)
    }
}

impl MinMaxAccumulator {
    pub fn new(n: usize) -> MmAccState {
        MmAccState::new(n)
    }
}