//! uint64_t-like operations that work even on hosts lacking uint64_t
//!
//! Original C code Copyright (C) 2006, 2009-2023 Free Software Foundation, Inc.
//! Translated to Rust with equivalent functionality and safety guarantees.

#![allow(non_camel_case_types)]

use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct u64 {
    hi: u32,
    lo: u32,
}

impl u64 {
    /// Create a u64 from high and low 32-bit parts (like u64hilo in C)
    pub fn from_hilo(hi: u32, lo: u32) -> Self {
        Self { hi, lo }
    }

    /// Create a u64 from a low 32-bit value (like u64lo in C)
    pub fn from_lo(lo: u32) -> Self {
        Self { hi: 0, lo }
    }

    /// Create a u64 from a size value (like u64size in C)
    pub fn from_size(size: usize) -> Self {
        Self {
            hi: (size >> 32) as u32,
            lo: size as u32,
        }
    }

    /// Bitwise AND (like u64and in C)
    pub fn and(self, other: Self) -> Self {
        Self {
            hi: self.hi & other.hi,
            lo: self.lo & other.lo,
        }
    }

    /// Bitwise OR (like u64or in C)
    pub fn or(self, other: Self) -> Self {
        Self {
            hi: self.hi | other.hi,
            lo: self.lo | other.lo,
        }
    }

    /// Bitwise XOR (like u64xor in C)
    pub fn xor(self, other: Self) -> Self {
        Self {
            hi: self.hi ^ other.hi,
            lo: self.lo ^ other.lo,
        }
    }

    /// Addition with carry (like u64plus in C)
    pub fn add(self, other: Self) -> Self {
        let lo = self.lo.wrapping_add(other.lo);
        let carry = if lo < self.lo { 1 } else { 0 };
        let hi = self.hi.wrapping_add(other.hi).wrapping_add(carry);
        Self { hi, lo }
    }

    /// Left shift (like u64shl in C)
    pub fn shl(self, n: u32) -> Self {
        if n < 32 {
            Self {
                hi: (self.hi << n) | (self.lo >> (32 - n)),
                lo: self.lo << n,
            }
        } else if n < 64 {
            Self {
                hi: self.lo << (n - 32),
                lo: 0,
            }
        } else {
            Self { hi: 0, lo: 0 }
        }
    }

    /// Right shift (like u64shr in C)
    pub fn shr(self, n: u32) -> Self {
        if n < 32 {
            Self {
                hi: self.hi >> n,
                lo: (self.hi << (32 - n)) | (self.lo >> n),
            }
        } else if n < 64 {
            Self {
                hi: 0,
                lo: self.hi >> (n - 32),
            }
        } else {
            Self { hi: 0, lo: 0 }
        }
    }

    /// Rotate left (like u64rol in C)
    pub fn rotate_left(self, n: u32) -> Self {
        self.shl(n).or(self.shr(64 - n))
    }
}

impl PartialOrd for u64 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.hi.cmp(&other.hi) {
            Ordering::Equal => Some(self.lo.cmp(&other.lo)),
            ord => Some(ord),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u64_operations() {
        let a = u64::from_hilo(0x12345678, 0x9ABCDEF0);
        let b = u64::from_hilo(0xFEDCBA98, 0x76543210);

        assert_eq!(a.and(b), u64::from_hilo(0x12341218, 0x12441210));
        assert_eq!(a.or(b), u64::from_hilo(0xFEDEFEF8, 0xFEFDFEF0));
        assert_eq!(a.xor(b), u64::from_hilo(0xECEAECE0, 0xECB9ECE0));
        assert_eq!(a.add(b), u64::from_hilo(0x11111110, 0x11111100));
        assert_eq!(a.shl(4), u64::from_hilo(0x23456789, 0xABCDEF00));
        assert_eq!(a.shr(4), u64::from_hilo(0x01234567, 0x89ABCDEF));
        assert_eq!(a.rotate_left(4), u64::from_hilo(0x23456789, 0xABCDEF01));
    }
}