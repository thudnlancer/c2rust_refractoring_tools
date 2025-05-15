use libc::{c_int, c_uint, c_void, c_char, c_ulong, c_uchar, c_ushort, c_long, size_t};
use std::ptr;
use std::os::unix::io::RawFd;
use std::mem;
use std::ffi::CStr;
use std::io::{Error, Result};

const EPOLLIN: c_uint = 0x001;
const EPOLLOUT: c_uint = 0x004;
const EPOLLERR: c_uint = 0x008;
const EPOLLHUP: c_uint = 0x010;
const EPOLLET: c_uint = 0x80000000;

#[repr(C)]
pub struct EpollEvent {
    pub events: u32,
    pub data: EpollData,
}

#[repr(C)]
pub union EpollData {
    pub ptr: *mut c_void,
    pub fd: c_int,
    pub u32: u32,
    pub u64: u64,
}

pub struct EventBase {
    ep: RawFd,
    events: Vec<EpollEvent>,
    callback: Option<Box<dyn FnMut(*mut c_void, u32)>>,
}

impl EventBase {
    pub fn new(nevent: c_int, cb: Option<Box<dyn FnMut(*mut c_void, u32)>>) -> Result<Self> {
        let ep = unsafe { libc::epoll_create(nevent) };
        if ep < 0 {
            return Err(Error::last_os_error());
        }

        let events = vec![EpollEvent {
            events: 0,
            data: EpollData { u64: 0 },
        }; nevent as usize];

        Ok(EventBase {
            ep,
            events,
            callback: cb,
        })
    }

    pub fn add_in(&mut self, fd: RawFd, conn: *mut c_void) -> Result<()> {
        let mut event = EpollEvent {
            events: EPOLLIN | EPOLLET,
            data: EpollData { ptr: conn },
        };
        self.ctl(libc::EPOLL_CTL_ADD, fd, &mut event)
    }

    pub fn add_out(&mut self, fd: RawFd, conn: *mut c_void) -> Result<()> {
        let mut event = EpollEvent {
            events: EPOLLIN | EPOLLOUT | EPOLLET,
            data: EpollData { ptr: conn },
        };
        self.ctl(libc::EPOLL_CTL_ADD, fd, &mut event)
    }

    pub fn del(&mut self, fd: RawFd) -> Result<()> {
        self.ctl(libc::EPOLL_CTL_DEL, fd, ptr::null_mut())
    }

    pub fn modify(&mut self, fd: RawFd, events: u32, conn: *mut c_void) -> Result<()> {
        let mut event = EpollEvent {
            events,
            data: EpollData { ptr: conn },
        };
        self.ctl(libc::EPOLL_CTL_MOD, fd, &mut event)
    }

    fn ctl(&self, op: c_int, fd: RawFd, event: *mut EpollEvent) -> Result<()> {
        let res = unsafe { libc::epoll_ctl(self.ep, op, fd, event as *mut _) };
        if res < 0 {
            Err(Error::last_os_error())
        } else {
            Ok(())
        }
    }

    pub fn wait(&mut self, timeout: c_int) -> Result<usize> {
        loop {
            let res = unsafe {
                libc::epoll_wait(
                    self.ep,
                    self.events.as_mut_ptr() as *mut _,
                    self.events.len() as c_int,
                    timeout,
                )
            };

            if res < 0 {
                let err = Error::last_os_error();
                if err.raw_os_error() == Some(libc::EINTR) {
                    continue;
                }
                return Err(err);
            }

            if let Some(ref mut cb) = self.callback {
                for i in 0..res as usize {
                    let event = &self.events[i];
                    let mut events = 0u32;
                    
                    if event.events & EPOLLERR != 0 {
                        events |= 0xff0000;
                    }
                    if event.events & (EPOLLIN | EPOLLHUP) != 0 {
                        events |= 0xff;
                    }
                    if event.events & EPOLLOUT != 0 {
                        events |= 0xff00;
                    }

                    cb(unsafe { event.data.ptr }, events);
                }
            }

            return Ok(res as usize);
        }
    }
}

impl Drop for EventBase {
    fn drop(&mut self) {
        if self.ep >= 0 {
            let _ = unsafe { libc::close(self.ep) };
        }
    }
}

pub fn event_loop_stats<F>(interval: c_int, fd: RawFd, mut callback: F) -> Result<()> 
where
    F: FnMut(c_int),
{
    let ep = unsafe { libc::epoll_create(1) };
    if ep < 0 {
        return Err(Error::last_os_error());
    }

    let mut event = EpollEvent {
        events: EPOLLIN,
        data: EpollData { fd },
    };

    if unsafe { libc::epoll_ctl(ep, libc::EPOLL_CTL_ADD, fd, &mut event) } < 0 {
        let err = Error::last_os_error();
        unsafe { libc::close(ep) };
        return Err(err);
    }

    loop {
        let mut event = EpollEvent {
            events: 0,
            data: EpollData { u64: 0 },
        };
        
        let res = unsafe { libc::epoll_wait(ep, &mut event, 1, interval) };
        
        if res < 0 {
            let err = Error::last_os_error();
            if err.raw_os_error() == Some(libc::EINTR) {
                continue;
            }
            unsafe { libc::close(ep) };
            return Err(err);
        }

        if res > 0 {
            callback(res);
        }
    }
}