use std::mem;
use std::ptr;

#[derive(Debug, Clone, Copy)]
pub enum ZError {
    ErrnoSet,
    ZeroPowZero,
    ZeroDivZero,
    DivZero,
    Negative,
    InvalidRadix,
}

#[derive(Debug, Clone)]
pub struct Z {
    sign: i32,
    used: usize,
    alloced: usize,
    chars: Vec<u64>,
}

impl Z {
    pub fn zero() -> Self {
        Z {
            sign: 0,
            used: 0,
            alloced: 0,
            chars: Vec::new(),
        }
    }

    pub fn is_zero(&self) -> bool {
        self.sign == 0
    }

    pub fn signum(&self) -> i32 {
        self.sign
    }

    pub fn is_odd(&self) -> bool {
        self.sign != 0 && (self.chars[0] & 1) != 0
    }

    pub fn bits(&self) -> usize {
        if self.is_zero() {
            return 1;
        }

        let mut used = self.used;
        while self.chars[used - 1] == 0 {
            used -= 1;
        }

        let leading_zeros = self.chars[used - 1].leading_zeros() as usize;
        used * 8 * mem::size_of::<u64>() - leading_zeros
    }

    pub fn set(&mut self, other: &Z) {
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

    pub fn set_u64(&mut self, value: u64) {
        if value == 0 {
            self.sign = 0;
            return;
        }
        if self.alloced < 1 {
            self.chars.resize(1, 0);
            self.alloced = 1;
        }
        self.sign = 1;
        self.chars[0] = value;
        self.used = 1;
    }

    pub fn abs(&mut self, other: &Z) {
        if ptr::eq(self, other) {
            self.sign &= 1;
        } else {
            self.set(other);
            self.sign &= 1;
        }
    }

    pub fn neg(&mut self, other: &Z) {
        if ptr::eq(self, other) {
            self.sign = -self.sign;
        } else {
            self.set(other);
            self.sign = -self.sign;
        }
    }
}

pub struct ZContext {
    tmp_pow_b: Z,
    tmp_pow_c: Z,
}

impl ZContext {
    pub fn new() -> Self {
        ZContext {
            tmp_pow_b: Z::zero(),
            tmp_pow_c: Z::zero(),
        }
    }

    pub fn pow(&mut self, a: &mut Z, b: &Z, c: &Z) -> Result<(), ZError> {
        if c.signum() <= 0 {
            if c.is_zero() {
                if b.is_zero() {
                    return Err(ZError::ZeroPowZero);
                }
                a.set_u64(1);
            } else if b.is_zero() {
                return Err(ZError::DivZero);
            } else {
                a.sign = 0;
            }
            return Ok(());
        } else if b.is_zero() {
            a.sign = 0;
            return Ok(());
        }

        let bits = c.bits();
        let n = bits >> 6;
        let neg = b.signum() < 0 && b.is_odd();

        self.tmp_pow_b.abs(b);
        self.tmp_pow_c.set(c);
        a.set_u64(1);

        for i in 0..n {
            let mut x = self.tmp_pow_c.chars[i];
            for _ in 0..64 {
                if x & 1 != 0 {
                    self.mul_ll(a, &self.tmp_pow_b)?;
                }
                self.sqr_ll(&mut self.tmp_pow_b)?;
                x >>= 1;
            }
        }

        let mut x = self.tmp_pow_c.chars[n];
        while x != 0 {
            if x & 1 != 0 {
                self.mul_ll(a, &self.tmp_pow_b)?;
            }
            self.sqr_ll(&mut self.tmp_pow_b)?;
            x >>= 1;
        }

        if neg {
            a.neg(a);
        }

        Ok(())
    }

    fn mul_ll(&self, _a: &mut Z, _b: &Z) -> Result<(), ZError> {
        // Implementation of zmul_ll would go here
        Ok(())
    }

    fn sqr_ll(&self, _a: &mut Z) -> Result<(), ZError> {
        // Implementation of zsqr_ll would go here
        Ok(())
    }
}