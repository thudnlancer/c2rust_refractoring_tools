use std::sync::{Once, Mutex, RwLock};
use std::sync::atomic::{AtomicBool, Ordering};

pub struct GlRecursiveLock(Mutex<()>);

impl GlRecursiveLock {
    pub fn new() -> Result<Self, i32> {
        Ok(Self(Mutex::new(())))
    }
}

pub struct GlRwLock(RwLock<()>);

impl GlRwLock {
    pub fn new() -> Result<Self, i32> {
        Ok(Self(RwLock::new(())))
    }
}

pub struct GlOnce {
    inner: Once,
    initialized: AtomicBool,
}

impl GlOnce {
    pub const fn new() -> Self {
        Self {
            inner: Once::new(),
            initialized: AtomicBool::new(false),
        }
    }

    pub fn call_once<F>(&self, f: F) -> bool
    where
        F: FnOnce(),
    {
        let mut called = false;
        self.inner.call_once(|| {
            f();
            self.initialized.store(true, Ordering::SeqCst);
            called = true;
        });
        called
    }

    pub fn is_completed(&self) -> bool {
        self.initialized.load(Ordering::SeqCst)
    }
}