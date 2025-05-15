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
**  pth_uctx.c: Pth user-space context handling (stand-alone sub-API)
*/
                             /* ``It worries me however, to realize
                                how tough an ass-hole I have had to
                                be, in order to get to stick to the
                                principle of doing things right,
                                rather than "just hack it in".''
                                -- Poul-Henning Kamp <phk@FreeBSD.org> */

use std::alloc::{self, Layout};
use std::ptr;
use std::mem;
use std::process;
use libc::{sigset_t, sigprocmask, SIG_SETMASK};

pub type PthMctx = [u8; 0]; // Placeholder for actual machine context type

#[derive(Debug)]
pub struct PthUctx {
    uc_stack_own: bool,
    uc_stack_ptr: Option<Vec<u8>>,
    uc_stack_len: usize,
    uc_mctx_set: bool,
    uc_mctx: PthMctx,
}

pub type PthUctxTrampoline = Box<dyn FnOnce()>;

static mut PTH_UCTX_TRAMPOLINE_CTX: Option<PthUctxTrampoline> = None;

pub fn pth_uctx_create() -> Result<Box<PthUctx>, i32> {
    let uctx = Box::new(PthUctx {
        uc_stack_own: false,
        uc_stack_ptr: None,
        uc_stack_len: 0,
        uc_mctx_set: false,
        uc_mctx: [0; 0],
    });

    Ok(uctx)
}

fn pth_uctx_trampoline() {
    unsafe {
        if let Some(ctx) = PTH_UCTX_TRAMPOLINE_CTX.take() {
            ctx();
        }
    }
}

pub fn pth_uctx_make(
    mut uctx: Box<PthUctx>,
    sk_addr: Option<Vec<u8>>,
    sk_size: usize,
    sigmask: Option<sigset_t>,
    start_func: Box<dyn FnOnce()>,
    uctx_after: Option<Box<PthUctx>>,
) -> Result<Box<PthUctx>, i32> {
    if sk_size < 16 * 1024 {
        return Err(libc::EINVAL);
    }

    let stack = match sk_addr {
        Some(stack) => {
            uctx.uc_stack_own = false;
            stack
        }
        None => {
            let layout = Layout::from_size_align(sk_size, 16).unwrap();
            let stack = unsafe { alloc::alloc(layout) };
            if stack.is_null() {
                return Err(libc::ENOMEM);
            }
            uctx.uc_stack_own = true;
            unsafe { Vec::from_raw_parts(stack, sk_size, sk_size) }
        }
    };

    uctx.uc_stack_ptr = Some(stack);
    uctx.uc_stack_len = sk_size;

    // Placeholder for actual context setup
    uctx.uc_mctx = [0; 0];

    unsafe {
        PTH_UCTX_TRAMPOLINE_CTX = Some(Box::new(move || {
            start_func();
            if let Some(after) = uctx_after {
                // Restore context
            }
            process::exit(0);
        }));
    }

    // Placeholder for signal mask handling
    if let Some(mask) = sigmask {
        unsafe {
            let mut oldset: sigset_t = mem::zeroed();
            sigprocmask(SIG_SETMASK, &mask, &mut oldset);
        }
    }

    uctx.uc_mctx_set = true;
    Ok(uctx)
}

pub fn pth_uctx_switch(
    mut uctx_from: Box<PthUctx>,
    uctx_to: &mut PthUctx,
) -> Result<(), i32> {
    if !uctx_to.uc_mctx_set {
        return Err(libc::EPERM);
    }

    uctx_from.uc_mctx_set = true;
    // Placeholder for actual context switch
    Ok(())
}

pub fn pth_uctx_destroy(mut uctx: Box<PthUctx>) -> Result<(), i32> {
    if uctx.uc_stack_own {
        if let Some(stack) = uctx.uc_stack_ptr.take() {
            drop(stack);
        }
    }
    Ok(())
}