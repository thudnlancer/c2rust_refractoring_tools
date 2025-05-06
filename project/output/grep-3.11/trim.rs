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
    fn __ctype_get_mb_cur_max() -> size_t;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: u64,
    ) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn strdup(_: *const i8) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn mbsinit(__ps: *const mbstate_t) -> i32;
    fn iswspace(__wc: wint_t) -> i32;
    fn rpl_mbrtowc(
        pwc: *mut wchar_t,
        s: *const i8,
        n: size_t,
        ps: *mut mbstate_t,
    ) -> size_t;
    static is_basic_table: [u32; 0];
    fn xalloc_die();
}
pub type size_t = u64;
pub type wchar_t = i32;
pub const _ISspace: C2RustUnnamed_0 = 8192;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbchar {
    pub ptr: *const i8,
    pub bytes: size_t,
    pub wc_valid: bool,
    pub wc: wchar_t,
    pub buf: [i8; 24],
}
pub type mbi_iterator_t = mbiter_multi;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbiter_multi {
    pub limit: *const i8,
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
unsafe extern "C" fn mbiter_multi_next(mut iter: *mut mbiter_multi) {
    let mut current_block: u64;
    if (*iter).next_done {
        return;
    }
    if (*iter).in_shift {
        current_block = 16084692321815031027;
    } else if is_basic(*(*iter).cur.ptr) {
        (*iter).cur.bytes = 1 as i32 as size_t;
        (*iter).cur.wc = *(*iter).cur.ptr as wchar_t;
        (*iter).cur.wc_valid = 1 as i32 != 0;
        current_block = 15089075282327824602;
    } else {
        if mbsinit(&mut (*iter).state) != 0 {} else {
            __assert_fail(
                b"mbsinit (&iter->state)\0" as *const u8 as *const i8,
                b"./mbiter.h\0" as *const u8 as *const i8,
                136 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 46],
                    &[i8; 46],
                >(b"void mbiter_multi_next(struct mbiter_multi *)\0"))
                    .as_ptr(),
            );
        }
        'c_1962: {
            if mbsinit(&mut (*iter).state) != 0 {} else {
                __assert_fail(
                    b"mbsinit (&iter->state)\0" as *const u8 as *const i8,
                    b"./mbiter.h\0" as *const u8 as *const i8,
                    136 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 46],
                        &[i8; 46],
                    >(b"void mbiter_multi_next(struct mbiter_multi *)\0"))
                        .as_ptr(),
                );
            }
        };
        (*iter).in_shift = 1 as i32 != 0;
        current_block = 16084692321815031027;
    }
    match current_block {
        16084692321815031027 => {
            (*iter).cur.bytes = rpl_mbrtowc(
                &mut (*iter).cur.wc,
                (*iter).cur.ptr,
                ((*iter).limit).offset_from((*iter).cur.ptr) as i64 as size_t,
                &mut (*iter).state,
            );
            if (*iter).cur.bytes == -(1 as i32) as size_t {
                (*iter).cur.bytes = 1 as i32 as size_t;
                (*iter).cur.wc_valid = 0 as i32 != 0;
            } else if (*iter).cur.bytes == -(2 as i32) as size_t {
                (*iter).cur.bytes = ((*iter).limit).offset_from((*iter).cur.ptr) as i64
                    as size_t;
                (*iter).cur.wc_valid = 0 as i32 != 0;
            } else {
                if (*iter).cur.bytes == 0 as i32 as u64 {
                    (*iter).cur.bytes = 1 as i32 as size_t;
                    if *(*iter).cur.ptr as i32 == '\0' as i32 {} else {
                        __assert_fail(
                            b"*iter->cur.ptr == '\\0'\0" as *const u8 as *const i8,
                            b"./mbiter.h\0" as *const u8 as *const i8,
                            163 as i32 as u32,
                            (*::core::mem::transmute::<
                                &[u8; 46],
                                &[i8; 46],
                            >(b"void mbiter_multi_next(struct mbiter_multi *)\0"))
                                .as_ptr(),
                        );
                    }
                    'c_1794: {
                        if *(*iter).cur.ptr as i32 == '\0' as i32 {} else {
                            __assert_fail(
                                b"*iter->cur.ptr == '\\0'\0" as *const u8 as *const i8,
                                b"./mbiter.h\0" as *const u8 as *const i8,
                                163 as i32 as u32,
                                (*::core::mem::transmute::<
                                    &[u8; 46],
                                    &[i8; 46],
                                >(b"void mbiter_multi_next(struct mbiter_multi *)\0"))
                                    .as_ptr(),
                            );
                        }
                    };
                    if (*iter).cur.wc == 0 as i32 {} else {
                        __assert_fail(
                            b"iter->cur.wc == 0\0" as *const u8 as *const i8,
                            b"./mbiter.h\0" as *const u8 as *const i8,
                            164 as i32 as u32,
                            (*::core::mem::transmute::<
                                &[u8; 46],
                                &[i8; 46],
                            >(b"void mbiter_multi_next(struct mbiter_multi *)\0"))
                                .as_ptr(),
                        );
                    }
                    'c_1747: {
                        if (*iter).cur.wc == 0 as i32 {} else {
                            __assert_fail(
                                b"iter->cur.wc == 0\0" as *const u8 as *const i8,
                                b"./mbiter.h\0" as *const u8 as *const i8,
                                164 as i32 as u32,
                                (*::core::mem::transmute::<
                                    &[u8; 46],
                                    &[i8; 46],
                                >(b"void mbiter_multi_next(struct mbiter_multi *)\0"))
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
pub unsafe extern "C" fn trim2(mut s: *const i8, mut how: i32) -> *mut i8 {
    let mut d: *mut i8 = 0 as *mut i8;
    d = strdup(s);
    if d.is_null() {
        xalloc_die();
    }
    if __ctype_get_mb_cur_max() > 1 as i32 as u64 {
        let mut i: mbi_iterator_t = mbi_iterator_t {
            limit: 0 as *const i8,
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
        if how != 0 as i32 {
            i.cur.ptr = d;
            i.limit = (i.cur.ptr).offset(strlen(d) as isize);
            i.in_shift = 0 as i32 != 0;
            memset(
                &mut i.state as *mut mbstate_t as *mut libc::c_void,
                '\0' as i32,
                ::core::mem::size_of::<mbstate_t>() as u64,
            );
            i.next_done = 0 as i32 != 0;
            while i.cur.ptr < i.limit
                && {
                    mbiter_multi_next(&mut i);
                    1 as i32 != 0
                } && (i.cur.wc_valid as i32 != 0 && iswspace(i.cur.wc as wint_t) != 0)
            {
                i.cur.ptr = (i.cur.ptr).offset(i.cur.bytes as isize);
                i.next_done = 0 as i32 != 0;
            }
            memmove(
                d as *mut libc::c_void,
                i.cur.ptr as *const libc::c_void,
                (strlen(i.cur.ptr)).wrapping_add(1 as i32 as u64),
            );
        }
        if how != 1 as i32 {
            let mut start_of_spaces: *mut i8 = 0 as *mut i8;
            i.cur.ptr = d;
            i.limit = (i.cur.ptr).offset(strlen(d) as isize);
            i.in_shift = 0 as i32 != 0;
            memset(
                &mut i.state as *mut mbstate_t as *mut libc::c_void,
                '\0' as i32,
                ::core::mem::size_of::<mbstate_t>() as u64,
            );
            i.next_done = 0 as i32 != 0;
            while i.cur.ptr < i.limit
                && {
                    mbiter_multi_next(&mut i);
                    1 as i32 != 0
                }
            {
                if i.cur.wc_valid as i32 != 0 && iswspace(i.cur.wc as wint_t) != 0 {
                    if start_of_spaces.is_null() {
                        start_of_spaces = i.cur.ptr as *mut i8;
                    }
                } else {
                    start_of_spaces = 0 as *mut i8;
                }
                i.cur.ptr = (i.cur.ptr).offset(i.cur.bytes as isize);
                i.next_done = 0 as i32 != 0;
            }
            if !start_of_spaces.is_null() {
                *start_of_spaces = '\0' as i32 as i8;
            }
        }
    } else {
        let mut p: *mut i8 = 0 as *mut i8;
        if how != 0 as i32 {
            p = d;
            while *p as i32 != 0
                && *(*__ctype_b_loc()).offset(*p as u8 as i32 as isize) as i32
                    & _ISspace as i32 as libc::c_ushort as i32 != 0
            {
                p = p.offset(1);
                p;
            }
            memmove(
                d as *mut libc::c_void,
                p as *const libc::c_void,
                (strlen(p)).wrapping_add(1 as i32 as u64),
            );
        }
        if how != 1 as i32 {
            p = d.offset(strlen(d) as isize).offset(-(1 as i32 as isize));
            while p >= d
                && *(*__ctype_b_loc()).offset(*p as u8 as i32 as isize) as i32
                    & _ISspace as i32 as libc::c_ushort as i32 != 0
            {
                *p = '\0' as i32 as i8;
                p = p.offset(-1);
                p;
            }
        }
    }
    return d;
}