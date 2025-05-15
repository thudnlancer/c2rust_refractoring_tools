use std::fs::File;
use std::io::{Read, Error as IoError};
use std::path::{Path, PathBuf};
use std::env;
use std::ffi::{CString, CStr};
use std::ptr;
use std::mem;

struct RcParser {
    argc: i32,
    argv: Vec<*mut libc::c_char>,
}

impl RcParser {
    fn new() -> Self {
        RcParser {
            argc: 0,
            argv: Vec::new(),
        }
    }

    fn expand_argcv(&mut self, new_args: Vec<String>) {
        let mut c_args: Vec<*mut libc::c_char> = new_args
            .into_iter()
            .map(|s| CString::new(s).unwrap().into_raw())
            .collect();
        self.argv.append(&mut c_args);
        self.argc += c_args.len() as i32;
    }

    fn parse_rc(&mut self, path: &Path) -> Result<(), IoError> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        for line in contents.lines() {
            let args: Vec<String> = line.split_whitespace()
                .filter(|s| !s.starts_with('#'))
                .map(|s| s.to_string())
                .collect();
            if !args.is_empty() {
                self.expand_argcv(args);
            }
        }
        Ok(())
    }

    fn sourcerc(&mut self) {
        if let Ok(env) = env::var("CFLOW_OPTIONS") {
            let args: Vec<String> = env.split_whitespace()
                .filter(|s| !s.starts_with('#'))
                .map(|s| s.to_string())
                .collect();
            if !args.is_empty() {
                self.expand_argcv(args);
            }
        }

        if let Ok(rc_path) = env::var("CFLOWRC") {
            let path = Path::new(&rc_path);
            let _ = self.parse_rc(path);
        } else if let Ok(home) = env::var("HOME") {
            let mut path = PathBuf::from(home);
            path.push(".cflowrc");
            let _ = self.parse_rc(&path);
        }
    }

    fn into_raw(self) -> (*mut i32, *mut *mut libc::c_char) {
        let mut argv = self.argv;
        argv.push(ptr::null_mut());
        let argv_ptr = argv.as_mut_ptr();
        let argc = self.argc;
        mem::forget(argv);
        (Box::into_raw(Box::new(argc)), argv_ptr)
    }
}

pub fn parse_arguments() -> (i32, Vec<String>) {
    let mut parser = RcParser::new();
    parser.sourcerc();

    let mut args: Vec<String> = env::args().collect();
    if parser.argc > 0 {
        let c_args: Vec<String> = unsafe {
            parser.argv[..parser.argc as usize]
                .iter()
                .map(|&ptr| CStr::from_ptr(ptr).to_string_lossy().into_owned())
                .collect()
        };
        args.extend(c_args);
    }

    (args.len() as i32, args)
}