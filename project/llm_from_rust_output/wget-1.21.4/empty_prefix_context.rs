use std::num::NonZeroU32;

pub type UInt32 = u32;

#[derive(Copy, Clone, Default)]
pub struct CasingPrefixContext {
    pub last_char_except_ignorable: Option<NonZeroU32>,
    pub last_char_normal_or_above: Option<NonZeroU32>,
}

pub type CasingPrefixContextT = CasingPrefixContext;

pub const UNICASE_EMPTY_PREFIX_CONTEXT: CasingPrefixContextT = CasingPrefixContext {
    last_char_except_ignorable: None,
    last_char_normal_or_above: None,
};