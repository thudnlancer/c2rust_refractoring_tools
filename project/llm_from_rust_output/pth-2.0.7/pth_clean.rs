use std::alloc::{alloc, dealloc, Layout};
use std::ptr::NonNull;
use std::sync::atomic::{AtomicPtr, Ordering};
use std::sync::Mutex;

type PthCleanupFunc = Box<dyn FnOnce(Box<dyn std::any::Any>)>;

struct PthCleanup {
    func: PthCleanupFunc,
    arg: Box<dyn std::any::Any>,
    next: Option<Box<PthCleanup>>,
}

struct PthCurrent {
    cleanups: Mutex<Option<Box<PthCleanup>>>,
}

static PTH_CURRENT: PthCurrent = PthCurrent {
    cleanups: Mutex::new(None),
};

fn pth_cleanup_push(func: PthCleanupFunc, arg: Box<dyn std::any::Any>) -> Result<(), ()> {
    let mut cleanups = PTH_CURRENT.cleanups.lock().unwrap();
    *cleanups = Some(Box::new(PthCleanup {
        func,
        arg,
        next: cleanups.take(),
    }));
    Ok(())
}

fn pth_cleanup_pop(execute: bool) -> Result<(), ()> {
    let mut cleanups = PTH_CURRENT.cleanups.lock().unwrap();
    if let Some(mut cleanup) = cleanups.take() {
        if execute {
            (cleanup.func)(cleanup.arg);
        }
        *cleanups = cleanup.next;
    }
    Ok(())
}

fn pth_cleanup_popall(execute: bool) {
    let mut cleanups = PTH_CURRENT.cleanups.lock().unwrap();
    while let Some(mut cleanup) = cleanups.take() {
        if execute {
            (cleanup.func)(cleanup.arg);
        }
        *cleanups = cleanup.next;
    }
}