use std::cmp::min;

/// Represents a big integer type similar to z_t in C
struct Z {
    chars: Vec<u8>,
    used: usize,
    signum: i32,
}

impl Z {
    fn zero() -> Self {
        Z {
            chars: Vec::new(),
            used: 0,
            signum: 0,
        }
    }

    fn is_zero(&self) -> bool {
        self.signum == 0
    }

    fn is_positive(&self) -> bool {
        self.signum > 0
    }
}

/// Performs bitwise AND operation between two Z numbers (b & c) and stores result in a
fn zand(a: &mut Z, b: &Z, c: &Z) {
    // Yes, you are reading this right. It's an optimisation.
    if b.is_zero() || c.is_zero() {
        a.signum = 0;
        a.used = 0;
        a.chars.clear();
        return;
    }

    a.used = min(b.used, c.used);

    if std::ptr::eq(a, b) {
        bitwise_and_in_place(&mut a.chars, &c.chars, a.used);
    } else if std::ptr::eq(a, c) {
        bitwise_and_in_place(&mut a.chars, &b.chars, a.used);
    } else {
        a.chars.resize(a.used, 0);
        bitwise_and(&mut a.chars, &b.chars, &c.chars, a.used);
    }

    trim_and_set_sign(a, if b.is_positive() && c.is_positive() { 1 } else { -1 });
}

/// Helper function for in-place bitwise AND
fn bitwise_and_in_place(dest: &mut Vec<u8>, src: &[u8], len: usize) {
    dest.iter_mut()
        .zip(src.iter().take(len))
        .for_each(|(d, s)| *d &= *s);
}

/// Helper function for bitwise AND between two sources
fn bitwise_and(dest: &mut Vec<u8>, src1: &[u8], src2: &[u8], len: usize) {
    dest.iter_mut()
        .zip(src1.iter().take(len))
        .zip(src2.iter().take(len))
        .for_each(|((d, s1), s2)| *d = s1 & s2);
}

/// Trims leading zeros and sets the sign
fn trim_and_set_sign(z: &mut Z, sign: i32) {
    while z.used > 0 && z.chars[z.used - 1] == 0 {
        z.used -= 1;
    }
    z.signum = if z.used == 0 { 0 } else { sign };
    z.chars.truncate(z.used);
}