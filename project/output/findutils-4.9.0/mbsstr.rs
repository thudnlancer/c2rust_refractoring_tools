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
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: u64) -> i32;
    fn strlen(_: *const i8) -> u64;
    fn strnlen(__string: *const i8, __maxlen: size_t) -> size_t;
    fn mbslen(string: *const i8) -> size_t;
    fn __ctype_get_mb_cur_max() -> size_t;
    fn abort() -> !;
    fn mmalloca(n: size_t) -> *mut libc::c_void;
    fn freea(p: *mut libc::c_void);
    fn mbsinit(__ps: *const mbstate_t) -> i32;
    static is_basic_table: [u32; 0];
    fn rpl_mbrtowc(
        pwc: *mut wchar_t,
        s: *const i8,
        n: size_t,
        ps: *mut mbstate_t,
    ) -> size_t;
    fn strnlen1(string: *const i8, maxlen: size_t) -> size_t;
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
}
pub type size_t = u64;
pub type wchar_t = i32;
pub type uintptr_t = u64;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_0 {
    sa_alignment_max = 16,
    sa_alignment_longdouble = 16,
    sa_alignment_longlong = 8,
    sa_alignment_double = 8,
    sa_alignment_long = 8,
}
impl C2RustUnnamed_0 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_0::sa_alignment_max => 16,
            C2RustUnnamed_0::sa_alignment_longdouble => 16,
            C2RustUnnamed_0::sa_alignment_longlong => 8,
            C2RustUnnamed_0::sa_alignment_double => 8,
            C2RustUnnamed_0::sa_alignment_long => 8,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_0 {
        match value {
            16 => C2RustUnnamed_0::sa_alignment_max,
            16 => C2RustUnnamed_0::sa_alignment_longdouble,
            8 => C2RustUnnamed_0::sa_alignment_longlong,
            8 => C2RustUnnamed_0::sa_alignment_double,
            8 => C2RustUnnamed_0::sa_alignment_long,
        }
    }
}
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
pub type mbchar_t = mbchar;
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
        current_block = 13748428005892234741;
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
        'c_1466: {
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
        current_block = 13748428005892234741;
    }
    match current_block {
        13748428005892234741 => {
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
                    'c_1292: {
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
                    'c_1239: {
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
unsafe extern "C" fn knuth_morris_pratt(
    mut haystack: *const u8,
    mut needle: *const u8,
    mut needle_len: size_t,
    mut resultp: *mut *const u8,
) -> bool {
    let mut m: size_t = needle_len;
    let mut table: *mut size_t = (if ::core::mem::size_of::<size_t>() as u64
        != 0 as i32 as u64
        && (if (9223372036854775807 as i64 as u64) < 18446744073709551615 as u64 {
            9223372036854775807 as i64 as u64
        } else {
            (18446744073709551615 as u64).wrapping_sub(1 as i32 as u64)
        })
            .wrapping_div(::core::mem::size_of::<size_t>() as u64) < m
    {
        0 as *mut libc::c_void
    } else if m.wrapping_mul(::core::mem::size_of::<size_t>() as u64)
        < (4032 as i32
            - (2 as i32 * C2RustUnnamed_0::sa_alignment_max as i32 - 1 as i32)) as u64
    {
        let mut fresh0 = ::std::vec::from_elem(
            0,
            m
                .wrapping_mul(::core::mem::size_of::<size_t>() as u64)
                .wrapping_add(
                    (2 as i32 * C2RustUnnamed_0::sa_alignment_max as i32) as u64,
                )
                .wrapping_sub(1 as i32 as u64) as usize,
        );
        ((fresh0.as_mut_ptr() as *mut i8 as uintptr_t)
            .wrapping_add(
                (2 as i32 * C2RustUnnamed_0::sa_alignment_max as i32 - 1 as i32) as u64,
            )
            & !((2 as i32 * C2RustUnnamed_0::sa_alignment_max as i32 - 1 as i32)
                as uintptr_t)) as *mut libc::c_void
    } else {
        mmalloca(m.wrapping_mul(::core::mem::size_of::<size_t>() as u64))
    }) as *mut size_t;
    if table.is_null() {
        return 0 as i32 != 0;
    }
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    *table.offset(1 as i32 as isize) = 1 as i32 as size_t;
    j = 0 as i32 as size_t;
    i = 2 as i32 as size_t;
    while i < m {
        let mut b: u8 = *needle.offset(i.wrapping_sub(1 as i32 as u64) as isize);
        loop {
            if b as i32 == *needle.offset(j as isize) as i32 {
                j = j.wrapping_add(1);
                *table.offset(i as isize) = i.wrapping_sub(j);
                break;
            } else if j == 0 as i32 as u64 {
                *table.offset(i as isize) = i;
                break;
            } else {
                j = j.wrapping_sub(*table.offset(j as isize));
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    let mut j_0: size_t = 0;
    let mut rhaystack: *const u8 = 0 as *const u8;
    let mut phaystack: *const u8 = 0 as *const u8;
    *resultp = 0 as *const u8;
    j_0 = 0 as i32 as size_t;
    rhaystack = haystack;
    phaystack = haystack;
    while *phaystack as i32 != 0 as i32 {
        if *needle.offset(j_0 as isize) as i32 == *phaystack as i32 {
            j_0 = j_0.wrapping_add(1);
            j_0;
            phaystack = phaystack.offset(1);
            phaystack;
            if !(j_0 == m) {
                continue;
            }
            *resultp = rhaystack;
            break;
        } else if j_0 > 0 as i32 as u64 {
            rhaystack = rhaystack.offset(*table.offset(j_0 as isize) as isize);
            j_0 = (j_0 as u64).wrapping_sub(*table.offset(j_0 as isize)) as size_t
                as size_t;
        } else {
            rhaystack = rhaystack.offset(1);
            rhaystack;
            phaystack = phaystack.offset(1);
            phaystack;
        }
    }
    freea(table as *mut libc::c_void);
    return 1 as i32 != 0;
}
unsafe extern "C" fn knuth_morris_pratt_multibyte(
    mut haystack: *const i8,
    mut needle: *const i8,
    mut resultp: *mut *const i8,
) -> bool {
    let mut m: size_t = mbslen(needle);
    let mut needle_mbchars: *mut mbchar_t = 0 as *mut mbchar_t;
    let mut table: *mut size_t = 0 as *mut size_t;
    let mut memory: *mut libc::c_void = if (::core::mem::size_of::<mbchar_t>() as u64)
        .wrapping_add(::core::mem::size_of::<size_t>() as u64) != 0 as i32 as u64
        && (if (9223372036854775807 as i64 as u64) < 18446744073709551615 as u64 {
            9223372036854775807 as i64 as u64
        } else {
            (18446744073709551615 as u64).wrapping_sub(1 as i32 as u64)
        })
            .wrapping_div(
                (::core::mem::size_of::<mbchar_t>() as u64)
                    .wrapping_add(::core::mem::size_of::<size_t>() as u64),
            ) < m
    {
        0 as *mut libc::c_void
    } else if m
        .wrapping_mul(
            (::core::mem::size_of::<mbchar_t>() as u64)
                .wrapping_add(::core::mem::size_of::<size_t>() as u64),
        )
        < (4032 as i32
            - (2 as i32 * C2RustUnnamed_0::sa_alignment_max as i32 - 1 as i32)) as u64
    {
        let mut fresh1 = ::std::vec::from_elem(
            0,
            m
                .wrapping_mul(
                    (::core::mem::size_of::<mbchar_t>() as u64)
                        .wrapping_add(::core::mem::size_of::<size_t>() as u64),
                )
                .wrapping_add(
                    (2 as i32 * C2RustUnnamed_0::sa_alignment_max as i32) as u64,
                )
                .wrapping_sub(1 as i32 as u64) as usize,
        );
        ((fresh1.as_mut_ptr() as *mut i8 as uintptr_t)
            .wrapping_add(
                (2 as i32 * C2RustUnnamed_0::sa_alignment_max as i32 - 1 as i32) as u64,
            )
            & !((2 as i32 * C2RustUnnamed_0::sa_alignment_max as i32 - 1 as i32)
                as uintptr_t)) as *mut libc::c_void
    } else {
        mmalloca(
            m
                .wrapping_mul(
                    (::core::mem::size_of::<mbchar_t>() as u64)
                        .wrapping_add(::core::mem::size_of::<size_t>() as u64),
                ),
        )
    };
    let mut table_memory: *mut libc::c_void = 0 as *mut libc::c_void;
    if memory.is_null() {
        return 0 as i32 != 0;
    }
    needle_mbchars = memory as *mut mbchar_t;
    table_memory = needle_mbchars.offset(m as isize) as *mut libc::c_void;
    table = table_memory as *mut size_t;
    let mut iter: mbui_iterator_t = mbui_iterator_t {
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
    let mut j: size_t = 0;
    j = 0 as i32 as size_t;
    iter.cur.ptr = needle;
    iter.in_shift = 0 as i32 != 0;
    memset(
        &mut iter.state as *mut mbstate_t as *mut libc::c_void,
        '\0' as i32,
        ::core::mem::size_of::<mbstate_t>() as u64,
    );
    iter.next_done = 0 as i32 != 0;
    loop {
        mbuiter_multi_next(&mut iter);
        if !(!(iter.cur.wc_valid as i32 != 0 && iter.cur.wc == 0 as i32) as i32 != 0) {
            break;
        }
        mb_copy(&mut *needle_mbchars.offset(j as isize), &mut iter.cur);
        iter.cur.ptr = (iter.cur.ptr).offset(iter.cur.bytes as isize);
        iter.next_done = 0 as i32 != 0;
        j = j.wrapping_add(1);
        j;
    }
    let mut i: size_t = 0;
    let mut j_0: size_t = 0;
    *table.offset(1 as i32 as isize) = 1 as i32 as size_t;
    j_0 = 0 as i32 as size_t;
    i = 2 as i32 as size_t;
    while i < m {
        let mut b: *mut mbchar_t = &mut *needle_mbchars
            .offset(i.wrapping_sub(1 as i32 as u64) as isize) as *mut mbchar_t;
        loop {
            if if (*b).wc_valid as i32 != 0
                && (*needle_mbchars.offset(j_0 as isize)).wc_valid as i32 != 0
            {
                ((*b).wc == (*needle_mbchars.offset(j_0 as isize)).wc) as i32
            } else {
                ((*b).bytes == (*needle_mbchars.offset(j_0 as isize)).bytes
                    && memcmp(
                        (*b).ptr as *const libc::c_void,
                        (*needle_mbchars.offset(j_0 as isize)).ptr
                            as *const libc::c_void,
                        (*b).bytes,
                    ) == 0 as i32) as i32
            } != 0
            {
                j_0 = j_0.wrapping_add(1);
                *table.offset(i as isize) = i.wrapping_sub(j_0);
                break;
            } else if j_0 == 0 as i32 as u64 {
                *table.offset(i as isize) = i;
                break;
            } else {
                j_0 = j_0.wrapping_sub(*table.offset(j_0 as isize));
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    let mut j_1: size_t = 0;
    let mut rhaystack: mbui_iterator_t = mbui_iterator_t {
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
    let mut phaystack: mbui_iterator_t = mbui_iterator_t {
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
    *resultp = 0 as *const i8;
    j_1 = 0 as i32 as size_t;
    rhaystack.cur.ptr = haystack;
    rhaystack.in_shift = 0 as i32 != 0;
    memset(
        &mut rhaystack.state as *mut mbstate_t as *mut libc::c_void,
        '\0' as i32,
        ::core::mem::size_of::<mbstate_t>() as u64,
    );
    rhaystack.next_done = 0 as i32 != 0;
    phaystack.cur.ptr = haystack;
    phaystack.in_shift = 0 as i32 != 0;
    memset(
        &mut phaystack.state as *mut mbstate_t as *mut libc::c_void,
        '\0' as i32,
        ::core::mem::size_of::<mbstate_t>() as u64,
    );
    phaystack.next_done = 0 as i32 != 0;
    loop {
        mbuiter_multi_next(&mut phaystack);
        if !(!(phaystack.cur.wc_valid as i32 != 0 && phaystack.cur.wc == 0 as i32) as i32
            != 0)
        {
            break;
        }
        if if (*needle_mbchars.offset(j_1 as isize)).wc_valid as i32 != 0
            && phaystack.cur.wc_valid as i32 != 0
        {
            ((*needle_mbchars.offset(j_1 as isize)).wc == phaystack.cur.wc) as i32
        } else {
            ((*needle_mbchars.offset(j_1 as isize)).bytes == phaystack.cur.bytes
                && memcmp(
                    (*needle_mbchars.offset(j_1 as isize)).ptr as *const libc::c_void,
                    phaystack.cur.ptr as *const libc::c_void,
                    (*needle_mbchars.offset(j_1 as isize)).bytes,
                ) == 0 as i32) as i32
        } != 0
        {
            j_1 = j_1.wrapping_add(1);
            j_1;
            phaystack.cur.ptr = (phaystack.cur.ptr).offset(phaystack.cur.bytes as isize);
            phaystack.next_done = 0 as i32 != 0;
            if !(j_1 == m) {
                continue;
            }
            *resultp = rhaystack.cur.ptr;
            break;
        } else if j_1 > 0 as i32 as u64 {
            let mut count: size_t = *table.offset(j_1 as isize);
            j_1 = (j_1 as u64).wrapping_sub(count) as size_t as size_t;
            while count > 0 as i32 as u64 {
                mbuiter_multi_next(&mut rhaystack);
                if !(rhaystack.cur.wc_valid as i32 != 0 && rhaystack.cur.wc == 0 as i32)
                    as i32 == 0
                {
                    abort();
                }
                rhaystack.cur.ptr = (rhaystack.cur.ptr)
                    .offset(rhaystack.cur.bytes as isize);
                rhaystack.next_done = 0 as i32 != 0;
                count = count.wrapping_sub(1);
                count;
            }
        } else {
            mbuiter_multi_next(&mut rhaystack);
            if !(rhaystack.cur.wc_valid as i32 != 0 && rhaystack.cur.wc == 0 as i32)
                as i32 == 0
            {
                abort();
            }
            rhaystack.cur.ptr = (rhaystack.cur.ptr).offset(rhaystack.cur.bytes as isize);
            rhaystack.next_done = 0 as i32 != 0;
            phaystack.cur.ptr = (phaystack.cur.ptr).offset(phaystack.cur.bytes as isize);
            phaystack.next_done = 0 as i32 != 0;
        }
    }
    freea(memory);
    return 1 as i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn mbsstr(
    mut haystack: *const i8,
    mut needle: *const i8,
) -> *mut i8 {
    if __ctype_get_mb_cur_max() > 1 as i32 as u64 {
        let mut iter_needle: mbui_iterator_t = mbui_iterator_t {
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
        iter_needle.cur.ptr = needle;
        iter_needle.in_shift = 0 as i32 != 0;
        memset(
            &mut iter_needle.state as *mut mbstate_t as *mut libc::c_void,
            '\0' as i32,
            ::core::mem::size_of::<mbstate_t>() as u64,
        );
        iter_needle.next_done = 0 as i32 != 0;
        mbuiter_multi_next(&mut iter_needle);
        if !(iter_needle.cur.wc_valid as i32 != 0 && iter_needle.cur.wc == 0 as i32)
            as i32 != 0
        {
            let mut try_kmp: bool = 1 as i32 != 0;
            let mut outer_loop_count: size_t = 0 as i32 as size_t;
            let mut comparison_count: size_t = 0 as i32 as size_t;
            let mut last_ccount: size_t = 0 as i32 as size_t;
            let mut iter_needle_last_ccount: mbui_iterator_t = mbui_iterator_t {
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
            let mut iter_haystack: mbui_iterator_t = mbui_iterator_t {
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
            iter_needle_last_ccount.cur.ptr = needle;
            iter_needle_last_ccount.in_shift = 0 as i32 != 0;
            memset(
                &mut iter_needle_last_ccount.state as *mut mbstate_t
                    as *mut libc::c_void,
                '\0' as i32,
                ::core::mem::size_of::<mbstate_t>() as u64,
            );
            iter_needle_last_ccount.next_done = 0 as i32 != 0;
            iter_haystack.cur.ptr = haystack;
            iter_haystack.in_shift = 0 as i32 != 0;
            memset(
                &mut iter_haystack.state as *mut mbstate_t as *mut libc::c_void,
                '\0' as i32,
                ::core::mem::size_of::<mbstate_t>() as u64,
            );
            iter_haystack.next_done = 0 as i32 != 0;
            loop {
                mbuiter_multi_next(&mut iter_haystack);
                if !(iter_haystack.cur.wc_valid as i32 != 0
                    && iter_haystack.cur.wc == 0 as i32) as i32 == 0
                {
                    return 0 as *mut i8;
                }
                if try_kmp as i32 != 0 && outer_loop_count >= 10 as i32 as u64
                    && comparison_count
                        >= (5 as i32 as u64).wrapping_mul(outer_loop_count)
                {
                    let mut count: size_t = comparison_count.wrapping_sub(last_ccount);
                    while count > 0 as i32 as u64
                        && {
                            mbuiter_multi_next(&mut iter_needle_last_ccount);
                            !(iter_needle_last_ccount.cur.wc_valid as i32 != 0
                                && iter_needle_last_ccount.cur.wc == 0 as i32) as i32 != 0
                        }
                    {
                        iter_needle_last_ccount.cur.ptr = (iter_needle_last_ccount
                            .cur
                            .ptr)
                            .offset(iter_needle_last_ccount.cur.bytes as isize);
                        iter_needle_last_ccount.next_done = 0 as i32 != 0;
                        count = count.wrapping_sub(1);
                        count;
                    }
                    last_ccount = comparison_count;
                    mbuiter_multi_next(&mut iter_needle_last_ccount);
                    if !(iter_needle_last_ccount.cur.wc_valid as i32 != 0
                        && iter_needle_last_ccount.cur.wc == 0 as i32) as i32 == 0
                    {
                        let mut result: *const i8 = 0 as *const i8;
                        let mut success: bool = knuth_morris_pratt_multibyte(
                            haystack,
                            needle,
                            &mut result,
                        );
                        if success {
                            return result as *mut i8;
                        }
                        try_kmp = 0 as i32 != 0;
                    }
                }
                outer_loop_count = outer_loop_count.wrapping_add(1);
                outer_loop_count;
                comparison_count = comparison_count.wrapping_add(1);
                comparison_count;
                if if iter_haystack.cur.wc_valid as i32 != 0
                    && iter_needle.cur.wc_valid as i32 != 0
                {
                    (iter_haystack.cur.wc == iter_needle.cur.wc) as i32
                } else {
                    (iter_haystack.cur.bytes == iter_needle.cur.bytes
                        && memcmp(
                            iter_haystack.cur.ptr as *const libc::c_void,
                            iter_needle.cur.ptr as *const libc::c_void,
                            iter_haystack.cur.bytes,
                        ) == 0 as i32) as i32
                } != 0
                {
                    let mut rhaystack: mbui_iterator_t = mbui_iterator_t {
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
                    let mut rneedle: mbui_iterator_t = mbui_iterator_t {
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
                    memcpy(
                        &mut rhaystack as *mut mbui_iterator_t as *mut libc::c_void,
                        &mut iter_haystack as *mut mbui_iterator_t
                            as *const libc::c_void,
                        ::core::mem::size_of::<mbui_iterator_t>() as u64,
                    );
                    rhaystack.cur.ptr = (rhaystack.cur.ptr)
                        .offset(rhaystack.cur.bytes as isize);
                    rhaystack.next_done = 0 as i32 != 0;
                    rneedle.cur.ptr = needle;
                    rneedle.in_shift = 0 as i32 != 0;
                    memset(
                        &mut rneedle.state as *mut mbstate_t as *mut libc::c_void,
                        '\0' as i32,
                        ::core::mem::size_of::<mbstate_t>() as u64,
                    );
                    rneedle.next_done = 0 as i32 != 0;
                    mbuiter_multi_next(&mut rneedle);
                    if !(rneedle.cur.wc_valid as i32 != 0 && rneedle.cur.wc == 0 as i32)
                        as i32 == 0
                    {
                        abort();
                    }
                    rneedle.cur.ptr = (rneedle.cur.ptr)
                        .offset(rneedle.cur.bytes as isize);
                    rneedle.next_done = 0 as i32 != 0;
                    loop {
                        mbuiter_multi_next(&mut rneedle);
                        if !(rneedle.cur.wc_valid as i32 != 0
                            && rneedle.cur.wc == 0 as i32) as i32 == 0
                        {
                            return iter_haystack.cur.ptr as *mut i8;
                        }
                        mbuiter_multi_next(&mut rhaystack);
                        if !(rhaystack.cur.wc_valid as i32 != 0
                            && rhaystack.cur.wc == 0 as i32) as i32 == 0
                        {
                            return 0 as *mut i8;
                        }
                        comparison_count = comparison_count.wrapping_add(1);
                        comparison_count;
                        if if rhaystack.cur.wc_valid as i32 != 0
                            && rneedle.cur.wc_valid as i32 != 0
                        {
                            (rhaystack.cur.wc == rneedle.cur.wc) as i32
                        } else {
                            (rhaystack.cur.bytes == rneedle.cur.bytes
                                && memcmp(
                                    rhaystack.cur.ptr as *const libc::c_void,
                                    rneedle.cur.ptr as *const libc::c_void,
                                    rhaystack.cur.bytes,
                                ) == 0 as i32) as i32
                        } == 0
                        {
                            break;
                        }
                        rhaystack.cur.ptr = (rhaystack.cur.ptr)
                            .offset(rhaystack.cur.bytes as isize);
                        rhaystack.next_done = 0 as i32 != 0;
                        rneedle.cur.ptr = (rneedle.cur.ptr)
                            .offset(rneedle.cur.bytes as isize);
                        rneedle.next_done = 0 as i32 != 0;
                    }
                }
                iter_haystack.cur.ptr = (iter_haystack.cur.ptr)
                    .offset(iter_haystack.cur.bytes as isize);
                iter_haystack.next_done = 0 as i32 != 0;
            }
        } else {
            return haystack as *mut i8
        }
    } else if *needle as i32 != '\0' as i32 {
        let mut try_kmp_0: bool = 1 as i32 != 0;
        let mut outer_loop_count_0: size_t = 0 as i32 as size_t;
        let mut comparison_count_0: size_t = 0 as i32 as size_t;
        let mut last_ccount_0: size_t = 0 as i32 as size_t;
        let mut needle_last_ccount: *const i8 = needle;
        let fresh2 = needle;
        needle = needle.offset(1);
        let mut b: i8 = *fresh2;
        loop {
            if *haystack as i32 == '\0' as i32 {
                return 0 as *mut i8;
            }
            if try_kmp_0 as i32 != 0 && outer_loop_count_0 >= 10 as i32 as u64
                && comparison_count_0
                    >= (5 as i32 as u64).wrapping_mul(outer_loop_count_0)
            {
                if !needle_last_ccount.is_null() {
                    needle_last_ccount = needle_last_ccount
                        .offset(
                            strnlen(
                                needle_last_ccount,
                                comparison_count_0.wrapping_sub(last_ccount_0),
                            ) as isize,
                        );
                    if *needle_last_ccount as i32 == '\0' as i32 {
                        needle_last_ccount = 0 as *const i8;
                    }
                    last_ccount_0 = comparison_count_0;
                }
                if needle_last_ccount.is_null() {
                    let mut result_0: *const u8 = 0 as *const u8;
                    let mut success_0: bool = knuth_morris_pratt(
                        haystack as *const u8,
                        needle.offset(-(1 as i32 as isize)) as *const u8,
                        strlen(needle.offset(-(1 as i32 as isize))),
                        &mut result_0,
                    );
                    if success_0 {
                        return result_0 as *mut i8;
                    }
                    try_kmp_0 = 0 as i32 != 0;
                }
            }
            outer_loop_count_0 = outer_loop_count_0.wrapping_add(1);
            outer_loop_count_0;
            comparison_count_0 = comparison_count_0.wrapping_add(1);
            comparison_count_0;
            if *haystack as i32 == b as i32 {
                let mut rhaystack_0: *const i8 = haystack.offset(1 as i32 as isize);
                let mut rneedle_0: *const i8 = needle;
                loop {
                    if *rneedle_0 as i32 == '\0' as i32 {
                        return haystack as *mut i8;
                    }
                    if *rhaystack_0 as i32 == '\0' as i32 {
                        return 0 as *mut i8;
                    }
                    comparison_count_0 = comparison_count_0.wrapping_add(1);
                    comparison_count_0;
                    if *rhaystack_0 as i32 != *rneedle_0 as i32 {
                        break;
                    }
                    rhaystack_0 = rhaystack_0.offset(1);
                    rhaystack_0;
                    rneedle_0 = rneedle_0.offset(1);
                    rneedle_0;
                }
            }
            haystack = haystack.offset(1);
            haystack;
        }
    } else {
        return haystack as *mut i8
    };
}