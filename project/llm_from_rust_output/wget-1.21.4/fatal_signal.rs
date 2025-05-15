use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Mutex, Once};
use libc::{c_int, c_uint, c_ulong, c_void, sigaction, sigset_t, pthread_mutex_t};

const FATAL_SIGNALS: [c_int; 7] = [
    libc::SIGINT,
    libc::SIGTERM,
    libc::SIGHUP,
    libc::SIGPIPE,
    libc::SIGXCPU,
    libc::SIGXFSZ,
    0,
];

static ACTIONS: Mutex<Vec<Option<extern "C" fn(c_int)>>> = Mutex::new(Vec::new());
static SAVED_SIGACTIONS: Mutex<[sigaction; 64]> = Mutex::new(unsafe { std::mem::zeroed() });
static FATAL_SIGNAL_SET: Mutex<sigset_t> = Mutex::new(unsafe { std::mem::zeroed() });
static FATAL_SIGNAL_SET_ONCE: Once = Once::new();
static FATAL_SIGNALS_BLOCK_COUNTER: AtomicUsize = AtomicUsize::new(0);
static FATAL_SIGNALS_BLOCK_LOCK: Mutex<()> = Mutex::new(());

extern "C" fn fatal_signal_handler(sig: c_int) {
    let mut actions = ACTIONS.lock().unwrap();
    while let Some(action) = actions.pop() {
        if let Some(handler) = action {
            handler(sig);
        }
    }
    
    uninstall_handlers();
    unsafe { libc::raise(sig) };
}

fn install_handlers() {
    let mut action = unsafe { std::mem::zeroed::<sigaction>() };
    action.sa_sigaction = fatal_signal_handler as *const () as usize;
    action.sa_flags = libc::SA_SIGINFO;
    unsafe { libc::sigemptyset(&mut action.sa_mask) };

    let mut saved = SAVED_SIGACTIONS.lock().unwrap();
    for &sig in FATAL_SIGNALS.iter().take(6) {
        if sig > 0 {
            unsafe {
                libc::sigaction(sig, &action, &mut saved[sig as usize] as *mut _);
            }
        }
    }
}

fn uninstall_handlers() {
    let saved = SAVED_SIGACTIONS.lock().unwrap();
    for &sig in FATAL_SIGNALS.iter().take(6) {
        if sig > 0 {
            unsafe {
                libc::sigaction(sig, &saved[sig as usize], std::ptr::null_mut());
            }
        }
    }
}

pub extern "C" fn at_fatal_signal(action: Option<extern "C" fn(c_int)>) -> c_int {
    let _lock = FATAL_SIGNALS_BLOCK_LOCK.lock().unwrap();
    let mut actions = ACTIONS.lock().unwrap();
    actions.push(action);
    0
}

fn init_fatal_signal_set() {
    FATAL_SIGNAL_SET_ONCE.call_once(|| {
        let mut set = unsafe { std::mem::zeroed::<sigset_t>() };
        unsafe { libc::sigemptyset(&mut set) };
        
        for &sig in FATAL_SIGNALS.iter().take(6) {
            if sig > 0 {
                unsafe { libc::sigaddset(&mut set, sig) };
            }
        }
        
        *FATAL_SIGNAL_SET.lock().unwrap() = set;
    });
}

pub extern "C" fn block_fatal_signals() {
    let _lock = FATAL_SIGNALS_BLOCK_LOCK.lock().unwrap();
    let count = FATAL_SIGNALS_BLOCK_COUNTER.fetch_add(1, Ordering::SeqCst);
    
    if count == 0 {
        init_fatal_signal_set();
        let set = FATAL_SIGNAL_SET.lock().unwrap();
        unsafe { libc::sigprocmask(libc::SIG_BLOCK, &*set, std::ptr::null_mut()) };
    }
}

pub extern "C" fn unblock_fatal_signals() {
    let _lock = FATAL_SIGNALS_BLOCK_LOCK.lock().unwrap();
    let count = FATAL_SIGNALS_BLOCK_COUNTER.fetch_sub(1, Ordering::SeqCst);
    
    if count == 1 {
        init_fatal_signal_set();
        let set = FATAL_SIGNAL_SET.lock().unwrap();
        unsafe { libc::sigprocmask(libc::SIG_UNBLOCK, &*set, std::ptr::null_mut()) };
    }
}

pub extern "C" fn get_fatal_signals(signals: *mut c_int) -> c_uint {
    init_fatal_signal_set();
    let mut count = 0;
    
    for &sig in FATAL_SIGNALS.iter().take(6) {
        if sig > 0 {
            unsafe { *signals.add(count) = sig };
            count += 1;
        }
    }
    
    count as c_uint
}

pub extern "C" fn get_fatal_signal_set() -> *const sigset_t {
    init_fatal_signal_set();
    &*FATAL_SIGNAL_SET.lock().unwrap() as *const _
}