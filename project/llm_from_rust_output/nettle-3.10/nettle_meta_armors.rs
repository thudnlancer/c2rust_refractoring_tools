use std::ffi::CStr;
use std::os::raw::{c_char, c_uint, c_int, c_void};

pub type size_t = usize;
pub type uint8_t = u8;

pub struct ArmorFunctions {
    pub encode_init: fn(*mut c_void),
    pub encode_length: fn(size_t) -> size_t,
    pub encode_update: fn(*mut c_void, *mut c_char, size_t, *const uint8_t) -> size_t,
    pub encode_final: fn(*mut c_void, *mut c_char) -> size_t,
    pub decode_init: fn(*mut c_void),
    pub decode_length: fn(size_t) -> size_t,
    pub decode_update: fn(*mut c_void, *mut size_t, *mut uint8_t, size_t, *const c_char) -> c_int,
    pub decode_final: fn(*mut c_void) -> c_int,
}

pub struct Armor {
    pub name: &'static CStr,
    pub encode_context_size: c_uint,
    pub decode_context_size: c_uint,
    pub encode_final_length: c_uint,
    pub functions: ArmorFunctions,
}

static BASE64: Armor = Armor {
    name: unsafe { CStr::from_bytes_with_nul_unchecked(b"base64\0") },
    encode_context_size: 0,
    decode_context_size: 0,
    encode_final_length: 0,
    functions: ArmorFunctions {
        encode_init: |_| {},
        encode_length: |_| 0,
        encode_update: |_, _, _, _| 0,
        encode_final: |_, _| 0,
        decode_init: |_| {},
        decode_length: |_| 0,
        decode_update: |_, _, _, _, _| 0,
        decode_final: |_| 0,
    },
};

static BASE64URL: Armor = Armor {
    name: unsafe { CStr::from_bytes_with_nul_unchecked(b"base64url\0") },
    encode_context_size: 0,
    decode_context_size: 0,
    encode_final_length: 0,
    functions: ArmorFunctions {
        encode_init: |_| {},
        encode_length: |_| 0,
        encode_update: |_, _, _, _| 0,
        encode_final: |_, _| 0,
        decode_init: |_| {},
        decode_length: |_| 0,
        decode_update: |_, _, _, _, _| 0,
        decode_final: |_| 0,
    },
};

static BASE16: Armor = Armor {
    name: unsafe { CStr::from_bytes_with_nul_unchecked(b"base16\0") },
    encode_context_size: 0,
    decode_context_size: 0,
    encode_final_length: 0,
    functions: ArmorFunctions {
        encode_init: |_| {},
        encode_length: |_| 0,
        encode_update: |_, _, _, _| 0,
        encode_final: |_, _| 0,
        decode_init: |_| {},
        decode_length: |_| 0,
        decode_update: |_, _, _, _, _| 0,
        decode_final: |_| 0,
    },
};

pub fn get_armors() -> &'static [&'static Armor] {
    &[&BASE64, &BASE64URL, &BASE16]
}