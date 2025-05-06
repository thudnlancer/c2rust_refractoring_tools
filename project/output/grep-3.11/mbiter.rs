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
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
    fn mbsinit(__ps: *const mbstate_t) -> i32;
    static is_basic_table: [u32; 0];
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn rpl_mbrtowc(
        pwc: *mut wchar_t,
        s: *const i8,
        n: size_t,
        ps: *mut mbstate_t,
    ) -> size_t;
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
pub struct mbiter_multi {
    pub limit: *const i8,
    pub in_shift: bool,
    pub state: mbstate_t,
    pub next_done: bool,
    pub cur: mbchar,
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
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mbiter_multi_next(mut iter: *mut mbiter_multi) {
    let mut current_block: u64;
    if (*iter).next_done {
        return;
    }
    if (*iter).in_shift {
        current_block = 15727723544652228167;
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
        'c_1667: {
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
        current_block = 15727723544652228167;
    }
    match current_block {
        15727723544652228167 => {
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
                    'c_1506: {
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
                    'c_1459: {
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
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mbiter_multi_reloc(
    mut iter: *mut mbiter_multi,
    mut ptrdiff: ptrdiff_t,
) {
    (*iter).cur.ptr = ((*iter).cur.ptr).offset(ptrdiff as isize);
    (*iter).limit = ((*iter).limit).offset(ptrdiff as isize);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn mbiter_multi_copy(
    mut new_iter: *mut mbiter_multi,
    mut old_iter: *const mbiter_multi,
) {
    (*new_iter).limit = (*old_iter).limit;
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
#[inline]
unsafe extern "C" fn is_basic(mut c: i8) -> bool {
    return *is_basic_table.as_ptr().offset((c as u8 as i32 >> 5 as i32) as isize)
        >> (c as u8 as i32 & 31 as i32) & 1 as i32 as u32 != 0;
}