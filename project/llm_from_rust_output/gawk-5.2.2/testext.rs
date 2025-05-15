use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_double, c_int, c_uint, c_ulong, c_void};
use std::ptr;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct awk_string_t {
    pub str_: *mut c_char,
    pub len: size_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct awk_number_t {
    pub d: c_double,
    pub type_: AWK_NUMBER_TYPE,
    pub ptr: *mut c_void,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct awk_value_t {
    pub val_type: awk_valtype_t,
    pub u: awk_value_union,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub union awk_value_union {
    pub s: awk_string_t,
    pub n: awk_number_t,
    pub a: awk_array_t,
    pub scl: awk_scalar_t,
    pub vc: awk_value_cookie_t,
    pub b: awk_bool_t,
}

pub type awk_array_t = *mut c_void;
pub type awk_scalar_t = *mut c_void;
pub type awk_value_cookie_t = *mut c_void;

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum awk_valtype_t {
    AWK_UNDEFINED = 0,
    AWK_NUMBER = 1,
    AWK_STRING = 2,
    AWK_REGEX = 3,
    AWK_STRNUM = 4,
    AWK_ARRAY = 5,
    AWK_SCALAR = 6,
    AWK_VALUE_COOKIE = 7,
    AWK_BOOL = 8,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AWK_NUMBER_TYPE {
    AWK_NUMBER_TYPE_DOUBLE = 0,
    AWK_NUMBER_TYPE_MPFR = 1,
    AWK_NUMBER_TYPE_MPZ = 2,
}

pub type awk_bool_t = c_uint;
pub const awk_true: awk_bool_t = 1;
pub const awk_false: awk_bool_t = 0;

pub type size_t = c_ulong;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct awk_ext_func_t {
    pub name: *const c_char,
    pub function: Option<
        unsafe extern "C" fn(
            c_int,
            *mut awk_value_t,
            *mut awk_ext_func_t,
        ) -> *mut awk_value_t,
    >,
    pub max_expected_args: size_t,
    pub min_required_args: size_t,
    pub suppress_lint: awk_bool_t,
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
        unsafe extern "C" fn(
            awk_ext_id_t,
            *const c_char,
            *mut awk_ext_func_t,
        ) -> awk_bool_t,
    >,
    // ... other API functions omitted for brevity
}

pub type awk_ext_id_t = *mut c_void;

static mut API: *const gawk_api_t = ptr::null();
static mut EXT_ID: awk_ext_id_t = ptr::null_mut();

#[no_mangle]
pub unsafe extern "C" fn dl_load(api_p: *const gawk_api_t, id: awk_ext_id_t) -> c_int {
    API = api_p;
    EXT_ID = id;
    
    // Initialize extension
    if init_testext() == awk_false {
        eprintln!("testext: initialization failed");
        return 0;
    }
    
    1 // success
}

unsafe fn init_testext() -> awk_bool_t {
    let api = API;
    let ext_id = EXT_ID;
    
    // Register extension functions
    let funcs = [
        awk_ext_func_t {
            name: b"test_function\0".as_ptr() as *const c_char,
            function: Some(test_function),
            max_expected_args: 0,
            min_required_args: 0,
            suppress_lint: awk_false,
            data: ptr::null_mut(),
        },
        // ... other functions
    ];
    
    for func in &funcs {
        if let Some(add_func) = (*api).api_add_ext_func {
            if add_func(ext_id, b"\0".as_ptr() as *const c_char, func as *const _ as *mut _) == awk_false {
                eprintln!("Failed to add function: {:?}", CStr::from_ptr(func.name));
                return awk_false;
            }
        }
    }
    
    awk_true
}

unsafe extern "C" fn test_function(
    _nargs: c_int,
    result: *mut awk_value_t,
    _unused: *mut awk_ext_func_t,
) -> *mut awk_value_t {
    println!("test::test_function() called");
    make_number(0.0, result)
}

unsafe fn make_number(num: c_double, result: *mut awk_value_t) -> *mut awk_value_t {
    (*result).val_type = awk_valtype_t::AWK_NUMBER;
    (*result).u.n = awk_number_t {
        d: num,
        type_: AWK_NUMBER_TYPE::AWK_NUMBER_TYPE_DOUBLE,
        ptr: ptr::null_mut(),
    };
    result
}

// ... other helper functions