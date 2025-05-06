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
    fn glp_error_(file: *const i8, line: i32) -> glp_errfunc;
    fn glp_get_num_rows(P: *mut glp_prob) -> i32;
    fn glp_get_num_cols(P: *mut glp_prob) -> i32;
}
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
#[no_mangle]
pub unsafe extern "C" fn glp_set_rii(
    mut lp: *mut glp_prob,
    mut i: i32,
    mut rii: libc::c_double,
) {
    if !(1 as i32 <= i && i <= (*lp).m) {
        (glp_error_(b"api/prob4.c\0" as *const u8 as *const i8, 41 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_set_rii: i = %d; row number out of range\n\0" as *const u8
                as *const i8,
            i,
        );
    }
    if rii <= 0.0f64 {
        (glp_error_(b"api/prob4.c\0" as *const u8 as *const i8, 43 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_set_rii: i = %d; rii = %g; invalid scale factor\n\0" as *const u8
                as *const i8,
            i,
            rii,
        );
    }
    if (*lp).valid != 0 && (**((*lp).row).offset(i as isize)).rii != rii {
        let mut aij: *mut GLPAIJ = 0 as *mut GLPAIJ;
        aij = (**((*lp).row).offset(i as isize)).ptr;
        while !aij.is_null() {
            if (*(*aij).col).stat == 1 as i32 {
                (*lp).valid = 0 as i32;
                break;
            } else {
                aij = (*aij).r_next;
            }
        }
    }
    (**((*lp).row).offset(i as isize)).rii = rii;
}
#[no_mangle]
pub unsafe extern "C" fn glp_set_sjj(
    mut lp: *mut glp_prob,
    mut j: i32,
    mut sjj: libc::c_double,
) {
    if !(1 as i32 <= j && j <= (*lp).n) {
        (glp_error_(b"api/prob4.c\0" as *const u8 as *const i8, 75 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_set_sjj: j = %d; column number out of range\n\0" as *const u8
                as *const i8,
            j,
        );
    }
    if sjj <= 0.0f64 {
        (glp_error_(b"api/prob4.c\0" as *const u8 as *const i8, 77 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_set_sjj: j = %d; sjj = %g; invalid scale factor\n\0" as *const u8
                as *const i8,
            j,
            sjj,
        );
    }
    if (*lp).valid != 0 && (**((*lp).col).offset(j as isize)).sjj != sjj
        && (**((*lp).col).offset(j as isize)).stat == 1 as i32
    {
        (*lp).valid = 0 as i32;
    }
    (**((*lp).col).offset(j as isize)).sjj = sjj;
}
#[no_mangle]
pub unsafe extern "C" fn glp_get_rii(
    mut lp: *mut glp_prob,
    mut i: i32,
) -> libc::c_double {
    if !(1 as i32 <= i && i <= (*lp).m) {
        (glp_error_(b"api/prob4.c\0" as *const u8 as *const i8, 104 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_get_rii: i = %d; row number out of range\n\0" as *const u8
                as *const i8,
            i,
        );
    }
    return (**((*lp).row).offset(i as isize)).rii;
}
#[no_mangle]
pub unsafe extern "C" fn glp_get_sjj(
    mut lp: *mut glp_prob,
    mut j: i32,
) -> libc::c_double {
    if !(1 as i32 <= j && j <= (*lp).n) {
        (glp_error_(b"api/prob4.c\0" as *const u8 as *const i8, 124 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_get_sjj: j = %d; column number out of range\n\0" as *const u8
                as *const i8,
            j,
        );
    }
    return (**((*lp).col).offset(j as isize)).sjj;
}
#[no_mangle]
pub unsafe extern "C" fn glp_unscale_prob(mut lp: *mut glp_prob) {
    let mut m: i32 = glp_get_num_rows(lp);
    let mut n: i32 = glp_get_num_cols(lp);
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    i = 1 as i32;
    while i <= m {
        glp_set_rii(lp, i, 1.0f64);
        i += 1;
        i;
    }
    j = 1 as i32;
    while j <= n {
        glp_set_sjj(lp, j, 1.0f64);
        j += 1;
        j;
    }
}