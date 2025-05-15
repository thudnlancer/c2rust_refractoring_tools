//! Locking primitives for multithreaded situations.
//! 
//! This module provides thread synchronization primitives similar to those in the C code,
//! implemented using Rust's standard library threading facilities.

use std::sync::{Mutex, MutexGuard, Condvar, Once};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

/// A simple non-recursive mutex.
pub struct GlLock {
    mutex: Mutex<()>,
    init_needed: AtomicBool,
    init_once: Once,
}

impl GlLock {
    /// Creates a new uninitialized lock.
    pub fn new() -> Self {
        GlLock {
            mutex: Mutex::new(()),
            init_needed: AtomicBool::new(true),
            init_once: Once::new(),
        }
    }

    /// Initializes the lock.
    pub fn init(&self) -> Result<(), ()> {
        self.init_once.call_once(|| {
            self.init_needed.store(false, Ordering::SeqCst);
        });
        Ok(())
    }

    /// Locks the mutex.
    pub fn lock(&self) -> Result<MutexGuard<()>, ()> {
        if self.init_needed.load(Ordering::SeqCst) {
            self.init()?;
        }
        self.mutex.lock().map_err(|_| ())
    }

    /// Unlocks the mutex.
    pub fn unlock(&self) -> Result<(), ()> {
        if self.init_needed.load(Ordering::SeqCst) {
            self.init()?;
        }
        Ok(())
    }

    /// Destroys the lock.
    pub fn destroy(&self) -> Result<(), ()> {
        if self.init_needed.load(Ordering::SeqCst) {
            self.init()?;
        }
        Ok(())
    }
}

/// A read-write lock.
pub struct GlRwLock {
    lock: Mutex<()>,
    waiting_readers: Condvar,
    waiting_writers: Condvar,
    waiting_writers_count: Mutex<u32>,
    runcount: Mutex<i32>,
    init_needed: AtomicBool,
    init_once: Once,
}

impl GlRwLock {
    /// Creates a new uninitialized read-write lock.
    pub fn new() -> Self {
        GlRwLock {
            lock: Mutex::new(()),
            waiting_readers: Condvar::new(),
            waiting_writers: Condvar::new(),
            waiting_writers_count: Mutex::new(0),
            runcount: Mutex::new(0),
            init_needed: AtomicBool::new(true),
            init_once: Once::new(),
        }
    }

    /// Initializes the read-write lock.
    pub fn init(&self) -> Result<(), ()> {
        self.init_once.call_once(|| {
            self.init_needed.store(false, Ordering::SeqCst);
        });
        Ok(())
    }

    /// Locks for reading.
    pub fn rdlock(&self) -> Result<(), ()> {
        if self.init_needed.load(Ordering::SeqCst) {
            self.init()?;
        }

        let mut guard = self.lock.lock().map_err(|_| ())?;
        
        while {
            let runcount = *self.runcount.lock().map_err(|_| ())?;
            let waiting_writers = *self.waiting_writers_count.lock().map_err(|_| ())?;
            !(runcount + 1 > 0 && waiting_writers == 0)
        } {
            guard = self.waiting_readers.wait(guard).map_err(|_| ())?;
        }

        *self.runcount.lock().map_err(|_| ())? += 1;
        Ok(())
    }

    /// Locks for writing.
    pub fn wrlock(&self) -> Result<(), ()> {
        if self.init_needed.load(Ordering::SeqCst) {
            self.init()?;
        }

        let mut guard = self.lock.lock().map_err(|_| ())?;
        
        *self.waiting_writers_count.lock().map_err(|_| ())? += 1;
        
        while *self.runcount.lock().map_err(|_| ())? != 0 {
            guard = self.waiting_writers.wait(guard).map_err(|_| ())?;
        }
        
        *self.waiting_writers_count.lock().map_err(|_| ())? -= 1;
        *self.runcount.lock().map_err(|_| ())? = -1;
        Ok(())
    }

    /// Unlocks the read-write lock.
    pub fn unlock(&self) -> Result<(), ()> {
        if self.init_needed.load(Ordering::SeqCst) {
            self.init()?;
        }

        let mut guard = self.lock.lock().map_err(|_| ())?;
        let mut runcount = self.runcount.lock().map_err(|_| ())?;
        
        if *runcount < 0 {
            // Writer unlocking
            if *runcount != -1 {
                return Err(());
            }
            *runcount = 0;
        } else {
            // Reader unlocking
            if *runcount <= 0 {
                return Err(());
            }
            *runcount -= 1;
        }

        if *runcount == 0 {
            let waiting_writers = *self.waiting_writers_count.lock().map_err(|_| ())?;
            if waiting_writers > 0 {
                self.waiting_writers.notify_one();
            } else {
                self.waiting_readers.notify_all();
            }
        }
        Ok(())
    }

    /// Destroys the read-write lock.
    pub fn destroy(&self) -> Result<(), ()> {
        if self.init_needed.load(Ordering::SeqCst) {
            self.init()?;
        }
        Ok(())
    }
}

/// A recursive mutex.
pub struct GlRecursiveLock {
    mutex: Mutex<()>,
    owner: Mutex<Option<thread::ThreadId>>,
    depth: Mutex<u32>,
    init_needed: AtomicBool,
    init_once: Once,
}

impl GlRecursiveLock {
    /// Creates a new uninitialized recursive mutex.
    pub fn new() -> Self {
        GlRecursiveLock {
            mutex: Mutex::new(()),
            owner: Mutex::new(None),
            depth: Mutex::new(0),
            init_needed: AtomicBool::new(true),
            init_once: Once::new(),
        }
    }

    /// Initializes the recursive mutex.
    pub fn init(&self) -> Result<(), ()> {
        self.init_once.call_once(|| {
            self.init_needed.store(false, Ordering::SeqCst);
        });
        Ok(())
    }

    /// Locks the recursive mutex.
    pub fn lock(&self) -> Result<(), ()> {
        if self.init_needed.load(Ordering::SeqCst) {
            self.init()?;
        }

        let current_thread = thread::current().id();
        let mut owner = self.owner.lock().map_err(|_| ())?;
        
        if *owner != Some(current_thread) {
            drop(owner);
            self.mutex.lock().map_err(|_| ())?;
            *self.owner.lock().map_err(|_| ())? = Some(current_thread);
        }
        
        let mut depth = self.depth.lock().map_err(|_| ())?;
        *depth += 1;
        if *depth == 0 {
            *depth -= 1;
            return Err(());
        }
        Ok(())
    }

    /// Unlocks the recursive mutex.
    pub fn unlock(&self) -> Result<(), ()> {
        if self.init_needed.load(Ordering::SeqCst) {
            self.init()?;
        }

        let current_thread = thread::current().id();
        let owner = self.owner.lock().map_err(|_| ())?;
        
        if *owner != Some(current_thread) {
            return Err(());
        }
        
        let mut depth = self.depth.lock().map_err(|_| ())?;
        if *depth == 0 {
            return Err(());
        }
        
        *depth -= 1;
        if *depth == 0 {
            *self.owner.lock().map_err(|_| ())? = None;
            drop(depth);
            drop(owner);
            self.mutex.unlock().map_err(|_| ())?;
        }
        Ok(())
    }

    /// Destroys the recursive mutex.
    pub fn destroy(&self) -> Result<(), ()> {
        if self.init_needed.load(Ordering::SeqCst) {
            self.init()?;
        }
        
        let owner = self.owner.lock().map_err(|_| ())?;
        if owner.is_some() {
            return Err(());
        }
        Ok(())
    }
}

/// A once-only execution primitive.
pub struct GlOnce {
    once: Once,
    completed: AtomicBool,
}

impl GlOnce {
    /// Creates a new once primitive.
    pub fn new() -> Self {
        GlOnce {
            once: Once::new(),
            completed: AtomicBool::new(false),
        }
    }

    /// Executes the function exactly once.
    pub fn call<F: FnOnce()>(&self, f: F) -> Result<(), ()> {
        if !self.completed.load(Ordering::SeqCst) {
            self.once.call_once(|| {
                f();
                self.completed.store(true, Ordering::SeqCst);
            });
        }
        Ok(())
    }
}