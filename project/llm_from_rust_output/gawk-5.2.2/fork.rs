use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_double, c_int, c_uint, c_ulong, c_void};
use std::ptr;
use std::process::{Command, exit};

const AWK_STRING: u32 = 2;
const AWK_NUMBER: u32 = 1;
const AWK_ARRAY: u32 = 5;
const AWK_UNDEFINED: u32 = 0;
const awk_true: u32 = 1;
const awk_false: u32 = 0;

#[derive(Debug, Clone)]
struct AwkString {
    str_: CString,
    len: usize,
}

#[derive(Debug, Clone)]
struct AwkNumber {
    d: c_double,
    type_: u32,
    ptr: *mut c_void,
}

#[derive(Debug, Clone)]
enum AwkValue {
    String(AwkString),
    Number(AwkNumber),
    Array(*mut c_void),
    Undefined,
}

impl AwkValue {
    fn new_string(s: &str, duplicate: bool) -> Self {
        let cstr = CString::new(s).unwrap();
        let len = cstr.as_bytes().len();
        AwkValue::String(AwkString {
            str_: if duplicate { cstr.clone() } else { cstr },
            len,
        })
    }

    fn new_number(num: c_double) -> Self {
        AwkValue::Number(AwkNumber {
            d: num,
            type_: 0,
            ptr: ptr::null_mut(),
        })
    }
}

struct GawkApi {
    major_version: c_int,
    minor_version: c_int,
}

impl GawkApi {
    fn new() -> Self {
        GawkApi {
            major_version: 3,
            minor_version: 2,
        }
    }

    fn update_errno(&self, err: c_int) {
        unsafe { *libc::__errno_location() = err };
    }

    fn make_string(&self, s: &str, duplicate: bool) -> AwkValue {
        AwkValue::new_string(s, duplicate)
    }

    fn make_number(&self, num: c_double) -> AwkValue {
        AwkValue::new_number(num)
    }
}

struct ForkExtension {
    api: GawkApi,
}

impl ForkExtension {
    fn new(api: GawkApi) -> Self {
        ForkExtension { api }
    }

    fn do_fork(&self) -> AwkValue {
        match unsafe { libc::fork() } {
            -1 => {
                self.api.update_errno(unsafe { *libc::__errno_location() });
                self.api.make_number(-1.0)
            }
            0 => {
                // Child process
                let pid = unsafe { libc::getpid() } as c_double;
                let ppid = unsafe { libc::getppid() } as c_double;
                // In a real implementation, would update PROCINFO here
                self.api.make_number(0.0)
            }
            ret => self.api.make_number(ret as c_double),
        }
    }

    fn do_waitpid(&self, pid: c_int) -> AwkValue {
        let options = 1 | 2; // WNOHANG | WUNTRACED
        match unsafe { libc::waitpid(pid, ptr::null_mut(), options) } {
            -1 => {
                self.api.update_errno(unsafe { *libc::__errno_location() });
                self.api.make_number(-1.0)
            }
            ret => self.api.make_number(ret as c_double),
        }
    }

    fn do_wait(&self) -> AwkValue {
        match unsafe { libc::wait(ptr::null_mut()) } {
            -1 => {
                self.api.update_errno(unsafe { *libc::__errno_location() });
                self.api.make_number(-1.0)
            }
            ret => self.api.make_number(ret as c_double),
        }
    }
}

#[no_mangle]
pub extern "C" fn dl_load(api: *const GawkApi, _id: *mut c_void) -> c_int {
    let api = unsafe { &*api };
    if api.major_version != 3 || api.minor_version < 2 {
        eprintln!("fork: version mismatch with gawk!");
        eprintln!("\tmy version (API 3.2), gawk version (API {}.{})", 
                 api.major_version, api.minor_version);
        exit(1);
    }

    let ext = ForkExtension::new(api.clone());
    
    // In a real implementation, would register functions here
    // For now just return success
    1
}

#[no_mangle]
pub static plugin_is_GPL_compatible: c_int = 1;