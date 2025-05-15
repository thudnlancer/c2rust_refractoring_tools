//! Checked size_t computations.
//!
//! This module provides utilities for performing checked size computations
//! to prevent overflow when working with memory sizes.

use std::usize::MAX as SIZE_MAX;

/// Convert an arbitrary value >= 0 to type usize.
/// Returns SIZE_MAX if the value exceeds usize::MAX.
pub const fn xcast_size_t(n: usize) -> usize {
    if n <= SIZE_MAX {
        n
    } else {
        SIZE_MAX
    }
}

/// Sum of two sizes, with overflow check.
/// Returns SIZE_MAX on overflow.
pub const fn xsum(size1: usize, size2: usize) -> usize {
    match size1.checked_add(size2) {
        Some(sum) => sum,
        None => SIZE_MAX,
    }
}

/// Sum of three sizes, with overflow check.
/// Returns SIZE_MAX on overflow.
pub const fn xsum3(size1: usize, size2: usize, size3: usize) -> usize {
    xsum(xsum(size1, size2), size3)
}

/// Sum of four sizes, with overflow check.
/// Returns SIZE_MAX on overflow.
pub const fn xsum4(size1: usize, size2: usize, size3: usize, size4: usize) -> usize {
    xsum(xsum(xsum(size1, size2), size3), size4)
}

/// Maximum of two sizes.
/// Since usize is a well-ordered type, no overflow check is needed.
pub const fn xmax(size1: usize, size2: usize) -> usize {
    if size1 >= size2 {
        size1
    } else {
        size2
    }
}

/// Multiplication of a count with an element size, with overflow check.
/// The count must be >= 0 and the element size must be > 0.
/// Returns SIZE_MAX on overflow.
pub const fn xtimes(n: usize, elsize: usize) -> usize {
    if n == 0 || elsize == 0 {
        return 0;
    }
    if n <= SIZE_MAX / elsize {
        n * elsize
    } else {
        SIZE_MAX
    }
}

/// Check for overflow.
pub const fn size_overflow_p(size: usize) -> bool {
    size == SIZE_MAX
}

/// Check against overflow.
pub const fn size_in_bounds_p(size: usize) -> bool {
    size != SIZE_MAX
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xsum() {
        assert_eq!(xsum(1, 2), 3);
        assert_eq!(xsum(SIZE_MAX, 1), SIZE_MAX);
    }

    #[test]
    fn test_xsum3() {
        assert_eq!(xsum3(1, 2, 3), 6);
        assert_eq!(xsum3(SIZE_MAX, 1, 1), SIZE_MAX);
    }

    #[test]
    fn test_xsum4() {
        assert_eq!(xsum4(1, 2, 3, 4), 10);
        assert_eq!(xsum4(SIZE_MAX, 1, 1, 1), SIZE_MAX);
    }

    #[test]
    fn test_xmax() {
        assert_eq!(xmax(1, 2), 2);
        assert_eq!(xmax(SIZE_MAX, 1), SIZE_MAX);
    }

    #[test]
    fn test_xtimes() {
        assert_eq!(xtimes(2, 3), 6);
        assert_eq!(xtimes(SIZE_MAX, 2), SIZE_MAX);
        assert_eq!(xtimes(0, SIZE_MAX), 0);
    }

    #[test]
    fn test_overflow_checks() {
        assert!(size_overflow_p(SIZE_MAX));
        assert!(!size_overflow_p(100));
        assert!(size_in_bounds_p(100));
        assert!(!size_in_bounds_p(SIZE_MAX));
    }
}