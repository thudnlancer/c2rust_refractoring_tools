// Normally this would be wctype.c, but that name's already taken.
#![allow(non_snake_case)]

use libc::{wctype_t, wint_t};
use std::os::raw::c_int;

extern "C" {
    fn iswctype(wc: wint_t, desc: wctype_t) -> c_int;
}

pub type wctype = wctype_t;
pub const _GL_WCTYPE_INLINE: bool = true;