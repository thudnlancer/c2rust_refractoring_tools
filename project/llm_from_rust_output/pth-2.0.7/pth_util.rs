use nix::errno::Errno;
use nix::fcntl;
use nix::sys::signal::{self, SigSet, Signal};
use nix::sys::time::TimeVal;
use std::ffi::CStr;
use std::os::unix::io::RawFd;
use std::ptr;

pub fn pth_util_sigdelete(sig: Signal) -> Result<(), Errno> {
    let pending = SigSet::pending();
    if !pending.contains(sig) {
        return Ok(());
    }

    let mut mask = SigSet::empty();
    mask.add(sig);

    let old_mask = SigSet::thread_block_mask(&mask)?;

    let handler = signal::SigHandler::Handler(pth_util_sigdelete_sighandler);
    let mut action = signal::SigAction::new(
        handler,
        signal::SaFlags::empty(),
        SigSet::all(),
    );

    let old_action = signal::sigaction(sig, &action)?;

    let mut suspend_mask = SigSet::all();
    suspend_mask.remove(sig);
    signal::sigsuspend(&suspend_mask)?;

    signal::sigaction(sig, &old_action)?;
    signal::sigprocmask(signal::SigmaskHow::SIG_SETMASK, Some(&old_mask), None)?;

    Ok(())
}

extern "C" fn pth_util_sigdelete_sighandler(_sig: libc::c_int) {}

pub fn pth_util_cpystrn(dst: &mut [u8], src: &[u8]) -> &mut [u8] {
    if dst.is_empty() {
        return dst;
    }

    let mut src_iter = src.iter();
    for d in dst.iter_mut().take(dst.len() - 1) {
        *d = *src_iter.next().unwrap_or(&0);
        if *d == 0 {
            return dst;
        }
    }

    *dst.last_mut().unwrap() = 0;
    dst
}

pub fn pth_util_fd_valid(fd: RawFd) -> bool {
    if fd < 0 || fd >= 1024 {
        return false;
    }

    match fcntl::fcntl(fd, fcntl::F_GETFL) {
        Ok(_) => true,
        Err(e) => e != Errno::EBADF,
    }
}

pub fn pth_util_fds_merge(
    nfd: usize,
    ifds1: Option<&libc::fd_set>,
    ofds1: Option<&mut libc::fd_set>,
    ifds2: Option<&libc::fd_set>,
    ofds2: Option<&mut libc::fd_set>,
    ifds3: Option<&libc::fd_set>,
    ofds3: Option<&mut libc::fd_set>,
) {
    for s in 0..nfd {
        if let (Some(ifds), Some(ofds)) = (ifds1, ofds1) {
            if unsafe { libc::FD_ISSET(s as i32, ifds) } {
                unsafe { libc::FD_SET(s as i32, ofds) };
            }
        }

        if let (Some(ifds), Some(ofds)) = (ifds2, ofds2) {
            if unsafe { libc::FD_ISSET(s as i32, ifds) } {
                unsafe { libc::FD_SET(s as i32, ofds) };
            }
        }

        if let (Some(ifds), Some(ofds)) = (ifds3, ofds3) {
            if unsafe { libc::FD_ISSET(s as i32, ifds) } {
                unsafe { libc::FD_SET(s as i32, ofds) };
            }
        }
    }
}

pub fn pth_util_fds_test(
    nfd: usize,
    ifds1: Option<&libc::fd_set>,
    ofds1: Option<&libc::fd_set>,
    ifds2: Option<&libc::fd_set>,
    ofds2: Option<&libc::fd_set>,
    ifds3: Option<&libc::fd_set>,
    ofds3: Option<&libc::fd_set>,
) -> bool {
    for s in 0..nfd {
        if let (Some(ifds), Some(ofds)) = (ifds1, ofds1) {
            if unsafe { libc::FD_ISSET(s as i32, ifds) } && unsafe { libc::FD_ISSET(s as i32, ofds) } {
                return true;
            }
        }

        if let (Some(ifds), Some(ofds)) = (ifds2, ofds2) {
            if unsafe { libc::FD_ISSET(s as i32, ifds) } && unsafe { libc::FD_ISSET(s as i32, ofds) } {
                return true;
            }
        }

        if let (Some(ifds), Some(ofds)) = (ifds3, ofds3) {
            if unsafe { libc::FD_ISSET(s as i32, ifds) } && unsafe { libc::FD_ISSET(s as i32, ofds) } {
                return true;
            }
        }
    }
    false
}

pub fn pth_util_fds_select(
    nfd: usize,
    ifds1: Option<&mut libc::fd_set>,
    ofds1: Option<&libc::fd_set>,
    ifds2: Option<&mut libc::fd_set>,
    ofds2: Option<&libc::fd_set>,
    ifds3: Option<&mut libc::fd_set>,
    ofds3: Option<&libc::fd_set>,
) -> usize {
    let mut n = 0;

    for s in 0..nfd {
        if let (Some(ifds), Some(ofds)) = (ifds1, ofds1) {
            if unsafe { libc::FD_ISSET(s as i32, ifds) } {
                if unsafe { libc::FD_ISSET(s as i32, ofds) } {
                    n += 1;
                } else {
                    unsafe { libc::FD_CLR(s as i32, ifds) };
                }
            }
        }

        if let (Some(ifds), Some(ofds)) = (ifds2, ofds2) {
            if unsafe { libc::FD_ISSET(s as i32, ifds) } {
                if unsafe { libc::FD_ISSET(s as i32, ofds) } {
                    n += 1;
                } else {
                    unsafe { libc::FD_CLR(s as i32, ifds) };
                }
            }
        }

        if let (Some(ifds), Some(ofds)) = (ifds3, ofds3) {
            if unsafe { libc::FD_ISSET(s as i32, ifds) } {
                if unsafe { libc::FD_ISSET(s as i32, ofds) } {
                    n += 1;
                } else {
                    unsafe { libc::FD_CLR(s as i32, ifds) };
                }
            }
        }
    }

    n
}