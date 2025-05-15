/*
 * fnmatch.rs - Provide an interface to fnmatch routine
 *
 * Translated from C to Rust
 */

use std::ffi::{CString, CStr};
use std::os::raw::c_int;
use libc::{FNM_NOMATCH, FNM_CASEFOLD, FNM_FILE_NAME, FNM_LEADING_DIR, 
          FNM_NOESCAPE, FNM_PATHNAME, FNM_PERIOD};
use gawkapi::{GawkApi, AwkValue, AwkValueType, AwkExtId, AwkExtFunc, AwkBool};

static mut API: Option<&'static GawkApi> = None;
static mut EXT_ID: Option<AwkExtId> = None;
const EXT_VERSION: &str = "fnmatch extension: version 1.0";

struct FnmFlags {
    name: &'static str,
    value: c_int,
}

const FLAG_TABLE: &[FnmFlags] = &[
    FnmFlags { name: "CASEFOLD", value: FNM_CASEFOLD },
    FnmFlags { name: "FILE_NAME", value: FNM_FILE_NAME },
    FnmFlags { name: "LEADING_DIR", value: FNM_LEADING_DIR },
    FnmFlags { name: "NOESCAPE", value: FNM_NOESCAPE },
    FnmFlags { name: "PATHNAME", value: FNM_PATHNAME },
    FnmFlags { name: "PERIOD", value: FNM_PERIOD },
    FnmFlags { name: "", value: 0 },
];

fn do_fnmatch(nargs: c_int, result: &mut AwkValue, _unused: *mut AwkExtFunc) -> *mut AwkValue {
    let flags_mask = FNM_CASEFOLD | FNM_FILE_NAME | FNM_LEADING_DIR | 
                    FNM_NOESCAPE | FNM_PATHNAME | FNM_PERIOD;

    unsafe {
        if API.is_none() {
            result.set_number(-1.0);
            return result;
        }
        let api = API.unwrap();

        let mut pattern = AwkValue::new();
        let mut string = AwkValue::new();
        let mut flags = AwkValue::new();

        result.set_number(-1.0); // default return

        if api.get_argument(0, AwkValueType::String, &mut pattern) == 0 {
            api.warning(EXT_ID.unwrap(), "fnmatch: could not get first argument");
            return result;
        }

        if api.get_argument(1, AwkValueType::String, &mut string) == 0 {
            api.warning(EXT_ID.unwrap(), "fnmatch: could not get second argument");
            return result;
        }

        if api.get_argument(2, AwkValueType::Number, &mut flags) == 0 {
            api.warning(EXT_ID.unwrap(), "fnmatch: could not get third argument");
            return result;
        }

        let int_flags = flags.num_value() as c_int & flags_mask;
        let pattern_cstr = CString::new(pattern.str_value()).unwrap();
        let string_cstr = CString::new(string.str_value()).unwrap();

        let retval = libc::fnmatch(
            pattern_cstr.as_ptr(),
            string_cstr.as_ptr(),
            int_flags
        );

        result.set_number(retval as f64);
    }

    result
}

fn init_fnmatch() -> AwkBool {
    let mut errors = 0;
    unsafe {
        if API.is_none() || EXT_ID.is_none() {
            return 0;
        }
        let api = API.unwrap();

        let mut value = AwkValue::new();
        value.set_number(FNM_NOMATCH as f64);
        if api.sym_update("FNM_NOMATCH", &value) == 0 {
            api.warning(EXT_ID.unwrap(), "fnmatch init: could not add FNM_NOMATCH variable");
            errors += 1;
        }

        let new_array = api.create_array();
        for flag in FLAG_TABLE.iter().take_while(|f| !f.name.is_empty()) {
            let mut index = AwkValue::new();
            let mut value = AwkValue::new();
            
            index.set_const_string(flag.name);
            value.set_number(flag.value as f64);
            
            if api.set_array_element(new_array, &index, &value) == 0 {
                api.warning(EXT_ID.unwrap(), 
                           &format!("fnmatch init: could not set array element {}", flag.name));
                errors += 1;
            }
        }

        let mut the_array = AwkValue::new();
        the_array.set_array(new_array);

        if api.sym_update("FNM", &the_array) == 0 {
            api.warning(EXT_ID.unwrap(), "fnmatch init: could not install FNM array");
            errors += 1;
        }
    }

    (errors == 0) as AwkBool
}

#[no_mangle]
pub extern "C" fn dl_load(api: &'static GawkApi, ext_id: AwkExtId) -> c_int {
    unsafe {
        API = Some(api);
        EXT_ID = Some(ext_id);
    }

    let func_table = &[
        AwkExtFunc {
            name: "fnmatch\0".as_ptr() as *const i8,
            func: Some(do_fnmatch),
            num_expected_args: 3,
            min_required_args: 3,
            suppress_lint: 0,
            data: std::ptr::null_mut(),
        }
    ];

    unsafe {
        if API.unwrap().register_extension(ext_id, EXT_VERSION.as_ptr() as *const i8, func_table.as_ptr()) == 0 {
            return 0;
        }
    }

    if init_fnmatch() == 0 {
        return 0;
    }

    1
}

#[no_mangle]
pub extern "C" fn plugin_is_GPL_compatible() -> c_int {
    1
}