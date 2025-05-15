/* See LICENSE file for copyright and license details. */

use std::num::NonZeroUsize;

#[derive(Debug)]
struct Z {
    chars: Vec<u8>,
    used: usize,
    signum: i32,
}

impl Z {
    fn zero() -> Self {
        Self {
            chars: Vec::new(),
            used: 0,
            signum: 0,
        }
    }

    fn is_zero(&self) -> bool {
        self.signum == 0
    }

    fn ensure_size(&mut self, size: usize) {
        if self.chars.len() < size {
            self.chars.resize(size, 0);
        }
    }

    fn trim_and_zero(&mut self) {
        while self.used > 0 && self.chars[self.used - 1] == 0 {
            self.used -= 1;
        }
        if self.used == 0 {
            self.signum = 0;
        }
    }
}

fn floor_bits_to_chars(bit: usize) -> usize {
    bit / 8
}

fn bits_in_last_char(bit: usize) -> usize {
    bit % 8
}

macro_rules! prologue {
    ($a:expr, $bit:expr, $may_increase:expr) => {{
        let mut mask = 1u8;
        let mut chars = floor_bits_to_chars($bit);
        if $may_increase {
            if $a.is_zero() {
                $a.used = 0;
                $a.signum = 1;
            }
            if chars >= $a.used {
                $a.ensure_size(chars + 1);
                for i in $a.used..=chars {
                    $a.chars[i] = 0;
                }
                $a.used = chars + 1;
            }
        } else if chars >= $a.used {
            return;
        }
        let bit_in_char = bits_in_last_char($bit);
        mask <<= bit_in_char;
        (chars, mask)
    }};
}

pub fn zbset_ll_set(a: &mut Z, bit: usize) {
    let (chars, mask) = prologue!(a, bit, true);
    a.chars[chars] |= mask;
}

pub fn zbset_ll_clear(a: &mut Z, bit: usize) {
    let (chars, mask) = prologue!(a, bit, false);
    a.chars[chars] &= !mask;
    a.trim_and_zero();
}

pub fn zbset_ll_flip(a: &mut Z, bit: usize) {
    let (chars, mask) = prologue!(a, bit, true);
    a.chars[chars] ^= mask;
    a.trim_and_zero();
}