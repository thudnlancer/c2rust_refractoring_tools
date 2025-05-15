use std::sync::atomic::{AtomicI32, Ordering};

pub static EXIT_STATUS: AtomicI32 = AtomicI32::new(0);