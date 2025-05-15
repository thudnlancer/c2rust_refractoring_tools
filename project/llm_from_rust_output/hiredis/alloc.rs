use std::alloc::{alloc, alloc_zeroed, dealloc, realloc};
use std::ffi::{CStr, CString};
use std::mem;
use std::sync::Mutex;

pub type size_t = usize;

#[derive(Clone)]
pub struct HiredisAllocFuncs {
    pub malloc_fn: Option<fn(size_t) -> *mut u8>,
    pub calloc_fn: Option<fn(size_t, size_t) -> *mut u8>,
    pub realloc_fn: Option<fn(*mut u8, size_t) -> *mut u8>,
    pub strdup_fn: Option<fn(*const i8) -> *mut i8>,
    pub free_fn: Option<fn(*mut u8)>,
}

impl Default for HiredisAllocFuncs {
    fn default() -> Self {
        HiredisAllocFuncs {
            malloc_fn: Some(default_malloc),
            calloc_fn: Some(default_calloc),
            realloc_fn: Some(default_realloc),
            strdup_fn: Some(default_strdup),
            free_fn: Some(default_free),
        }
    }
}

fn default_malloc(size: size_t) -> *mut u8 {
    unsafe { alloc(mem::size_of::<u8>() * size) }
}

fn default_calloc(nmemb: size_t, size: size_t) -> *mut u8 {
    unsafe { alloc_zeroed(mem::size_of::<u8>() * nmemb * size) }
}

fn default_realloc(ptr: *mut u8, size: size_t) -> *mut u8 {
    unsafe { realloc(ptr, mem::size_of::<u8>() * size) }
}

fn default_strdup(s: *const i8) -> *mut i8 {
    unsafe {
        let c_str = CStr::from_ptr(s);
        match CString::new(c_str.to_bytes()) {
            Ok(c_string) => c_string.into_raw(),
            Err(_) => std::ptr::null_mut(),
        }
    }
}

fn default_free(ptr: *mut u8) {
    unsafe { dealloc(ptr, mem::size_of::<u8>()) }
}

lazy_static! {
    static ref HIREDIS_ALLOC_FNS: Mutex<HiredisAllocFuncs> = Mutex::new(HiredisAllocFuncs::default());
}

pub fn hiredis_set_allocators(override_fns: HiredisAllocFuncs) -> HiredisAllocFuncs {
    let mut fns = HIREDIS_ALLOC_FNS.lock().unwrap();
    let orig = fns.clone();
    *fns = override_fns;
    orig
}

pub fn hiredis_reset_allocators() {
    let mut fns = HIREDIS_ALLOC_FNS.lock().unwrap();
    *fns = HiredisAllocFuncs::default();
}