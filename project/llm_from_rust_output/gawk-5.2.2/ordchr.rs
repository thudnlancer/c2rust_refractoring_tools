use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_double, c_int, c_uint, c_void};
use std::ptr;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct awk_string_t {
    pub str_: *mut c_char,
    pub len: usize,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct awk_number_t {
    pub d: c_double,
    pub type_: AWK_NUMBER_TYPE,
    pub ptr: *mut c_void,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum AWK_NUMBER_TYPE {
    DOUBLE = 0,
    MPFR = 1,
    MPZ = 2,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone)]
pub enum awk_valtype_t {
    UNDEFINED = 0,
    NUMBER = 1,
    STRING = 2,
    REGEX = 3,
    STRNUM = 4,
    ARRAY = 5,
    SCALAR = 6,
    VALUE_COOKIE = 7,
    BOOL = 8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub union awk_value_u {
    pub s: awk_string_t,
    pub n: awk_number_t,
    pub a: *mut c_void,
    pub scl: *mut c_void,
    pub vc: *mut c_void,
    pub b: c_uint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct awk_value_t {
    pub val_type: awk_valtype_t,
    pub u: awk_value_u,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct awk_ext_func_t {
    pub name: *const c_char,
    pub function: Option<
        extern "C" fn(
            c_int,
            *mut awk_value_t,
            *mut awk_ext_func_t,
        ) -> *mut awk_value_t,
    >,
    pub max_expected_args: usize,
    pub min_required_args: usize,
    pub suppress_lint: c_uint,
    pub data: *mut c_void,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gawk_api_t {
    pub major_version: c_int,
    pub minor_version: c_int,
    pub gmp_major_version: c_int,
    pub gmp_minor_version: c_int,
    pub mpfr_major_version: c_int,
    pub mpfr_minor_version: c_int,
    pub do_flags: [c_int; 6],
    pub api_add_ext_func: Option<
        extern "C" fn(
            *mut c_void,
            *const c_char,
            *mut awk_ext_func_t,
        ) -> c_uint,
    >,
    // ... other fields omitted for brevity
}

static mut API: *const gawk_api_t = ptr::null();
static mut EXT_ID: *mut c_void = ptr::null_mut();
static EXT_VERSION: &'static CStr = unsafe {
    CStr::from_bytes_with_nul_unchecked(b"ordchr extension: version 1.0\0")
};

pub static mut PLUGIN_IS_GPL_COMPATIBLE: c_int = 0;

extern "C" fn do_ord(
    nargs: c_int,
    result: *mut awk_value_t,
    _unused: *mut awk_ext_func_t,
) -> *mut awk_value_t {
    unsafe {
        let mut str = awk_value_t {
            val_type: awk_valtype_t::UNDEFINED,
            u: awk_value_u { s: awk_string_t { str_: ptr::null_mut(), len: 0 } },
        };

        let mut ret = -1.0;
        if let Some(api) = API.as_ref() {
            if let Some(get_arg) = api.api_get_argument {
                if get_arg(EXT_ID, 0, awk_valtype_t::STRING, &mut str) != 0 {
                    if !str.u.s.str_.is_null() {
                        ret = *str.u.s.str_ as u8 as c_double;
                    }
                } else if api.do_flags[0] != 0 {
                    if let Some(lintwarn) = api.api_lintwarn {
                        let msg = CStr::from_bytes_with_nul_unchecked(
                            b"ord: first argument is not a string\0"
                        );
                        lintwarn(EXT_ID, msg.as_ptr());
                    }
                }
            }
        }

        make_number(ret, result)
    }
}

extern "C" fn do_chr(
    nargs: c_int,
    result: *mut awk_value_t,
    _unused: *mut awk_ext_func_t,
) -> *mut awk_value_t {
    unsafe {
        let mut num = awk_value_t {
            val_type: awk_valtype_t::UNDEFINED,
            u: awk_value_u { s: awk_string_t { str_: ptr::null_mut(), len: 0 } },
        };

        let mut ret = 0u32;
        let mut val = 0.0;
        let mut str = [0i8; 2];
        str[1] = 0;

        if let Some(api) = API.as_ref() {
            if let Some(get_arg) = api.api_get_argument {
                if get_arg(EXT_ID, 0, awk_valtype_t::NUMBER, &mut num) != 0 {
                    val = num.u.n.d;
                    ret = val as u32;
                    ret &= 0xff;
                    str[0] = ret as i8;
                    str[1] = 0;
                } else if api.do_flags[0] != 0 {
                    if let Some(lintwarn) = api.api_lintwarn {
                        let msg = CStr::from_bytes_with_nul_unchecked(
                            b"chr: first argument is not a number\0"
                        );
                        lintwarn(EXT_ID, msg.as_ptr());
                    }
                }
            }

            r_make_string(
                API,
                EXT_ID,
                str.as_ptr(),
                1,
                1,
                result,
            )
        } else {
            result
        }
    }
}

fn make_number(num: c_double, result: *mut awk_value_t) -> *mut awk_value_t {
    unsafe {
        (*result).val_type = awk_valtype_t::NUMBER;
        (*result).u.n = awk_number_t {
            d: num,
            type_: AWK_NUMBER_TYPE::DOUBLE,
            ptr: ptr::null_mut(),
        };
        result
    }
}

fn r_make_string(
    api: *const gawk_api_t,
    ext_id: *mut c_void,
    string: *const c_char,
    length: usize,
    duplicate: c_uint,
    result: *mut awk_value_t,
) -> *mut awk_value_t {
    unsafe {
        ptr::write_bytes(result, 0, 1);
        (*result).val_type = awk_valtype_t::STRING;
        (*result).u.s.len = length;

        if duplicate != 0 {
            if let Some(api) = api.as_ref() {
                if let Some(alloc) = api.api_malloc {
                    let cp = alloc(length + 1) as *mut c_char;
                    if cp.is_null() {
                        if let Some(fatal) = api.api_fatal {
                            let msg = CStr::from_bytes_with_nul_unchecked(
                                b"%s: malloc of %d bytes failed\0"
                            );
                            let func = CStr::from_bytes_with_nul_unchecked(
                                b"r_make_string\0"
                            );
                            fatal(ext_id, msg.as_ptr(), func.as_ptr(), length + 1);
                        }
                    } else {
                        ptr::copy_nonoverlapping(string, cp, length);
                        *cp.add(length) = 0;
                        (*result).u.s.str_ = cp;
                    }
                }
            }
        } else {
            (*result).u.s.str_ = string as *mut c_char;
        }

        result
    }
}

static FUNC_TABLE: [awk_ext_func_t; 2] = [
    awk_ext_func_t {
        name: unsafe { CStr::from_bytes_with_nul_unchecked(b"ord\0").as_ptr() },
        function: Some(do_ord),
        max_expected_args: 1,
        min_required_args: 1,
        suppress_lint: 0,
        data: ptr::null_mut(),
    },
    awk_ext_func_t {
        name: unsafe { CStr::from_bytes_with_nul_unchecked(b"chr\0").as_ptr() },
        function: Some(do_chr),
        max_expected_args: 1,
        min_required_args: 1,
        suppress_lint: 0,
        data: ptr::null_mut(),
    },
];

#[no_mangle]
pub unsafe extern "C" fn dl_load(api_p: *const gawk_api_t, id: *mut c_void) -> c_int {
    API = api_p;
    EXT_ID = id;

    if let Some(api) = API.as_ref() {
        if api.major_version != 3 || api.minor_version < 2 {
            eprintln!("ord_chr: version mismatch with gawk!");
            eprintln!("\tmy version (API 3.2), gawk version (API {}.{})",
                api.major_version, api.minor_version);
            return 0;
        }

        let mut errors = 0;

        for func in &FUNC_TABLE {
            if let Some(add_func) = api.api_add_ext_func {
                if add_func(EXT_ID, CStr::from_bytes_with_nul_unchecked(b"\0").as_ptr(), 
                    func as *const _ as *mut _) == 0 
                {
                    if let Some(warn) = api.api_warning {
                        let msg = CStr::from_bytes_with_nul_unchecked(
                            b"ord_chr: could not add %s\0"
                        );
                        warn(EXT_ID, msg.as_ptr(), func.name);
                    }
                    errors += 1;
                }
            }
        }

        if let Some(register_version) = api.api_register_ext_version {
            register_version(EXT_ID, EXT_VERSION.as_ptr());
        }

        (errors == 0) as c_int
    } else {
        0
    }
}