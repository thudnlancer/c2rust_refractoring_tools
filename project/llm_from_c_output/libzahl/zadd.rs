use std::cmp::max;

#[derive(Debug, Clone, PartialEq)]
struct Z {
    chars: Vec<u64>,
    used: usize,
    signum: i8,
}

impl Z {
    fn zero() -> Self {
        Z {
            chars: vec![0],
            used: 0,
            signum: 0,
        }
    }

    fn is_zero(&self) -> bool {
        self.signum == 0
    }

    fn is_negative(&self) -> bool {
        self.signum < 0
    }

    fn abs(&mut self, other: &Z) {
        self.chars = other.chars.clone();
        self.used = other.used;
        self.signum = if other.is_zero() { 0 } else { 1 };
    }

    fn set(&mut self, other: &Z) {
        self.chars = other.chars.clone();
        self.used = other.used;
        self.signum = other.signum;
    }

    fn ensure_size(&mut self, size: usize) {
        if self.chars.len() < size {
            self.chars.resize(size, 0);
        }
    }

    fn add_overflow(a: &mut u64, b: u64, c: u64) -> u64 {
        let (sum1, overflow1) = b.overflowing_add(c);
        let (sum2, overflow2) = (*a).overflowing_add(sum1);
        *a = sum2;
        overflow1 as u64 | overflow2 as u64
    }

    fn zadd_impl_4(a: &mut Z, b: &Z, c: &Z, n: usize) {
        let mut carry = 0;
        let mut i = 0;

        while i < n {
            let tcarry = Self::add_overflow(&mut a.chars[i], b.chars[i], c.chars[i]);
            carry = tcarry | Self::add_overflow(&mut a.chars[i], a.chars[i], carry);
            i += 1;
        }

        while carry != 0 {
            carry = Self::add_overflow(&mut a.chars[i], a.chars[i], 1);
            i += 1;
        }

        if a.used < i {
            a.used = i;
        }
    }

    fn zadd_impl_3(a: &mut Z, b: &Z, n: usize) {
        Self::zadd_impl_4(a, a, b, n);
    }

    fn zadd_unsigned(a: &mut Z, b: &Z, c: &Z) {
        if b.is_zero() {
            a.abs(c);
            return;
        } else if c.is_zero() {
            a.abs(b);
            return;
        }

        let size = max(b.used, c.used);
        let mut n = b.used + c.used - size;

        a.ensure_size(size + 1);
        a.chars[size] = 0;

        if a == b {
            if a.used < c.used {
                n = c.used;
                a.chars[a.used..n].fill(0);
            }
            Self::zadd_impl_3(a, c, n);
        } else if a == c {
            if a.used < b.used {
                n = b.used;
                a.chars[a.used..n].fill(0);
            }
            Self::zadd_impl_3(a, b, n);
        } else if b.used > c.used {
            a.chars[n..size].copy_from_slice(&b.chars[n..size]);
            a.used = size;
            Self::zadd_impl_4(a, b, c, n);
        } else {
            a.chars[n..size].copy_from_slice(&c.chars[n..size]);
            a.used = size;
            Self::zadd_impl_4(a, b, c, n);
        }

        a.signum = 1;
    }

    fn zadd_unsigned_assign(a: &mut Z, b: &Z) {
        if a.is_zero() {
            a.abs(b);
            return;
        } else if b.is_zero() {
            return;
        }

        let size = max(a.used, b.used);
        let mut n = a.used + b.used - size;

        a.ensure_size(size + 1);
        a.chars[size] = 0;

        if a.used < b.used {
            n = b.used;
            a.chars[a.used..n].fill(0);
        }
        Self::zadd_impl_3(a, b, n);

        a.signum = 1;
    }

    fn zadd(a: &mut Z, b: &Z, c: &Z) {
        if b.is_zero() {
            a.set(c);
        } else if c.is_zero() {
            a.set(b);
        } else if b.is_negative() {
            if c.is_negative() {
                Self::zadd_unsigned(a, b, c);
                a.signum = -a.signum;
            } else {
                // zsub_unsigned would need to be implemented
                unimplemented!("zsub_unsigned not implemented");
            }
        } else if c.is_negative() {
            // zsub_unsigned would need to be implemented
            unimplemented!("zsub_unsigned not implemented");
        } else {
            Self::zadd_unsigned(a, b, c);
        }
    }
}