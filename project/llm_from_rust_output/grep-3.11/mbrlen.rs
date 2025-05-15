use std::ffi::CStr;
use std::os::raw::{c_char, c_int, c_uint, c_ulong};

pub type size_t = c_ulong;
pub type wchar_t = c_int;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct MbState {
    pub __count: c_int,
    pub __value: MbStateValue,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union MbStateValue {
    pub __wch: c_uint,
    pub __wchb: [c_char; 4],
}

impl Default for MbState {
    fn default() -> Self {
        Self {
            __count: 0,
            __value: MbStateValue { __wch: 0 },
        }
    }
}

static INTERNAL_STATE: std::sync::Mutex<MbState> = std::sync::Mutex::new(MbState::default());

pub fn rpl_mbrlen(s: Option<&CStr>, n: size_t, ps: Option<&mut MbState>) -> size_t {
    let mut state = match ps {
        Some(state) => state,
        None => INTERNAL_STATE.lock().unwrap(),
    };

    let s_ptr = s.map(|s| s.as_ptr()).unwrap_or(std::ptr::null());
    
    unsafe {
        // This is the minimal unsafe block required for the FFI call
        // The safety relies on proper state management and valid pointers
        libc::mbrtowc(
            std::ptr::null_mut(),
            s_ptr,
            n,
            state as *mut MbState as *mut libc::mbstate_t,
        )
    }
}