use std::cmp::Ordering;
use std::ops::{BitAnd, Neg};

type SizeT = usize;
type ZahlCharT = u64;

#[derive(Clone, Debug)]
pub struct Zahl {
    pub sign: i32,
    pub used: SizeT,
    pub alloced: SizeT,
    pub chars: Vec<ZahlCharT>,
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

    pub fn zero(&mut self) {
        self.sign = 0;
        self.used = 0;
    }

    pub fn is_zero(&self) -> bool {
        self.sign == 0
    }

    pub fn signum(&self) -> i32 {
        self.sign
    }

    pub fn abs(&mut self, other: &Zahl) {
        if !std::ptr::eq(self, other) {
            self.set(other);
        }
        self.sign &= 1;
    }

    pub fn set(&mut self, other: &Zahl) {
        if other.is_zero() {
            self.zero();
        } else {
            self.sign = other.sign;
            self.used = other.used;
            if self.alloced < other.used {
                self.realloc(other.used);
            }
            self.chars[..other.used].copy_from_slice(&other.chars[..other.used]);
        }
    }

    fn realloc(&mut self, new_size: SizeT) {
        self.chars.resize(new_size, 0);
        self.alloced = new_size;
    }

    fn memcpy(&mut self, src: &[ZahlCharT], n: SizeT) {
        self.chars[..n].copy_from_slice(&src[..n]);
    }

    fn memset(&mut self, value: ZahlCharT, n: SizeT) {
        for i in 0..n {
            self.chars[i] = value;
        }
    }

    fn add_impl_4(&mut self, b: &Zahl, c: &Zahl, n: SizeT) {
        let mut carry = 0u64;
        let mut i = 0;

        while i <= n {
            if i + 4 <= n {
                let (sum0, c0) = b.chars[i].overflowing_add(c.chars[i]);
                let (sum0, c1) = sum0.overflowing_add(carry);
                self.chars[i] = sum0;
                carry = (c0 | c1) as u64;

                let (sum1, c0) = b.chars[i+1].overflowing_add(c.chars[i+1]);
                let (sum1, c1) = sum1.overflowing_add(carry);
                self.chars[i+1] = sum1;
                carry = (c0 | c1) as u64;

                let (sum2, c0) = b.chars[i+2].overflowing_add(c.chars[i+2]);
                let (sum2, c1) = sum2.overflowing_add(carry);
                self.chars[i+2] = sum2;
                carry = (c0 | c1) as u64;

                let (sum3, c0) = b.chars[i+3].overflowing_add(c.chars[i+3]);
                let (sum3, c1) = sum3.overflowing_add(carry);
                self.chars[i+3] = sum3;
                carry = (c0 | c1) as u64;

                i += 4;
            } else {
                let (sum, c0) = b.chars[i].overflowing_add(c.chars[i]);
                let (sum, c1) = sum.overflowing_add(carry);
                self.chars[i] = sum;
                carry = (c0 | c1) as u64;
                i += 1;
            }
        }

        while carry != 0 {
            if i >= self.chars.len() {
                self.chars.push(0);
                self.used += 1;
            }
            let (sum, c) = self.chars[i].overflowing_add(carry);
            self.chars[i] = sum;
            carry = c as u64;
            i += 1;
        }

        if self.used < i {
            self.used = i;
        }
    }

    fn add_impl_3(&mut self, b: &Zahl, n: SizeT) {
        let mut carry = 0u64;
        let mut i = 0;

        while i <= n {
            if i + 4 <= n {
                let (sum0, c0) = self.chars[i].overflowing_add(b.chars[i]);
                let (sum0, c1) = sum0.overflowing_add(carry);
                self.chars[i] = sum0;
                carry = (c0 | c1) as u64;

                let (sum1, c0) = self.chars[i+1].overflowing_add(b.chars[i+1]);
                let (sum1, c1) = sum1.overflowing_add(carry);
                self.chars[i+1] = sum1;
                carry = (c0 | c1) as u64;

                let (sum2, c0) = self.chars[i+2].overflowing_add(b.chars[i+2]);
                let (sum2, c1) = sum2.overflowing_add(carry);
                self.chars[i+2] = sum2;
                carry = (c0 | c1) as u64;

                let (sum3, c0) = self.chars[i+3].overflowing_add(b.chars[i+3]);
                let (sum3, c1) = sum3.overflowing_add(carry);
                self.chars[i+3] = sum3;
                carry = (c0 | c1) as u64;

                i += 4;
            } else {
                let (sum, c0) = self.chars[i].overflowing_add(b.chars[i]);
                let (sum, c1) = sum.overflowing_add(carry);
                self.chars[i] = sum;
                carry = (c0 | c1) as u64;
                i += 1;
            }
        }

        while carry != 0 {
            if i >= self.chars.len() {
                self.chars.push(0);
                self.used += 1;
            }
            let (sum, c) = self.chars[i].overflowing_add(carry);
            self.chars[i] = sum;
            carry = c as u64;
            i += 1;
        }

        if self.used < i {
            self.used = i;
        }
    }

    pub fn add_unsigned(&mut self, b: &Zahl, c: &Zahl) {
        if b.is_zero() {
            self.abs(c);
            return;
        } else if c.is_zero() {
            self.abs(b);
            return;
        }

        let size = std::cmp::max(b.used, c.used);
        let n = b.used + c.used - size;

        if self.alloced < size + 1 {
            self.realloc(size + 1);
        }

        if std::ptr::eq(self, b) {
            if self.used < c.used {
                let new_n = c.used;
                self.memset(0, new_n - self.used);
            }
            self.add_impl_3(c, n);
        } else if std::ptr::eq(self, c) {
            if self.used < b.used {
                let new_n = b.used;
                self.memset(0, new_n - self.used);
            }
            self.add_impl_3(b, n);
        } else if b.used > c.used {
            self.memcpy(&b.chars[n..size], size - n);
            self.used = size;
            self.add_impl_4(b, c, n);
        } else {
            self.memcpy(&c.chars[n..size], size - n);
            self.used = size;
            self.add_impl_4(b, c, n);
        }

        self.sign = 1;
    }

    pub fn add_unsigned_assign(&mut self, b: &Zahl) {
        if self.is_zero() {
            self.abs(b);
            return;
        } else if b.is_zero() {
            return;
        }

        let size = std::cmp::max(self.used, b.used);
        let n = self.used + b.used - size;

        if self.alloced < size + 1 {
            self.realloc(size + 1);
        }

        if self.used < b.used {
            let new_n = b.used;
            self.memset(0, new_n - self.used);
        }

        self.add_impl_3(b, n);
        self.sign = 1;
    }

    pub fn add(&mut self, b: &Zahl, c: &Zahl) {
        if b.is_zero() {
            if !std::ptr::eq(self, c) {
                self.set(c);
            }
        } else if c.is_zero() {
            if !std::ptr::eq(self, b) {
                self.set(b);
            }
        } else if b.signum() < 0 {
            if c.signum() < 0 {
                self.add_unsigned(b, c);
                self.sign = -self.signum();
            } else {
                self.sub_unsigned(c, b);
            }
        } else if c.signum() < 0 {
            self.sub_unsigned(b, c);
        } else {
            self.add_unsigned(b, c);
        }
    }

    pub fn sub_unsigned(&mut self, b: &Zahl, c: &Zahl) {
        // Implementation of unsigned subtraction
        // Similar pattern to add_unsigned but with subtraction
        unimplemented!()
    }
}