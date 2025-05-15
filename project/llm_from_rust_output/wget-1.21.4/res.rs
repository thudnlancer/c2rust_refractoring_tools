use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_ulong, c_void};
use std::ptr;
use std::mem;
use std::cmp;

#[repr(C)]
pub struct Options {
    // ... (same fields as in C struct)
}

#[repr(C)]
pub struct Iri {
    uri_encoding: *mut c_char,
    content_encoding: *mut c_char,
    orig_url: *mut c_char,
    utf8_encode: bool,
}

#[repr(C)]
pub struct FileMemory {
    content: *mut c_char,
    length: c_long,
    mmap_p: c_int,
}

#[repr(C)]
pub struct Url {
    url: *mut c_char,
    scheme: UrlScheme,
    host: *mut c_char,
    port: c_int,
    path: *mut c_char,
    params: *mut c_char,
    query: *mut c_char,
    fragment: *mut c_char,
    dir: *mut c_char,
    file: *mut c_char,
    user: *mut c_char,
    passwd: *mut c_char,
}

#[repr(u32)]
pub enum UrlScheme {
    Http = 0,
    Https = 1,
    Ftp = 2,
    Ftps = 3,
    Invalid = 4,
}

#[repr(C)]
pub struct RobotSpecs {
    count: c_int,
    size: c_int,
    paths: *mut PathInfo,
}

#[repr(C)]
pub struct PathInfo {
    path: *mut c_char,
    allowedp: bool,
    user_agent_exact_p: bool,
}

static mut OPT: Options = Options { /* initialized fields */ };
static mut REGISTERED_SPECS: *mut HashTable = ptr::null_mut();

// Helper functions
fn c_isalnum(c: c_int) -> bool {
    matches!(c, 
        b'0'..=b'9' | 
        b'a'..=b'z' | 
        b'A'..=b'Z'
    )
}

fn c_isspace(c: c_int) -> bool {
    matches!(c, b' ' | b'\t' | b'\n' | b'\v' | b'\f' | b'\r')
}

fn c_isxdigit(c: c_int) -> bool {
    matches!(c, b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F')
}

fn unhex(c: u8) -> u8 {
    match c {
        b'0'..=b'9' => c - b'0',
        b'A'..=b'F' => c - b'A' + 10,
        b'a'..=b'f' => c - b'a' + 10,
        _ => 0,
    }
}

// Main functions
fn match_user_agent(agent: &[u8], matches: &mut bool, exact_match: &mut bool) {
    if agent.len() == 1 && agent[0] == b'*' {
        *matches = true;
        *exact_match = false;
    } else if agent.len() == 4 && agent.eq_ignore_ascii_case(b"wget") {
        *matches = true;
        *exact_match = true;
    } else {
        *matches = false;
        *exact_match = false;
    }
}

fn add_path(specs: &mut RobotSpecs, path: &[u8], allowedp: bool, exactp: bool) {
    unsafe {
        let pp = PathInfo {
            path: CString::new(path).unwrap().into_raw(),
            allowedp,
            user_agent_exact_p: exactp,
        };
        
        specs.count += 1;
        if specs.count > specs.size {
            specs.size = if specs.size == 0 { 1 } else { specs.size << 1 };
            specs.paths = libc::realloc(
                specs.paths as *mut c_void,
                (specs.size as usize) * mem::size_of::<PathInfo>()
            ) as *mut PathInfo;
        }
        
        *specs.paths.offset((specs.count - 1) as isize) = pp;
    }
}

fn prune_non_exact(specs: &mut RobotSpecs) {
    unsafe {
        let mut cnt = 0;
        for i in 0..specs.count {
            if (*specs.paths.offset(i as isize)).user_agent_exact_p {
                cnt += 1;
            }
        }
        
        let newpaths = libc::malloc((cnt as usize) * mem::size_of::<PathInfo>()) as *mut PathInfo;
        let mut j = 0;
        
        for i in 0..specs.count {
            if (*specs.paths.offset(i as isize)).user_agent_exact_p {
                *newpaths.offset(j as isize) = *specs.paths.offset(i as isize);
                j += 1;
            } else {
                libc::free((*specs.paths.offset(i as isize)).path as *mut c_void);
            }
        }
        
        libc::free(specs.paths as *mut c_void);
        specs.paths = newpaths;
        specs.count = cnt;
        specs.size = cnt;
    }
}

// ... (other functions similarly translated)

fn res_parse_from_file(filename: &str) -> Option<Box<RobotSpecs>> {
    unsafe {
        let fm = wget_read_file(CString::new(filename).unwrap().as_ptr());
        if fm.is_null() {
            logprintf(
                LOG_NOTQUIET,
                format!("Cannot open {}: {}\n", filename, CStr::from_ptr(strerror(*__errno_location())).to_str().unwrap())
            );
            return None;
        }
        
        let specs = res_parse((*fm).content, (*fm).length as c_int);
        wget_read_file_free(fm);
        Some(Box::from_raw(specs))
    }
}

fn free_specs(specs: Box<RobotSpecs>) {
    unsafe {
        for i in 0..specs.count {
            libc::free((*specs.paths.offset(i as isize)).path as *mut c_void);
        }
        libc::free(specs.paths as *mut c_void);
    }
}

// ... (remaining functions similarly translated)

fn is_robots_txt_url(url: &str) -> bool {
    unsafe {
        let robots_url = uri_merge(
            CString::new(url).unwrap().as_ptr(),
            CString::new("/robots.txt").unwrap().as_ptr()
        );
        let ret = are_urls_equal(
            CString::new(url).unwrap().as_ptr(),
            robots_url
        );
        libc::free(robots_url as *mut c_void);
        ret
    }
}