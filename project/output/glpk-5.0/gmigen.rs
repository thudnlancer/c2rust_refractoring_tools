#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types)]
extern "C" {
    pub type AVL;
    pub type AVLNODE;
    pub type BFD;
    pub type DMP;
    pub type glp_tree;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn glp_error_(file: *const i8, line: i32) -> glp_errfunc;
    fn glp_alloc(n: i32, size: i32) -> *mut libc::c_void;
    fn glp_free(ptr: *mut libc::c_void);
    fn glp_add_rows(P: *mut glp_prob, nrs: i32) -> i32;
    fn glp_set_row_bnds(
        P: *mut glp_prob,
        i: i32,
        type_0: i32,
        lb: libc::c_double,
        ub: libc::c_double,
    );
    fn glp_set_mat_row(
        P: *mut glp_prob,
        i: i32,
        len: i32,
        ind: *const i32,
        val: *const libc::c_double,
    );
    fn glp_gmi_cut(
        P: *mut glp_prob,
        j: i32,
        ind: *mut i32,
        val: *mut libc::c_double,
        phi: *mut libc::c_double,
    ) -> i32;
}
pub type size_t = u64;
pub type __compar_fn_t = Option<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32,
>;
pub type glp_errfunc = Option<unsafe extern "C" fn(*const i8, ...) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glp_prob {
    pub pool: *mut DMP,
    pub tree: *mut glp_tree,
    pub name: *mut i8,
    pub obj: *mut i8,
    pub dir: i32,
    pub c0: libc::c_double,
    pub m_max: i32,
    pub n_max: i32,
    pub m: i32,
    pub n: i32,
    pub nnz: i32,
    pub row: *mut *mut GLPROW,
    pub col: *mut *mut GLPCOL,
    pub r_tree: *mut AVL,
    pub c_tree: *mut AVL,
    pub valid: i32,
    pub head: *mut i32,
    pub bfd: *mut BFD,
    pub pbs_stat: i32,
    pub dbs_stat: i32,
    pub obj_val: libc::c_double,
    pub it_cnt: i32,
    pub some: i32,
    pub ipt_stat: i32,
    pub ipt_obj: libc::c_double,
    pub mip_stat: i32,
    pub mip_obj: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GLPCOL {
    pub j: i32,
    pub name: *mut i8,
    pub node: *mut AVLNODE,
    pub kind: i32,
    pub type_0: i32,
    pub lb: libc::c_double,
    pub ub: libc::c_double,
    pub coef: libc::c_double,
    pub ptr: *mut GLPAIJ,
    pub sjj: libc::c_double,
    pub stat: i32,
    pub bind: i32,
    pub prim: libc::c_double,
    pub dual: libc::c_double,
    pub pval: libc::c_double,
    pub dval: libc::c_double,
    pub mipx: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GLPAIJ {
    pub row: *mut GLPROW,
    pub col: *mut GLPCOL,
    pub val: libc::c_double,
    pub r_prev: *mut GLPAIJ,
    pub r_next: *mut GLPAIJ,
    pub c_prev: *mut GLPAIJ,
    pub c_next: *mut GLPAIJ,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GLPROW {
    pub i: i32,
    pub name: *mut i8,
    pub node: *mut AVLNODE,
    pub level: i32,
    pub origin: u8,
    pub klass: u8,
    pub type_0: i32,
    pub lb: libc::c_double,
    pub ub: libc::c_double,
    pub ptr: *mut GLPAIJ,
    pub rii: libc::c_double,
    pub stat: i32,
    pub bind: i32,
    pub prim: libc::c_double,
    pub dual: libc::c_double,
    pub pval: libc::c_double,
    pub dval: libc::c_double,
    pub mipx: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct var {
    pub j: i32,
    pub f: libc::c_double,
}
unsafe extern "C" fn fcmp(
    mut p1: *const libc::c_void,
    mut p2: *const libc::c_void,
) -> i32 {
    let mut v1: *const var = p1 as *const var;
    let mut v2: *const var = p2 as *const var;
    if (*v1).f > (*v2).f {
        return -(1 as i32);
    }
    if (*v1).f < (*v2).f {
        return 1 as i32;
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn glp_gmi_gen(
    mut P: *mut glp_prob,
    mut pool: *mut glp_prob,
    mut max_cuts: i32,
) -> i32 {
    let mut current_block: u64;
    let mut m: i32 = (*P).m;
    let mut n: i32 = (*P).n;
    let mut col: *mut GLPCOL = 0 as *mut GLPCOL;
    let mut var: *mut var = 0 as *mut var;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut t: i32 = 0;
    let mut len: i32 = 0;
    let mut nv: i32 = 0;
    let mut nnn: i32 = 0;
    let mut ind: *mut i32 = 0 as *mut i32;
    let mut frac: libc::c_double = 0.;
    let mut val: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut phi: *mut libc::c_double = 0 as *mut libc::c_double;
    if !((*P).m == 0 as i32 || (*P).valid != 0) {
        (glp_error_(b"intopt/gmigen.c\0" as *const u8 as *const i8, 77 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_gmi_gen: basis factorization does not exist\n\0" as *const u8
                as *const i8,
        );
    }
    if !((*P).pbs_stat == 2 as i32 && (*P).dbs_stat == 2 as i32) {
        (glp_error_(b"intopt/gmigen.c\0" as *const u8 as *const i8, 79 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_gmi_gen: optimal basic solution required\n\0" as *const u8 as *const i8,
        );
    }
    if (*pool).n != n {
        (glp_error_(b"intopt/gmigen.c\0" as *const u8 as *const i8, 81 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_gmi_gen: cut pool has wrong number of columns\n\0" as *const u8
                as *const i8,
        );
    }
    var = glp_alloc(1 as i32 + n, ::core::mem::size_of::<var>() as u64 as i32)
        as *mut var;
    ind = glp_alloc(1 as i32 + n, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    val = glp_alloc(1 as i32 + n, ::core::mem::size_of::<libc::c_double>() as u64 as i32)
        as *mut libc::c_double;
    phi = glp_alloc(
        1 as i32 + m + n,
        ::core::mem::size_of::<libc::c_double>() as u64 as i32,
    ) as *mut libc::c_double;
    nv = 0 as i32;
    j = 1 as i32;
    while j <= n {
        col = *((*P).col).offset(j as isize);
        if !((*col).kind != 2 as i32) {
            if !((*col).type_0 == 5 as i32) {
                if !((*col).stat != 1 as i32) {
                    frac = (*col).prim - floor((*col).prim);
                    if 0.05f64 <= frac && frac <= 0.95f64 {
                        nv += 1;
                        nv;
                        (*var.offset(nv as isize)).j = j;
                        (*var.offset(nv as isize)).f = frac;
                    }
                }
            }
        }
        j += 1;
        j;
    }
    qsort(
        &mut *var.offset(1 as i32 as isize) as *mut var as *mut libc::c_void,
        nv as size_t,
        ::core::mem::size_of::<var>() as u64,
        Some(
            fcmp as unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32,
        ),
    );
    nnn = 0 as i32;
    t = 1 as i32;
    while t <= nv {
        len = glp_gmi_cut(P, (*var.offset(t as isize)).j, ind, val, phi);
        if !(len < 1 as i32) {
            k = 1 as i32;
            loop {
                if !(k <= len) {
                    current_block = 13550086250199790493;
                    break;
                }
                if fabs(*val.offset(k as isize)) < 1e-03f64 {
                    current_block = 11584701595673473500;
                    break;
                }
                if fabs(*val.offset(k as isize)) > 1e+03f64 {
                    current_block = 11584701595673473500;
                    break;
                }
                k += 1;
                k;
            }
            match current_block {
                11584701595673473500 => {}
                _ => {
                    i = glp_add_rows(pool, 1 as i32);
                    glp_set_row_bnds(
                        pool,
                        i,
                        2 as i32,
                        *val.offset(0 as i32 as isize),
                        0 as i32 as libc::c_double,
                    );
                    glp_set_mat_row(
                        pool,
                        i,
                        len,
                        ind as *const i32,
                        val as *const libc::c_double,
                    );
                    nnn += 1;
                    nnn;
                    if nnn == max_cuts {
                        break;
                    }
                }
            }
        }
        t += 1;
        t;
    }
    glp_free(var as *mut libc::c_void);
    glp_free(ind as *mut libc::c_void);
    glp_free(val as *mut libc::c_void);
    glp_free(phi as *mut libc::c_void);
    return nnn;
}