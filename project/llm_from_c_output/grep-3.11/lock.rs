// Locking in multithreaded situations.
// Copyright (C) 2005-2023 Free Software Foundation, Inc.
//
// This file is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation; either version 2.1 of the
// License, or (at your option) any later version.
//
// This file is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::sync::{Mutex, Condvar, Once};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

// -------------------------- gl_lock_t datatype --------------------------

pub struct GlLock {
    init_needed: AtomicBool,
    init_once: Once,
    mutex: Mutex<()>,
}

impl GlLock {
    pub fn new() -> Self {
        GlLock {
            init_needed: AtomicBool::new(true),
            init_once: Once::new(),
            mutex: Mutex::new(()),
        }
    }

    pub fn init(&self) -> Result<(), ()> {
        self.init_once.call_once(|| {
            self.init_needed.store(false, Ordering::SeqCst);
        });
        Ok(())
    }

    pub fn lock(&self) -> Result<(), ()> {
        if self.init_needed.load(Ordering::SeqCst) {
            self.init()?;
        }
        self.mutex.lock().map_err(|_| ())?;
        Ok(())
    }

    pub fn unlock(&self) -> Result<(), ()> {
        if self.init_needed.load(Ordering::SeqCst) {
            self.init()?;
        }
        unsafe { self.mutex.force_unlock(); }
        Ok(())
    }

    pub fn destroy(&self) -> Result<(), ()> {
        if self.init_needed.load(Ordering::SeqCst) {
            self.init()?;
        }
        // Mutex in Rust doesn't need explicit destruction
        Ok(())
    }
}

// ------------------------- gl_rwlock_t datatype -------------------------

pub struct GlRWLock {
    init_needed: AtomicBool,
    init_once: Once,
    lock: Mutex<()>,
    waiting_readers: Condvar,
    waiting_writers: Condvar,
    waiting_writers_count: Mutex<u32>,
    runcount: Mutex<i32>,
}

impl GlRWLock {
    pub fn new() -> Self {
        GlRWLock {
            init_needed: AtomicBool::new(true),
            init_once: Once::new(),
            lock: Mutex::new(()),
            waiting_readers: Condvar::new(),
            waiting_writers: Condvar::new(),
            waiting_writers_count: Mutex::new(0),
            runcount: Mutex::new(0),
        }
    }

    pub fn init(&self) -> Result<(), ()> {
        self.init_once.call_once(|| {
            self.init_needed.store(false, Ordering::SeqCst);
        });
        Ok(())
    }

    pub fn rdlock(&self) -> Result<(), ()> {
        if self.init_needed.load(Ordering::SeqCst) {
            self.init()?;
        }

        let mut lock = self.lock.lock().map_err(|_| ())?;
        while !(*self.runcount.lock().map_err(|_| ())? + 1 > 0 && 
              *self.waiting_writers_count.lock().map_err(|_| ())? == 0 {
            lock = self.waiting_readers.wait(lock).map_err(|_| ())?;
        }
        *self.runcount.lock().map_err(|_| ())? += 1;
        Ok(())
    }

    pub fn wrlock(&self) -> Result<(), ()> {
        if self.init_needed.load(Ordering::SeqCst) {
            self.init()?;
        }

        let mut lock = self.lock.lock().map_err(|_| ())?;
        while *self.runcount.lock().map_err(|_| ())? != 0 {
            *self.waiting_writers_count.lock().map_err(|_| ())? += 1;
            lock = self.waiting_writers.wait(lock).map_err(|_| ())?;
            *self.waiting_writers_count.lock().map_err(|_| ())? -= 1;
        }
        *self.runcount.lock().map_err(|_| ())? = -1;
        Ok(())
    }

    pub fn unlock(&self) -> Result<(), ()> {
        if self.init_needed.load(Ordering::SeqCst) {
            self.init()?;
        }

        let mut runcount = self.runcount.lock().map_err(|_| ())?;
        if *runcount < 0 {
            *runcount = 0;
        } else {
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

    pub fn destroy(&self) -> Result<(), ()> {
        if self.init_needed.load(Ordering::SeqCst) {
            self.init()?;
        }
        Ok(())
    }
}

// --------------------- gl_recursive_lock_t datatype ---------------------

pub struct GlRecursiveLock {
    init_needed: AtomicBool,
    init_once: Once,
    mutex: Mutex<()>,
    owner: Mutex<Option<thread::ThreadId>>,
    depth: Mutex<u32>,
}

impl GlRecursiveLock {
    pub fn new() -> Self {
        GlRecursiveLock {
            init_needed: AtomicBool::new(true),
            init_once: Once::new(),
            mutex: Mutex::new(()),
            owner: Mutex::new(None),
            depth: Mutex::new(0),
        }
    }

    pub fn init(&self) -> Result<(), ()> {
        self.init_once.call_once(|| {
            self.init_needed.store(false, Ordering::SeqCst);
        });
        Ok(())
    }

    pub fn lock(&self) -> Result<(), ()> {
        if self.init_needed.load(Ordering::SeqCst) {
            self.init()?;
        }

        let current = thread::current().id();
        let mut owner = self.owner.lock().map_err(|_| ())?;
        if *owner != Some(current) {
            drop(owner);
            self.mutex.lock().map_err(|_| ())?;
            *self.owner.lock().map_err(|_| ())? = Some(current);
        }
        *self.depth.lock().map_err(|_| ())? += 1;
        Ok(())
    }

    pub fn unlock(&self) -> Result<(), ()> {
        if self.init_needed.load(Ordering::SeqCst) {
            self.init()?;
        }

        let current = thread::current().id();
        let mut owner = self.owner.lock().map_err(|_| ())?;
        if *owner != Some(current) {
            return Err(());
        }

        let mut depth = self.depth.lock().map_err(|_| ())?;
        if *depth == 0 {
            return Err(());
        }

        *depth -= 1;
        if *depth == 0 {
            *owner = None;
            unsafe { self.mutex.force_unlock(); }
        }
        Ok(())
    }

    pub fn destroy(&self) -> Result<(), ()> {
        if self.init_needed.load(Ordering::SeqCst) {
            self.init()?;
        }
        Ok(())
    }
}

// -------------------------- gl_once_t datatype --------------------------

pub struct GlOnce {
    once: Once,
    completed: AtomicBool,
}

impl GlOnce {
    pub fn new() -> Self {
        GlOnce {
            once: Once::new(),
            completed: AtomicBool::new(false),
        }
    }

    pub fn call(&self, f: impl FnOnce()) -> Result<(), ()> {
        if !self.completed.load(Ordering::SeqCst) {
            self.once.call_once(|| {
                f();
                self.completed.store(true, Ordering::SeqCst);
            });
        }
        Ok(())
    }
}