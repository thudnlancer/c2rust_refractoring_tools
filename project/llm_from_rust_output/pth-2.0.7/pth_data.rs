use std::sync::atomic::{AtomicBool, Ordering};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

type PthKey = i32;
type PthKeyDestructor = Box<dyn FnOnce(*mut std::ffi::c_void)>;

struct PthKeyTab {
    used: AtomicBool,
    destructor: Option<PthKeyDestructor>,
}

struct ThreadLocalData {
    data: HashMap<PthKey, *mut std::ffi::c_void>,
}

impl ThreadLocalData {
    fn new() -> Self {
        ThreadLocalData {
            data: HashMap::new(),
        }
    }
}

struct PthCurrent {
    data: ThreadLocalData,
}

static PTH_KEY_TAB: [PthKeyTab; 256] = {
    const INIT: PthKeyTab = PthKeyTab {
        used: AtomicBool::new(false),
        destructor: None,
    };
    [INIT; 256]
};

thread_local! {
    static CURRENT_THREAD: PthCurrent = PthCurrent {
        data: ThreadLocalData::new(),
    };
}

pub fn pth_key_create(key: &mut PthKey, func: Option<PthKeyDestructor>) -> i32 {
    if key.is_null() {
        return 0;
    }

    for (idx, entry) in PTH_KEY_TAB.iter().enumerate() {
        if !entry.used.load(Ordering::Relaxed) {
            if let Some(dtor) = func {
                // Safety: We're the only ones accessing this entry
                unsafe {
                    PTH_KEY_TAB[idx].used.store(true, Ordering::Relaxed);
                    PTH_KEY_TAB[idx].destructor = Some(dtor);
                }
                *key = idx as PthKey;
                return 1;
            }
        }
    }

    0
}

pub fn pth_key_delete(key: PthKey) -> i32 {
    if key < 0 || key >= 256 {
        return 0;
    }

    PTH_KEY_TAB[key as usize].used.store(false, Ordering::Relaxed);
    PTH_KEY_TAB[key as usize].destructor = None;
    1
}

pub fn pth_key_setdata(key: PthKey, value: *mut std::ffi::c_void) -> i32 {
    if key < 0 || key >= 256 || !PTH_KEY_TAB[key as usize].used.load(Ordering::Relaxed) {
        return 0;
    }

    CURRENT_THREAD.with(|current| {
        current.data.data.insert(key, value);
    });
    1
}

pub fn pth_key_getdata(key: PthKey) -> *mut std::ffi::c_void {
    if key < 0 || key >= 256 || !PTH_KEY_TAB[key as usize].used.load(Ordering::Relaxed) {
        return std::ptr::null_mut();
    }

    CURRENT_THREAD.with(|current| {
        *current.data.data.get(&key).unwrap_or(&std::ptr::null_mut())
    })
}

pub fn pth_key_destroydata() {
    CURRENT_THREAD.with(|current| {
        for (key, value) in current.data.data.drain() {
            if let Some(dtor) = PTH_KEY_TAB[key as usize].destructor.take() {
                dtor(value);
            }
        }
    });
}