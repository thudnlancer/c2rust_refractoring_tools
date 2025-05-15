/*
 * Locking in multithreaded situations.
 * Copyright (C) 2005-2023 Free Software Foundation, Inc.
 *
 * This file is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as
 * published by the Free Software Foundation; either version 2.1 of the
 * License, or (at your option) any later version.
 *
 * This file is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use std::sync::{Mutex, Condvar, Once, RwLock, OnceState};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

// -------------------------- gl_lock_t datatype --------------------------

pub struct GlLock {
    mutex: Mutex<()>,
    once: Once,
}

impl GlLock {
    pub fn new() -> Self {
        GlLock {
            mutex: Mutex::new(()),
            once: Once::new(),
        }
    }

    pub fn lock(&self) -> Result<(), ()> {
        self.once.call_once(|| {});
        self.mutex.lock().map(|_| ()).map_err(|_| ())
    }

    pub fn unlock(&self) -> Result<(), ()> {
        Ok(())
    }

    pub fn destroy(&self) -> Result<(), ()> {
        Ok(())
    }
}

// ------------------------- gl_rwlock_t datatype -------------------------

pub struct GlRwLock {
    lock: Mutex<()>,
    waiting_readers: Condvar,
    waiting_writers: Condvar,
    waiting_writers_count: usize,
    runcount: i32,
    once: Once,
}

impl GlRwLock {
    pub fn new() -> Self {
        GlRwLock {
            lock: Mutex::new(()),
            waiting_readers: Condvar::new(),
            waiting_writers: Condvar::new(),
            waiting_writers_count: 0,
            runcount: 0,
            once: Once::new(),
        }
    }

    pub fn rdlock(&self) -> Result<(), ()> {
        self.once.call_once(|| {});
        let guard = self.lock.lock().map_err(|_| ())?;

        let mut guard = guard;
        while !(self.runcount + 1 > 0 && self.waiting_writers_count == 0) {
            guard = self.waiting_readers.wait(guard).map_err(|_| ())?;
        }
        self.runcount += 1;
        Ok(())
    }

    pub fn wrlock(&self) -> Result<(), ()> {
        self.once.call_once(|| {});
        let guard = self.lock.lock().map_err(|_| ())?;

        let mut guard = guard;
        while !(self.runcount == 0) {
            self.waiting_writers_count += 1;
            guard = self.waiting_writers.wait(guard).map_err(|_| ())?;
            self.waiting_writers_count -= 1;
        }
        self.runcount = -1;
        Ok(())
    }

    pub fn unlock(&self) -> Result<(), ()> {
        self.once.call_once(|| {});
        let guard = self.lock.lock().map_err(|_| ())?;

        if self.runcount < 0 {
            if self.runcount != -1 {
                return Err(());
            }
            self.runcount = 0;
        } else {
            if self.runcount <= 0 {
                return Err(());
            }
            self.runcount -= 1;
        }

        if self.runcount == 0 {
            if self.waiting_writers_count > 0 {
                self.waiting_writers.notify_one();
            } else {
                self.waiting_readers.notify_all();
            }
        }
        Ok(())
    }

    pub fn destroy(&self) -> Result<(), ()> {
        self.once.call_once(|| {});
        Ok(())
    }
}

// --------------------- gl_recursive_lock_t datatype ---------------------

pub struct GlRecursiveLock {
    mutex: RwLock<()>,
    owner: Mutex<Option<thread::ThreadId>>,
    depth: Mutex<u32>,
    once: Once,
}

impl GlRecursiveLock {
    pub fn new() -> Self {
        GlRecursiveLock {
            mutex: RwLock::new(()),
            owner: Mutex::new(None),
            depth: Mutex::new(0),
            once: Once::new(),
        }
    }

    pub fn lock(&self) -> Result<(), ()> {
        self.once.call_once(|| {});
        let current = thread::current().id();
        let mut owner = self.owner.lock().map_err(|_| ())?;
        let mut depth = self.depth.lock().map_err(|_| ())?;

        if *owner != Some(current) {
            drop(owner);
            drop(depth);
            self.mutex.write().map_err(|_| ())?;
            *self.owner.lock().map_err(|_| ())? = Some(current);
            *self.depth.lock().map_err(|_| ())? = 1;
        } else {
            *depth += 1;
            if *depth == 0 {
                return Err(());
            }
        }
        Ok(())
    }

    pub fn unlock(&self) -> Result<(), ()> {
        self.once.call_once(|| {});
        let current = thread::current().id();
        let mut owner = self.owner.lock().map_err(|_| ())?;
        let mut depth = self.depth.lock().map_err(|_| ())?;

        if *owner != Some(current) {
            return Err(());
        }
        if *depth == 0 {
            return Err(());
        }
        *depth -= 1;
        if *depth == 0 {
            *owner = None;
            drop(owner);
            drop(depth);
            Ok(())
        } else {
            Ok(())
        }
    }

    pub fn destroy(&self) -> Result<(), ()> {
        self.once.call_once(|| {});
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