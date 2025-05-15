use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};
use std::ptr;
use std::sync::OnceLock;

#[derive(Debug)]
struct Env {
    err_st: c_int,
    err_file: Option<CString>,
    err_line: c_int,
    err_hook: Option<Box<dyn FnMut()>>,
    err_buf: Vec<c_char>,
}

static ENV: OnceLock<Env> = OnceLock::new();

fn get_env() -> &'static mut Env {
    ENV.get_or_init(|| Env {
        err_st: 0,
        err_file: None,
        err_line: 0,
        err_hook: None,
        err_buf: vec![0; 1024],
    })
}

pub type GlpErrFunc = Box<dyn Fn(&str)>;

fn errfunc(fmt: &str, args: std::fmt::Arguments) {
    let env = get_env();
    env.err_st = 1;
    
    eprint!("{}", args);
    eprintln!(
        "Error detected in file {:?} at line {}",
        env.err_file.as_ref().map(|s| s.to_string_lossy()),
        env.err_line
    );

    if let Some(ref mut hook) = env.err_hook {
        hook();
    }

    std::process::abort();
}

pub fn glp_error(file: &str, line: c_int) -> GlpErrFunc {
    let env = get_env();
    env.err_file = Some(CString::new(file).unwrap());
    env.err_line = line;
    
    Box::new(move |msg| {
        errfunc(msg, format_args!("Assertion failed: {}\n", msg));
    })
}

pub fn glp_at_error() -> c_int {
    get_env().err_st
}

pub fn glp_assert(expr: &str, file: &str, line: c_int) {
    let err_func = glp_error(file, line);
    err_func(expr);
}

pub fn glp_error_hook<F: FnMut() + 'static>(func: Option<F>) {
    let env = get_env();
    env.err_hook = func.map(|f| Box::new(f) as _);
}

pub fn _glp_put_err_msg(msg: &str) {
    let env = get_env();
    let mut len = msg.len().min(1023);
    let msg_bytes = msg.as_bytes();
    
    if len > 0 && msg_bytes[len - 1] == b'\n' {
        len -= 1;
    }
    
    unsafe {
        ptr::copy_nonoverlapping(
            msg_bytes.as_ptr() as *const c_void,
            env.err_buf.as_mut_ptr() as *mut c_void,
            len,
        );
        *env.err_buf.get_unchecked_mut(len) = 0;
    }
}

pub fn _glp_get_err_msg() -> &'static CStr {
    let env = get_env();
    unsafe { CStr::from_ptr(env.err_buf.as_ptr()) }
}