use std::ffi::CStr;
use std::os::raw::{c_char, c_int, c_void};
use std::ptr;
use std::sync::atomic::{AtomicI32, Ordering};
use libc::{signal, SIGCHLD, SIG_DFL};

static TOP: Option<&Top> = None;

struct Top {
    program: Program,
}

struct Program {
    tyag: c_int,
}

fn maybe_reset_sigchld() {
    unsafe {
        signal(SIGCHLD, SIG_DFL);
    }
}

fn werr(s: &str) {
    if s.is_empty() {
        return;
    }
    let bytes = s.as_bytes();
    unsafe {
        if libc::write(2, bytes.as_ptr() as *const c_void, bytes.len()) != bytes.len() as isize {
            thank_you_and_goodnight(TOP.unwrap().program.tyag);
        }
    }
}

fn complain_signal(msg: &str, signo: c_int) {
    werr(msg);
    werr(": ");
    unsafe {
        let sig_str = libc::strsignal(signo);
        if !sig_str.is_null() {
            werr(CStr::from_ptr(sig_str).to_str().unwrap());
        }
    }
    werr("\n");
}

struct IsrScratch {
    held: AtomicI32,
    level: AtomicI32,
    access_name: Option<&'static str>,
    be_quiet: bool,
    catching: Catching,
}

struct Catching {
    regular: bool,
    memory_map: bool,
}

fn access_page(scratch: &mut IsrScratch, filename: &str, p: *const c_char) -> c_char {
    scratch.access_name = Some(filename);
    let t = unsafe { ptr::read_volatile(p) };
    scratch.access_name = None;
    t
}

fn ignore(scratch: &mut IsrScratch) {
    scratch.level.fetch_add(1, Ordering::SeqCst);
}

fn catchsigaction(signo: c_int, info: *mut libc::siginfo_t, _uc: *mut c_void) {
    let scratch = unsafe { &mut *(*TOP.unwrap()).behavior.isr };
    let from_mmap = signo == 7;

    if scratch.level.load(Ordering::SeqCst) != 0 {
        scratch.held.store(signo, Ordering::SeqCst);
        if !info.is_null() {
            unsafe {
                (*scratch).bufinfo = *info;
                (*scratch).held_info = &mut (*scratch).bufinfo;
            }
        }
        return;
    }

    ignore(scratch);
    setrid();

    if !scratch.be_quiet && !(from_mmap && scratch.access_name.is_some()) {
        let mut nrcs = "\nRCS";
        if from_mmap && !info.is_null() && unsafe { (*info).si_errno != 0 } {
            unsafe {
                libc::perror(nrcs.as_ptr() as *const c_char);
            }
            nrcs = "";
        }
        if !info.is_null() {
            unsafe {
                libc::psiginfo(info, nrcs.as_ptr() as *const c_char);
            }
        } else {
            complain_signal(nrcs, signo);
        }
    }

    werr("RCS: ");
    if from_mmap {
        if let Some(name) = scratch.access_name {
            werr(name);
            werr(": Permission denied.  ");
        } else {
            werr("Was a file changed by some other process?  ");
        }
    }
    werr("Cleaning up.\n");
    thank_you_and_goodnight(TOP.unwrap().program.tyag);
}

fn setup_catchsig(count: usize, set: &[c_int]) {
    let mut blocked = unsafe {
        let mut set = std::mem::zeroed();
        libc::sigemptyset(&mut set);
        set
    };

    for &sig in set.iter().take(count) {
        if unsafe { libc::sigaddset(&mut blocked, sig) } < 0 {
            fatal_sys("signal handling");
        }
    }

    for &sig in set.iter().take(count) {
        let mut act = unsafe { std::mem::zeroed() };
        if unsafe { libc::sigaction(sig, ptr::null(), &mut act) } < 0 {
            fatal_sys("signal handling");
        }

        if act.sa_handler != Some(libc::SIG_IGN) {
            act.sa_sigaction = catchsigaction as usize;
            act.sa_flags |= libc::SA_SIGINFO | libc::SA_ONSTACK;
            act.sa_mask = blocked;
            if unsafe { libc::sigaction(sig, &act, ptr::null_mut()) } < 0 {
                fatal_sys("signal handling");
            }
        }
    }
}

fn isr_init(be_quiet: bool) -> Box<IsrScratch> {
    let mut scratch = Box::new(IsrScratch {
        held: AtomicI32::new(0),
        level: AtomicI32::new(0),
        access_name: None,
        be_quiet,
        catching: Catching {
            regular: false,
            memory_map: false,
        },
    });

    let mut ss = libc::stack_t {
        ss_sp: unsafe { libc::malloc(10 * 8192) },
        ss_flags: 0,
        ss_size: 10 * 8192,
    };

    if unsafe { libc::sigaltstack(&ss, ptr::null_mut()) } < 0 {
        fatal_sys("sigaltstack");
    }

    scratch
}

fn isr_do(scratch: &mut IsrScratch, action: IsrActions) {
    match action {
        IsrActions::CatchInts => {
            if !scratch.catching.regular {
                scratch.catching.regular = true;
                let regular = [1, 2, 3, 13, 15, 24, 25];
                setup_catchsig(regular.len(), &regular);
            }
        }
        IsrActions::IgnoreInts => {
            ignore(scratch);
        }
        IsrActions::RestoreInts => {
            let prev = scratch.level.fetch_sub(1, Ordering::SeqCst) - 1;
            if prev == 0 && scratch.held.load(Ordering::SeqCst) != 0 {
                catchsigaction(
                    scratch.held.load(Ordering::SeqCst),
                    ptr::null_mut(),
                    ptr::null_mut(),
                );
            }
        }
        IsrActions::CatchMmapInts => {
            if !scratch.catching.memory_map {
                scratch.catching.memory_map = true;
                let mmapsigs = [7];
                setup_catchsig(mmapsigs.len(), &mmapsigs);
            }
        }
    }
}

enum IsrActions {
    CatchInts,
    IgnoreInts,
    RestoreInts,
    CatchMmapInts,
}

fn fatal_sys(msg: &str) {
    werr(msg);
    unsafe {
        libc::exit(1);
    }
}

fn thank_you_and_goodnight(how: c_int) {
    unsafe {
        libc::exit(how);
    }
}

fn setrid() {
    // Implementation omitted for brevity
}