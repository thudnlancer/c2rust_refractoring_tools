use std::mem;
use std::ptr;
use std::process;
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicUsize, Ordering};
use std::sync::{Mutex, Once};
use nix::sys::signal;
use nix::sys::wait::{waitpid, WaitStatus};
use nix::unistd::Pid;
use libc::{c_char, c_int};

const INITIAL_SLAVES_CAPACITY: usize = 32;

#[derive(Debug)]
struct SlaveEntry {
    used: AtomicBool,
    child: Mutex<Option<Pid>>,
}

impl SlaveEntry {
    fn new() -> Self {
        SlaveEntry {
            used: AtomicBool::new(false),
            child: Mutex::new(None),
        }
    }
}

struct Slaves {
    entries: Mutex<Vec<SlaveEntry>>,
    count: AtomicUsize,
}

impl Slaves {
    fn new() -> Self {
        let mut entries = Vec::with_capacity(INITIAL_SLAVES_CAPACITY);
        for _ in 0..INITIAL_SLAVES_CAPACITY {
            entries.push(SlaveEntry::new());
        }
        
        Slaves {
            entries: Mutex::new(entries),
            count: AtomicUsize::new(0),
        }
    }

    fn cleanup(&self) {
        let entries = self.entries.lock().unwrap();
        let count = self.count.load(Ordering::SeqCst);
        
        for i in 0..count {
            if entries[i].used.load(Ordering::SeqCst) {
                if let Some(pid) = *entries[i].child.lock().unwrap() {
                    let _ = signal::kill(pid, signal::SIGTERM);
                }
            }
        }
    }

    fn register(&self, child: Pid) -> Result<(), ()> {
        let mut entries = self.entries.lock().unwrap();
        let count = self.count.load(Ordering::SeqCst);

        // Try to find an empty slot
        for i in 0..count {
            if !entries[i].used.load(Ordering::SeqCst) {
                entries[i].used.store(true, Ordering::SeqCst);
                *entries[i].child.lock().unwrap() = Some(child);
                return Ok(());
            }
        }

        // Need to expand the vector
        if count >= entries.len() {
            let new_capacity = entries.len() * 2;
            entries.reserve(new_capacity);
            for _ in entries.len()..new_capacity {
                entries.push(SlaveEntry::new());
            }
        }

        // Add new entry
        entries[count].used.store(true, Ordering::SeqCst);
        *entries[count].child.lock().unwrap() = Some(child);
        self.count.store(count + 1, Ordering::SeqCst);

        Ok(())
    }

    fn unregister(&self, child: Pid) {
        let entries = self.entries.lock().unwrap();
        let count = self.count.load(Ordering::SeqCst);
        
        for i in 0..count {
            if entries[i].used.load(Ordering::SeqCst) {
                if let Some(pid) = *entries[i].child.lock().unwrap() {
                    if pid == child {
                        entries[i].used.store(false, Ordering::SeqCst);
                    }
                }
            }
        }
    }
}

lazy_static! {
    static ref SLAVES: Slaves = Slaves::new();
    static ref CLEANUP_REGISTERED: AtomicBool = AtomicBool::new(false);
}

fn register_cleanup_handlers() {
    if CLEANUP_REGISTERED.compare_and_swap(false, true, Ordering::SeqCst) == false {
        // Register atexit handler
        unsafe {
            libc::atexit(Some(cleanup_slaves));
        }

        // Register fatal signal handlers
        let signals = [
            signal::SIGINT,
            signal::SIGTERM,
            signal::SIGHUP,
            signal::SIGQUIT,
        ];
        
        for sig in signals.iter() {
            let _ = unsafe {
                signal::signal(*sig, signal::SigHandler::Handler(cleanup_slaves_action))
            };
        }
    }
}

extern "C" fn cleanup_slaves() {
    SLAVES.cleanup();
}

extern "C" fn cleanup_slaves_action(sig: c_int) {
    cleanup_slaves();
}

pub fn register_slave_subprocess(child: Pid) -> Result<(), ()> {
    register_cleanup_handlers();
    SLAVES.register(child)
}

fn unregister_slave_subprocess(child: Pid) {
    SLAVES.unregister(child);
}

pub fn wait_subprocess(
    child: Pid,
    progname: *const c_char,
    ignore_sigpipe: bool,
    null_stderr: bool,
    slave_process: bool,
    exit_on_error: bool,
    termsigp: Option<&mut c_int>,
) -> Result<c_int, c_int> {
    if let Some(termsig) = termsigp {
        *termsig = 0;
    }

    loop {
        match waitpid(child, None) {
            Ok(WaitStatus::Exited(pid, status)) if pid == child => {
                if slave_process {
                    unregister_slave_subprocess(child);
                }
                return Ok(status);
            }
            Ok(WaitStatus::Signaled(pid, sig, _)) if pid == child => {
                if slave_process {
                    unregister_slave_subprocess(child);
                }
                
                if let Some(termsig) = termsigp {
                    *termsig = sig as c_int;
                }

                if sig == signal::SIGPIPE && ignore_sigpipe {
                    return Ok(0);
                }

                if exit_on_error || (!null_stderr && termsigp.is_none()) {
                    // TODO: Implement proper error reporting
                    unsafe {
                        libc::fprintf(
                            libc::stderr,
                            b"%s subprocess got fatal signal %d\n\0".as_ptr() as *const c_char,
                            progname,
                            sig as c_int,
                        );
                    }
                }

                return Err(127);
            }
            Ok(WaitStatus::Stopped(_, _)) => continue,
            Err(nix::Error::Sys(errno)) if errno == nix::errno::Errno::EINTR => continue,
            Err(e) => {
                if exit_on_error || !null_stderr {
                    // TODO: Implement proper error reporting
                    unsafe {
                        libc::fprintf(
                            libc::stderr,
                            b"%s subprocess failed: %s\n\0".as_ptr() as *const c_char,
                            progname,
                            e.to_string().as_ptr() as *const c_char,
                        );
                    }
                }
                return Err(127);
            }
            _ => continue,
        }
    }
}