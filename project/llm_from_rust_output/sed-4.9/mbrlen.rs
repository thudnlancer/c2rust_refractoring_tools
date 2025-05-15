use std::ffi::CStr;
use std::os::raw::{c_char, c_int, c_uint, c_ulong};

pub type size_t = c_ulong;
pub type wchar_t = c_int;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct MbState {
    pub count: c_int,
    pub value: MbStateValue,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union MbStateValue {
    pub wch: c_uint,
    pub wchb: [c_char; 4],
}

impl Default for MbState {
    fn default() -> Self {
        Self {
            count: 0,
            value: MbStateValue { wch: 0 },
        }
    }
}

static INTERNAL_STATE: std::sync::Mutex<MbState> = std::sync::Mutex::new(MbState::default());

pub fn rpl_mbrlen(s: Option<&CStr>, n: size_t, ps: Option<&mut MbState>) -> size_t {
    let mut state = match ps {
        Some(state) => state,
        None => &mut *INTERNAL_STATE.lock().unwrap(),
    };

    let s_ptr = s.map_or(std::ptr::null(), |s| s.as_ptr());
    
    unsafe {
        rpl_mbrtowc(None, s_ptr, n, state)
    }
}

extern "C" {
    fn rpl_mbrtowc(
        pwc: Option<&mut wchar_t>,
        s: *const c_char,
        n: size_t,
        ps: &mut MbState,
    ) -> size_t;
}