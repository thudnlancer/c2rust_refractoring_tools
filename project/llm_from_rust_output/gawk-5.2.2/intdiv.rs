use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_double, c_int, c_void};
use std::ptr;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct awk_string {
    pub str_: *mut c_char,
    pub len: usize,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AWK_NUMBER_TYPE {
    DOUBLE = 0,
    MPFR = 1,
    MPZ = 2,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct awk_number {
    pub d: c_double,
    pub type_: AWK_NUMBER_TYPE,
    pub ptr: *mut c_void,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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
    pub s: awk_string,
    pub n: awk_number,
    pub a: *mut c_void,
    pub scl: *mut c_void,
    pub vc: *mut c_void,
    pub b: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct awk_value {
    pub val_type: awk_valtype_t,
    pub u: awk_value_u,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct awk_ext_func {
    pub name: *const c_char,
    pub function: Option<
        extern "C" fn(
            c_int,
            *mut awk_value,
            *mut awk_ext_func,
        ) -> *mut awk_value,
    >,
    pub max_expected_args: usize,
    pub min_required_args: usize,
    pub suppress_lint: u32,
    pub data: *mut c_void,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gawk_api {
    pub major_version: c_int,
    pub minor_version: c_int,
    // ... other fields omitted for brevity
    pub api_add_ext_func: Option<
        extern "C" fn(
            *mut c_void,
            *const c_char,
            *mut awk_ext_func,
        ) -> u32,
    >,
    pub api_warning: Option<
        extern "C" fn(*mut c_void, *const c_char, ...) -> (),
    >,
    pub api_get_argument: Option<
        extern "C" fn(
            *mut c_void,
            usize,
            awk_valtype_t,
            *mut awk_value,
        ) -> u32,
    >,
    pub api_set_array_element: Option<
        extern "C" fn(
            *mut c_void,
            *mut c_void,
            *const awk_value,
            *const awk_value,
        ) -> u32,
    >,
    pub api_clear_array: Option<
        extern "C" fn(*mut c_void, *mut c_void) -> u32,
    >,
    pub api_malloc: Option<extern "C" fn(usize) -> *mut c_void>,
    pub api_fatal: Option<extern "C" fn(*mut c_void, *const c_char, ...) -> ()>,
    pub api_register_ext_version: Option<
        extern "C" fn(*mut c_void, *const c_char) -> (),
    >,
}

static mut API: *const gawk_api = ptr::null();
static mut EXT_ID: *mut c_void = ptr::null_mut();
static EXT_VERSION: &'static [u8] = b"intdiv extension: version 1.0\0";

fn double_to_int(d: c_double) -> c_double {
    if d >= 0.0 {
        d.floor()
    } else {
        d.ceil()
    }
}

fn array_set_number(
    array: *mut c_void,
    sub: &[u8],
    num: c_double,
) -> Result<(), String> {
    let mut index = awk_value {
        val_type: awk_valtype_t::UNDEFINED,
        u: awk_value_u { s: awk_string { str_: ptr::null_mut(), len: 0 } },
    };
    let mut tmp = awk_value {
        val_type: awk_valtype_t::UNDEFINED,
        u: awk_value_u { s: awk_string { str_: ptr::null_mut(), len: 0 } },
    };

    let sub_cstr = CString::new(sub).map_err(|e| e.to_string())?;
    
    unsafe {
        let index_val = make_string(
            sub_cstr.as_ptr(),
            sub.len(),
            true,
            &mut index,
            awk_valtype_t::STRING,
        )?;
        
        let num_val = make_number(num, &mut tmp);
        
        if let Some(func) = (*API).api_set_array_element {
            if func(EXT_ID, array, index_val, num_val) == 0 {
                return Err("Failed to set array element".to_string());
            }
        } else {
            return Err("api_set_array_element is null".to_string());
        }
    }
    
    Ok(())
}

fn make_string(
    string: *const c_char,
    length: usize,
    duplicate: bool,
    result: *mut awk_value,
    val_type: awk_valtype_t,
) -> Result<*const awk_value, String> {
    unsafe {
        (*result).val_type = val_type;
        (*result).u.s.len = length;
        
        if duplicate {
            let cp = ((*API).api_malloc.ok_or("api_malloc is null")?)(length + 1) as *mut c_char;
            if cp.is_null() {
                return Err("malloc failed".to_string());
            }
            
            ptr::copy_nonoverlapping(string, cp, length);
            *cp.add(length) = 0;
            (*result).u.s.str_ = cp;
        } else {
            (*result).u.s.str_ = string as *mut c_char;
        }
    }
    
    Ok(result)
}

fn make_number(num: c_double, result: *mut awk_value) -> *const awk_value {
    unsafe {
        (*result).val_type = awk_valtype_t::NUMBER;
        (*result).u.n.d = num;
        (*result).u.n.type_ = AWK_NUMBER_TYPE::DOUBLE;
    }
    result
}

extern "C" fn do_intdiv(
    nargs: c_int,
    result: *mut awk_value,
    _unused: *mut awk_ext_func,
) -> *mut awk_value {
    let mut nv = awk_value {
        val_type: awk_valtype_t::UNDEFINED,
        u: awk_value_u { s: awk_string { str_: ptr::null_mut(), len: 0 } },
    };
    let mut dv = awk_value {
        val_type: awk_valtype_t::UNDEFINED,
        u: awk_value_u { s: awk_string { str_: ptr::null_mut(), len: 0 } },
    };
    let mut array_param = awk_value {
        val_type: awk_valtype_t::UNDEFINED,
        u: awk_value_u { s: awk_string { str_: ptr::null_mut(), len: 0 } },
    };

    unsafe {
        if let Some(func) = (*API).api_get_argument {
            if func(EXT_ID, 0, awk_valtype_t::NUMBER, &mut nv) == 0 {
                warn("intdiv: first argument must be numeric");
                return make_number(-1.0, result);
            }
            
            if func(EXT_ID, 1, awk_valtype_t::NUMBER, &mut dv) == 0 {
                warn("intdiv: second argument must be numeric");
                return make_number(-1.0, result);
            }
            
            if func(EXT_ID, 2, awk_valtype_t::ARRAY, &mut array_param) == 0 {
                warn("intdiv: third argument must be an array");
                return make_number(-1.0, result);
            }
        } else {
            return make_number(-1.0, result);
        }

        let array = array_param.u.a;
        if let Some(func) = (*API).api_clear_array {
            func(EXT_ID, array);
        }

        let num = double_to_int(nv.u.n.d);
        let denom = double_to_int(dv.u.n.d);
        
        if denom == 0.0 {
            warn("intdiv: division by zero attempted");
            return make_number(-1.0, result);
        }

        let quotient = double_to_int(num / denom);
        let remainder = num.rem_euclid(denom);
        let remainder = double_to_int(remainder);

        if let Err(e) = array_set_number(array, b"quotient", quotient) {
            warn(&format!("intdiv: {}", e));
            return make_number(-1.0, result);
        }
        
        if let Err(e) = array_set_number(array, b"remainder", remainder) {
            warn(&format!("intdiv: {}", e));
            return make_number(-1.0, result);
        }
    }

    make_number(0.0, result)
}

fn warn(msg: &str) {
    unsafe {
        if let Some(func) = (*API).api_warning {
            let msg_cstr = CString::new(msg).unwrap();
            func(EXT_ID, msg_cstr.as_ptr());
        }
    }
}

static FUNC_TABLE: [awk_ext_func; 1] = [
    awk_ext_func {
        name: b"intdiv\0".as_ptr() as *const c_char,
        function: Some(do_intdiv),
        max_expected_args: 3,
        min_required_args: 3,
        suppress_lint: 0,
        data: ptr::null_mut(),
    },
];

#[no_mangle]
pub extern "C" fn dl_load(api_p: *const gawk_api, id: *mut c_void) -> c_int {
    unsafe {
        API = api_p;
        EXT_ID = id;
        
        if (*API).major_version != 3 || (*API).minor_version < 2 {
            eprintln!("intdiv: version mismatch with gawk!");
            eprintln!("\tmy version (API 3.2), gawk version (API {}.{})", 
                     (*API).major_version, (*API).minor_version);
            return 0;
        }
        
        for func in &FUNC_TABLE {
            if let Some(add_func) = (*API).api_add_ext_func {
                if add_func(EXT_ID, b"\0".as_ptr() as *const c_char, func as *const _ as *mut _) == 0 {
                    warn("intdiv: could not add function");
                    return 0;
                }
            }
        }
        
        if let Some(register_func) = (*API).api_register_ext_version {
            register_func(EXT_ID, EXT_VERSION.as_ptr() as *const c_char);
        }
    }
    
    1
}