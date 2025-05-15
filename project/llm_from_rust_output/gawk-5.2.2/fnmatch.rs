use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_double, c_int, c_void};
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
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AWK_NUMBER_TYPE {
    AWK_NUMBER_TYPE_DOUBLE = 0,
    AWK_NUMBER_TYPE_MPFR = 1,
    AWK_NUMBER_TYPE_MPZ = 2,
}

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

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub union awk_value_u {
    pub s: awk_string_t,
    pub n: awk_number_t,
    pub a: *mut c_void,
    pub scl: *mut c_void,
    pub vc: *mut c_void,
    pub b: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct awk_value_t {
    pub val_type: awk_valtype_t,
    pub u: awk_value_u,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fnmflags {
    pub name: *const c_char,
    pub value: c_int,
}

static mut FLAGTABLE: [fnmflags; 7] = [
    fnmflags {
        name: b"CASEFOLD\0".as_ptr() as *const c_char,
        value: 1 << 4,
    },
    fnmflags {
        name: b"FILE_NAME\0".as_ptr() as *const c_char,
        value: 1 << 0,
    },
    fnmflags {
        name: b"LEADING_DIR\0".as_ptr() as *const c_char,
        value: 1 << 3,
    },
    fnmflags {
        name: b"NOESCAPE\0".as_ptr() as *const c_char,
        value: 1 << 1,
    },
    fnmflags {
        name: b"PATHNAME\0".as_ptr() as *const c_char,
        value: 1 << 0,
    },
    fnmflags {
        name: b"PERIOD\0".as_ptr() as *const c_char,
        value: 1 << 2,
    },
    fnmflags {
        name: ptr::null(),
        value: 0,
    },
];

fn make_number(num: c_double, result: &mut awk_value_t) {
    result.val_type = awk_valtype_t::AWK_NUMBER;
    unsafe {
        result.u.n = awk_number_t {
            d: num,
            type_: AWK_NUMBER_TYPE::AWK_NUMBER_TYPE_DOUBLE,
            ptr: ptr::null_mut(),
        };
    }
}

fn r_make_string_type(
    string: &str,
    duplicate: bool,
    result: &mut awk_value_t,
    val_type: awk_valtype_t,
) {
    result.val_type = val_type;
    unsafe {
        result.u.s = awk_string_t {
            str_: if duplicate {
                let cstr = CString::new(string).unwrap();
                cstr.into_raw()
            } else {
                string.as_ptr() as *mut c_char
            },
            len: string.len(),
        };
    }
}

fn r_make_string(string: &str, duplicate: bool, result: &mut awk_value_t) {
    r_make_string_type(string, duplicate, result, awk_valtype_t::AWK_STRING);
}

fn do_fnmatch(
    pattern: &str,
    string: &str,
    flags: c_int,
) -> Result<c_int, &'static str> {
    let flags_mask = (1 << 4) | (1 << 0) | (1 << 3) | (1 << 1) | (1 << 0) | (1 << 2);
    let int_flags = flags & flags_mask;
    
    let pattern_c = CString::new(pattern).map_err(|_| "Pattern contains null byte")?;
    let string_c = CString::new(string).map_err(|_| "String contains null byte")?;
    
    let ret = unsafe {
        libc::fnmatch(
            pattern_c.as_ptr(),
            string_c.as_ptr(),
            int_flags,
        )
    };
    
    Ok(ret)
}

fn init_fnmatch() -> Result<(), &'static str> {
    let mut errors = 0;
    
    let mut value = awk_value_t {
        val_type: awk_valtype_t::AWK_UNDEFINED,
        u: unsafe { std::mem::zeroed() },
    };
    
    make_number(1.0, &mut value);
    
    // TODO: Implement symbol updates using safe Rust
    
    let mut new_array = ptr::null_mut(); // TODO: Create array using safe Rust
    
    for flag in unsafe { &FLAGTABLE } {
        if flag.name.is_null() {
            break;
        }
        
        let mut index = awk_value_t {
            val_type: awk_valtype_t::AWK_UNDEFINED,
            u: unsafe { std::mem::zeroed() },
        };
        
        let name = unsafe { CStr::from_ptr(flag.name) }.to_str().unwrap();
        r_make_string(name, true, &mut index);
        
        make_number(flag.value as c_double, &mut value);
        
        // TODO: Implement array element setting using safe Rust
    }
    
    if errors == 0 {
        Ok(())
    } else {
        Err("Initialization failed")
    }
}

struct Plugin {
    api: *const gawk_api_t,
    ext_id: *mut c_void,
    ext_version: &'static str,
}

impl Plugin {
    unsafe fn new(api: *const gawk_api_t, id: *mut c_void) -> Result<Self, &'static str> {
        if (*api).major_version != 3 || (*api).minor_version < 2 {
            return Err("Version mismatch with gawk!");
        }
        
        Ok(Self {
            api,
            ext_id: id,
            ext_version: "fnmatch extension: version 1.0",
        })
    }
    
    fn load(&self) -> Result<(), &'static str> {
        // TODO: Implement function registration using safe Rust
        init_fnmatch()?;
        Ok(())
    }
}

#[no_mangle]
pub unsafe extern "C" fn dl_load(api_p: *const gawk_api_t, id: *mut c_void) -> c_int {
    match Plugin::new(api_p, id) {
        Ok(plugin) => match plugin.load() {
            Ok(_) => 1,
            Err(_) => 0,
        },
        Err(_) => 0,
    }
}

#[no_mangle]
pub static plugin_is_GPL_compatible: c_int = 1;