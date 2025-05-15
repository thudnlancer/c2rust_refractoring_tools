/*
 * twemproxy - A fast and lightweight proxy for memcached protocol.
 * Copyright (C) 2013 Twitter, Inc.
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

use libc::{c_int, c_void, close, pollfd, POLLIN, POLLOUT, POLLERR, EINTR, EAGAIN, ETIME, ENOENT};
use std::ptr;
use std::time::{Duration, Instant};
use std::os::unix::io::RawFd;
use std::mem;
use std::ffi::CStr;
use std::io::{Error, ErrorKind};

#[cfg(target_os = "solaris")]
use libc::{port_create, port_associate, port_dissociate, port_getn, port_event_t, PORT_SOURCE_FD};

type EventCb = Box<dyn FnMut(*mut c_void, u32) -> c_int>;
type EventStatsCb = Box<dyn FnMut(*mut Stats, *mut u32)>;

struct EventBase {
    evp: RawFd,
    events: Vec<port_event_t>,
    cb: Option<EventCb>,
}

struct Conn {
    sd: RawFd,
    recv_active: bool,
    send_active: bool,
}

struct Stats {
    sd: RawFd,
    interval: i64,
}

impl EventBase {
    fn create(nevent: usize, cb: Option<EventCb>) -> Result<Self, Error> {
        assert!(nevent > 0);

        let evp = unsafe { port_create() };
        if evp < 0 {
            return Err(Error::last_os_error());
        }

        let events = vec![unsafe { mem::zeroed() }; nevent];

        Ok(EventBase {
            evp,
            events,
            cb,
        })
    }

    fn destroy(&mut self) -> Result<(), Error> {
        if self.evp < 0 {
            return Ok(());
        }

        unsafe {
            if close(self.evp) < 0 {
                return Err(Error::last_os_error());
            }
        }
        self.evp = -1;
        Ok(())
    }

    fn add_out(&mut self, c: &mut Conn) -> Result<(), Error> {
        assert!(self.evp > 0);
        assert!(c.sd > 0);
        assert!(c.recv_active);

        if c.send_active {
            return Ok(());
        }

        let status = unsafe {
            port_associate(
                self.evp,
                PORT_SOURCE_FD,
                c.sd,
                POLLIN | POLLOUT,
                c as *mut _ as *mut c_void,
            )
        };

        if status < 0 {
            Err(Error::last_os_error())
        } else {
            c.send_active = true;
            Ok(())
        }
    }

    fn del_out(&mut self, c: &mut Conn) -> Result<(), Error> {
        assert!(self.evp > 0);
        assert!(c.sd > 0);
        assert!(c.recv_active);

        if !c.send_active {
            return Ok(());
        }

        let status = unsafe {
            port_associate(
                self.evp,
                PORT_SOURCE_FD,
                c.sd,
                POLLIN,
                c as *mut _ as *mut c_void,
            )
        };

        if status < 0 {
            Err(Error::last_os_error())
        } else {
            c.send_active = false;
            Ok(())
        }
    }

    fn add_conn(&mut self, c: &mut Conn) -> Result<(), Error> {
        assert!(self.evp > 0);
        assert!(c.sd > 0);
        assert!(!c.recv_active);
        assert!(!c.send_active);

        let status = unsafe {
            port_associate(
                self.evp,
                PORT_SOURCE_FD,
                c.sd,
                POLLIN | POLLOUT,
                c as *mut _ as *mut c_void,
            )
        };

        if status < 0 {
            Err(Error::last_os_error())
        } else {
            c.send_active = true;
            c.recv_active = true;
            Ok(())
        }
    }

    fn del_conn(&mut self, c: &mut Conn) -> Result<(), Error> {
        assert!(self.evp > 0);
        assert!(c.sd > 0);

        if !c.send_active && !c.recv_active {
            return Ok(());
        }

        let status = unsafe { port_dissociate(self.evp, PORT_SOURCE_FD, c.sd) };
        if status < 0 {
            let err = Error::last_os_error();
            if err.raw_os_error() != Some(ENOENT) {
                return Err(err);
            }
        }

        c.recv_active = false;
        c.send_active = false;
        Ok(())
    }

    fn reassociate(&mut self, c: &mut Conn) -> Result<(), Error> {
        assert!(self.evp > 0);
        assert!(c.sd > 0);
        assert!(c.recv_active);

        let events = if c.send_active {
            POLLIN | POLLOUT
        } else {
            POLLIN
        };

        let status = unsafe {
            port_associate(
                self.evp,
                PORT_SOURCE_FD,
                c.sd,
                events,
                c as *mut _ as *mut c_void,
            )
        };

        if status < 0 {
            Err(Error::last_os_error())
        } else {
            Ok(())
        }
    }

    fn wait(&mut self, timeout: i64) -> Result<usize, Error> {
        assert!(self.evp > 0);
        assert!(!self.events.is_empty());

        let mut tsp = None;
        let mut ts = unsafe { mem::zeroed() };

        if timeout >= 0 {
            ts.tv_sec = timeout / 1000;
            ts.tv_nsec = (timeout % 1000) * 1000000;
            tsp = Some(&ts);
        }

        loop {
            let mut nreturned = 1;
            let status = unsafe {
                port_getn(
                    self.evp,
                    self.events.as_mut_ptr(),
                    self.events.len() as u32,
                    &mut nreturned,
                    tsp.map(|p| p as *const _).unwrap_or(ptr::null()),
                )
            };

            if status < 0 {
                let err = Error::last_os_error();
                if err.raw_os_error() == Some(EINTR) || err.raw_os_error() == Some(EAGAIN) {
                    continue;
                }
                if err.raw_os_error() != Some(ETIME) {
                    return Err(err);
                }
            }

            if nreturned > 0 {
                for i in 0..nreturned {
                    let ev = &self.events[i];
                    let mut events = 0;

                    if ev.portev_events & POLLERR != 0 {
                        events |= 0x01; // EVENT_ERR
                    }
                    if ev.portev_events & POLLIN != 0 {
                        events |= 0x02; // EVENT_READ
                    }
                    if ev.portev_events & POLLOUT != 0 {
                        events |= 0x04; // EVENT_WRITE
                    }

                    if let Some(ref mut cb) = self.cb {
                        if events != 0 {
                            let status = cb(ev.portev_user, events);
                            if status < 0 {
                                continue;
                            }
                            self.reassociate(unsafe { &mut *(ev.portev_user as *mut Conn) })?;
                        }
                    }
                }
                return Ok(nreturned as usize);
            }

            if timeout == -1 {
                return Err(Error::new(ErrorKind::Other, "no events returned"));
            }
            return Ok(0);
        }
    }
}

fn event_loop_stats(cb: EventStatsCb, arg: &mut Stats) {
    let evp = unsafe { port_create() };
    if evp < 0 {
        return;
    }

    let status = unsafe { port_associate(evp, PORT_SOURCE_FD, arg.sd, POLLIN, ptr::null_mut()) };
    if status < 0 {
        unsafe { close(evp) };
        return;
    }

    let mut tsp = None;
    let mut ts = unsafe { mem::zeroed() };

    if arg.interval >= 0 {
        ts.tv_sec = arg.interval / 1000;
        ts.tv_nsec = (arg.interval % 1000) * 1000000;
        tsp = Some(&ts);
    }

    loop {
        let mut event = unsafe { mem::zeroed() };
        let mut nreturned = 1;

        let status = unsafe {
            port_getn(
                evp,
                &mut event,
                1,
                &mut nreturned,
                tsp.map(|p| p as *const _).unwrap_or(ptr::null()),
            )
        };

        if status != 0 {
            let err = Error::last_os_error();
            if err.raw_os_error() == Some(EINTR) || err.raw_os_error() == Some(EAGAIN) {
                continue;
            }
            if err.raw_os_error() != Some(ETIME) {
                break;
            }
        }

        assert!(nreturned <= 1);

        if nreturned == 1 {
            unsafe {
                port_associate(evp, PORT_SOURCE_FD, arg.sd, POLLIN, ptr::null_mut());
            }
        }

        cb(arg, &mut nreturned);
    }

    unsafe { close(evp) };
}