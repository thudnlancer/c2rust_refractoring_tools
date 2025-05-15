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

pub static GLOBAL_MBSRTOWCS_STATE: std::sync::Mutex<MbState> = std::sync::Mutex::new(MbState::default());