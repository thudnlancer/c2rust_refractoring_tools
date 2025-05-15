/* See LICENSE file for copyright and license details. */

use std::cmp::Ordering;
use std::mem;

#[derive(Debug, Clone, PartialEq, Eq)]
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

    fn ensure_size(&mut self, size: usize) {
        if self.chars.len() < size {
            self.chars.resize(size, 0);
        }
    }

    fn zmemmoveb(&mut self, dest: usize, src: usize, len: usize) {
        if dest == src || len == 0 {
            return;
        }
        if dest < src {
            for i in 0..len {
                self.chars[dest + i] = self.chars[src + i];
            }
        } else {
            for i in (0..len).rev() {
                self.chars[dest + i] = self.chars[src + i];
            }
        }
    }
}

const BITS_PER_CHAR: usize = 8;

fn floor_bits_to_chars(bits: usize) -> usize {
    bits / BITS_PER_CHAR
}

fn bits_in_last_char(bits: usize) -> usize {
    bits % BITS_PER_CHAR
}

pub fn zlsh(a: &mut Z, b: &Z, bits: usize) {
    if b.is_zero() {
        a.signum = 0;
        return;
    }

    let chars = floor_bits_to_chars(bits);
    let bits = bits_in_last_char(bits);
    let cbits = BITS_PER_CHAR - bits;

    a.ensure_size(b.used + chars + 1);
    if a as *const _ == b as *const _ {
        a.zmemmoveb(chars, 0, b.used);
    } else {
        a.chars[chars..chars + b.used].copy_from_slice(&b.chars[..b.used]);
    }
    a.chars[..chars].fill(0);
    a.used = b.used + chars;

    if bits != 0 {
        let mut carry: u8 = 0;
        for i in chars..a.used {
            let tcarry = a.chars[i] >> cbits;
            a.chars[i] <<= bits;
            a.chars[i] |= carry;
            carry = tcarry;
        }
        if carry != 0 {
            a.chars[a.used] = carry;
            a.used += 1;
        }
    }

    a.signum = b.signum();
}