use ::libc;
extern "C" {
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_movstat_alloc(K: size_t) -> *mut gsl_movstat_workspace;
    fn gsl_movstat_free(w: *mut gsl_movstat_workspace);
    fn gsl_movstat_median(
        endtype: gsl_movstat_end_t,
        x: *const gsl_vector,
        y: *mut gsl_vector,
        w: *mut gsl_movstat_workspace,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type C2RustUnnamed = libc::c_int;
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
    pub owner: libc::c_int,
}
pub type gsl_movstat_end_t = libc::c_uint;
pub const GSL_MOVSTAT_END_TRUNCATE: gsl_movstat_end_t = 2;
pub const GSL_MOVSTAT_END_PADVALUE: gsl_movstat_end_t = 1;
pub const GSL_MOVSTAT_END_PADZERO: gsl_movstat_end_t = 0;
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
pub type gsl_filter_end_t = libc::c_uint;
pub const GSL_FILTER_END_TRUNCATE: gsl_filter_end_t = 2;
pub const GSL_FILTER_END_PADVALUE: gsl_filter_end_t = 1;
pub const GSL_FILTER_END_PADZERO: gsl_filter_end_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_filter_median_workspace {
    pub movstat_workspace_p: *mut gsl_movstat_workspace,
}
#[no_mangle]
pub unsafe extern "C" fn gsl_filter_median_alloc(
    K: size_t,
) -> *mut gsl_filter_median_workspace {
    let mut w: *mut gsl_filter_median_workspace = 0 as *mut gsl_filter_median_workspace;
    let mut H: size_t = K.wrapping_div(2 as libc::c_int as libc::c_ulong);
    w = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::core::mem::size_of::<gsl_filter_median_workspace>() as libc::c_ulong,
    ) as *mut gsl_filter_median_workspace;
    if w.is_null() {
        gsl_error(
            b"failed to allocate space for workspace\0" as *const u8
                as *const libc::c_char,
            b"median.c\0" as *const u8 as *const libc::c_char,
            40 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_filter_median_workspace;
    }
    (*w)
        .movstat_workspace_p = gsl_movstat_alloc(
        (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(H)
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    if ((*w).movstat_workspace_p).is_null() {
        gsl_filter_median_free(w);
        gsl_error(
            b"failed to allocate space for movstat workspace\0" as *const u8
                as *const libc::c_char,
            b"median.c\0" as *const u8 as *const libc::c_char,
            47 as libc::c_int,
            GSL_ENOMEM as libc::c_int,
        );
        return 0 as *mut gsl_filter_median_workspace;
    }
    return w;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_filter_median_free(
    mut w: *mut gsl_filter_median_workspace,
) {
    if !((*w).movstat_workspace_p).is_null() {
        gsl_movstat_free((*w).movstat_workspace_p);
    }
    free(w as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn gsl_filter_median(
    endtype: gsl_filter_end_t,
    mut x: *const gsl_vector,
    mut y: *mut gsl_vector,
    mut w: *mut gsl_filter_median_workspace,
) -> libc::c_int {
    let mut status: libc::c_int = gsl_movstat_median(
        endtype as gsl_movstat_end_t,
        x,
        y,
        (*w).movstat_workspace_p,
    );
    return status;
}
