use std::ffi::{CStr, CString};
use std::os::raw::{c_int, c_char, c_void};
use std::ptr;
use std::process;
use libc::{sigaction, sigemptyset, sigset_t, siginfo_t, SIG_DFL, SA_RESTART};

#[derive(Debug, Clone, Copy)]
pub struct Signal {
    pub signo: c_int,
    pub signame: &'static str,
    pub flags: c_int,
    pub handler: Option<extern "C" fn(c_int)>,
}

impl Signal {
    const fn new(signo: c_int, signame: &'static str, flags: c_int, handler: Option<extern "C" fn(c_int)>) -> Self {
        Signal { signo, signame, flags, handler }
    }
}

const SIGNALS: &[Signal] = &[
    Signal::new(10, "SIGUSR1", 0, Some(signal_handler)),
    Signal::new(12, "SIGUSR2", 0, Some(signal_handler)),
    Signal::new(21, "SIGTTIN", 0, Some(signal_handler)),
    Signal::new(22, "SIGTTOU", 0, Some(signal_handler)),
    Signal::new(1, "SIGHUP", 0, Some(signal_handler)),
    Signal::new(2, "SIGINT", 0, Some(signal_handler)),
    Signal::new(11, "SIGSEGV", 0x80000000, Some(signal_handler)),
    Signal::new(13, "SIGPIPE", 0, Some(SIG_DFL)),
    Signal::new(0, "", 0, None),
];

pub fn signal_init() -> Result<(), String> {
    for sig in SIGNALS.iter().take_while(|s| s.signo != 0) {
        let mut sa: sigaction = unsafe { std::mem::zeroed() };
        sa.sa_sigaction = sig.handler.map(|h| h as usize).unwrap_or(0) as *mut c_void;
        sa.sa_flags = sig.flags | SA_RESTART;
        
        unsafe { sigemptyset(&mut sa.sa_mask) };
        
        if unsafe { sigaction(sig.signo, &sa, ptr::null_mut()) } < 0 {
            let errno = unsafe { *libc::__errno_location() };
            let err_msg = unsafe { CStr::from_ptr(libc::strerror(errno)) };
            return Err(format!(
                "sigaction({}) failed: {}",
                sig.signame,
                err_msg.to_string_lossy()
            ));
        }
    }
    Ok(())
}

pub fn signal_deinit() {}

#[no_mangle]
pub extern "C" fn signal_handler(signo: c_int) {
    let sig = SIGNALS.iter().find(|s| s.signo == signo).unwrap_or(&SIGNALS[SIGNALS.len() - 1]);
    
    let (action_str, action, done) = match signo {
        21 => (", up logging level", Some(log_level_up), false),
        22 => (", down logging level", Some(log_level_down), false),
        1 => (", reopening log file", Some(log_reopen), false),
        2 => (", exiting", None, true),
        11 => {
            log_stacktrace();
            unsafe { libc::raise(11) };
            (", core dumping", None, false)
        }
        _ => ("", None, false),
    };
    
    log_safe(&format!(
        "signal {} ({}) received{}",
        signo, sig.signame, action_str
    ));
    
    if let Some(act) = action {
        act();
    }
    
    if done {
        process::exit(1);
    }
}

// Mock logging functions - these would be implemented elsewhere
extern "C" {
    fn log_level_up();
    fn log_level_down();
    fn log_stacktrace();
    fn log_reopen();
    fn log_safe(msg: &str);
}