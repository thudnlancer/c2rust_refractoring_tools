/* See LICENSE file for copyright and license details. */
use std::cmp::min;

#[derive(Debug)]
struct Z {
    chars: Vec<u8>,
    sign: i8,
    used: usize,
}

fn zzero(b: &Z) -> bool {
    b.used == 0
}

fn ceiling_bits_to_chars(bits: usize) -> usize {
    (bits + 7) / 8
}

fn bits_in_last_char(bits: usize) -> usize {
    bits % 8
}

fn ztrunc(a: &mut Z, b: &Z, bits: usize) {
    if zzero(b) {
        a.sign = 0;
        a.used = 0;
        a.chars.clear();
        return;
    }

    let mut chars = ceiling_bits_to_chars(bits);
    a.used = min(chars, b.used);
    let mut remaining_bits = if a.used < chars { 0 } else { bits };
    
    if a as *const _ != b as *const _ {
        a.sign = b.sign;
        a.chars.resize(a.used, 0);
        a.chars[..a.used].copy_from_slice(&b.chars[..a.used]);
    }
    
    remaining_bits = bits_in_last_char(remaining_bits);
    if remaining_bits != 0 && a.used > 0 {
        let mask = (1 << remaining_bits) - 1;
        a.chars[a.used - 1] &= mask;
    }

    // Trim leading zeros
    while a.used > 0 && a.chars[a.used - 1] == 0 {
        a.used -= 1;
    }
    a.chars.truncate(a.used);
}