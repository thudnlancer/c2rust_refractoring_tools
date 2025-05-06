#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
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
pub type gsl_comparison_fn_t = Option<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32,
>;
#[inline]
unsafe extern "C" fn downheap(
    mut p: *mut size_t,
    mut data: *const libc::c_void,
    size: size_t,
    N: size_t,
    mut k: size_t,
    mut compare: gsl_comparison_fn_t,
) {
    let pki: size_t = *p.offset(k as isize);
    while k <= N.wrapping_div(2 as i32 as u64) {
        let mut j: size_t = (2 as i32 as u64).wrapping_mul(k);
        if j < N
            && compare
                .expect(
                    "non-null function pointer",
                )(
                (data as *const i8)
                    .offset(size.wrapping_mul(*p.offset(j as isize)) as isize)
                    as *const libc::c_void,
                (data as *const i8)
                    .offset(
                        size
                            .wrapping_mul(
                                *p.offset(j.wrapping_add(1 as i32 as u64) as isize),
                            ) as isize,
                    ) as *const libc::c_void,
            ) < 0 as i32
        {
            j = j.wrapping_add(1);
            j;
        }
        if compare
            .expect(
                "non-null function pointer",
            )(
            (data as *const i8).offset(size.wrapping_mul(pki) as isize)
                as *const libc::c_void,
            (data as *const i8).offset(size.wrapping_mul(*p.offset(j as isize)) as isize)
                as *const libc::c_void,
        ) >= 0 as i32
        {
            break;
        }
        *p.offset(k as isize) = *p.offset(j as isize);
        k = j;
    }
    *p.offset(k as isize) = pki;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_heapsort_index(
    mut p: *mut size_t,
    mut data: *const libc::c_void,
    mut count: size_t,
    mut size: size_t,
    mut compare: gsl_comparison_fn_t,
) -> i32 {
    let mut i: size_t = 0;
    let mut k: size_t = 0;
    let mut N: size_t = 0;
    if count == 0 as i32 as u64 {
        return GSL_SUCCESS as i32;
    }
    i = 0 as i32 as size_t;
    while i < count {
        *p.offset(i as isize) = i;
        i = i.wrapping_add(1);
        i;
    }
    N = count.wrapping_sub(1 as i32 as u64);
    k = N.wrapping_div(2 as i32 as u64);
    k = k.wrapping_add(1);
    k;
    loop {
        k = k.wrapping_sub(1);
        k;
        downheap(p, data, size, N, k, compare);
        if !(k > 0 as i32 as u64) {
            break;
        }
    }
    while N > 0 as i32 as u64 {
        let mut tmp: size_t = *p.offset(0 as i32 as isize);
        *p.offset(0 as i32 as isize) = *p.offset(N as isize);
        *p.offset(N as isize) = tmp;
        N = N.wrapping_sub(1);
        N;
        downheap(p, data, size, N, 0 as i32 as size_t, compare);
    }
    return GSL_SUCCESS as i32;
}