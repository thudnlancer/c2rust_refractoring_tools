use std::sync::atomic::{AtomicI32, Ordering};

pub static __pth_compat_unit: AtomicI32 = AtomicI32::new(0);