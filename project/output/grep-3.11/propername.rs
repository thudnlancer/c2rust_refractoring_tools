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
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn __ctype_get_mb_cur_max() -> size_t;
    fn abort() -> !;
    fn rpl_free(ptr: *mut libc::c_void);
    fn mbsstr(haystack: *const i8, needle: *const i8) -> *mut i8;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn strlen(_: *const i8) -> u64;
    fn trim2(_: *const i8, _: i32) -> *mut i8;
    fn mbsinit(__ps: *const mbstate_t) -> i32;
    fn iswalnum(__wc: wint_t) -> i32;
    static is_basic_table: [u32; 0];
    fn rpl_mbrtowc(
        pwc: *mut wchar_t,
        s: *const i8,
        n: size_t,
        ps: *mut mbstate_t,
    ) -> size_t;
    fn strnlen1(string: *const i8, maxlen: size_t) -> size_t;
    fn locale_charset() -> *const i8;
    fn c_strcasecmp(s1: *const i8, s2: *const i8) -> i32;
    fn xstr_iconv(
        src: *const i8,
        from_codeset: *const i8,
        to_codeset: *const i8,
    ) -> *mut i8;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xnmalloc(n: size_t, s: size_t) -> *mut libc::c_void;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
}
pub type size_t = u64;
pub const _ISalnum: C2RustUnnamed_0 = 8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbchar {
    pub ptr: *const i8,
    pub bytes: size_t,
    pub wc_valid: bool,
    pub wc: wchar_t,
    pub buf: [i8; 24],
}
pub type wchar_t = i32;
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
pub type mbchar_t = mbchar;
pub type C2RustUnnamed_0 = u32;
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
pub const _ISupper: C2RustUnnamed_0 = 256;
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
        current_block = 11675860686847285950;
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
                144 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 48],
                    &[i8; 48],
                >(b"void mbuiter_multi_next(struct mbuiter_multi *)\0"))
                    .as_ptr(),
            );
        }
        'c_786: {
            if mbsinit(&mut (*iter).state) != 0 {} else {
                __assert_fail(
                    b"mbsinit (&iter->state)\0" as *const u8 as *const i8,
                    b"./mbuiter.h\0" as *const u8 as *const i8,
                    144 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 48],
                        &[i8; 48],
                    >(b"void mbuiter_multi_next(struct mbuiter_multi *)\0"))
                        .as_ptr(),
                );
            }
        };
        (*iter).in_shift = 1 as i32 != 0;
        current_block = 11675860686847285950;
    }
    match current_block {
        11675860686847285950 => {
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
                            172 as i32 as u32,
                            (*::core::mem::transmute::<
                                &[u8; 48],
                                &[i8; 48],
                            >(b"void mbuiter_multi_next(struct mbuiter_multi *)\0"))
                                .as_ptr(),
                        );
                    }
                    'c_610: {
                        if *(*iter).cur.ptr as i32 == '\0' as i32 {} else {
                            __assert_fail(
                                b"*iter->cur.ptr == '\\0'\0" as *const u8 as *const i8,
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
                    if (*iter).cur.wc == 0 as i32 {} else {
                        __assert_fail(
                            b"iter->cur.wc == 0\0" as *const u8 as *const i8,
                            b"./mbuiter.h\0" as *const u8 as *const i8,
                            173 as i32 as u32,
                            (*::core::mem::transmute::<
                                &[u8; 48],
                                &[i8; 48],
                            >(b"void mbuiter_multi_next(struct mbuiter_multi *)\0"))
                                .as_ptr(),
                        );
                    }
                    'c_563: {
                        if (*iter).cur.wc == 0 as i32 {} else {
                            __assert_fail(
                                b"iter->cur.wc == 0\0" as *const u8 as *const i8,
                                b"./mbuiter.h\0" as *const u8 as *const i8,
                                173 as i32 as u32,
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
unsafe extern "C" fn mbsstr_trimmed_wordbounded(
    mut string: *const i8,
    mut sub: *const i8,
) -> bool {
    let mut tsub: *mut i8 = trim2(sub, 2 as i32);
    let mut found: bool = 0 as i32 != 0;
    while *string as i32 != '\0' as i32 {
        let mut tsub_in_string: *const i8 = mbsstr(string, tsub);
        if tsub_in_string.is_null() {
            break;
        }
        if __ctype_get_mb_cur_max() > 1 as i32 as u64 {
            let mut string_iter: mbui_iterator_t = mbui_iterator_t {
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
            let mut word_boundary_before: bool = false;
            let mut word_boundary_after: bool = false;
            string_iter.cur.ptr = string;
            string_iter.in_shift = 0 as i32 != 0;
            memset(
                &mut string_iter.state as *mut mbstate_t as *mut libc::c_void,
                '\0' as i32,
                ::core::mem::size_of::<mbstate_t>() as u64,
            );
            string_iter.next_done = 0 as i32 != 0;
            word_boundary_before = 1 as i32 != 0;
            if string_iter.cur.ptr < tsub_in_string {
                let mut last_char_before_tsub: mbchar_t = mbchar {
                    ptr: 0 as *const i8,
                    bytes: 0,
                    wc_valid: false,
                    wc: 0,
                    buf: [0; 24],
                };
                loop {
                    mbuiter_multi_next(&mut string_iter);
                    if !(string_iter.cur.wc_valid as i32 != 0
                        && string_iter.cur.wc == 0 as i32) as i32 == 0
                    {
                        abort();
                    }
                    last_char_before_tsub = string_iter.cur;
                    string_iter.cur.ptr = (string_iter.cur.ptr)
                        .offset(string_iter.cur.bytes as isize);
                    string_iter.next_done = 0 as i32 != 0;
                    if !(string_iter.cur.ptr < tsub_in_string) {
                        break;
                    }
                }
                if last_char_before_tsub.wc_valid as i32 != 0
                    && iswalnum(last_char_before_tsub.wc as wint_t) != 0
                {
                    word_boundary_before = 0 as i32 != 0;
                }
            }
            string_iter.cur.ptr = tsub_in_string;
            string_iter.in_shift = 0 as i32 != 0;
            memset(
                &mut string_iter.state as *mut mbstate_t as *mut libc::c_void,
                '\0' as i32,
                ::core::mem::size_of::<mbstate_t>() as u64,
            );
            string_iter.next_done = 0 as i32 != 0;
            let mut tsub_iter: mbui_iterator_t = mbui_iterator_t {
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
            tsub_iter.cur.ptr = tsub;
            tsub_iter.in_shift = 0 as i32 != 0;
            memset(
                &mut tsub_iter.state as *mut mbstate_t as *mut libc::c_void,
                '\0' as i32,
                ::core::mem::size_of::<mbstate_t>() as u64,
            );
            tsub_iter.next_done = 0 as i32 != 0;
            loop {
                mbuiter_multi_next(&mut tsub_iter);
                if !(!(tsub_iter.cur.wc_valid as i32 != 0
                    && tsub_iter.cur.wc == 0 as i32) as i32 != 0)
                {
                    break;
                }
                mbuiter_multi_next(&mut string_iter);
                if !(string_iter.cur.wc_valid as i32 != 0
                    && string_iter.cur.wc == 0 as i32) as i32 == 0
                {
                    abort();
                }
                string_iter.cur.ptr = (string_iter.cur.ptr)
                    .offset(string_iter.cur.bytes as isize);
                string_iter.next_done = 0 as i32 != 0;
                tsub_iter.cur.ptr = (tsub_iter.cur.ptr)
                    .offset(tsub_iter.cur.bytes as isize);
                tsub_iter.next_done = 0 as i32 != 0;
            }
            word_boundary_after = 1 as i32 != 0;
            mbuiter_multi_next(&mut string_iter);
            if !(string_iter.cur.wc_valid as i32 != 0 && string_iter.cur.wc == 0 as i32)
                as i32 != 0
            {
                let mut first_char_after_tsub: mbchar_t = string_iter.cur;
                if first_char_after_tsub.wc_valid as i32 != 0
                    && iswalnum(first_char_after_tsub.wc as wint_t) != 0
                {
                    word_boundary_after = 0 as i32 != 0;
                }
            }
            if word_boundary_before as i32 != 0 && word_boundary_after as i32 != 0 {
                found = 1 as i32 != 0;
                break;
            } else {
                string_iter.cur.ptr = tsub_in_string;
                string_iter.in_shift = 0 as i32 != 0;
                memset(
                    &mut string_iter.state as *mut mbstate_t as *mut libc::c_void,
                    '\0' as i32,
                    ::core::mem::size_of::<mbstate_t>() as u64,
                );
                string_iter.next_done = 0 as i32 != 0;
                mbuiter_multi_next(&mut string_iter);
                if !(string_iter.cur.wc_valid as i32 != 0
                    && string_iter.cur.wc == 0 as i32) as i32 == 0
                {
                    break;
                }
                string = tsub_in_string.offset(string_iter.cur.bytes as isize);
            }
        } else {
            let mut word_boundary_before_0: bool = false;
            let mut p: *const i8 = 0 as *const i8;
            let mut word_boundary_after_0: bool = false;
            word_boundary_before_0 = 1 as i32 != 0;
            if string < tsub_in_string {
                if *(*__ctype_b_loc())
                    .offset(
                        *tsub_in_string.offset(-(1 as i32) as isize) as u8 as i32
                            as isize,
                    ) as i32 & _ISalnum as i32 as libc::c_ushort as i32 != 0
                {
                    word_boundary_before_0 = 0 as i32 != 0;
                }
            }
            p = tsub_in_string.offset(strlen(tsub) as isize);
            word_boundary_after_0 = 1 as i32 != 0;
            if *p as i32 != '\0' as i32 {
                if *(*__ctype_b_loc()).offset(*p as u8 as i32 as isize) as i32
                    & _ISalnum as i32 as libc::c_ushort as i32 != 0
                {
                    word_boundary_after_0 = 0 as i32 != 0;
                }
            }
            if word_boundary_before_0 as i32 != 0 && word_boundary_after_0 as i32 != 0 {
                found = 1 as i32 != 0;
                break;
            } else {
                if *tsub_in_string as i32 == '\0' as i32 {
                    break;
                }
                string = tsub_in_string.offset(1 as i32 as isize);
            }
        }
    }
    rpl_free(tsub as *mut libc::c_void);
    return found;
}
#[no_mangle]
pub unsafe extern "C" fn proper_name(mut name: *const i8) -> *const i8 {
    let mut translation: *const i8 = dcgettext(0 as *const i8, name, 5 as i32);
    if translation != name {
        if mbsstr_trimmed_wordbounded(translation, name) {
            return translation
        } else {
            let mut result: *mut i8 = (if ::core::mem::size_of::<i8>() as u64
                == 1 as i32 as u64
            {
                xmalloc(
                    (strlen(translation))
                        .wrapping_add(2 as i32 as u64)
                        .wrapping_add(strlen(name))
                        .wrapping_add(1 as i32 as u64)
                        .wrapping_add(1 as i32 as u64),
                )
            } else {
                xnmalloc(
                    (strlen(translation))
                        .wrapping_add(2 as i32 as u64)
                        .wrapping_add(strlen(name))
                        .wrapping_add(1 as i32 as u64)
                        .wrapping_add(1 as i32 as u64),
                    ::core::mem::size_of::<i8>() as u64,
                )
            }) as *mut i8;
            sprintf(result, b"%s (%s)\0" as *const u8 as *const i8, translation, name);
            return result;
        }
    } else {
        return name
    };
}
#[no_mangle]
pub unsafe extern "C" fn proper_name_utf8(
    mut name_ascii: *const i8,
    mut name_utf8: *const i8,
) -> *const i8 {
    let mut translation: *const i8 = dcgettext(0 as *const i8, name_ascii, 5 as i32);
    let mut locale_code: *const i8 = locale_charset();
    let mut alloc_name_converted: *mut i8 = 0 as *mut i8;
    let mut alloc_name_converted_translit: *mut i8 = 0 as *mut i8;
    let mut name_converted: *const i8 = 0 as *const i8;
    let mut name_converted_translit: *const i8 = 0 as *const i8;
    let mut name: *const i8 = 0 as *const i8;
    if c_strcasecmp(locale_code, b"UTF-8\0" as *const u8 as *const i8) != 0 as i32 {
        alloc_name_converted = xstr_iconv(
            name_utf8,
            b"UTF-8\0" as *const u8 as *const i8,
            locale_code,
        );
        name_converted = alloc_name_converted;
        let mut converted_translit: *mut i8 = 0 as *mut i8;
        let mut len: size_t = strlen(locale_code);
        let mut locale_code_translit: *mut i8 = (if ::core::mem::size_of::<i8>() as u64
            == 1 as i32 as u64
        {
            xmalloc(len.wrapping_add(10 as i32 as u64).wrapping_add(1 as i32 as u64))
        } else {
            xnmalloc(
                len.wrapping_add(10 as i32 as u64).wrapping_add(1 as i32 as u64),
                ::core::mem::size_of::<i8>() as u64,
            )
        }) as *mut i8;
        memcpy(
            locale_code_translit as *mut libc::c_void,
            locale_code as *const libc::c_void,
            len,
        );
        memcpy(
            locale_code_translit.offset(len as isize) as *mut libc::c_void,
            b"//TRANSLIT\0" as *const u8 as *const i8 as *const libc::c_void,
            (10 as i32 + 1 as i32) as u64,
        );
        converted_translit = xstr_iconv(
            name_utf8,
            b"UTF-8\0" as *const u8 as *const i8,
            locale_code_translit,
        );
        rpl_free(locale_code_translit as *mut libc::c_void);
        if !converted_translit.is_null() {
            if !(strchr(converted_translit, '?' as i32)).is_null() {
                rpl_free(converted_translit as *mut libc::c_void);
            } else {
                alloc_name_converted_translit = converted_translit;
                name_converted_translit = alloc_name_converted_translit;
            }
        }
    } else {
        name_converted = name_utf8;
        name_converted_translit = name_utf8;
    }
    name = if !name_converted.is_null() {
        name_converted
    } else if !name_converted_translit.is_null() {
        name_converted_translit
    } else {
        name_ascii
    };
    if strcmp(translation, name_ascii) != 0 as i32 {
        if mbsstr_trimmed_wordbounded(translation, name_ascii) as i32 != 0
            || !name_converted.is_null()
                && mbsstr_trimmed_wordbounded(translation, name_converted) as i32 != 0
            || !name_converted_translit.is_null()
                && mbsstr_trimmed_wordbounded(translation, name_converted_translit)
                    as i32 != 0
        {
            if !alloc_name_converted.is_null() {
                rpl_free(alloc_name_converted as *mut libc::c_void);
            }
            if !alloc_name_converted_translit.is_null() {
                rpl_free(alloc_name_converted_translit as *mut libc::c_void);
            }
            return translation;
        } else {
            let mut result: *mut i8 = (if ::core::mem::size_of::<i8>() as u64
                == 1 as i32 as u64
            {
                xmalloc(
                    (strlen(translation))
                        .wrapping_add(2 as i32 as u64)
                        .wrapping_add(strlen(name))
                        .wrapping_add(1 as i32 as u64)
                        .wrapping_add(1 as i32 as u64),
                )
            } else {
                xnmalloc(
                    (strlen(translation))
                        .wrapping_add(2 as i32 as u64)
                        .wrapping_add(strlen(name))
                        .wrapping_add(1 as i32 as u64)
                        .wrapping_add(1 as i32 as u64),
                    ::core::mem::size_of::<i8>() as u64,
                )
            }) as *mut i8;
            sprintf(result, b"%s (%s)\0" as *const u8 as *const i8, translation, name);
            if !alloc_name_converted.is_null() {
                rpl_free(alloc_name_converted as *mut libc::c_void);
            }
            if !alloc_name_converted_translit.is_null() {
                rpl_free(alloc_name_converted_translit as *mut libc::c_void);
            }
            return result;
        }
    } else {
        if !alloc_name_converted.is_null() && alloc_name_converted != name as *mut i8 {
            rpl_free(alloc_name_converted as *mut libc::c_void);
        }
        if !alloc_name_converted_translit.is_null()
            && alloc_name_converted_translit != name as *mut i8
        {
            rpl_free(alloc_name_converted_translit as *mut libc::c_void);
        }
        return name;
    };
}