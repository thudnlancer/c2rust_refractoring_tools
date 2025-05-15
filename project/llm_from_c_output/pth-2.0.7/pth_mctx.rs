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
**  pth_mctx.rs: Pth machine context handling (Rust translation)
*/

use libc::{c_void, sigset_t, SIG_BLOCK, SIG_SETMASK, SIGUSR1, SA_ONSTACK, SS_DISABLE};
use std::mem;
use std::ptr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Once;

/*
 * machine context state structure
 */
#[repr(C)]
pub struct PthMctx {
    uc: ucontext_t,
    restored: i32,
    sigs: sigset_t,
    error: i32,
}

/*
** ____ MACHINE STATE SWITCHING ______________________________________
*/

/*
 * save the current machine context
 */
pub unsafe fn pth_mctx_save(mctx: *mut PthMctx) -> i32 {
    (*mctx).error = errno::errno();
    (*mctx).restored = 0;
    getcontext(&mut (*mctx).uc);
    (*mctx).restored
}

/*
 * restore the current machine context
 * (at the location of the old context)
 */
pub unsafe fn pth_mctx_restore(mctx: *mut PthMctx) {
    errno::set_errno((*mctx).error);
    (*mctx).restored = 1;
    setcontext(&(*mctx).uc);
}

/*
 * switch from one machine context to another
 */
pub unsafe fn pth_mctx_switch(old: *mut PthMctx, new: *mut PthMctx) {
    _pth_mctx_switch_debug();
    swapcontext(&mut (*old).uc, &(*new).uc);
}

/*
** ____ MACHINE STATE INITIALIZATION ________________________________
*/

/*
 * VARIANT 1: THE STANDARDIZED SVR4/SUSv2 APPROACH
 */
pub unsafe fn pth_mctx_set(
    mctx: *mut PthMctx,
    func: extern "C" fn(),
    sk_addr_lo: *mut c_void,
    sk_addr_hi: *mut c_void,
) -> bool {
    if getcontext(&mut (*mctx).uc) != 0 {
        return false;
    }

    (*mctx).uc.uc_link = ptr::null_mut();
    (*mctx).uc.uc_stack.ss_sp = pth_skaddr(makecontext, sk_addr_lo, sk_addr_hi.offset_from(sk_addr_lo) as usize);
    (*mctx).uc.uc_stack.ss_size = pth_sksize(makecontext, sk_addr_lo, sk_addr_hi.offset_from(sk_addr_lo) as usize);
    (*mctx).uc.uc_stack.ss_flags = 0;

    makecontext(&mut (*mctx).uc, func, 0);

    true
}

/*
 * Trampoline and bootstrap functions for signal stack trick
 */
static MCTX_TRAMPOLINE: Once = Once::new();
static mut MCTX_CALLER: PthMctx = unsafe { mem::zeroed() };
static MCTX_CALLED: AtomicBool = AtomicBool::new(false);

static mut MCTX_CREATING: *mut PthMctx = ptr::null_mut();
static mut MCTX_CREATING_FUNC: Option<extern "C" fn()> = None;
static mut MCTX_CREATING_SIGS: sigset_t = unsafe { mem::zeroed() };

extern "C" fn pth_mctx_set_trampoline(_sig: i32) {
    unsafe {
        if setjmp(&mut MCTX_TRAMPOLINE as *mut _ as *mut c_void) == 0 {
            MCTX_CALLED.store(true, Ordering::SeqCst);
            return;
        }

        pth_mctx_set_bootstrap();
    }
}

extern "C" fn pth_mctx_set_bootstrap() {
    unsafe {
        let mctx_starting = MCTX_CREATING;
        let mctx_starting_func = MCTX_CREATING_FUNC.unwrap();

        pth_sc_sigprocmask(SIG_SETMASK, &MCTX_CREATING_SIGS, ptr::null_mut());
        pth_mctx_switch(mctx_starting, &mut MCTX_CALLER);

        mctx_starting_func();
        std::process::abort();
    }
}

/*
 * VARIANT 2: THE SIGNAL STACK TRICK
 */
pub unsafe fn pth_mctx_set_signal_stack(
    mctx: *mut PthMctx,
    func: extern "C" fn(),
    sk_addr_lo: *mut c_void,
    sk_addr_hi: *mut c_void,
) -> bool {
    let mut sa: libc::sigaction = mem::zeroed();
    let mut osa: libc::sigaction = mem::zeroed();
    let mut ss: libc::stack_t = mem::zeroed();
    let mut oss: libc::stack_t = mem::zeroed();
    let mut osigs: sigset_t = mem::zeroed();
    let mut sigs: sigset_t = mem::zeroed();

    libc::sigemptyset(&mut sigs);
    libc::sigaddset(&mut sigs, SIGUSR1);
    pth_sc_sigprocmask(SIG_BLOCK, &sigs, &mut osigs);

    sa.sa_handler = pth_mctx_set_trampoline;
    libc::sigemptyset(&mut sa.sa_mask);
    sa.sa_flags = SA_ONSTACK;

    if libc::sigaction(SIGUSR1, &sa, &mut osa) != 0 {
        return false;
    }

    ss.ss_sp = pth_skaddr(sigaltstack, sk_addr_lo, sk_addr_hi.offset_from(sk_addr_lo) as usize);
    ss.ss_size = pth_sksize(sigaltstack, sk_addr_lo, sk_addr_hi.offset_from(sk_addr_lo) as usize);
    ss.ss_flags = 0;

    if libc::sigaltstack(&ss, &mut oss) < 0 {
        return false;
    }

    MCTX_CALLED.store(false, Ordering::SeqCst);
    libc::kill(libc::getpid(), SIGUSR1);

    libc::sigfillset(&mut sigs);
    libc::sigdelset(&mut sigs, SIGUSR1);
    while !MCTX_CALLED.load(Ordering::SeqCst) {
        libc::sigsuspend(&sigs);
    }

    libc::sigaltstack(ptr::null(), &mut ss);
    ss.ss_flags = SS_DISABLE;
    if libc::sigaltstack(&ss, ptr::null_mut()) < 0 {
        return false;
    }

    libc::sigaltstack(ptr::null(), &mut ss);
    if (ss.ss_flags & SS_DISABLE) == 0 {
        return false;
    }
    if (oss.ss_flags & SS_DISABLE) == 0 {
        libc::sigaltstack(&oss, ptr::null_mut());
    }

    libc::sigaction(SIGUSR1, &osa, ptr::null_mut());
    pth_sc_sigprocmask(SIG_SETMASK, &osigs, ptr::null_mut());

    libc::sigemptyset(&mut (*mctx).sigs);
    (*mctx).error = 0;

    MCTX_CREATING = mctx;
    MCTX_CREATING_FUNC = Some(func);
    MCTX_CREATING_SIGS = osigs;

    if pth_mctx_save(&mut MCTX_CALLER as *mut _) == 0 {
        libc::longjmp(&mut MCTX_TRAMPOLINE as *mut _ as *mut c_void, 1);
    }

    true
}