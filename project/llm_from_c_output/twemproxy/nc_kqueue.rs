/*
 * twemproxy - A fast and lightweight proxy for memcached protocol.
 * Copyright (C) 2011 Twitter, Inc.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use libc::{c_int, c_void, close, kevent, kqueue, timespec, EV_ADD, EV_CLEAR, EV_DELETE, EV_ERROR, EVFILT_READ, EVFILT_WRITE, EBADF, EINTR, EINVAL, ENOENT};
use std::ptr;
use std::time::Duration;
use std::os::unix::io::RawFd;
use std::mem::MaybeUninit;
use std::sync::Arc;
use std::cell::RefCell;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum EventError {
    KqueueFailed(std::io::Error),
    CloseFailed(std::io::Error),
    KeventFailed(std::io::Error),
    AllocationFailed,
}

impl fmt::Display for EventError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EventError::KqueueFailed(e) => write!(f, "kqueue failed: {}", e),
            EventError::CloseFailed(e) => write!(f, "close failed: {}", e),
            EventError::KeventFailed(e) => write!(f, "kevent failed: {}", e),
            EventError::AllocationFailed => write!(f, "memory allocation failed"),
        }
    }
}

impl Error for EventError {}

pub type EventCb = Box<dyn FnMut(*mut c_void, u32)>;
pub type EventStatsCb = Box<dyn FnMut(&Stats, &i32)>;

pub struct EventBase {
    kq: RawFd,
    changes: Vec<libc::kevent>,
    events: Vec<libc::kevent>,
    nchange: usize,
    nreturned: i32,
    nprocessed: usize,
    cb: Option<EventCb>,
}

pub struct Stats {
    sd: RawFd,
    interval: i64,
}

impl EventBase {
    pub fn create(nevent: usize, cb: Option<EventCb>) -> Result<Arc<RefCell<Self>>, EventError> {
        assert!(nevent > 0);

        let kq = unsafe { kqueue() };
        if kq < 0 {
            return Err(EventError::KqueueFailed(std::io::Error::last_os_error()));
        }

        let changes = vec![unsafe { MaybeUninit::zeroed().assume_init() }; nevent];
        let events = vec![unsafe { MaybeUninit::zeroed().assume_init() }; nevent];

        Ok(Arc::new(RefCell::new(EventBase {
            kq,
            changes,
            events,
            nchange: 0,
            nreturned: 0,
            nprocessed: 0,
            cb,
        })))
    }

    pub fn destroy(&mut self) -> Result<(), EventError> {
        if self.kq < 0 {
            return Ok(());
        }

        if unsafe { close(self.kq) } < 0 {
            return Err(EventError::CloseFailed(std::io::Error::last_os_error()));
        }
        self.kq = -1;

        Ok(())
    }

    pub fn add_in(&mut self, c: &mut Conn) -> Result<(), EventError> {
        assert!(self.kq > 0);
        assert!(c.sd > 0);
        assert!(self.nchange < self.changes.len());

        if c.recv_active {
            return Ok(());
        }

        unsafe {
            libc::EV_SET(
                self.changes.as_mut_ptr().add(self.nchange),
                c.sd as libc::uintptr_t,
                EVFILT_READ,
                EV_ADD | EV_CLEAR,
                0,
                0,
                c as *mut _ as *mut c_void,
            );
        }
        self.nchange += 1;
        c.recv_active = true;

        Ok(())
    }

    pub fn del_in(&mut self, c: &mut Conn) -> Result<(), EventError> {
        assert!(self.kq > 0);
        assert!(c.sd > 0);
        assert!(self.nchange < self.changes.len());

        if !c.recv_active {
            return Ok(());
        }

        unsafe {
            libc::EV_SET(
                self.changes.as_mut_ptr().add(self.nchange),
                c.sd as libc::uintptr_t,
                EVFILT_READ,
                EV_DELETE,
                0,
                0,
                c as *mut _ as *mut c_void,
            );
        }
        self.nchange += 1;
        c.recv_active = false;

        Ok(())
    }

    pub fn add_out(&mut self, c: &mut Conn) -> Result<(), EventError> {
        assert!(self.kq > 0);
        assert!(c.sd > 0);
        assert!(c.recv_active);
        assert!(self.nchange < self.changes.len());

        if c.send_active {
            return Ok(());
        }

        unsafe {
            libc::EV_SET(
                self.changes.as_mut_ptr().add(self.nchange),
                c.sd as libc::uintptr_t,
                EVFILT_WRITE,
                EV_ADD | EV_CLEAR,
                0,
                0,
                c as *mut _ as *mut c_void,
            );
        }
        self.nchange += 1;
        c.send_active = true;

        Ok(())
    }

    pub fn del_out(&mut self, c: &mut Conn) -> Result<(), EventError> {
        assert!(self.kq > 0);
        assert!(c.sd > 0);
        assert!(c.recv_active);
        assert!(self.nchange < self.changes.len());

        if !c.send_active {
            return Ok(());
        }

        unsafe {
            libc::EV_SET(
                self.changes.as_mut_ptr().add(self.nchange),
                c.sd as libc::uintptr_t,
                EVFILT_WRITE,
                EV_DELETE,
                0,
                0,
                c as *mut _ as *mut c_void,
            );
        }
        self.nchange += 1;
        c.send_active = false;

        Ok(())
    }

    pub fn add_conn(&mut self, c: &mut Conn) -> Result<(), EventError> {
        assert!(self.kq > 0);
        assert!(c.sd > 0);
        assert!(!c.recv_active);
        assert!(!c.send_active);
        assert!(self.nchange < self.changes.len());

        self.add_in(c)?;
        self.add_out(c)?;

        Ok(())
    }

    pub fn del_conn(&mut self, c: &mut Conn) -> Result<(), EventError> {
        assert!(self.kq > 0);
        assert!(c.sd > 0);
        assert!(self.nchange < self.changes.len());

        self.del_out(c)?;
        self.del_in(c)?;

        for i in self.nprocessed + 1..self.nreturned as usize {
            let ev = &mut self.events[i];
            if ev.ident == c.sd as libc::uintptr_t {
                ev.flags = 0;
                ev.filter = 0;
                break;
            }
        }

        Ok(())
    }

    pub fn wait(&mut self, timeout: i32) -> Result<i32, EventError> {
        assert!(self.kq > 0);

        let mut ts = timespec {
            tv_sec: (timeout / 1000) as libc::time_t,
            tv_nsec: ((timeout % 1000) * 1_000_000) as libc::c_long,
        };
        let tsp = if timeout < 0 {
            ptr::null()
        } else {
            &mut ts
        };

        loop {
            self.nreturned = unsafe {
                kevent(
                    self.kq,
                    self.changes.as_ptr() as *const libc::kevent,
                    self.nchange as c_int,
                    self.events.as_mut_ptr(),
                    self.events.len() as c_int,
                    tsp,
                )
            };
            self.nchange = 0;

            if self.nreturned > 0 {
                for i in 0..self.nreturned as usize {
                    self.nprocessed = i;
                    let ev = &self.events[i];
                    let mut events = 0;

                    if ev.flags & EV_ERROR != 0 {
                        let data = ev.data as i32;
                        if data == EBADF || data == EINVAL || data == ENOENT || data == EINTR {
                            continue;
                        }
                        events |= EVENT_ERR;
                    }

                    if ev.filter == EVFILT_READ {
                        events |= EVENT_READ;
                    }

                    if ev.filter == EVFILT_WRITE {
                        events |= EVENT_WRITE;
                    }

                    if let Some(ref mut cb) = self.cb {
                        if events != 0 {
                            cb(ev.udata, events);
                        }
                    }
                }
                return Ok(self.nreturned);
            }

            if self.nreturned == 0 {
                if timeout == -1 {
                    return Err(EventError::KeventFailed(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "kevent returned no events",
                    )));
                }
                return Ok(0);
            }

            let err = std::io::Error::last_os_error();
            if err.raw_os_error() == Some(EINTR) {
                continue;
            }

            return Err(EventError::KeventFailed(err));
        }
    }
}

pub fn event_loop_stats(cb: EventStatsCb, st: &mut Stats) -> Result<(), EventError> {
    let kq = unsafe { kqueue() };
    if kq < 0 {
        return Err(EventError::KqueueFailed(std::io::Error::last_os_error()));
    }

    let mut change = unsafe { MaybeUninit::zeroed().assume_init() };
    unsafe {
        libc::EV_SET(
            &mut change,
            st.sd as libc::uintptr_t,
            EVFILT_READ,
            EV_ADD | EV_CLEAR,
            0,
            0,
            ptr::null_mut(),
        );
    }

    let mut ts = timespec {
        tv_sec: (st.interval / 1000) as libc::time_t,
        tv_nsec: ((st.interval % 1000) * 1_000_000) as libc::c_long,
    };
    let tsp = if st.interval < 0 {
        ptr::null()
    } else {
        &mut ts
    };

    let mut event = unsafe { MaybeUninit::zeroed().assume_init() };

    loop {
        let nreturned = unsafe { kevent(kq, &change, 1, &mut event, 1, tsp) };
        if nreturned < 0 {
            let err = std::io::Error::last_os_error();
            if err.raw_os_error() == Some(EINTR) {
                continue;
            }
            unsafe { close(kq) };
            return Err(EventError::KeventFailed(err));
        }

        assert!(nreturned <= 1);

        if nreturned == 1 {
            if event.flags & EV_ERROR != 0 {
                if event.data == EINTR as i64 {
                    continue;
                }
                unsafe { close(kq) };
                return Err(EventError::KeventFailed(std::io::Error::from_raw_os_error(
                    event.data as i32,
                )));
            }
        }

        cb(st, &nreturned);
    }
}

pub struct Conn {
    pub sd: RawFd,
    pub recv_active: bool,
    pub send_active: bool,
}

const EVENT_ERR: u32 = 0x20;
const EVENT_READ: u32 = 0x02;
const EVENT_WRITE: u32 = 0x04;