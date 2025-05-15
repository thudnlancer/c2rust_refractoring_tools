use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};
use std::ptr;
use std::process::exit;

#[repr(C)]
pub struct MainParam {
    loop_fn: Option<extern "C" fn(*mut Stream, *mut MainParam, *const c_char) -> c_int>,
    dir_callback: Option<extern "C" fn(*mut DirEntry, *mut MainParam) -> c_int>,
    callback: Option<extern "C" fn(*mut DirEntry, *mut MainParam) -> c_int>,
    unix_callback: Option<extern "C" fn(*mut MainParam) -> c_int>,
    arg: *mut c_void,
    open_flags: c_int,
    lookup_flags: c_int,
    fast_quit: c_int,
    short_name: BoundedString,
    long_name: BoundedString,
    file: *mut Stream,
    dir_entry: *mut DirEntry,
    unix_source_name: *mut c_char,
    target_dir: *mut Stream,
    target_name: *const c_char,
    original_arg: *mut c_char,
    basename_has_wildcard: c_int,
    mcwd: [c_char; 132],
    target_buffer: [c_char; 1021],
}

#[repr(C)]
pub struct BoundedString {
    data: *mut c_char,
    len: usize,
}

#[repr(C)]
pub struct Stream {
    class: *mut Class,
    refs: c_int,
    next: *mut Stream,
}

#[repr(C)]
pub struct Class {
    // ... class methods would be defined here
}

#[repr(C)]
pub struct DirEntry {
    dir: *mut Stream,
    entry: c_int,
    // ... other fields
    name: [c_int; 256],
    begin_slot: u32,
    end_slot: u32,
}

extern "C" {
    fn fprint_short_pwd(file: *mut FILE, entry: *mut DirEntry);
    fn main_loop(mp: *mut MainParam, argv: *mut *mut c_char, argc: c_int) -> c_int;
    fn init_mp(mp: *mut MainParam);
    fn set_cmd_line_image(img: *mut c_char);
    fn getopt(argc: c_int, argv: *const *mut c_char, optstr: *const c_char) -> c_int;
    static mut optarg: *mut c_char;
    static mut optind: c_int;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    static mut progname: *const c_char;
    static mut mversion: *const c_char;
    static mut mdate: *const c_char;
}

type FILE = std::ffi::c_void;

fn print_short_name(entry: *mut DirEntry, mp: *mut MainParam) -> c_int {
    unsafe {
        fprint_short_pwd(stdout, entry);
        println!();
    }
    4
}

fn usage(ret: c_int) -> ! {
    unsafe {
        eprintln!(
            "Mtools version {}, dated {}",
            CStr::from_ptr(mversion).to_string_lossy(),
            CStr::from_ptr(mdate).to_string_lossy()
        );
        eprintln!(
            "Usage: {} msdosfile [msdosfiles...]",
            CStr::from_ptr(progname).to_string_lossy()
        );
    }
    exit(ret);
}

pub extern "C" fn mshortname(argc: c_int, argv: *mut *mut c_char, _type: c_int) {
    let mut mp = MainParam {
        loop_fn: None,
        dir_callback: None,
        callback: Some(print_short_name),
        unix_callback: None,
        arg: ptr::null_mut(),
        open_flags: 0,
        lookup_flags: 0x20 | 0x10,
        fast_quit: 0,
        short_name: BoundedString {
            data: ptr::null_mut(),
            len: 0,
        },
        long_name: BoundedString {
            data: ptr::null_mut(),
            len: 0,
        },
        file: ptr::null_mut(),
        dir_entry: ptr::null_mut(),
        unix_source_name: ptr::null_mut(),
        target_dir: ptr::null_mut(),
        target_name: ptr::null(),
        original_arg: ptr::null_mut(),
        basename_has_wildcard: 0,
        mcwd: [0; 132],
        target_buffer: [0; 1021],
    };

    unsafe {
        init_mp(&mut mp);

        let optstr = CString::new("i:h").unwrap();
        loop {
            let c = getopt(argc, argv, optstr.as_ptr());
            if c == -1 {
                break;
            }
            match c {
                b'i' as c_int => set_cmd_line_image(optarg),
                b'h' as c_int => usage(0),
                _ => usage(1),
            }
        }

        if optind == argc || optind >= argc {
            usage(1);
        }

        exit(main_loop(&mut mp, argv.offset(optind as isize), argc - optind));
    }
}