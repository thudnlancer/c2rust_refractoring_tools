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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_permutation_struct {
    pub size: size_t,
    pub data: *mut size_t,
}
pub type gsl_permutation = gsl_permutation_struct;
#[no_mangle]
pub unsafe extern "C" fn gsl_permutation_size(mut p: *const gsl_permutation) -> size_t {
    return (*p).size;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_permutation_data(
    mut p: *const gsl_permutation,
) -> *mut size_t {
    return (*p).data;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_permutation_swap(
    mut p: *mut gsl_permutation,
    i: size_t,
    j: size_t,
) -> i32 {
    let size: size_t = (*p).size;
    if i >= size {
        gsl_error(
            b"first index is out of range\0" as *const u8 as *const i8,
            b"permutation.c\0" as *const u8 as *const i8,
            43 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    }
    if j >= size {
        gsl_error(
            b"second index is out of range\0" as *const u8 as *const i8,
            b"permutation.c\0" as *const u8 as *const i8,
            48 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    }
    if i != j {
        let mut tmp: size_t = *((*p).data).offset(i as isize);
        *((*p).data).offset(i as isize) = *((*p).data).offset(j as isize);
        *((*p).data).offset(j as isize) = tmp;
    }
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_permutation_valid(mut p: *const gsl_permutation) -> i32 {
    let size: size_t = (*p).size;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as i32 as size_t;
    while i < size {
        if *((*p).data).offset(i as isize) >= size {
            gsl_error(
                b"permutation index outside range\0" as *const u8 as *const i8,
                b"permutation.c\0" as *const u8 as *const i8,
                73 as i32,
                GSL_FAILURE as i32,
            );
            return GSL_FAILURE as i32;
        }
        j = 0 as i32 as size_t;
        while j < i {
            if *((*p).data).offset(i as isize) == *((*p).data).offset(j as isize) {
                gsl_error(
                    b"duplicate permutation index\0" as *const u8 as *const i8,
                    b"permutation.c\0" as *const u8 as *const i8,
                    80 as i32,
                    GSL_FAILURE as i32,
                );
                return GSL_FAILURE as i32;
            }
            j = j.wrapping_add(1);
            j;
        }
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_permutation_reverse(mut p: *mut gsl_permutation) {
    let size: size_t = (*p).size;
    let mut i: size_t = 0;
    i = 0 as i32 as size_t;
    while i < size.wrapping_div(2 as i32 as u64) {
        let mut j: size_t = size.wrapping_sub(i).wrapping_sub(1 as i32 as u64);
        let mut tmp: size_t = *((*p).data).offset(i as isize);
        *((*p).data).offset(i as isize) = *((*p).data).offset(j as isize);
        *((*p).data).offset(j as isize) = tmp;
        i = i.wrapping_add(1);
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn gsl_permutation_inverse(
    mut inv: *mut gsl_permutation,
    mut p: *const gsl_permutation,
) -> i32 {
    let size: size_t = (*p).size;
    let mut i: size_t = 0;
    if (*inv).size != size {
        gsl_error(
            b"permutation lengths are not equal\0" as *const u8 as *const i8,
            b"permutation.c\0" as *const u8 as *const i8,
            114 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    }
    i = 0 as i32 as size_t;
    while i < size {
        *((*inv).data).offset(*((*p).data).offset(i as isize) as isize) = i;
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_permutation_next(mut p: *mut gsl_permutation) -> i32 {
    let size: size_t = (*p).size;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    if size < 2 as i32 as u64 {
        return GSL_FAILURE as i32;
    }
    i = size.wrapping_sub(2 as i32 as u64);
    while *((*p).data).offset(i as isize)
        > *((*p).data).offset(i.wrapping_add(1 as i32 as u64) as isize)
        && i != 0 as i32 as u64
    {
        i = i.wrapping_sub(1);
        i;
    }
    if i == 0 as i32 as u64
        && *((*p).data).offset(0 as i32 as isize)
            > *((*p).data).offset(1 as i32 as isize)
    {
        return GSL_FAILURE as i32;
    }
    k = i.wrapping_add(1 as i32 as u64);
    j = i.wrapping_add(2 as i32 as u64);
    while j < size {
        if *((*p).data).offset(j as isize) > *((*p).data).offset(i as isize)
            && *((*p).data).offset(j as isize) < *((*p).data).offset(k as isize)
        {
            k = j;
        }
        j = j.wrapping_add(1);
        j;
    }
    let mut tmp: size_t = *((*p).data).offset(i as isize);
    *((*p).data).offset(i as isize) = *((*p).data).offset(k as isize);
    *((*p).data).offset(k as isize) = tmp;
    j = i.wrapping_add(1 as i32 as u64);
    while j <= size.wrapping_add(i).wrapping_div(2 as i32 as u64) {
        let mut tmp_0: size_t = *((*p).data).offset(j as isize);
        *((*p).data).offset(j as isize) = *((*p).data)
            .offset(size.wrapping_add(i).wrapping_sub(j) as isize);
        *((*p).data).offset(size.wrapping_add(i).wrapping_sub(j) as isize) = tmp_0;
        j = j.wrapping_add(1);
        j;
    }
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_permutation_prev(mut p: *mut gsl_permutation) -> i32 {
    let size: size_t = (*p).size;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut k: size_t = 0;
    if size < 2 as i32 as u64 {
        return GSL_FAILURE as i32;
    }
    i = size.wrapping_sub(2 as i32 as u64);
    while *((*p).data).offset(i as isize)
        < *((*p).data).offset(i.wrapping_add(1 as i32 as u64) as isize)
        && i != 0 as i32 as u64
    {
        i = i.wrapping_sub(1);
        i;
    }
    if i == 0 as i32 as u64
        && *((*p).data).offset(0 as i32 as isize)
            < *((*p).data).offset(1 as i32 as isize)
    {
        return GSL_FAILURE as i32;
    }
    k = i.wrapping_add(1 as i32 as u64);
    j = i.wrapping_add(2 as i32 as u64);
    while j < size {
        if *((*p).data).offset(j as isize) < *((*p).data).offset(i as isize)
            && *((*p).data).offset(j as isize) > *((*p).data).offset(k as isize)
        {
            k = j;
        }
        j = j.wrapping_add(1);
        j;
    }
    let mut tmp: size_t = *((*p).data).offset(i as isize);
    *((*p).data).offset(i as isize) = *((*p).data).offset(k as isize);
    *((*p).data).offset(k as isize) = tmp;
    j = i.wrapping_add(1 as i32 as u64);
    while j <= size.wrapping_add(i).wrapping_div(2 as i32 as u64) {
        let mut tmp_0: size_t = *((*p).data).offset(j as isize);
        *((*p).data).offset(j as isize) = *((*p).data)
            .offset(size.wrapping_add(i).wrapping_sub(j) as isize);
        *((*p).data).offset(size.wrapping_add(i).wrapping_sub(j) as isize) = tmp_0;
        j = j.wrapping_add(1);
        j;
    }
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_permutation_mul(
    mut p: *mut gsl_permutation,
    mut pa: *const gsl_permutation,
    mut pb: *const gsl_permutation,
) -> i32 {
    let mut i: size_t = 0;
    let size: size_t = (*p).size;
    if (*pa).size != size {
        gsl_error(
            b"size of result does not match size of pa\0" as *const u8 as *const i8,
            b"permutation.c\0" as *const u8 as *const i8,
            238 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    }
    if (*pb).size != size {
        gsl_error(
            b"size of result does not match size of pb\0" as *const u8 as *const i8,
            b"permutation.c\0" as *const u8 as *const i8,
            243 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    }
    i = 0 as i32 as size_t;
    while i < size {
        *((*p).data).offset(i as isize) = *((*pb).data)
            .offset(*((*pa).data).offset(i as isize) as isize);
        i = i.wrapping_add(1);
        i;
    }
    return GSL_SUCCESS as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_permutation_memcpy(
    mut dest: *mut gsl_permutation,
    mut src: *const gsl_permutation,
) -> i32 {
    let src_size: size_t = (*src).size;
    let dest_size: size_t = (*dest).size;
    if src_size != dest_size {
        gsl_error(
            b"permutation lengths are not equal\0" as *const u8 as *const i8,
            b"permutation.c\0" as *const u8 as *const i8,
            263 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    }
    let mut j: size_t = 0;
    j = 0 as i32 as size_t;
    while j < src_size {
        *((*dest).data).offset(j as isize) = *((*src).data).offset(j as isize);
        j = j.wrapping_add(1);
        j;
    }
    return GSL_SUCCESS as i32;
}