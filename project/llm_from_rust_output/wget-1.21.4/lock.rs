use std::sync::{Once, Mutex, RwLock};
use std::sync::atomic::{AtomicBool, Ordering};

pub type GlRecursiveLock = Mutex<()>;
pub type GlRwLock = RwLock<()>;

pub fn glthread_rwlock_init() -> Result<GlRwLock, i32> {
    Ok(RwLock::new(()))
}

pub fn glthread_recursive_lock_init_multithreaded() -> Result<GlRecursiveLock, i32> {
    Ok(Mutex::new(()))
}

static ONCE_INIT: Once = Once::new();
static INITIALIZED: AtomicBool = AtomicBool::new(false);

pub fn glthread_once_singlethreaded() -> bool {
    ONCE_INIT.call_once(|| {
        INITIALIZED.store(true, Ordering::SeqCst);
    });
    INITIALIZED.load(Ordering::SeqCst)
}