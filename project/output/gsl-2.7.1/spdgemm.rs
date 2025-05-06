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
    fn gsl_spmatrix_realloc(nzmax: size_t, m: *mut gsl_spmatrix) -> i32;
    fn gsl_spmatrix_scale(m: *mut gsl_spmatrix, x: libc::c_double) -> i32;
}
pub type size_t = u64;
pub type C2RustUnnamed = u32;
pub const GSL_SPMATRIX_CRS: C2RustUnnamed = 2;
pub const GSL_SPMATRIX_CCS: C2RustUnnamed = 1;
pub const GSL_SPMATRIX_TRIPLET: C2RustUnnamed = 0;
pub const GSL_SPMATRIX_CSR: C2RustUnnamed = 2;
pub const GSL_SPMATRIX_CSC: C2RustUnnamed = 1;
pub const GSL_SPMATRIX_COO: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_spmatrix_pool_node {
    pub next: *mut gsl_spmatrix_pool_node,
    pub block_ptr: *mut libc::c_void,
    pub free_slot: *mut u8,
}
pub type gsl_spmatrix_pool = gsl_spmatrix_pool_node;
pub type gsl_bst_cmp_function = unsafe extern "C" fn(
    *const libc::c_void,
    *const libc::c_void,
    *mut libc::c_void,
) -> i32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_bst_allocator {
    pub alloc: Option<
        unsafe extern "C" fn(size_t, *mut libc::c_void) -> *mut libc::c_void,
    >,
    pub free: Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_bst_avl_node {
    pub avl_link: [*mut gsl_bst_avl_node; 2],
    pub avl_data: *mut libc::c_void,
    pub avl_balance: libc::c_schar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_bst_avl_table {
    pub avl_root: *mut gsl_bst_avl_node,
    pub avl_compare: Option<gsl_bst_cmp_function>,
    pub avl_param: *mut libc::c_void,
    pub avl_alloc: *const gsl_bst_allocator,
    pub avl_count: size_t,
    pub avl_generation: u64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_bst_rb_node {
    pub rb_link: [*mut gsl_bst_rb_node; 2],
    pub rb_data: *mut libc::c_void,
    pub rb_color: u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_bst_rb_table {
    pub rb_root: *mut gsl_bst_rb_node,
    pub rb_compare: Option<gsl_bst_cmp_function>,
    pub rb_param: *mut libc::c_void,
    pub rb_alloc: *const gsl_bst_allocator,
    pub rb_count: size_t,
    pub rb_generation: u64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_bst_type {
    pub name: *const i8,
    pub node_size: size_t,
    pub init: Option<
        unsafe extern "C" fn(
            *const gsl_bst_allocator,
            Option<gsl_bst_cmp_function>,
            *mut libc::c_void,
            *mut libc::c_void,
        ) -> i32,
    >,
    pub nodes: Option<unsafe extern "C" fn(*const libc::c_void) -> size_t>,
    pub insert: Option<
        unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> *mut libc::c_void,
    >,
    pub find: Option<
        unsafe extern "C" fn(
            *const libc::c_void,
            *const libc::c_void,
        ) -> *mut libc::c_void,
    >,
    pub remove: Option<
        unsafe extern "C" fn(*const libc::c_void, *mut libc::c_void) -> *mut libc::c_void,
    >,
    pub empty: Option<unsafe extern "C" fn(*mut libc::c_void) -> i32>,
    pub trav_init: Option<
        unsafe extern "C" fn(*mut libc::c_void, *const libc::c_void) -> i32,
    >,
    pub trav_first: Option<
        unsafe extern "C" fn(*mut libc::c_void, *const libc::c_void) -> *mut libc::c_void,
    >,
    pub trav_last: Option<
        unsafe extern "C" fn(*mut libc::c_void, *const libc::c_void) -> *mut libc::c_void,
    >,
    pub trav_find: Option<
        unsafe extern "C" fn(
            *const libc::c_void,
            *mut libc::c_void,
            *const libc::c_void,
        ) -> *mut libc::c_void,
    >,
    pub trav_insert: Option<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut libc::c_void,
            *mut libc::c_void,
        ) -> *mut libc::c_void,
    >,
    pub trav_copy: Option<
        unsafe extern "C" fn(*mut libc::c_void, *const libc::c_void) -> *mut libc::c_void,
    >,
    pub trav_next: Option<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
    pub trav_prev: Option<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
    pub trav_cur: Option<unsafe extern "C" fn(*const libc::c_void) -> *mut libc::c_void>,
    pub trav_replace: Option<
        unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> *mut libc::c_void,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_bst_workspace {
    pub type_0: *const gsl_bst_type,
    pub table: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub avl_table: gsl_bst_avl_table,
    pub rb_table: gsl_bst_rb_table,
}
pub type C2RustUnnamed_1 = i32;
pub const GSL_EOF: C2RustUnnamed_1 = 32;
pub const GSL_ETOLG: C2RustUnnamed_1 = 31;
pub const GSL_ETOLX: C2RustUnnamed_1 = 30;
pub const GSL_ETOLF: C2RustUnnamed_1 = 29;
pub const GSL_ENOPROGJ: C2RustUnnamed_1 = 28;
pub const GSL_ENOPROG: C2RustUnnamed_1 = 27;
pub const GSL_ETABLE: C2RustUnnamed_1 = 26;
pub const GSL_ECACHE: C2RustUnnamed_1 = 25;
pub const GSL_EUNIMPL: C2RustUnnamed_1 = 24;
pub const GSL_EUNSUP: C2RustUnnamed_1 = 23;
pub const GSL_EDIVERGE: C2RustUnnamed_1 = 22;
pub const GSL_ESING: C2RustUnnamed_1 = 21;
pub const GSL_ENOTSQR: C2RustUnnamed_1 = 20;
pub const GSL_EBADLEN: C2RustUnnamed_1 = 19;
pub const GSL_EROUND: C2RustUnnamed_1 = 18;
pub const GSL_ELOSS: C2RustUnnamed_1 = 17;
pub const GSL_EOVRFLW: C2RustUnnamed_1 = 16;
pub const GSL_EUNDRFLW: C2RustUnnamed_1 = 15;
pub const GSL_ETOL: C2RustUnnamed_1 = 14;
pub const GSL_EBADTOL: C2RustUnnamed_1 = 13;
pub const GSL_EZERODIV: C2RustUnnamed_1 = 12;
pub const GSL_EMAXITER: C2RustUnnamed_1 = 11;
pub const GSL_ERUNAWAY: C2RustUnnamed_1 = 10;
pub const GSL_EBADFUNC: C2RustUnnamed_1 = 9;
pub const GSL_ENOMEM: C2RustUnnamed_1 = 8;
pub const GSL_ESANITY: C2RustUnnamed_1 = 7;
pub const GSL_EFACTOR: C2RustUnnamed_1 = 6;
pub const GSL_EFAILED: C2RustUnnamed_1 = 5;
pub const GSL_EINVAL: C2RustUnnamed_1 = 4;
pub const GSL_EFAULT: C2RustUnnamed_1 = 3;
pub const GSL_ERANGE: C2RustUnnamed_1 = 2;
pub const GSL_EDOM: C2RustUnnamed_1 = 1;
pub const GSL_CONTINUE: C2RustUnnamed_1 = -2;
pub const GSL_FAILURE: C2RustUnnamed_1 = -1;
pub const GSL_SUCCESS: C2RustUnnamed_1 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_spmatrix {
    pub size1: size_t,
    pub size2: size_t,
    pub i: *mut i32,
    pub data: *mut libc::c_double,
    pub p: *mut i32,
    pub nzmax: size_t,
    pub nz: size_t,
    pub tree: *mut gsl_bst_workspace,
    pub pool: *mut gsl_spmatrix_pool,
    pub node_size: size_t,
    pub work: C2RustUnnamed_2,
    pub sptype: i32,
    pub spflags: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub work_void: *mut libc::c_void,
    pub work_int: *mut i32,
    pub work_atomic: *mut libc::c_double,
}
#[no_mangle]
pub unsafe extern "C" fn gsl_spblas_dgemm(
    alpha: libc::c_double,
    mut A: *const gsl_spmatrix,
    mut B: *const gsl_spmatrix,
    mut C: *mut gsl_spmatrix,
) -> i32 {
    if (*A).size2 != (*B).size1 || (*A).size1 != (*C).size1 || (*B).size2 != (*C).size2 {
        gsl_error(
            b"matrix dimensions do not match\0" as *const u8 as *const i8,
            b"spdgemm.c\0" as *const u8 as *const i8,
            48 as i32,
            GSL_EBADLEN as i32,
        );
        return GSL_EBADLEN as i32;
    } else if (*A).sptype != (*B).sptype || (*A).sptype != (*C).sptype {
        gsl_error(
            b"matrix storage formats do not match\0" as *const u8 as *const i8,
            b"spdgemm.c\0" as *const u8 as *const i8,
            52 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    } else if !((*A).sptype == GSL_SPMATRIX_CSC as i32) {
        gsl_error(
            b"compressed column format required\0" as *const u8 as *const i8,
            b"spdgemm.c\0" as *const u8 as *const i8,
            56 as i32,
            GSL_EINVAL as i32,
        );
        return GSL_EINVAL as i32;
    } else {
        let mut status: i32 = GSL_SUCCESS as i32;
        let M: size_t = (*A).size1;
        let N: size_t = (*B).size2;
        let mut Bi: *mut i32 = (*B).i;
        let mut Bp: *mut i32 = (*B).p;
        let mut Bd: *mut libc::c_double = (*B).data;
        let mut w: *mut i32 = (*A).work.work_int;
        let mut x: *mut libc::c_double = (*C).work.work_atomic;
        let mut Cp: *mut i32 = 0 as *mut i32;
        let mut Ci: *mut i32 = 0 as *mut i32;
        let mut Cd: *mut libc::c_double = 0 as *mut libc::c_double;
        let mut j: size_t = 0;
        let mut p: i32 = 0;
        let mut nz: size_t = 0 as i32 as size_t;
        if (*C).nzmax < ((*A).nz).wrapping_add((*B).nz) {
            status = gsl_spmatrix_realloc(((*A).nz).wrapping_add((*B).nz), C);
            if status != 0 {
                gsl_error(
                    b"unable to realloc matrix C\0" as *const u8 as *const i8,
                    b"spdgemm.c\0" as *const u8 as *const i8,
                    79 as i32,
                    status,
                );
                return status;
            }
        }
        j = 0 as i32 as size_t;
        while j < M {
            *w.offset(j as isize) = 0 as i32;
            j = j.wrapping_add(1);
            j;
        }
        Cp = (*C).p;
        Ci = (*C).i;
        Cd = (*C).data;
        j = 0 as i32 as size_t;
        while j < N {
            if nz.wrapping_add(M) > (*C).nzmax {
                status = gsl_spmatrix_realloc(
                    (2 as i32 as u64).wrapping_mul((*C).nzmax).wrapping_add(M),
                    C,
                );
                if status != 0 {
                    gsl_error(
                        b"unable to realloc matrix C\0" as *const u8 as *const i8,
                        b"spdgemm.c\0" as *const u8 as *const i8,
                        98 as i32,
                        status,
                    );
                    return status;
                }
                Ci = (*C).i;
                Cd = (*C).data;
            }
            *Cp.offset(j as isize) = nz as i32;
            p = *Bp.offset(j as isize);
            while p < *Bp.offset(j.wrapping_add(1 as i32 as u64) as isize) {
                nz = gsl_spblas_scatter(
                    A,
                    *Bi.offset(p as isize) as size_t,
                    *Bd.offset(p as isize),
                    w,
                    x,
                    j.wrapping_add(1 as i32 as u64) as i32,
                    C,
                    nz,
                );
                p += 1;
                p;
            }
            p = *Cp.offset(j as isize);
            while p < nz as i32 {
                *Cd.offset(p as isize) = *x.offset(*Ci.offset(p as isize) as isize);
                p += 1;
                p;
            }
            j = j.wrapping_add(1);
            j;
        }
        *Cp.offset(N as isize) = nz as i32;
        (*C).nz = nz;
        gsl_spmatrix_scale(C, alpha);
        return status;
    };
}
#[no_mangle]
pub unsafe extern "C" fn gsl_spblas_scatter(
    mut A: *const gsl_spmatrix,
    j: size_t,
    alpha: libc::c_double,
    mut w: *mut i32,
    mut x: *mut libc::c_double,
    mark: i32,
    mut C: *mut gsl_spmatrix,
    mut nz: size_t,
) -> size_t {
    let mut p: i32 = 0;
    let mut Ai: *mut i32 = (*A).i;
    let mut Ap: *mut i32 = (*A).p;
    let mut Ad: *mut libc::c_double = (*A).data;
    let mut Ci: *mut i32 = (*C).i;
    p = *Ap.offset(j as isize);
    while p < *Ap.offset(j.wrapping_add(1 as i32 as u64) as isize) {
        let mut i: size_t = *Ai.offset(p as isize) as size_t;
        if *w.offset(i as isize) < mark {
            *w.offset(i as isize) = mark;
            let fresh0 = nz;
            nz = nz.wrapping_add(1);
            *Ci.offset(fresh0 as isize) = i as i32;
            *x.offset(i as isize) = alpha * *Ad.offset(p as isize);
        } else {
            *x.offset(i as isize) += alpha * *Ad.offset(p as isize);
        }
        p += 1;
        p;
    }
    return nz;
}