#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(label_break_value)]
extern "C" {
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: u64) -> i32;
    fn strlen(_: *const i8) -> u64;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    fn towlower(__wc: wint_t) -> wint_t;
    static is_basic_table: [u32; 0];
    fn rpl_mbrtowc(
        pwc: *mut wchar_t,
        s: *const i8,
        n: size_t,
        ps: *mut mbstate_t,
    ) -> size_t;
    fn strnlen1(string: *const i8, maxlen: size_t) -> size_t;
    fn __ctype_get_mb_cur_max() -> size_t;
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
    fn mbsinit(__ps: *const mbstate_t) -> i32;
}
pub type size_t = u64;
pub type wchar_t = i32;
pub type __int32_t = i32;
pub const _ISupper: C2RustUnnamed_0 = 256;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbchar {
    pub ptr: *const i8,
    pub bytes: size_t,
    pub wc_valid: bool,
    pub wc: wchar_t,
    pub buf: [i8; 24],
}
pub type mbui_iterator_t = mbuiter_multi;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbuiter_multi {
    pub in_shift: bool,
    pub state: mbstate_t,
    pub next_done: bool,
    pub cur: mbchar,
}
pub type __mbstate_t = mbstate_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbstate_t {
    pub __count: i32,
    pub __value: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __wch: u32,
    pub __wchb: [i8; 4],
}
pub type wint_t = u32;
pub type C2RustUnnamed_0 = u32;
pub const _ISalnum: C2RustUnnamed_0 = 8;
pub const _ISpunct: C2RustUnnamed_0 = 4;
pub const _IScntrl: C2RustUnnamed_0 = 2;
pub const _ISblank: C2RustUnnamed_0 = 1;
pub const _ISgraph: C2RustUnnamed_0 = 32768;
pub const _ISprint: C2RustUnnamed_0 = 16384;
pub const _ISspace: C2RustUnnamed_0 = 8192;
pub const _ISxdigit: C2RustUnnamed_0 = 4096;
pub const _ISdigit: C2RustUnnamed_0 = 2048;
pub const _ISalpha: C2RustUnnamed_0 = 1024;
pub const _ISlower: C2RustUnnamed_0 = 512;
#[inline]
unsafe extern "C" fn tolower(mut __c: i32) -> i32 {
    return if __c >= -(128 as i32) && __c < 256 as i32 {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[inline]
unsafe extern "C" fn is_basic(mut c: i8) -> bool {
    return *is_basic_table.as_ptr().offset((c as u8 as i32 >> 5 as i32) as isize)
        >> (c as u8 as i32 & 31 as i32) & 1 as i32 as u32 != 0;
}
#[inline]
unsafe extern "C" fn mbuiter_multi_next(mut iter: *mut mbuiter_multi) {
    let mut current_block: u64;
    if (*iter).next_done {
        return;
    }
    if (*iter).in_shift {
        current_block = 9615712722277884017;
    } else if is_basic(*(*iter).cur.ptr) {
        (*iter).cur.bytes = 1 as i32 as size_t;
        (*iter).cur.wc = *(*iter).cur.ptr as wchar_t;
        (*iter).cur.wc_valid = 1 as i32 != 0;
        current_block = 15089075282327824602;
    } else {
        if mbsinit(&mut (*iter).state) != 0 {} else {
            __assert_fail(
                b"mbsinit (&iter->state)\0" as *const u8 as *const i8,
                b"./mbuiter.h\0" as *const u8 as *const i8,
                143 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 48],
                    &[i8; 48],
                >(b"void mbuiter_multi_next(struct mbuiter_multi *)\0"))
                    .as_ptr(),
            );
        }
        'c_1106: {
            if mbsinit(&mut (*iter).state) != 0 {} else {
                __assert_fail(
                    b"mbsinit (&iter->state)\0" as *const u8 as *const i8,
                    b"./mbuiter.h\0" as *const u8 as *const i8,
                    143 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 48],
                        &[i8; 48],
                    >(b"void mbuiter_multi_next(struct mbuiter_multi *)\0"))
                        .as_ptr(),
                );
            }
        };
        (*iter).in_shift = 1 as i32 != 0;
        current_block = 9615712722277884017;
    }
    match current_block {
        9615712722277884017 => {
            (*iter).cur.bytes = rpl_mbrtowc(
                &mut (*iter).cur.wc,
                (*iter).cur.ptr,
                strnlen1((*iter).cur.ptr, __ctype_get_mb_cur_max()),
                &mut (*iter).state,
            );
            if (*iter).cur.bytes == -(1 as i32) as size_t {
                (*iter).cur.bytes = 1 as i32 as size_t;
                (*iter).cur.wc_valid = 0 as i32 != 0;
            } else if (*iter).cur.bytes == -(2 as i32) as size_t {
                (*iter).cur.bytes = strlen((*iter).cur.ptr);
                (*iter).cur.wc_valid = 0 as i32 != 0;
            } else {
                if (*iter).cur.bytes == 0 as i32 as u64 {
                    (*iter).cur.bytes = 1 as i32 as size_t;
                    if *(*iter).cur.ptr as i32 == '\0' as i32 {} else {
                        __assert_fail(
                            b"*iter->cur.ptr == '\\0'\0" as *const u8 as *const i8,
                            b"./mbuiter.h\0" as *const u8 as *const i8,
                            171 as i32 as u32,
                            (*::core::mem::transmute::<
                                &[u8; 48],
                                &[i8; 48],
                            >(b"void mbuiter_multi_next(struct mbuiter_multi *)\0"))
                                .as_ptr(),
                        );
                    }
                    'c_930: {
                        if *(*iter).cur.ptr as i32 == '\0' as i32 {} else {
                            __assert_fail(
                                b"*iter->cur.ptr == '\\0'\0" as *const u8 as *const i8,
                                b"./mbuiter.h\0" as *const u8 as *const i8,
                                171 as i32 as u32,
                                (*::core::mem::transmute::<
                                    &[u8; 48],
                                    &[i8; 48],
                                >(b"void mbuiter_multi_next(struct mbuiter_multi *)\0"))
                                    .as_ptr(),
                            );
                        }
                    };
                    if (*iter).cur.wc == 0 as i32 {} else {
                        __assert_fail(
                            b"iter->cur.wc == 0\0" as *const u8 as *const i8,
                            b"./mbuiter.h\0" as *const u8 as *const i8,
                            172 as i32 as u32,
                            (*::core::mem::transmute::<
                                &[u8; 48],
                                &[i8; 48],
                            >(b"void mbuiter_multi_next(struct mbuiter_multi *)\0"))
                                .as_ptr(),
                        );
                    }
                    'c_877: {
                        if (*iter).cur.wc == 0 as i32 {} else {
                            __assert_fail(
                                b"iter->cur.wc == 0\0" as *const u8 as *const i8,
                                b"./mbuiter.h\0" as *const u8 as *const i8,
                                172 as i32 as u32,
                                (*::core::mem::transmute::<
                                    &[u8; 48],
                                    &[i8; 48],
                                >(b"void mbuiter_multi_next(struct mbuiter_multi *)\0"))
                                    .as_ptr(),
                            );
                        }
                    };
                }
                (*iter).cur.wc_valid = 1 as i32 != 0;
                if mbsinit(&mut (*iter).state) != 0 {
                    (*iter).in_shift = 0 as i32 != 0;
                }
            }
        }
        _ => {}
    }
    (*iter).next_done = 1 as i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn mbscasecmp(mut s1: *const i8, mut s2: *const i8) -> i32 {
    if s1 == s2 {
        return 0 as i32;
    }
    if __ctype_get_mb_cur_max() > 1 as i32 as u64 {
        let mut iter1: mbui_iterator_t = mbui_iterator_t {
            in_shift: false,
            state: mbstate_t {
                __count: 0,
                __value: C2RustUnnamed { __wch: 0 },
            },
            next_done: false,
            cur: mbchar {
                ptr: 0 as *const i8,
                bytes: 0,
                wc_valid: false,
                wc: 0,
                buf: [0; 24],
            },
        };
        let mut iter2: mbui_iterator_t = mbui_iterator_t {
            in_shift: false,
            state: mbstate_t {
                __count: 0,
                __value: C2RustUnnamed { __wch: 0 },
            },
            next_done: false,
            cur: mbchar {
                ptr: 0 as *const i8,
                bytes: 0,
                wc_valid: false,
                wc: 0,
                buf: [0; 24],
            },
        };
        iter1.cur.ptr = s1;
        iter1.in_shift = 0 as i32 != 0;
        memset(
            &mut iter1.state as *mut mbstate_t as *mut libc::c_void,
            '\0' as i32,
            ::core::mem::size_of::<mbstate_t>() as u64,
        );
        iter1.next_done = 0 as i32 != 0;
        iter2.cur.ptr = s2;
        iter2.in_shift = 0 as i32 != 0;
        memset(
            &mut iter2.state as *mut mbstate_t as *mut libc::c_void,
            '\0' as i32,
            ::core::mem::size_of::<mbstate_t>() as u64,
        );
        iter2.next_done = 0 as i32 != 0;
        loop {
            mbuiter_multi_next(&mut iter1);
            if !(!(iter1.cur.wc_valid as i32 != 0 && iter1.cur.wc == 0 as i32) as i32
                != 0
                && {
                    mbuiter_multi_next(&mut iter2);
                    !(iter2.cur.wc_valid as i32 != 0 && iter2.cur.wc == 0 as i32) as i32
                        != 0
                })
            {
                break;
            }
            let mut cmp: i32 = if iter1.cur.wc_valid as i32 != 0 {
                if iter2.cur.wc_valid as i32 != 0 {
                    towlower(iter1.cur.wc as wint_t) as i32
                        - towlower(iter2.cur.wc as wint_t) as i32
                } else {
                    -(1 as i32)
                }
            } else if iter2.cur.wc_valid as i32 != 0 {
                1 as i32
            } else if iter1.cur.bytes == iter2.cur.bytes {
                memcmp(
                    iter1.cur.ptr as *const libc::c_void,
                    iter2.cur.ptr as *const libc::c_void,
                    iter1.cur.bytes,
                )
            } else if iter1.cur.bytes < iter2.cur.bytes {
                if memcmp(
                    iter1.cur.ptr as *const libc::c_void,
                    iter2.cur.ptr as *const libc::c_void,
                    iter1.cur.bytes,
                ) > 0 as i32
                {
                    1 as i32
                } else {
                    -(1 as i32)
                }
            } else if memcmp(
                iter1.cur.ptr as *const libc::c_void,
                iter2.cur.ptr as *const libc::c_void,
                iter2.cur.bytes,
            ) >= 0 as i32
            {
                1 as i32
            } else {
                -(1 as i32)
            };
            if cmp != 0 as i32 {
                return cmp;
            }
            iter1.cur.ptr = (iter1.cur.ptr).offset(iter1.cur.bytes as isize);
            iter1.next_done = 0 as i32 != 0;
            iter2.cur.ptr = (iter2.cur.ptr).offset(iter2.cur.bytes as isize);
            iter2.next_done = 0 as i32 != 0;
        }
        mbuiter_multi_next(&mut iter1);
        if !(iter1.cur.wc_valid as i32 != 0 && iter1.cur.wc == 0 as i32) as i32 != 0 {
            return 1 as i32;
        }
        mbuiter_multi_next(&mut iter2);
        if !(iter2.cur.wc_valid as i32 != 0 && iter2.cur.wc == 0 as i32) as i32 != 0 {
            return -(1 as i32);
        }
        return 0 as i32;
    } else {
        let mut p1: *const u8 = s1 as *const u8;
        let mut p2: *const u8 = s2 as *const u8;
        let mut c1: u8 = 0;
        let mut c2: u8 = 0;
        loop {
            c1 = (if *(*__ctype_b_loc()).offset(*p1 as i32 as isize) as i32
                & _ISupper as i32 as libc::c_ushort as i32 != 0
            {
                ({
                    let mut __res: i32 = 0;
                    if ::core::mem::size_of::<u8>() as u64 > 1 as i32 as u64 {
                        if 0 != 0 {
                            let mut __c: i32 = *p1 as i32;
                            __res = if __c < -(128 as i32) || __c > 255 as i32 {
                                __c
                            } else {
                                *(*__ctype_tolower_loc()).offset(__c as isize)
                            };
                        } else {
                            __res = tolower(*p1 as i32);
                        }
                    } else {
                        __res = *(*__ctype_tolower_loc()).offset(*p1 as i32 as isize);
                    }
                    __res
                })
            } else {
                *p1 as i32
            }) as u8;
            c2 = (if *(*__ctype_b_loc()).offset(*p2 as i32 as isize) as i32
                & _ISupper as i32 as libc::c_ushort as i32 != 0
            {
                ({
                    let mut __res: i32 = 0;
                    if ::core::mem::size_of::<u8>() as u64 > 1 as i32 as u64 {
                        if 0 != 0 {
                            let mut __c: i32 = *p2 as i32;
                            __res = if __c < -(128 as i32) || __c > 255 as i32 {
                                __c
                            } else {
                                *(*__ctype_tolower_loc()).offset(__c as isize)
                            };
                        } else {
                            __res = tolower(*p2 as i32);
                        }
                    } else {
                        __res = *(*__ctype_tolower_loc()).offset(*p2 as i32 as isize);
                    }
                    __res
                })
            } else {
                *p2 as i32
            }) as u8;
            if c1 as i32 == '\0' as i32 {
                break;
            }
            p1 = p1.offset(1);
            p1;
            p2 = p2.offset(1);
            p2;
            if !(c1 as i32 == c2 as i32) {
                break;
            }
        }
        if 127 as i32 * 2 as i32 + 1 as i32 <= 2147483647 as i32 {
            return c1 as i32 - c2 as i32
        } else {
            return (c1 as i32 > c2 as i32) as i32 - ((c1 as i32) < c2 as i32) as i32
        }
    };
}