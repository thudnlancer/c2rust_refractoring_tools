use std::ffi::{CStr, CString};
use std::mem;
use std::ptr;
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{DateTime, NaiveDateTime, Utc};
use libc::{c_char, c_double, c_int, c_void, size_t, time_t, tm};
use lazy_static::lazy_static;
use std::sync::Mutex;

const AWK_NUMBER: u32 = 1;
const AWK_STRING: u32 = 2;
const AWK_UNDEFINED: u32 = 0;
const GAWK_API_MAJOR_VERSION: u32 = 3;
const GAWK_API_MINOR_VERSION: u32 = 2;

#[repr(C)]
struct AwkValue {
    val_type: u32,
    str_val: *mut c_char,
    str_len: size_t,
    num_val: c_double,
}

#[repr(C)]
struct AwkExtFunc {
    name: *const c_char,
    function: extern "C" fn(c_int, *mut AwkValue, *mut c_void) -> *mut AwkValue,
    max_expected_args: size_t,
    min_required_args: size_t,
    suppress_lint: c_int,
    data: *mut c_void,
}

struct GawkApi {
    major_version: c_int,
    minor_version: c_int,
    api_add_ext_func: extern "C" fn(*mut c_void, *const c_char, *mut AwkExtFunc) -> c_int,
    api_warning: extern "C" fn(*mut c_void, *const c_char, ...),
    api_update_ERRNO_int: extern "C" fn(*mut c_void, c_int),
    api_update_ERRNO_string: extern "C" fn(*mut c_void, *const c_char),
    api_get_argument: extern "C" fn(*mut c_void, size_t, u32, *mut AwkValue) -> c_int,
}

lazy_static! {
    static ref API: Mutex<Option<&'static GawkApi>> = Mutex::new(None);
    static ref EXT_ID: Mutex<Option<*mut c_void>> = Mutex::new(None);
}

const EXT_VERSION: &str = "time extension: version 1.2\0";

extern "C" fn do_gettimeofday(
    _nargs: c_int,
    result: *mut AwkValue,
    _unused: *mut c_void,
) -> *mut AwkValue {
    unsafe {
        let duration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap();
        let secs = duration.as_secs() as c_double;
        let micros = duration.subsec_micros() as c_double;
        (*result).val_type = AWK_NUMBER;
        (*result).num_val = secs + micros / 1_000_000.0;
        result
    }
}

extern "C" fn do_sleep(
    nargs: c_int,
    result: *mut AwkValue,
    _unused: *mut c_void,
) -> *mut AwkValue {
    unsafe {
        if nargs < 1 {
            update_errno_string("sleep: missing required numeric argument");
            (*result).val_type = AWK_NUMBER;
            (*result).num_val = -1.0;
            return result;
        }

        let mut num = AwkValue {
            val_type: AWK_UNDEFINED,
            str_val: ptr::null_mut(),
            str_len: 0,
            num_val: 0.0,
        };

        let api = API.lock().unwrap().unwrap();
        if (api.api_get_argument)(EXT_ID.lock().unwrap().unwrap(), 0, AWK_NUMBER, &mut num) == 0 {
            update_errno_string("sleep: argument is not a number");
            (*result).val_type = AWK_NUMBER;
            (*result).num_val = -1.0;
            return result;
        }

        let secs = num.num_val;
        if secs < 0.0 {
            update_errno_string("sleep: argument is negative");
            (*result).val_type = AWK_NUMBER;
            (*result).num_val = -1.0;
            return result;
        }

        let sleep_time = std::time::Duration::from_secs_f64(secs);
        std::thread::sleep(sleep_time);

        (*result).val_type = AWK_NUMBER;
        (*result).num_val = 0.0;
        result
    }
}

extern "C" fn do_strptime(
    nargs: c_int,
    result: *mut AwkValue,
    _unused: *mut c_void,
) -> *mut AwkValue {
    unsafe {
        if nargs < 2 {
            (*result).val_type = AWK_NUMBER;
            (*result).num_val = -1.0;
            return result;
        }

        let mut string = AwkValue {
            val_type: AWK_UNDEFINED,
            str_val: ptr::null_mut(),
            str_len: 0,
            num_val: 0.0,
        };

        let mut format = AwkValue {
            val_type: AWK_UNDEFINED,
            str_val: ptr::null_mut(),
            str_len: 0,
            num_val: 0.0,
        };

        let api = API.lock().unwrap().unwrap();
        if (api.api_get_argument)(EXT_ID.lock().unwrap().unwrap(), 0, AWK_STRING, &mut string) == 0
            || (api.api_get_argument)(EXT_ID.lock().unwrap().unwrap(), 1, AWK_STRING, &mut format) == 0
        {
            (*result).val_type = AWK_NUMBER;
            (*result).num_val = -1.0;
            return result;
        }

        let c_str = CStr::from_ptr(string.str_val);
        let format_str = CStr::from_ptr(format.str_val);

        if let (Ok(date_str), Ok(fmt_str)) = (c_str.to_str(), format_str.to_str()) {
            if let Ok(parsed) = NaiveDateTime::parse_from_str(date_str, fmt_str) {
                let datetime: DateTime<Utc> = DateTime::from_utc(parsed, Utc);
                let timestamp = datetime.timestamp() as c_double;
                (*result).val_type = AWK_NUMBER;
                (*result).num_val = timestamp;
                return result;
            }
        }

        (*result).val_type = AWK_NUMBER;
        (*result).num_val = -1.0;
        result
    }
}

fn update_errno_string(msg: &str) {
    unsafe {
        let c_msg = CString::new(msg).unwrap();
        if let Some(api) = API.lock().unwrap().as_ref() {
            (api.api_update_ERRNO_string)(EXT_ID.lock().unwrap().unwrap(), c_msg.as_ptr());
        }
    }
}

#[no_mangle]
pub extern "C" fn dl_load(api_p: *mut GawkApi, id: *mut c_void) -> c_int {
    unsafe {
        let api = &*api_p;
        if api.major_version != GAWK_API_MAJOR_VERSION as c_int
            || api.minor_version < GAWK_API_MINOR_VERSION as c_int
        {
            eprintln!("time: version mismatch with gawk!");
            eprintln!(
                "\tmy version (API {}.{}), gawk version (API {}.{})",
                GAWK_API_MAJOR_VERSION,
                GAWK_API_MINOR_VERSION,
                api.major_version,
                api.minor_version
            );
            return 0;
        }

        *API.lock().unwrap() = Some(api);
        *EXT_ID.lock().unwrap() = Some(id);

        let funcs = [
            AwkExtFunc {
                name: b"gettimeofday\0".as_ptr() as *const c_char,
                function: do_gettimeofday,
                max_expected_args: 0,
                min_required_args: 0,
                suppress_lint: 0,
                data: ptr::null_mut(),
            },
            AwkExtFunc {
                name: b"sleep\0".as_ptr() as *const c_char,
                function: do_sleep,
                max_expected_args: 1,
                min_required_args: 1,
                suppress_lint: 0,
                data: ptr::null_mut(),
            },
            AwkExtFunc {
                name: b"strptime\0".as_ptr() as *const c_char,
                function: do_strptime,
                max_expected_args: 2,
                min_required_args: 2,
                suppress_lint: 0,
                data: ptr::null_mut(),
            },
        ];

        for func in &funcs {
            if (api.api_add_ext_func)(id, b"\0".as_ptr() as *const c_char, func as *const _ as *mut _) == 0 {
                let msg = CString::new(format!("time: could not add {}", 
                    CStr::from_ptr(func.name).to_string_lossy())).unwrap();
                (api.api_warning)(id, msg.as_ptr());
                return 0;
            }
        }

        let ext_version_cstr = CString::new(EXT_VERSION).unwrap();
        (api.api_register_ext_version)(id, ext_version_cstr.as_ptr());

        1
    }
}

#[no_mangle]
pub static plugin_is_GPL_compatible: c_int = 1;