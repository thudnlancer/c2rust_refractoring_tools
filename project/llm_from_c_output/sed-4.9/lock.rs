/*
 * Locking in multithreaded situations.
 * Copyright (C) 2005-2022 Free Software Foundation, Inc.
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

use std::sync::{Mutex, Condvar, Once, RwLock};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

// -------------------------- gl_lock_t datatype --------------------------

pub struct GlLock {
    mutex: Mutex<()>,
    init_needed: AtomicBool,
    init_once: Once,
}

impl GlLock {
    pub fn new() -> Self {
        GlLock {
            mutex: Mutex::new(()),
            init_needed: AtomicBool::new(true),
            init_once: Once::new(),
        }
    }

    pub fn lock(&self) -> Result<(), i32> {
        if self.init_needed.load(Ordering::SeqCst) {
            self.init_once.call_once(|| {
                self.init_needed.store(false, Ordering::SeqCst);
            });
        }
        self.mutex.lock().map_err(|_| EAGAIN)?;
        Ok(())
    }

    pub fn unlock(&self) -> Result<(), i32> {
        if self.init_needed.load(Ordering::SeqCst) {
            self.init_once.call_once(|| {
                self.init_needed.store(false, Ordering::SeqCst);
            });
        }
        unsafe {
            self.mutex.force_unlock();
        }
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
    init_needed: AtomicBool,
    init_once: Once,
}

impl GlRwLock {
    pub fn new() -> Self {
        GlRwLock {
            lock: Mutex::new(()),
            waiting_readers: Condvar::new(),
            waiting_writers: Condvar::new(),
            waiting_writers_count: 0,
            runcount: 0,
            init_needed: AtomicBool::new(true),
            init_once: Once::new(),
        }
    }

    pub fn rdlock(&self) -> Result<(), i32> {
        if self.init_needed.load(Ordering::SeqCst) {
            self.init_once.call_once(|| {
                self.init_needed.store(false, Ordering::SeqCst);
            });
        }

        let guard = self.lock.lock().map_err(|_| EAGAIN)?;
        let mut guard = guard;

        while !(self.runcount + 1 > 0 && self.waiting_writers_count == 0) {
            guard = self.waiting_readers.wait(guard).map_err(|_| EINVAL)?;
        }

        self.runcount += 1;
        Ok(())
    }

    pub fn wrlock(&self) -> Result<(), i32> {
        if self.init_needed.load(Ordering::SeqCst) {
            self.init_once.call_once(|| {
                self.init_needed.store(false, Ordering::SeqCst);
            });
        }

        let guard = self.lock.lock().map_err(|_| EAGAIN)?;
        let mut guard = guard;

        while self.runcount != 0 {
            self.waiting_writers_count += 1;
            guard = self.waiting_writers.wait(guard).map_err(|_| {
                self.waiting_writers_count -= 1;
                EINVAL
            })?;
            self.waiting_writers_count -= 1;
        }

        self.runcount = -1;
        Ok(())
    }

    pub fn unlock(&self) -> Result<(), i32> {
        if self.init_needed.load(Ordering::SeqCst) {
            self.init_once.call_once(|| {
                self.init_needed.store(false, Ordering::SeqCst);
            });
        }

        let guard = self.lock.lock().map_err(|_| EAGAIN)?;
        let mut guard = guard;

        if self.runcount < 0 {
            if self.runcount != -1 {
                return Err(EINVAL);
            }
            self.runcount = 0;
        } else {
            if self.runcount <= 0 {
                return Err(EINVAL);
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
}

// --------------------- gl_recursive_lock_t datatype ---------------------

pub struct GlRecursiveLock {
    mutex: Mutex<()>,
    owner: Mutex<Option<thread::ThreadId>>,
    depth: Mutex<u32>,
    init_needed: AtomicBool,
    init_once: Once,
}

impl GlRecursiveLock {
    pub fn new() -> Self {
        GlRecursiveLock {
            mutex: Mutex::new(()),
            owner: Mutex::new(None),
            depth: Mutex::new(0),
            init_needed: AtomicBool::new(true),
            init_once: Once::new(),
        }
    }

    pub fn lock(&self) -> Result<(), i32> {
        if self.init_needed.load(Ordering::SeqCst) {
            self.init_once.call_once(|| {
                self.init_needed.store(false, Ordering::SeqCst);
            });
        }

        let current_thread = thread::current().id();
        let mut owner = self.owner.lock().map_err(|_| EAGAIN)?;

        if *owner != Some(current_thread) {
            drop(owner);
            self.mutex.lock().map_err(|_| EAGAIN)?;
            *self.owner.lock().map_err(|_| EAGAIN)? = Some(current_thread);
        }

        let mut depth = self.depth.lock().map_err(|_| EAGAIN)?;
        *depth = depth.checked_add(1).ok_or(EAGAIN)?;
        Ok(())
    }

    pub fn unlock(&self) -> Result<(), i32> {
        if self.init_needed.load(Ordering::SeqCst) {
            self.init_once.call_once(|| {
                self.init_needed.store(false, Ordering::SeqCst);
            });
        }

        let current_thread = thread::current().id();
        let mut owner = self.owner.lock().map_err(|_| EPERM)?;

        if *owner != Some(current_thread) {
            return Err(EPERM);
        }

        let mut depth = self.depth.lock().map_err(|_| EINVAL)?;
        if *depth == 0 {
            return Err(EINVAL);
        }

        *depth -= 1;
        if *depth == 0 {
            *owner = None;
            unsafe {
                self.mutex.force_unlock();
            }
        }

        Ok(())
    }
}

// -------------------------- gl_once_t datatype --------------------------

pub struct GlOnce {
    once: Once,
}

impl GlOnce {
    pub fn new() -> Self {
        GlOnce {
            once: Once::new(),
        }
    }

    pub fn call(&self, init_function: fn()) -> Result<(), i32> {
        self.once.call_once(init_function);
        Ok(())
    }
}

// Error codes
const EAGAIN: i32 = 11;
const EINVAL: i32 = 22;
const EPERM: i32 = 1;