use std::ffi::{CStr, CString};
use std::fs::{File, Metadata};
use std::io::{Read, Error as IoError, ErrorKind};
use std::os::unix::fs::MetadataExt;
use std::path::Path;
use std::ptr;
use libc::{c_char, c_int, c_void, size_t, ssize_t};

// Constants and types
const GAWK_API_MAJOR_VERSION: c_int = 3;
const GAWK_API_MINOR_VERSION: c_int = 2;

type AwkBool = u32;
const AWK_TRUE: AwkBool = 1;
const AWK_FALSE: AwkBool = 0;

#[repr(u32)]
enum AwkValType {
    Undefined = 0,
    Number = 1,
    String = 2,
    Regex = 3,
    Strnum = 4,
    Array = 5,
    Scalar = 6,
    ValueCookie = 7,
    Bool = 8,
}

#[repr(C)]
struct AwkString {
    str_: *mut c_char,
    len: size_t,
}

#[repr(C)]
struct AwkNumber {
    d: f64,
    type_: u32,
    ptr: *mut c_void,
}

#[repr(C)]
union AwkValueUnion {
    s: AwkString,
    n: AwkNumber,
    a: *mut c_void,
    scl: *mut c_void,
    vc: *mut c_void,
    b: AwkBool,
}

#[repr(C)]
struct AwkValue {
    val_type: AwkValType,
    u: AwkValueUnion,
}

#[repr(C)]
struct AwkExtFunc {
    name: *const c_char,
    function: Option<extern "C" fn(c_int, *mut AwkValue, *mut AwkExtFunc) -> *mut AwkValue>,
    max_expected_args: size_t,
    min_required_args: size_t,
    suppress_lint: AwkBool,
    data: *mut c_void,
}

#[repr(C)]
struct GawkApi {
    major_version: c_int,
    minor_version: c_int,
    // ... other fields omitted for brevity
    api_add_ext_func: Option<extern "C" fn(*mut c_void, *const c_char, *mut AwkExtFunc) -> AwkBool>,
    api_fatal: Option<extern "C" fn(*mut c_void, *const c_char, ...) -> ()>,
    api_malloc: Option<extern "C" fn(size_t) -> *mut c_void>,
    api_free: Option<extern "C" fn(*mut c_void) -> ()>,
    api_unset_errno: Option<extern "C" fn(*mut c_void) -> ()>,
    api_update_errno_int: Option<extern "C" fn(*mut c_void, c_int) -> ()>,
    api_get_argument: Option<extern "C" fn(*mut c_void, size_t, AwkValType, *mut AwkValue) -> AwkBool>,
    // ... other API functions omitted
}

static mut API: *const GawkApi = ptr::null();
static mut EXT_ID: *mut c_void = ptr::null_mut();
static EXT_VERSION: &'static CStr = c"readfile extension: version 2.0";

fn read_file_to_buffer(path: &Path) -> Result<Vec<u8>, IoError> {
    let mut file = File::open(path)?;
    let metadata = file.metadata()?;
    
    if !metadata.is_file() {
        return Err(IoError::new(ErrorKind::InvalidInput, "Not a regular file"));
    }

    let mut buffer = Vec::with_capacity(metadata.size() as usize + 1);
    file.read_to_end(&mut buffer)?;
    buffer.push(b'\0');
    Ok(buffer)
}

extern "C" fn do_readfile(
    nargs: c_int,
    result: *mut AwkValue,
    _unused: *mut AwkExtFunc,
) -> *mut AwkValue {
    unsafe {
        let api = API;
        let ext_id = EXT_ID;

        ((*api).api_unset_errno.unwrap())(ext_id);

        let mut filename = AwkValue {
            val_type: AwkValType::Undefined,
            u: AwkValueUnion { s: AwkString { str_: ptr::null_mut(), len: 0 } },
        };

        if ((*api).api_get_argument.unwrap())(ext_id, 0, AwkValType::String, &mut filename) == AWK_FALSE {
            return result;
        }

        let c_str = unsafe { CStr::from_ptr(filename.u.s.str_) };
        let path = Path::new(c_str.to_str().unwrap());

        match read_file_to_buffer(path) {
            Ok(data) => {
                let str_ptr = data.as_ptr() as *const c_char;
                let len = data.len() - 1;
                
                let mut res = AwkValue {
                    val_type: AwkValType::String,
                    u: AwkValueUnion {
                        s: AwkString {
                            str_: str_ptr as *mut c_char,
                            len,
                        },
                    },
                };
                
                ptr::write(result, res);
            }
            Err(e) => {
                ((*api).api_update_errno_int.unwrap())(ext_id, e.raw_os_error().unwrap_or(0));
            }
        }

        result
    }
}

static FUNC_TABLE: [AwkExtFunc; 1] = [
    AwkExtFunc {
        name: c"readfile\0".as_ptr(),
        function: Some(do_readfile),
        max_expected_args: 1,
        min_required_args: 1,
        suppress_lint: AWK_FALSE,
        data: ptr::null_mut(),
    },
];

#[no_mangle]
pub extern "C" fn dl_load(api_p: *const GawkApi, id: *mut c_void) -> c_int {
    unsafe {
        API = api_p;
        EXT_ID = id;

        let api = API;
        let ext_id = EXT_ID;

        if (*api).major_version != GAWK_API_MAJOR_VERSION || (*api).minor_version < GAWK_API_MINOR_VERSION {
            eprintln!("readfile: version mismatch with gawk!");
            eprintln!("\tmy version (API {}.{}), gawk version (API {}.{})",
                GAWK_API_MAJOR_VERSION, GAWK_API_MINOR_VERSION,
                (*api).major_version, (*api).minor_version);
            return 0;
        }

        let mut errors = 0;

        for func in &FUNC_TABLE {
            if ((*api).api_add_ext_func.unwrap())(ext_id, c"\0".as_ptr(), func as *const _ as *mut _) == AWK_FALSE {
                errors += 1;
            }
        }

        if errors == 0 { 1 } else { 0 }
    }
}

#[no_mangle]
pub static plugin_is_GPL_compatible: c_int = 1;