use std::env;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::path::{Path, PathBuf};
use std::process;
use std::str;

mod base {
    pub struct Top;
    pub struct Divvy;
    pub struct Yacmd {
        pub aka: *const u8,
        pub func: fn(*const c_char, i32, *const *const c_char) -> i32,
        pub pr: *const Pr,
    }
    pub struct Pr {
        pub desc: *const c_char,
    }
    pub struct Tinysym {
        pub len: usize,
        pub bytes: [u8; 0],
    }
    pub fn looking_at(sym: &Tinysym, maybe: &str) -> bool {
        unsafe {
            let sym_bytes = std::slice::from_raw_parts(sym.bytes.as_ptr(), sym.len);
            maybe.as_bytes() == sym_bytes
        }
    }
    pub fn one_beyond_last_dir_sep(path: &str) -> Option<&str> {
        Path::new(path).file_name().and_then(|s| s.to_str())
    }
    pub fn str_save(s: &str) -> *const c_char {
        CString::new(s).unwrap().into_raw()
    }
}

use base::*;

struct DynamicRoot {
    top: *mut Top,
    single: *mut Divvy,
    plexus: *mut Divvy,
}

impl DynamicRoot {
    fn global_to_stack(&mut self) {
        unsafe {
            self.top = TOP;
            self.single = SINGLE;
            self.plexus = PLEXUS;
        }
    }

    fn stack_to_global(&self) {
        unsafe {
            TOP = self.top;
            SINGLE = self.single;
            PLEXUS = self.plexus;
        }
    }
}

static mut TOP: *mut Top = std::ptr::null_mut();
static mut SINGLE: *mut Divvy = std::ptr::null_mut();
static mut PLEXUS: *mut Divvy = std::ptr::null_mut();

type SubmainT = fn(*const c_char, i32, *const *const c_char) -> i32;

fn dispatch(exitval: &mut i32, sub: SubmainT, cmd: &str, argc: i32, argv: *const *const c_char) {
    let mut super_root = DynamicRoot {
        top: std::ptr::null_mut(),
        single: std::ptr::null_mut(),
        plexus: std::ptr::null_mut(),
    };

    super_root.global_to_stack();
    *exitval = sub(CString::new(cmd).unwrap().as_ptr(), argc, argv);
    super_root.stack_to_global();
}

extern "C" {
    fn YA_ci() -> Yacmd;
    fn YA_co() -> Yacmd;
    fn YA_rcs() -> Yacmd;
    fn YA_rcsclean() -> Yacmd;
    fn YA_rcsdiff() -> Yacmd;
    fn YA_rcsmerge() -> Yacmd;
    fn YA_rlog() -> Yacmd;
}

fn avail() -> Vec<&'static Yacmd> {
    unsafe {
        vec![
            &YA_ci(),
            &YA_co(),
            &YA_rcs(),
            &YA_rcsclean(),
            &YA_rcsdiff(),
            &YA_rcsmerge(),
            &YA_rlog(),
        ]
    }
}

fn recognize(maybe: &str) -> Option<SubmainT> {
    let mlen = maybe.len();
    for y in avail() {
        unsafe {
            let aka = y.aka;
            let mut count = *aka;
            let mut aka_ptr = aka.add(1);

            for _ in 0..count {
                let sym = aka_ptr as *const Tinysym;
                if mlen == (*sym).len && looking_at(&*sym, maybe) {
                    return Some(y.func);
                }
                aka_ptr = aka_ptr.add((*sym).len + 1);
            }
        }
    }
    None
}

const MAX_COMMAND_SIZE: usize = 64;

fn string_from_sym(dest: &mut [u8; MAX_COMMAND_SIZE], sym: &Tinysym) {
    let len = std::cmp::min(MAX_COMMAND_SIZE - 1, sym.len);
    unsafe {
        std::ptr::copy_nonoverlapping(sym.bytes.as_ptr(), dest.as_mut_ptr(), len);
    }
    dest[len] = b'\0';
}

fn display_commands() {
    println!("{:<10}  {}", "(command)", "(description)");
    for y in avail() {
        unsafe {
            let aka = y.aka;
            let mut name = [0u8; MAX_COMMAND_SIZE];
            string_from_sym(&mut name, &*(aka.add(1) as *const Tinysym));
            let name_str = CStr::from_bytes_until_nul(&name).unwrap().to_str().unwrap();
            let desc = CStr::from_ptr((*y.pr).desc).to_str().unwrap();
            println!(" {:<10}  {}", name_str, desc);
        }
    }
}

fn display_aliases() {
    println!("{:<10}  {}", "(command)", "(aliases)");
    for y in avail() {
        unsafe {
            let aka = y.aka;
            let mut count = *aka;
            let mut aka_ptr = aka.add(1);

            for j in 0..count {
                let sym = aka_ptr as *const Tinysym;
                let mut name = [0u8; MAX_COMMAND_SIZE];
                string_from_sym(&mut name, &*sym);

                let name_str = CStr::from_bytes_until_nul(&name).unwrap().to_str().unwrap();
                match j {
                    0 => print!(" {:<10} ", name_str),
                    1 => print!(" {}", name_str),
                    _ => print!(", {}", name_str),
                }
                aka_ptr = aka_ptr.add((*sym).len + 1);
            }
            println!();
        }
    }
}

fn all_options_short_p(argv: &[String]) -> bool {
    for arg in argv.iter().skip(1) {
        if !arg.starts_with('-') {
            break;
        }
        if arg.starts_with("--") {
            return false;
        }
    }
    true
}

fn huh(what: &str, argv1: &str) -> ! {
    eprintln!("unknown {}: {}{}", what, argv1, " (try --help)");
    process::exit(1);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let argc = args.len() as i32;
    let mut argv: Vec<*const c_char> = args.iter().map(|s| CString::new(s.as_str()).unwrap().as_ptr()).collect();
    argv.push(std::ptr::null());

    if argc == 3 && args[1] == "--help" {
        argv.swap(1, 2);
    }

    let mut exitval = 0;
    let cmd = if argc < 2 || (args[1].starts_with('-') && all_options_short_p(&args)) {
        "rcs"
    } else {
        &args[1]
    };

    if let Some(sub) = recognize(cmd) {
        dispatch(&mut exitval, sub, cmd, argc - 1, argv[1..].as_ptr());
    } else if cmd.contains('/') || Path::new(cmd).exists() {
        let sub = recognize("rcs").unwrap();
        dispatch(&mut exitval, sub, "rcs", argc, argv.as_ptr());
    } else {
        huh("command", cmd);
    }

    process::exit(exitval);
}