use std::cmp::{max, min};
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

    fn memcpy(dest: &mut [u64], src: &[u64], n: usize) {
        if n <= src.len() && n <= dest.len() {
            dest[..n].copy_from_slice(&src[..n]);
        }
    }

    fn memcpy_range(dest: &mut [u64], src: &[u64], i: usize, n: usize) {
        let len = min(n - i, src.len() - i);
        if len > 0 {
            dest[i..i + len].copy_from_slice(&src[i..i + len]);
        }
    }

    pub fn xor(&mut self, b: &Zahl, c: &Zahl) {
        if b.is_zero() {
            self.set(c);
            return;
        } else if c.is_zero() {
            self.set(b);
            return;
        }

        let bn = b.used;
        let bc = &b.chars;
        let cn = c.used;
        let cc = &c.chars;

        let n = min(bn, cn);
        let m = max(bn, cn);

        if self.alloced < m {
            self.chars.resize(m, 0);
            self.alloced = m;
        }

        if ptr::eq(self, b) {
            for i in 0..n {
                self.chars[i] ^= cc[i];
            }
            if self.used < cn {
                Zahl::memcpy_range(&mut self.chars, cc, n, m);
            }
        } else if ptr::eq(self, c) {
            for i in 0..n {
                self.chars[i] ^= bc[i];
            }
            if self.used < bn {
                Zahl::memcpy_range(&mut self.chars, bc, n, m);
            }
        } else if m == bn {
            for i in 0..n {
                self.chars[i] = bc[i] ^ cc[i];
            }
            Zahl::memcpy_range(&mut self.chars, bc, n, m);
        } else {
            for i in 0..n {
                self.chars[i] = bc[i] ^ cc[i];
            }
            Zahl::memcpy_range(&mut self.chars, cc, n, m);
        }

        self.used = m;
        while self.used > 0 && self.chars[self.used - 1] == 0 {
            self.used -= 1;
        }

        self.sign = if self.used != 0 {
            1 - 2 * ((b.signum() ^ c.signum()) < 0) as i32
        } else {
            0
        };
    }
}