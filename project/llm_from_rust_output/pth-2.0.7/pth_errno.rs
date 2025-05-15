use std::sync::atomic::{AtomicI32, Ordering};

#[no_mangle]
pub static __pth_errno_storage: AtomicI32 = AtomicI32::new(0);
#[no_mangle]
pub static __pth_errno_flag: AtomicI32 = AtomicI32::new(0);