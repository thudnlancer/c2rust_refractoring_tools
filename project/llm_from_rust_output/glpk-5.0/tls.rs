use std::sync::atomic::{AtomicPtr, Ordering};
use std::ptr;

#[thread_local]
static TLS: AtomicPtr<()> = AtomicPtr::new(ptr::null_mut());

#[no_mangle]
pub extern "C" fn _glp_tls_set_ptr(ptr: *mut ()) {
    TLS.store(ptr, Ordering::Relaxed);
}

#[no_mangle]
pub extern "C" fn _glp_tls_get_ptr() -> *mut () {
    TLS.load(Ordering::Relaxed)
}