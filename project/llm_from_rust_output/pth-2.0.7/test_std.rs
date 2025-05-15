use std::ffi::CString;
use std::process::exit;
use std::ptr;

mod ffi {
    use libc::{c_char, c_int, c_long, c_void};
    
    #[repr(C)]
    pub struct pth_st;
    #[repr(C)]
    pub struct pth_attr_st;
    
    pub type pth_t = *mut pth_st;
    pub type pth_attr_t = *mut pth_attr_st;
    
    extern "C" {
        pub static stderr: *mut FILE;
        pub fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
        pub fn __errno_location() -> *mut c_int;
        pub fn pth_init() -> c_int;
        pub fn pth_kill() -> c_int;
        pub fn pth_version() -> c_long;
        pub fn pth_attr_new() -> pth_attr_t;
        pub fn pth_attr_set(attr: pth_attr_t, field: c_int, ...) -> c_int;
        pub fn pth_attr_destroy(attr: pth_attr_t) -> c_int;
        pub fn pth_spawn(
            attr: pth_attr_t,
            func: Option<unsafe extern "C" fn(*mut c_void) -> *mut c_void>,
            arg: *mut c_void,
        ) -> pth_t;
        pub fn pth_yield(tid: pth_t) -> c_int;
        pub fn pth_join(tid: pth_t, retval: *mut *mut c_void) -> c_int;
    }
    
    #[repr(C)]
    pub struct FILE {
        _unused: [u8; 0],
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
enum PthAttr {
    Prio = 0,
    Name = 1,
    Joinable = 2,
    CancelState = 3,
    StackSize = 4,
    StackAddr = 5,
    Dispatches = 6,
    TimeSpawn = 7,
    TimeLast = 8,
    TimeRan = 9,
    StartFunc = 10,
    StartArg = 11,
    State = 12,
    Events = 13,
    Bound = 14,
}

fn t1_func(arg: usize) -> usize {
    let mut val = arg;
    if val != 123 {
        eprintln!("*** ERROR, TEST FAILED: invalid argument value");
        exit(1);
    }

    for _ in 0..100 {
        val += 10;
        unsafe { ffi::pth_yield(ptr::null_mut()); }
    }
    val
}

fn t2_func(arg: usize) -> usize {
    let mut val = arg;
    if val < 9 {
        val += 1;
        eprintln!("Spawning thread {}", val);

        let tid = unsafe {
            ffi::pth_spawn(
                ptr::null_mut(),
                Some(t2_func_wrapper),
                val as *mut libc::c_void,
            )
        };

        if tid.is_null() {
            eprintln!("*** ERROR, TEST FAILED: failed to spawn thread");
            exit(1);
        }

        let mut rval = ptr::null_mut();
        let rc = unsafe { ffi::pth_join(tid, &mut rval) };
        eprintln!("Joined thread {}", val);

        if rc == 0 {
            eprintln!("*** ERROR, TEST FAILED: thread join failed");
            exit(1);
        }

        val * (rval as usize)
    } else {
        val
    }
}

unsafe extern "C" fn t1_func_wrapper(arg: *mut libc::c_void) -> *mut libc::c_void {
    t1_func(arg as usize) as *mut libc::c_void
}

unsafe extern "C" fn t2_func_wrapper(arg: *mut libc::c_void) -> *mut libc::c_void {
    t2_func(arg as usize) as *mut libc::c_void
}

fn main() {
    eprintln!("\n=== TESTING GLOBAL LIBRARY API ===\n");

    eprintln!("Fetching library version");
    let version = unsafe { ffi::pth_version() };
    if version == 0 {
        eprintln!("*** ERROR, TEST FAILED: invalid version");
        exit(1);
    }
    eprintln!("version = 0x{:X}", version);

    eprintln!("\n=== TESTING BASIC OPERATION ===\n");

    eprintln!("Initializing Pth system (spawns scheduler and main thread)");
    let rc = unsafe { ffi::pth_init() };
    if rc == 0 {
        eprintln!("*** ERROR, TEST FAILED: initialization failed");
        exit(1);
    }

    eprintln!("Killing Pth system for testing purposes");
    unsafe { ffi::pth_kill(); }

    eprintln!("Re-Initializing Pth system");
    let rc = unsafe { ffi::pth_init() };
    if rc == 0 {
        eprintln!("*** ERROR, TEST FAILED: reinitialization failed");
        exit(1);
    }

    eprintln!("\n=== TESTING BASIC THREAD OPERATION ===\n");

    eprintln!("Creating attribute object");
    let attr = unsafe { ffi::pth_attr_new() };
    if attr.is_null() {
        eprintln!("*** ERROR, TEST FAILED: attribute creation failed");
        exit(1);
    }

    let name = CString::new("test1").unwrap();
    let rc = unsafe { ffi::pth_attr_set(attr, PthAttr::Name as i32, name.as_ptr()) };
    if rc == 0 {
        eprintln!("*** ERROR, TEST FAILED: name setting failed");
        exit(1);
    }

    let rc = unsafe { ffi::pth_attr_set(attr, PthAttr::Prio as i32, 5) };
    if rc == 0 {
        eprintln!("*** ERROR, TEST FAILED: priority setting failed");
        exit(1);
    }

    eprintln!("Spawning thread");
    let tid = unsafe {
        ffi::pth_spawn(
            attr,
            Some(t1_func_wrapper),
            123 as *mut libc::c_void,
        )
    };

    if tid.is_null() {
        eprintln!("*** ERROR, TEST FAILED: thread spawn failed");
        exit(1);
    }

    unsafe { ffi::pth_attr_destroy(attr); }

    eprintln!("Joining thread");
    let mut val = ptr::null_mut();
    let rc = unsafe { ffi::pth_join(tid, &mut val) };
    if rc == 0 {
        eprintln!("*** ERROR, TEST FAILED: thread join failed");
        exit(1);
    }

    if val != 1123 as *mut libc::c_void {
        eprintln!("*** ERROR, TEST FAILED: invalid return value");
        exit(1);
    }

    eprintln!("\n=== TESTING NESTED THREAD OPERATION ===\n");

    eprintln!("Spawning thread 1");
    let tid = unsafe {
        ffi::pth_spawn(
            ptr::null_mut(),
            Some(t2_func_wrapper),
            1 as *mut libc::c_void,
        )
    };

    if tid.is_null() {
        eprintln!("*** ERROR, TEST FAILED: thread spawn failed");
        exit(1);
    }

    let mut val = ptr::null_mut();
    let rc = unsafe { ffi::pth_join(tid, &mut val) };
    eprintln!("Joined thread 1");

    if rc == 0 {
        eprintln!("*** ERROR, TEST FAILED: thread join failed");
        exit(1);
    }

    let expected = 1 * 2 * 3 * 4 * 5 * 6 * 7 * 8 * 9;
    if val != expected as *mut libc::c_void {
        eprintln!("*** ERROR, TEST FAILED: invalid return value");
        exit(1);
    }

    unsafe { ffi::pth_kill(); }

    eprintln!("\nOK - ALL TESTS SUCCESSFULLY PASSED.\n");
    exit(0);
}