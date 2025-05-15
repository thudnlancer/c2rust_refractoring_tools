/* See LICENSE file for copyright and license details. */
use crate::internals::*;
use crate::zahl::*;

pub fn znot(a: &mut Z, b: &Z) {
    let mut bits;

    if unlikely(b.is_zero()) {
        a.set_signum(0);
        return;
    }

    bits = zbits(b);
    a.used = b.used;
    a.set_signum(-b.signum());

    for i in 0..a.used {
        a.chars[i] = !b.chars[i];
    }
    
    bits = bits_in_last_char(bits);
    if bits != 0 {
        a.chars[a.used - 1] &= ((1 as zahl_char_t) << bits) - 1;
    }

    trim_and_zero(a);
}