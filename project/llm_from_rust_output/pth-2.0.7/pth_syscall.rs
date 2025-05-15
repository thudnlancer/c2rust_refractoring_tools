use std::sync::atomic::{AtomicI32, Ordering};

#[no_mangle]
pub static pth_syscall_soft: AtomicI32 = AtomicI32::new(0);
#[no_mangle]
pub static pth_syscall_hard: AtomicI32 = AtomicI32::new(0);

#[no_mangle]
pub extern "C" fn __pth_syscall_init() {}

#[no_mangle]
pub extern "C" fn __pth_syscall_kill() {}