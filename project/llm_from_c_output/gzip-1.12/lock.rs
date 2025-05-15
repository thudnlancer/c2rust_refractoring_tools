//! Locking primitives for multithreaded situations.
//!
//! This module provides thread synchronization primitives similar to those in the C code,
//! implemented using Rust's standard library threading facilities.

use std::sync::{Mutex, Condvar, RwLock, Once};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

/// Normal (non-recursive) lock type.
pub struct GlLock {
    inner: Mutex<()>,
    initialized: AtomicBool,
    init_once: Once,
}

impl GlLock {
    pub const fn new() -> Self {
        GlLock {
            inner: Mutex::new(()),
            initialized: AtomicBool::new(false),
            init_once: Once::new(),
        }
    }

    pub fn init(&self) -> Result<(), ()> {
        self.init_once.call_once(|| {
            self.initialized.store(true, Ordering::SeqCst);
        });
        Ok(())
    }

    pub fn lock(&self) -> Result<(), ()> {
        self.init()?;
        self.inner.lock().map_err(|_| ())?;
        Ok(())
    }

    pub fn unlock(&self) -> Result<(), ()> {
        self.init()?;
        unsafe { self.inner.force_unlock(); }
        Ok(())
    }

    pub fn destroy(&self) -> Result<(), ()> {
        self.init()?;
        // Mutex in Rust doesn't need explicit destruction
        Ok(())
    }
}

/// Read-Write lock type.
pub struct GlRwLock {
    inner: RwLock<()>,
    waiting_writers: Condvar,
    waiting_readers: Condvar,
    waiting_writers_count: Mutex<u32>,
    runcount: Mutex<i32>,
    initialized: AtomicBool,
    init_once: Once,
}

impl GlRwLock {
    pub const fn new() -> Self {
        GlRwLock {
            inner: RwLock::new(()),
            waiting_writers: Condvar::new(),
            waiting_readers: Condvar::new(),
            waiting_writers_count: Mutex::new(0),
            runcount: Mutex::new(0),
            initialized: AtomicBool::new(false),
            init_once: Once::new(),
        }
    }

    pub fn init(&self) -> Result<(), ()> {
        self.init_once.call_once(|| {
            self.initialized.store(true, Ordering::SeqCst);
        });
        Ok(())
    }

    pub fn rdlock(&self) -> Result<(), ()> {
        self.init()?;
        let mut runcount = self.runcount.lock().map_err(|_| ())?;
        let mut writers_count = self.waiting_writers_count.lock().map_err(|_| ())?;

        while !(*runcount + 1 > 0 && *writers_count == 0) {
            runcount = self.waiting_readers.wait(runcount).map_err(|_| ())?;
        }

        *runcount += 1;
        Ok(())
    }

    pub fn wrlock(&self) -> Result<(), ()> {
        self.init()?;
        let mut runcount = self.runcount.lock().map_err(|_| ())?;
        let mut writers_count = self.waiting_writers_count.lock().map_err(|_| ())?;

        *writers_count += 1;
        while *runcount != 0 {
            runcount = self.waiting_writers.wait(runcount).map_err(|_| ())?;
        }
        *writers_count -= 1;
        *runcount = -1;
        Ok(())
    }

    pub fn unlock(&self) -> Result<(), ()> {
        self.init()?;
        let mut runcount = self.runcount.lock().map_err(|_| ())?;

        if *runcount < 0 {
            *runcount = 0;
        } else {
            *runcount -= 1;
        }

        if *runcount == 0 {
            let writers_count = self.waiting_writers_count.lock().map_err(|_| ())?;
            if *writers_count > 0 {
                self.waiting_writers.notify_one();
            } else {
                self.waiting_readers.notify_all();
            }
        }
        Ok(())
    }

    pub fn destroy(&self) -> Result<(), ()> {
        self.init()?;
        // Rust's types handle cleanup automatically
        Ok(())
    }
}

/// Recursive lock type.
pub struct GlRecursiveLock {
    inner: Mutex<()>,
    owner: Mutex<Option<thread::ThreadId>>,
    depth: Mutex<u32>,
    initialized: AtomicBool,
    init_once: Once,
}

impl GlRecursiveLock {
    pub const fn new() -> Self {
        GlRecursiveLock {
            inner: Mutex::new(()),
            owner: Mutex::new(None),
            depth: Mutex::new(0),
            initialized: AtomicBool::new(false),
            init_once: Once::new(),
        }
    }

    pub fn init(&self) -> Result<(), ()> {
        self.init_once.call_once(|| {
            self.initialized.store(true, Ordering::SeqCst);
        });
        Ok(())
    }

    pub fn lock(&self) -> Result<(), ()> {
        self.init()?;
        let self_id = thread::current().id();
        let mut owner = self.owner.lock().map_err(|_| ())?;
        let mut depth = self.depth.lock().map_err(|_| ())?;

        if *owner != Some(self_id) {
            drop(owner);
            drop(depth);
            self.inner.lock().map_err(|_| ())?;
            *self.owner.lock().map_err(|_| ())? = Some(self_id);
            *self.depth.lock().map_err(|_| ())? = 1;
        } else {
            *depth += 1;
        }
        Ok(())
    }

    pub fn unlock(&self) -> Result<(), ()> {
        self.init()?;
        let self_id = thread::current().id();
        let mut owner = self.owner.lock().map_err(|_| ())?;
        let mut depth = self.depth.lock().map_err(|_| ())?;

        if *owner != Some(self_id) {
            return Err(());
        }

        if *depth == 0 {
            return Err(());
        }

        *depth -= 1;
        if *depth == 0 {
            *owner = None;
            unsafe { self.inner.force_unlock(); }
        }
        Ok(())
    }

    pub fn destroy(&self) -> Result<(), ()> {
        self.init()?;
        // Rust's types handle cleanup automatically
        Ok(())
    }
}

/// Once-only execution type.
pub struct GlOnce {
    inner: Once,
}

impl GlOnce {
    pub const fn new() -> Self {
        GlOnce {
            inner: Once::new(),
        }
    }

    pub fn call(&self, init_function: fn()) -> Result<(), ()> {
        self.inner.call_once(init_function);
        Ok(())
    }
}

// Macro equivalents
#[macro_export]
macro_rules! gl_lock_define {
    ($vis:vis $name:ident) => {
        $vis static $name: $crate::GlLock = $crate::GlLock::new();
    };
}

#[macro_export]
macro_rules! gl_lock_define_initialized {
    ($vis:vis $name:ident) => {
        $vis static $name: $crate::GlLock = $crate::GlLock::new();
    };
}

#[macro_export]
macro_rules! gl_rwlock_define {
    ($vis:vis $name:ident) => {
        $vis static $name: $crate::GlRwLock = $crate::GlRwLock::new();
    };
}

#[macro_export]
macro_rules! gl_rwlock_define_initialized {
    ($vis:vis $name:ident) => {
        $vis static $name: $crate::GlRwLock = $crate::GlRwLock::new();
    };
}

#[macro_export]
macro_rules! gl_recursive_lock_define {
    ($vis:vis $name:ident) => {
        $vis static $name: $crate::GlRecursiveLock = $crate::GlRecursiveLock::new();
    };
}

#[macro_export]
macro_rules! gl_recursive_lock_define_initialized {
    ($vis:vis $name:ident) => {
        $vis static $name: $crate::GlRecursiveLock = $crate::GlRecursiveLock::new();
    };
}

#[macro_export]
macro_rules! gl_once_define {
    ($vis:vis $name:ident) => {
        $vis static $name: $crate::GlOnce = $crate::GlOnce::new();
    };
}