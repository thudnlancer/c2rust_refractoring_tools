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
    fn gsl_error(reason: *const i8, file: *const i8, line: i32, gsl_errno: i32);
}
pub type size_t = u64;
pub type C2RustUnnamed = i32;
pub const GSL_EOF: C2RustUnnamed = 32;
pub const GSL_ETOLG: C2RustUnnamed = 31;
pub const GSL_ETOLX: C2RustUnnamed = 30;
pub const GSL_ETOLF: C2RustUnnamed = 29;
pub const GSL_ENOPROGJ: C2RustUnnamed = 28;
pub const GSL_ENOPROG: C2RustUnnamed = 27;
pub const GSL_ETABLE: C2RustUnnamed = 26;
pub const GSL_ECACHE: C2RustUnnamed = 25;
pub const GSL_EUNIMPL: C2RustUnnamed = 24;
pub const GSL_EUNSUP: C2RustUnnamed = 23;
pub const GSL_EDIVERGE: C2RustUnnamed = 22;
pub const GSL_ESING: C2RustUnnamed = 21;
pub const GSL_ENOTSQR: C2RustUnnamed = 20;
pub const GSL_EBADLEN: C2RustUnnamed = 19;
pub const GSL_EROUND: C2RustUnnamed = 18;
pub const GSL_ELOSS: C2RustUnnamed = 17;
pub const GSL_EOVRFLW: C2RustUnnamed = 16;
pub const GSL_EUNDRFLW: C2RustUnnamed = 15;
pub const GSL_ETOL: C2RustUnnamed = 14;
pub const GSL_EBADTOL: C2RustUnnamed = 13;
pub const GSL_EZERODIV: C2RustUnnamed = 12;
pub const GSL_EMAXITER: C2RustUnnamed = 11;
pub const GSL_ERUNAWAY: C2RustUnnamed = 10;
pub const GSL_EBADFUNC: C2RustUnnamed = 9;
pub const GSL_ENOMEM: C2RustUnnamed = 8;
pub const GSL_ESANITY: C2RustUnnamed = 7;
pub const GSL_EFACTOR: C2RustUnnamed = 6;
pub const GSL_EFAILED: C2RustUnnamed = 5;
pub const GSL_EINVAL: C2RustUnnamed = 4;
pub const GSL_EFAULT: C2RustUnnamed = 3;
pub const GSL_ERANGE: C2RustUnnamed = 2;
pub const GSL_EDOM: C2RustUnnamed = 1;
pub const GSL_CONTINUE: C2RustUnnamed = -2;
pub const GSL_FAILURE: C2RustUnnamed = -1;
pub const GSL_SUCCESS: C2RustUnnamed = 0;
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_ulong_select(
    mut data: *mut u64,
    stride: size_t,
    n: size_t,
    k: size_t,
) -> u64 {
    if n == 0 as i32 as u64 {
        gsl_error(
            b"array size must be positive\0" as *const u8 as *const i8,
            b"./select_source.c\0" as *const u8 as *const i8,
            43 as i32,
            GSL_EBADLEN as i32,
        );
        return 0.0f64 as u64;
    } else {
        let mut left: size_t = 0 as i32 as size_t;
        let mut right: size_t = n.wrapping_sub(1 as i32 as u64);
        let mut mid: size_t = 0;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        let mut pivot: u64 = 0;
        let mut tmp: u64 = 0;
        loop {
            if right <= left.wrapping_add(1 as i32 as u64) {
                if right == left.wrapping_add(1 as i32 as u64)
                    && *data.offset(right.wrapping_mul(stride) as isize)
                        < *data.offset(left.wrapping_mul(stride) as isize)
                {
                    tmp = *data.offset(right.wrapping_mul(stride) as isize);
                    *data.offset(right.wrapping_mul(stride) as isize) = *data
                        .offset(left.wrapping_mul(stride) as isize);
                    *data.offset(left.wrapping_mul(stride) as isize) = tmp;
                }
                return *data.offset(k.wrapping_mul(stride) as isize);
            } else {
                mid = left.wrapping_add(right) >> 1 as i32;
                tmp = *data
                    .offset(
                        left.wrapping_add(1 as i32 as u64).wrapping_mul(stride) as isize,
                    );
                *data
                    .offset(
                        left.wrapping_add(1 as i32 as u64).wrapping_mul(stride) as isize,
                    ) = *data.offset(mid.wrapping_mul(stride) as isize);
                *data.offset(mid.wrapping_mul(stride) as isize) = tmp;
                if *data.offset(left.wrapping_mul(stride) as isize)
                    > *data.offset(right.wrapping_mul(stride) as isize)
                {
                    tmp = *data.offset(right.wrapping_mul(stride) as isize);
                    *data.offset(right.wrapping_mul(stride) as isize) = *data
                        .offset(left.wrapping_mul(stride) as isize);
                    *data.offset(left.wrapping_mul(stride) as isize) = tmp;
                }
                if *data
                    .offset(
                        left.wrapping_add(1 as i32 as u64).wrapping_mul(stride) as isize,
                    ) > *data.offset(right.wrapping_mul(stride) as isize)
                {
                    tmp = *data.offset(right.wrapping_mul(stride) as isize);
                    *data.offset(right.wrapping_mul(stride) as isize) = *data
                        .offset(
                            left.wrapping_add(1 as i32 as u64).wrapping_mul(stride)
                                as isize,
                        );
                    *data
                        .offset(
                            left.wrapping_add(1 as i32 as u64).wrapping_mul(stride)
                                as isize,
                        ) = tmp;
                }
                if *data.offset(left.wrapping_mul(stride) as isize)
                    > *data
                        .offset(
                            left.wrapping_add(1 as i32 as u64).wrapping_mul(stride)
                                as isize,
                        )
                {
                    tmp = *data
                        .offset(
                            left.wrapping_add(1 as i32 as u64).wrapping_mul(stride)
                                as isize,
                        );
                    *data
                        .offset(
                            left.wrapping_add(1 as i32 as u64).wrapping_mul(stride)
                                as isize,
                        ) = *data.offset(left.wrapping_mul(stride) as isize);
                    *data.offset(left.wrapping_mul(stride) as isize) = tmp;
                }
                i = left.wrapping_add(1 as i32 as u64);
                j = right;
                pivot = *data
                    .offset(
                        left.wrapping_add(1 as i32 as u64).wrapping_mul(stride) as isize,
                    );
                loop {
                    loop {
                        i = i.wrapping_add(1);
                        i;
                        if !(*data.offset(i.wrapping_mul(stride) as isize) < pivot) {
                            break;
                        }
                    }
                    loop {
                        j = j.wrapping_sub(1);
                        j;
                        if !(*data.offset(j.wrapping_mul(stride) as isize) > pivot) {
                            break;
                        }
                    }
                    if j < i {
                        break;
                    }
                    tmp = *data.offset(j.wrapping_mul(stride) as isize);
                    *data.offset(j.wrapping_mul(stride) as isize) = *data
                        .offset(i.wrapping_mul(stride) as isize);
                    *data.offset(i.wrapping_mul(stride) as isize) = tmp;
                }
                *data
                    .offset(
                        left.wrapping_add(1 as i32 as u64).wrapping_mul(stride) as isize,
                    ) = *data.offset(j.wrapping_mul(stride) as isize);
                *data.offset(j.wrapping_mul(stride) as isize) = pivot;
                if j >= k {
                    right = j.wrapping_sub(1 as i32 as u64);
                }
                if j <= k {
                    left = i;
                }
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_uchar_select(
    mut data: *mut u8,
    stride: size_t,
    n: size_t,
    k: size_t,
) -> u8 {
    if n == 0 as i32 as u64 {
        gsl_error(
            b"array size must be positive\0" as *const u8 as *const i8,
            b"./select_source.c\0" as *const u8 as *const i8,
            43 as i32,
            GSL_EBADLEN as i32,
        );
        return 0.0f64 as u8;
    } else {
        let mut left: size_t = 0 as i32 as size_t;
        let mut right: size_t = n.wrapping_sub(1 as i32 as u64);
        let mut mid: size_t = 0;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        let mut pivot: u8 = 0;
        let mut tmp: u8 = 0;
        loop {
            if right <= left.wrapping_add(1 as i32 as u64) {
                if right == left.wrapping_add(1 as i32 as u64)
                    && (*data.offset(right.wrapping_mul(stride) as isize) as i32)
                        < *data.offset(left.wrapping_mul(stride) as isize) as i32
                {
                    tmp = *data.offset(right.wrapping_mul(stride) as isize);
                    *data.offset(right.wrapping_mul(stride) as isize) = *data
                        .offset(left.wrapping_mul(stride) as isize);
                    *data.offset(left.wrapping_mul(stride) as isize) = tmp;
                }
                return *data.offset(k.wrapping_mul(stride) as isize);
            } else {
                mid = left.wrapping_add(right) >> 1 as i32;
                tmp = *data
                    .offset(
                        left.wrapping_add(1 as i32 as u64).wrapping_mul(stride) as isize,
                    );
                *data
                    .offset(
                        left.wrapping_add(1 as i32 as u64).wrapping_mul(stride) as isize,
                    ) = *data.offset(mid.wrapping_mul(stride) as isize);
                *data.offset(mid.wrapping_mul(stride) as isize) = tmp;
                if *data.offset(left.wrapping_mul(stride) as isize) as i32
                    > *data.offset(right.wrapping_mul(stride) as isize) as i32
                {
                    tmp = *data.offset(right.wrapping_mul(stride) as isize);
                    *data.offset(right.wrapping_mul(stride) as isize) = *data
                        .offset(left.wrapping_mul(stride) as isize);
                    *data.offset(left.wrapping_mul(stride) as isize) = tmp;
                }
                if *data
                    .offset(
                        left.wrapping_add(1 as i32 as u64).wrapping_mul(stride) as isize,
                    ) as i32 > *data.offset(right.wrapping_mul(stride) as isize) as i32
                {
                    tmp = *data.offset(right.wrapping_mul(stride) as isize);
                    *data.offset(right.wrapping_mul(stride) as isize) = *data
                        .offset(
                            left.wrapping_add(1 as i32 as u64).wrapping_mul(stride)
                                as isize,
                        );
                    *data
                        .offset(
                            left.wrapping_add(1 as i32 as u64).wrapping_mul(stride)
                                as isize,
                        ) = tmp;
                }
                if *data.offset(left.wrapping_mul(stride) as isize) as i32
                    > *data
                        .offset(
                            left.wrapping_add(1 as i32 as u64).wrapping_mul(stride)
                                as isize,
                        ) as i32
                {
                    tmp = *data
                        .offset(
                            left.wrapping_add(1 as i32 as u64).wrapping_mul(stride)
                                as isize,
                        );
                    *data
                        .offset(
                            left.wrapping_add(1 as i32 as u64).wrapping_mul(stride)
                                as isize,
                        ) = *data.offset(left.wrapping_mul(stride) as isize);
                    *data.offset(left.wrapping_mul(stride) as isize) = tmp;
                }
                i = left.wrapping_add(1 as i32 as u64);
                j = right;
                pivot = *data
                    .offset(
                        left.wrapping_add(1 as i32 as u64).wrapping_mul(stride) as isize,
                    );
                loop {
                    loop {
                        i = i.wrapping_add(1);
                        i;
                        if !((*data.offset(i.wrapping_mul(stride) as isize) as i32)
                            < pivot as i32)
                        {
                            break;
                        }
                    }
                    loop {
                        j = j.wrapping_sub(1);
                        j;
                        if !(*data.offset(j.wrapping_mul(stride) as isize) as i32
                            > pivot as i32)
                        {
                            break;
                        }
                    }
                    if j < i {
                        break;
                    }
                    tmp = *data.offset(j.wrapping_mul(stride) as isize);
                    *data.offset(j.wrapping_mul(stride) as isize) = *data
                        .offset(i.wrapping_mul(stride) as isize);
                    *data.offset(i.wrapping_mul(stride) as isize) = tmp;
                }
                *data
                    .offset(
                        left.wrapping_add(1 as i32 as u64).wrapping_mul(stride) as isize,
                    ) = *data.offset(j.wrapping_mul(stride) as isize);
                *data.offset(j.wrapping_mul(stride) as isize) = pivot;
                if j >= k {
                    right = j.wrapping_sub(1 as i32 as u64);
                }
                if j <= k {
                    left = i;
                }
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_long_select(
    mut data: *mut i64,
    stride: size_t,
    n: size_t,
    k: size_t,
) -> i64 {
    if n == 0 as i32 as u64 {
        gsl_error(
            b"array size must be positive\0" as *const u8 as *const i8,
            b"./select_source.c\0" as *const u8 as *const i8,
            43 as i32,
            GSL_EBADLEN as i32,
        );
        return 0.0f64 as i64;
    } else {
        let mut left: size_t = 0 as i32 as size_t;
        let mut right: size_t = n.wrapping_sub(1 as i32 as u64);
        let mut mid: size_t = 0;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        let mut pivot: i64 = 0;
        let mut tmp: i64 = 0;
        loop {
            if right <= left.wrapping_add(1 as i32 as u64) {
                if right == left.wrapping_add(1 as i32 as u64)
                    && *data.offset(right.wrapping_mul(stride) as isize)
                        < *data.offset(left.wrapping_mul(stride) as isize)
                {
                    tmp = *data.offset(right.wrapping_mul(stride) as isize);
                    *data.offset(right.wrapping_mul(stride) as isize) = *data
                        .offset(left.wrapping_mul(stride) as isize);
                    *data.offset(left.wrapping_mul(stride) as isize) = tmp;
                }
                return *data.offset(k.wrapping_mul(stride) as isize);
            } else {
                mid = left.wrapping_add(right) >> 1 as i32;
                tmp = *data
                    .offset(
                        left.wrapping_add(1 as i32 as u64).wrapping_mul(stride) as isize,
                    );
                *data
                    .offset(
                        left.wrapping_add(1 as i32 as u64).wrapping_mul(stride) as isize,
                    ) = *data.offset(mid.wrapping_mul(stride) as isize);
                *data.offset(mid.wrapping_mul(stride) as isize) = tmp;
                if *data.offset(left.wrapping_mul(stride) as isize)
                    > *data.offset(right.wrapping_mul(stride) as isize)
                {
                    tmp = *data.offset(right.wrapping_mul(stride) as isize);
                    *data.offset(right.wrapping_mul(stride) as isize) = *data
                        .offset(left.wrapping_mul(stride) as isize);
                    *data.offset(left.wrapping_mul(stride) as isize) = tmp;
                }
                if *data
                    .offset(
                        left.wrapping_add(1 as i32 as u64).wrapping_mul(stride) as isize,
                    ) > *data.offset(right.wrapping_mul(stride) as isize)
                {
                    tmp = *data.offset(right.wrapping_mul(stride) as isize);
                    *data.offset(right.wrapping_mul(stride) as isize) = *data
                        .offset(
                            left.wrapping_add(1 as i32 as u64).wrapping_mul(stride)
                                as isize,
                        );
                    *data
                        .offset(
                            left.wrapping_add(1 as i32 as u64).wrapping_mul(stride)
                                as isize,
                        ) = tmp;
                }
                if *data.offset(left.wrapping_mul(stride) as isize)
                    > *data
                        .offset(
                            left.wrapping_add(1 as i32 as u64).wrapping_mul(stride)
                                as isize,
                        )
                {
                    tmp = *data
                        .offset(
                            left.wrapping_add(1 as i32 as u64).wrapping_mul(stride)
                                as isize,
                        );
                    *data
                        .offset(
                            left.wrapping_add(1 as i32 as u64).wrapping_mul(stride)
                                as isize,
                        ) = *data.offset(left.wrapping_mul(stride) as isize);
                    *data.offset(left.wrapping_mul(stride) as isize) = tmp;
                }
                i = left.wrapping_add(1 as i32 as u64);
                j = right;
                pivot = *data
                    .offset(
                        left.wrapping_add(1 as i32 as u64).wrapping_mul(stride) as isize,
                    );
                loop {
                    loop {
                        i = i.wrapping_add(1);
                        i;
                        if !(*data.offset(i.wrapping_mul(stride) as isize) < pivot) {
                            break;
                        }
                    }
                    loop {
                        j = j.wrapping_sub(1);
                        j;
                        if !(*data.offset(j.wrapping_mul(stride) as isize) > pivot) {
                            break;
                        }
                    }
                    if j < i {
                        break;
                    }
                    tmp = *data.offset(j.wrapping_mul(stride) as isize);
                    *data.offset(j.wrapping_mul(stride) as isize) = *data
                        .offset(i.wrapping_mul(stride) as isize);
                    *data.offset(i.wrapping_mul(stride) as isize) = tmp;
                }
                *data
                    .offset(
                        left.wrapping_add(1 as i32 as u64).wrapping_mul(stride) as isize,
                    ) = *data.offset(j.wrapping_mul(stride) as isize);
                *data.offset(j.wrapping_mul(stride) as isize) = pivot;
                if j >= k {
                    right = j.wrapping_sub(1 as i32 as u64);
                }
                if j <= k {
                    left = i;
                }
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_char_select(
    mut data: *mut i8,
    stride: size_t,
    n: size_t,
    k: size_t,
) -> i8 {
    if n == 0 as i32 as u64 {
        gsl_error(
            b"array size must be positive\0" as *const u8 as *const i8,
            b"./select_source.c\0" as *const u8 as *const i8,
            43 as i32,
            GSL_EBADLEN as i32,
        );
        return 0.0f64 as i8;
    } else {
        let mut left: size_t = 0 as i32 as size_t;
        let mut right: size_t = n.wrapping_sub(1 as i32 as u64);
        let mut mid: size_t = 0;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        let mut pivot: i8 = 0;
        let mut tmp: i8 = 0;
        loop {
            if right <= left.wrapping_add(1 as i32 as u64) {
                if right == left.wrapping_add(1 as i32 as u64)
                    && (*data.offset(right.wrapping_mul(stride) as isize) as i32)
                        < *data.offset(left.wrapping_mul(stride) as isize) as i32
                {
                    tmp = *data.offset(right.wrapping_mul(stride) as isize);
                    *data.offset(right.wrapping_mul(stride) as isize) = *data
                        .offset(left.wrapping_mul(stride) as isize);
                    *data.offset(left.wrapping_mul(stride) as isize) = tmp;
                }
                return *data.offset(k.wrapping_mul(stride) as isize);
            } else {
                mid = left.wrapping_add(right) >> 1 as i32;
                tmp = *data
                    .offset(
                        left.wrapping_add(1 as i32 as u64).wrapping_mul(stride) as isize,
                    );
                *data
                    .offset(
                        left.wrapping_add(1 as i32 as u64).wrapping_mul(stride) as isize,
                    ) = *data.offset(mid.wrapping_mul(stride) as isize);
                *data.offset(mid.wrapping_mul(stride) as isize) = tmp;
                if *data.offset(left.wrapping_mul(stride) as isize) as i32
                    > *data.offset(right.wrapping_mul(stride) as isize) as i32
                {
                    tmp = *data.offset(right.wrapping_mul(stride) as isize);
                    *data.offset(right.wrapping_mul(stride) as isize) = *data
                        .offset(left.wrapping_mul(stride) as isize);
                    *data.offset(left.wrapping_mul(stride) as isize) = tmp;
                }
                if *data
                    .offset(
                        left.wrapping_add(1 as i32 as u64).wrapping_mul(stride) as isize,
                    ) as i32 > *data.offset(right.wrapping_mul(stride) as isize) as i32
                {
                    tmp = *data.offset(right.wrapping_mul(stride) as isize);
                    *data.offset(right.wrapping_mul(stride) as isize) = *data
                        .offset(
                            left.wrapping_add(1 as i32 as u64).wrapping_mul(stride)
                                as isize,
                        );
                    *data
                        .offset(
                            left.wrapping_add(1 as i32 as u64).wrapping_mul(stride)
                                as isize,
                        ) = tmp;
                }
                if *data.offset(left.wrapping_mul(stride) as isize) as i32
                    > *data
                        .offset(
                            left.wrapping_add(1 as i32 as u64).wrapping_mul(stride)
                                as isize,
                        ) as i32
                {
                    tmp = *data
                        .offset(
                            left.wrapping_add(1 as i32 as u64).wrapping_mul(stride)
                                as isize,
                        );
                    *data
                        .offset(
                            left.wrapping_add(1 as i32 as u64).wrapping_mul(stride)
                                as isize,
                        ) = *data.offset(left.wrapping_mul(stride) as isize);
                    *data.offset(left.wrapping_mul(stride) as isize) = tmp;
                }
                i = left.wrapping_add(1 as i32 as u64);
                j = right;
                pivot = *data
                    .offset(
                        left.wrapping_add(1 as i32 as u64).wrapping_mul(stride) as isize,
                    );
                loop {
                    loop {
                        i = i.wrapping_add(1);
                        i;
                        if !((*data.offset(i.wrapping_mul(stride) as isize) as i32)
                            < pivot as i32)
                        {
                            break;
                        }
                    }
                    loop {
                        j = j.wrapping_sub(1);
                        j;
                        if !(*data.offset(j.wrapping_mul(stride) as isize) as i32
                            > pivot as i32)
                        {
                            break;
                        }
                    }
                    if j < i {
                        break;
                    }
                    tmp = *data.offset(j.wrapping_mul(stride) as isize);
                    *data.offset(j.wrapping_mul(stride) as isize) = *data
                        .offset(i.wrapping_mul(stride) as isize);
                    *data.offset(i.wrapping_mul(stride) as isize) = tmp;
                }
                *data
                    .offset(
                        left.wrapping_add(1 as i32 as u64).wrapping_mul(stride) as isize,
                    ) = *data.offset(j.wrapping_mul(stride) as isize);
                *data.offset(j.wrapping_mul(stride) as isize) = pivot;
                if j >= k {
                    right = j.wrapping_sub(1 as i32 as u64);
                }
                if j <= k {
                    left = i;
                }
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_uint_select(
    mut data: *mut u32,
    stride: size_t,
    n: size_t,
    k: size_t,
) -> u32 {
    if n == 0 as i32 as u64 {
        gsl_error(
            b"array size must be positive\0" as *const u8 as *const i8,
            b"./select_source.c\0" as *const u8 as *const i8,
            43 as i32,
            GSL_EBADLEN as i32,
        );
        return 0.0f64 as u32;
    } else {
        let mut left: size_t = 0 as i32 as size_t;
        let mut right: size_t = n.wrapping_sub(1 as i32 as u64);
        let mut mid: size_t = 0;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        let mut pivot: u32 = 0;
        let mut tmp: u32 = 0;
        loop {
            if right <= left.wrapping_add(1 as i32 as u64) {
                if right == left.wrapping_add(1 as i32 as u64)
                    && *data.offset(right.wrapping_mul(stride) as isize)
                        < *data.offset(left.wrapping_mul(stride) as isize)
                {
                    tmp = *data.offset(right.wrapping_mul(stride) as isize);
                    *data.offset(right.wrapping_mul(stride) as isize) = *data
                        .offset(left.wrapping_mul(stride) as isize);
                    *data.offset(left.wrapping_mul(stride) as isize) = tmp;
                }
                return *data.offset(k.wrapping_mul(stride) as isize);
            } else {
                mid = left.wrapping_add(right) >> 1 as i32;
                tmp = *data
                    .offset(
                        left.wrapping_add(1 as i32 as u64).wrapping_mul(stride) as isize,
                    );
                *data
                    .offset(
                        left.wrapping_add(1 as i32 as u64).wrapping_mul(stride) as isize,
                    ) = *data.offset(mid.wrapping_mul(stride) as isize);
                *data.offset(mid.wrapping_mul(stride) as isize) = tmp;
                if *data.offset(left.wrapping_mul(stride) as isize)
                    > *data.offset(right.wrapping_mul(stride) as isize)
                {
                    tmp = *data.offset(right.wrapping_mul(stride) as isize);
                    *data.offset(right.wrapping_mul(stride) as isize) = *data
                        .offset(left.wrapping_mul(stride) as isize);
                    *data.offset(left.wrapping_mul(stride) as isize) = tmp;
                }
                if *data
                    .offset(
                        left.wrapping_add(1 as i32 as u64).wrapping_mul(stride) as isize,
                    ) > *data.offset(right.wrapping_mul(stride) as isize)
                {
                    tmp = *data.offset(right.wrapping_mul(stride) as isize);
                    *data.offset(right.wrapping_mul(stride) as isize) = *data
                        .offset(
                            left.wrapping_add(1 as i32 as u64).wrapping_mul(stride)
                                as isize,
                        );
                    *data
                        .offset(
                            left.wrapping_add(1 as i32 as u64).wrapping_mul(stride)
                                as isize,
                        ) = tmp;
                }
                if *data.offset(left.wrapping_mul(stride) as isize)
                    > *data
                        .offset(
                            left.wrapping_add(1 as i32 as u64).wrapping_mul(stride)
                                as isize,
                        )
                {
                    tmp = *data
                        .offset(
                            left.wrapping_add(1 as i32 as u64).wrapping_mul(stride)
                                as isize,
                        );
                    *data
                        .offset(
                            left.wrapping_add(1 as i32 as u64).wrapping_mul(stride)
                                as isize,
                        ) = *data.offset(left.wrapping_mul(stride) as isize);
                    *data.offset(left.wrapping_mul(stride) as isize) = tmp;
                }
                i = left.wrapping_add(1 as i32 as u64);
                j = right;
                pivot = *data
                    .offset(
                        left.wrapping_add(1 as i32 as u64).wrapping_mul(stride) as isize,
                    );
                loop {
                    loop {
                        i = i.wrapping_add(1);
                        i;
                        if !(*data.offset(i.wrapping_mul(stride) as isize) < pivot) {
                            break;
                        }
                    }
                    loop {
                        j = j.wrapping_sub(1);
                        j;
                        if !(*data.offset(j.wrapping_mul(stride) as isize) > pivot) {
                            break;
                        }
                    }
                    if j < i {
                        break;
                    }
                    tmp = *data.offset(j.wrapping_mul(stride) as isize);
                    *data.offset(j.wrapping_mul(stride) as isize) = *data
                        .offset(i.wrapping_mul(stride) as isize);
                    *data.offset(i.wrapping_mul(stride) as isize) = tmp;
                }
                *data
                    .offset(
                        left.wrapping_add(1 as i32 as u64).wrapping_mul(stride) as isize,
                    ) = *data.offset(j.wrapping_mul(stride) as isize);
                *data.offset(j.wrapping_mul(stride) as isize) = pivot;
                if j >= k {
                    right = j.wrapping_sub(1 as i32 as u64);
                }
                if j <= k {
                    left = i;
                }
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_int_select(
    mut data: *mut i32,
    stride: size_t,
    n: size_t,
    k: size_t,
) -> i32 {
    if n == 0 as i32 as u64 {
        gsl_error(
            b"array size must be positive\0" as *const u8 as *const i8,
            b"./select_source.c\0" as *const u8 as *const i8,
            43 as i32,
            GSL_EBADLEN as i32,
        );
        return 0.0f64 as i32;
    } else {
        let mut left: size_t = 0 as i32 as size_t;
        let mut right: size_t = n.wrapping_sub(1 as i32 as u64);
        let mut mid: size_t = 0;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        let mut pivot: i32 = 0;
        let mut tmp: i32 = 0;
        loop {
            if right <= left.wrapping_add(1 as i32 as u64) {
                if right == left.wrapping_add(1 as i32 as u64)
                    && *data.offset(right.wrapping_mul(stride) as isize)
                        < *data.offset(left.wrapping_mul(stride) as isize)
                {
                    tmp = *data.offset(right.wrapping_mul(stride) as isize);
                    *data.offset(right.wrapping_mul(stride) as isize) = *data
                        .offset(left.wrapping_mul(stride) as isize);
                    *data.offset(left.wrapping_mul(stride) as isize) = tmp;
                }
                return *data.offset(k.wrapping_mul(stride) as isize);
            } else {
                mid = left.wrapping_add(right) >> 1 as i32;
                tmp = *data
                    .offset(
                        left.wrapping_add(1 as i32 as u64).wrapping_mul(stride) as isize,
                    );
                *data
                    .offset(
                        left.wrapping_add(1 as i32 as u64).wrapping_mul(stride) as isize,
                    ) = *data.offset(mid.wrapping_mul(stride) as isize);
                *data.offset(mid.wrapping_mul(stride) as isize) = tmp;
                if *data.offset(left.wrapping_mul(stride) as isize)
                    > *data.offset(right.wrapping_mul(stride) as isize)
                {
                    tmp = *data.offset(right.wrapping_mul(stride) as isize);
                    *data.offset(right.wrapping_mul(stride) as isize) = *data
                        .offset(left.wrapping_mul(stride) as isize);
                    *data.offset(left.wrapping_mul(stride) as isize) = tmp;
                }
                if *data
                    .offset(
                        left.wrapping_add(1 as i32 as u64).wrapping_mul(stride) as isize,
                    ) > *data.offset(right.wrapping_mul(stride) as isize)
                {
                    tmp = *data.offset(right.wrapping_mul(stride) as isize);
                    *data.offset(right.wrapping_mul(stride) as isize) = *data
                        .offset(
                            left.wrapping_add(1 as i32 as u64).wrapping_mul(stride)
                                as isize,
                        );
                    *data
                        .offset(
                            left.wrapping_add(1 as i32 as u64).wrapping_mul(stride)
                                as isize,
                        ) = tmp;
                }
                if *data.offset(left.wrapping_mul(stride) as isize)
                    > *data
                        .offset(
                            left.wrapping_add(1 as i32 as u64).wrapping_mul(stride)
                                as isize,
                        )
                {
                    tmp = *data
                        .offset(
                            left.wrapping_add(1 as i32 as u64).wrapping_mul(stride)
                                as isize,
                        );
                    *data
                        .offset(
                            left.wrapping_add(1 as i32 as u64).wrapping_mul(stride)
                                as isize,
                        ) = *data.offset(left.wrapping_mul(stride) as isize);
                    *data.offset(left.wrapping_mul(stride) as isize) = tmp;
                }
                i = left.wrapping_add(1 as i32 as u64);
                j = right;
                pivot = *data
                    .offset(
                        left.wrapping_add(1 as i32 as u64).wrapping_mul(stride) as isize,
                    );
                loop {
                    loop {
                        i = i.wrapping_add(1);
                        i;
                        if !(*data.offset(i.wrapping_mul(stride) as isize) < pivot) {
                            break;
                        }
                    }
                    loop {
                        j = j.wrapping_sub(1);
                        j;
                        if !(*data.offset(j.wrapping_mul(stride) as isize) > pivot) {
                            break;
                        }
                    }
                    if j < i {
                        break;
                    }
                    tmp = *data.offset(j.wrapping_mul(stride) as isize);
                    *data.offset(j.wrapping_mul(stride) as isize) = *data
                        .offset(i.wrapping_mul(stride) as isize);
                    *data.offset(i.wrapping_mul(stride) as isize) = tmp;
                }
                *data
                    .offset(
                        left.wrapping_add(1 as i32 as u64).wrapping_mul(stride) as isize,
                    ) = *data.offset(j.wrapping_mul(stride) as isize);
                *data.offset(j.wrapping_mul(stride) as isize) = pivot;
                if j >= k {
                    right = j.wrapping_sub(1 as i32 as u64);
                }
                if j <= k {
                    left = i;
                }
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_ushort_select(
    mut data: *mut libc::c_ushort,
    stride: size_t,
    n: size_t,
    k: size_t,
) -> libc::c_ushort {
    if n == 0 as i32 as u64 {
        gsl_error(
            b"array size must be positive\0" as *const u8 as *const i8,
            b"./select_source.c\0" as *const u8 as *const i8,
            43 as i32,
            GSL_EBADLEN as i32,
        );
        return 0.0f64 as libc::c_ushort;
    } else {
        let mut left: size_t = 0 as i32 as size_t;
        let mut right: size_t = n.wrapping_sub(1 as i32 as u64);
        let mut mid: size_t = 0;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        let mut pivot: libc::c_ushort = 0;
        let mut tmp: libc::c_ushort = 0;
        loop {
            if right <= left.wrapping_add(1 as i32 as u64) {
                if right == left.wrapping_add(1 as i32 as u64)
                    && (*data.offset(right.wrapping_mul(stride) as isize) as i32)
                        < *data.offset(left.wrapping_mul(stride) as isize) as i32
                {
                    tmp = *data.offset(right.wrapping_mul(stride) as isize);
                    *data.offset(right.wrapping_mul(stride) as isize) = *data
                        .offset(left.wrapping_mul(stride) as isize);
                    *data.offset(left.wrapping_mul(stride) as isize) = tmp;
                }
                return *data.offset(k.wrapping_mul(stride) as isize);
            } else {
                mid = left.wrapping_add(right) >> 1 as i32;
                tmp = *data
                    .offset(
                        left.wrapping_add(1 as i32 as u64).wrapping_mul(stride) as isize,
                    );
                *data
                    .offset(
                        left.wrapping_add(1 as i32 as u64).wrapping_mul(stride) as isize,
                    ) = *data.offset(mid.wrapping_mul(stride) as isize);
                *data.offset(mid.wrapping_mul(stride) as isize) = tmp;
                if *data.offset(left.wrapping_mul(stride) as isize) as i32
                    > *data.offset(right.wrapping_mul(stride) as isize) as i32
                {
                    tmp = *data.offset(right.wrapping_mul(stride) as isize);
                    *data.offset(right.wrapping_mul(stride) as isize) = *data
                        .offset(left.wrapping_mul(stride) as isize);
                    *data.offset(left.wrapping_mul(stride) as isize) = tmp;
                }
                if *data
                    .offset(
                        left.wrapping_add(1 as i32 as u64).wrapping_mul(stride) as isize,
                    ) as i32 > *data.offset(right.wrapping_mul(stride) as isize) as i32
                {
                    tmp = *data.offset(right.wrapping_mul(stride) as isize);
                    *data.offset(right.wrapping_mul(stride) as isize) = *data
                        .offset(
                            left.wrapping_add(1 as i32 as u64).wrapping_mul(stride)
                                as isize,
                        );
                    *data
                        .offset(
                            left.wrapping_add(1 as i32 as u64).wrapping_mul(stride)
                                as isize,
                        ) = tmp;
                }
                if *data.offset(left.wrapping_mul(stride) as isize) as i32
                    > *data
                        .offset(
                            left.wrapping_add(1 as i32 as u64).wrapping_mul(stride)
                                as isize,
                        ) as i32
                {
                    tmp = *data
                        .offset(
                            left.wrapping_add(1 as i32 as u64).wrapping_mul(stride)
                                as isize,
                        );
                    *data
                        .offset(
                            left.wrapping_add(1 as i32 as u64).wrapping_mul(stride)
                                as isize,
                        ) = *data.offset(left.wrapping_mul(stride) as isize);
                    *data.offset(left.wrapping_mul(stride) as isize) = tmp;
                }
                i = left.wrapping_add(1 as i32 as u64);
                j = right;
                pivot = *data
                    .offset(
                        left.wrapping_add(1 as i32 as u64).wrapping_mul(stride) as isize,
                    );
                loop {
                    loop {
                        i = i.wrapping_add(1);
                        i;
                        if !((*data.offset(i.wrapping_mul(stride) as isize) as i32)
                            < pivot as i32)
                        {
                            break;
                        }
                    }
                    loop {
                        j = j.wrapping_sub(1);
                        j;
                        if !(*data.offset(j.wrapping_mul(stride) as isize) as i32
                            > pivot as i32)
                        {
                            break;
                        }
                    }
                    if j < i {
                        break;
                    }
                    tmp = *data.offset(j.wrapping_mul(stride) as isize);
                    *data.offset(j.wrapping_mul(stride) as isize) = *data
                        .offset(i.wrapping_mul(stride) as isize);
                    *data.offset(i.wrapping_mul(stride) as isize) = tmp;
                }
                *data
                    .offset(
                        left.wrapping_add(1 as i32 as u64).wrapping_mul(stride) as isize,
                    ) = *data.offset(j.wrapping_mul(stride) as isize);
                *data.offset(j.wrapping_mul(stride) as isize) = pivot;
                if j >= k {
                    right = j.wrapping_sub(1 as i32 as u64);
                }
                if j <= k {
                    left = i;
                }
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_short_select(
    mut data: *mut libc::c_short,
    stride: size_t,
    n: size_t,
    k: size_t,
) -> libc::c_short {
    if n == 0 as i32 as u64 {
        gsl_error(
            b"array size must be positive\0" as *const u8 as *const i8,
            b"./select_source.c\0" as *const u8 as *const i8,
            43 as i32,
            GSL_EBADLEN as i32,
        );
        return 0.0f64 as libc::c_short;
    } else {
        let mut left: size_t = 0 as i32 as size_t;
        let mut right: size_t = n.wrapping_sub(1 as i32 as u64);
        let mut mid: size_t = 0;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        let mut pivot: libc::c_short = 0;
        let mut tmp: libc::c_short = 0;
        loop {
            if right <= left.wrapping_add(1 as i32 as u64) {
                if right == left.wrapping_add(1 as i32 as u64)
                    && (*data.offset(right.wrapping_mul(stride) as isize) as i32)
                        < *data.offset(left.wrapping_mul(stride) as isize) as i32
                {
                    tmp = *data.offset(right.wrapping_mul(stride) as isize);
                    *data.offset(right.wrapping_mul(stride) as isize) = *data
                        .offset(left.wrapping_mul(stride) as isize);
                    *data.offset(left.wrapping_mul(stride) as isize) = tmp;
                }
                return *data.offset(k.wrapping_mul(stride) as isize);
            } else {
                mid = left.wrapping_add(right) >> 1 as i32;
                tmp = *data
                    .offset(
                        left.wrapping_add(1 as i32 as u64).wrapping_mul(stride) as isize,
                    );
                *data
                    .offset(
                        left.wrapping_add(1 as i32 as u64).wrapping_mul(stride) as isize,
                    ) = *data.offset(mid.wrapping_mul(stride) as isize);
                *data.offset(mid.wrapping_mul(stride) as isize) = tmp;
                if *data.offset(left.wrapping_mul(stride) as isize) as i32
                    > *data.offset(right.wrapping_mul(stride) as isize) as i32
                {
                    tmp = *data.offset(right.wrapping_mul(stride) as isize);
                    *data.offset(right.wrapping_mul(stride) as isize) = *data
                        .offset(left.wrapping_mul(stride) as isize);
                    *data.offset(left.wrapping_mul(stride) as isize) = tmp;
                }
                if *data
                    .offset(
                        left.wrapping_add(1 as i32 as u64).wrapping_mul(stride) as isize,
                    ) as i32 > *data.offset(right.wrapping_mul(stride) as isize) as i32
                {
                    tmp = *data.offset(right.wrapping_mul(stride) as isize);
                    *data.offset(right.wrapping_mul(stride) as isize) = *data
                        .offset(
                            left.wrapping_add(1 as i32 as u64).wrapping_mul(stride)
                                as isize,
                        );
                    *data
                        .offset(
                            left.wrapping_add(1 as i32 as u64).wrapping_mul(stride)
                                as isize,
                        ) = tmp;
                }
                if *data.offset(left.wrapping_mul(stride) as isize) as i32
                    > *data
                        .offset(
                            left.wrapping_add(1 as i32 as u64).wrapping_mul(stride)
                                as isize,
                        ) as i32
                {
                    tmp = *data
                        .offset(
                            left.wrapping_add(1 as i32 as u64).wrapping_mul(stride)
                                as isize,
                        );
                    *data
                        .offset(
                            left.wrapping_add(1 as i32 as u64).wrapping_mul(stride)
                                as isize,
                        ) = *data.offset(left.wrapping_mul(stride) as isize);
                    *data.offset(left.wrapping_mul(stride) as isize) = tmp;
                }
                i = left.wrapping_add(1 as i32 as u64);
                j = right;
                pivot = *data
                    .offset(
                        left.wrapping_add(1 as i32 as u64).wrapping_mul(stride) as isize,
                    );
                loop {
                    loop {
                        i = i.wrapping_add(1);
                        i;
                        if !((*data.offset(i.wrapping_mul(stride) as isize) as i32)
                            < pivot as i32)
                        {
                            break;
                        }
                    }
                    loop {
                        j = j.wrapping_sub(1);
                        j;
                        if !(*data.offset(j.wrapping_mul(stride) as isize) as i32
                            > pivot as i32)
                        {
                            break;
                        }
                    }
                    if j < i {
                        break;
                    }
                    tmp = *data.offset(j.wrapping_mul(stride) as isize);
                    *data.offset(j.wrapping_mul(stride) as isize) = *data
                        .offset(i.wrapping_mul(stride) as isize);
                    *data.offset(i.wrapping_mul(stride) as isize) = tmp;
                }
                *data
                    .offset(
                        left.wrapping_add(1 as i32 as u64).wrapping_mul(stride) as isize,
                    ) = *data.offset(j.wrapping_mul(stride) as isize);
                *data.offset(j.wrapping_mul(stride) as isize) = pivot;
                if j >= k {
                    right = j.wrapping_sub(1 as i32 as u64);
                }
                if j <= k {
                    left = i;
                }
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_float_select(
    mut data: *mut libc::c_float,
    stride: size_t,
    n: size_t,
    k: size_t,
) -> libc::c_float {
    if n == 0 as i32 as u64 {
        gsl_error(
            b"array size must be positive\0" as *const u8 as *const i8,
            b"./select_source.c\0" as *const u8 as *const i8,
            43 as i32,
            GSL_EBADLEN as i32,
        );
        return 0.0f64 as libc::c_float;
    } else {
        let mut left: size_t = 0 as i32 as size_t;
        let mut right: size_t = n.wrapping_sub(1 as i32 as u64);
        let mut mid: size_t = 0;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        let mut pivot: libc::c_float = 0.;
        let mut tmp: libc::c_float = 0.;
        loop {
            if right <= left.wrapping_add(1 as i32 as u64) {
                if right == left.wrapping_add(1 as i32 as u64)
                    && *data.offset(right.wrapping_mul(stride) as isize)
                        < *data.offset(left.wrapping_mul(stride) as isize)
                {
                    tmp = *data.offset(right.wrapping_mul(stride) as isize);
                    *data.offset(right.wrapping_mul(stride) as isize) = *data
                        .offset(left.wrapping_mul(stride) as isize);
                    *data.offset(left.wrapping_mul(stride) as isize) = tmp;
                }
                return *data.offset(k.wrapping_mul(stride) as isize);
            } else {
                mid = left.wrapping_add(right) >> 1 as i32;
                tmp = *data
                    .offset(
                        left.wrapping_add(1 as i32 as u64).wrapping_mul(stride) as isize,
                    );
                *data
                    .offset(
                        left.wrapping_add(1 as i32 as u64).wrapping_mul(stride) as isize,
                    ) = *data.offset(mid.wrapping_mul(stride) as isize);
                *data.offset(mid.wrapping_mul(stride) as isize) = tmp;
                if *data.offset(left.wrapping_mul(stride) as isize)
                    > *data.offset(right.wrapping_mul(stride) as isize)
                {
                    tmp = *data.offset(right.wrapping_mul(stride) as isize);
                    *data.offset(right.wrapping_mul(stride) as isize) = *data
                        .offset(left.wrapping_mul(stride) as isize);
                    *data.offset(left.wrapping_mul(stride) as isize) = tmp;
                }
                if *data
                    .offset(
                        left.wrapping_add(1 as i32 as u64).wrapping_mul(stride) as isize,
                    ) > *data.offset(right.wrapping_mul(stride) as isize)
                {
                    tmp = *data.offset(right.wrapping_mul(stride) as isize);
                    *data.offset(right.wrapping_mul(stride) as isize) = *data
                        .offset(
                            left.wrapping_add(1 as i32 as u64).wrapping_mul(stride)
                                as isize,
                        );
                    *data
                        .offset(
                            left.wrapping_add(1 as i32 as u64).wrapping_mul(stride)
                                as isize,
                        ) = tmp;
                }
                if *data.offset(left.wrapping_mul(stride) as isize)
                    > *data
                        .offset(
                            left.wrapping_add(1 as i32 as u64).wrapping_mul(stride)
                                as isize,
                        )
                {
                    tmp = *data
                        .offset(
                            left.wrapping_add(1 as i32 as u64).wrapping_mul(stride)
                                as isize,
                        );
                    *data
                        .offset(
                            left.wrapping_add(1 as i32 as u64).wrapping_mul(stride)
                                as isize,
                        ) = *data.offset(left.wrapping_mul(stride) as isize);
                    *data.offset(left.wrapping_mul(stride) as isize) = tmp;
                }
                i = left.wrapping_add(1 as i32 as u64);
                j = right;
                pivot = *data
                    .offset(
                        left.wrapping_add(1 as i32 as u64).wrapping_mul(stride) as isize,
                    );
                loop {
                    loop {
                        i = i.wrapping_add(1);
                        i;
                        if !(*data.offset(i.wrapping_mul(stride) as isize) < pivot) {
                            break;
                        }
                    }
                    loop {
                        j = j.wrapping_sub(1);
                        j;
                        if !(*data.offset(j.wrapping_mul(stride) as isize) > pivot) {
                            break;
                        }
                    }
                    if j < i {
                        break;
                    }
                    tmp = *data.offset(j.wrapping_mul(stride) as isize);
                    *data.offset(j.wrapping_mul(stride) as isize) = *data
                        .offset(i.wrapping_mul(stride) as isize);
                    *data.offset(i.wrapping_mul(stride) as isize) = tmp;
                }
                *data
                    .offset(
                        left.wrapping_add(1 as i32 as u64).wrapping_mul(stride) as isize,
                    ) = *data.offset(j.wrapping_mul(stride) as isize);
                *data.offset(j.wrapping_mul(stride) as isize) = pivot;
                if j >= k {
                    right = j.wrapping_sub(1 as i32 as u64);
                }
                if j <= k {
                    left = i;
                }
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_select(
    mut data: *mut libc::c_double,
    stride: size_t,
    n: size_t,
    k: size_t,
) -> libc::c_double {
    if n == 0 as i32 as u64 {
        gsl_error(
            b"array size must be positive\0" as *const u8 as *const i8,
            b"./select_source.c\0" as *const u8 as *const i8,
            43 as i32,
            GSL_EBADLEN as i32,
        );
        return 0.0f64;
    } else {
        let mut left: size_t = 0 as i32 as size_t;
        let mut right: size_t = n.wrapping_sub(1 as i32 as u64);
        let mut mid: size_t = 0;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        let mut pivot: libc::c_double = 0.;
        let mut tmp: libc::c_double = 0.;
        loop {
            if right <= left.wrapping_add(1 as i32 as u64) {
                if right == left.wrapping_add(1 as i32 as u64)
                    && *data.offset(right.wrapping_mul(stride) as isize)
                        < *data.offset(left.wrapping_mul(stride) as isize)
                {
                    tmp = *data.offset(right.wrapping_mul(stride) as isize);
                    *data.offset(right.wrapping_mul(stride) as isize) = *data
                        .offset(left.wrapping_mul(stride) as isize);
                    *data.offset(left.wrapping_mul(stride) as isize) = tmp;
                }
                return *data.offset(k.wrapping_mul(stride) as isize);
            } else {
                mid = left.wrapping_add(right) >> 1 as i32;
                tmp = *data
                    .offset(
                        left.wrapping_add(1 as i32 as u64).wrapping_mul(stride) as isize,
                    );
                *data
                    .offset(
                        left.wrapping_add(1 as i32 as u64).wrapping_mul(stride) as isize,
                    ) = *data.offset(mid.wrapping_mul(stride) as isize);
                *data.offset(mid.wrapping_mul(stride) as isize) = tmp;
                if *data.offset(left.wrapping_mul(stride) as isize)
                    > *data.offset(right.wrapping_mul(stride) as isize)
                {
                    tmp = *data.offset(right.wrapping_mul(stride) as isize);
                    *data.offset(right.wrapping_mul(stride) as isize) = *data
                        .offset(left.wrapping_mul(stride) as isize);
                    *data.offset(left.wrapping_mul(stride) as isize) = tmp;
                }
                if *data
                    .offset(
                        left.wrapping_add(1 as i32 as u64).wrapping_mul(stride) as isize,
                    ) > *data.offset(right.wrapping_mul(stride) as isize)
                {
                    tmp = *data.offset(right.wrapping_mul(stride) as isize);
                    *data.offset(right.wrapping_mul(stride) as isize) = *data
                        .offset(
                            left.wrapping_add(1 as i32 as u64).wrapping_mul(stride)
                                as isize,
                        );
                    *data
                        .offset(
                            left.wrapping_add(1 as i32 as u64).wrapping_mul(stride)
                                as isize,
                        ) = tmp;
                }
                if *data.offset(left.wrapping_mul(stride) as isize)
                    > *data
                        .offset(
                            left.wrapping_add(1 as i32 as u64).wrapping_mul(stride)
                                as isize,
                        )
                {
                    tmp = *data
                        .offset(
                            left.wrapping_add(1 as i32 as u64).wrapping_mul(stride)
                                as isize,
                        );
                    *data
                        .offset(
                            left.wrapping_add(1 as i32 as u64).wrapping_mul(stride)
                                as isize,
                        ) = *data.offset(left.wrapping_mul(stride) as isize);
                    *data.offset(left.wrapping_mul(stride) as isize) = tmp;
                }
                i = left.wrapping_add(1 as i32 as u64);
                j = right;
                pivot = *data
                    .offset(
                        left.wrapping_add(1 as i32 as u64).wrapping_mul(stride) as isize,
                    );
                loop {
                    loop {
                        i = i.wrapping_add(1);
                        i;
                        if !(*data.offset(i.wrapping_mul(stride) as isize) < pivot) {
                            break;
                        }
                    }
                    loop {
                        j = j.wrapping_sub(1);
                        j;
                        if !(*data.offset(j.wrapping_mul(stride) as isize) > pivot) {
                            break;
                        }
                    }
                    if j < i {
                        break;
                    }
                    tmp = *data.offset(j.wrapping_mul(stride) as isize);
                    *data.offset(j.wrapping_mul(stride) as isize) = *data
                        .offset(i.wrapping_mul(stride) as isize);
                    *data.offset(i.wrapping_mul(stride) as isize) = tmp;
                }
                *data
                    .offset(
                        left.wrapping_add(1 as i32 as u64).wrapping_mul(stride) as isize,
                    ) = *data.offset(j.wrapping_mul(stride) as isize);
                *data.offset(j.wrapping_mul(stride) as isize) = pivot;
                if j >= k {
                    right = j.wrapping_sub(1 as i32 as u64);
                }
                if j <= k {
                    left = i;
                }
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn gsl_stats_long_double_select(
    mut data: *mut f128::f128,
    stride: size_t,
    n: size_t,
    k: size_t,
) -> f128::f128 {
    if n == 0 as i32 as u64 {
        gsl_error(
            b"array size must be positive\0" as *const u8 as *const i8,
            b"./select_source.c\0" as *const u8 as *const i8,
            43 as i32,
            GSL_EBADLEN as i32,
        );
        return f128::f128::new(0.0f64);
    } else {
        let mut left: size_t = 0 as i32 as size_t;
        let mut right: size_t = n.wrapping_sub(1 as i32 as u64);
        let mut mid: size_t = 0;
        let mut i: size_t = 0;
        let mut j: size_t = 0;
        let mut pivot: f128::f128 = f128::f128::ZERO;
        let mut tmp: f128::f128 = f128::f128::ZERO;
        loop {
            if right <= left.wrapping_add(1 as i32 as u64) {
                if right == left.wrapping_add(1 as i32 as u64)
                    && *data.offset(right.wrapping_mul(stride) as isize)
                        < *data.offset(left.wrapping_mul(stride) as isize)
                {
                    tmp = *data.offset(right.wrapping_mul(stride) as isize);
                    *data.offset(right.wrapping_mul(stride) as isize) = *data
                        .offset(left.wrapping_mul(stride) as isize);
                    *data.offset(left.wrapping_mul(stride) as isize) = tmp;
                }
                return *data.offset(k.wrapping_mul(stride) as isize);
            } else {
                mid = left.wrapping_add(right) >> 1 as i32;
                tmp = *data
                    .offset(
                        left.wrapping_add(1 as i32 as u64).wrapping_mul(stride) as isize,
                    );
                *data
                    .offset(
                        left.wrapping_add(1 as i32 as u64).wrapping_mul(stride) as isize,
                    ) = *data.offset(mid.wrapping_mul(stride) as isize);
                *data.offset(mid.wrapping_mul(stride) as isize) = tmp;
                if *data.offset(left.wrapping_mul(stride) as isize)
                    > *data.offset(right.wrapping_mul(stride) as isize)
                {
                    tmp = *data.offset(right.wrapping_mul(stride) as isize);
                    *data.offset(right.wrapping_mul(stride) as isize) = *data
                        .offset(left.wrapping_mul(stride) as isize);
                    *data.offset(left.wrapping_mul(stride) as isize) = tmp;
                }
                if *data
                    .offset(
                        left.wrapping_add(1 as i32 as u64).wrapping_mul(stride) as isize,
                    ) > *data.offset(right.wrapping_mul(stride) as isize)
                {
                    tmp = *data.offset(right.wrapping_mul(stride) as isize);
                    *data.offset(right.wrapping_mul(stride) as isize) = *data
                        .offset(
                            left.wrapping_add(1 as i32 as u64).wrapping_mul(stride)
                                as isize,
                        );
                    *data
                        .offset(
                            left.wrapping_add(1 as i32 as u64).wrapping_mul(stride)
                                as isize,
                        ) = tmp;
                }
                if *data.offset(left.wrapping_mul(stride) as isize)
                    > *data
                        .offset(
                            left.wrapping_add(1 as i32 as u64).wrapping_mul(stride)
                                as isize,
                        )
                {
                    tmp = *data
                        .offset(
                            left.wrapping_add(1 as i32 as u64).wrapping_mul(stride)
                                as isize,
                        );
                    *data
                        .offset(
                            left.wrapping_add(1 as i32 as u64).wrapping_mul(stride)
                                as isize,
                        ) = *data.offset(left.wrapping_mul(stride) as isize);
                    *data.offset(left.wrapping_mul(stride) as isize) = tmp;
                }
                i = left.wrapping_add(1 as i32 as u64);
                j = right;
                pivot = *data
                    .offset(
                        left.wrapping_add(1 as i32 as u64).wrapping_mul(stride) as isize,
                    );
                loop {
                    loop {
                        i = i.wrapping_add(1);
                        i;
                        if !(*data.offset(i.wrapping_mul(stride) as isize) < pivot) {
                            break;
                        }
                    }
                    loop {
                        j = j.wrapping_sub(1);
                        j;
                        if !(*data.offset(j.wrapping_mul(stride) as isize) > pivot) {
                            break;
                        }
                    }
                    if j < i {
                        break;
                    }
                    tmp = *data.offset(j.wrapping_mul(stride) as isize);
                    *data.offset(j.wrapping_mul(stride) as isize) = *data
                        .offset(i.wrapping_mul(stride) as isize);
                    *data.offset(i.wrapping_mul(stride) as isize) = tmp;
                }
                *data
                    .offset(
                        left.wrapping_add(1 as i32 as u64).wrapping_mul(stride) as isize,
                    ) = *data.offset(j.wrapping_mul(stride) as isize);
                *data.offset(j.wrapping_mul(stride) as isize) = pivot;
                if j >= k {
                    right = j.wrapping_sub(1 as i32 as u64);
                }
                if j <= k {
                    left = i;
                }
            }
        }
    };
}