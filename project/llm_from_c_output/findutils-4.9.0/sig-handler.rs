//! Convenience declarations when working with signal handling.
//!
//! This module provides a safe interface for working with signal handlers in Rust,
//! equivalent to the original C code functionality.

use std::os::raw::c_int;
use libc::{sigaction, SIG_IGN};

/// Convenience type when working with signal handlers.
pub type SaHandlerT = extern "C" fn(c_int);

/// Returns the handler of a signal from a sigaction structure.
///
/// The resulting function can be compared to special values like SIG_IGN,
/// but it is not portable to call it directly.
///
/// # Arguments
/// * `a` - Reference to a sigaction structure
///
/// # Safety
/// This function is safe as long as the sigaction pointer is valid and properly initialized.
/// The caller must ensure the sigaction structure remains valid for the duration of the reference.
pub fn get_handler(a: &sigaction) -> SaHandlerT {
    // POSIX says that special values like SIG_IGN can only occur when
    // action.sa_flags does not contain SA_SIGINFO. But in Linux 2.4,
    // for example, sa_sigaction and sa_handler are aliases and a signal
    // is ignored if sa_sigaction (after casting) equals SIG_IGN. In
    // this case, this implementation relies on the fact that the two
    // are aliases, and simply returns sa_handler.
    unsafe {
        std::mem::transmute(a.sa_handler)
    }
}