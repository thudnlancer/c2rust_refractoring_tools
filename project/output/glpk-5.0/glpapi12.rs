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
    pub type BFD;
    pub type AVL;
    pub type AVLNODE;
    pub type glp_tree;
    pub type DMP;
    fn glp_get_obj_dir(P: *mut glp_prob) -> i32;
    fn glp_get_num_rows(P: *mut glp_prob) -> i32;
    fn glp_get_num_cols(P: *mut glp_prob) -> i32;
    fn glp_get_row_type(P: *mut glp_prob, i: i32) -> i32;
    fn glp_get_row_lb(P: *mut glp_prob, i: i32) -> libc::c_double;
    fn glp_get_row_ub(P: *mut glp_prob, i: i32) -> libc::c_double;
    fn glp_get_col_type(P: *mut glp_prob, j: i32) -> i32;
    fn glp_get_col_lb(P: *mut glp_prob, j: i32) -> libc::c_double;
    fn glp_get_col_ub(P: *mut glp_prob, j: i32) -> libc::c_double;
    fn glp_get_mat_col(
        P: *mut glp_prob,
        j: i32,
        ind: *mut i32,
        val: *mut libc::c_double,
    ) -> i32;
    fn glp_get_prim_stat(P: *mut glp_prob) -> i32;
    fn glp_get_dual_stat(P: *mut glp_prob) -> i32;
    fn glp_get_row_stat(P: *mut glp_prob, i: i32) -> i32;
    fn glp_get_row_prim(P: *mut glp_prob, i: i32) -> libc::c_double;
    fn glp_get_row_dual(P: *mut glp_prob, i: i32) -> libc::c_double;
    fn glp_get_col_stat(P: *mut glp_prob, j: i32) -> i32;
    fn glp_get_col_prim(P: *mut glp_prob, j: i32) -> libc::c_double;
    fn glp_get_col_dual(P: *mut glp_prob, j: i32) -> libc::c_double;
    fn glp_error_(file: *const i8, line: i32) -> glp_errfunc;
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
    fn glp_alloc(n: i32, size: i32) -> *mut libc::c_void;
    fn glp_free(ptr: *mut libc::c_void);
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn _glp_bfd_get_bfcp(bfd: *mut BFD, parm: *mut libc::c_void);
    fn _glp_bfd_set_bfcp(bfd: *mut BFD, parm: *const libc::c_void);
    fn _glp_bfd_create_it() -> *mut BFD;
    fn _glp_bfd_factorize(
        bfd: *mut BFD,
        m: i32,
        col: Option<
            unsafe extern "C" fn(
                *mut libc::c_void,
                i32,
                *mut i32,
                *mut libc::c_double,
            ) -> i32,
        >,
        info: *mut libc::c_void,
    ) -> i32;
    fn _glp_bfd_ftran(bfd: *mut BFD, x: *mut libc::c_double);
    fn _glp_bfd_btran(bfd: *mut BFD, x: *mut libc::c_double);
    fn _glp_bfd_get_count(bfd: *mut BFD) -> i32;
}
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
pub struct glp_bfcp {
    pub msg_lev: i32,
    pub type_0: i32,
    pub lu_size: i32,
    pub piv_tol: libc::c_double,
    pub piv_lim: i32,
    pub suhl: i32,
    pub eps_tol: libc::c_double,
    pub max_gro: libc::c_double,
    pub nfs_max: i32,
    pub upd_tol: libc::c_double,
    pub nrs_max: i32,
    pub rs_size: i32,
    pub foo_bar: [libc::c_double; 38],
}
pub type glp_errfunc = Option<unsafe extern "C" fn(*const i8, ...) -> ()>;
#[no_mangle]
pub unsafe extern "C" fn glp_bf_exists(mut lp: *mut glp_prob) -> i32 {
    let mut ret: i32 = 0;
    ret = ((*lp).m == 0 as i32 || (*lp).valid != 0) as i32;
    return ret;
}
unsafe extern "C" fn b_col(
    mut info: *mut libc::c_void,
    mut j: i32,
    mut ind: *mut i32,
    mut val: *mut libc::c_double,
) -> i32 {
    let mut lp: *mut glp_prob = info as *mut glp_prob;
    let mut m: i32 = (*lp).m;
    let mut aij: *mut GLPAIJ = 0 as *mut GLPAIJ;
    let mut k: i32 = 0;
    let mut len: i32 = 0;
    (1 as i32 <= j && j <= m
        || {
            glp_assert_(
                b"1 <= j && j <= m\0" as *const u8 as *const i8,
                b"draft/glpapi12.c\0" as *const u8 as *const i8,
                82 as i32,
            );
            1 as i32 != 0
        }) as i32;
    k = *((*lp).head).offset(j as isize);
    if k <= m {
        len = 1 as i32;
        *ind.offset(1 as i32 as isize) = k;
        *val.offset(1 as i32 as isize) = 1.0f64;
    } else {
        len = 0 as i32;
        aij = (**((*lp).col).offset((k - m) as isize)).ptr;
        while !aij.is_null() {
            len += 1;
            len;
            *ind.offset(len as isize) = (*(*aij).row).i;
            *val.offset(len as isize) = -(*(*aij).row).rii * (*aij).val
                * (*(*aij).col).sjj;
            aij = (*aij).c_next;
        }
    }
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn glp_factorize(mut lp: *mut glp_prob) -> i32 {
    let mut current_block: u64;
    let mut m: i32 = (*lp).m;
    let mut n: i32 = (*lp).n;
    let mut row: *mut *mut GLPROW = (*lp).row;
    let mut col: *mut *mut GLPCOL = (*lp).col;
    let mut head: *mut i32 = (*lp).head;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut stat: i32 = 0;
    let mut ret: i32 = 0;
    (*lp).valid = 0 as i32;
    j = 0 as i32;
    k = 1 as i32;
    loop {
        if !(k <= m + n) {
            current_block = 5689001924483802034;
            break;
        }
        if k <= m {
            stat = (**row.offset(k as isize)).stat;
            (**row.offset(k as isize)).bind = 0 as i32;
        } else {
            stat = (**col.offset((k - m) as isize)).stat;
            (**col.offset((k - m) as isize)).bind = 0 as i32;
        }
        if stat == 1 as i32 {
            j += 1;
            j;
            if j > m {
                ret = 0x1 as i32;
                current_block = 7350059504414396885;
                break;
            } else {
                *head.offset(j as isize) = k;
                if k <= m {
                    (**row.offset(k as isize)).bind = j;
                } else {
                    (**col.offset((k - m) as isize)).bind = j;
                }
            }
        }
        k += 1;
        k;
    }
    match current_block {
        5689001924483802034 => {
            if j < m {
                ret = 0x1 as i32;
            } else {
                if m > 0 as i32 {
                    if ((*lp).bfd).is_null() {
                        (*lp).bfd = _glp_bfd_create_it();
                    }
                    match _glp_bfd_factorize(
                        (*lp).bfd,
                        m,
                        Some(
                            b_col
                                as unsafe extern "C" fn(
                                    *mut libc::c_void,
                                    i32,
                                    *mut i32,
                                    *mut libc::c_double,
                                ) -> i32,
                        ),
                        lp as *mut libc::c_void,
                    ) {
                        0 => {
                            current_block = 16924917904204750491;
                        }
                        1 => {
                            ret = 0x2 as i32;
                            current_block = 7350059504414396885;
                        }
                        2 => {
                            ret = 0x3 as i32;
                            current_block = 7350059504414396885;
                        }
                        _ => {
                            (lp != lp
                                || {
                                    glp_assert_(
                                        b"lp != lp\0" as *const u8 as *const i8,
                                        b"draft/glpapi12.c\0" as *const u8 as *const i8,
                                        166 as i32,
                                    );
                                    1 as i32 != 0
                                }) as i32;
                            current_block = 16924917904204750491;
                        }
                    }
                    match current_block {
                        7350059504414396885 => {}
                        _ => {
                            (*lp).valid = 1 as i32;
                            current_block = 17788412896529399552;
                        }
                    }
                } else {
                    current_block = 17788412896529399552;
                }
                match current_block {
                    7350059504414396885 => {}
                    _ => {
                        ret = 0 as i32;
                    }
                }
            }
        }
        _ => {}
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn glp_bf_updated(mut lp: *mut glp_prob) -> i32 {
    let mut cnt: i32 = 0;
    if !((*lp).m == 0 as i32 || (*lp).valid != 0) {
        (glp_error_(b"draft/glpapi12.c\0" as *const u8 as *const i8, 194 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_bf_update: basis factorization does not exist\n\0" as *const u8
                as *const i8,
        );
    }
    cnt = if (*lp).m == 0 as i32 { 0 as i32 } else { _glp_bfd_get_count((*lp).bfd) };
    return cnt;
}
#[no_mangle]
pub unsafe extern "C" fn glp_get_bfcp(mut P: *mut glp_prob, mut parm: *mut glp_bfcp) {
    if ((*P).bfd).is_null() {
        (*P).bfd = _glp_bfd_create_it();
    }
    _glp_bfd_get_bfcp((*P).bfd, parm as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn glp_set_bfcp(mut P: *mut glp_prob, mut parm: *const glp_bfcp) {
    if ((*P).bfd).is_null() {
        (*P).bfd = _glp_bfd_create_it();
    }
    if !parm.is_null() {
        if !((*parm).type_0 == 0 as i32 + 0x1 as i32
            || (*parm).type_0 == 0 as i32 + 0x2 as i32
            || (*parm).type_0 == 0 as i32 + 0x3 as i32
            || (*parm).type_0 == 0x10 as i32 + 0x2 as i32
            || (*parm).type_0 == 0x10 as i32 + 0x3 as i32)
        {
            (glp_error_(b"draft/glpapi12.c\0" as *const u8 as *const i8, 261 as i32))
                .expect(
                    "non-null function pointer",
                )(
                b"glp_set_bfcp: type = 0x%02X; invalid parameter\n\0" as *const u8
                    as *const i8,
                (*parm).type_0,
            );
        }
        if !(0.0f64 < (*parm).piv_tol && (*parm).piv_tol < 1.0f64) {
            (glp_error_(b"draft/glpapi12.c\0" as *const u8 as *const i8, 264 as i32))
                .expect(
                    "non-null function pointer",
                )(
                b"glp_set_bfcp: piv_tol = %g; invalid parameter\n\0" as *const u8
                    as *const i8,
                (*parm).piv_tol,
            );
        }
        if (*parm).piv_lim < 1 as i32 {
            (glp_error_(b"draft/glpapi12.c\0" as *const u8 as *const i8, 267 as i32))
                .expect(
                    "non-null function pointer",
                )(
                b"glp_set_bfcp: piv_lim = %d; invalid parameter\n\0" as *const u8
                    as *const i8,
                (*parm).piv_lim,
            );
        }
        if !((*parm).suhl == 1 as i32 || (*parm).suhl == 0 as i32) {
            (glp_error_(b"draft/glpapi12.c\0" as *const u8 as *const i8, 270 as i32))
                .expect(
                    "non-null function pointer",
                )(
                b"glp_set_bfcp: suhl = %d; invalid parameter\n\0" as *const u8
                    as *const i8,
                (*parm).suhl,
            );
        }
        if !(0.0f64 <= (*parm).eps_tol && (*parm).eps_tol <= 1e-6f64) {
            (glp_error_(b"draft/glpapi12.c\0" as *const u8 as *const i8, 273 as i32))
                .expect(
                    "non-null function pointer",
                )(
                b"glp_set_bfcp: eps_tol = %g; invalid parameter\n\0" as *const u8
                    as *const i8,
                (*parm).eps_tol,
            );
        }
        if !(1 as i32 <= (*parm).nfs_max && (*parm).nfs_max <= 32767 as i32) {
            (glp_error_(b"draft/glpapi12.c\0" as *const u8 as *const i8, 276 as i32))
                .expect(
                    "non-null function pointer",
                )(
                b"glp_set_bfcp: nfs_max = %d; invalid parameter\n\0" as *const u8
                    as *const i8,
                (*parm).nfs_max,
            );
        }
        if !(1 as i32 <= (*parm).nrs_max && (*parm).nrs_max <= 32767 as i32) {
            (glp_error_(b"draft/glpapi12.c\0" as *const u8 as *const i8, 279 as i32))
                .expect(
                    "non-null function pointer",
                )(
                b"glp_set_bfcp: nrs_max = %d; invalid parameter\n\0" as *const u8
                    as *const i8,
                (*parm).nrs_max,
            );
        }
    }
    _glp_bfd_set_bfcp((*P).bfd, parm as *const libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn glp_get_bhead(mut lp: *mut glp_prob, mut k: i32) -> i32 {
    if !((*lp).m == 0 as i32 || (*lp).valid != 0) {
        (glp_error_(b"draft/glpapi12.c\0" as *const u8 as *const i8, 310 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_get_bhead: basis factorization does not exist\n\0" as *const u8
                as *const i8,
        );
    }
    if !(1 as i32 <= k && k <= (*lp).m) {
        (glp_error_(b"draft/glpapi12.c\0" as *const u8 as *const i8, 312 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_get_bhead: k = %d; index out of range\n\0" as *const u8 as *const i8,
            k,
        );
    }
    return *((*lp).head).offset(k as isize);
}
#[no_mangle]
pub unsafe extern "C" fn glp_get_row_bind(mut lp: *mut glp_prob, mut i: i32) -> i32 {
    if !((*lp).m == 0 as i32 || (*lp).valid != 0) {
        (glp_error_(b"draft/glpapi12.c\0" as *const u8 as *const i8, 335 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_get_row_bind: basis factorization does not exist\n\0" as *const u8
                as *const i8,
        );
    }
    if !(1 as i32 <= i && i <= (*lp).m) {
        (glp_error_(b"draft/glpapi12.c\0" as *const u8 as *const i8, 338 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_get_row_bind: i = %d; row number out of range\n\0" as *const u8
                as *const i8,
            i,
        );
    }
    return (**((*lp).row).offset(i as isize)).bind;
}
#[no_mangle]
pub unsafe extern "C" fn glp_get_col_bind(mut lp: *mut glp_prob, mut j: i32) -> i32 {
    if !((*lp).m == 0 as i32 || (*lp).valid != 0) {
        (glp_error_(b"draft/glpapi12.c\0" as *const u8 as *const i8, 362 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_get_col_bind: basis factorization does not exist\n\0" as *const u8
                as *const i8,
        );
    }
    if !(1 as i32 <= j && j <= (*lp).n) {
        (glp_error_(b"draft/glpapi12.c\0" as *const u8 as *const i8, 365 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_get_col_bind: j = %d; column number out of range\n\0" as *const u8
                as *const i8,
            j,
        );
    }
    return (**((*lp).col).offset(j as isize)).bind;
}
#[no_mangle]
pub unsafe extern "C" fn glp_ftran(mut lp: *mut glp_prob, mut x: *mut libc::c_double) {
    let mut m: i32 = (*lp).m;
    let mut row: *mut *mut GLPROW = (*lp).row;
    let mut col: *mut *mut GLPCOL = (*lp).col;
    let mut i: i32 = 0;
    let mut k: i32 = 0;
    if !(m == 0 as i32 || (*lp).valid != 0) {
        (glp_error_(b"draft/glpapi12.c\0" as *const u8 as *const i8, 432 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_ftran: basis factorization does not exist\n\0" as *const u8
                as *const i8,
        );
    }
    i = 1 as i32;
    while i <= m {
        *x.offset(i as isize) *= (**row.offset(i as isize)).rii;
        i += 1;
        i;
    }
    if m > 0 as i32 {
        _glp_bfd_ftran((*lp).bfd, x);
    }
    i = 1 as i32;
    while i <= m {
        k = *((*lp).head).offset(i as isize);
        if k <= m {
            *x.offset(i as isize) /= (**row.offset(k as isize)).rii;
        } else {
            *x.offset(i as isize) *= (**col.offset((k - m) as isize)).sjj;
        }
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn glp_btran(mut lp: *mut glp_prob, mut x: *mut libc::c_double) {
    let mut m: i32 = (*lp).m;
    let mut row: *mut *mut GLPROW = (*lp).row;
    let mut col: *mut *mut GLPCOL = (*lp).col;
    let mut i: i32 = 0;
    let mut k: i32 = 0;
    if !(m == 0 as i32 || (*lp).valid != 0) {
        (glp_error_(b"draft/glpapi12.c\0" as *const u8 as *const i8, 482 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_btran: basis factorization does not exist\n\0" as *const u8
                as *const i8,
        );
    }
    i = 1 as i32;
    while i <= m {
        k = *((*lp).head).offset(i as isize);
        if k <= m {
            *x.offset(i as isize) /= (**row.offset(k as isize)).rii;
        } else {
            *x.offset(i as isize) *= (**col.offset((k - m) as isize)).sjj;
        }
        i += 1;
        i;
    }
    if m > 0 as i32 {
        _glp_bfd_btran((*lp).bfd, x);
    }
    i = 1 as i32;
    while i <= m {
        *x.offset(i as isize) *= (**row.offset(i as isize)).rii;
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn glp_warm_up(mut P: *mut glp_prob) -> i32 {
    let mut current_block: u64;
    let mut row: *mut GLPROW = 0 as *mut GLPROW;
    let mut col: *mut GLPCOL = 0 as *mut GLPCOL;
    let mut aij: *mut GLPAIJ = 0 as *mut GLPAIJ;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut type_0: i32 = 0;
    let mut stat: i32 = 0;
    let mut ret: i32 = 0;
    let mut eps: libc::c_double = 0.;
    let mut temp: libc::c_double = 0.;
    let mut work: *mut libc::c_double = 0 as *mut libc::c_double;
    (*P).dbs_stat = 1 as i32;
    (*P).pbs_stat = (*P).dbs_stat;
    (*P).obj_val = 0.0f64;
    (*P).some = 0 as i32;
    i = 1 as i32;
    while i <= (*P).m {
        row = *((*P).row).offset(i as isize);
        (*row).dual = 0.0f64;
        (*row).prim = (*row).dual;
        i += 1;
        i;
    }
    j = 1 as i32;
    while j <= (*P).n {
        col = *((*P).col).offset(j as isize);
        (*col).dual = 0.0f64;
        (*col).prim = (*col).dual;
        j += 1;
        j;
    }
    if glp_bf_exists(P) == 0 {
        ret = glp_factorize(P);
        if ret != 0 as i32 {
            current_block = 3679754657907894490;
        } else {
            current_block = 17860125682698302841;
        }
    } else {
        current_block = 17860125682698302841;
    }
    match current_block {
        17860125682698302841 => {
            work = glp_alloc(
                1 as i32 + (*P).m,
                ::core::mem::size_of::<libc::c_double>() as u64 as i32,
            ) as *mut libc::c_double;
            i = 1 as i32;
            while i <= (*P).m {
                *work.offset(i as isize) = 0.0f64;
                i += 1;
                i;
            }
            i = 1 as i32;
            while i <= (*P).m {
                row = *((*P).row).offset(i as isize);
                if !((*row).stat == 1 as i32) {
                    if (*row).stat == 2 as i32 {
                        (*row).prim = (*row).lb;
                    } else if (*row).stat == 3 as i32 {
                        (*row).prim = (*row).ub;
                    } else if (*row).stat == 4 as i32 {
                        (*row).prim = 0.0f64;
                    } else if (*row).stat == 5 as i32 {
                        (*row).prim = (*row).lb;
                    } else {
                        (row != row
                            || {
                                glp_assert_(
                                    b"row != row\0" as *const u8 as *const i8,
                                    b"draft/glpapi12.c\0" as *const u8 as *const i8,
                                    575 as i32,
                                );
                                1 as i32 != 0
                            }) as i32;
                    }
                    *work.offset(i as isize) -= (*row).prim;
                }
                i += 1;
                i;
            }
            j = 1 as i32;
            while j <= (*P).n {
                col = *((*P).col).offset(j as isize);
                if !((*col).stat == 1 as i32) {
                    if (*col).stat == 2 as i32 {
                        (*col).prim = (*col).lb;
                    } else if (*col).stat == 3 as i32 {
                        (*col).prim = (*col).ub;
                    } else if (*col).stat == 4 as i32 {
                        (*col).prim = 0.0f64;
                    } else if (*col).stat == 5 as i32 {
                        (*col).prim = (*col).lb;
                    } else {
                        (col != col
                            || {
                                glp_assert_(
                                    b"col != col\0" as *const u8 as *const i8,
                                    b"draft/glpapi12.c\0" as *const u8 as *const i8,
                                    592 as i32,
                                );
                                1 as i32 != 0
                            }) as i32;
                    }
                    if (*col).prim != 0.0f64 {
                        aij = (*col).ptr;
                        while !aij.is_null() {
                            *work.offset((*(*aij).row).i as isize)
                                += (*aij).val * (*col).prim;
                            aij = (*aij).c_next;
                        }
                    }
                }
                j += 1;
                j;
            }
            glp_ftran(P, work);
            (*P).pbs_stat = 2 as i32;
            i = 1 as i32;
            while i <= (*P).m {
                row = *((*P).row).offset(i as isize);
                if !((*row).stat != 1 as i32) {
                    (*row).prim = *work.offset((*row).bind as isize);
                    type_0 = (*row).type_0;
                    if type_0 == 2 as i32 || type_0 == 4 as i32 || type_0 == 5 as i32 {
                        eps = 1e-6f64 + 1e-9f64 * fabs((*row).lb);
                        if (*row).prim < (*row).lb - eps {
                            (*P).pbs_stat = 3 as i32;
                        }
                    }
                    if type_0 == 3 as i32 || type_0 == 4 as i32 || type_0 == 5 as i32 {
                        eps = 1e-6f64 + 1e-9f64 * fabs((*row).ub);
                        if (*row).prim > (*row).ub + eps {
                            (*P).pbs_stat = 3 as i32;
                        }
                    }
                }
                i += 1;
                i;
            }
            j = 1 as i32;
            while j <= (*P).n {
                col = *((*P).col).offset(j as isize);
                if !((*col).stat != 1 as i32) {
                    (*col).prim = *work.offset((*col).bind as isize);
                    type_0 = (*col).type_0;
                    if type_0 == 2 as i32 || type_0 == 4 as i32 || type_0 == 5 as i32 {
                        eps = 1e-6f64 + 1e-9f64 * fabs((*col).lb);
                        if (*col).prim < (*col).lb - eps {
                            (*P).pbs_stat = 3 as i32;
                        }
                    }
                    if type_0 == 3 as i32 || type_0 == 4 as i32 || type_0 == 5 as i32 {
                        eps = 1e-6f64 + 1e-9f64 * fabs((*col).ub);
                        if (*col).prim > (*col).ub + eps {
                            (*P).pbs_stat = 3 as i32;
                        }
                    }
                }
                j += 1;
                j;
            }
            (*P).obj_val = (*P).c0;
            j = 1 as i32;
            while j <= (*P).n {
                col = *((*P).col).offset(j as isize);
                (*P).obj_val += (*col).coef * (*col).prim;
                j += 1;
                j;
            }
            i = 1 as i32;
            while i <= (*P).m {
                *work.offset(i as isize) = 0.0f64;
                i += 1;
                i;
            }
            j = 1 as i32;
            while j <= (*P).n {
                col = *((*P).col).offset(j as isize);
                if (*col).stat == 1 as i32 {
                    *work.offset((*col).bind as isize) = (*col).coef;
                }
                j += 1;
                j;
            }
            glp_btran(P, work);
            (*P).dbs_stat = 2 as i32;
            i = 1 as i32;
            while i <= (*P).m {
                row = *((*P).row).offset(i as isize);
                if (*row).stat == 1 as i32 {
                    (*row).dual = 0.0f64;
                } else {
                    (*row).dual = -*work.offset(i as isize);
                    stat = (*row).stat;
                    temp = if (*P).dir == 1 as i32 { (*row).dual } else { -(*row).dual };
                    if (stat == 4 as i32 || stat == 2 as i32) && temp < -1e-5f64
                        || (stat == 4 as i32 || stat == 3 as i32) && temp > 1e-5f64
                    {
                        (*P).dbs_stat = 3 as i32;
                    }
                }
                i += 1;
                i;
            }
            j = 1 as i32;
            while j <= (*P).n {
                col = *((*P).col).offset(j as isize);
                if (*col).stat == 1 as i32 {
                    (*col).dual = 0.0f64;
                } else {
                    (*col).dual = (*col).coef;
                    aij = (*col).ptr;
                    while !aij.is_null() {
                        (*col).dual
                            += (*aij).val * *work.offset((*(*aij).row).i as isize);
                        aij = (*aij).c_next;
                    }
                    stat = (*col).stat;
                    temp = if (*P).dir == 1 as i32 { (*col).dual } else { -(*col).dual };
                    if (stat == 4 as i32 || stat == 2 as i32) && temp < -1e-5f64
                        || (stat == 4 as i32 || stat == 3 as i32) && temp > 1e-5f64
                    {
                        (*P).dbs_stat = 3 as i32;
                    }
                }
                j += 1;
                j;
            }
            glp_free(work as *mut libc::c_void);
            ret = 0 as i32;
        }
        _ => {}
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn glp_eval_tab_row(
    mut lp: *mut glp_prob,
    mut k: i32,
    mut ind: *mut i32,
    mut val: *mut libc::c_double,
) -> i32 {
    let mut m: i32 = (*lp).m;
    let mut n: i32 = (*lp).n;
    let mut i: i32 = 0;
    let mut t: i32 = 0;
    let mut len: i32 = 0;
    let mut lll: i32 = 0;
    let mut iii: *mut i32 = 0 as *mut i32;
    let mut alfa: libc::c_double = 0.;
    let mut rho: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut vvv: *mut libc::c_double = 0 as *mut libc::c_double;
    if !(m == 0 as i32 || (*lp).valid != 0) {
        (glp_error_(b"draft/glpapi12.c\0" as *const u8 as *const i8, 808 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_eval_tab_row: basis factorization does not exist\n\0" as *const u8
                as *const i8,
        );
    }
    if !(1 as i32 <= k && k <= m + n) {
        (glp_error_(b"draft/glpapi12.c\0" as *const u8 as *const i8, 811 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_eval_tab_row: k = %d; variable number out of range\0" as *const u8
                as *const i8,
            k,
        );
    }
    if k <= m {
        i = glp_get_row_bind(lp, k);
    } else {
        i = glp_get_col_bind(lp, k - m);
    }
    if i == 0 as i32 {
        (glp_error_(b"draft/glpapi12.c\0" as *const u8 as *const i8, 819 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_eval_tab_row: k = %d; variable must be basic\0" as *const u8
                as *const i8,
            k,
        );
    }
    (1 as i32 <= i && i <= m
        || {
            glp_assert_(
                b"1 <= i && i <= m\0" as *const u8 as *const i8,
                b"draft/glpapi12.c\0" as *const u8 as *const i8,
                820 as i32,
            );
            1 as i32 != 0
        }) as i32;
    rho = glp_alloc(1 as i32 + m, ::core::mem::size_of::<libc::c_double>() as u64 as i32)
        as *mut libc::c_double;
    iii = glp_alloc(1 as i32 + m, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    vvv = glp_alloc(1 as i32 + m, ::core::mem::size_of::<libc::c_double>() as u64 as i32)
        as *mut libc::c_double;
    t = 1 as i32;
    while t <= m {
        *rho.offset(t as isize) = 0.0f64;
        t += 1;
        t;
    }
    *rho.offset(i as isize) = 1.0f64;
    glp_btran(lp, rho);
    len = 0 as i32;
    let mut current_block_27: u64;
    k = 1 as i32;
    while k <= m + n {
        if k <= m {
            if glp_get_row_stat(lp, k) == 1 as i32 {
                current_block_27 = 12039483399334584727;
            } else {
                alfa = -*rho.offset(k as isize);
                current_block_27 = 2232869372362427478;
            }
        } else if glp_get_col_stat(lp, k - m) == 1 as i32 {
            current_block_27 = 12039483399334584727;
        } else {
            lll = glp_get_mat_col(lp, k - m, iii, vvv);
            alfa = 0.0f64;
            t = 1 as i32;
            while t <= lll {
                alfa
                    += *rho.offset(*iii.offset(t as isize) as isize)
                        * *vvv.offset(t as isize);
                t += 1;
                t;
            }
            current_block_27 = 2232869372362427478;
        }
        match current_block_27 {
            2232869372362427478 => {
                if alfa != 0.0f64 {
                    len += 1;
                    len;
                    *ind.offset(len as isize) = k;
                    *val.offset(len as isize) = alfa;
                }
            }
            _ => {}
        }
        k += 1;
        k;
    }
    (len <= n
        || {
            glp_assert_(
                b"len <= n\0" as *const u8 as *const i8,
                b"draft/glpapi12.c\0" as *const u8 as *const i8,
                850 as i32,
            );
            1 as i32 != 0
        }) as i32;
    glp_free(rho as *mut libc::c_void);
    glp_free(iii as *mut libc::c_void);
    glp_free(vvv as *mut libc::c_void);
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn glp_eval_tab_col(
    mut lp: *mut glp_prob,
    mut k: i32,
    mut ind: *mut i32,
    mut val: *mut libc::c_double,
) -> i32 {
    let mut m: i32 = (*lp).m;
    let mut n: i32 = (*lp).n;
    let mut t: i32 = 0;
    let mut len: i32 = 0;
    let mut stat: i32 = 0;
    let mut col: *mut libc::c_double = 0 as *mut libc::c_double;
    if !(m == 0 as i32 || (*lp).valid != 0) {
        (glp_error_(b"draft/glpapi12.c\0" as *const u8 as *const i8, 923 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_eval_tab_col: basis factorization does not exist\n\0" as *const u8
                as *const i8,
        );
    }
    if !(1 as i32 <= k && k <= m + n) {
        (glp_error_(b"draft/glpapi12.c\0" as *const u8 as *const i8, 926 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_eval_tab_col: k = %d; variable number out of range\0" as *const u8
                as *const i8,
            k,
        );
    }
    if k <= m {
        stat = glp_get_row_stat(lp, k);
    } else {
        stat = glp_get_col_stat(lp, k - m);
    }
    if stat == 1 as i32 {
        (glp_error_(b"draft/glpapi12.c\0" as *const u8 as *const i8, 933 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_eval_tab_col: k = %d; variable must be non-basic\0" as *const u8
                as *const i8,
            k,
        );
    }
    col = glp_alloc(1 as i32 + m, ::core::mem::size_of::<libc::c_double>() as u64 as i32)
        as *mut libc::c_double;
    t = 1 as i32;
    while t <= m {
        *col.offset(t as isize) = 0.0f64;
        t += 1;
        t;
    }
    if k <= m {
        *col.offset(k as isize) = -1.0f64;
    } else {
        len = glp_get_mat_col(lp, k - m, ind, val);
        t = 1 as i32;
        while t <= len {
            *col.offset(*ind.offset(t as isize) as isize) = *val.offset(t as isize);
            t += 1;
            t;
        }
    }
    glp_ftran(lp, col);
    len = 0 as i32;
    t = 1 as i32;
    while t <= m {
        if *col.offset(t as isize) != 0.0f64 {
            len += 1;
            len;
            *ind.offset(len as isize) = glp_get_bhead(lp, t);
            *val.offset(len as isize) = *col.offset(t as isize);
        }
        t += 1;
        t;
    }
    glp_free(col as *mut libc::c_void);
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn glp_transform_row(
    mut P: *mut glp_prob,
    mut len: i32,
    mut ind: *mut i32,
    mut val: *mut libc::c_double,
) -> i32 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut m: i32 = 0;
    let mut n: i32 = 0;
    let mut t: i32 = 0;
    let mut lll: i32 = 0;
    let mut iii: *mut i32 = 0 as *mut i32;
    let mut alfa: libc::c_double = 0.;
    let mut a: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut aB: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut rho: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut vvv: *mut libc::c_double = 0 as *mut libc::c_double;
    if glp_bf_exists(P) == 0 {
        (glp_error_(b"draft/glpapi12.c\0" as *const u8 as *const i8, 1050 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_transform_row: basis factorization does not exist \n\0" as *const u8
                as *const i8,
        );
    }
    m = glp_get_num_rows(P);
    n = glp_get_num_cols(P);
    a = glp_alloc(1 as i32 + n, ::core::mem::size_of::<libc::c_double>() as u64 as i32)
        as *mut libc::c_double;
    j = 1 as i32;
    while j <= n {
        *a.offset(j as isize) = 0.0f64;
        j += 1;
        j;
    }
    if !(0 as i32 <= len && len <= n) {
        (glp_error_(b"draft/glpapi12.c\0" as *const u8 as *const i8, 1058 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_transform_row: len = %d; invalid row length\n\0" as *const u8
                as *const i8,
            len,
        );
    }
    t = 1 as i32;
    while t <= len {
        j = *ind.offset(t as isize);
        if !(1 as i32 <= j && j <= n) {
            (glp_error_(b"draft/glpapi12.c\0" as *const u8 as *const i8, 1063 as i32))
                .expect(
                    "non-null function pointer",
                )(
                b"glp_transform_row: ind[%d] = %d; column index out of range\n\0"
                    as *const u8 as *const i8,
                t,
                j,
            );
        }
        if *val.offset(t as isize) == 0.0f64 {
            (glp_error_(b"draft/glpapi12.c\0" as *const u8 as *const i8, 1066 as i32))
                .expect(
                    "non-null function pointer",
                )(
                b"glp_transform_row: val[%d] = 0; zero coefficient not allowed\n\0"
                    as *const u8 as *const i8,
                t,
            );
        }
        if *a.offset(j as isize) != 0.0f64 {
            (glp_error_(b"draft/glpapi12.c\0" as *const u8 as *const i8, 1069 as i32))
                .expect(
                    "non-null function pointer",
                )(
                b"glp_transform_row: ind[%d] = %d; duplicate column indices not allowed\n\0"
                    as *const u8 as *const i8,
                t,
                j,
            );
        }
        *a.offset(j as isize) = *val.offset(t as isize);
        t += 1;
        t;
    }
    aB = glp_alloc(1 as i32 + m, ::core::mem::size_of::<libc::c_double>() as u64 as i32)
        as *mut libc::c_double;
    i = 1 as i32;
    while i <= m {
        k = glp_get_bhead(P, i);
        (1 as i32 <= k && k <= m + n
            || {
                glp_assert_(
                    b"1 <= k && k <= m+n\0" as *const u8 as *const i8,
                    b"draft/glpapi12.c\0" as *const u8 as *const i8,
                    1078 as i32,
                );
                1 as i32 != 0
            }) as i32;
        *aB.offset(i as isize) = if k <= m {
            0.0f64
        } else {
            *a.offset((k - m) as isize)
        };
        i += 1;
        i;
    }
    rho = aB;
    glp_btran(P, rho);
    len = 0 as i32;
    i = 1 as i32;
    while i <= m {
        if glp_get_row_stat(P, i) != 1 as i32 {
            alfa = -*rho.offset(i as isize);
            if alfa != 0.0f64 {
                len += 1;
                len;
                *ind.offset(len as isize) = i;
                *val.offset(len as isize) = alfa;
            }
        }
        i += 1;
        i;
    }
    iii = glp_alloc(1 as i32 + m, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    vvv = glp_alloc(1 as i32 + m, ::core::mem::size_of::<libc::c_double>() as u64 as i32)
        as *mut libc::c_double;
    j = 1 as i32;
    while j <= n {
        if glp_get_col_stat(P, j) != 1 as i32 {
            alfa = *a.offset(j as isize);
            lll = glp_get_mat_col(P, j, iii, vvv);
            t = 1 as i32;
            while t <= lll {
                alfa
                    += *vvv.offset(t as isize)
                        * *rho.offset(*iii.offset(t as isize) as isize);
                t += 1;
                t;
            }
            if alfa != 0.0f64 {
                len += 1;
                len;
                *ind.offset(len as isize) = m + j;
                *val.offset(len as isize) = alfa;
            }
        }
        j += 1;
        j;
    }
    (len <= n
        || {
            glp_assert_(
                b"len <= n\0" as *const u8 as *const i8,
                b"draft/glpapi12.c\0" as *const u8 as *const i8,
                1110 as i32,
            );
            1 as i32 != 0
        }) as i32;
    glp_free(iii as *mut libc::c_void);
    glp_free(vvv as *mut libc::c_void);
    glp_free(aB as *mut libc::c_void);
    glp_free(a as *mut libc::c_void);
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn glp_transform_col(
    mut P: *mut glp_prob,
    mut len: i32,
    mut ind: *mut i32,
    mut val: *mut libc::c_double,
) -> i32 {
    let mut i: i32 = 0;
    let mut m: i32 = 0;
    let mut t: i32 = 0;
    let mut a: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut alfa: *mut libc::c_double = 0 as *mut libc::c_double;
    if glp_bf_exists(P) == 0 {
        (glp_error_(b"draft/glpapi12.c\0" as *const u8 as *const i8, 1190 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_transform_col: basis factorization does not exist \n\0" as *const u8
                as *const i8,
        );
    }
    m = glp_get_num_rows(P);
    a = glp_alloc(1 as i32 + m, ::core::mem::size_of::<libc::c_double>() as u64 as i32)
        as *mut libc::c_double;
    i = 1 as i32;
    while i <= m {
        *a.offset(i as isize) = 0.0f64;
        i += 1;
        i;
    }
    if !(0 as i32 <= len && len <= m) {
        (glp_error_(b"draft/glpapi12.c\0" as *const u8 as *const i8, 1197 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_transform_col: len = %d; invalid column length\n\0" as *const u8
                as *const i8,
            len,
        );
    }
    t = 1 as i32;
    while t <= len {
        i = *ind.offset(t as isize);
        if !(1 as i32 <= i && i <= m) {
            (glp_error_(b"draft/glpapi12.c\0" as *const u8 as *const i8, 1202 as i32))
                .expect(
                    "non-null function pointer",
                )(
                b"glp_transform_col: ind[%d] = %d; row index out of range\n\0"
                    as *const u8 as *const i8,
                t,
                i,
            );
        }
        if *val.offset(t as isize) == 0.0f64 {
            (glp_error_(b"draft/glpapi12.c\0" as *const u8 as *const i8, 1205 as i32))
                .expect(
                    "non-null function pointer",
                )(
                b"glp_transform_col: val[%d] = 0; zero coefficient not allowed\n\0"
                    as *const u8 as *const i8,
                t,
            );
        }
        if *a.offset(i as isize) != 0.0f64 {
            (glp_error_(b"draft/glpapi12.c\0" as *const u8 as *const i8, 1208 as i32))
                .expect(
                    "non-null function pointer",
                )(
                b"glp_transform_col: ind[%d] = %d; duplicate row indices not allowed\n\0"
                    as *const u8 as *const i8,
                t,
                i,
            );
        }
        *a.offset(i as isize) = *val.offset(t as isize);
        t += 1;
        t;
    }
    alfa = a;
    glp_ftran(P, alfa);
    len = 0 as i32;
    i = 1 as i32;
    while i <= m {
        if *alfa.offset(i as isize) != 0.0f64 {
            len += 1;
            len;
            *ind.offset(len as isize) = glp_get_bhead(P, i);
            *val.offset(len as isize) = *alfa.offset(i as isize);
        }
        i += 1;
        i;
    }
    glp_free(a as *mut libc::c_void);
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn glp_prim_rtest(
    mut P: *mut glp_prob,
    mut len: i32,
    mut ind: *const i32,
    mut val: *const libc::c_double,
    mut dir: i32,
    mut eps: libc::c_double,
) -> i32 {
    let mut k: i32 = 0;
    let mut m: i32 = 0;
    let mut n: i32 = 0;
    let mut piv: i32 = 0;
    let mut t: i32 = 0;
    let mut type_0: i32 = 0;
    let mut stat: i32 = 0;
    let mut alfa: libc::c_double = 0.;
    let mut big: libc::c_double = 0.;
    let mut beta: libc::c_double = 0.;
    let mut lb: libc::c_double = 0.;
    let mut ub: libc::c_double = 0.;
    let mut temp: libc::c_double = 0.;
    let mut teta: libc::c_double = 0.;
    if glp_get_prim_stat(P) != 2 as i32 {
        (glp_error_(b"draft/glpapi12.c\0" as *const u8 as *const i8, 1290 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_prim_rtest: basic solution is not primal feasible \n\0" as *const u8
                as *const i8,
        );
    }
    if !(dir == 1 as i32 || dir == -(1 as i32)) {
        (glp_error_(b"draft/glpapi12.c\0" as *const u8 as *const i8, 1293 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_prim_rtest: dir = %d; invalid parameter\n\0" as *const u8 as *const i8,
            dir,
        );
    }
    if !(0.0f64 < eps && eps < 1.0f64) {
        (glp_error_(b"draft/glpapi12.c\0" as *const u8 as *const i8, 1295 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_prim_rtest: eps = %g; invalid parameter\n\0" as *const u8 as *const i8,
            eps,
        );
    }
    m = glp_get_num_rows(P);
    n = glp_get_num_cols(P);
    piv = 0 as i32;
    teta = 1.7976931348623157e+308f64;
    big = 0.0f64;
    let mut current_block_36: u64;
    t = 1 as i32;
    while t <= len {
        k = *ind.offset(t as isize);
        if !(1 as i32 <= k && k <= m + n) {
            (glp_error_(b"draft/glpapi12.c\0" as *const u8 as *const i8, 1305 as i32))
                .expect(
                    "non-null function pointer",
                )(
                b"glp_prim_rtest: ind[%d] = %d; variable number out of range\n\0"
                    as *const u8 as *const i8,
                t,
                k,
            );
        }
        if k <= m {
            type_0 = glp_get_row_type(P, k);
            lb = glp_get_row_lb(P, k);
            ub = glp_get_row_ub(P, k);
            stat = glp_get_row_stat(P, k);
            beta = glp_get_row_prim(P, k);
        } else {
            type_0 = glp_get_col_type(P, k - m);
            lb = glp_get_col_lb(P, k - m);
            ub = glp_get_col_ub(P, k - m);
            stat = glp_get_col_stat(P, k - m);
            beta = glp_get_col_prim(P, k - m);
        }
        if stat != 1 as i32 {
            (glp_error_(b"draft/glpapi12.c\0" as *const u8 as *const i8, 1324 as i32))
                .expect(
                    "non-null function pointer",
                )(
                b"glp_prim_rtest: ind[%d] = %d; non-basic variable not allowed\n\0"
                    as *const u8 as *const i8,
                t,
                k,
            );
        }
        alfa = if dir > 0 as i32 {
            *val.offset(t as isize)
        } else {
            -*val.offset(t as isize)
        };
        if !(type_0 == 1 as i32) {
            if type_0 == 2 as i32 {
                current_block_36 = 10043043949733653460;
            } else {
                if type_0 == 3 as i32 {
                    current_block_36 = 14763689060501151050;
                } else if type_0 == 4 as i32 {
                    if alfa < 0.0f64 {
                        current_block_36 = 10043043949733653460;
                    } else {
                        current_block_36 = 14763689060501151050;
                    }
                } else if type_0 == 5 as i32 {
                    if -eps < alfa && alfa < eps {
                        current_block_36 = 10886091980245723256;
                    } else {
                        temp = 0.0f64;
                        current_block_36 = 14832935472441733737;
                    }
                } else {
                    (type_0 != type_0
                        || {
                            glp_assert_(
                                b"type != type\0" as *const u8 as *const i8,
                                b"draft/glpapi12.c\0" as *const u8 as *const i8,
                                1356 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                    current_block_36 = 14832935472441733737;
                }
                match current_block_36 {
                    14832935472441733737 => {}
                    10886091980245723256 => {}
                    10043043949733653460 => {}
                    _ => {
                        if alfa < eps {
                            current_block_36 = 10886091980245723256;
                        } else {
                            temp = (ub - beta) / alfa;
                            current_block_36 = 14832935472441733737;
                        }
                    }
                }
            }
            match current_block_36 {
                10886091980245723256 => {}
                _ => {
                    match current_block_36 {
                        10043043949733653460 => {
                            if alfa > -eps {
                                current_block_36 = 10886091980245723256;
                            } else {
                                temp = (lb - beta) / alfa;
                                current_block_36 = 14832935472441733737;
                            }
                        }
                        _ => {}
                    }
                    match current_block_36 {
                        10886091980245723256 => {}
                        _ => {
                            if temp < 0.0f64 {
                                temp = 0.0f64;
                            }
                            if teta > temp || teta == temp && big < fabs(alfa) {
                                piv = t;
                                teta = temp;
                                big = fabs(alfa);
                            }
                        }
                    }
                }
            }
        }
        t += 1;
        t;
    }
    return piv;
}
#[no_mangle]
pub unsafe extern "C" fn glp_dual_rtest(
    mut P: *mut glp_prob,
    mut len: i32,
    mut ind: *const i32,
    mut val: *const libc::c_double,
    mut dir: i32,
    mut eps: libc::c_double,
) -> i32 {
    let mut k: i32 = 0;
    let mut m: i32 = 0;
    let mut n: i32 = 0;
    let mut piv: i32 = 0;
    let mut t: i32 = 0;
    let mut stat: i32 = 0;
    let mut alfa: libc::c_double = 0.;
    let mut big: libc::c_double = 0.;
    let mut cost: libc::c_double = 0.;
    let mut obj: libc::c_double = 0.;
    let mut temp: libc::c_double = 0.;
    let mut teta: libc::c_double = 0.;
    if glp_get_dual_stat(P) != 2 as i32 {
        (glp_error_(b"draft/glpapi12.c\0" as *const u8 as *const i8, 1432 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_dual_rtest: basic solution is not dual feasible\n\0" as *const u8
                as *const i8,
        );
    }
    if !(dir == 1 as i32 || dir == -(1 as i32)) {
        (glp_error_(b"draft/glpapi12.c\0" as *const u8 as *const i8, 1435 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_dual_rtest: dir = %d; invalid parameter\n\0" as *const u8 as *const i8,
            dir,
        );
    }
    if !(0.0f64 < eps && eps < 1.0f64) {
        (glp_error_(b"draft/glpapi12.c\0" as *const u8 as *const i8, 1437 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_dual_rtest: eps = %g; invalid parameter\n\0" as *const u8 as *const i8,
            eps,
        );
    }
    m = glp_get_num_rows(P);
    n = glp_get_num_cols(P);
    obj = if glp_get_obj_dir(P) == 1 as i32 { 1.0f64 } else { -1.0f64 };
    piv = 0 as i32;
    teta = 1.7976931348623157e+308f64;
    big = 0.0f64;
    let mut current_block_31: u64;
    t = 1 as i32;
    while t <= len {
        k = *ind.offset(t as isize);
        if !(1 as i32 <= k && k <= m + n) {
            (glp_error_(b"draft/glpapi12.c\0" as *const u8 as *const i8, 1449 as i32))
                .expect(
                    "non-null function pointer",
                )(
                b"glp_dual_rtest: ind[%d] = %d; variable number out of range\n\0"
                    as *const u8 as *const i8,
                t,
                k,
            );
        }
        if k <= m {
            stat = glp_get_row_stat(P, k);
            cost = glp_get_row_dual(P, k);
        } else {
            stat = glp_get_col_stat(P, k - m);
            cost = glp_get_col_dual(P, k - m);
        }
        if stat == 1 as i32 {
            (glp_error_(b"draft/glpapi12.c\0" as *const u8 as *const i8, 1462 as i32))
                .expect(
                    "non-null function pointer",
                )(
                b"glp_dual_rtest: ind[%d] = %d; basic variable not allowed\n\0"
                    as *const u8 as *const i8,
                t,
                k,
            );
        }
        alfa = if dir > 0 as i32 {
            *val.offset(t as isize)
        } else {
            -*val.offset(t as isize)
        };
        if stat == 2 as i32 {
            if alfa < eps {
                current_block_31 = 8515828400728868193;
            } else {
                temp = obj * cost / alfa;
                current_block_31 = 11636175345244025579;
            }
        } else if stat == 3 as i32 {
            if alfa > -eps {
                current_block_31 = 8515828400728868193;
            } else {
                temp = obj * cost / alfa;
                current_block_31 = 11636175345244025579;
            }
        } else if stat == 4 as i32 {
            if -eps < alfa && alfa < eps {
                current_block_31 = 8515828400728868193;
            } else {
                temp = 0.0f64;
                current_block_31 = 11636175345244025579;
            }
        } else if stat == 5 as i32 {
            current_block_31 = 8515828400728868193;
        } else {
            (stat != stat
                || {
                    glp_assert_(
                        b"stat != stat\0" as *const u8 as *const i8,
                        b"draft/glpapi12.c\0" as *const u8 as *const i8,
                        1490 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            current_block_31 = 11636175345244025579;
        }
        match current_block_31 {
            11636175345244025579 => {
                if temp < 0.0f64 {
                    temp = 0.0f64;
                }
                if teta > temp || teta == temp && big < fabs(alfa) {
                    piv = t;
                    teta = temp;
                    big = fabs(alfa);
                }
            }
            _ => {}
        }
        t += 1;
        t;
    }
    return piv;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_analyze_row(
    mut P: *mut glp_prob,
    mut len: i32,
    mut ind: *const i32,
    mut val: *const libc::c_double,
    mut type_0: i32,
    mut rhs: libc::c_double,
    mut eps: libc::c_double,
    mut _piv: *mut i32,
    mut _x: *mut libc::c_double,
    mut _dx: *mut libc::c_double,
    mut _y: *mut libc::c_double,
    mut _dy: *mut libc::c_double,
    mut _dz: *mut libc::c_double,
) -> i32 {
    let mut current_block: u64;
    let mut t: i32 = 0;
    let mut k: i32 = 0;
    let mut dir: i32 = 0;
    let mut piv: i32 = 0;
    let mut ret: i32 = 0 as i32;
    let mut x: libc::c_double = 0.;
    let mut dx: libc::c_double = 0.;
    let mut y: libc::c_double = 0.;
    let mut dy: libc::c_double = 0.;
    let mut dz: libc::c_double = 0.;
    if (*P).pbs_stat == 1 as i32 {
        (glp_error_(b"draft/glpapi12.c\0" as *const u8 as *const i8, 1589 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_analyze_row: primal basic solution components are undefined\n\0"
                as *const u8 as *const i8,
        );
    }
    if (*P).dbs_stat != 2 as i32 {
        (glp_error_(b"draft/glpapi12.c\0" as *const u8 as *const i8, 1592 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_analyze_row: basic solution is not dual feasible\n\0" as *const u8
                as *const i8,
        );
    }
    if !(0 as i32 <= len && len <= (*P).n) {
        (glp_error_(b"draft/glpapi12.c\0" as *const u8 as *const i8, 1597 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_analyze_row: len = %d; invalid row length\n\0" as *const u8
                as *const i8,
            len,
        );
    }
    y = 0.0f64;
    t = 1 as i32;
    while t <= len {
        k = *ind.offset(t as isize);
        if !(1 as i32 <= k && k <= (*P).m + (*P).n) {
            (glp_error_(b"draft/glpapi12.c\0" as *const u8 as *const i8, 1603 as i32))
                .expect(
                    "non-null function pointer",
                )(
                b"glp_analyze_row: ind[%d] = %d; row/column index out of range\n\0"
                    as *const u8 as *const i8,
                t,
                k,
            );
        }
        if k <= (*P).m {
            if (**((*P).row).offset(k as isize)).stat == 1 as i32 {
                (glp_error_(
                    b"draft/glpapi12.c\0" as *const u8 as *const i8,
                    1608 as i32,
                ))
                    .expect(
                        "non-null function pointer",
                    )(
                    b"glp_analyze_row: ind[%d] = %d; basic auxiliary variable is not allowed\n\0"
                        as *const u8 as *const i8,
                    t,
                    k,
                );
            }
            x = (**((*P).row).offset(k as isize)).prim;
        } else {
            if (**((*P).col).offset((k - (*P).m) as isize)).stat == 1 as i32 {
                (glp_error_(
                    b"draft/glpapi12.c\0" as *const u8 as *const i8,
                    1615 as i32,
                ))
                    .expect(
                        "non-null function pointer",
                    )(
                    b"glp_analyze_row: ind[%d] = %d; basic structural variable is not allowed\n\0"
                        as *const u8 as *const i8,
                    t,
                    k,
                );
            }
            x = (**((*P).col).offset((k - (*P).m) as isize)).prim;
        }
        y += *val.offset(t as isize) * x;
        t += 1;
        t;
    }
    if type_0 == 2 as i32 {
        if y >= rhs {
            ret = 1 as i32;
            current_block = 15329523206286409568;
        } else {
            dir = 1 as i32;
            current_block = 15345278821338558188;
        }
    } else if type_0 == 3 as i32 {
        if y <= rhs {
            ret = 1 as i32;
            current_block = 15329523206286409568;
        } else {
            dir = -(1 as i32);
            current_block = 15345278821338558188;
        }
    } else {
        (glp_error_(b"draft/glpapi12.c\0" as *const u8 as *const i8, 1642 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_analyze_row: type = %d; invalid parameter\n\0" as *const u8
                as *const i8,
            type_0,
        );
        current_block = 15345278821338558188;
    }
    match current_block {
        15345278821338558188 => {
            dy = rhs - y;
            piv = glp_dual_rtest(P, len, ind, val, dir, eps);
            if piv == 0 as i32 {
                ret = 2 as i32;
            } else {
                k = *ind.offset(piv as isize);
                (1 as i32 <= k && k <= (*P).m + (*P).n
                    || {
                        glp_assert_(
                            b"1 <= k && k <= P->m+P->n\0" as *const u8 as *const i8,
                            b"draft/glpapi12.c\0" as *const u8 as *const i8,
                            1656 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                if k <= (*P).m {
                    x = (**((*P).row).offset(k as isize)).prim;
                } else {
                    x = (**((*P).col).offset((k - (*P).m) as isize)).prim;
                }
                (*val.offset(piv as isize) != 0.0f64
                    || {
                        glp_assert_(
                            b"val[piv] != 0.0\0" as *const u8 as *const i8,
                            b"draft/glpapi12.c\0" as *const u8 as *const i8,
                            1663 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                dx = dy / *val.offset(piv as isize);
                if k <= (*P).m {
                    dz = (**((*P).row).offset(k as isize)).dual * dx;
                } else {
                    dz = (**((*P).col).offset((k - (*P).m) as isize)).dual * dx;
                }
                if !_piv.is_null() {
                    *_piv = piv;
                }
                if !_x.is_null() {
                    *_x = x;
                }
                if !_dx.is_null() {
                    *_dx = dx;
                }
                if !_y.is_null() {
                    *_y = y;
                }
                if !_dy.is_null() {
                    *_dy = dy;
                }
                if !_dz.is_null() {
                    *_dz = dz;
                }
            }
        }
        _ => {}
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn glp_analyze_bound(
    mut P: *mut glp_prob,
    mut k: i32,
    mut value1: *mut libc::c_double,
    mut var1: *mut i32,
    mut value2: *mut libc::c_double,
    mut var2: *mut i32,
) {
    let mut row: *mut GLPROW = 0 as *mut GLPROW;
    let mut col: *mut GLPCOL = 0 as *mut GLPCOL;
    let mut m: i32 = 0;
    let mut n: i32 = 0;
    let mut stat: i32 = 0;
    let mut kase: i32 = 0;
    let mut p: i32 = 0;
    let mut len: i32 = 0;
    let mut piv: i32 = 0;
    let mut ind: *mut i32 = 0 as *mut i32;
    let mut x: libc::c_double = 0.;
    let mut new_x: libc::c_double = 0.;
    let mut ll: libc::c_double = 0.;
    let mut uu: libc::c_double = 0.;
    let mut xx: libc::c_double = 0.;
    let mut delta: libc::c_double = 0.;
    let mut val: *mut libc::c_double = 0 as *mut libc::c_double;
    m = (*P).m;
    n = (*P).n;
    if !((*P).pbs_stat == 2 as i32 && (*P).dbs_stat == 2 as i32) {
        (glp_error_(b"draft/glpapi12.c\0" as *const u8 as *const i8, 1807 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_analyze_bound: optimal basic solution required\n\0" as *const u8
                as *const i8,
        );
    }
    if !(m == 0 as i32 || (*P).valid != 0) {
        (glp_error_(b"draft/glpapi12.c\0" as *const u8 as *const i8, 1809 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_analyze_bound: basis factorization required\n\0" as *const u8
                as *const i8,
        );
    }
    if !(1 as i32 <= k && k <= m + n) {
        (glp_error_(b"draft/glpapi12.c\0" as *const u8 as *const i8, 1811 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_analyze_bound: k = %d; variable number out of range\n\0" as *const u8
                as *const i8,
            k,
        );
    }
    if k <= m {
        row = *((*P).row).offset(k as isize);
        stat = (*row).stat;
        x = (*row).prim;
    } else {
        col = *((*P).col).offset((k - m) as isize);
        stat = (*col).stat;
        x = (*col).prim;
    }
    if stat == 1 as i32 {
        (glp_error_(b"draft/glpapi12.c\0" as *const u8 as *const i8, 1826 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_analyze_bound: k = %d; basic variable not allowed \n\0" as *const u8
                as *const i8,
            k,
        );
    }
    ind = glp_alloc(1 as i32 + m, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    val = glp_alloc(1 as i32 + m, ::core::mem::size_of::<libc::c_double>() as u64 as i32)
        as *mut libc::c_double;
    len = glp_eval_tab_col(P, k, ind, val);
    (0 as i32 <= len && len <= m
        || {
            glp_assert_(
                b"0 <= len && len <= m\0" as *const u8 as *const i8,
                b"draft/glpapi12.c\0" as *const u8 as *const i8,
                1834 as i32,
            );
            1 as i32 != 0
        }) as i32;
    kase = -(1 as i32);
    while kase <= 1 as i32 {
        piv = glp_prim_rtest(
            P,
            len,
            ind as *const i32,
            val as *const libc::c_double,
            kase,
            1e-9f64,
        );
        if piv == 0 as i32 {
            p = 0 as i32;
            new_x = if kase < 0 as i32 {
                -1.7976931348623157e+308f64
            } else {
                1.7976931348623157e+308f64
            };
        } else {
            (1 as i32 <= piv && piv <= len
                || {
                    glp_assert_(
                        b"1 <= piv && piv <= len\0" as *const u8 as *const i8,
                        b"draft/glpapi12.c\0" as *const u8 as *const i8,
                        1850 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            p = *ind.offset(piv as isize);
            if p <= m {
                row = *((*P).row).offset(p as isize);
                ll = glp_get_row_lb(P, (*row).i);
                uu = glp_get_row_ub(P, (*row).i);
                stat = (*row).stat;
                xx = (*row).prim;
            } else {
                col = *((*P).col).offset((p - m) as isize);
                ll = glp_get_col_lb(P, (*col).j);
                uu = glp_get_col_ub(P, (*col).j);
                stat = (*col).stat;
                xx = (*col).prim;
            }
            (stat == 1 as i32
                || {
                    glp_assert_(
                        b"stat == GLP_BS\0" as *const u8 as *const i8,
                        b"draft/glpapi12.c\0" as *const u8 as *const i8,
                        1866 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            if kase < 0 as i32 && *val.offset(piv as isize) > 0.0f64
                || kase > 0 as i32 && *val.offset(piv as isize) < 0.0f64
            {
                (ll != -1.7976931348623157e+308f64
                    || {
                        glp_assert_(
                            b"ll != -DBL_MAX\0" as *const u8 as *const i8,
                            b"draft/glpapi12.c\0" as *const u8 as *const i8,
                            1871 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                delta = ll - xx;
            } else {
                (uu != 1.7976931348623157e+308f64
                    || {
                        glp_assert_(
                            b"uu != +DBL_MAX\0" as *const u8 as *const i8,
                            b"draft/glpapi12.c\0" as *const u8 as *const i8,
                            1876 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                delta = uu - xx;
            }
            (*val.offset(piv as isize) != 0.0f64
                || {
                    glp_assert_(
                        b"val[piv] != 0.0\0" as *const u8 as *const i8,
                        b"draft/glpapi12.c\0" as *const u8 as *const i8,
                        1882 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            new_x = x + delta / *val.offset(piv as isize);
        }
        if kase < 0 as i32 {
            if !value1.is_null() {
                *value1 = new_x;
            }
            if !var1.is_null() {
                *var1 = p;
            }
        } else {
            if !value2.is_null() {
                *value2 = new_x;
            }
            if !var2.is_null() {
                *var2 = p;
            }
        }
        kase += 2 as i32;
    }
    glp_free(ind as *mut libc::c_void);
    glp_free(val as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn glp_analyze_coef(
    mut P: *mut glp_prob,
    mut k: i32,
    mut coef1: *mut libc::c_double,
    mut var1: *mut i32,
    mut value1: *mut libc::c_double,
    mut coef2: *mut libc::c_double,
    mut var2: *mut i32,
    mut value2: *mut libc::c_double,
) {
    let mut row: *mut GLPROW = 0 as *mut GLPROW;
    let mut col: *mut GLPCOL = 0 as *mut GLPCOL;
    let mut m: i32 = 0;
    let mut n: i32 = 0;
    let mut type_0: i32 = 0;
    let mut stat: i32 = 0;
    let mut kase: i32 = 0;
    let mut p: i32 = 0;
    let mut q: i32 = 0;
    let mut dir: i32 = 0;
    let mut clen: i32 = 0;
    let mut cpiv: i32 = 0;
    let mut rlen: i32 = 0;
    let mut rpiv: i32 = 0;
    let mut cind: *mut i32 = 0 as *mut i32;
    let mut rind: *mut i32 = 0 as *mut i32;
    let mut lb: libc::c_double = 0.;
    let mut ub: libc::c_double = 0.;
    let mut coef: libc::c_double = 0.;
    let mut x: libc::c_double = 0.;
    let mut lim_coef: libc::c_double = 0.;
    let mut new_x: libc::c_double = 0.;
    let mut d: libc::c_double = 0.;
    let mut delta: libc::c_double = 0.;
    let mut ll: libc::c_double = 0.;
    let mut uu: libc::c_double = 0.;
    let mut xx: libc::c_double = 0.;
    let mut rval: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut cval: *mut libc::c_double = 0 as *mut libc::c_double;
    m = (*P).m;
    n = (*P).n;
    if !((*P).pbs_stat == 2 as i32 && (*P).dbs_stat == 2 as i32) {
        (glp_error_(b"draft/glpapi12.c\0" as *const u8 as *const i8, 1974 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_analyze_coef: optimal basic solution required\n\0" as *const u8
                as *const i8,
        );
    }
    if !(m == 0 as i32 || (*P).valid != 0) {
        (glp_error_(b"draft/glpapi12.c\0" as *const u8 as *const i8, 1976 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_analyze_coef: basis factorization required\n\0" as *const u8
                as *const i8,
        );
    }
    if !(1 as i32 <= k && k <= m + n) {
        (glp_error_(b"draft/glpapi12.c\0" as *const u8 as *const i8, 1978 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_analyze_coef: k = %d; variable number out of range\n\0" as *const u8
                as *const i8,
            k,
        );
    }
    if k <= m {
        row = *((*P).row).offset(k as isize);
        type_0 = (*row).type_0;
        lb = (*row).lb;
        ub = (*row).ub;
        coef = 0.0f64;
        stat = (*row).stat;
        x = (*row).prim;
    } else {
        col = *((*P).col).offset((k - m) as isize);
        type_0 = (*col).type_0;
        lb = (*col).lb;
        ub = (*col).ub;
        coef = (*col).coef;
        stat = (*col).stat;
        x = (*col).prim;
    }
    if stat != 1 as i32 {
        (glp_error_(b"draft/glpapi12.c\0" as *const u8 as *const i8, 2001 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_analyze_coef: k = %d; non-basic variable not allowed\n\0" as *const u8
                as *const i8,
            k,
        );
    }
    cind = glp_alloc(1 as i32 + m, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    cval = glp_alloc(
        1 as i32 + m,
        ::core::mem::size_of::<libc::c_double>() as u64 as i32,
    ) as *mut libc::c_double;
    rind = glp_alloc(1 as i32 + n, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    rval = glp_alloc(
        1 as i32 + n,
        ::core::mem::size_of::<libc::c_double>() as u64 as i32,
    ) as *mut libc::c_double;
    rlen = glp_eval_tab_row(P, k, rind, rval);
    (0 as i32 <= rlen && rlen <= n
        || {
            glp_assert_(
                b"0 <= rlen && rlen <= n\0" as *const u8 as *const i8,
                b"draft/glpapi12.c\0" as *const u8 as *const i8,
                2011 as i32,
            );
            1 as i32 != 0
        }) as i32;
    kase = -(1 as i32);
    while kase <= 1 as i32 {
        if (*P).dir == 1 as i32 {
            dir = -kase;
        } else if (*P).dir == 2 as i32 {
            dir = kase;
        } else {
            (P != P
                || {
                    glp_assert_(
                        b"P != P\0" as *const u8 as *const i8,
                        b"draft/glpapi12.c\0" as *const u8 as *const i8,
                        2024 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        }
        rpiv = glp_dual_rtest(
            P,
            rlen,
            rind as *const i32,
            rval as *const libc::c_double,
            dir,
            1e-9f64,
        );
        if rpiv == 0 as i32 {
            lim_coef = if kase < 0 as i32 {
                -1.7976931348623157e+308f64
            } else {
                1.7976931348623157e+308f64
            };
            q = 0 as i32;
            new_x = x;
        } else {
            (1 as i32 <= rpiv && rpiv <= rlen
                || {
                    glp_assert_(
                        b"1 <= rpiv && rpiv <= rlen\0" as *const u8 as *const i8,
                        b"draft/glpapi12.c\0" as *const u8 as *const i8,
                        2039 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            q = *rind.offset(rpiv as isize);
            (1 as i32 <= q && q <= m + n
                || {
                    glp_assert_(
                        b"1 <= q && q <= m+n\0" as *const u8 as *const i8,
                        b"draft/glpapi12.c\0" as *const u8 as *const i8,
                        2041 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            if q <= m {
                row = *((*P).row).offset(q as isize);
                stat = (*row).stat;
                d = (*row).dual;
            } else {
                col = *((*P).col).offset((q - m) as isize);
                stat = (*col).stat;
                d = (*col).dual;
            }
            (*rval.offset(rpiv as isize) != 0.0f64
                || {
                    glp_assert_(
                        b"rval[rpiv] != 0.0\0" as *const u8 as *const i8,
                        b"draft/glpapi12.c\0" as *const u8 as *const i8,
                        2055 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            delta = -d / *rval.offset(rpiv as isize);
            lim_coef = coef + delta;
            if kase < 0 as i32 && *rval.offset(rpiv as isize) > 0.0f64
                || kase > 0 as i32 && *rval.offset(rpiv as isize) < 0.0f64
            {
                dir = 1 as i32;
            } else {
                dir = -(1 as i32);
            }
            if (*P).dir == 2 as i32 {
                dir = -dir;
            }
            if dir > 0 as i32 {
                (stat == 2 as i32 || stat == 4 as i32
                    || {
                        glp_assert_(
                            b"stat == GLP_NL || stat == GLP_NF\0" as *const u8
                                as *const i8,
                            b"draft/glpapi12.c\0" as *const u8 as *const i8,
                            2082 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
            } else {
                (stat == 3 as i32 || stat == 4 as i32
                    || {
                        glp_assert_(
                            b"stat == GLP_NU || stat == GLP_NF\0" as *const u8
                                as *const i8,
                            b"draft/glpapi12.c\0" as *const u8 as *const i8,
                            2084 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
            }
            clen = glp_eval_tab_col(P, q, cind, cval);
            if k <= m {
                row = *((*P).row).offset(k as isize);
                (*row).type_0 = 1 as i32;
                (*row).ub = 0.0f64;
                (*row).lb = (*row).ub;
            } else {
                col = *((*P).col).offset((k - m) as isize);
                (*col).type_0 = 1 as i32;
                (*col).ub = 0.0f64;
                (*col).lb = (*col).ub;
            }
            cpiv = glp_prim_rtest(
                P,
                clen,
                cind as *const i32,
                cval as *const libc::c_double,
                dir,
                1e-9f64,
            );
            if k <= m {
                row = *((*P).row).offset(k as isize);
                (*row).type_0 = type_0;
                (*row).lb = lb;
                (*row).ub = ub;
            } else {
                col = *((*P).col).offset((k - m) as isize);
                (*col).type_0 = type_0;
                (*col).lb = lb;
                (*col).ub = ub;
            }
            if cpiv == 0 as i32 {
                if dir < 0 as i32 && *rval.offset(rpiv as isize) > 0.0f64
                    || dir > 0 as i32 && *rval.offset(rpiv as isize) < 0.0f64
                {
                    new_x = -1.7976931348623157e+308f64;
                } else {
                    new_x = 1.7976931348623157e+308f64;
                }
            } else {
                (1 as i32 <= cpiv && cpiv <= clen
                    || {
                        glp_assert_(
                            b"1 <= cpiv && cpiv <= clen\0" as *const u8 as *const i8,
                            b"draft/glpapi12.c\0" as *const u8 as *const i8,
                            2128 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                p = *cind.offset(cpiv as isize);
                (1 as i32 <= p && p <= m + n
                    || {
                        glp_assert_(
                            b"1 <= p && p <= m+n\0" as *const u8 as *const i8,
                            b"draft/glpapi12.c\0" as *const u8 as *const i8,
                            2130 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                (p != k
                    || {
                        glp_assert_(
                            b"p != k\0" as *const u8 as *const i8,
                            b"draft/glpapi12.c\0" as *const u8 as *const i8,
                            2131 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                if p <= m {
                    row = *((*P).row).offset(p as isize);
                    ((*row).stat == 1 as i32
                        || {
                            glp_assert_(
                                b"row->stat == GLP_BS\0" as *const u8 as *const i8,
                                b"draft/glpapi12.c\0" as *const u8 as *const i8,
                                2134 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                    ll = glp_get_row_lb(P, (*row).i);
                    uu = glp_get_row_ub(P, (*row).i);
                    xx = (*row).prim;
                } else {
                    col = *((*P).col).offset((p - m) as isize);
                    ((*col).stat == 1 as i32
                        || {
                            glp_assert_(
                                b"col->stat == GLP_BS\0" as *const u8 as *const i8,
                                b"draft/glpapi12.c\0" as *const u8 as *const i8,
                                2141 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                    ll = glp_get_col_lb(P, (*col).j);
                    uu = glp_get_col_ub(P, (*col).j);
                    xx = (*col).prim;
                }
                if dir < 0 as i32 && *cval.offset(cpiv as isize) > 0.0f64
                    || dir > 0 as i32 && *cval.offset(cpiv as isize) < 0.0f64
                {
                    (ll != -1.7976931348623157e+308f64
                        || {
                            glp_assert_(
                                b"ll != -DBL_MAX\0" as *const u8 as *const i8,
                                b"draft/glpapi12.c\0" as *const u8 as *const i8,
                                2150 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                    delta = ll - xx;
                } else {
                    (uu != 1.7976931348623157e+308f64
                        || {
                            glp_assert_(
                                b"uu != +DBL_MAX\0" as *const u8 as *const i8,
                                b"draft/glpapi12.c\0" as *const u8 as *const i8,
                                2155 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                    delta = uu - xx;
                }
                (*cval.offset(cpiv as isize) != 0.0f64
                    || {
                        glp_assert_(
                            b"cval[cpiv] != 0.0\0" as *const u8 as *const i8,
                            b"draft/glpapi12.c\0" as *const u8 as *const i8,
                            2160 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                new_x = x
                    + *rval.offset(rpiv as isize) / *cval.offset(cpiv as isize) * delta;
            }
        }
        if kase < 0 as i32 {
            if !coef1.is_null() {
                *coef1 = lim_coef;
            }
            if !var1.is_null() {
                *var1 = q;
            }
            if !value1.is_null() {
                *value1 = new_x;
            }
        } else {
            if !coef2.is_null() {
                *coef2 = lim_coef;
            }
            if !var2.is_null() {
                *var2 = q;
            }
            if !value2.is_null() {
                *value2 = new_x;
            }
        }
        kase += 2 as i32;
    }
    glp_free(cind as *mut libc::c_void);
    glp_free(cval as *mut libc::c_void);
    glp_free(rind as *mut libc::c_void);
    glp_free(rval as *mut libc::c_void);
}