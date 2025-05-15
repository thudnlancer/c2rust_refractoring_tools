use std::sync::atomic::{AtomicI32, Ordering};

#[no_mangle]
pub static exit_failure: AtomicI32 = AtomicI32::new(1);