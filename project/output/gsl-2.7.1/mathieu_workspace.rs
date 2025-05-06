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
    fn free(__ptr: *mut libc::c_void);
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn gsl_error(reason: *const i8, file: *const i8, line: i32, gsl_errno: i32);
    fn gsl_vector_alloc(n: size_t) -> *mut gsl_vector;
    fn gsl_vector_free(v: *mut gsl_vector);
    fn gsl_matrix_alloc(n1: size_t, n2: size_t) -> *mut gsl_matrix;
    fn gsl_matrix_free(m: *mut gsl_matrix);
    fn gsl_eigen_symmv_alloc(n: size_t) -> *mut gsl_eigen_symmv_workspace;
    fn gsl_eigen_symmv_free(w: *mut gsl_eigen_symmv_workspace);
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_matrix {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut libc::c_double,
    pub block: *mut gsl_block,
    pub owner: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_eigen_symmv_workspace {
    pub size: size_t,
    pub d: *mut libc::c_double,
    pub sd: *mut libc::c_double,
    pub gc: *mut libc::c_double,
    pub gs: *mut libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_sf_mathieu_workspace {
    pub size: size_t,
    pub even_order: size_t,
    pub odd_order: size_t,
    pub extra_values: i32,
    pub qa: libc::c_double,
    pub qb: libc::c_double,
    pub aa: *mut libc::c_double,
    pub bb: *mut libc::c_double,
    pub dd: *mut libc::c_double,
    pub ee: *mut libc::c_double,
    pub tt: *mut libc::c_double,
    pub e2: *mut libc::c_double,
    pub zz: *mut libc::c_double,
    pub eval: *mut gsl_vector,
    pub evec: *mut gsl_matrix,
    pub wmat: *mut gsl_eigen_symmv_workspace,
}
#[no_mangle]
pub unsafe extern "C" fn gsl_sf_mathieu_alloc(
    nn: size_t,
    qq: libc::c_double,
) -> *mut gsl_sf_mathieu_workspace {
    let mut workspace: *mut gsl_sf_mathieu_workspace = 0
        as *mut gsl_sf_mathieu_workspace;
    let mut even_order: u32 = nn
        .wrapping_div(2 as i32 as u64)
        .wrapping_add(1 as i32 as u64) as u32;
    let mut odd_order: u32 = nn
        .wrapping_add(1 as i32 as u64)
        .wrapping_div(2 as i32 as u64) as u32;
    let mut extra_values: u32 = 0;
    extra_values = ((2.1f64 * pow(fabs(qq), 0.37f64)) as i32 + 9 as i32) as u32;
    extra_values = extra_values.wrapping_add(20 as i32 as u32);
    if nn.wrapping_add(1 as i32 as u64) == 0 as i32 as u64 {
        gsl_error(
            b"matrix dimension must be positive integer\0" as *const u8 as *const i8,
            b"mathieu_workspace.c\0" as *const u8 as *const i8,
            43 as i32,
            GSL_EINVAL as i32,
        );
        return 0 as *mut gsl_sf_mathieu_workspace;
    }
    workspace = malloc(::core::mem::size_of::<gsl_sf_mathieu_workspace>() as u64)
        as *mut gsl_sf_mathieu_workspace;
    if workspace.is_null() {
        gsl_error(
            b"failed to allocate space for workspace\0" as *const u8 as *const i8,
            b"mathieu_workspace.c\0" as *const u8 as *const i8,
            50 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_sf_mathieu_workspace;
    }
    even_order = even_order.wrapping_add(extra_values);
    odd_order = odd_order.wrapping_add(extra_values);
    (*workspace).size = nn;
    (*workspace).even_order = even_order as size_t;
    (*workspace).odd_order = odd_order as size_t;
    (*workspace).extra_values = extra_values as i32;
    (*workspace).aa = malloc(
        nn
            .wrapping_add(1 as i32 as u64)
            .wrapping_mul(::core::mem::size_of::<libc::c_double>() as u64),
    ) as *mut libc::c_double;
    if ((*workspace).aa).is_null() {
        free(workspace as *mut libc::c_void);
        gsl_error(
            b"Error allocating memory for characteristic a values\0" as *const u8
                as *const i8,
            b"mathieu_workspace.c\0" as *const u8 as *const i8,
            68 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_sf_mathieu_workspace;
    }
    (*workspace).bb = malloc(
        nn
            .wrapping_add(1 as i32 as u64)
            .wrapping_mul(::core::mem::size_of::<libc::c_double>() as u64),
    ) as *mut libc::c_double;
    if ((*workspace).bb).is_null() {
        free((*workspace).aa as *mut libc::c_void);
        free(workspace as *mut libc::c_void);
        gsl_error(
            b"Error allocating memory for characteristic b values\0" as *const u8
                as *const i8,
            b"mathieu_workspace.c\0" as *const u8 as *const i8,
            77 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_sf_mathieu_workspace;
    }
    (*workspace).dd = malloc(
        (even_order as u64).wrapping_mul(::core::mem::size_of::<libc::c_double>() as u64),
    ) as *mut libc::c_double;
    if ((*workspace).dd).is_null() {
        free((*workspace).aa as *mut libc::c_void);
        free((*workspace).bb as *mut libc::c_void);
        free(workspace as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for diagonal\0" as *const u8 as *const i8,
            b"mathieu_workspace.c\0" as *const u8 as *const i8,
            89 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_sf_mathieu_workspace;
    }
    (*workspace).ee = malloc(
        (even_order as u64).wrapping_mul(::core::mem::size_of::<libc::c_double>() as u64),
    ) as *mut libc::c_double;
    if ((*workspace).ee).is_null() {
        free((*workspace).dd as *mut libc::c_void);
        free((*workspace).aa as *mut libc::c_void);
        free((*workspace).bb as *mut libc::c_void);
        free(workspace as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for diagonal\0" as *const u8 as *const i8,
            b"mathieu_workspace.c\0" as *const u8 as *const i8,
            99 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_sf_mathieu_workspace;
    }
    (*workspace).tt = malloc(
        ((3 as i32 as u32).wrapping_mul(even_order) as u64)
            .wrapping_mul(::core::mem::size_of::<libc::c_double>() as u64),
    ) as *mut libc::c_double;
    if ((*workspace).tt).is_null() {
        free((*workspace).ee as *mut libc::c_void);
        free((*workspace).dd as *mut libc::c_void);
        free((*workspace).aa as *mut libc::c_void);
        free((*workspace).bb as *mut libc::c_void);
        free(workspace as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for diagonal\0" as *const u8 as *const i8,
            b"mathieu_workspace.c\0" as *const u8 as *const i8,
            110 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_sf_mathieu_workspace;
    }
    (*workspace).e2 = malloc(
        (even_order as u64).wrapping_mul(::core::mem::size_of::<libc::c_double>() as u64),
    ) as *mut libc::c_double;
    if ((*workspace).e2).is_null() {
        free((*workspace).tt as *mut libc::c_void);
        free((*workspace).ee as *mut libc::c_void);
        free((*workspace).dd as *mut libc::c_void);
        free((*workspace).aa as *mut libc::c_void);
        free((*workspace).bb as *mut libc::c_void);
        free(workspace as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for diagonal\0" as *const u8 as *const i8,
            b"mathieu_workspace.c\0" as *const u8 as *const i8,
            122 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_sf_mathieu_workspace;
    }
    (*workspace).zz = malloc(
        (even_order.wrapping_mul(even_order) as u64)
            .wrapping_mul(::core::mem::size_of::<libc::c_double>() as u64),
    ) as *mut libc::c_double;
    if ((*workspace).zz).is_null() {
        free((*workspace).e2 as *mut libc::c_void);
        free((*workspace).tt as *mut libc::c_void);
        free((*workspace).ee as *mut libc::c_void);
        free((*workspace).dd as *mut libc::c_void);
        free((*workspace).aa as *mut libc::c_void);
        free((*workspace).bb as *mut libc::c_void);
        free(workspace as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for diagonal\0" as *const u8 as *const i8,
            b"mathieu_workspace.c\0" as *const u8 as *const i8,
            135 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_sf_mathieu_workspace;
    }
    (*workspace).eval = gsl_vector_alloc(even_order as size_t);
    if ((*workspace).eval).is_null() {
        free((*workspace).zz as *mut libc::c_void);
        free((*workspace).e2 as *mut libc::c_void);
        free((*workspace).tt as *mut libc::c_void);
        free((*workspace).ee as *mut libc::c_void);
        free((*workspace).dd as *mut libc::c_void);
        free((*workspace).aa as *mut libc::c_void);
        free((*workspace).bb as *mut libc::c_void);
        free(workspace as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for eval\0" as *const u8 as *const i8,
            b"mathieu_workspace.c\0" as *const u8 as *const i8,
            150 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_sf_mathieu_workspace;
    }
    (*workspace).evec = gsl_matrix_alloc(even_order as size_t, even_order as size_t);
    if ((*workspace).evec).is_null() {
        gsl_vector_free((*workspace).eval);
        free((*workspace).zz as *mut libc::c_void);
        free((*workspace).e2 as *mut libc::c_void);
        free((*workspace).tt as *mut libc::c_void);
        free((*workspace).ee as *mut libc::c_void);
        free((*workspace).dd as *mut libc::c_void);
        free((*workspace).aa as *mut libc::c_void);
        free((*workspace).bb as *mut libc::c_void);
        free(workspace as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for evec\0" as *const u8 as *const i8,
            b"mathieu_workspace.c\0" as *const u8 as *const i8,
            166 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_sf_mathieu_workspace;
    }
    (*workspace).wmat = gsl_eigen_symmv_alloc(even_order as size_t);
    if ((*workspace).wmat).is_null() {
        gsl_matrix_free((*workspace).evec);
        gsl_vector_free((*workspace).eval);
        free((*workspace).zz as *mut libc::c_void);
        free((*workspace).e2 as *mut libc::c_void);
        free((*workspace).tt as *mut libc::c_void);
        free((*workspace).ee as *mut libc::c_void);
        free((*workspace).dd as *mut libc::c_void);
        free((*workspace).aa as *mut libc::c_void);
        free((*workspace).bb as *mut libc::c_void);
        free(workspace as *mut libc::c_void);
        gsl_error(
            b"failed to allocate space for wmat\0" as *const u8 as *const i8,
            b"mathieu_workspace.c\0" as *const u8 as *const i8,
            183 as i32,
            GSL_ENOMEM as i32,
        );
        return 0 as *mut gsl_sf_mathieu_workspace;
    }
    return workspace;
}
#[no_mangle]
pub unsafe extern "C" fn gsl_sf_mathieu_free(
    mut workspace: *mut gsl_sf_mathieu_workspace,
) {
    if workspace.is_null() {
        return;
    }
    gsl_vector_free((*workspace).eval);
    gsl_matrix_free((*workspace).evec);
    gsl_eigen_symmv_free((*workspace).wmat);
    free((*workspace).aa as *mut libc::c_void);
    free((*workspace).bb as *mut libc::c_void);
    free((*workspace).dd as *mut libc::c_void);
    free((*workspace).ee as *mut libc::c_void);
    free((*workspace).tt as *mut libc::c_void);
    free((*workspace).e2 as *mut libc::c_void);
    free((*workspace).zz as *mut libc::c_void);
    free(workspace as *mut libc::c_void);
}