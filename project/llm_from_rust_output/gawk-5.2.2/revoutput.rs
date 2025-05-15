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
    pub b: c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct awk_value {
    pub val_type: awk_valtype_t,
    pub u: awk_value_u,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct awk_output_buf {
    pub name: *const c_char,
    pub mode: *const c_char,
    pub fp: *mut libc::FILE,
    pub redirected: c_int,
    pub opaque: *mut c_void,
    pub gawk_fwrite: Option<
        extern "C" fn(*const c_void, usize, usize, *mut libc::FILE, *mut c_void) -> usize,
    >,
    pub gawk_fflush: Option<extern "C" fn(*mut libc::FILE, *mut c_void) -> c_int>,
    pub gawk_ferror: Option<extern "C" fn(*mut libc::FILE, *mut c_void) -> c_int>,
    pub gawk_fclose: Option<extern "C" fn(*mut libc::FILE, *mut c_void) -> c_int>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct awk_output_wrapper {
    pub name: *const c_char,
    pub can_take_file: Option<extern "C" fn(*const awk_output_buf) -> c_int>,
    pub take_control_of: Option<extern "C" fn(*mut awk_output_buf) -> c_int>,
    pub next: *const awk_output_wrapper,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gawk_api {
    pub major_version: c_int,
    pub minor_version: c_int,
    // ... other fields omitted for brevity
    pub api_register_output_wrapper:
        Option<extern "C" fn(*mut c_void, *mut awk_output_wrapper)>,
    pub api_sym_lookup: Option<
        extern "C" fn(
            *mut c_void,
            *const c_char,
            *const c_char,
            awk_valtype_t,
            *mut awk_value,
        ) -> c_int,
    >,
    pub api_sym_update: Option<
        extern "C" fn(*mut c_void, *const c_char, *const c_char, *mut awk_value) -> c_int,
    >,
    pub api_warning: Option<extern "C" fn(*mut c_void, *const c_char, ...)>,
    // ... other fields omitted for brevity
}

static mut API: *const gawk_api = ptr::null();
static mut EXT_ID: *mut c_void = ptr::null_mut();
static EXT_VERSION: &'static [u8] = b"revoutput extension: version 1.1\0";

extern "C" fn rev_fwrite(
    buf: *const c_void,
    size: usize,
    count: usize,
    fp: *mut libc::FILE,
    _opaque: *mut c_void,
) -> usize {
    unsafe {
        let cp = buf as *const c_char;
        let mut nbytes = size.wrapping_mul(count) as c_int;
        while nbytes >= 1 {
            libc::putc(*cp.offset((nbytes - 1) as isize) as c_int, fp);
            nbytes -= 1;
        }
        size.wrapping_mul(count)
    }
}

extern "C" fn revoutput_can_take_file(outbuf: *const awk_output_buf) -> c_int {
    unsafe {
        if outbuf.is_null() {
            return 0;
        }

        let mut value = awk_value {
            val_type: awk_valtype_t::NUMBER,
            u: awk_value_u { n: awk_number { d: 0.0, type_: AWK_NUMBER_TYPE::DOUBLE, ptr: ptr::null_mut() } },
        };

        if let Some(api_sym_lookup) = (*API).api_sym_lookup {
            if api_sym_lookup(
                EXT_ID,
                b"\0".as_ptr() as *const c_char,
                b"REVOUT\0".as_ptr() as *const c_char,
                awk_valtype_t::NUMBER,
                &mut value,
            ) == 0
            {
                return 0;
            }
        }

        (value.u.n.d != 0.0) as c_int
    }
}

extern "C" fn revoutput_take_control_of(outbuf: *mut awk_output_buf) -> c_int {
    unsafe {
        if outbuf.is_null() {
            return 0;
        }

        (*outbuf).gawk_fwrite = Some(rev_fwrite);
        (*outbuf).redirected = 1;
        1
    }
}

static mut OUTPUT_WRAPPER: awk_output_wrapper = awk_output_wrapper {
    name: b"revoutput\0".as_ptr() as *const c_char,
    can_take_file: Some(revoutput_can_take_file),
    take_control_of: Some(revoutput_take_control_of),
    next: ptr::null(),
};

#[no_mangle]
pub extern "C" fn dl_load(api_p: *const gawk_api, id: *mut c_void) -> c_int {
    unsafe {
        API = api_p;
        EXT_ID = id;

        if (*API).major_version != 3 || (*API).minor_version < 2 {
            eprintln!("revoutput: version mismatch with gawk!");
            eprintln!(
                "\tmy version (API 3.2), gawk version (API {}.{})",
                (*API).major_version, (*API).minor_version
            );
            return 0;
        }

        if let Some(api_register_output_wrapper) = (*API).api_register_output_wrapper {
            api_register_output_wrapper(EXT_ID, &mut OUTPUT_WRAPPER);
        }

        if let Some(api_register_ext_version) = (*API).api_register_ext_version {
            api_register_ext_version(EXT_ID, EXT_VERSION.as_ptr() as *const c_char);
        }

        1
    }
}