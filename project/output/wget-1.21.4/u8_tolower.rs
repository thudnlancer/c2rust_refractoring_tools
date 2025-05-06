#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types)]
extern "C" {
    pub type unicode_normalization_form;
    fn uc_tolower(uc: ucs4_t) -> ucs4_t;
    static unicase_empty_prefix_context: casing_prefix_context_t;
    static unicase_empty_suffix_context: casing_suffix_context_t;
    fn u8_casemap(
        s: *const uint8_t,
        n: size_t,
        prefix_context: casing_prefix_context_t,
        suffix_context: casing_suffix_context_t,
        iso639_language: *const i8,
        single_character_map: Option<unsafe extern "C" fn(ucs4_t) -> ucs4_t>,
        offset_in_rule: size_t,
        nf: uninorm_t,
        resultbuf: *mut uint8_t,
        lengthp: *mut size_t,
    ) -> *mut uint8_t;
}
pub type __uint8_t = u8;
pub type __uint32_t = u32;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type ucs4_t = uint32_t;
pub type size_t = u64;
pub type uninorm_t = *const unicode_normalization_form;
pub type casing_suffix_context_t = casing_suffix_context;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct casing_suffix_context {
    pub first_char_except_ignorable: uint32_t,
    pub bits: uint32_t,
}
pub type casing_prefix_context_t = casing_prefix_context;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct casing_prefix_context {
    pub last_char_except_ignorable: uint32_t,
    pub last_char_normal_or_above: uint32_t,
}
#[no_mangle]
pub unsafe extern "C" fn u8_tolower(
    mut s: *const uint8_t,
    mut n: size_t,
    mut iso639_language: *const i8,
    mut nf: uninorm_t,
    mut resultbuf: *mut uint8_t,
    mut lengthp: *mut size_t,
) -> *mut uint8_t {
    return u8_casemap(
        s,
        n,
        unicase_empty_prefix_context,
        unicase_empty_suffix_context,
        iso639_language,
        Some(uc_tolower as unsafe extern "C" fn(ucs4_t) -> ucs4_t),
        12 as u64,
        nf,
        resultbuf,
        lengthp,
    );
}