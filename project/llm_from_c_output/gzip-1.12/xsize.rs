//! Checked size_t computations.
//!
//! This module provides utilities for performing checked size computations
//! to prevent overflow when working with memory sizes.

use std::usize::MAX as SIZE_MAX;

/// Convert an arbitrary value >= 0 to type usize.
/// Returns SIZE_MAX if the value exceeds usize::MAX.
pub const fn xcast_size_t(n: usize) -> usize {
    if n <= SIZE_MAX { n } else { SIZE_MAX }
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
/// No overflow check needed since max(SIZE_MAX, n) == SIZE_MAX.
pub const fn xmax(size1: usize, size2: usize) -> usize {
    if size1 >= size2 { size1 } else { size2 }
}

/// Multiplication of a count with an element size, with overflow check.
/// The count must be >= 0 and the element size must be > 0.
/// Returns SIZE_MAX on overflow.
pub const fn xtimes(n: usize, elsize: usize) -> usize {
    if elsize == 0 {
        panic!("Element size must be greater than 0");
    }
    match n.checked_mul(elsize) {
        Some(product) => product,
        None => SIZE_MAX,
    }
}

/// Check for overflow (SIZE == SIZE_MAX).
pub const fn size_overflow_p(size: usize) -> bool {
    size == SIZE_MAX
}

/// Check against overflow (SIZE != SIZE_MAX).
pub const fn size_in_bounds_p(size: usize) -> bool {
    size != SIZE_MAX
}