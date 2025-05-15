use std::cmp::{min, max};

/// Performs a bitwise OR operation between two numbers and stores the result in `a`.
/// Equivalent to the C function `zor`.
pub fn zor(a: &mut Z, b: &Z, c: &Z) {
    if unlikely(b.is_zero()) {
        a.set(c);
        return;
    } else if unlikely(c.is_zero()) {
        a.set(b);
        return;
    }

    let n = min(b.used(), c.used());
    let m = max(b.used(), c.used());
    a.ensure_size(m);

    if a.as_ptr() == b.as_ptr() {
        unsafe {
            zmem_2op_precise(a.chars_mut(), a.chars(), c.chars(), n, |x, y| x | y);
        }
        if a.used() < c.used() {
            zmemcpy_range(a.chars_mut(), c.chars(), n, m);
        }
    } else if unlikely(a.as_ptr() == c.as_ptr()) {
        unsafe {
            zmem_2op_precise(a.chars_mut(), a.chars(), b.chars(), n, |x, y| x | y);
        }
        if a.used() < b.used() {
            zmemcpy_range(a.chars_mut(), b.chars(), n, m);
        }
    } else if m == b.used() {
        unsafe {
            zmem_2op(a.chars_mut(), c.chars(), b.chars(), n, |x, y| x | y);
        }
        zmemcpy_range(a.chars_mut(), b.chars(), n, m);
    } else {
        unsafe {
            zmem_2op(a.chars_mut(), b.chars(), c.chars(), n, |x, y| x | y);
        }
        zmemcpy_range(a.chars_mut(), c.chars(), n, m);
    }

    a.set_used(m);
    a.set_signum(zpositive2(b, c) * 2 - 1);
}

// Helper functions and types would need to be defined elsewhere in the codebase:
// - Z type with methods: is_zero, set, used, ensure_size, chars_mut, chars, set_used, set_signum
// - Utility functions: unlikely, zmem_2op_precise, zmemcpy_range, zmem_2op, zpositive2
// - Proper memory safety would need to be ensured in the unsafe blocks