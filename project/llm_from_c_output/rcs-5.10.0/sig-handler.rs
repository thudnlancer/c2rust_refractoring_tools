//! Convenience declarations when working with signal handling.
//!
//! This module provides a safe interface for working with signal handlers in Rust.
//! It is based on the original C code from GNU, adapted to follow Rust's safety
//! practices and idioms.

use libc::{c_int, sigaction, SA_SIGINFO, SIG_IGN};

/// Convenience type when working with signal handlers.
pub type SaHandlerT = extern "C" fn(c_int);

/// Returns the handler of a signal as a `SaHandlerT` value.
/// 
/// The resulting function can be compared to special values like `SIG_IGN` but
/// it is not portable to call it.
/// 
/// # Arguments
/// * `a` - Reference to a sigaction struct containing the signal handler
/// 
/// # Note
/// POSIX says that special values like SIG_IGN can only occur when
/// action.sa_flags does not contain SA_SIGINFO. But in Linux 2.4,
/// for example, sa_sigaction and sa_handler are aliases and a signal
/// is ignored if sa_sigaction (after casting) equals SIG_IGN. In
/// this case, this implementation relies on the fact that the two
/// are aliases, and simply returns sa_handler.
pub fn get_handler(a: &sigaction) -> SaHandlerT {
    unsafe {
        // SAFETY: This is safe because we're just reading the union field
        // and returning it as a function pointer. We don't execute the pointer.
        a.__sigaction_handler.sa_handler
    }
}