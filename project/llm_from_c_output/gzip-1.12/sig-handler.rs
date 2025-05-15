//! Convenience declarations when working with signal handling.
//!
//! This module provides a safe Rust interface for working with signal handlers,
//! similar to the functionality provided by the C header <signal.h>.

use libc::{sigaction, SIG_IGN};

/// Convenience type when working with signal handlers.
pub type SaHandlerT = extern "C" fn(i32);

/// Returns the handler of a signal, as a SaHandlerT value regardless of its true type.
/// The resulting function can be compared to special values like SIG_IGN but it is not
/// portable to call it.
///
/// # Arguments
/// * `a` - Reference to a sigaction struct containing the signal handler information
///
/// # Notes
/// POSIX says that special values like SIG_IGN can only occur when action.sa_flags
/// does not contain SA_SIGINFO. But in Linux 2.4, for example, sa_sigaction and
/// sa_handler are aliases and a signal is ignored if sa_sigaction (after casting)
/// equals SIG_IGN. This implementation relies on the fact that the two are aliases,
/// and simply returns sa_handler.
#[inline]
pub fn get_handler(a: &sigaction) -> SaHandlerT {
    unsafe { std::mem::transmute(a.sa_sigaction) }
}