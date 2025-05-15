use std::num::NonZeroU32;

pub type UInt32 = u32;

#[derive(Copy, Clone, Debug)]
pub struct CasingSuffixContext {
    pub first_char_except_ignorable: Option<NonZeroU32>,
    pub bits: UInt32,
}

pub type CasingSuffixContextT = CasingSuffixContext;

pub const UNICASE_EMPTY_SUFFIX_CONTEXT: CasingSuffixContextT = CasingSuffixContext {
    first_char_except_ignorable: None,
    bits: 0,
};