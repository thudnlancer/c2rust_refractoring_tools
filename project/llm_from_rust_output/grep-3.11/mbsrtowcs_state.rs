use std::mem::MaybeUninit;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct __mbstate_t {
    pub __count: i32,
    pub __value: C2RustUnnamed,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __wch: u32,
    pub __wchb: [MaybeUninit<u8>; 4],
}

impl Default for C2RustUnnamed {
    fn default() -> Self {
        unsafe { C2RustUnnamed { __wch: 0 } }
    }
}

pub type mbstate_t = __mbstate_t;

#[no_mangle]
pub static _gl_mbsrtowcs_state: mbstate_t = mbstate_t {
    __count: 0,
    __value: C2RustUnnamed::default(),
};