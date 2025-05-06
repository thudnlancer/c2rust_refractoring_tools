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
    static mut gsl_movstat_accum_userfunc: *const gsl_movstat_accum;
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
pub struct gsl_block_struct {
    pub size: size_t,
    pub data: *mut libc::c_double,
}
pub type gsl_block = gsl_block_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut libc::c_double,
    pub block: *mut gsl_block,
    pub owner: i32,
}
pub type gsl_movstat_end_t = u32;
pub const GSL_MOVSTAT_END_TRUNCATE: gsl_movstat_end_t = 2;
pub const GSL_MOVSTAT_END_PADVALUE: gsl_movstat_end_t = 1;
pub const GSL_MOVSTAT_END_PADZERO: gsl_movstat_end_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_movstat_accum {
    pub size: Option<unsafe extern "C" fn(size_t) -> size_t>,
    pub init: Option<unsafe extern "C" fn(size_t, *mut libc::c_void) -> i32>,
    pub insert: Option<unsafe extern "C" fn(libc::c_double, *mut libc::c_void) -> i32>,
    pub delete_oldest: Option<unsafe extern "C" fn(*mut libc::c_void) -> i32>,
    pub get: Option<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut libc::c_double,
            *const libc::c_void,
        ) -> i32,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_movstat_function {
    pub function: Option<
        unsafe extern "C" fn(
            size_t,
            *mut libc::c_double,
            *mut libc::c_void,
        ) -> libc::c_double,
    >,
    pub params: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_movstat_workspace {
    pub H: size_t,
    pub J: size_t,
    pub K: size_t,
    pub work: *mut libc::c_double,
    pub state: *mut libc::c_void,
    pub state_size: size_t,
}
#[inline]
unsafe extern "C" fn gsl_vector_get(
    mut v: *const gsl_vector,
    i: size_t,
) -> libc::c_double {
    return *((*v).data).offset(i.wrapping_mul((*v).stride) as isize);
}
#[inline]
unsafe extern "C" fn gsl_vector_set(
    mut v: *mut gsl_vector,
    i: size_t,
    mut x: libc::c_double,
) {
    *((*v).data).offset(i.wrapping_mul((*v).stride) as isize) = x;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_movstat_apply_accum(
    endtype: gsl_movstat_end_t,
    mut x: *const gsl_vector,
    mut accum: *const gsl_movstat_accum,
    mut accum_params: *mut libc::c_void,
    mut y: *mut gsl_vector,
    mut z: *mut gsl_vector,
    mut w: *mut gsl_movstat_workspace,
) -> i32 {
    if (*x).size != (*y).size {
        gsl_error(
            b"input and output vectors must have same length\0" as *const u8
                as *const i8,
            b"apply.c\0" as *const u8 as *const i8,
            57 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    } else if !z.is_null() && (*x).size != (*z).size {
        gsl_error(
            b"input and output vectors must have same length\0" as *const u8
                as *const i8,
            b"apply.c\0" as *const u8 as *const i8,
            61 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    } else {
        let n: i32 = (*x).size as i32;
        let H: i32 = (*w).H as i32;
        let J: i32 = (*w).J as i32;
        let mut i: i32 = 0;
        let mut x1: libc::c_double = 0.0f64;
        let mut xN: libc::c_double = 0.0f64;
        let mut result: [libc::c_double; 2] = [0.; 2];
        ((*accum).init).expect("non-null function pointer")((*w).K, (*w).state);
        if endtype as u32 != GSL_MOVSTAT_END_TRUNCATE as i32 as u32 {
            if endtype as u32 == GSL_MOVSTAT_END_PADZERO as i32 as u32 {
                x1 = 0.0f64;
                xN = 0.0f64;
            } else if endtype as u32 == GSL_MOVSTAT_END_PADVALUE as i32 as u32 {
                x1 = gsl_vector_get(x, 0 as i32 as size_t);
                xN = gsl_vector_get(x, (n - 1 as i32) as size_t);
            }
            i = 0 as i32;
            while i < H {
                ((*accum).insert).expect("non-null function pointer")(x1, (*w).state);
                i += 1;
                i;
            }
        } else if ((*accum).delete_oldest).is_none() {
            let mut idx1: i32 = if n - J - H > 0 as i32 { n - J - H } else { 0 as i32 };
            let mut idx2: i32 = n - 1 as i32;
            i = idx1;
            while i <= idx2 {
                *((*w).work).offset((i - idx1) as isize) = gsl_vector_get(
                    x,
                    i as size_t,
                );
                i += 1;
                i;
            }
        }
        i = 0 as i32;
        while i < n {
            let mut xi: libc::c_double = gsl_vector_get(x, i as size_t);
            let mut idx: i32 = i - J;
            ((*accum).insert).expect("non-null function pointer")(xi, (*w).state);
            if idx >= 0 as i32 {
                ((*accum).get)
                    .expect(
                        "non-null function pointer",
                    )(accum_params, result.as_mut_ptr(), (*w).state);
                gsl_vector_set(y, idx as size_t, result[0 as i32 as usize]);
                if !z.is_null() {
                    gsl_vector_set(z, idx as size_t, result[1 as i32 as usize]);
                }
            }
            i += 1;
            i;
        }
        if endtype as u32 == GSL_MOVSTAT_END_TRUNCATE as i32 as u32 {
            let mut idx1_0: i32 = if n - J > 0 as i32 { n - J } else { 0 as i32 };
            let mut idx2_0: i32 = n - 1 as i32;
            if ((*accum).delete_oldest).is_none() {
                let mut wsize: i32 = n
                    - (if n - J - H > 0 as i32 { n - J - H } else { 0 as i32 });
                i = idx1_0;
                while i <= idx2_0 {
                    let mut nsamp: i32 = n
                        - (if i - H > 0 as i32 { i - H } else { 0 as i32 });
                    let mut j: i32 = 0;
                    ((*accum).init)
                        .expect("non-null function pointer")((*w).K, (*w).state);
                    j = wsize - nsamp;
                    while j < wsize {
                        ((*accum).insert)
                            .expect(
                                "non-null function pointer",
                            )(*((*w).work).offset(j as isize), (*w).state);
                        j += 1;
                        j;
                    }
                    ((*accum).get)
                        .expect(
                            "non-null function pointer",
                        )(accum_params, result.as_mut_ptr(), (*w).state);
                    gsl_vector_set(y, i as size_t, result[0 as i32 as usize]);
                    if !z.is_null() {
                        gsl_vector_set(z, i as size_t, result[1 as i32 as usize]);
                    }
                    i += 1;
                    i;
                }
            } else {
                i = idx1_0;
                while i <= idx2_0 {
                    if i - H > 0 as i32 {
                        ((*accum).delete_oldest)
                            .expect("non-null function pointer")((*w).state);
                    }
                    ((*accum).get)
                        .expect(
                            "non-null function pointer",
                        )(accum_params, result.as_mut_ptr(), (*w).state);
                    gsl_vector_set(y, i as size_t, result[0 as i32 as usize]);
                    if !z.is_null() {
                        gsl_vector_set(z, i as size_t, result[1 as i32 as usize]);
                    }
                    i += 1;
                    i;
                }
            }
        } else {
            i = 0 as i32;
            while i < J {
                let mut idx_0: i32 = n - J + i;
                ((*accum).insert).expect("non-null function pointer")(xN, (*w).state);
                if idx_0 >= 0 as i32 {
                    ((*accum).get)
                        .expect(
                            "non-null function pointer",
                        )(accum_params, result.as_mut_ptr(), (*w).state);
                    gsl_vector_set(y, idx_0 as size_t, result[0 as i32 as usize]);
                    if !z.is_null() {
                        gsl_vector_set(z, idx_0 as size_t, result[1 as i32 as usize]);
                    }
                }
                i += 1;
                i;
            }
        }
        return GSL_SUCCESS as i32;
    };
}
#[no_mangle]
pub unsafe extern "C" fn gsl_movstat_apply(
    endtype: gsl_movstat_end_t,
    mut F: *const gsl_movstat_function,
    mut x: *const gsl_vector,
    mut y: *mut gsl_vector,
    mut w: *mut gsl_movstat_workspace,
) -> i32 {
    let mut status: i32 = gsl_movstat_apply_accum(
        endtype,
        x,
        gsl_movstat_accum_userfunc,
        F as *mut libc::c_void,
        y,
        0 as *mut gsl_vector,
        w,
    );
    return status;
}