use std::ffi::{CStr, CString};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write, Seek, SeekFrom};
use std::mem;
use std::os::raw::{c_char, c_int, c_void};
use std::ptr;
use std::slice;

type AwkBool = c_int;
const AWK_TRUE: AwkBool = 1;
const AWK_FALSE: AwkBool = 0;

type SizeT = usize;
type SSizeT = isize;
type UInt32T = u32;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct AwkString {
    str_: *mut c_char,
    len: SizeT,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct AwkNumber {
    d: f64,
    type_: UInt32T,
    ptr: *mut c_void,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
union AwkValueUnion {
    s: AwkString,
    n: AwkNumber,
    a: *mut c_void,
    scl: *mut c_void,
    vc: *mut c_void,
    b: AwkBool,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct AwkValue {
    val_type: UInt32T,
    u: AwkValueUnion,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct AwkElement {
    next: *mut AwkElement,
    flags: UInt32T,
    index: AwkValue,
    value: AwkValue,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct AwkFlatArray {
    opaque1: *const c_void,
    opaque2: *const c_void,
    count: SizeT,
    elements: [AwkElement; 1],
}

type AwkArray = *mut c_void;
type AwkExtId = *mut c_void;

#[repr(C)]
struct AwkExtFunc {
    name: *const c_char,
    function: Option<extern "C" fn(c_int, *mut AwkValue, *mut AwkExtFunc) -> *mut AwkValue>,
    max_expected_args: SizeT,
    min_required_args: SizeT,
    suppress_lint: AwkBool,
    data: *mut c_void,
}

#[repr(C)]
struct GawkApi {
    major_version: c_int,
    minor_version: c_int,
    gmp_major_version: c_int,
    gmp_minor_version: c_int,
    mpfr_major_version: c_int,
    mpfr_minor_version: c_int,
    do_flags: [c_int; 6],
    api_add_ext_func: Option<extern "C" fn(AwkExtId, *const c_char, *mut AwkExtFunc) -> AwkBool>,
    api_register_ext_version: Option<extern "C" fn(AwkExtId, *const c_char)>,
    api_warning: Option<extern "C" fn(AwkExtId, *const c_char, ...)>,
    api_get_argument: Option<extern "C" fn(AwkExtId, SizeT, UInt32T, *mut AwkValue) -> AwkBool>,
    api_sym_lookup: Option<extern "C" fn(AwkExtId, *const c_char, *const c_char, UInt32T, *mut AwkValue) -> AwkBool>,
    api_sym_update: Option<extern "C" fn(AwkExtId, *const c_char, *const c_char, *mut AwkValue) -> AwkBool>,
    api_clear_array: Option<extern "C" fn(AwkExtId, AwkArray) -> AwkBool>,
    api_flatten_array_typed: Option<extern "C" fn(AwkExtId, AwkArray, *mut *mut AwkFlatArray, UInt32T, UInt32T) -> AwkBool>,
    api_release_flattened_array: Option<extern "C" fn(AwkExtId, AwkArray, *mut AwkFlatArray) -> AwkBool>,
    api_set_array_element: Option<extern "C" fn(AwkExtId, AwkArray, *const AwkValue, *const AwkValue) -> AwkBool>,
    api_create_array: Option<extern "C" fn(AwkExtId) -> AwkArray>,
    api_destroy_array: Option<extern "C" fn(AwkExtId, AwkArray) -> AwkBool>,
    api_malloc: Option<extern "C" fn(SizeT) -> *mut c_void>,
    api_free: Option<extern "C" fn(*mut c_void)>,
    api_fatal: Option<extern "C" fn(AwkExtId, *const c_char, ...)>,
    api_update_ERRNO_int: Option<extern "C" fn(AwkExtId, c_int)>,
}

static mut API: *const GawkApi = ptr::null();
static mut EXT_ID: AwkExtId = ptr::null_mut();
static EXT_VERSION: &'static str = "rwarray extension: version 2.1\0";

fn make_null_string(result: &mut AwkValue) -> &mut AwkValue {
    unsafe {
        ptr::write_bytes(result, 0, 1);
        result.val_type = 0; // AWK_UNDEFINED
    }
    result
}

fn make_number(num: f64, result: &mut AwkValue) -> &mut AwkValue {
    result.val_type = 1; // AWK_NUMBER
    unsafe {
        result.u.n = AwkNumber {
            d: num,
            type_: 0, // AWK_NUMBER_TYPE_DOUBLE
            ptr: ptr::null_mut(),
        };
    }
    result
}

fn r_make_string_type(
    api: *const GawkApi,
    ext_id: AwkExtId,
    string: *const c_char,
    length: SizeT,
    duplicate: AwkBool,
    result: &mut AwkValue,
    val_type: UInt32T,
) -> &mut AwkValue {
    unsafe {
        ptr::write_bytes(result, 0, 1);
        result.val_type = val_type;
        result.u.s.len = length;

        if duplicate != AWK_FALSE {
            let cp = ((*api).api_malloc.unwrap())(length + 1) as *mut c_char;
            if cp.is_null() {
                ((*api).api_fatal.unwrap())(
                    ext_id,
                    b"%s: malloc of %d bytes failed\0".as_ptr() as *const c_char,
                    b"r_make_string\0".as_ptr() as *const c_char,
                    length + 1,
                );
            }
            ptr::copy_nonoverlapping(string, cp, length);
            *cp.add(length) = 0;
            result.u.s.str_ = cp;
        } else {
            result.u.s.str_ = string as *mut c_char;
        }
    }
    result
}

fn r_make_string(
    api: *const GawkApi,
    ext_id: AwkExtId,
    string: *const c_char,
    length: SizeT,
    duplicate: AwkBool,
    result: &mut AwkValue,
) -> &mut AwkValue {
    r_make_string_type(api, ext_id, string, length, duplicate, result, 2) // AWK_STRING
}

// ... (其他函数实现类似转换)

#[no_mangle]
pub extern "C" fn dl_load(api_p: *const GawkApi, id: AwkExtId) -> c_int {
    unsafe {
        API = api_p;
        EXT_ID = id;

        if (*API).major_version != 3 || (*API).minor_version < 2 {
            eprintln!("rwarray: version mismatch with gawk!");
            eprintln!("\tmy version (API 3.2), gawk version (API {}.{})", 
                     (*API).major_version, (*API).minor_version);
            return 1;
        }

        let funcs = [
            AwkExtFunc {
                name: b"writea\0".as_ptr() as *const c_char,
                function: Some(do_writea),
                max_expected_args: 2,
                min_required_args: 2,
                suppress_lint: AWK_FALSE,
                data: ptr::null_mut(),
            },
            // ... (其他函数定义)
        ];

        let mut errors = 0;
        for func in &funcs {
            if ((*API).api_add_ext_func.unwrap())(EXT_ID, b"\0".as_ptr() as *const c_char, func as *const _ as *mut _) == AWK_FALSE {
                ((*API).api_warning.unwrap())(
                    EXT_ID,
                    b"rwarray: could not add %s\0".as_ptr() as *const c_char,
                    func.name,
                );
                errors += 1;
            }
        }

        if !EXT_VERSION.is_empty() {
            ((*API).api_register_ext_version.unwrap())(EXT_ID, EXT_VERSION.as_ptr() as *const c_char);
        }

        if errors == 0 { 1 } else { 0 }
    }
}

// ... (其他函数的Rust实现)

#[no_mangle]
pub static plugin_is_GPL_compatible: c_int = 1;