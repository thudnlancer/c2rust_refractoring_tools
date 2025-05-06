#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn __errno_location() -> *mut i32;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: u64) -> *mut libc::c_void;
    fn abort() -> !;
    fn rpl_free(ptr: *mut libc::c_void);
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn u8_mbtouc_unsafe_aux(puc: *mut ucs4_t, s: *const uint8_t, n: size_t) -> i32;
    fn u8_uctomb_aux(s: *mut uint8_t, uc: ucs4_t, n: ptrdiff_t) -> i32;
    fn u8_cpy(dest: *mut uint8_t, src: *const uint8_t, n: size_t) -> *mut uint8_t;
    fn uc_combining_class(uc: ucs4_t) -> i32;
    fn gl_uninorm_decompose_merge_sort_inplace(
        src: *mut ucs4_with_ccc,
        n: size_t,
        tmp: *mut ucs4_with_ccc,
    );
}
pub type ptrdiff_t = i64;
pub type size_t = u64;
pub type __uint8_t = u8;
pub type __uint32_t = u32;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type ucs4_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unicode_normalization_form {
    pub description: u32,
    pub decomposer: Option<unsafe extern "C" fn(ucs4_t, *mut ucs4_t) -> i32>,
    pub composer: Option<unsafe extern "C" fn(ucs4_t, ucs4_t) -> ucs4_t>,
    pub decomposing_variant: *const unicode_normalization_form,
}
pub type uninorm_t = *const unicode_normalization_form;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ucs4_with_ccc {
    pub code: ucs4_t,
    pub ccc: i32,
}
#[inline]
unsafe extern "C" fn u8_mbtouc_unsafe(
    mut puc: *mut ucs4_t,
    mut s: *const uint8_t,
    mut n: size_t,
) -> i32 {
    let mut c: uint8_t = *s;
    if (c as i32) < 0x80 as i32 {
        *puc = c as ucs4_t;
        return 1 as i32;
    } else {
        return u8_mbtouc_unsafe_aux(puc, s, n)
    };
}
#[inline]
unsafe extern "C" fn u8_uctomb(
    mut s: *mut uint8_t,
    mut uc: ucs4_t,
    mut n: ptrdiff_t,
) -> i32 {
    if uc < 0x80 as i32 as u32 && n > 0 as i32 as i64 {
        *s.offset(0 as i32 as isize) = uc as uint8_t;
        return 1 as i32;
    } else {
        return u8_uctomb_aux(s, uc, n)
    };
}
#[no_mangle]
pub unsafe extern "C" fn u8_normalize(
    mut nf: uninorm_t,
    mut s: *const uint8_t,
    mut n: size_t,
    mut resultbuf: *mut uint8_t,
    mut lengthp: *mut size_t,
) -> *mut uint8_t {
    let mut current_block: u64;
    let mut decomposer: Option<unsafe extern "C" fn(ucs4_t, *mut ucs4_t) -> i32> = (*nf)
        .decomposer;
    let mut composer: Option<unsafe extern "C" fn(ucs4_t, ucs4_t) -> ucs4_t> = (*nf)
        .composer;
    let mut result: *mut uint8_t = 0 as *mut uint8_t;
    let mut length: size_t = 0;
    let mut allocated: size_t = 0;
    let mut sortbuf_preallocated: [ucs4_with_ccc; 128] = [ucs4_with_ccc {
        code: 0,
        ccc: 0,
    }; 128];
    let mut sortbuf: *mut ucs4_with_ccc = 0 as *mut ucs4_with_ccc;
    let mut sortbuf_allocated: size_t = 0;
    let mut sortbuf_count: size_t = 0;
    if resultbuf.is_null() {
        result = 0 as *mut uint8_t;
        allocated = 0 as i32 as size_t;
    } else {
        result = resultbuf;
        allocated = *lengthp;
    }
    length = 0 as i32 as size_t;
    sortbuf = sortbuf_preallocated.as_mut_ptr();
    sortbuf_allocated = 64 as i32 as size_t;
    sortbuf_count = 0 as i32 as size_t;
    let mut s_end: *const uint8_t = s.offset(n as isize);
    's_65: loop {
        let mut count: i32 = 0;
        let mut decomposed: [ucs4_t; 32] = [0; 32];
        let mut decomposed_count: i32 = 0;
        let mut i: i32 = 0;
        if s < s_end {
            count = u8_mbtouc_unsafe(
                &mut *decomposed.as_mut_ptr().offset(0 as i32 as isize),
                s,
                s_end.offset_from(s) as i64 as size_t,
            );
            decomposed_count = 1 as i32;
            let mut curr: i32 = 0;
            curr = 0 as i32;
            while curr < decomposed_count {
                let mut curr_decomposed: [ucs4_t; 32] = [0; 32];
                let mut curr_decomposed_count: i32 = 0;
                curr_decomposed_count = decomposer
                    .expect(
                        "non-null function pointer",
                    )(decomposed[curr as usize], curr_decomposed.as_mut_ptr());
                if curr_decomposed_count >= 0 as i32 {
                    let mut shift: i32 = curr_decomposed_count - 1 as i32;
                    if shift < 0 as i32 {
                        abort();
                    }
                    if shift > 0 as i32 {
                        let mut j: i32 = 0;
                        decomposed_count += shift;
                        if decomposed_count > 32 as i32 {
                            abort();
                        }
                        j = decomposed_count - 1 as i32 - shift;
                        while j > curr {
                            decomposed[(j + shift) as usize] = decomposed[j as usize];
                            j -= 1;
                            j;
                        }
                    }
                    while shift >= 0 as i32 {
                        decomposed[(curr + shift) as usize] = curr_decomposed[shift
                            as usize];
                        shift -= 1;
                        shift;
                    }
                } else {
                    curr += 1;
                    curr;
                }
            }
        } else {
            count = 0 as i32;
            decomposed_count = 0 as i32;
        }
        i = 0 as i32;
        loop {
            let mut uc: ucs4_t = 0;
            let mut ccc: i32 = 0;
            if s < s_end {
                if i == decomposed_count {
                    break;
                }
                uc = decomposed[i as usize];
                ccc = uc_combining_class(uc);
            } else {
                uc = 0 as i32 as ucs4_t;
                ccc = 0 as i32;
            }
            if ccc == 0 as i32 {
                let mut j_0: size_t = 0;
                if sortbuf_count > 1 as i32 as u64 {
                    gl_uninorm_decompose_merge_sort_inplace(
                        sortbuf,
                        sortbuf_count,
                        sortbuf.offset(sortbuf_count as isize),
                    );
                }
                if composer.is_some() {
                    if sortbuf_count > 0 as i32 as u64
                        && (*sortbuf.offset(0 as i32 as isize)).ccc == 0 as i32
                    {
                        j_0 = 1 as i32 as size_t;
                        while j_0 < sortbuf_count {
                            if (*sortbuf.offset(j_0 as isize)).ccc
                                > (*sortbuf
                                    .offset(j_0.wrapping_sub(1 as i32 as u64) as isize))
                                    .ccc
                            {
                                let mut combined: ucs4_t = composer
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    (*sortbuf.offset(0 as i32 as isize)).code,
                                    (*sortbuf.offset(j_0 as isize)).code,
                                );
                                if combined != 0 {
                                    let mut k: size_t = 0;
                                    (*sortbuf.offset(0 as i32 as isize)).code = combined;
                                    k = j_0.wrapping_add(1 as i32 as u64);
                                    while k < sortbuf_count {
                                        *sortbuf.offset(k.wrapping_sub(1 as i32 as u64) as isize) = *sortbuf
                                            .offset(k as isize);
                                        k = k.wrapping_add(1);
                                        k;
                                    }
                                    sortbuf_count = sortbuf_count.wrapping_sub(1);
                                    sortbuf_count;
                                    continue;
                                }
                            }
                            j_0 = j_0.wrapping_add(1);
                            j_0;
                        }
                        if s < s_end && sortbuf_count == 1 as i32 as u64 {
                            let mut combined_0: ucs4_t = composer
                                .expect(
                                    "non-null function pointer",
                                )((*sortbuf.offset(0 as i32 as isize)).code, uc);
                            if combined_0 != 0 {
                                uc = combined_0;
                                ccc = 0 as i32;
                                sortbuf_count = 0 as i32 as size_t;
                            }
                        }
                    }
                }
                j_0 = 0 as i32 as size_t;
                while j_0 < sortbuf_count {
                    let mut muc: ucs4_t = (*sortbuf.offset(j_0 as isize)).code;
                    if length < allocated {
                        let mut ret: i32 = u8_uctomb(
                            result.offset(length as isize),
                            muc,
                            allocated.wrapping_sub(length) as ptrdiff_t,
                        );
                        if ret == -(1 as i32) {
                            *__errno_location() = 22 as i32;
                            current_block = 6368818216055879348;
                            break 's_65;
                        } else if ret >= 0 as i32 {
                            length = (length as u64).wrapping_add(ret as u64) as size_t
                                as size_t;
                            current_block = 7419121793134201633;
                        } else {
                            current_block = 726525485109251713;
                        }
                    } else {
                        current_block = 726525485109251713;
                    }
                    match current_block {
                        726525485109251713 => {
                            let mut old_allocated: size_t = allocated;
                            let mut new_allocated: size_t = (2 as i32 as u64)
                                .wrapping_mul(old_allocated);
                            if new_allocated < 64 as i32 as u64 {
                                new_allocated = 64 as i32 as size_t;
                            }
                            if new_allocated < old_allocated {
                                abort();
                            }
                            let mut larger_result: *mut uint8_t = 0 as *mut uint8_t;
                            if result.is_null() {
                                larger_result = malloc(
                                    new_allocated
                                        .wrapping_mul(::core::mem::size_of::<uint8_t>() as u64),
                                ) as *mut uint8_t;
                                if larger_result.is_null() {
                                    *__errno_location() = 12 as i32;
                                    current_block = 6368818216055879348;
                                    break 's_65;
                                }
                            } else if result == resultbuf {
                                larger_result = malloc(
                                    new_allocated
                                        .wrapping_mul(::core::mem::size_of::<uint8_t>() as u64),
                                ) as *mut uint8_t;
                                if larger_result.is_null() {
                                    *__errno_location() = 12 as i32;
                                    current_block = 6368818216055879348;
                                    break 's_65;
                                } else {
                                    u8_cpy(larger_result, resultbuf, length);
                                }
                            } else {
                                larger_result = realloc(
                                    result as *mut libc::c_void,
                                    new_allocated
                                        .wrapping_mul(::core::mem::size_of::<uint8_t>() as u64),
                                ) as *mut uint8_t;
                                if larger_result.is_null() {
                                    *__errno_location() = 12 as i32;
                                    current_block = 6368818216055879348;
                                    break 's_65;
                                }
                            }
                            result = larger_result;
                            allocated = new_allocated;
                            let mut ret_0: i32 = u8_uctomb(
                                result.offset(length as isize),
                                muc,
                                allocated.wrapping_sub(length) as ptrdiff_t,
                            );
                            if ret_0 == -(1 as i32) {
                                *__errno_location() = 22 as i32;
                                current_block = 6368818216055879348;
                                break 's_65;
                            } else {
                                if ret_0 < 0 as i32 {
                                    abort();
                                }
                                length = (length as u64).wrapping_add(ret_0 as u64)
                                    as size_t as size_t;
                            }
                        }
                        _ => {}
                    }
                    j_0 = j_0.wrapping_add(1);
                    j_0;
                }
                sortbuf_count = 0 as i32 as size_t;
            }
            if !(s < s_end) {
                break;
            }
            if sortbuf_count == sortbuf_allocated {
                let mut new_sortbuf: *mut ucs4_with_ccc = 0 as *mut ucs4_with_ccc;
                sortbuf_allocated = (2 as i32 as u64).wrapping_mul(sortbuf_allocated);
                if sortbuf_allocated < sortbuf_count {
                    abort();
                }
                new_sortbuf = malloc(
                    (2 as i32 as u64)
                        .wrapping_mul(sortbuf_allocated)
                        .wrapping_mul(::core::mem::size_of::<ucs4_with_ccc>() as u64),
                ) as *mut ucs4_with_ccc;
                if new_sortbuf.is_null() {
                    *__errno_location() = 12 as i32;
                    current_block = 6368818216055879348;
                    break 's_65;
                } else {
                    memcpy(
                        new_sortbuf as *mut libc::c_void,
                        sortbuf as *const libc::c_void,
                        sortbuf_count
                            .wrapping_mul(::core::mem::size_of::<ucs4_with_ccc>() as u64),
                    );
                    if sortbuf != sortbuf_preallocated.as_mut_ptr() {
                        rpl_free(sortbuf as *mut libc::c_void);
                    }
                    sortbuf = new_sortbuf;
                }
            }
            (*sortbuf.offset(sortbuf_count as isize)).code = uc;
            (*sortbuf.offset(sortbuf_count as isize)).ccc = ccc;
            sortbuf_count = sortbuf_count.wrapping_add(1);
            sortbuf_count;
            i += 1;
            i;
        }
        if !(s < s_end) {
            current_block = 16314074004867283505;
            break;
        }
        s = s.offset(count as isize);
    }
    match current_block {
        16314074004867283505 => {
            if length == 0 as i32 as u64 {
                if result.is_null() {
                    result = malloc(1 as i32 as u64) as *mut uint8_t;
                    if result.is_null() {
                        *__errno_location() = 12 as i32;
                        current_block = 6368818216055879348;
                    } else {
                        current_block = 18201902862271706575;
                    }
                } else {
                    current_block = 18201902862271706575;
                }
            } else {
                if result != resultbuf && length < allocated {
                    let mut memory: *mut uint8_t = 0 as *mut uint8_t;
                    memory = realloc(
                        result as *mut libc::c_void,
                        length.wrapping_mul(::core::mem::size_of::<uint8_t>() as u64),
                    ) as *mut uint8_t;
                    if !memory.is_null() {
                        result = memory;
                    }
                }
                current_block = 18201902862271706575;
            }
            match current_block {
                6368818216055879348 => {}
                _ => {
                    if sortbuf_count > 0 as i32 as u64 {
                        abort();
                    }
                    if sortbuf != sortbuf_preallocated.as_mut_ptr() {
                        rpl_free(sortbuf as *mut libc::c_void);
                    }
                    *lengthp = length;
                    return result;
                }
            }
        }
        _ => {}
    }
    let mut saved_errno: i32 = *__errno_location();
    if sortbuf != sortbuf_preallocated.as_mut_ptr() {
        rpl_free(sortbuf as *mut libc::c_void);
    }
    if result != resultbuf {
        rpl_free(result as *mut libc::c_void);
    }
    *__errno_location() = saved_errno;
    return 0 as *mut uint8_t;
}