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

use std::os::unix::io::{RawFd, AsRawFd};
use std::ptr;
use std::mem;
use libc::{epoll_create, epoll_ctl, epoll_wait, close, EPOLLIN, EPOLLOUT, EPOLLERR, EPOLLHUP, EPOLLET, EPOLL_CTL_ADD, EPOLL_CTL_MOD, EPOLL_CTL_DEL};
use std::io::{Error, Result};
use log::{error, debug};

const EVENT_ERR: u32 = 0b001;
const EVENT_READ: u32 = 0b010;
const EVENT_WRITE: u32 = 0b100;

type EventCb = Box<dyn FnMut(*mut Conn, u32)>;
type EventStatsCb = Box<dyn FnMut(&Stats, &i32)>;

struct EventBase {
    ep: RawFd,
    events: Vec<libc::epoll_event>,
    cb: Option<EventCb>,
}

struct Conn {
    sd: RawFd,
    recv_active: bool,
    send_active: bool,
}

struct Stats {
    sd: RawFd,
    interval: i32,
}

impl EventBase {
    fn create(nevent: usize, cb: Option<EventCb>) -> Result<Self> {
        assert!(nevent > 0);

        let ep = unsafe { epoll_create(nevent as i32) };
        if ep < 0 {
            error!("epoll create of size {} failed: {}", nevent, Error::last_os_error());
            return Err(Error::last_os_error());
        }

        let events = vec![unsafe { mem::zeroed() }; nevent];

        debug!("e {} with nevent {}", ep, nevent);

        Ok(EventBase { ep, events, cb })
    }

    fn add_in(&mut self, c: &mut Conn) -> Result<()> {
        assert!(self.ep > 0);
        assert!(c.sd > 0);

        if c.recv_active {
            return Ok(());
        }

        let mut event = libc::epoll_event {
            events: EPOLLIN | EPOLLET,
            u64: c as *mut _ as u64,
        };

        let status = unsafe { epoll_ctl(self.ep, EPOLL_CTL_MOD, c.sd, &mut event) };
        if status < 0 {
            error!("epoll ctl on e {} sd {} failed: {}", self.ep, c.sd, Error::last_os_error());
            Err(Error::last_os_error())
        } else {
            c.recv_active = true;
            Ok(())
        }
    }

    fn del_in(&mut self, _c: &mut Conn) -> Result<()> {
        Ok(())
    }

    fn add_out(&mut self, c: &mut Conn) -> Result<()> {
        assert!(self.ep > 0);
        assert!(c.sd > 0);
        assert!(c.recv_active);

        if c.send_active {
            return Ok(());
        }

        let mut event = libc::epoll_event {
            events: EPOLLIN | EPOLLOUT | EPOLLET,
            u64: c as *mut _ as u64,
        };

        let status = unsafe { epoll_ctl(self.ep, EPOLL_CTL_MOD, c.sd, &mut event) };
        if status < 0 {
            error!("epoll ctl on e {} sd {} failed: {}", self.ep, c.sd, Error::last_os_error());
            Err(Error::last_os_error())
        } else {
            c.send_active = true;
            Ok(())
        }
    }

    fn del_out(&mut self, c: &mut Conn) -> Result<()> {
        assert!(self.ep > 0);
        assert!(c.sd > 0);
        assert!(c.recv_active);

        if !c.send_active {
            return Ok(());
        }

        let mut event = libc::epoll_event {
            events: EPOLLIN | EPOLLET,
            u64: c as *mut _ as u64,
        };

        let status = unsafe { epoll_ctl(self.ep, EPOLL_CTL_MOD, c.sd, &mut event) };
        if status < 0 {
            error!("epoll ctl on e {} sd {} failed: {}", self.ep, c.sd, Error::last_os_error());
            Err(Error::last_os_error())
        } else {
            c.send_active = false;
            Ok(())
        }
    }

    fn add_conn(&mut self, c: &mut Conn) -> Result<()> {
        assert!(self.ep > 0);
        assert!(c.sd > 0);

        let mut event = libc::epoll_event {
            events: EPOLLIN | EPOLLOUT | EPOLLET,
            u64: c as *mut _ as u64,
        };

        let status = unsafe { epoll_ctl(self.ep, EPOLL_CTL_ADD, c.sd, &mut event) };
        if status < 0 {
            error!("epoll ctl on e {} sd {} failed: {}", self.ep, c.sd, Error::last_os_error());
            Err(Error::last_os_error())
        } else {
            c.send_active = true;
            c.recv_active = true;
            Ok(())
        }
    }

    fn del_conn(&mut self, c: &mut Conn) -> Result<()> {
        assert!(self.ep > 0);
        assert!(c.sd > 0);

        let status = unsafe { epoll_ctl(self.ep, EPOLL_CTL_DEL, c.sd, ptr::null_mut()) };
        if status < 0 {
            error!("epoll ctl on e {} sd {} failed: {}", self.ep, c.sd, Error::last_os_error());
            Err(Error::last_os_error())
        } else {
            c.recv_active = false;
            c.send_active = false;
            Ok(())
        }
    }

    fn wait(&mut self, timeout: i32) -> Result<i32> {
        assert!(self.ep > 0);
        assert!(!self.events.is_empty());

        loop {
            let nsd = unsafe { epoll_wait(self.ep, self.events.as_mut_ptr(), self.events.len() as i32, timeout) };
            if nsd > 0 {
                for i in 0..nsd as usize {
                    let ev = &self.events[i];
                    let mut events = 0;

                    debug!("epoll {:04X} triggered on conn {:p}", ev.events, ev.u64 as *mut Conn);

                    if ev.events & EPOLLERR != 0 {
                        events |= EVENT_ERR;
                    }

                    if ev.events & (EPOLLIN | EPOLLHUP) != 0 {
                        events |= EVENT_READ;
                    }

                    if ev.events & EPOLLOUT != 0 {
                        events |= EVENT_WRITE;
                    }

                    if let Some(ref mut cb) = self.cb {
                        cb(ev.u64 as *mut Conn, events);
                    }
                }
                return Ok(nsd);
            }

            if nsd == 0 {
                if timeout == -1 {
                    error!("epoll wait on e {} with {} events and {} timeout returned no events", 
                           self.ep, self.events.len(), timeout);
                    return Err(Error::last_os_error());
                }
                return Ok(0);
            }

            let err = Error::last_os_error();
            if err.raw_os_error() == Some(libc::EINTR) {
                continue;
            }

            error!("epoll wait on e {} with {} events failed: {}", self.ep, self.events.len(), err);
            return Err(err);
        }
    }
}

impl Drop for EventBase {
    fn drop(&mut self) {
        if self.ep > 0 {
            if unsafe { close(self.ep) } < 0 {
                error!("close e {} failed, ignored: {}", self.ep, Error::last_os_error());
            }
            self.ep = -1;
        }
    }
}

fn event_loop_stats(cb: EventStatsCb, arg: &mut Stats) {
    let ep = unsafe { epoll_create(1) };
    if ep < 0 {
        error!("epoll create failed: {}", Error::last_os_error());
        return;
    }

    let mut ev = libc::epoll_event {
        events: EPOLLIN,
        u64: arg.sd as u64,
    };

    let status = unsafe { epoll_ctl(ep, EPOLL_CTL_ADD, arg.sd, &mut ev) };
    if status < 0 {
        error!("epoll ctl on e {} sd {} failed: {}", ep, arg.sd, Error::last_os_error());
        if unsafe { close(ep) } < 0 {
            error!("close e {} failed, ignored: {}", ep, Error::last_os_error());
        }
        return;
    }

    loop {
        let mut ev = unsafe { mem::zeroed() };
        let n = unsafe { epoll_wait(ep, &mut ev, 1, arg.interval) };
        if n < 0 {
            let err = Error::last_os_error();
            if err.raw_os_error() == Some(libc::EINTR) {
                continue;
            }
            error!("epoll wait on e {} with m {} failed: {}", ep, arg.sd, err);
            break;
        }

        cb(arg, &n);
    }

    if unsafe { close(ep) } < 0 {
        error!("close e {} failed, ignored: {}", ep, Error::last_os_error());
    }
}