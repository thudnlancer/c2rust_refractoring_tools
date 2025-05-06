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
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
    fn glp_error_(file: *const i8, line: i32) -> glp_errfunc;
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
pub unsafe extern "C" fn glp_get_prob_name(mut lp: *mut glp_prob) -> *const i8 {
    let mut name: *mut i8 = 0 as *mut i8;
    name = (*lp).name;
    return name;
}
#[no_mangle]
pub unsafe extern "C" fn glp_get_obj_name(mut lp: *mut glp_prob) -> *const i8 {
    let mut name: *mut i8 = 0 as *mut i8;
    name = (*lp).obj;
    return name;
}
#[no_mangle]
pub unsafe extern "C" fn glp_get_obj_dir(mut lp: *mut glp_prob) -> i32 {
    let mut dir: i32 = (*lp).dir;
    return dir;
}
#[no_mangle]
pub unsafe extern "C" fn glp_get_num_rows(mut lp: *mut glp_prob) -> i32 {
    let mut m: i32 = (*lp).m;
    return m;
}
#[no_mangle]
pub unsafe extern "C" fn glp_get_num_cols(mut lp: *mut glp_prob) -> i32 {
    let mut n: i32 = (*lp).n;
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn glp_get_row_name(
    mut lp: *mut glp_prob,
    mut i: i32,
) -> *const i8 {
    let mut name: *mut i8 = 0 as *mut i8;
    if !(1 as i32 <= i && i <= (*lp).m) {
        (glp_error_(b"api/prob2.c\0" as *const u8 as *const i8, 146 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_get_row_name: i = %d; row number out of range\n\0" as *const u8
                as *const i8,
            i,
        );
    }
    name = (**((*lp).row).offset(i as isize)).name;
    return name;
}
#[no_mangle]
pub unsafe extern "C" fn glp_get_col_name(
    mut lp: *mut glp_prob,
    mut j: i32,
) -> *const i8 {
    let mut name: *mut i8 = 0 as *mut i8;
    if !(1 as i32 <= j && j <= (*lp).n) {
        (glp_error_(b"api/prob2.c\0" as *const u8 as *const i8, 170 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_get_col_name: j = %d; column number out of range\n\0" as *const u8
                as *const i8,
            j,
        );
    }
    name = (**((*lp).col).offset(j as isize)).name;
    return name;
}
#[no_mangle]
pub unsafe extern "C" fn glp_get_row_type(mut lp: *mut glp_prob, mut i: i32) -> i32 {
    if !(1 as i32 <= i && i <= (*lp).m) {
        (glp_error_(b"api/prob2.c\0" as *const u8 as *const i8, 198 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_get_row_type: i = %d; row number out of range\n\0" as *const u8
                as *const i8,
            i,
        );
    }
    return (**((*lp).row).offset(i as isize)).type_0;
}
#[no_mangle]
pub unsafe extern "C" fn glp_get_row_lb(
    mut lp: *mut glp_prob,
    mut i: i32,
) -> libc::c_double {
    let mut lb: libc::c_double = 0.;
    if !(1 as i32 <= i && i <= (*lp).m) {
        (glp_error_(b"api/prob2.c\0" as *const u8 as *const i8, 221 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_get_row_lb: i = %d; row number out of range\n\0" as *const u8
                as *const i8,
            i,
        );
    }
    match (**((*lp).row).offset(i as isize)).type_0 {
        1 | 3 => {
            lb = -1.7976931348623157e+308f64;
        }
        2 | 4 | 5 => {
            lb = (**((*lp).row).offset(i as isize)).lb;
        }
        _ => {
            (lp != lp
                || {
                    glp_assert_(
                        b"lp != lp\0" as *const u8 as *const i8,
                        b"api/prob2.c\0" as *const u8 as *const i8,
                        231 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        }
    }
    return lb;
}
#[no_mangle]
pub unsafe extern "C" fn glp_get_row_ub(
    mut lp: *mut glp_prob,
    mut i: i32,
) -> libc::c_double {
    let mut ub: libc::c_double = 0.;
    if !(1 as i32 <= i && i <= (*lp).m) {
        (glp_error_(b"api/prob2.c\0" as *const u8 as *const i8, 254 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_get_row_ub: i = %d; row number out of range\n\0" as *const u8
                as *const i8,
            i,
        );
    }
    match (**((*lp).row).offset(i as isize)).type_0 {
        1 | 2 => {
            ub = 1.7976931348623157e+308f64;
        }
        3 | 4 | 5 => {
            ub = (**((*lp).row).offset(i as isize)).ub;
        }
        _ => {
            (lp != lp
                || {
                    glp_assert_(
                        b"lp != lp\0" as *const u8 as *const i8,
                        b"api/prob2.c\0" as *const u8 as *const i8,
                        264 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        }
    }
    return ub;
}
#[no_mangle]
pub unsafe extern "C" fn glp_get_col_type(mut lp: *mut glp_prob, mut j: i32) -> i32 {
    if !(1 as i32 <= j && j <= (*lp).n) {
        (glp_error_(b"api/prob2.c\0" as *const u8 as *const i8, 291 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_get_col_type: j = %d; column number out of range\n\0" as *const u8
                as *const i8,
            j,
        );
    }
    return (**((*lp).col).offset(j as isize)).type_0;
}
#[no_mangle]
pub unsafe extern "C" fn glp_get_col_lb(
    mut lp: *mut glp_prob,
    mut j: i32,
) -> libc::c_double {
    let mut lb: libc::c_double = 0.;
    if !(1 as i32 <= j && j <= (*lp).n) {
        (glp_error_(b"api/prob2.c\0" as *const u8 as *const i8, 314 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_get_col_lb: j = %d; column number out of range\n\0" as *const u8
                as *const i8,
            j,
        );
    }
    match (**((*lp).col).offset(j as isize)).type_0 {
        1 | 3 => {
            lb = -1.7976931348623157e+308f64;
        }
        2 | 4 | 5 => {
            lb = (**((*lp).col).offset(j as isize)).lb;
        }
        _ => {
            (lp != lp
                || {
                    glp_assert_(
                        b"lp != lp\0" as *const u8 as *const i8,
                        b"api/prob2.c\0" as *const u8 as *const i8,
                        325 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        }
    }
    return lb;
}
#[no_mangle]
pub unsafe extern "C" fn glp_get_col_ub(
    mut lp: *mut glp_prob,
    mut j: i32,
) -> libc::c_double {
    let mut ub: libc::c_double = 0.;
    if !(1 as i32 <= j && j <= (*lp).n) {
        (glp_error_(b"api/prob2.c\0" as *const u8 as *const i8, 348 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_get_col_ub: j = %d; column number out of range\n\0" as *const u8
                as *const i8,
            j,
        );
    }
    match (**((*lp).col).offset(j as isize)).type_0 {
        1 | 2 => {
            ub = 1.7976931348623157e+308f64;
        }
        3 | 4 | 5 => {
            ub = (**((*lp).col).offset(j as isize)).ub;
        }
        _ => {
            (lp != lp
                || {
                    glp_assert_(
                        b"lp != lp\0" as *const u8 as *const i8,
                        b"api/prob2.c\0" as *const u8 as *const i8,
                        359 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        }
    }
    return ub;
}
#[no_mangle]
pub unsafe extern "C" fn glp_get_obj_coef(
    mut lp: *mut glp_prob,
    mut j: i32,
) -> libc::c_double {
    if !(0 as i32 <= j && j <= (*lp).n) {
        (glp_error_(b"api/prob2.c\0" as *const u8 as *const i8, 383 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_get_obj_coef: j = %d; column number out of range\n\0" as *const u8
                as *const i8,
            j,
        );
    }
    return if j == 0 as i32 {
        (*lp).c0
    } else {
        (**((*lp).col).offset(j as isize)).coef
    };
}
#[no_mangle]
pub unsafe extern "C" fn glp_get_num_nz(mut lp: *mut glp_prob) -> i32 {
    let mut nnz: i32 = (*lp).nnz;
    return nnz;
}
#[no_mangle]
pub unsafe extern "C" fn glp_get_mat_row(
    mut lp: *mut glp_prob,
    mut i: i32,
    mut ind: *mut i32,
    mut val: *mut libc::c_double,
) -> i32 {
    let mut aij: *mut GLPAIJ = 0 as *mut GLPAIJ;
    let mut len: i32 = 0;
    if !(1 as i32 <= i && i <= (*lp).m) {
        (glp_error_(b"api/prob2.c\0" as *const u8 as *const i8, 436 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_get_mat_row: i = %d; row number out of range\n\0" as *const u8
                as *const i8,
            i,
        );
    }
    len = 0 as i32;
    aij = (**((*lp).row).offset(i as isize)).ptr;
    while !aij.is_null() {
        len += 1;
        len;
        if !ind.is_null() {
            *ind.offset(len as isize) = (*(*aij).col).j;
        }
        if !val.is_null() {
            *val.offset(len as isize) = (*aij).val;
        }
        aij = (*aij).r_next;
    }
    (len <= (*lp).n
        || {
            glp_assert_(
                b"len <= lp->n\0" as *const u8 as *const i8,
                b"api/prob2.c\0" as *const u8 as *const i8,
                444 as i32,
            );
            1 as i32 != 0
        }) as i32;
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn glp_get_mat_col(
    mut lp: *mut glp_prob,
    mut j: i32,
    mut ind: *mut i32,
    mut val: *mut libc::c_double,
) -> i32 {
    let mut aij: *mut GLPAIJ = 0 as *mut GLPAIJ;
    let mut len: i32 = 0;
    if !(1 as i32 <= j && j <= (*lp).n) {
        (glp_error_(b"api/prob2.c\0" as *const u8 as *const i8, 477 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_get_mat_col: j = %d; column number out of range\n\0" as *const u8
                as *const i8,
            j,
        );
    }
    len = 0 as i32;
    aij = (**((*lp).col).offset(j as isize)).ptr;
    while !aij.is_null() {
        len += 1;
        len;
        if !ind.is_null() {
            *ind.offset(len as isize) = (*(*aij).row).i;
        }
        if !val.is_null() {
            *val.offset(len as isize) = (*aij).val;
        }
        aij = (*aij).c_next;
    }
    (len <= (*lp).m
        || {
            glp_assert_(
                b"len <= lp->m\0" as *const u8 as *const i8,
                b"api/prob2.c\0" as *const u8 as *const i8,
                485 as i32,
            );
            1 as i32 != 0
        }) as i32;
    return len;
}