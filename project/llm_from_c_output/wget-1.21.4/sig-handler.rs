//! Convenience declarations when working with signal handling.
//!
//! This module provides a safe interface for working with signal handlers in Rust,
//! equivalent to the original C code functionality.

#![deny(unsafe_code)]

use std::os::raw::c_int;

/// Convenience type when working with signal handlers.
pub type SaHandlerT = extern "C" fn(c_int);

/// Wrapper around libc's sigaction structure
#[repr(C)]
pub struct SigAction {
    inner: libc::sigaction,
}

impl SigAction {
    /// Creates a new SigAction from a raw sigaction pointer.
    /// 
    /// # Safety
    /// The pointer must be valid and non-null.
    pub unsafe fn from_raw(ptr: *const libc::sigaction) -> Self {
        Self {
            inner: *ptr,
        }
    }

    /// Returns the handler of the signal as a SaHandlerT.
    /// 
    /// The resulting function can be compared to special values like SIG_IGN
    /// but it is not portable to call it.
    pub fn get_handler(&self) -> SaHandlerT {
        // POSIX says that special values like SIG_IGN can only occur when
        // action.sa_flags does not contain SA_SIGINFO. But in Linux 2.4,
        // for example, sa_sigaction and sa_handler are aliases and a signal
        // is ignored if sa_sigaction (after casting) equals SIG_IGN. In
        // this case, this implementation relies on the fact that the two
        // are aliases, and simply returns sa_handler.
        unsafe { std::mem::transmute(self.inner.sa_sigaction) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use libc::{SIG_DFL, SIG_IGN};

    extern "C" fn test_handler(_: c_int) {}

    #[test]
    fn test_get_handler() {
        let mut action = libc::sigaction {
            sa_sigaction: test_handler as usize,
            sa_mask: unsafe { std::mem::zeroed() },
            sa_flags: 0,
            sa_restorer: None,
        };

        let sa = unsafe { SigAction::from_raw(&action) };
        let handler = sa.get_handler();
        assert_eq!(handler as usize, test_handler as usize);

        action.sa_sigaction = SIG_DFL as usize;
        let sa = unsafe { SigAction::from_raw(&action) };
        assert_eq!(sa.get_handler() as usize, SIG_DFL as usize);

        action.sa_sigaction = SIG_IGN as usize;
        let sa = unsafe { SigAction::from_raw(&action) };
        assert_eq!(sa.get_handler() as usize, SIG_IGN as usize);
    }
}