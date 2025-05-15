use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void, c_double, c_uint, c_long, c_ulong};
use std::mem;
use std::ptr;
use std::path::Path;
use std::fs;
use std::io;
use libc::{stat, statvfs, dev_t, ino_t, mode_t, nlink_t, uid_t, gid_t, off_t, blksize_t, blkcnt_t, time_t, timespec, size_t, ssize_t};

// Constants
const GAWK_API_MAJOR_VERSION: c_int = 3;
const GAWK_API_MINOR_VERSION: c_int = 2;
const AWK_STRING: c_uint = 2;
const AWK_NUMBER: c_uint = 1;
const AWK_ARRAY: c_uint = 5;
const AWK_UNDEFINED: c_uint = 0;
const AWK_TRUE: c_uint = 1;
const AWK_FALSE: c_uint = 0;

// Types
type awk_bool_t = c_uint;
type awk_valtype_t = c_uint;
type awk_array_t = *mut c_void;
type awk_ext_id_t = *mut c_void;

#[repr(C)]
struct awk_string {
    str_: *mut c_char,
    len: size_t,
}

#[repr(C)]
struct awk_number {
    d: c_double,
    type_: c_uint,
    ptr: *mut c_void,
}

#[repr(C)]
union awk_value_union {
    s: awk_string,
    n: awk_number,
    a: awk_array_t,
}

#[repr(C)]
struct awk_value {
    val_type: awk_valtype_t,
    u: awk_value_union,
}

#[repr(C)]
struct awk_ext_func {
    name: *const c_char,
    function: Option<extern "C" fn(c_int, *mut awk_value, *mut awk_ext_func) -> *mut awk_value>,
    max_expected_args: size_t,
    min_required_args: size_t,
    suppress_lint: awk_bool_t,
    data: *mut c_void,
}

#[repr(C)]
struct gawk_api {
    major_version: c_int,
    minor_version: c_int,
    // ... other fields omitted for brevity
    api_add_ext_func: Option<extern "C" fn(awk_ext_id_t, *const c_char, *mut awk_ext_func) -> awk_bool_t>,
    api_get_argument: Option<extern "C" fn(awk_ext_id_t, size_t, awk_valtype_t, *mut awk_value) -> awk_bool_t>,
    api_sym_update: Option<extern "C" fn(awk_ext_id_t, *const c_char, *const c_char, *mut awk_value) -> awk_bool_t>,
    api_create_array: Option<extern "C" fn(awk_ext_id_t) -> awk_array_t>,
    api_clear_array: Option<extern "C" fn(awk_ext_id_t, awk_array_t) -> awk_bool_t>,
    api_set_array_element: Option<extern "C" fn(awk_ext_id_t, awk_array_t, *const awk_value, *const awk_value) -> awk_bool_t>,
    api_malloc: Option<extern "C" fn(size_t) -> *mut c_void>,
    api_free: Option<extern "C" fn(*mut c_void)>,
    api_warning: Option<extern "C" fn(awk_ext_id_t, *const c_char, ...)>,
    api_update_ERRNO_int: Option<extern "C" fn(awk_ext_id_t, c_int)>,
    api_fatal: Option<extern "C" fn(awk_ext_id_t, *const c_char, ...) -> !>,
    // ... other fields omitted for brevity
}

static mut API: *const gawk_api = ptr::null();
static mut EXT_ID: awk_ext_id_t = ptr::null_mut();

// Helper functions
fn make_number(num: c_double, result: &mut awk_value) -> &mut awk_value {
    result.val_type = AWK_NUMBER;
    unsafe {
        result.u.n = awk_number {
            d: num,
            type_: 0, // AWK_NUMBER_TYPE_DOUBLE
            ptr: ptr::null_mut(),
        };
    }
    result
}

fn r_make_string(
    api: *const gawk_api,
    ext_id: awk_ext_id_t,
    string: &str,
    duplicate: bool,
    result: &mut awk_value,
) -> Result<&mut awk_value, String> {
    result.val_type = AWK_STRING;
    let c_str = CString::new(string).map_err(|e| e.to_string())?;
    let len = string.len() as size_t;

    if duplicate {
        let buf = unsafe {
            let malloc = (*api).api_malloc.expect("api_malloc is null");
            malloc(len + 1) as *mut c_char
        };
        if buf.is_null() {
            unsafe {
                let fatal = (*api).api_fatal.expect("api_fatal is null");
                fatal(
                    ext_id,
                    b"r_make_string: malloc failed\0".as_ptr() as *const c_char,
                );
            }
        }
        unsafe {
            ptr::copy_nonoverlapping(c_str.as_ptr(), buf, len);
            *buf.add(len) = 0;
            result.u.s = awk_string {
                str_: buf,
                len,
            };
        }
    } else {
        unsafe {
            result.u.s = awk_string {
                str_: c_str.into_raw(),
                len,
            };
        }
    }
    Ok(result)
}

// Main functions
extern "C" fn do_chdir(
    nargs: c_int,
    result: *mut awk_value,
    _unused: *mut awk_ext_func,
) -> *mut awk_value {
    unsafe {
        let mut newdir = awk_value {
            val_type: AWK_UNDEFINED,
            u: awk_value_union { s: awk_string { str_: ptr::null_mut(), len: 0 } },
        };

        if ((*API).api_get_argument.expect("api_get_argument is null"))(
            EXT_ID,
            0,
            AWK_STRING,
            &mut newdir,
        ) == AWK_TRUE
        {
            let path = CStr::from_ptr(newdir.u.s.str_).to_string_lossy();
            match std::env::set_current_dir(&*path) {
                Ok(_) => make_number(0.0, &mut *result),
                Err(e) => {
                    ((*API).api_update_ERRNO_int.expect("api_update_ERRNO_int is null"))(
                        EXT_ID,
                        e.raw_os_error().unwrap_or(0),
                    );
                    make_number(-1.0, &mut *result)
                }
            }
        } else {
            ((*API).api_warning.expect("api_warning is null"))(
                EXT_ID,
                b"chdir: first argument is not a string\0".as_ptr() as *const c_char,
            );
            make_number(-1.0, &mut *result)
        }
    }
}

// ... other functions would follow similar patterns

#[no_mangle]
pub extern "C" fn dl_load(api_p: *const gawk_api, id: awk_ext_id_t) -> c_int {
    unsafe {
        API = api_p;
        EXT_ID = id;

        if (*API).major_version != GAWK_API_MAJOR_VERSION
            || (*API).minor_version < GAWK_API_MINOR_VERSION
        {
            eprintln!("filefuncs: version mismatch with gawk!");
            eprintln!(
                "\tmy version (API {}.{}), gawk version (API {}.{})",
                GAWK_API_MAJOR_VERSION,
                GAWK_API_MINOR_VERSION,
                (*API).major_version,
                (*API).minor_version
            );
            return 0;
        }

        // Register functions
        let funcs = [
            awk_ext_func {
                name: b"chdir\0".as_ptr() as *const c_char,
                function: Some(do_chdir),
                max_expected_args: 1,
                min_required_args: 1,
                suppress_lint: AWK_FALSE,
                data: ptr::null_mut(),
            },
            // ... other functions
        ];

        let mut errors = 0;
        for func in &funcs {
            if ((*API).api_add_ext_func.expect("api_add_ext_func is null"))(
                EXT_ID,
                b"\0".as_ptr() as *const c_char,
                func as *const _ as *mut awk_ext_func,
            ) == AWK_FALSE
            {
                errors += 1;
            }
        }

        (errors == 0) as c_int
    }
}