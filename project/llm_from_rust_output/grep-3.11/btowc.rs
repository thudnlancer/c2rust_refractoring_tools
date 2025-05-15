use std::mem;

pub type wchar_t = i32;
pub type wint_t = u32;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct MbState {
    pub count: i32,
    pub value: MbStateValue,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union MbStateValue {
    pub wch: u32,
    pub wchb: [i8; 4],
}

impl Default for MbState {
    fn default() -> Self {
        MbState {
            count: 0,
            value: MbStateValue { wch: 0 },
        }
    }
}

pub fn rpl_btowc(c: i32) -> wint_t {
    if c == -1 {
        return 0xffffffff;
    }

    let buf = [c as i8];
    let mut wc: wchar_t = 0;
    let mut state = MbState::default();

    let ret = unsafe {
        libc::mbrtowc(
            &mut wc as *mut wchar_t,
            buf.as_ptr(),
            1,
            &mut state as *mut MbState as *mut libc::mbstate_t,
        )
    };

    if ret != libc::size_t::MAX && ret != libc::size_t::MAX - 1 {
        wc as wint_t
    } else {
        0xffffffff
    }
}