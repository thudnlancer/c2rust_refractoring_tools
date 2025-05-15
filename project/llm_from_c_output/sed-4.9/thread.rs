//! Thread creation and control utilities.
//!
//! This module provides a cross-platform abstraction for thread operations,
//! similar to the C version but implemented in safe Rust.

use std::sync::{Once, ONCE_INIT};
use std::thread;
use std::sync::mpsc::{channel, Sender};
use std::sync::Arc;
use std::panic;

pub type GlThread = thread::JoinHandle<()>;

/// Creates a new thread.
pub fn gl_thread_create<F, T>(func: F) -> thread::JoinHandle<T>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
{
    thread::spawn(func)
}

/// Waits for a thread to finish and returns its return value.
pub fn gl_thread_join<T>(handle: thread::JoinHandle<T>) -> T {
    handle.join().unwrap_or_else(|e| panic::resume_unwind(e))
}

/// Returns the current thread's ID.
pub fn gl_thread_self() -> thread::ThreadId {
    thread::current().id()
}

/// Terminates the current thread with the given return value.
pub fn gl_thread_exit<T>(_return_value: T) -> ! {
    thread::exit(())
}

/// Sets up fork handlers (not supported in Rust's standard library).
pub fn gl_thread_atfork(
    _prepare_func: fn(),
    _parent_func: fn(),
    _child_func: fn(),
) -> i32 {
    0
}

/// Thread-local storage key initialization.
static INIT_THREAD_KEY: Once = ONCE_INIT;

/// Thread-specific data structure.
struct ThreadData<T> {
    exit_value: Option<T>,
    sender: Sender<T>,
}

impl<T> ThreadData<T> {
    fn new(sender: Sender<T>) -> Self {
        ThreadData {
            exit_value: None,
            sender,
        }
    }
}

/// Creates a thread with proper exit value handling.
pub fn gl_thread_create_with_exit<F, T>(
    func: F,
) -> (thread::JoinHandle<()>, Arc<Sender<T>>)
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
{
    let (sender, receiver) = channel();
    let sender = Arc::new(sender);
    let sender_clone = sender.clone();

    let handle = thread::spawn(move || {
        let result = func();
        sender_clone.send(result).unwrap();
    });

    (handle, sender)
}

/// Joins a thread and retrieves its exit value.
pub fn gl_thread_join_with_exit<T>(
    handle: thread::JoinHandle<()>,
    receiver: Arc<Sender<T>>,
) -> Option<T> {
    handle.join().ok()?;
    receiver.recv().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thread_creation() {
        let handle = gl_thread_create(|| {
            println!("Hello from thread!");
            42
        });
        let result = gl_thread_join(handle);
        assert_eq!(result, 42);
    }

    #[test]
    fn test_thread_with_exit() {
        let (handle, sender) = gl_thread_create_with_exit(|| {
            println!("Thread with exit value");
            "success"
        });
        let result = gl_thread_join_with_exit(handle, sender);
        assert_eq!(result, Some("success"));
    }
}