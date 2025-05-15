use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Mutex, Once};
use libc::{c_int, sigaction, sigemptyset, sigaddset, sigprocmask, SIG_BLOCK, SIG_UNBLOCK, SIG_DFL, SIG_IGN};
use std::ptr;
use std::mem;

static FATAL_SIGNALS_INITIALIZED: AtomicBool = AtomicBool::new(false);
static CLEANUP_INITIALIZED: AtomicBool = AtomicBool::new(false);

const NUM_FATAL_SIGNALS: usize = 6; // Adjust based on actual signals
const STATIC_ACTIONS_SIZE: usize = 32;
const SAVED_SIGACTIONS_SIZE: usize = 64;

type Action = unsafe extern fn(c_int);

struct ActionsEntry {
    action: Action,
}

static mut STATIC_ACTIONS: [ActionsEntry; STATIC_ACTIONS_SIZE] = [ActionsEntry { action: ptr::null_mut() }; STATIC_ACTIONS_SIZE];
static mut ACTIONS: *mut ActionsEntry = STATIC_ACTIONS.as_mut_ptr();
static ACTIONS_COUNT: AtomicUsize = AtomicUsize::new(0);
static ACTIONS_ALLOCATED: AtomicUsize = AtomicUsize::new(STATIC_ACTIONS_SIZE);

static mut SAVED_SIGACTIONS: [sigaction; SAVED_SIGACTIONS_SIZE] = unsafe { mem::zeroed() };

static FATAL_SIGNAL_SET: Once = Once::new();
static mut FATAL_SIGNAL_SET: libc::sigset_t = unsafe { mem::zeroed() };

lazy_static! {
    static ref AT_FATAL_SIGNAL_LOCK: Mutex<()> = Mutex::new(());
    static ref FATAL_SIGNALS_BLOCK_LOCK: Mutex<()> = Mutex::new(());
    static ref FATAL_SIGNALS_BLOCK_COUNTER: Mutex<u32> = Mutex::new(0);
}

fn init_fatal_signals() {
    if !FATAL_SIGNALS_INITIALIZED.swap(true, Ordering::SeqCst) {
        let fatal_signals = get_fatal_signals_list();
        
        for &sig in &fatal_signals {
            if sig >= 0 {
                let mut action: sigaction = unsafe { mem::zeroed() };
                if unsafe { libc::sigaction(sig, ptr::null(), &mut action) } >= 0 {
                    if action.sa_sigaction == SIG_IGN as _ {
                        // Mark as ignored
                    }
                }
            }
        }
    }
}

fn get_fatal_signals_list() -> [c_int; NUM_FATAL_SIGNALS] {
    [
        libc::SIGINT,
        libc::SIGTERM,
        libc::SIGHUP,
        libc::SIGPIPE,
        libc::SIGXCPU,
        libc::SIGXFSZ,
    ]
}

unsafe extern "C" fn fatal_signal_handler(sig: c_int) {
    loop {
        let n = ACTIONS_COUNT.load(Ordering::SeqCst);
        if n == 0 {
            break;
        }
        let new_n = n - 1;
        ACTIONS_COUNT.store(new_n, Ordering::SeqCst);
        
        let action = (*ACTIONS.add(new_n)).action;
        if !action.is_null() {
            action(sig);
        }
    }
    
    uninstall_handlers();
    libc::raise(sig);
}

fn uninstall_handlers() {
    let fatal_signals = get_fatal_signals_list();
    
    for &sig in &fatal_signals {
        if sig >= 0 && (sig as usize) < SAVED_SIGACTIONS_SIZE {
            unsafe {
                let mut sa = SAVED_SIGACTIONS[sig as usize];
                if sa.sa_sigaction == SIG_IGN as _ {
                    sa.sa_sigaction = SIG_DFL as _;
                }
                libc::sigaction(sig, &sa, ptr::null_mut());
            }
        }
    }
}

fn install_handlers() {
    let fatal_signals = get_fatal_signals_list();
    let mut action: sigaction = unsafe { mem::zeroed() };
    
    action.sa_sigaction = fatal_signal_handler as _;
    action.sa_flags = libc::SA_NODEFER;
    unsafe { sigemptyset(&mut action.sa_mask) };
    
    for &sig in &fatal_signals {
        if sig >= 0 && (sig as usize) < SAVED_SIGACTIONS_SIZE {
            unsafe {
                libc::sigaction(sig, &action, &mut SAVED_SIGACTIONS[sig as usize]);
            }
        }
    }
}

pub fn at_fatal_signal(action: Action) -> Result<(), ()> {
    let _lock = AT_FATAL_SIGNAL_LOCK.lock().unwrap();
    
    if !CLEANUP_INITIALIZED.swap(true, Ordering::SeqCst) {
        init_fatal_signals();
        install_handlers();
    }
    
    let count = ACTIONS_COUNT.load(Ordering::SeqCst);
    let allocated = ACTIONS_ALLOCATED.load(Ordering::SeqCst);
    
    if count == allocated {
        let new_allocated = allocated * 2;
        let new_actions = unsafe {
            let layout = std::alloc::Layout::array::<ActionsEntry>(new_allocated).unwrap();
            std::alloc::alloc(layout) as *mut ActionsEntry
        };
        
        if new_actions.is_null() {
            return Err(());
        }
        
        unsafe {
            for i in 0..allocated {
                ptr::write(new_actions.add(i), ptr::read(ACTIONS.add(i)));
            }
            
            ACTIONS = new_actions;
            ACTIONS_ALLOCATED.store(new_allocated, Ordering::SeqCst);
        }
    }
    
    unsafe {
        ptr::write(ACTIONS.add(count), ActionsEntry { action });
    }
    ACTIONS_COUNT.store(count + 1, Ordering::SeqCst);
    
    Ok(())
}

fn init_fatal_signal_set() {
    FATAL_SIGNAL_SET.call_once(|| unsafe {
        sigemptyset(&mut FATAL_SIGNAL_SET);
        for &sig in &get_fatal_signals_list() {
            if sig >= 0 {
                sigaddset(&mut FATAL_SIGNAL_SET, sig);
            }
        }
    });
}

pub fn block_fatal_signals() {
    let _lock = FATAL_SIGNALS_BLOCK_LOCK.lock().unwrap();
    let mut counter = FATAL_SIGNALS_BLOCK_COUNTER.lock().unwrap();
    
    if *counter == 0 {
        init_fatal_signal_set();
        unsafe {
            sigprocmask(SIG_BLOCK, &FATAL_SIGNAL_SET, ptr::null_mut());
        }
    }
    *counter += 1;
}

pub fn unblock_fatal_signals() {
    let _lock = FATAL_SIGNALS_BLOCK_LOCK.lock().unwrap();
    let mut counter = FATAL_SIGNALS_BLOCK_COUNTER.lock().unwrap();
    
    if *counter == 0 {
        panic!("More unblock_fatal_signals() calls than block_fatal_signals()");
    }
    
    *counter -= 1;
    if *counter == 0 {
        init_fatal_signal_set();
        unsafe {
            sigprocmask(SIG_UNBLOCK, &FATAL_SIGNAL_SET, ptr::null_mut());
        }
    }
}

pub fn get_fatal_signals(signals: &mut [c_int; 64]) -> usize {
    init_fatal_signal_set();
    let fatal_signals = get_fatal_signals_list();
    
    let mut count = 0;
    for &sig in &fatal_signals {
        if sig >= 0 {
            signals[count] = sig;
            count += 1;
        }
    }
    count
}

pub fn get_fatal_signal_set() -> &'static libc::sigset_t {
    init_fatal_signal_set();
    unsafe { &FATAL_SIGNAL_SET }
}