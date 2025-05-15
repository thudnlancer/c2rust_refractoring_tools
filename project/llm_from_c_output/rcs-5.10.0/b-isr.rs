use std::sync::atomic::{AtomicBool, Ordering};
use std::ffi::CStr;
use std::os::unix::prelude::*;
use std::ptr;
use std::mem;
use libc::{c_int, c_void, sigaction, sigemptyset, sigaddset, sigaltstack, stack_t, SIG_DFL, SIG_IGN, SA_SIGINFO, SA_ONSTACK};
use nix::sys::signal::{self, Signal, SigHandler, SigSet, SigAction, SaFlags};
use nix::errno::Errno;
use nix::unistd::write;

#[derive(Debug)]
struct IsrScratch {
    held: AtomicBool,
    level: AtomicBool,
    held_info: *mut libc::siginfo_t,
    access_name: Option<String>,
    catching: Catching,
    be_quiet: *mut bool,
}

#[derive(Debug)]
struct Catching {
    regular: bool,
    memory_map: bool,
}

#[derive(Debug, Clone, Copy)]
enum IsrActions {
    CatchInts,
    IgnoreInts,
    RestoreInts,
    CatchMmapInts,
}

fn maybe_reset_sigchld() {
    if BAD_WAIT_IF_SIGCHLD_IGNORED {
        unsafe {
            signal::signal(Signal::SIGCHLD, SigHandler::SigDfl).unwrap();
        }
    }
}

fn complain_signal(msg: &str, signo: i32) {
    werr(msg);
    werr(": ");
    werr(signal(signo).unwrap_or("Unknown signal"));
    werr("\n");
}

fn werr(s: &str) {
    if s.is_empty() {
        return;
    }
    write(libc::STDERR_FILENO, s.as_bytes()).unwrap();
}

fn access_page(scratch: &mut IsrScratch, filename: Option<&str>, p: *const u8) -> u8 {
    scratch.access_name = filename.map(|s| s.to_string());
    let t = unsafe { *p };
    scratch.access_name = None;
    t
}

fn ignore(scratch: &mut IsrScratch) {
    scratch.level.store(true, Ordering::SeqCst);
}

fn catchsigaction(signo: i32, info: *mut libc::siginfo_t, _uc: *mut c_void) {
    let scratch = unsafe { &mut *ISR_SCRATCH };
    let from_mmap = MMAP_SIGNAL == Some(signo);

    if scratch.level.load(Ordering::SeqCst) {
        scratch.held.store(true, Ordering::SeqCst);
        if !info.is_null() {
            unsafe {
                *scratch.held_info = *info;
            }
        }
        return;
    }

    ignore(scratch);
    setrid();
    if !unsafe { *scratch.be_quiet } {
        if !(from_mmap && scratch.access_name.is_some()) {
            let mut nrcs = "\nRCS";
            if from_mmap && !info.is_null() && unsafe { (*info).si_errno != 0 } {
                unsafe {
                    libc::errno = (*info).si_errno;
                    libc::perror(nrcs.as_ptr() as *const i8);
                    nrcs = &nrcs[1..];
                }
            }
            if !info.is_null() {
                psiginfo(info, nrcs);
            } else {
                complain_signal(nrcs, signo);
            }
        }

        werr("RCS: ");
        if from_mmap {
            if let Some(name) = &scratch.access_name {
                werr(name);
                werr(": Permission denied.  ");
            } else {
                werr("Was a file changed by some other process?  ");
            }
        }
        werr("Cleaning up.\n");
    }
    BOW_OUT();
}

fn setup_catchsig(count: usize, set: &[i32]) -> Result<(), nix::Error> {
    let mut blocked = SigSet::empty();
    for sig in set.iter().take(count) {
        blocked.add(Signal::from_c_int(*sig)?);
    }

    for sig in set.iter().take(count) {
        let old_act = signal::sigaction(
            Signal::from_c_int(*sig)?,
            &SigAction::new(
                SigHandler::SigAction(catchsigaction),
                SaFlags::SA_SIGINFO | SaFlags::SA_ONSTACK,
                blocked,
            ),
        )?;
        if old_act.handler() != SigHandler::SigIgn {
            signal::sigaction(
                Signal::from_c_int(*sig)?,
                &SigAction::new(
                    SigHandler::SigAction(catchsigaction),
                    SaFlags::SA_SIGINFO | SaFlags::SA_ONSTACK,
                    blocked,
                ),
            )?;
        }
    }
    Ok(())
}

const ISR_STACK_SIZE: usize = if cfg!(HAVE_SIGALTSTACK) && cfg!(SIGSTKSZ) {
    10 * SIGSTKSZ
} else {
    0
};

fn isr_init(be_quiet: &mut bool) -> Box<IsrScratch> {
    let mut scratch = Box::new(IsrScratch {
        held: AtomicBool::new(false),
        level: AtomicBool::new(false),
        held_info: ptr::null_mut(),
        access_name: None,
        catching: Catching {
            regular: false,
            memory_map: false,
        },
        be_quiet: be_quiet as *mut bool,
    });

    if ISR_STACK_SIZE > 0 {
        let ss = stack_t {
            ss_sp: unsafe { libc::malloc(ISR_STACK_SIZE) } as *mut c_void,
            ss_size: ISR_STACK_SIZE,
            ss_flags: 0,
        };
        unsafe {
            if sigaltstack(&ss, ptr::null_mut()) != 0 {
                fatal_sys("sigaltstack");
            }
        }
    }

    scratch
}

fn isr_do(scratch: &mut IsrScratch, action: IsrActions) {
    match action {
        IsrActions::CatchInts => {
            if !scratch.catching.regular {
                scratch.catching.regular = true;
                let regular = [
                    Signal::SIGHUP as i32,
                    Signal::SIGINT as i32,
                    Signal::SIGQUIT as i32,
                    Signal::SIGPIPE as i32,
                    Signal::SIGTERM as i32,
                    Signal::SIGXCPU as i32,
                    Signal::SIGXFSZ as i32,
                ];
                setup_catchsig(regular.len(), &regular).unwrap();
            }
        }
        IsrActions::IgnoreInts => ignore(scratch),
        IsrActions::RestoreInts => {
            scratch.level.store(false, Ordering::SeqCst);
            if scratch.held.load(Ordering::SeqCst) {
                catchsigaction(
                    scratch.held.load(Ordering::SeqCst) as i32,
                    scratch.held_info,
                    ptr::null_mut(),
                );
            }
        }
        IsrActions::CatchMmapInts => {
            if let Some(sig) = MMAP_SIGNAL {
                if !scratch.catching.memory_map {
                    scratch.catching.memory_map = true;
                    let mmapsigs = [sig];
                    setup_catchsig(mmapsigs.len(), &mmapsigs).unwrap();
                }
            }
        }
    }
}

static mut ISR_SCRATCH: *mut IsrScratch = ptr::null_mut();
static BAD_WAIT_IF_SIGCHLD_IGNORED: bool = false;
static MMAP_SIGNAL: Option<i32> = None;

fn BOW_OUT() {
    std::process::exit(1);
}

fn fatal_sys(msg: &str) {
    eprintln!("{}: {}", msg, std::io::Error::last_os_error());
    std::process::exit(1);
}