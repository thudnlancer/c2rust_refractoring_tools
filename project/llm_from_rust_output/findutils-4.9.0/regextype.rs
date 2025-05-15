use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_uint, c_ulong};
use std::ptr;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RegexTypeMap {
    pub name: *const c_char,
    pub context: c_int,
    pub option_val: c_int,
}

pub const CONTEXT_FINDUTILS: c_uint = 1;
pub const CONTEXT_GENERIC: c_uint = 2;
pub const CONTEXT_ALL: c_uint = 3;
pub const N_REGEX_MAP_ENTRIES: usize = 13;

static REGEX_MAP: [RegexTypeMap; N_REGEX_MAP_ENTRIES] = [
    RegexTypeMap {
        name: b"findutils-default\0".as_ptr() as *const c_char,
        context: CONTEXT_FINDUTILS as c_int,
        option_val: 0 | ((1 << 1) << 1) << 1 << 1 << 1 << 1 << 1,
    },
    RegexTypeMap {
        name: b"ed\0".as_ptr() as *const c_char,
        context: CONTEXT_GENERIC as c_int,
        option_val: ((1 << 1) << 1)
            | (((1 << 1) << 1 << 1 << 1 << 1 << 1)
            | ((((1 << 1) << 1 << 1 << 1 << 1 << 1) << 1)
            | (((((1 << 1) << 1 << 1 << 1 << 1 << 1) << 1) << 1)
            | (((((((1 << 1) << 1 << 1 << 1 << 1 << 1) << 1) << 1) << 1) << 1)
            | (1 << 1)
            | (((((((((((((((((((((((1 << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1) << 1),
    },
    // ... (其他正则表达式类型的类似定义)
];

pub fn get_regex_type(s: &str) -> c_int {
    let c_s = CString::new(s).unwrap();
    for entry in REGEX_MAP.iter() {
        unsafe {
            if CStr::from_ptr(entry.name).to_str().unwrap() == s {
                return entry.option_val;
            }
        }
    }
    
    let mut msg = String::new();
    for (i, entry) in REGEX_MAP.iter().enumerate() {
        if i > 0 {
            msg.push_str(", ");
        }
        unsafe {
            msg.push_str(CStr::from_ptr(entry.name).to_str().unwrap());
        }
    }
    
    panic!("Unknown regular expression type {}; valid types are {}.", s, msg);
}

pub fn get_regex_type_name(ix: c_uint) -> Option<&'static str> {
    if (ix as usize) < REGEX_MAP.len() {
        unsafe {
            Some(CStr::from_ptr(REGEX_MAP[ix as usize].name).to_str().unwrap())
        }
    } else {
        None
    }
}

pub fn get_regex_type_flags(ix: c_uint) -> Option<c_int> {
    if (ix as usize) < REGEX_MAP.len() {
        Some(REGEX_MAP[ix as usize].option_val)
    } else {
        None
    }
}

pub fn get_regex_type_context(ix: c_uint) -> Option<c_uint> {
    if (ix as usize) < REGEX_MAP.len() {
        Some(REGEX_MAP[ix as usize].context as c_uint)
    } else {
        None
    }
}

pub fn get_regex_type_synonym(ix: c_uint, context: c_uint) -> Option<c_uint> {
    if (ix as usize) >= REGEX_MAP.len() {
        return None;
    }
    
    let flags = REGEX_MAP[ix as usize].option_val;
    for i in 0..ix {
        if (REGEX_MAP[i as usize].context as c_uint & context) != 0 && 
           flags == REGEX_MAP[i as usize].option_val {
            return Some(i);
        }
    }
    None
}