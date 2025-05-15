use std::cmp::Ordering;
use std::mem::size_of;
use std::ptr;

#[derive(Debug, Clone)]
pub struct Zahl {
    pub sign: i32,
    pub used: usize,
    pub alloced: usize,
    pub chars: Vec<u64>,
}

impl Zahl {
    pub fn zero() -> Self {
        Zahl {
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

    pub fn set(&mut self, other: &Zahl) {
        if other.is_zero() {
            self.sign = 0;
            self.used = 0;
            self.chars.clear();
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

    pub fn cmp_mag(&self, other: &Zahl) -> Ordering {
        if self.is_zero() {
            return if other.is_zero() { Ordering::Equal } else { Ordering::Less };
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

    pub fn lsb(&self) -> usize {
        if self.is_zero() {
            return usize::MAX;
        }

        let mut i = 0;
        while i < self.used && self.chars[i] == 0 {
            i += 1;
        }

        if i >= self.used {
            return usize::MAX;
        }

        i * 64 + self.chars[i].trailing_zeros() as usize
    }

    pub fn bits(&self) -> usize {
        if self.is_zero() {
            return 1;
        }

        let mut used = self.used;
        while used > 0 && self.chars[used - 1] == 0 {
            used -= 1;
        }

        if used == 0 {
            return 1;
        }

        used * 64 - self.chars[used - 1].leading_zeros() as usize
    }

    pub fn rsh_taint(&mut self, bits: usize) {
        if bits == 0 || self.is_zero() {
            return;
        }

        let chars = bits / 64;
        if chars >= self.used || self.bits() <= bits {
            self.sign = 0;
            self.used = 0;
            return;
        }

        let bits = bits % 64;
        let cbits = 64 - bits;

        if chars > 0 {
            self.used -= chars;
            self.chars.drain(0..chars);
        }

        if bits != 0 {
            self.chars[0] >>= bits;
            for i in 1..self.used {
                self.chars[i - 1] |= self.chars[i] << cbits;
                self.chars[i] >>= bits;
            }

            while self.used > 0 && self.chars[self.used - 1] == 0 {
                self.used -= 1;
            }
        }
    }
}

pub fn zswap_tainted_unsigned(a: &mut Zahl, b: &mut Zahl) {
    std::mem::swap(&mut a.used, &mut b.used);
    std::mem::swap(&mut a.chars, &mut b.chars);
}

pub fn zgcd(a: &mut Zahl, b: &Zahl, c: &Zahl) {
    if b.is_zero() {
        a.set(c);
        return;
    }
    if c.is_zero() {
        a.set(b);
        return;
    }

    let neg = (b.signum() & c.signum()) < 0;
    let u_lsb = b.lsb();
    let v_lsb = c.lsb();
    let shifts = u_lsb.min(v_lsb);

    let mut u = Zahl::zero();
    let mut v = Zahl::zero();
    
    u.set(b);
    v.set(c);
    
    u.rsh_taint(u_lsb);
    v.rsh_taint(v_lsb);

    loop {
        match u.cmp_mag(&v) {
            Ordering::Less => {
                zswap_tainted_unsigned(&mut u, &mut v);
            }
            Ordering::Equal => break,
            Ordering::Greater => (),
        }

        // Simulate zsub_positive_assign
        let cmp = u.cmp_mag(&v);
        if cmp == Ordering::Less {
            zswap_tainted_unsigned(&mut u, &mut v);
        }
        
        // Subtract smaller from larger
        for i in 0..v.used {
            if i < u.used {
                u.chars[i] = u.chars[i].wrapping_sub(v.chars[i]);
            }
        }
        
        // Handle borrow
        let mut borrow = false;
        for i in 0..u.used {
            if borrow {
                if u.chars[i] == 0 {
                    u.chars[i] = u64::MAX;
                } else {
                    u.chars[i] -= 1;
                    borrow = false;
                }
            }
            if i < v.used && u.chars[i] < v.chars[i] {
                borrow = true;
            }
        }

        v.rsh_taint(v.lsb());
    }

    // Left shift
    let chars = shifts / 64;
    let bits = shifts % 64;
    let cbits = 64 - bits;

    u.chars.resize(u.used + chars + 1, 0);
    u.used += chars + 1;

    for i in (0..u.used - chars).rev() {
        u.chars[i + chars] = u.chars[i];
    }
    for i in 0..chars {
        u.chars[i] = 0;
    }

    if bits != 0 {
        for i in (1..u.used).rev() {
            u.chars[i] = (u.chars[i] << bits) | (u.chars[i - 1] >> cbits);
        }
        u.chars[0] <<= bits;
    }

    while u.used > 0 && u.chars[u.used - 1] == 0 {
        u.used -= 1;
    }

    u.sign = if neg { -1 } else { 1 };
    a.set(&u);
}