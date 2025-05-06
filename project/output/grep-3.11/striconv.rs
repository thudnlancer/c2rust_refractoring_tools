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
    fn malloc(_: u64) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: u64) -> *mut libc::c_void;
    fn abort() -> !;
    fn iconv_close(__cd: iconv_t) -> i32;
    fn iconv(
        __cd: iconv_t,
        __inbuf: *mut *mut i8,
        __inbytesleft: *mut size_t,
        __outbuf: *mut *mut i8,
        __outbytesleft: *mut size_t,
    ) -> size_t;
    fn rpl_free(ptr: *mut libc::c_void);
    fn iconv_open(__tocode: *const i8, __fromcode: *const i8) -> iconv_t;
    fn __errno_location() -> *mut i32;
    fn strdup(_: *const i8) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn c_strcasecmp(s1: *const i8, s2: *const i8) -> i32;
}
pub type size_t = u64;
pub type iconv_t = *mut libc::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub align: u32,
    pub buf: [i8; 4096],
}
#[no_mangle]
pub unsafe extern "C" fn mem_cd_iconv(
    mut src: *const i8,
    mut srclen: size_t,
    mut cd: iconv_t,
    mut resultp: *mut *mut i8,
    mut lengthp: *mut size_t,
) -> i32 {
    let mut current_block: u64;
    let mut length: size_t = 0;
    let mut result: *mut i8 = 0 as *mut i8;
    iconv(cd, 0 as *mut *mut i8, 0 as *mut size_t, 0 as *mut *mut i8, 0 as *mut size_t);
    let mut count: size_t = 0 as i32 as size_t;
    let mut tmp: C2RustUnnamed = C2RustUnnamed { align: 0 };
    let mut inptr: *const i8 = src;
    let mut insize: size_t = srclen;
    while insize > 0 as i32 as u64 {
        let mut outptr: *mut i8 = (tmp.buf).as_mut_ptr();
        let mut outsize: size_t = 4096 as i32 as size_t;
        let mut res: size_t = iconv(
            cd,
            &mut inptr as *mut *const i8 as *mut *mut i8,
            &mut insize,
            &mut outptr,
            &mut outsize,
        );
        if res == -(1 as i32) as size_t {
            if !(*__errno_location() == 7 as i32) {
                if *__errno_location() == 22 as i32 {
                    break;
                }
                return -(1 as i32);
            }
        }
        count = (count as u64)
            .wrapping_add(outptr.offset_from((tmp.buf).as_mut_ptr()) as i64 as u64)
            as size_t as size_t;
    }
    let mut outptr_0: *mut i8 = (tmp.buf).as_mut_ptr();
    let mut outsize_0: size_t = 4096 as i32 as size_t;
    let mut res_0: size_t = iconv(
        cd,
        0 as *mut *mut i8,
        0 as *mut size_t,
        &mut outptr_0,
        &mut outsize_0,
    );
    if res_0 == -(1 as i32) as size_t {
        return -(1 as i32);
    }
    count = (count as u64)
        .wrapping_add(outptr_0.offset_from((tmp.buf).as_mut_ptr()) as i64 as u64)
        as size_t as size_t;
    length = count;
    if length == 0 as i32 as u64 {
        *lengthp = 0 as i32 as size_t;
        return 0 as i32;
    }
    if !(*resultp).is_null() && *lengthp >= length {
        result = *resultp;
    } else {
        result = malloc(length) as *mut i8;
        if result.is_null() {
            *__errno_location() = 12 as i32;
            return -(1 as i32);
        }
    }
    iconv(cd, 0 as *mut *mut i8, 0 as *mut size_t, 0 as *mut *mut i8, 0 as *mut size_t);
    let mut inptr_0: *const i8 = src;
    let mut insize_0: size_t = srclen;
    let mut outptr_1: *mut i8 = result;
    let mut outsize_1: size_t = length;
    loop {
        if !(insize_0 > 0 as i32 as u64) {
            current_block = 652864300344834934;
            break;
        }
        let mut res_1: size_t = iconv(
            cd,
            &mut inptr_0 as *mut *const i8 as *mut *mut i8,
            &mut insize_0,
            &mut outptr_1,
            &mut outsize_1,
        );
        if !(res_1 == -(1 as i32) as size_t) {
            continue;
        }
        if *__errno_location() == 22 as i32 {
            current_block = 652864300344834934;
            break;
        } else {
            current_block = 14485383533848458216;
            break;
        }
    }
    match current_block {
        652864300344834934 => {
            let mut res_2: size_t = iconv(
                cd,
                0 as *mut *mut i8,
                0 as *mut size_t,
                &mut outptr_1,
                &mut outsize_1,
            );
            if !(res_2 == -(1 as i32) as size_t) {
                if outsize_1 != 0 as i32 as u64 {
                    abort();
                }
                *resultp = result;
                *lengthp = length;
                return 0 as i32;
            }
        }
        _ => {}
    }
    if result != *resultp {
        rpl_free(result as *mut libc::c_void);
    }
    return -(1 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn str_cd_iconv(mut src: *const i8, mut cd: iconv_t) -> *mut i8 {
    let mut current_block: u64;
    let mut result: *mut i8 = 0 as *mut i8;
    let mut result_size: size_t = 0;
    let mut length: size_t = 0;
    let mut inptr: *const i8 = src;
    let mut inbytes_remaining: size_t = strlen(src);
    result_size = inbytes_remaining;
    let mut approx_sqrt_SIZE_MAX: size_t = -(1 as i32) as size_t
        >> (::core::mem::size_of::<size_t>() as u64)
            .wrapping_mul(8 as i32 as u64)
            .wrapping_div(2 as i32 as u64);
    if result_size <= approx_sqrt_SIZE_MAX.wrapping_div(16 as i32 as u64) {
        result_size = (result_size as u64).wrapping_mul(16 as i32 as u64) as size_t
            as size_t;
    }
    result_size = (result_size as u64).wrapping_add(1 as i32 as u64) as size_t as size_t;
    result = malloc(result_size) as *mut i8;
    if result.is_null() {
        *__errno_location() = 12 as i32;
        return 0 as *mut i8;
    }
    iconv(cd, 0 as *mut *mut i8, 0 as *mut size_t, 0 as *mut *mut i8, 0 as *mut size_t);
    let mut outptr: *mut i8 = result;
    let mut outbytes_remaining: size_t = result_size.wrapping_sub(1 as i32 as u64);
    loop {
        let mut res: size_t = iconv(
            cd,
            &mut inptr as *mut *const i8 as *mut *mut i8,
            &mut inbytes_remaining,
            &mut outptr,
            &mut outbytes_remaining,
        );
        if !(res == -(1 as i32) as size_t) {
            current_block = 6669252993407410313;
            break;
        }
        if *__errno_location() == 22 as i32 {
            current_block = 6669252993407410313;
            break;
        }
        if !(*__errno_location() == 7 as i32) {
            current_block = 16180931482009742287;
            break;
        }
        let mut used: size_t = outptr.offset_from(result) as i64 as size_t;
        let mut newsize: size_t = result_size.wrapping_mul(2 as i32 as u64);
        let mut newresult: *mut i8 = 0 as *mut i8;
        if !(newsize > result_size) {
            *__errno_location() = 12 as i32;
            current_block = 16180931482009742287;
            break;
        } else {
            newresult = realloc(result as *mut libc::c_void, newsize) as *mut i8;
            if newresult.is_null() {
                *__errno_location() = 12 as i32;
                current_block = 16180931482009742287;
                break;
            } else {
                result = newresult;
                result_size = newsize;
                outptr = result.offset(used as isize);
                outbytes_remaining = result_size
                    .wrapping_sub(1 as i32 as u64)
                    .wrapping_sub(used);
            }
        }
    }
    match current_block {
        6669252993407410313 => {
            loop {
                let mut res_0: size_t = iconv(
                    cd,
                    0 as *mut *mut i8,
                    0 as *mut size_t,
                    &mut outptr,
                    &mut outbytes_remaining,
                );
                if !(res_0 == -(1 as i32) as size_t) {
                    current_block = 6450597802325118133;
                    break;
                }
                if !(*__errno_location() == 7 as i32) {
                    current_block = 16180931482009742287;
                    break;
                }
                let mut used_0: size_t = outptr.offset_from(result) as i64 as size_t;
                let mut newsize_0: size_t = result_size.wrapping_mul(2 as i32 as u64);
                let mut newresult_0: *mut i8 = 0 as *mut i8;
                if !(newsize_0 > result_size) {
                    *__errno_location() = 12 as i32;
                    current_block = 16180931482009742287;
                    break;
                } else {
                    newresult_0 = realloc(result as *mut libc::c_void, newsize_0)
                        as *mut i8;
                    if newresult_0.is_null() {
                        *__errno_location() = 12 as i32;
                        current_block = 16180931482009742287;
                        break;
                    } else {
                        result = newresult_0;
                        result_size = newsize_0;
                        outptr = result.offset(used_0 as isize);
                        outbytes_remaining = result_size
                            .wrapping_sub(1 as i32 as u64)
                            .wrapping_sub(used_0);
                    }
                }
            }
            match current_block {
                16180931482009742287 => {}
                _ => {
                    let fresh0 = outptr;
                    outptr = outptr.offset(1);
                    *fresh0 = '\0' as i32 as i8;
                    length = outptr.offset_from(result) as i64 as size_t;
                    if length < result_size {
                        let mut smaller_result: *mut i8 = realloc(
                            result as *mut libc::c_void,
                            length,
                        ) as *mut i8;
                        if !smaller_result.is_null() {
                            result = smaller_result;
                        }
                    }
                    return result;
                }
            }
        }
        _ => {}
    }
    rpl_free(result as *mut libc::c_void);
    return 0 as *mut i8;
}
#[no_mangle]
pub unsafe extern "C" fn str_iconv(
    mut src: *const i8,
    mut from_codeset: *const i8,
    mut to_codeset: *const i8,
) -> *mut i8 {
    if *src as i32 == '\0' as i32 || c_strcasecmp(from_codeset, to_codeset) == 0 as i32 {
        let mut result: *mut i8 = strdup(src);
        if result.is_null() {
            *__errno_location() = 12 as i32;
        }
        return result;
    } else {
        let mut cd: iconv_t = 0 as *mut libc::c_void;
        let mut result_0: *mut i8 = 0 as *mut i8;
        cd = iconv_open(to_codeset, from_codeset);
        if cd == -(1 as i32) as iconv_t {
            return 0 as *mut i8;
        }
        result_0 = str_cd_iconv(src, cd);
        if result_0.is_null() {
            let mut saved_errno: i32 = *__errno_location();
            iconv_close(cd);
            *__errno_location() = saved_errno;
        } else if iconv_close(cd) < 0 as i32 {
            rpl_free(result_0 as *mut libc::c_void);
            return 0 as *mut i8;
        }
        return result_0;
    };
}