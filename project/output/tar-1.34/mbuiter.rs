#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(label_break_value, linkage)]
extern "C" {
    fn __ctype_get_mb_cur_max() -> size_t;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
    fn strnlen1(string: *const i8, maxlen: size_t) -> size_t;
    static is_basic_table: [u32; 0];
    fn mbsinit(__ps: *const mbstate_t) -> i32;
    fn rpl_mbrtowc(
        pwc: *mut wchar_t,
        s: *const i8,
        n: size_t,
        ps: *mut mbstate_t,
    ) -> size_t;
    fn strlen(_: *const i8) -> u64;
}
pub type ptrdiff_t = i64;
pub type size_t = u64;
pub type wchar_t = i32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __mbstate_t {
    pub __count: i32,
    pub __value: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __wch: u32,
    pub __wchb: [i8; 4],
}
pub type mbstate_t = __mbstate_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbchar {
    pub ptr: *const i8,
    pub bytes: size_t,
    pub wc_valid: bool,
    pub wc: wchar_t,
    pub buf: [i8; 24],
}
pub type mbchar_t = mbchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbuiter_multi {
    pub in_shift: bool,
    pub state: mbstate_t,
    pub next_done: bool,
    pub cur: mbchar,
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mbuiter_multi_copy(
    mut new_iter: *mut mbuiter_multi,
    mut old_iter: *const mbuiter_multi,
) {
    (*new_iter).in_shift = (*old_iter).in_shift;
    if (*new_iter).in_shift {
        memcpy(
            &mut (*new_iter).state as *mut mbstate_t as *mut libc::c_void,
            &(*old_iter).state as *const mbstate_t as *const libc::c_void,
            ::core::mem::size_of::<mbstate_t>() as u64,
        );
    } else {
        memset(
            &mut (*new_iter).state as *mut mbstate_t as *mut libc::c_void,
            0 as i32,
            ::core::mem::size_of::<mbstate_t>() as u64,
        );
    }
    (*new_iter).next_done = (*old_iter).next_done;
    mb_copy(&mut (*new_iter).cur, &(*old_iter).cur);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mbuiter_multi_reloc(
    mut iter: *mut mbuiter_multi,
    mut ptrdiff: ptrdiff_t,
) {
    (*iter).cur.ptr = ((*iter).cur.ptr).offset(ptrdiff as isize);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mbuiter_multi_next(mut iter: *mut mbuiter_multi) {
    let mut current_block: u64;
    if (*iter).next_done {
        return;
    }
    if (*iter).in_shift {
        current_block = 8828581542860656313;
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
        'c_2816: {
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
        current_block = 8828581542860656313;
    }
    match current_block {
        8828581542860656313 => {
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
                    'c_2653: {
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
                    'c_2606: {
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
#[inline]
unsafe extern "C" fn is_basic(mut c: i8) -> bool {
    return *is_basic_table.as_ptr().offset((c as u8 as i32 >> 5 as i32) as isize)
        >> (c as u8 as i32 & 31 as i32) & 1 as i32 as u32 != 0;
}
#[inline]
unsafe extern "C" fn mb_copy(mut new_mbc: *mut mbchar_t, mut old_mbc: *const mbchar_t) {
    if (*old_mbc).ptr
        == &*((*old_mbc).buf).as_ptr().offset(0 as i32 as isize) as *const i8
    {
        memcpy(
            &mut *((*new_mbc).buf).as_mut_ptr().offset(0 as i32 as isize) as *mut i8
                as *mut libc::c_void,
            &*((*old_mbc).buf).as_ptr().offset(0 as i32 as isize) as *const i8
                as *const libc::c_void,
            (*old_mbc).bytes,
        );
        (*new_mbc).ptr = &mut *((*new_mbc).buf).as_mut_ptr().offset(0 as i32 as isize)
            as *mut i8;
    } else {
        (*new_mbc).ptr = (*old_mbc).ptr;
    }
    (*new_mbc).bytes = (*old_mbc).bytes;
    (*new_mbc).wc_valid = (*old_mbc).wc_valid;
    if (*new_mbc).wc_valid {
        (*new_mbc).wc = (*old_mbc).wc;
    }
}