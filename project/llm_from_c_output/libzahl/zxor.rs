use std::cmp::{min, max};

// Assuming the following types and functions are defined elsewhere in the Rust codebase:
// - z_t: A struct representing a big integer
// - zahl_char_t: The underlying character type for the big integer digits
// - zzero: Checks if a big integer is zero
// - zsignum: Gets the sign of a big integer (-1, 0, 1)
// - SET: Sets one big integer equal to another
// - ZMEM_2OP: Performs element-wise operation between two big integers
// - ZMEM_2OP_PRECISE: Similar to ZMEM_2OP but with precise bounds
// - zmemcpy_range: Copies a range of elements between big integers
// - ENSURE_SIZE: Ensures the destination big integer has enough capacity
// - TRIM_AND_SIGN: Trims leading zeros and sets the sign

pub fn zxor(a: &mut z_t, b: &z_t, c: &z_t) {
    if zzero(b) {
        SET(a, c);
        return;
    } else if zzero(c) {
        SET(a, b);
        return;
    }

    let bn = b.used;
    let bc = &b.chars;
    let cn = c.used;
    let cc = &c.chars;

    let n = min(bn, cn);
    let m = max(bn, cn);

    ENSURE_SIZE(a, m);

    if std::ptr::eq(a, b) {
        ZMEM_2OP_PRECISE(&mut a.chars, &a.chars, cc, n, |x, y| x ^ y);
        if a.used < cn {
            zmemcpy_range(&mut a.chars, cc, n, m);
        }
    } else if std::ptr::eq(a, c) {
        ZMEM_2OP_PRECISE(&mut a.chars, &a.chars, bc, n, |x, y| x ^ y);
        if a.used < bn {
            zmemcpy_range(&mut a.chars, bc, n, m);
        }
    } else if m == bn {
        ZMEM_2OP(&mut a.chars, cc, bc, n, |x, y| x ^ y);
        zmemcpy_range(&mut a.chars, bc, n, m);
    } else {
        ZMEM_2OP(&mut a.chars, bc, cc, n, |x, y| x ^ y);
        zmemcpy_range(&mut a.chars, cc, n, m);
    }

    a.used = m;
    let sign = 1 - 2 * ((zsignum(b) ^ zsignum(c)) < 0);
    TRIM_AND_SIGN(a, sign);
}