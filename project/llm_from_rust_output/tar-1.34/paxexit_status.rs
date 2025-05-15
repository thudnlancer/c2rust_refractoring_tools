use std::sync::atomic::{AtomicI32, Ordering};

#[no_mangle]
pub static exit_status: AtomicI32 = AtomicI32::new(0);