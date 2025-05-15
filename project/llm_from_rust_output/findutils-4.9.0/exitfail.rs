use std::sync::atomic::{AtomicI32, Ordering};

pub static EXIT_FAILURE: AtomicI32 = AtomicI32::new(1);