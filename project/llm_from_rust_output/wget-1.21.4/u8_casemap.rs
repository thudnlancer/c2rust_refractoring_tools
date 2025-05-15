use std::ptr;
use std::mem;
use std::ffi::CStr;
use std::os::raw::{c_char, c_int, c_uchar, c_uint, c_ulong, c_ushort, c_void};
use std::slice;

type ptrdiff_t = i64;
type size_t = u64;
type uint8_t = u8;
type uint32_t = u32;
type ucs4_t = uint32_t;

#[repr(C)]
pub struct unicode_normalization_form;

type uninorm_t = *const unicode_normalization_form;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct casing_prefix_context {
    pub last_char_except_ignorable: uint32_t,
    pub last_char_normal_or_above: uint32_t,
}

pub type casing_prefix_context_t = casing_prefix_context;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct casing_suffix_context {
    pub first_char_except_ignorable: uint32_t,
    pub bits: uint32_t,
}

pub type casing_suffix_context_t = casing_suffix_context;

const UC_CCC_NR: u32 = 0;
const UC_CCC_A: u32 = 230;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct special_casing_rule {
    pub code: [c_char; 3],
    pub has_next_context: u8,
    pub language: [c_char; 2],
    pub upper: [c_ushort; 3],
    pub lower: [c_ushort; 3],
    pub title: [c_ushort; 3],
    pub casefold: [c_ushort; 3],
}

impl special_casing_rule {
    fn has_next(&self) -> bool {
        (self.has_next_context & 0x1) != 0
    }

    fn context(&self) -> i32 {
        ((self.has_next_context >> 1) & 0x7) as i32
    }
}

const SCC_AFTER_I: u32 = 5;
const SCC_BEFORE_DOT: u32 = 4;
const SCC_MORE_ABOVE: u32 = 3;
const SCC_AFTER_SOFT_DOTTED: u32 = 2;
const SCC_FINAL_SIGMA: u32 = 1;
const SCC_ALWAYS: u32 = 0;

const UC_CCC_IS: u32 = 240;
const UC_CCC_DA: u32 = 234;
const UC_CCC_DB: u32 = 233;
const UC_CCC_AR: u32 = 232;
const UC_CCC_AL: u32 = 228;
const UC_CCC_R: u32 = 226;
const UC_CCC_L: u32 = 224;
const UC_CCC_BR: u32 = 222;
const UC_CCC_B: u32 = 220;
const UC_CCC_BL: u32 = 218;
const UC_CCC_ATAR: u32 = 216;
const UC_CCC_ATA: u32 = 214;
const UC_CCC_ATB: u32 = 202;
const UC_CCC_ATBL: u32 = 200;
const UC_CCC_VR: u32 = 9;
const UC_CCC_KV: u32 = 8;
const UC_CCC_NK: u32 = 7;
const UC_CCC_OV: u32 = 1;

fn u8_mbtouc_unsafe(puc: &mut ucs4_t, s: &[u8]) -> i32 {
    if s.is_empty() {
        return -1;
    }
    let c = s[0];
    if c < 0x80 {
        *puc = c as ucs4_t;
        1
    } else {
        unsafe { u8_mbtouc_unsafe_aux(puc, s.as_ptr(), s.len() as size_t) }
    }
}

fn u8_uctomb(s: &mut [u8], uc: ucs4_t) -> i32 {
    if uc < 0x80 && !s.is_empty() {
        s[0] = uc as u8;
        1
    } else {
        unsafe { u8_uctomb_aux(s.as_mut_ptr(), uc, s.len() as ptrdiff_t) }
    }
}

fn u8_casemap(
    s: &[u8],
    prefix_context: casing_prefix_context_t,
    suffix_context: casing_suffix_context_t,
    iso639_language: Option<&str>,
    single_character_map: impl Fn(ucs4_t) -> ucs4_t,
    offset_in_rule: size_t,
    nf: Option<uninorm_t>,
    resultbuf: Option<&mut [u8]>,
) -> Result<Vec<u8>, i32> {
    let mut result = Vec::new();
    let mut last_char_except_ignorable = prefix_context.last_char_except_ignorable;
    let mut last_char_normal_or_above = prefix_context.last_char_normal_or_above;

    let mut s_pos = 0;
    while s_pos < s.len() {
        let remaining = &s[s_pos..];
        let mut uc = 0;
        let count = u8_mbtouc_unsafe(&mut uc, remaining) as usize;
        if count == 0 || count > remaining.len() {
            return Err(22); // EINVAL
        }

        let mut mapped_uc = [0; 3];
        let mapped_count = if uc < 0x10000 {
            let code = [
                ((uc >> 8) & 0xff) as c_char,
                (uc & 0xff) as c_char,
                0 as c_char,
            ];
            let mut found = false;
            let mut mapped = 0;
            let mut code_copy = code;
            
            loop {
                let rule_ptr = unsafe {
                    gl_unicase_special_lookup(code_copy.as_ptr(), 3)
                };
                if rule_ptr.is_null() {
                    break;
                }

                let rule = unsafe { &*rule_ptr };
                let lang_match = rule.language[0] == 0 || 
                    iso639_language.map_or(false, |lang| {
                        let lang_bytes = lang.as_bytes();
                        lang_bytes.len() >= 2 &&
                        lang_bytes[0] == rule.language[0] as u8 &&
                        lang_bytes[1] == rule.language[1] as u8
                    });

                if lang_match {
                    let mut context = rule.context();
                    let mut applies = false;

                    if context < 0 {
                        context = -context;
                    }

                    applies = match context {
                        SCC_ALWAYS => true,
                        SCC_FINAL_SIGMA => {
                            let has_cased_before = uc_is_cased(last_char_except_ignorable);
                            if !has_cased_before {
                                false
                            } else {
                                let mut s2_pos = s_pos + count;
                                let mut found_non_ignorable = false;
                                while s2_pos < s.len() {
                                    let mut uc2 = 0;
                                    let count2 = u8_mbtouc_unsafe(&mut uc2, &s[s2_pos..]) as usize;
                                    if !uc_is_case_ignorable(uc2) {
                                        found_non_ignorable = true;
                                        !uc_is_cased(uc2)
                                    } else {
                                        s2_pos += count2;
                                        continue;
                                    }
                                }
                                if !found_non_ignorable {
                                    !uc_is_cased(suffix_context.first_char_except_ignorable)
                                } else {
                                    false
                                }
                            }
                        }
                        SCC_AFTER_SOFT_DOTTED => uc_is_property_soft_dotted(last_char_normal_or_above),
                        SCC_MORE_ABOVE => {
                            let mut s2_pos = s_pos + count;
                            let mut applies = false;
                            while s2_pos < s.len() {
                                let mut uc2 = 0;
                                let count2 = u8_mbtouc_unsafe(&mut uc2, &s[s2_pos..]) as usize;
                                let ccc = uc_combining_class(uc2);
                                if ccc == UC_CCC_A as i32 {
                                    applies = true;
                                    break;
                                } else if ccc == UC_CCC_NR as i32 {
                                    break;
                                }
                                s2_pos += count2;
                            }
                            if !applies {
                                (suffix_context.bits & 1) != 0
                            } else {
                                applies
                            }
                        }
                        SCC_BEFORE_DOT => {
                            let mut s2_pos = s_pos + count;
                            let mut applies = false;
                            while s2_pos < s.len() {
                                let mut uc2 = 0;
                                let count2 = u8_mbtouc_unsafe(&mut uc2, &s[s2_pos..]) as usize;
                                if uc2 == 0x307 {
                                    applies = true;
                                    break;
                                } else {
                                    let ccc = uc_combining_class(uc2);
                                    if ccc == UC_CCC_A as i32 || ccc == UC_CCC_NR as i32 {
                                        break;
                                    }
                                    s2_pos += count2;
                                }
                            }
                            if !applies {
                                (suffix_context.bits & 2) != 0
                            } else {
                                applies
                            }
                        }
                        SCC_AFTER_I => last_char_normal_or_above == 'I' as u32,
                        _ => false,
                    };

                    if rule.context() < 0 {
                        applies = !applies;
                    }

                    if applies {
                        let mapped_in_rule = unsafe {
                            let ptr = (rule as *const _ as *const u16)
                                .offset(offset_in_rule as isize);
                            slice::from_raw_parts(ptr, 3)
                        };

                        if mapped_in_rule[0] == 0 {
                            mapped = 0;
                        } else {
                            mapped_uc[0] = mapped_in_rule[0] as u32;
                            if mapped_in_rule[1] == 0 {
                                mapped = 1;
                            } else {
                                mapped_uc[1] = mapped_in_rule[1] as u32;
                                if mapped_in_rule[2] == 0 {
                                    mapped = 2;
                                } else {
                                    mapped_uc[2] = mapped_in_rule[2] as u32;
                                    mapped = 3;
                                }
                            }
                        }
                        found = true;
                        break;
                    }
                }

                if !rule.has_next() {
                    break;
                }
                code_copy[2] += 1;
            }

            if found {
                mapped
            } else {
                mapped_uc[0] = single_character_map(uc);
                1
            }
        } else {
            mapped_uc[0] = single_character_map(uc);
            1
        };

        for i in 0..mapped_count {
            let muc = mapped_uc[i as usize];
            let mut buf = [0; 4];
            let ret = u8_uctomb(&mut buf, muc);
            if ret <= 0 {
                return Err(22); // EINVAL
            }
            result.extend_from_slice(&buf[..ret as usize]);
        }

        if !uc_is_case_ignorable(uc) {
            last_char_except_ignorable = uc;
        }
        let ccc = uc_combining_class(uc);
        if ccc == UC_CCC_A as i32 || ccc == UC_CCC_NR as i32 {
            last_char_normal_or_above = uc;
        }

        s_pos += count;
    }

    if let Some(nf) = nf {
        let mut length = result.len();
        let mut normalized = unsafe {
            let mut normalized_ptr = u8_normalize(
                nf,
                result.as_ptr(),
                length as size_t,
                ptr::null_mut(),
                &mut length as *mut _,
            );
            if normalized_ptr.is_null() {
                return Err(*__errno_location());
            }
            Vec::from_raw_parts(normalized_ptr, length, length)
        };
        Ok(normalized)
    } else {
        Ok(result)
    }
}

// External C functions - these would need to be linked from a C library
extern "C" {
    fn u8_normalize(
        nf: uninorm_t,
        s: *const uint8_t,
        n: size_t,
        resultbuf: *mut uint8_t,
        lengthp: *mut size_t,
    ) -> *mut uint8_t;
    fn __errno_location() -> *mut c_int;
    fn u8_mbtouc_unsafe_aux(
        puc: *mut ucs4_t,
        s: *const uint8_t,
        n: size_t,
    ) -> c_int;
    fn u8_uctomb_aux(s: *mut uint8_t, uc: ucs4_t, n: ptrdiff_t) -> c_int;
    fn u8_cpy(dest: *mut uint8_t, src: *const uint8_t, n: size_t) -> *mut uint8_t;
    fn uc_combining_class(uc: ucs4_t) -> c_int;
    fn uc_is_property_soft_dotted(uc: ucs4_t) -> bool;
    fn uc_is_cased(uc: ucs4_t) -> bool;
    fn uc_is_case_ignorable(uc: ucs4_t) -> bool;
    fn gl_unicase_special_lookup(
        str: *const c_char,
        len: size_t,
    ) -> *const special_casing_rule;
    fn malloc(size: c_ulong) -> *mut c_void;
    fn realloc(ptr: *mut c_void, size: c_ulong) -> *mut c_void;
    fn rpl_free(ptr: *mut c_void);
}