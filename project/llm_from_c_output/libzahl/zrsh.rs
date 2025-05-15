use std::cmp;

pub struct Z {
    chars: Vec<u8>,
    used: usize,
    signum: i8,
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

    fn signum(&self) -> i8 {
        self.signum
    }

    fn bits(&self) -> usize {
        if self.is_zero() {
            return 0;
        }
        let leading_zeros = self.chars[self.used - 1].leading_zeros() as usize;
        8 * self.used - leading_zeros
    }

    fn trim_nonzero(&mut self) {
        while self.used > 0 && self.chars[self.used - 1] == 0 {
            self.used -= 1;
        }
        if self.used == 0 {
            self.signum = 0;
        }
    }

    fn ensure_size(&mut self, size: usize) {
        if self.chars.len() < size {
            self.chars.resize(size, 0);
        }
    }
}

fn floor_bits_to_chars(bits: usize) -> usize {
    bits / 8
}

fn bits_in_last_char(bits: usize) -> usize {
    bits % 8
}

pub fn zrsh(a: &mut Z, b: &Z, bits: usize) {
    if bits == 0 {
        *a = Z {
            chars: b.chars.clone(),
            used: b.used,
            signum: b.signum,
        };
        return;
    }

    let chars = floor_bits_to_chars(bits);

    if b.is_zero() || chars >= b.used || b.bits() <= bits {
        *a = Z::zero();
        return;
    }

    let bits = bits_in_last_char(bits);
    let cbits = 8 - bits;

    if chars > 0 && a as *const _ == b as *const _ {
        a.used -= chars;
        a.chars.copy_within(chars..chars + a.used, 0);
    } else if a as *const _ != b as *const _ {
        a.used = b.used - chars;
        a.ensure_size(a.used);
        a.chars[..a.used].copy_from_slice(&b.chars[chars..chars + a.used]);
    }

    if bits != 0 {
        a.chars[0] >>= bits;
        for i in 1..a.used {
            a.chars[i - 1] |= a.chars[i] << cbits;
            a.chars[i] >>= bits;
        }
        a.trim_nonzero();
    }

    a.signum = b.signum();
}