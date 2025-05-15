use std::cmp::Ordering;
use std::ptr::NonNull;

type SizeT = usize;
type Uint64T = u64;
type ZahlCharT = Uint64T;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Zahl {
    pub sign: i32,
    pub used: SizeT,
    pub alloced: SizeT,
    pub chars: Vec<ZahlCharT>,
}

impl Zahl {
    pub fn new() -> Self {
        Self {
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

    pub fn set(&mut self, other: &Self) {
        if other.is_zero() {
            self.zero();
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

    pub fn cmp_mag(&self, other: &Self) -> Ordering {
        if self.is_zero() {
            return if other.is_zero() {
                Ordering::Equal
            } else {
                Ordering::Less
            };
        }
        if other.is_zero() {
            return Ordering::Greater;
        }

        let mut i = self.used.saturating_sub(1);
        let mut j = other.used.saturating_sub(1);

        while i > j {
            if self.chars[i] != 0 {
                return Ordering::Greater;
            }
            i = i.saturating_sub(1);
        }

        while j > i {
            if other.chars[j] != 0 {
                return Ordering::Less;
            }
            j = j.saturating_sub(1);
        }

        while i > 0 && self.chars[i] == other.chars[i] {
            i = i.saturating_sub(1);
        }

        self.chars[i].cmp(&other.chars[i])
    }

    pub fn signum(&self) -> i32 {
        self.sign
    }

    pub fn abs(&mut self, other: &Self) {
        if self as *const _ != other as *const _ {
            self.set(other);
        }
        self.sign &= 1;
    }

    pub fn neg(&mut self, other: &Self) {
        if self as *const _ != other as *const _ {
            self.set(other);
        }
        self.sign = -self.sign;
    }

    fn sub_impl(&mut self, other: &Self, n: SizeT) {
        let mut carry = 0u64;
        let mut tcarry;

        for i in 0..n {
            tcarry = if carry != 0 {
                (self.chars[i] <= other.chars[i]) as u64
            } else {
                (self.chars[i] < other.chars[i]) as u64
            };

            self.chars[i] = self.chars[i].wrapping_sub(other.chars[i]);
            self.chars[i] = self.chars[i].wrapping_sub(carry);
            carry = tcarry;
        }

        if carry != 0 {
            let mut i = n;
            while self.chars[i] == 0 {
                self.chars[i] = u64::MAX;
                i += 1;
            }

            if self.chars[i] == 1 {
                self.used -= 1;
            } else {
                self.chars[i] -= 1;
            }
        }
    }

    pub fn sub_unsigned(&mut self, b: &Self, c: &Self) {
        if b.is_zero() {
            self.abs(c);
            self.neg(self);
            return;
        } else if c.is_zero() {
            self.abs(b);
            return;
        }

        let mag_cmp = b.cmp_mag(c);
        match mag_cmp {
            Ordering::Less | Ordering::Equal => {
                if mag_cmp == Ordering::Equal {
                    self.zero();
                    return;
                }

                let n = b.used;
                if self as *const _ == b as *const _ {
                    let tmp = b.clone();
                    if self as *const _ != c as *const _ {
                        self.set(c);
                    }
                    self.sub_impl(&tmp, n);
                } else {
                    if self as *const _ != c as *const _ {
                        self.set(c);
                    }
                    self.sub_impl(b, n);
                }
                self.sign = mag_cmp as i32;
            }
            Ordering::Greater => {
                let n = c.used;
                if self as *const _ == c as *const _ {
                    let tmp = c.clone();
                    if self as *const _ != b as *const _ {
                        self.set(b);
                    }
                    self.sub_impl(&tmp, n);
                } else {
                    if self as *const _ != b as *const _ {
                        self.set(b);
                    }
                    self.sub_impl(c, n);
                }
                self.sign = mag_cmp as i32;
            }
        }
    }

    pub fn sub_nonnegative_assign(&mut self, b: &Self) {
        if b.is_zero() {
            self.abs(self);
        } else if self.cmp_mag(b) == Ordering::Equal {
            self.zero();
        } else {
            self.sub_impl(b, b.used);
        }
    }

    pub fn sub_positive_assign(&mut self, b: &Self) {
        self.sub_impl(b, b.used);
    }

    pub fn sub(&mut self, b: &Self, c: &Self) {
        if b.is_zero() {
            self.neg(c);
        } else if c.is_zero() {
            if self as *const _ != b as *const _ {
                self.set(b);
            }
        } else if b.signum() < 0 {
            if c.signum() < 0 {
                self.sub_unsigned(c, b);
            } else {
                // Assuming zadd_unsigned is implemented elsewhere
                // zadd_unsigned(self, b, c);
                self.sign = -self.signum();
            }
        } else if c.signum() < 0 {
            // zadd_unsigned(self, b, c);
        } else {
            self.sub_unsigned(b, c);
        }
    }
}

// Helper functions for memory operations would need to be implemented separately
// in safe Rust, likely using slice operations instead of raw pointers.