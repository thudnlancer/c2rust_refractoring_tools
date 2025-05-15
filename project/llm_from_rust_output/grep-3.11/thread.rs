use std::thread;
use std::os::raw::c_void;
use std::sync::mpsc::{channel, Sender};
use std::mem::transmute;

pub type gl_thread_t = thread::JoinHandle<()>;

#[no_mangle]
pub extern "C" fn gl_thread_create(
    func: Option<extern "C" fn(*mut c_void) -> *mut c_void>,
    arg: *mut c_void,
) -> gl_thread_t {
    let func = func.expect("Function pointer must not be null");
    
    let (tx, rx) = channel();
    let handle = thread::spawn(move || {
        let result = func(arg);
        tx.send(()).unwrap();
        result
    });

    // Wait for thread to start executing
    rx.recv().unwrap();
    
    handle
}