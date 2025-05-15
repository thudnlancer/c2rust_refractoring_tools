use std::cmp::{min, max};
use std::ptr;

#[derive(Clone)]
pub struct Zahl {
    pub sign: i32,
    pub used: usize,
    pub alloced: usize,
    pub chars: Vec<u64>,
}

impl Zahl {
    pub fn new() -> Self {
        Zahl {
            sign: 0,
            used: 0,
            alloced: 0,
            chars: Vec::new(),
        }
    }

    pub fn signum(&self) -> i32 {
        self.sign
    }

    pub fn is_zero(&self) -> bool {
        self.sign == 0
    }

    pub fn set(&mut self, other: &Zahl) {
        if other.is_zero() {
            self.sign = 0;
        } else {
            self.sign = other.sign;
            self.used = other.used;
            if self.alloced < other.used {
                self.chars.resize(other.used, 0);
                self.alloced = other.used;
            }
            self.chars[..other.used].copy_from_slice(&other.chars[..other.used]);
        }
    }

    fn memcpy(d: &mut [u64], s: &[u64], n: usize) {
        if n == 0 {
            return;
        }
        d[..n].copy_from_slice(&s[..n]);
    }

    fn memcpy_range(d: &mut [u64], s: &[u64], i: usize, n: usize) {
        let len = n - i;
        Self::memcpy(&mut d[i..], &s[i..], len);
    }

    pub fn or(&mut self, b: &Zahl, c: &Zahl) {
        if b.is_zero() {
            self.set(c);
            return;
        } else if c.is_zero() {
            self.set(b);
            return;
        }

        let n = min(b.used, c.used);
        let m = max(b.used, c.used);

        if self.alloced < m {
            self.chars.resize(m, 0);
            self.alloced = m;
        }

        if ptr::eq(self, b) {
            for i in 0..n {
                self.chars[i] = b.chars[i] | c.chars[i];
            }
            if self.used < c.used {
                Self::memcpy_range(&mut self.chars, &c.chars, n, m);
            }
        } else if ptr::eq(self, c) {
            for i in 0..n {
                self.chars[i] = b.chars[i] | c.chars[i];
            }
            if self.used < b.used {
                Self::memcpy_range(&mut self.chars, &b.chars, n, m);
            }
        } else if m == b.used {
            for i in 0..n {
                self.chars[i] = b.chars[i] | c.chars[i];
            }
            Self::memcpy_range(&mut self.chars, &b.chars, n, m);
        } else {
            for i in 0..n {
                self.chars[i] = b.chars[i] | c.chars[i];
            }
            Self::memcpy_range(&mut self.chars, &c.chars, n, m);
        }

        self.used = m;
        self.sign = if b.signum() + c.signum() == 2 { 1 } else { -1 };
    }
}