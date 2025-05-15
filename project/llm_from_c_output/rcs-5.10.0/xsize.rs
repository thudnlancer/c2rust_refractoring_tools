//! Checked size_t computations.
//!
//! This module provides utilities for performing size calculations with overflow checks.
//! The convention is that `usize::MAX` represents overflow.

use std::usize;

/// Convert an arbitrary value >= 0 to type usize.
/// Returns usize::MAX if the value exceeds usize::MAX.
pub const fn xcast_size_t(n: usize) -> usize {
    if n <= usize::MAX {
        n
    } else {
        usize::MAX
    }
}

/// Sum of two sizes, with overflow check.
/// Returns usize::MAX on overflow.
pub const fn xsum(size1: usize, size2: usize) -> usize {
    match size1.checked_add(size2) {
        Some(sum) => sum,
        None => usize::MAX,
    }
}

/// Sum of three sizes, with overflow check.
/// Returns usize::MAX on overflow.
pub const fn xsum3(size1: usize, size2: usize, size3: usize) -> usize {
    xsum(xsum(size1, size2), size3)
}

/// Sum of four sizes, with overflow check.
/// Returns usize::MAX on overflow.
pub const fn xsum4(size1: usize, size2: usize, size3: usize, size4: usize) -> usize {
    xsum(xsum(xsum(size1, size2), size3), size4)
}

/// Maximum of two sizes.
/// No overflow check needed since max(usize::MAX, n) == usize::MAX.
pub const fn xmax(size1: usize, size2: usize) -> usize {
    if size1 >= size2 {
        size1
    } else {
        size2
    }
}

/// Multiplication of a count with an element size, with overflow check.
/// The count must be >= 0 and the element size must be > 0.
/// Returns usize::MAX on overflow.
pub const fn xtimes(n: usize, elsize: usize) -> usize {
    match n.checked_mul(elsize) {
        Some(product) => product,
        None => usize::MAX,
    }
}

/// Check for overflow.
pub const fn size_overflow_p(size: usize) -> bool {
    size == usize::MAX
}

/// Check against overflow.
pub const fn size_in_bounds_p(size: usize) -> bool {
    size != usize::MAX
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xsum() {
        assert_eq!(xsum(1, 2), 3);
        assert_eq!(xsum(usize::MAX, 1), usize::MAX);
    }

    #[test]
    fn test_xsum3() {
        assert_eq!(xsum3(1, 2, 3), 6);
        assert_eq!(xsum3(usize::MAX, 1, 1), usize::MAX);
    }

    #[test]
    fn test_xsum4() {
        assert_eq!(xsum4(1, 2, 3, 4), 10);
        assert_eq!(xsum4(usize::MAX, 1, 1, 1), usize::MAX);
    }

    #[test]
    fn test_xmax() {
        assert_eq!(xmax(1, 2), 2);
        assert_eq!(xmax(usize::MAX, 1), usize::MAX);
    }

    #[test]
    fn test_xtimes() {
        assert_eq!(xtimes(2, 3), 6);
        assert_eq!(xtimes(usize::MAX, 2), usize::MAX);
    }

    #[test]
    fn test_overflow_checks() {
        assert!(size_overflow_p(usize::MAX));
        assert!(!size_overflow_p(1));
        assert!(size_in_bounds_p(1));
        assert!(!size_in_bounds_p(usize::MAX));
    }
}