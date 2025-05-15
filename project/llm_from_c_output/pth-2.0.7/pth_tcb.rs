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
**  pth_tcb.c: Pth thread control block handling
*/
                             /* Patient: Doctor, it hurts when I do this!
                                Doctor: Well, then don't do it. */

use std::alloc::{alloc, dealloc, Layout};
use std::ptr;
use std::ffi::CString;
use libc::{sigset_t, c_void};

const PTH_TCB_NAMELEN: usize = 40;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PthState {
    Scheduler,
    New,
    Ready,
    Running,
    Waiting,
    Dead,
}

pub type PthTimeT = u64;
pub type PthEventT = *mut c_void;
pub type PthMctxT = *mut c_void;
pub type PthCleanupT = *mut c_void;
pub type PthRingT = *mut c_void;

pub struct Pth {
    // priority queue handling
    pub q_next: Option<Box<Pth>>,         // next thread in pool
    pub q_prev: Option<Box<Pth>>,         // previous thread in pool
    pub q_prio: i32,                      // (relative) priority of thread when queued

    // standard thread control block ingredients
    pub prio: i32,                        // base priority of thread
    pub name: [u8; PTH_TCB_NAMELEN],      // name of thread (mainly for debugging)
    pub dispatches: i32,                   // total number of thread dispatches
    pub state: PthState,                   // current state indicator for thread

    // timing
    pub spawned: PthTimeT,                 // time point at which thread was spawned
    pub lastran: PthTimeT,                 // time point at which thread was last running
    pub running: PthTimeT,                 // time range the thread was already running

    // event handling
    pub events: PthEventT,                 // events the thread is waiting for

    // per-thread signal handling
    pub sigpending: sigset_t,              // set of pending signals
    pub sigpendcnt: i32,                   // number of pending signals

    // machine context
    pub mctx: PthMctxT,                    // last saved machine state of thread
    pub stack: *mut u8,                    // pointer to thread stack
    pub stacksize: u32,                    // size of thread stack
    pub stackguard: *mut i64,              // stack overflow guard
    pub stackloan: bool,                   // stack type
    pub start_func: Option<extern "C" fn(*mut c_void) -> *mut c_void>, // start routine
    pub start_arg: *mut c_void,            // start argument

    // thread joining
    pub joinable: bool,                    // whether thread is joinable
    pub join_arg: *mut c_void,             // joining argument

    // per-thread specific storage
    pub data_value: *mut *const c_void,    // thread specific values
    pub data_count: i32,                   // number of stored values

    // cancellation support
    pub cancelreq: bool,                   // cancellation request is pending
    pub cancelstate: u32,                  // cancellation state of thread
    pub cleanups: *mut PthCleanupT,        // stack of thread cleanup handlers

    // mutex ring
    pub mutexring: PthRingT,               // ring of acquired mutex structures
}

pub const PTH_STATE_NAMES: [&str; 6] = [
    "scheduler", "new", "ready", "running", "waiting", "dead"
];

const SIGSTKSZ: usize = 8192;

/// Allocate a thread control block
pub fn pth_tcb_alloc(stacksize: u32, stackaddr: *mut u8) -> Option<Box<Pth>> {
    let mut stacksize = stacksize;
    if stacksize > 0 && stacksize < SIGSTKSZ as u32 {
        stacksize = SIGSTKSZ as u32;
    }

    let mut tcb = Box::new(Pth {
        q_next: None,
        q_prev: None,
        q_prio: 0,
        prio: 0,
        name: [0; PTH_TCB_NAMELEN],
        dispatches: 0,
        state: PthState::New,
        spawned: 0,
        lastran: 0,
        running: 0,
        events: ptr::null_mut(),
        sigpending: 0,
        sigpendcnt: 0,
        mctx: ptr::null_mut(),
        stack: ptr::null_mut(),
        stacksize,
        stackguard: ptr::null_mut(),
        stackloan: !stackaddr.is_null(),
        start_func: None,
        start_arg: ptr::null_mut(),
        joinable: false,
        join_arg: ptr::null_mut(),
        data_value: ptr::null_mut(),
        data_count: 0,
        cancelreq: false,
        cancelstate: 0,
        cleanups: ptr::null_mut(),
        mutexring: ptr::null_mut(),
    });

    if stacksize > 0 {
        tcb.stack = if !stackaddr.is_null() {
            stackaddr
        } else {
            let layout = Layout::from_size_align(stacksize as usize, 1).unwrap();
            unsafe { alloc(layout) }
        };

        if tcb.stack.is_null() {
            return None;
        }

        #[cfg(PTH_STACKGROWTH < 0)]
        {
            tcb.stackguard = tcb.stack as *mut i64;
        }
        #[cfg(not(PTH_STACKGROWTH < 0))]
        {
            tcb.stackguard = unsafe { tcb.stack.add((stacksize as usize / std::mem::size_of::<i64>() - 1) * std::mem::size_of::<i64>()) } as *mut i64;
        }

        if !tcb.stackguard.is_null() {
            unsafe { *tcb.stackguard = 0xDEAD };
        }
    }

    Some(tcb)
}

/// Free a thread control block
pub fn pth_tcb_free(tcb: Option<Box<Pth>>) {
    if let Some(t) = tcb {
        if !t.stack.is_null() && !t.stackloan {
            let layout = Layout::from_size_align(t.stacksize as usize, 1).unwrap();
            unsafe { dealloc(t.stack, layout) };
        }
        
        if !t.data_value.is_null() {
            let layout = Layout::array::<*const c_void>(t.data_count as usize).unwrap();
            unsafe { dealloc(t.data_value as *mut u8, layout) };
        }
        
        if !t.cleanups.is_null() {
            // pth_cleanup_popall(t, false);
        }
    }
}