use std::sync::{Once, Mutex, RwLock};
use std::sync::atomic::{AtomicBool, Ordering};

pub type GlRecursiveLock = Mutex<()>;
pub type GlRwLock = RwLock<()>;

pub fn glthread_rwlock_init_for_glibc() -> Result<GlRwLock, std::io::Error> {
    Ok(RwLock::new(()))
}

pub fn glthread_recursive_lock_init_multithreaded() -> Result<GlRecursiveLock, std::io::Error> {
    Ok(Mutex::new(()))
}

static ONCE_CONTROL: Once = Once::new();
static INITIALIZED: AtomicBool = AtomicBool::new(false);

pub fn glthread_once_singlethreaded() -> bool {
    let mut initialized = INITIALIZED.load(Ordering::SeqCst);
    if !initialized {
        ONCE_CONTROL.call_once(|| {
            INITIALIZED.store(true, Ordering::SeqCst);
        });
        initialized = INITIALIZED.load(Ordering::SeqCst);
    }
    initialized
}