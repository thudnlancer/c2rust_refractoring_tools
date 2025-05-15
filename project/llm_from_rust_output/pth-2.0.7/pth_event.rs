use std::mem;
use std::ptr;
use std::sync::atomic::{AtomicPtr, Ordering};
use std::time::{Duration, Instant};

type PthStatus = u32;
const PTH_STATUS_PENDING: PthStatus = 0;
const PTH_STATUS_OCCURRED: PthStatus = 1;
const PTH_STATUS_FAILED: PthStatus = 2;

type PthState = u32;
const PTH_STATE_SCHEDULER: PthState = 0;
const PTH_STATE_NEW: PthState = 1;
const PTH_STATE_READY: PthState = 2;
const PTH_STATE_WAITING: PthState = 3;
const PTH_STATE_DEAD: PthState = 4;

#[derive(Debug, Clone)]
pub struct PthEvent {
    ev_next: Option<Box<PthEvent>>,
    ev_prev: Option<Box<PthEvent>>,
    ev_status: PthStatus,
    ev_type: i32,
    ev_goal: i32,
    ev_args: PthEventArgs,
}

#[derive(Debug, Clone)]
pub enum PthEventArgs {
    FD { fd: i32 },
    Select {
        n: Option<Box<i32>>,
        nfd: i32,
        rfds: Option<Box<libc::fd_set>>,
        wfds: Option<Box<libc::fd_set>>,
        efds: Option<Box<libc::fd_set>>,
    },
    Sig {
        sigs: Option<Box<libc::sigset_t>>,
        sig: Option<Box<i32>>,
    },
    Time { tv: libc::timeval },
    Msg { mp: Option<Box<libc::pth_msgport_st>> },
    Mutex { mutex: Option<Box<libc::pth_mutex_st>> },
    Cond { cond: Option<Box<libc::pth_cond_st>> },
    Tid { tid: Option<Box<libc::pth_st>> },
    Func {
        func: Option<extern "C" fn(*mut libc::c_void) -> i32>,
        arg: Option<Box<libc::c_void>>,
        tv: libc::timeval,
    },
}

impl PthEvent {
    pub fn new(spec: u64) -> Result<Self, i32> {
        let mut ev = PthEvent {
            ev_next: None,
            ev_prev: None,
            ev_status: PTH_STATUS_PENDING,
            ev_type: 0,
            ev_goal: 0,
            ev_args: PthEventArgs::FD { fd: 0 },
        };

        if spec & (1 << 20) != 0 {
            return Err(libc::EINVAL);
        }

        ev.ev_prev = Some(Box::new(ev.clone()));
        ev.ev_next = Some(Box::new(ev.clone()));

        Ok(ev)
    }

    pub fn typeof_event(&self) -> u64 {
        (self.ev_type | self.ev_goal) as u64
    }

    pub fn isolate(&mut self) -> Option<Box<PthEvent>> {
        if self.ev_next.is_none() || self.ev_prev.is_none() {
            return None;
        }

        let mut ring = None;
        if !(ptr::eq(self.ev_next.as_ref().unwrap().as_ref(), self)
            && ptr::eq(self.ev_prev.as_ref().unwrap().as_ref(), self))
        {
            ring = self.ev_next.take();
            self.ev_prev.as_mut().unwrap().ev_next = self.ev_next.take();
            self.ev_next.as_mut().unwrap().ev_prev = self.ev_prev.take();
            self.ev_prev = Some(Box::new(self.clone()));
            self.ev_next = Some(Box::new(self.clone()));
        }
        ring
    }

    pub fn status(&self) -> PthStatus {
        self.ev_status
    }

    pub fn free(&mut self, mode: i32) -> Result<(), i32> {
        if mode == 0 {
            if let Some(prev) = &mut self.ev_prev {
                prev.ev_next = self.ev_next.take();
            }
            if let Some(next) = &mut self.ev_next {
                next.ev_prev = self.ev_prev.take();
            }
        } else if mode == 1 {
            let mut current = self.clone();
            loop {
                let next = current.ev_next.take();
                if next.is_none() || ptr::eq(next.as_ref().unwrap().as_ref(), self) {
                    break;
                }
                current = *next.unwrap();
            }
        }
        Ok(())
    }
}

pub struct PthCurrent {
    events: Option<Box<PthEvent>>,
    state: PthState,
}

static PTH_CURRENT: AtomicPtr<PthCurrent> = AtomicPtr::new(ptr::null_mut());

pub fn pth_wait(ev_ring: Option<Box<PthEvent>>) -> Result<i32, i32> {
    let mut nonpending = 0;
    let mut current = unsafe { &mut *PTH_CURRENT.load(Ordering::SeqCst) };

    if ev_ring.is_none() {
        return Err(libc::EINVAL);
    }

    let mut ev = ev_ring.unwrap();
    loop {
        ev.ev_status = PTH_STATUS_PENDING;
        if ev.ev_next.is_none() {
            break;
        }
        ev = ev.ev_next.unwrap();
        if ptr::eq(ev.as_ref(), ev_ring.as_ref().unwrap().as_ref()) {
            break;
        }
    }

    current.events = Some(ev_ring.unwrap());
    current.state = PTH_STATE_WAITING;

    // Simulate yield and cancel point
    std::thread::yield_now();
    
    current.events = None;

    let mut ev = current.events.as_ref().unwrap();
    loop {
        if ev.ev_status != PTH_STATUS_PENDING {
            nonpending += 1;
        }
        if ev.ev_next.is_none() {
            break;
        }
        ev = ev.ev_next.as_ref().unwrap();
        if ptr::eq(ev.as_ref(), current.events.as_ref().unwrap().as_ref()) {
            break;
        }
    }

    Ok(nonpending)
}