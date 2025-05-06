#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
pub type __uint8_t = u8;
pub type __uint32_t = u32;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type ucs4_t = uint32_t;
pub type size_t = u64;
#[no_mangle]
pub unsafe extern "C" fn u8_mbtoucr(
    mut puc: *mut ucs4_t,
    mut s: *const uint8_t,
    mut n: size_t,
) -> i32 {
    let mut c: uint8_t = *s;
    if (c as i32) < 0x80 as i32 {
        *puc = c as ucs4_t;
        return 1 as i32;
    } else if c as i32 >= 0xc2 as i32 {
        if (c as i32) < 0xe0 as i32 {
            if n >= 2 as i32 as u64 {
                if (*s.offset(1 as i32 as isize) as i32 ^ 0x80 as i32) < 0x40 as i32 {
                    *puc = ((c as i32 & 0x1f as i32) as u32) << 6 as i32
                        | (*s.offset(1 as i32 as isize) as i32 ^ 0x80 as i32) as u32;
                    return 2 as i32;
                }
            } else {
                *puc = 0xfffd as i32 as ucs4_t;
                return -(2 as i32);
            }
        } else if (c as i32) < 0xf0 as i32 {
            if n >= 2 as i32 as u64 {
                if (*s.offset(1 as i32 as isize) as i32 ^ 0x80 as i32) < 0x40 as i32
                    && (c as i32 >= 0xe1 as i32
                        || *s.offset(1 as i32 as isize) as i32 >= 0xa0 as i32)
                    && (c as i32 != 0xed as i32
                        || (*s.offset(1 as i32 as isize) as i32) < 0xa0 as i32)
                {
                    if n >= 3 as i32 as u64 {
                        if (*s.offset(2 as i32 as isize) as i32 ^ 0x80 as i32)
                            < 0x40 as i32
                        {
                            *puc = ((c as i32 & 0xf as i32) as u32) << 12 as i32
                                | ((*s.offset(1 as i32 as isize) as i32 ^ 0x80 as i32)
                                    as u32) << 6 as i32
                                | (*s.offset(2 as i32 as isize) as i32 ^ 0x80 as i32)
                                    as u32;
                            return 3 as i32;
                        }
                    } else {
                        *puc = 0xfffd as i32 as ucs4_t;
                        return -(2 as i32);
                    }
                }
            } else {
                *puc = 0xfffd as i32 as ucs4_t;
                return -(2 as i32);
            }
        } else if (c as i32) < 0xf8 as i32 {
            if n >= 2 as i32 as u64 {
                if (*s.offset(1 as i32 as isize) as i32 ^ 0x80 as i32) < 0x40 as i32
                    && (c as i32 >= 0xf1 as i32
                        || *s.offset(1 as i32 as isize) as i32 >= 0x90 as i32)
                    && ((c as i32) < 0xf4 as i32
                        || c as i32 == 0xf4 as i32
                            && (*s.offset(1 as i32 as isize) as i32) < 0x90 as i32)
                {
                    if n >= 3 as i32 as u64 {
                        if (*s.offset(2 as i32 as isize) as i32 ^ 0x80 as i32)
                            < 0x40 as i32
                        {
                            if n >= 4 as i32 as u64 {
                                if (*s.offset(3 as i32 as isize) as i32 ^ 0x80 as i32)
                                    < 0x40 as i32
                                {
                                    *puc = ((c as i32 & 0x7 as i32) as u32) << 18 as i32
                                        | ((*s.offset(1 as i32 as isize) as i32 ^ 0x80 as i32)
                                            as u32) << 12 as i32
                                        | ((*s.offset(2 as i32 as isize) as i32 ^ 0x80 as i32)
                                            as u32) << 6 as i32
                                        | (*s.offset(3 as i32 as isize) as i32 ^ 0x80 as i32)
                                            as u32;
                                    return 4 as i32;
                                }
                            } else {
                                *puc = 0xfffd as i32 as ucs4_t;
                                return -(2 as i32);
                            }
                        }
                    } else {
                        *puc = 0xfffd as i32 as ucs4_t;
                        return -(2 as i32);
                    }
                }
            } else {
                *puc = 0xfffd as i32 as ucs4_t;
                return -(2 as i32);
            }
        }
    }
    *puc = 0xfffd as i32 as ucs4_t;
    return -(1 as i32);
}