use std::ffi::{CStr, CString};
use std::ptr;
use std::sync::Once;
use std::sync::atomic::{AtomicPtr, Ordering};
use std::mem;

static ENV_PTR: AtomicPtr<Env> = AtomicPtr::new(ptr::null_mut());

#[derive(Debug)]
struct Mbd {
    size: usize,
    prev: *mut Mbd,
    next: *mut Mbd,
}

#[derive(Debug)]
struct Env {
    term_buf: Vec<u8>,
    term_out: i32,
    term_hook: Option<Box<dyn Fn(&str) -> i32>>,
    term_info: Option<Box<dyn std::any::Any>>,
    tee_file: Option<std::fs::File>,
    err_buf: Vec<u8>,
    mem_limit: usize,
    mem_ptr: Option<Box<Mbd>>,
    mem_count: i32,
    mem_cpeak: i32,
    mem_total: usize,
    mem_tpeak: usize,
    h_odbc: Option<Box<dyn std::any::Any>>,
    h_mysql: Option<Box<dyn std::any::Any>>,
}

impl Env {
    fn new() -> Result<Self, i32> {
        let term_buf = vec![0u8; 4096];
        let err_buf = vec![0u8; 1024];
        
        Ok(Env {
            term_buf,
            term_out: 1,
            term_hook: None,
            term_info: None,
            tee_file: None,
            err_buf,
            mem_limit: usize::MAX,
            mem_ptr: None,
            mem_count: 0,
            mem_cpeak: 0,
            mem_total: 0,
            mem_tpeak: 0,
            h_odbc: None,
            h_mysql: None,
        })
    }
}

fn glp_init_env() -> i32 {
    static INIT: Once = Once::new();
    let mut result = 0;
    
    INIT.call_once(|| {
        if mem::size_of::<usize>() != 8 || mem::size_of::<u8>() != 1 || 
           mem::size_of::<u16>() != 2 || mem::size_of::<i32>() != 4 {
            result = 3;
            return;
        }
        
        match Env::new() {
            Ok(env) => {
                let env_ptr = Box::into_raw(Box::new(env));
                ENV_PTR.store(env_ptr, Ordering::SeqCst);
                result = 0;
            }
            Err(_) => {
                result = 2;
            }
        }
    });
    
    result
}

fn _glp_get_env_ptr() -> &'static mut Env {
    let env_ptr = ENV_PTR.load(Ordering::SeqCst);
    if env_ptr.is_null() {
        if glp_init_env() != 0 {
            eprintln!("GLPK initialization failed");
            std::process::abort();
        }
        ENV_PTR.load(Ordering::SeqCst)
    } else {
        unsafe { &mut *env_ptr }
    }
}

fn glp_version() -> &'static str {
    "5.0"
}

fn glp_config(option: &str) -> Option<&'static str> {
    match option {
        "TLS" => Some("_Thread_local"),
        "ODBC_DLNAME" | "MYSQL_DLNAME" => None,
        _ => None,
    }
}

fn glp_free_env() -> i32 {
    let env_ptr = ENV_PTR.swap(ptr::null_mut(), Ordering::SeqCst);
    if !env_ptr.is_null() {
        unsafe { Box::from_raw(env_ptr) };
        0
    } else {
        1
    }
}