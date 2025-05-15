/*
**  GNU Pth - The GNU Portable Threads
**  Copyright (c) 1999-2006 Ralf S. Engelschall <rse@engelschall.com>
**
**  This file is part of GNU Pth, a non-preemptive thread scheduling
**  library which can be found at http://www.gnu.org/software/pth/.
**
**  This library is free software; you can redistribute it and/or
**  modify it under the terms of the GNU Lesser General Public
**  License as published by the Free Software Foundation; either
**  version 2.1 of the License, or (at your option) any later version.
**
**  This library is distributed in the hope that it will be useful,
**  but WITHOUT ANY WARRANTY; without even the implied warranty of
**  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
**  Lesser General Public License for more details.
**
**  You should have received a copy of the GNU Lesser General Public
**  License along with this library; if not, write to the Free Software
**  Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA 02111-1307
**  USA, or contact Ralf S. Engelschall <rse@engelschall.com>.
**
**  pth_sync.rs: Pth synchronization facilities (Rust translation)
*/

use std::sync::{Arc, Mutex, Condvar};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::thread;
use std::time::Duration;
use std::collections::LinkedList;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum PthError {
    InvalidArgument,
    Deadlock,
    Busy,
    AccessDenied,
    Interrupted,
    Other(String),
}

impl Error for PthError {}
impl fmt::Display for PthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

type PthResult<T> = Result<T, PthError>;

#[derive(Clone, Copy, PartialEq)]
enum PthMutexState {
    Uninitialized,
    Initialized,
    Locked,
}

struct PthMutex {
    state: Mutex<PthMutexState>,
    owner: Mutex<Option<thread::ThreadId>>,
    count: Mutex<usize>,
    condvar: Condvar,
}

impl PthMutex {
    pub fn new() -> Self {
        PthMutex {
            state: Mutex::new(PthMutexState::Initialized),
            owner: Mutex::new(None),
            count: Mutex::new(0),
            condvar: Condvar::new(),
        }
    }

    pub fn init(&self) -> PthResult<()> {
        let mut state = self.state.lock().unwrap();
        *state = PthMutexState::Initialized;
        Ok(())
    }

    pub fn acquire(&self, try_only: bool) -> PthResult<()> {
        let current_thread = thread::current().id();
        let mut state = self.state.lock().unwrap();
        
        if *state == PthMutexState::Uninitialized {
            return Err(PthError::Deadlock);
        }

        if *state != PthMutexState::Locked {
            *state = PthMutexState::Locked;
            *self.owner.lock().unwrap() = Some(current_thread);
            *self.count.lock().unwrap() = 1;
            return Ok(());
        }

        let owner = self.owner.lock().unwrap();
        if let Some(owner_id) = *owner {
            if owner_id == current_thread {
                *self.count.lock().unwrap() += 1;
                return Ok(());
            }
        }

        if try_only {
            return Err(PthError::Busy);
        }

        while *state == PthMutexState::Locked {
            state = self.condvar.wait(state).unwrap();
        }

        *state = PthMutexState::Locked;
        *self.owner.lock().unwrap() = Some(current_thread);
        *self.count.lock().unwrap() = 1;
        Ok(())
    }

    pub fn release(&self) -> PthResult<()> {
        let current_thread = thread::current().id();
        let mut state = self.state.lock().unwrap();
        let mut owner = self.owner.lock().unwrap();
        let mut count = self.count.lock().unwrap();

        if *state == PthMutexState::Uninitialized {
            return Err(PthError::Deadlock);
        }

        if *state != PthMutexState::Locked {
            return Err(PthError::Deadlock);
        }

        if *owner != Some(current_thread) {
            return Err(PthError::AccessDenied);
        }

        *count -= 1;
        if *count == 0 {
            *state = PthMutexState::Initialized;
            *owner = None;
            self.condvar.notify_one();
        }
        Ok(())
    }
}

struct PthRwLock {
    state: Mutex<PthRwLockState>,
    readers: AtomicUsize,
    read_mutex: PthMutex,
    write_mutex: PthMutex,
}

#[derive(PartialEq)]
enum PthRwLockState {
    Uninitialized,
    Initialized,
}

enum PthRwLockOp {
    Read,
    Write,
}

impl PthRwLock {
    pub fn new() -> Self {
        PthRwLock {
            state: Mutex::new(PthRwLockState::Initialized),
            readers: AtomicUsize::new(0),
            read_mutex: PthMutex::new(),
            write_mutex: PthMutex::new(),
        }
    }

    pub fn init(&self) -> PthResult<()> {
        let mut state = self.state.lock().unwrap();
        *state = PthRwLockState::Initialized;
        self.read_mutex.init()?;
        self.write_mutex.init()?;
        Ok(())
    }

    pub fn acquire(&self, op: PthRwLockOp, try_only: bool) -> PthResult<()> {
        let state = self.state.lock().unwrap();
        if *state == PthRwLockState::Uninitialized {
            return Err(PthError::Deadlock);
        }

        match op {
            PthRwLockOp::Write => {
                self.write_mutex.acquire(try_only)?;
                Ok(())
            }
            PthRwLockOp::Read => {
                self.read_mutex.acquire(try_only)?;
                let readers = self.readers.fetch_add(1, Ordering::SeqCst);
                if readers == 0 {
                    if let Err(e) = self.write_mutex.acquire(try_only) {
                        self.readers.fetch_sub(1, Ordering::SeqCst);
                        self.read_mutex.release()?;
                        return Err(e);
                    }
                }
                self.read_mutex.release()?;
                Ok(())
            }
        }
    }

    pub fn release(&self) -> PthResult<()> {
        let state = self.state.lock().unwrap();
        if *state == PthRwLockState::Uninitialized {
            return Err(PthError::Deadlock);
        }

        if self.write_mutex.state.lock().unwrap() == &PthMutexState::Locked {
            self.write_mutex.release()
        } else {
            self.read_mutex.acquire(false)?;
            let readers = self.readers.fetch_sub(1, Ordering::SeqCst);
            if readers == 1 {
                self.write_mutex.release()?;
            }
            self.read_mutex.release()?;
            Ok(())
        }
    }
}

struct PthCond {
    state: Mutex<PthCondState>,
    waiters: AtomicUsize,
    condvar: Condvar,
}

#[derive(PartialEq)]
enum PthCondState {
    Uninitialized,
    Initialized,
    Signaled,
    Broadcast,
}

impl PthCond {
    pub fn new() -> Self {
        PthCond {
            state: Mutex::new(PthCondState::Initialized),
            waiters: AtomicUsize::new(0),
            condvar: Condvar::new(),
        }
    }

    pub fn init(&self) -> PthResult<()> {
        let mut state = self.state.lock().unwrap();
        *state = PthCondState::Initialized;
        Ok(())
    }

    pub fn await(&self, mutex: &PthMutex) -> PthResult<()> {
        let mut state = self.state.lock().unwrap();
        if *state == PthCondState::Uninitialized {
            return Err(PthError::Deadlock);
        }

        if *state == PthCondState::Signaled && *state != PthCondState::Broadcast {
            *state = PthCondState::Initialized;
            return Ok(());
        }

        self.waiters.fetch_add(1, Ordering::SeqCst);
        mutex.release()?;

        while *state != PthCondState::Signaled && *state != PthCondState::Broadcast {
            state = self.condvar.wait(state).unwrap();
        }

        self.waiters.fetch_sub(1, Ordering::SeqCst);
        mutex.acquire(false)?;
        Ok(())
    }

    pub fn notify(&self, broadcast: bool) -> PthResult<()> {
        let mut state = self.state.lock().unwrap();
        if *state == PthCondState::Uninitialized {
            return Err(PthError::Deadlock);
        }

        if self.waiters.load(Ordering::SeqCst) > 0 {
            *state = PthCondState::Signaled;
            if broadcast {
                *state = PthCondState::Broadcast;
            }
            if broadcast {
                self.condvar.notify_all();
            } else {
                self.condvar.notify_one();
            }
        }
        Ok(())
    }
}

struct PthBarrier {
    state: Mutex<PthBarrierState>,
    threshold: usize,
    count: AtomicUsize,
    cycle: AtomicBool,
    condvar: Condvar,
}

#[derive(PartialEq)]
enum PthBarrierState {
    Uninitialized,
    Initialized,
}

impl PthBarrier {
    pub fn new(threshold: usize) -> Self {
        PthBarrier {
            state: Mutex::new(PthBarrierState::Initialized),
            threshold,
            count: AtomicUsize::new(threshold),
            cycle: AtomicBool::new(false),
            condvar: Condvar::new(),
        }
    }

    pub fn init(&self) -> PthResult<()> {
        let mut state = self.state.lock().unwrap();
        *state = PthBarrierState::Initialized;
        Ok(())
    }

    pub fn reach(&self) -> PthResult<bool> {
        let state = self.state.lock().unwrap();
        if *state == PthBarrierState::Uninitialized {
            return Err(PthError::Deadlock);
        }

        let cycle = self.cycle.load(Ordering::SeqCst);
        let count = self.count.fetch_sub(1, Ordering::SeqCst);
        if count == 1 {
            self.cycle.store(!cycle, Ordering::SeqCst);
            self.count.store(self.threshold, Ordering::SeqCst);
            self.condvar.notify_all();
            Ok(true)
        } else {
            let mut state_guard = self.state.lock().unwrap();
            while cycle == self.cycle.load(Ordering::SeqCst) {
                state_guard = self.condvar.wait(state_guard).unwrap();
            }
            Ok(false)
        }
    }
}