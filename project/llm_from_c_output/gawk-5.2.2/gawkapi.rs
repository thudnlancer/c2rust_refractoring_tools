/*!
 * gawkapi.rs -- Definitions for use by extension functions calling into gawk.
 *
 * Translated from C to Rust with safety and idiomatic Rust practices in mind.
 */

use std::os::raw::{c_char, c_int, c_void};
use std::ffi::{CStr, CString};
use std::mem;
use std::ptr;
use std::fmt;
use std::error::Error;

// Allow use in C++ code
#[cfg(feature = "cxx")]
extern "C" {
    fn __cxa_atexit(
        func: extern "C" fn(*mut c_void),
        arg: *mut c_void,
        dso_handle: *mut c_void,
    ) -> c_int;
}

/// Allow extensions from modifying certain fields in some structs
#[cfg(not(feature = "gawk"))]
type awk_const = *const c_void;
#[cfg(feature = "gawk")]
type awk_const = *mut c_void;

/// Boolean type for the API
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub enum awk_bool_t {
    awk_false = 0,
    awk_true = 1,
}

impl From<bool> for awk_bool_t {
    fn from(b: bool) -> Self {
        if b { awk_bool_t::awk_true } else { awk_bool_t::awk_false }
    }
}

impl Into<bool> for awk_bool_t {
    fn into(self) -> bool {
        self == awk_bool_t::awk_true
    }
}

/// Field width information structure
#[repr(C)]
pub struct awk_field_info {
    pub skip: usize,
    pub len: usize,
}

/// Field width information container
#[repr(C)]
pub struct awk_fieldwidth_info_t {
    pub use_chars: awk_bool_t,
    pub nf: usize,
    pub fields: [awk_field_info; 1], // Actual dimension should be nf
}

/// Calculate total struct size needed for field width info
#[inline]
pub fn awk_fieldwidth_info_size(nf: usize) -> usize {
    mem::size_of::<awk_fieldwidth_info_t>() + ((nf - 1) * mem::size_of::<awk_field_info>())
}

/// Input file information
#[repr(C)]
pub struct awk_input {
    pub name: *const c_char,
    pub fd: c_int,
    pub opaque: *mut c_void,
    pub get_record: Option<
        extern "C" fn(
            *mut *mut c_char,
            *mut awk_input,
            *mut c_int,
            *mut *mut c_char,
            *mut usize,
            *mut *const awk_fieldwidth_info_t,
        ) -> c_int,
    >,
    pub read_func: Option<extern "C" fn(c_int, *mut c_void, usize) -> isize>,
    pub close_func: Option<extern "C" fn(*mut awk_input)>,
    // stat buf would go here but omitted for simplicity
}

pub const INVALID_HANDLE: c_int = -1;

/// Input parser structure
#[repr(C)]
pub struct awk_input_parser_t {
    pub name: *const c_char,
    pub can_take_file: Option<extern "C" fn(*const awk_input) -> awk_bool_t>,
    pub take_control_of: Option<extern "C" fn(*mut awk_input) -> awk_bool_t>,
    pub next: awk_const,
}

/// Output buffer structure
#[repr(C)]
pub struct awk_output_buf_t {
    pub name: *const c_char,
    pub mode: *const c_char,
    pub fp: *mut libc::FILE,
    pub redirected: awk_bool_t,
    pub opaque: *mut c_void,
    pub gawk_fwrite: Option<
        extern "C" fn(*const c_void, usize, usize, *mut libc::FILE, *mut c_void) -> usize,
    >,
    pub gawk_fflush: Option<extern "C" fn(*mut libc::FILE, *mut c_void) -> c_int>,
    pub gawk_ferror: Option<extern "C" fn(*mut libc::FILE, *mut c_void) -> c_int>,
    pub gawk_fclose: Option<extern "C" fn(*mut libc::FILE, *mut c_void) -> c_int>,
}

/// Output wrapper structure
#[repr(C)]
pub struct awk_output_wrapper_t {
    pub name: *const c_char,
    pub can_take_file: Option<extern "C" fn(*const awk_output_buf_t) -> awk_bool_t>,
    pub take_control_of: Option<extern "C" fn(*mut awk_output_buf_t) -> awk_bool_t>,
    pub next: awk_const,
}

/// Two-way processor structure
#[repr(C)]
pub struct awk_two_way_processor_t {
    pub name: *const c_char,
    pub can_take_two_way: Option<extern "C" fn(*const c_char) -> awk_bool_t>,
    pub take_control_of: Option<
        extern "C" fn(*const c_char, *mut awk_input, *mut awk_output_buf_t) -> awk_bool_t,
    >,
    pub next: awk_const,
}

/// API version constants
pub const GAWK_API_MAJOR_VERSION: c_int = 3;
pub const GAWK_API_MINOR_VERSION: c_int = 2;

/// Number type enumeration
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub enum AWK_NUMBER_TYPE {
    AWK_NUMBER_TYPE_DOUBLE,
    AWK_NUMBER_TYPE_MPFR,
    AWK_NUMBER_TYPE_MPZ,
}

/// Number value structure
#[repr(C)]
pub struct awk_number_t {
    pub d: f64,
    pub type_: AWK_NUMBER_TYPE,
    pub ptr: *mut c_void,
}

/// String value structure
#[repr(C)]
pub struct awk_string_t {
    pub str: *mut c_char,
    pub len: usize,
}

/// Value type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub enum awk_valtype_t {
    AWK_UNDEFINED,
    AWK_NUMBER,
    AWK_STRING,
    AWK_REGEX,
    AWK_STRNUM,
    AWK_ARRAY,
    AWK_SCALAR,
    AWK_VALUE_COOKIE,
    AWK_BOOL,
}

/// Main value union
#[repr(C)]
pub union awk_value_u {
    pub s: awk_string_t,
    pub n: awk_number_t,
    pub a: *mut c_void,
    pub scl: *mut c_void,
    pub vc: *mut c_void,
    pub b: awk_bool_t,
}

/// Main value structure
#[repr(C)]
pub struct awk_value_t {
    pub val_type: awk_valtype_t,
    pub u: awk_value_u,
}

// Convenience macros for accessing union fields
macro_rules! str_value {
    ($val:expr) => {
        unsafe { $val.u.s }
    };
}
macro_rules! strnum_value {
    ($val:expr) => {
        str_value!($val)
    };
}
macro_rules! regex_value {
    ($val:expr) => {
        str_value!($val)
    };
}
macro_rules! num_value {
    ($val:expr) => {
        unsafe { $val.u.n.d }
    };
}
macro_rules! num_type {
    ($val:expr) => {
        unsafe { $val.u.n.type_ }
    };
}
macro_rules! num_ptr {
    ($val:expr) => {
        unsafe { $val.u.n.ptr }
    };
}
macro_rules! array_cookie {
    ($val:expr) => {
        unsafe { $val.u.a }
    };
}
macro_rules! scalar_cookie {
    ($val:expr) => {
        unsafe { $val.u.scl }
    };
}
macro_rules! value_cookie {
    ($val:expr) => {
        unsafe { $val.u.vc }
    };
}
macro_rules! bool_value {
    ($val:expr) => {
        unsafe { $val.u.b }
    };
}

/// Array element flags
#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub enum AWK_ELEMENT_FLAGS {
    AWK_ELEMENT_DEFAULT = 0,
    AWK_ELEMENT_DELETE = 1,
}

/// Array element structure
#[repr(C)]
pub struct awk_element_t {
    pub next: *mut awk_element_t,
    pub flags: AWK_ELEMENT_FLAGS,
    pub index: awk_value_t,
    pub value: awk_value_t,
}

/// Flattened array structure
#[repr(C)]
pub struct awk_flat_array_t {
    pub opaque1: awk_const,
    pub opaque2: awk_const,
    pub count: usize,
    pub elements: [awk_element_t; 1], // Will be extended
}

/// Extension function structure
#[repr(C)]
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
    pub suppress_lint: awk_bool_t,
    pub data: *mut c_void,
}

/// Opaque extension ID type
pub type awk_ext_id_t = *mut c_void;

/// Main API structure containing all function pointers
#[repr(C)]
pub struct gawk_api_t {
    // Data fields
    pub major_version: c_int,
    pub minor_version: c_int,
    pub gmp_major_version: c_int,
    pub gmp_minor_version: c_int,
    pub mpfr_major_version: c_int,
    pub mpfr_minor_version: c_int,
    pub do_flags: [c_int; 6],

    // Function pointers
    pub api_add_ext_func: Option<
        extern "C" fn(
            awk_ext_id_t,
            *const c_char,
            *mut awk_ext_func_t,
        ) -> awk_bool_t,
    >,
    pub api_register_input_parser: Option<
        extern "C" fn(awk_ext_id_t, *mut awk_input_parser_t),
    >,
    pub api_register_output_wrapper: Option<
        extern "C" fn(awk_ext_id_t, *mut awk_output_wrapper_t),
    >,
    pub api_register_two_way_processor: Option<
        extern "C" fn(awk_ext_id_t, *mut awk_two_way_processor_t),
    >,
    pub api_awk_atexit: Option<
        extern "C" fn(
            awk_ext_id_t,
            extern "C" fn(*mut c_void, c_int),
            *mut c_void,
        ),
    >,
    pub api_register_ext_version: Option<
        extern "C" fn(awk_ext_id_t, *const c_char),
    >,
    pub api_fatal: Option<extern "C" fn(awk_ext_id_t, *const c_char, ...)>,
    pub api_warning: Option<extern "C" fn(awk_ext_id_t, *const c_char, ...)>,
    pub api_lintwarn: Option<extern "C" fn(awk_ext_id_t, *const c_char, ...)>,
    pub api_nonfatal: Option<extern "C" fn(awk_ext_id_t, *const c_char, ...)>,
    pub api_update_ERRNO_int: Option<extern "C" fn(awk_ext_id_t, c_int)>,
    pub api_update_ERRNO_string: Option<extern "C" fn(awk_ext_id_t, *const c_char)>,
    pub api_unset_ERRNO: Option<extern "C" fn(awk_ext_id_t)>,
    pub api_get_argument: Option<
        extern "C" fn(
            awk_ext_id_t,
            usize,
            awk_valtype_t,
            *mut awk_value_t,
        ) -> awk_bool_t,
    >,
    pub api_set_argument: Option<
        extern "C" fn(awk_ext_id_t, usize, *mut c_void) -> awk_bool_t,
    >,
    pub api_sym_lookup: Option<
        extern "C" fn(
            awk_ext_id_t,
            *const c_char,
            *const c_char,
            awk_valtype_t,
            *mut awk_value_t,
        ) -> awk_bool_t,
    >,
    pub api_sym_update: Option<
        extern "C" fn(
            awk_ext_id_t,
            *const c_char,
            *const c_char,
            *mut awk_value_t,
        ) -> awk_bool_t,
    >,
    pub api_sym_lookup_scalar: Option<
        extern "C" fn(
            awk_ext_id_t,
            *mut c_void,
            awk_valtype_t,
            *mut awk_value_t,
        ) -> awk_bool_t,
    >,
    pub api_sym_update_scalar: Option<
        extern "C" fn(awk_ext_id_t, *mut c_void, *mut awk_value_t) -> awk_bool_t,
    >,
    pub api_create_value: Option<
        extern "C" fn(
            awk_ext_id_t,
            *mut awk_value_t,
            *mut *mut c_void,
        ) -> awk_bool_t,
    >,
    pub api_release_value: Option<
        extern "C" fn(awk_ext_id_t, *mut c_void) -> awk_bool_t,
    >,
    pub api_get_element_count: Option<
        extern "C" fn(awk_ext_id_t, *mut c_void, *mut usize) -> awk_bool_t,
    >,
    pub api_get_array_element: Option<
        extern "C" fn(
            awk_ext_id_t,
            *mut c_void,
            *const awk_value_t,
            awk_valtype_t,
            *mut awk_value_t,
        ) -> awk_bool_t,
    >,
    pub api_set_array_element: Option<
        extern "C" fn(
            awk_ext_id_t,
            *mut c_void,
            *const awk_value_t,
            *const awk_value_t,
        ) -> awk_bool_t,
    >,
    pub api_del_array_element: Option<
        extern "C" fn(awk_ext_id_t, *mut c_void, *const awk_value_t) -> awk_bool_t,
    >,
    pub api_create_array: Option<extern "C" fn(awk_ext_id_t) -> *mut c_void>,
    pub api_clear_array: Option<
        extern "C" fn(awk_ext_id_t, *mut c_void) -> awk_bool_t,
    >,
    pub api_flatten_array_typed: Option<
        extern "C" fn(
            awk_ext_id_t,
            *mut c_void,
            *mut *mut awk_flat_array_t,
            awk_valtype_t,
            awk_valtype_t,
        ) -> awk_bool_t,
    >,
    pub api_release_flattened_array: Option<
        extern "C" fn(awk_ext_id_t, *mut c_void, *mut awk_flat_array_t) -> awk_bool_t,
    >,
    pub api_malloc: Option<extern "C" fn(usize) -> *mut c_void>,
    pub api_calloc: Option<extern "C" fn(usize, usize) -> *mut c_void>,
    pub api_realloc: Option<extern "C" fn(*mut c_void, usize) -> *mut c_void>,
    pub api_free: Option<extern "C" fn(*mut c_void)>,
    pub api_get_mpfr: Option<extern "C" fn(awk_ext_id_t) -> *mut c_void>,
    pub api_get_mpz: Option<extern "C" fn(awk_ext_id_t) -> *mut c_void>,
    pub api_get_file: Option<
        extern "C" fn(
            awk_ext_id_t,
            *const c_char,
            usize,
            *const c_char,
            c_int,
            *mut *const awk_input,
            *mut *const awk_output_buf_t,
        ) -> awk_bool_t,
    >,
    pub api_destroy_array: Option<
        extern "C" fn(awk_ext_id_t, *mut c_void) -> awk_bool_t,
    >,
}

// Implementation of constructor functions
extern "C" fn r_make_string_type(
    api: *const gawk_api_t,
    ext_id: awk_ext_id_t,
    string: *const c_char,
    length: usize,
    duplicate: awk_bool_t,
    result: *mut awk_value_t,
    val_type: awk_valtype_t,
) -> *mut awk_value_t {
    unsafe {
        (*result).val_type = val_type;
        str_value!(result).len = length;

        if duplicate.into() {
            let cp = ((*api).api_malloc.unwrap())(length + 1) as *mut c_char;
            libc::memcpy(
                cp as *mut c_void,
                string as *const c_void,
                length,
            );
            *cp.add(length) = 0;
            str_value!(result).str = cp;
        } else {
            str_value!(result).str = string as *mut c_char;
        }

        result
    }
}

extern "C" fn r_make_string(
    api: *const gawk_api_t,
    ext_id: awk_ext_id_t,
    string: *const c_char,
    length: usize,
    duplicate: awk_bool_t,
    result: *mut awk_value_t,
) -> *mut awk_value_t {
    r_make_string_type(
        api,
        ext_id,
        string,
        length,
        duplicate,
        result,
        awk_valtype_t::AWK_STRING,
    )
}

extern "C" fn make_null_string(result: *mut awk_value_t) -> *mut awk_value_t {
    unsafe {
        libc::memset(result as *mut c_void, 0, mem::size_of::<awk_value_t>());
        (*result).val_type = awk_valtype_t::AWK_UNDEFINED;
        result
    }
}

extern "C" fn make_number(num: f64, result: *mut awk_value_t) -> *mut awk_value_t {
    unsafe {
        (*result).val_type = awk_valtype_t::AWK_NUMBER;
        num_value!(result) = num;
        num_type