use std::ffi::{CStr, CString};
use std::fmt;
use std::fs::File;
use std::io::{self, Write};
use std::os::raw::{c_char, c_int, c_void};
use std::ptr;

const TBUF_SIZE: usize = 4096;

struct Env {
    term_out: c_int,
    term_hook: Option<Box<dyn FnMut(*mut c_void, &CStr) -> c_int>>,
    term_info: *mut c_void,
    tee_file: Option<File>,
    term_buf: Box<[u8; TBUF_SIZE]>,
}

impl Env {
    fn get() -> &'static mut Self {
        unsafe {
            &mut *super::_glp_get_env_ptr() as *mut _ as *mut Self
        }
    }
}

pub fn glp_puts(s: &CStr) {
    let env = Env::get();
    if env.term_out == 0 {
        return;
    }

    if let Some(ref mut hook) = env.term_hook {
        if hook(env.term_info, s) != 0 {
            return;
        }
    }

    io::stdout().write_all(s.to_bytes()).unwrap();
    io::stdout().flush().unwrap();

    if let Some(ref mut file) = env.tee_file {
        file.write_all(s.to_bytes()).unwrap();
        file.flush().unwrap();
    }
}

pub fn glp_printf(fmt: &CStr, args: fmt::Arguments) {
    let env = Env::get();
    if env.term_out == 0 {
        return;
    }

    let mut buf = String::new();
    fmt::write(&mut buf, args).unwrap();
    let c_str = CString::new(buf).unwrap();

    assert!(c_str.to_bytes().len() < TBUF_SIZE);
    env.term_buf[..c_str.to_bytes().len()].copy_from_slice(c_str.to_bytes());
    glp_puts(&c_str);
}

pub fn glp_term_out(flag: c_int) -> c_int {
    assert!(flag == 0 || flag == 1, "glp_term_out: invalid parameter");
    let env = Env::get();
    let old = env.term_out;
    env.term_out = flag;
    old
}

pub fn glp_term_hook<F>(func: Option<F>, info: *mut c_void) 
where
    F: FnMut(*mut c_void, &CStr) -> c_int + 'static,
{
    let env = Env::get();
    env.term_hook = func.map(|f| Box::new(f) as Box<_>);
    env.term_info = info;
}

pub fn glp_open_tee(name: &CStr) -> c_int {
    let env = Env::get();
    if env.tee_file.is_some() {
        return 1;
    }

    match File::create(name.to_str().unwrap()) {
        Ok(file) => {
            env.tee_file = Some(file);
            0
        }
        Err(_) => 2,
    }
}

pub fn glp_close_tee() -> c_int {
    let env = Env::get();
    if env.tee_file.is_none() {
        return 1;
    }
    env.tee_file = None;
    0
}