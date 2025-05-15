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
**  pth_util.c: Pth utility functions
*/
                             /* ``Premature optimization is
                                  the root of all evil.''
                                             -- D.E.Knuth */

use nix::sys::signal::{sigaction, SigAction, SigHandler, SigSet, Signal};
use nix::sys::signal::{sigpending, sigprocmask, SigProcMask};
use nix::unistd;
use std::os::unix::io::AsRawFd;
use std::os::unix::prelude::RawFd;
use std::io::{Error, ErrorKind, Result};

/* calculate numerical minimum */
pub fn pth_util_min<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
    if a > b { b } else { a }
}

/* delete a pending signal */
extern "C" fn pth_util_sigdelete_sighandler(_sig: i32) {
    /* nop */
}

pub fn pth_util_sigdelete(sig: Signal) -> Result<bool> {
    /* check status of signal */
    let pending = sigpending()?;
    if !pending.contains(sig) {
        return Ok(false);
    }

    /* block signal and remember old mask */
    let mut ss = SigSet::empty();
    ss.add(sig);
    let old_ss = sigprocmask(SigProcMask::SIG_BLOCK, Some(&ss), None)?;

    /* set signal action to our dummy handler */
    let sa = SigAction::new(
        SigHandler::Handler(pth_util_sigdelete_sighandler),
        SigSet::all(),
        0,
    );
    let old_sa = sigaction(sig, &sa)?;

    /* now let signal be delivered */
    let mut ss = SigSet::all();
    ss.remove(sig);
    unistd::sigsuspend(&ss);

    /* restore signal mask and handler */
    sigaction(sig, &old_sa)?;
    sigprocmask(SigProcMask::SIG_SETMASK, Some(&old_ss), None)?;
    Ok(true)
}

/* copy a string like strncpy() but always null-terminate */
pub fn pth_util_cpystrn(dst: &mut [u8], src: &[u8]) -> &mut [u8] {
    if dst.is_empty() {
        return dst;
    }

    let mut i = 0;
    let end = dst.len() - 1;
    while i < end && i < src.len() {
        dst[i] = src[i];
        if src[i] == b'\0' {
            return &mut dst[..=i];
        }
        i += 1;
    }
    dst[i] = b'\0';
    &mut dst[..=i]
}

/* check whether a file-descriptor is valid */
pub fn pth_util_fd_valid(fd: RawFd) -> bool {
    if fd < 0 || fd >= libc::FD_SETSIZE as RawFd {
        return false;
    }
    match nix::fcntl::fcntl(fd, nix::fcntl::FcntlArg::F_GETFL) {
        Ok(_) => true,
        Err(nix::errno::Errno::EBADF) => false,
        Err(_) => true, // Other errors might indicate the fd is valid
    }
}

/* merge input fd set into output fds */
pub fn pth_util_fds_merge(
    nfd: usize,
    ifds1: Option<&nix::sys::select::FdSet>,
    ofds1: &mut nix::sys::select::FdSet,
    ifds2: Option<&nix::sys::select::FdSet>,
    ofds2: &mut nix::sys::select::FdSet,
    ifds3: Option<&nix::sys::select::FdSet>,
    ofds3: &mut nix::sys::select::FdSet,
) {
    for s in 0..nfd {
        if let Some(ifds) = ifds1 {
            if ifds.contains(s as RawFd) {
                ofds1.insert(s as RawFd);
            }
        }
        if let Some(ifds) = ifds2 {
            if ifds.contains(s as RawFd) {
                ofds2.insert(s as RawFd);
            }
        }
        if let Some(ifds) = ifds3 {
            if ifds.contains(s as RawFd) {
                ofds3.insert(s as RawFd);
            }
        }
    }
}

/* test whether fds in the input fd sets occurred in the output fds */
pub fn pth_util_fds_test(
    nfd: usize,
    ifds1: Option<&nix::sys::select::FdSet>,
    ofds1: &nix::sys::select::FdSet,
    ifds2: Option<&nix::sys::select::FdSet>,
    ofds2: &nix::sys::select::FdSet,
    ifds3: Option<&nix::sys::select::FdSet>,
    ofds3: &nix::sys::select::FdSet,
) -> bool {
    for s in 0..nfd {
        if let Some(ifds) = ifds1 {
            if ifds.contains(s as RawFd) && ofds1.contains(s as RawFd) {
                return true;
            }
        }
        if let Some(ifds) = ifds2 {
            if ifds.contains(s as RawFd) && ofds2.contains(s as RawFd) {
                return true;
            }
        }
        if let Some(ifds) = ifds3 {
            if ifds.contains(s as RawFd) && ofds3.contains(s as RawFd) {
                return true;
            }
        }
    }
    false
}

/*
 * clear fds in input fd sets if not occurred in output fd sets and return
 * number of remaining input fds. This number uses BSD select(2) semantics: a
 * fd in two set counts twice!
 */
pub fn pth_util_fds_select(
    nfd: usize,
    ifds1: &mut nix::sys::select::FdSet,
    ofds1: &nix::sys::select::FdSet,
    ifds2: &mut nix::sys::select::FdSet,
    ofds2: &nix::sys::select::FdSet,
    ifds3: &mut nix::sys::select::FdSet,
    ofds3: &nix::sys::select::FdSet,
) -> usize {
    let mut n = 0;
    for s in 0..nfd {
        let fd = s as RawFd;
        if ifds1.contains(fd) {
            if !ofds1.contains(fd) {
                ifds1.remove(fd);
            } else {
                n += 1;
            }
        }
        if ifds2.contains(fd) {
            if !ofds2.contains(fd) {
                ifds2.remove(fd);
            } else {
                n += 1;
            }
        }
        if ifds3.contains(fd) {
            if !ofds3.contains(fd) {
                ifds3.remove(fd);
            } else {
                n += 1;
            }
        }
    }
    n
}