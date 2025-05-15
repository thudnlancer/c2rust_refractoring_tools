/* See LICENSE file for copyright and license details. */
use std::cmp::Ordering;

type ZahlChar = u64;
const ZAHL_CHAR_MAX: ZahlChar = ZahlChar::MAX;

#[derive(Debug, Clone, PartialEq)]
struct Z {
    chars: Vec<ZahlChar>,
    used: usize,
    signum: i8,
}

impl Z {
    fn zero() -> Self {
        Self {
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

    fn abs(&self) -> Self {
        let mut result = self.clone();
        result.signum = if self.is_zero() { 0 } else { 1 };
        result
    }

    fn neg(&mut self) {
        if !self.is_zero() {
            self.signum *= -1;
        }
    }

    fn cmp_mag(&self, other: &Self) -> Ordering {
        match self.used.cmp(&other.used) {
            Ordering::Equal => {
                for i in (0..self.used).rev() {
                    match self.chars[i].cmp(&other.chars[i]) {
                        Ordering::Equal => continue,
                        ord => return ord,
                    }
                }
                Ordering::Equal
            }
            ord => ord,
        }
    }
}

fn zsub_impl(a: &mut Z, b: &Z, n: usize) {
    let mut carry = false;
    let mut tcarry;

    for i in 0..n {
        tcarry = if carry {
            a.chars[i] <= b.chars[i]
        } else {
            a.chars[i] < b.chars[i]
        };
        a.chars[i] = a.chars[i].wrapping_sub(b.chars[i]);
        a.chars[i] = a.chars[i].wrapping_sub(carry as ZahlChar);
        carry = tcarry;
    }

    if carry {
        let mut i = n;
        while a.chars[i] == 0 {
            a.chars[i] = ZAHL_CHAR_MAX;
            i += 1;
        }
        if a.chars[i] == 1 {
            a.used -= 1;
        } else {
            a.chars[i] -= 1;
        }
    }
}

fn libzahl_zsub_unsigned(a: &mut Z, b: &Z, c: &Z) {
    if b.is_zero() {
        *a = c.abs();
        a.neg();
        return;
    } else if c.is_zero() {
        *a = b.abs();
        return;
    }

    let magcmp = b.cmp_mag(c);
    match magcmp {
        Ordering::Less | Ordering::Equal => {
            if magcmp == Ordering::Equal {
                a.signum = 0;
                return;
            }
            let n = b.used;
            if a == b {
                let tmp = b.clone();
                *a = c.clone();
                zsub_impl(a, &tmp, n);
            } else {
                *a = c.clone();
                zsub_impl(a, b, n);
            }
        }
        Ordering::Greater => {
            let n = c.used;
            if a == c {
                let tmp = c.clone();
                *a = b.clone();
                zsub_impl(a, &tmp, n);
            } else {
                *a = b.clone();
                zsub_impl(a, c, n);
            }
        }
    }

    a.signum = match magcmp {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    };
}

fn zsub_unsigned(a: &mut Z, b: &Z, c: &Z) {
    libzahl_zsub_unsigned(a, b, c);
}

fn zsub_nonnegative_assign(a: &mut Z, b: &Z) {
    if b.is_zero() {
        *a = a.abs();
    } else if a.cmp_mag(b) == Ordering::Equal {
        a.signum = 0;
    } else {
        zsub_impl(a, b, b.used);
    }
}

fn zsub_positive_assign(a: &mut Z, b: &Z) {
    zsub_impl(a, b, b.used);
}

fn zadd_unsigned(a: &mut Z, b: &Z, c: &Z) {
    // Implementation of unsigned addition would go here
    unimplemented!();
}

fn zsub(a: &mut Z, b: &Z, c: &Z) {
    if b.is_zero() {
        *a = c.clone();
        a.neg();
    } else if c.is_zero() {
        *a = b.clone();
    } else if b.is_negative() {
        if c.is_negative() {
            libzahl_zsub_unsigned(a, c, b);
        } else {
            zadd_unsigned(a, b, c);
            a.signum = -a.signum;
        }
    } else if c.is_negative() {
        zadd_unsigned(a, b, c);
    } else {
        libzahl_zsub_unsigned(a, b, c);
    }
}