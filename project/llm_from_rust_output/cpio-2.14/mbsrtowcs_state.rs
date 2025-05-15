use std::mem::MaybeUninit;

#[derive(Copy, Clone)]
pub struct MbState {
    count: i32,
    value: MbStateValue,
}

#[derive(Copy, Clone)]
pub union MbStateValue {
    wch: u32,
    wchb: [MaybeUninit<u8>; 4],
}

impl Default for MbState {
    fn default() -> Self {
        Self {
            count: 0,
            value: MbStateValue { wch: 0 },
        }
    }
}

pub static GL_MBSRTOWCS_STATE: MbState = MbState::default();