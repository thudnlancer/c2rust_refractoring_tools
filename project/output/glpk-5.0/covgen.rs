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
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn glp_free(ptr: *mut libc::c_void);
    fn glp_alloc(n: i32, size: i32) -> *mut libc::c_void;
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
    fn glp_printf(fmt: *const i8, _: ...);
    fn _glp_fvs_alloc_vec(x: *mut FVS, n: i32);
    fn _glp_fvs_check_vec(x: *const FVS);
    fn _glp_fvs_clear_vec(x: *mut FVS);
    fn _glp_fvs_adjust_vec(x: *mut FVS, eps: libc::c_double);
    fn _glp_fvs_free_vec(x: *mut FVS);
    fn _glp_ks_mt1(n: i32, a: *const i32, b: i32, c: *const i32, x: *mut i8) -> i32;
    fn _glp_ks_greedy(n: i32, a: *const i32, b: i32, c: *const i32, x: *mut i8) -> i32;
    fn glp_create_prob() -> *mut glp_prob;
    fn glp_add_rows(P: *mut glp_prob, nrs: i32) -> i32;
    fn glp_add_cols(P: *mut glp_prob, ncs: i32) -> i32;
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
    fn glp_delete_prob(P: *mut glp_prob);
    fn glp_get_row_ub(P: *mut glp_prob, i: i32) -> libc::c_double;
    fn glp_get_col_type(P: *mut glp_prob, j: i32) -> i32;
    fn glp_get_col_lb(P: *mut glp_prob, j: i32) -> libc::c_double;
    fn glp_get_col_ub(P: *mut glp_prob, j: i32) -> libc::c_double;
    fn glp_get_mat_row(
        P: *mut glp_prob,
        i: i32,
        ind: *mut i32,
        val: *mut libc::c_double,
    ) -> i32;
    fn glp_get_status(P: *mut glp_prob) -> i32;
    fn glp_get_col_prim(P: *mut glp_prob, j: i32) -> libc::c_double;
    fn glp_get_col_kind(P: *mut glp_prob, j: i32) -> i32;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FVS {
    pub n: i32,
    pub nnz: i32,
    pub ind: *mut i32,
    pub vec: *mut libc::c_double,
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
pub struct glp_cov {
    pub n: i32,
    pub set: *mut glp_prob,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bnd {
    pub z: i32,
    pub a: libc::c_double,
    pub b: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct csa {
    pub P: *mut glp_prob,
    pub l: *mut bnd,
    pub u: *mut bnd,
    pub set: *mut glp_prob,
}
unsafe extern "C" fn init_bounds(mut csa: *mut csa) {
    let mut P: *mut glp_prob = (*csa).P;
    let mut l: *mut bnd = (*csa).l;
    let mut u: *mut bnd = (*csa).u;
    let mut j: i32 = 0;
    j = 1 as i32;
    while j <= (*P).n {
        let ref mut fresh0 = (*u.offset(j as isize)).z;
        *fresh0 = 0 as i32;
        (*l.offset(j as isize)).z = *fresh0;
        let ref mut fresh1 = (*u.offset(j as isize)).a;
        *fresh1 = 0 as i32 as libc::c_double;
        (*l.offset(j as isize)).a = *fresh1;
        (*l.offset(j as isize)).b = glp_get_col_lb(P, j);
        (*u.offset(j as isize)).b = glp_get_col_ub(P, j);
        j += 1;
        j;
    }
}
unsafe extern "C" fn check_vb(
    mut csa: *mut csa,
    mut i: i32,
    mut x: *mut i32,
    mut z: *mut i32,
    mut a: *mut libc::c_double,
    mut b: *mut libc::c_double,
) -> i32 {
    let mut P: *mut glp_prob = (*csa).P;
    let mut row: *mut GLPROW = 0 as *mut GLPROW;
    let mut a1: *mut GLPAIJ = 0 as *mut GLPAIJ;
    let mut a2: *mut GLPAIJ = 0 as *mut GLPAIJ;
    let mut type_0: i32 = 0;
    let mut rhs: libc::c_double = 0.;
    (1 as i32 <= i && i <= (*P).m
        || {
            glp_assert_(
                b"1 <= i && i <= P->m\0" as *const u8 as *const i8,
                b"intopt/covgen.c\0" as *const u8 as *const i8,
                109 as i32,
            );
            1 as i32 != 0
        }) as i32;
    row = *((*P).row).offset(i as isize);
    match (*row).type_0 {
        2 | 3 => {}
        _ => return 0 as i32,
    }
    a1 = (*row).ptr;
    if a1.is_null() {
        return 0 as i32;
    }
    a2 = (*a1).r_next;
    if a2.is_null() {
        return 0 as i32;
    }
    if !((*a2).r_next).is_null() {
        return 0 as i32;
    }
    if glp_get_col_kind(P, (*(*a1).col).j) == 3 as i32 {
        let mut a_0: *mut GLPAIJ = 0 as *mut GLPAIJ;
        a_0 = a1;
        a1 = a2;
        a2 = a_0;
    }
    if (*(*a1).col).type_0 == 5 as i32 {
        return 0 as i32;
    }
    if glp_get_col_kind(P, (*(*a1).col).j) == 3 as i32 {
        return 0 as i32;
    }
    if glp_get_col_kind(P, (*(*a2).col).j) != 3 as i32 {
        return 0 as i32;
    }
    match (*row).type_0 {
        2 => {
            type_0 = if (*a1).val > 0 as i32 as libc::c_double {
                2 as i32
            } else {
                3 as i32
            };
            rhs = (*row).lb;
        }
        3 => {
            type_0 = if (*a1).val > 0 as i32 as libc::c_double {
                3 as i32
            } else {
                2 as i32
            };
            rhs = (*row).ub;
        }
        _ => {
            (type_0 != type_0
                || {
                    glp_assert_(
                        b"type != type\0" as *const u8 as *const i8,
                        b"intopt/covgen.c\0" as *const u8 as *const i8,
                        155 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        }
    }
    *x = (*(*a1).col).j;
    *z = (*(*a2).col).j;
    *a = -(*a2).val / (*a1).val;
    *b = rhs / (*a1).val;
    return type_0;
}
unsafe extern "C" fn set_vb(
    mut csa: *mut csa,
    mut type_0: i32,
    mut x: i32,
    mut z: i32,
    mut a: libc::c_double,
    mut b: libc::c_double,
) {
    let mut P: *mut glp_prob = (*csa).P;
    let mut l: *mut bnd = (*csa).l;
    let mut u: *mut bnd = (*csa).u;
    (glp_get_col_type(P, x) != 5 as i32
        || {
            glp_assert_(
                b"glp_get_col_type(P, x) != GLP_FX\0" as *const u8 as *const i8,
                b"intopt/covgen.c\0" as *const u8 as *const i8,
                177 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (glp_get_col_kind(P, x) != 3 as i32
        || {
            glp_assert_(
                b"glp_get_col_kind(P, x) != GLP_BV\0" as *const u8 as *const i8,
                b"intopt/covgen.c\0" as *const u8 as *const i8,
                178 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (glp_get_col_kind(P, z) == 3 as i32
        || {
            glp_assert_(
                b"glp_get_col_kind(P, z) == GLP_BV\0" as *const u8 as *const i8,
                b"intopt/covgen.c\0" as *const u8 as *const i8,
                179 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (a != 0 as i32 as libc::c_double
        || {
            glp_assert_(
                b"a != 0\0" as *const u8 as *const i8,
                b"intopt/covgen.c\0" as *const u8 as *const i8,
                180 as i32,
            );
            1 as i32 != 0
        }) as i32;
    match type_0 {
        2 => {
            (*l.offset(x as isize)).z = z;
            (*l.offset(x as isize)).a = a;
            (*l.offset(x as isize)).b = b;
        }
        3 => {
            (*u.offset(x as isize)).z = z;
            (*u.offset(x as isize)).a = a;
            (*u.offset(x as isize)).b = b;
        }
        _ => {
            (type_0 != type_0
                || {
                    glp_assert_(
                        b"type != type\0" as *const u8 as *const i8,
                        b"intopt/covgen.c\0" as *const u8 as *const i8,
                        191 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        }
    };
}
unsafe extern "C" fn obtain_vbs(mut csa: *mut csa) {
    let mut P: *mut glp_prob = (*csa).P;
    let mut i: i32 = 0;
    let mut x: i32 = 0;
    let mut z: i32 = 0;
    let mut type_0: i32 = 0;
    let mut save: i32 = 0;
    let mut a: libc::c_double = 0.;
    let mut b: libc::c_double = 0.;
    i = 1 as i32;
    while i <= (*P).m {
        match (**((*P).row).offset(i as isize)).type_0 {
            1 => {}
            2 | 3 => {
                type_0 = check_vb(csa, i, &mut x, &mut z, &mut a, &mut b);
                if type_0 != 0 {
                    set_vb(csa, type_0, x, z, a, b);
                }
            }
            4 | 5 => {
                save = (**((*P).row).offset(i as isize)).type_0;
                (**((*P).row).offset(i as isize)).type_0 = 2 as i32;
                type_0 = check_vb(csa, i, &mut x, &mut z, &mut a, &mut b);
                if type_0 != 0 {
                    set_vb(csa, type_0, x, z, a, b);
                }
                (**((*P).row).offset(i as isize)).type_0 = 3 as i32;
                type_0 = check_vb(csa, i, &mut x, &mut z, &mut a, &mut b);
                if type_0 != 0 {
                    set_vb(csa, type_0, x, z, a, b);
                }
                (**((*P).row).offset(i as isize)).type_0 = save;
            }
            _ => {
                (P != P
                    || {
                        glp_assert_(
                            b"P != P\0" as *const u8 as *const i8,
                            b"intopt/covgen.c\0" as *const u8 as *const i8,
                            234 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
            }
        }
        i += 1;
        i;
    }
}
unsafe extern "C" fn add_term(mut v: *mut FVS, mut j: i32, mut a: libc::c_double) {
    (1 as i32 <= j && j <= (*v).n
        || {
            glp_assert_(
                b"1 <= j && j <= v->n\0" as *const u8 as *const i8,
                b"intopt/covgen.c\0" as *const u8 as *const i8,
                251 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (a != 0 as i32 as libc::c_double
        || {
            glp_assert_(
                b"a != 0\0" as *const u8 as *const i8,
                b"intopt/covgen.c\0" as *const u8 as *const i8,
                252 as i32,
            );
            1 as i32 != 0
        }) as i32;
    if *((*v).vec).offset(j as isize) == 0 as i32 as libc::c_double {
        (*v).nnz += 1;
        (*v).nnz;
        ((*v).nnz <= (*v).n
            || {
                glp_assert_(
                    b"v->nnz <= v->n\0" as *const u8 as *const i8,
                    b"intopt/covgen.c\0" as *const u8 as *const i8,
                    256 as i32,
                );
                1 as i32 != 0
            }) as i32;
        *((*v).ind).offset((*v).nnz as isize) = j;
    }
    *((*v).vec).offset(j as isize) += a;
    if fabs(*((*v).vec).offset(j as isize))
        < 1e-9f64 * (1 as i32 as libc::c_double + fabs(a))
    {
        *((*v).vec).offset(j as isize) = 2.2250738585072014e-308f64;
    }
}
unsafe extern "C" fn build_ks(
    mut csa: *mut csa,
    mut n: i32,
    mut ind: *mut i32,
    mut a: *mut libc::c_double,
    mut b: *mut libc::c_double,
    mut v: *mut FVS,
) -> i32 {
    let mut current_block: u64;
    let mut P: *mut glp_prob = (*csa).P;
    let mut l: *mut bnd = (*csa).l;
    let mut u: *mut bnd = (*csa).u;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    ((*v).nnz == 0 as i32
        || {
            glp_assert_(
                b"v->nnz == 0\0" as *const u8 as *const i8,
                b"intopt/covgen.c\0" as *const u8 as *const i8,
                312 as i32,
            );
            1 as i32 != 0
        }) as i32;
    j = 1 as i32;
    loop {
        if !(j <= n) {
            current_block = 14401909646449704462;
            break;
        }
        k = *ind.offset(j as isize);
        if glp_get_col_kind(P, k) == 3 as i32 {
            add_term(v, k, *a.offset(j as isize));
        } else if *a.offset(j as isize) > 0 as i32 as libc::c_double {
            if (*l.offset(k as isize)).b == -1.7976931348623157e+308f64 {
                n = -(1 as i32);
                current_block = 2916441921396448118;
                break;
            } else if (*l.offset(k as isize)).z == 0 as i32 {
                *b -= *a.offset(j as isize) * (*l.offset(k as isize)).b;
            } else {
                add_term(
                    v,
                    (*l.offset(k as isize)).z,
                    *a.offset(j as isize) * (*l.offset(k as isize)).a,
                );
                *b -= *a.offset(j as isize) * (*l.offset(k as isize)).b;
            }
        } else if (*u.offset(k as isize)).b == 1.7976931348623157e+308f64 {
            n = -(1 as i32);
            current_block = 2916441921396448118;
            break;
        } else if (*u.offset(k as isize)).z == 0 as i32 {
            *b -= *a.offset(j as isize) * (*u.offset(k as isize)).b;
        } else {
            add_term(
                v,
                (*u.offset(k as isize)).z,
                *a.offset(j as isize) * (*u.offset(k as isize)).a,
            );
            *b -= *a.offset(j as isize) * (*u.offset(k as isize)).b;
        }
        j += 1;
        j;
    }
    match current_block {
        14401909646449704462 => {
            _glp_fvs_adjust_vec(
                v,
                2 as i32 as libc::c_double * 2.2250738585072014e-308f64,
            );
            ((*v).nnz <= n
                || {
                    glp_assert_(
                        b"v->nnz <= n\0" as *const u8 as *const i8,
                        b"intopt/covgen.c\0" as *const u8 as *const i8,
                        360 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            n = (*v).nnz;
            j = 1 as i32;
            while j <= n {
                *ind.offset(j as isize) = *((*v).ind).offset(j as isize);
                *a.offset(j as isize) = *((*v).vec)
                    .offset(*ind.offset(j as isize) as isize);
                j += 1;
                j;
            }
        }
        _ => {}
    }
    _glp_fvs_clear_vec(v);
    return n;
}
unsafe extern "C" fn can_be_active(
    mut n: i32,
    mut a: *const libc::c_double,
    mut b: libc::c_double,
) -> i32 {
    let mut j: i32 = 0;
    let mut s: libc::c_double = 0.;
    s = 0 as i32 as libc::c_double;
    j = 1 as i32;
    while j <= n {
        if *a.offset(j as isize) > 0 as i32 as libc::c_double {
            s += *a.offset(j as isize);
        }
        j += 1;
        j;
    }
    return (s > b + 0.001f64 * (1 as i32 as libc::c_double + fabs(b))) as i32;
}
unsafe extern "C" fn is_sos_ineq(
    mut n: i32,
    mut a: *const libc::c_double,
    mut b: libc::c_double,
) -> i32 {
    let mut j: i32 = 0;
    let mut p: i32 = 0;
    let mut q: i32 = 0;
    (n >= 2 as i32
        || {
            glp_assert_(
                b"n >= 2\0" as *const u8 as *const i8,
                b"intopt/covgen.c\0" as *const u8 as *const i8,
                431 as i32,
            );
            1 as i32 != 0
        }) as i32;
    j = 1 as i32;
    while j <= n {
        if *a.offset(j as isize) < 0 as i32 as libc::c_double {
            b -= *a.offset(j as isize);
        }
        j += 1;
        j;
    }
    p = 1 as i32;
    j = 2 as i32;
    while j <= n {
        if fabs(*a.offset(p as isize)) > fabs(*a.offset(j as isize)) {
            p = j;
        }
        j += 1;
        j;
    }
    q = 0 as i32;
    j = 1 as i32;
    while j <= n {
        if j != p {
            if q == 0 as i32 || fabs(*a.offset(q as isize)) > fabs(*a.offset(j as isize))
            {
                q = j;
            }
        }
        j += 1;
        j;
    }
    (q != 0 as i32
        || {
            glp_assert_(
                b"q != 0\0" as *const u8 as *const i8,
                b"intopt/covgen.c\0" as *const u8 as *const i8,
                451 as i32,
            );
            1 as i32 != 0
        }) as i32;
    return (fabs(*a.offset(p as isize)) + fabs(*a.offset(q as isize))
        > b + 0.001f64 * (1 as i32 as libc::c_double + fabs(b))) as i32;
}
unsafe extern "C" fn process_ineq(
    mut csa: *mut csa,
    mut n: i32,
    mut ind: *mut i32,
    mut a: *mut libc::c_double,
    mut b: libc::c_double,
    mut v: *mut FVS,
) {
    let mut i: i32 = 0;
    n = build_ks(csa, n, ind, a, &mut b, v);
    if !(n <= 1 as i32) {
        if !(can_be_active(n, a as *const libc::c_double, b) == 0) {
            if !(is_sos_ineq(n, a as *const libc::c_double, b) != 0) {
                i = glp_add_rows((*csa).set, 1 as i32);
                glp_set_mat_row(
                    (*csa).set,
                    i,
                    n,
                    ind as *const i32,
                    a as *const libc::c_double,
                );
                glp_set_row_bnds((*csa).set, i, 3 as i32, b, b);
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn glp_cov_init(mut P: *mut glp_prob) -> *mut glp_cov {
    let mut cov: *mut glp_cov = 0 as *mut glp_cov;
    let mut csa: csa = csa {
        P: 0 as *mut glp_prob,
        l: 0 as *mut bnd,
        u: 0 as *mut bnd,
        set: 0 as *mut glp_prob,
    };
    let mut i: i32 = 0;
    let mut k: i32 = 0;
    let mut len: i32 = 0;
    let mut ind: *mut i32 = 0 as *mut i32;
    let mut rhs: libc::c_double = 0.;
    let mut val: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut fvs: FVS = FVS {
        n: 0,
        nnz: 0,
        ind: 0 as *mut i32,
        vec: 0 as *mut libc::c_double,
    };
    csa.P = P;
    csa.l = glp_alloc(1 as i32 + (*P).n, ::core::mem::size_of::<bnd>() as u64 as i32)
        as *mut bnd;
    csa.u = glp_alloc(1 as i32 + (*P).n, ::core::mem::size_of::<bnd>() as u64 as i32)
        as *mut bnd;
    csa.set = glp_create_prob();
    glp_add_cols(csa.set, (*P).n);
    init_bounds(&mut csa);
    obtain_vbs(&mut csa);
    ind = glp_alloc(1 as i32 + (*P).n, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    val = glp_alloc(
        1 as i32 + (*P).n,
        ::core::mem::size_of::<libc::c_double>() as u64 as i32,
    ) as *mut libc::c_double;
    _glp_fvs_alloc_vec(&mut fvs, (*P).n);
    i = 1 as i32;
    while i <= (*P).m {
        match (**((*P).row).offset(i as isize)).type_0 {
            1 => {}
            2 => {
                len = glp_get_mat_row(P, i, ind, val);
                rhs = (**((*P).row).offset(i as isize)).lb;
                k = 1 as i32;
                while k <= len {
                    *val.offset(k as isize) = -*val.offset(k as isize);
                    k += 1;
                    k;
                }
                rhs = -rhs;
                process_ineq(&mut csa, len, ind, val, rhs, &mut fvs);
            }
            3 => {
                len = glp_get_mat_row(P, i, ind, val);
                rhs = (**((*P).row).offset(i as isize)).ub;
                process_ineq(&mut csa, len, ind, val, rhs, &mut fvs);
            }
            4 | 5 => {
                len = glp_get_mat_row(P, i, ind, val);
                rhs = (**((*P).row).offset(i as isize)).lb;
                k = 1 as i32;
                while k <= len {
                    *val.offset(k as isize) = -*val.offset(k as isize);
                    k += 1;
                    k;
                }
                rhs = -rhs;
                process_ineq(&mut csa, len, ind, val, rhs, &mut fvs);
                len = glp_get_mat_row(P, i, ind, val);
                rhs = (**((*P).row).offset(i as isize)).ub;
                process_ineq(&mut csa, len, ind, val, rhs, &mut fvs);
            }
            _ => {
                (P != P
                    || {
                        glp_assert_(
                            b"P != P\0" as *const u8 as *const i8,
                            b"intopt/covgen.c\0" as *const u8 as *const i8,
                            564 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
            }
        }
        i += 1;
        i;
    }
    glp_free(ind as *mut libc::c_void);
    glp_free(val as *mut libc::c_void);
    _glp_fvs_check_vec(&mut fvs);
    _glp_fvs_free_vec(&mut fvs);
    if (*csa.set).m == 0 as i32 {
        glp_printf(
            b"No 0-1 knapsack inequalities detected\n\0" as *const u8 as *const i8,
        );
        cov = 0 as *mut glp_cov;
        glp_delete_prob(csa.set);
    } else {
        glp_printf(
            b"Number of 0-1 knapsack inequalities = %d\n\0" as *const u8 as *const i8,
            (*csa.set).m,
        );
        cov = glp_alloc(1 as i32, ::core::mem::size_of::<glp_cov>() as u64 as i32)
            as *mut glp_cov;
        (*cov).n = (*P).n;
        (*cov).set = csa.set;
    }
    glp_free(csa.l as *mut libc::c_void);
    glp_free(csa.u as *mut libc::c_void);
    return cov;
}
unsafe extern "C" fn solve_ks(
    mut n: i32,
    mut a: *const i32,
    mut b: i32,
    mut c: *const i32,
    mut x: *mut i8,
) -> i32 {
    let mut z: i32 = 0;
    if n <= 16 as i32 {
        z = _glp_ks_mt1(n, a, b, c, x);
    } else {
        z = _glp_ks_greedy(n, a, b, c, x);
    }
    return z;
}
unsafe extern "C" fn simple_cover(
    mut n: i32,
    mut a: *const libc::c_double,
    mut b: libc::c_double,
    mut x: *const libc::c_double,
    mut z: *mut i8,
) -> libc::c_double {
    let mut j: i32 = 0;
    let mut aa: *mut i32 = 0 as *mut i32;
    let mut bb: i32 = 0;
    let mut cc: *mut i32 = 0 as *mut i32;
    let mut max_aj: libc::c_double = 0.;
    let mut min_aj: libc::c_double = 0.;
    let mut s: libc::c_double = 0.;
    let mut eps: libc::c_double = 0.;
    (n >= 3 as i32
        || {
            glp_assert_(
                b"n >= 3\0" as *const u8 as *const i8,
                b"intopt/covgen.c\0" as *const u8 as *const i8,
                709 as i32,
            );
            1 as i32 != 0
        }) as i32;
    aa = glp_alloc(1 as i32 + n, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    cc = glp_alloc(1 as i32 + n, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    max_aj = 0 as i32 as libc::c_double;
    min_aj = 1.7976931348623157e+308f64;
    j = 1 as i32;
    while j <= n {
        (*a.offset(j as isize) > 0 as i32 as libc::c_double
            || {
                glp_assert_(
                    b"a[j] > 0\0" as *const u8 as *const i8,
                    b"intopt/covgen.c\0" as *const u8 as *const i8,
                    716 as i32,
                );
                1 as i32 != 0
            }) as i32;
        if max_aj < *a.offset(j as isize) {
            max_aj = *a.offset(j as isize);
        }
        if min_aj > *a.offset(j as isize) {
            min_aj = *a.offset(j as isize);
        }
        j += 1;
        j;
    }
    s = 0 as i32 as libc::c_double;
    j = 1 as i32;
    while j <= n {
        s += *a.offset(j as isize);
        *aa.offset(j as isize) = ceil(
            *a.offset(j as isize) / max_aj * 1000 as i32 as libc::c_double,
        ) as i32;
        j += 1;
        j;
    }
    bb = (floor((s - b) / max_aj * 1000 as i32 as libc::c_double)
        - 1 as i32 as libc::c_double) as i32;
    j = 1 as i32;
    while j <= n {
        (0 as i32 as libc::c_double <= *x.offset(j as isize)
            && *x.offset(j as isize) <= 1 as i32 as libc::c_double
            || {
                glp_assert_(
                    b"0 <= x[j] && x[j] <= 1\0" as *const u8 as *const i8,
                    b"intopt/covgen.c\0" as *const u8 as *const i8,
                    735 as i32,
                );
                1 as i32 != 0
            }) as i32;
        *cc.offset(j as isize) = floor(
            (1 as i32 as libc::c_double - *x.offset(j as isize))
                * 1000 as i32 as libc::c_double,
        ) as i32;
        j += 1;
        j;
    }
    if solve_ks(n, aa as *const i32, bb, cc as *const i32, z)
        == -(2147483647 as i32) - 1 as i32
    {
        s = 1.7976931348623157e+308f64;
    } else {
        j = 1 as i32;
        while j <= n {
            (*z.offset(j as isize) as i32 == 0 as i32
                || *z.offset(j as isize) as i32 == 1 as i32
                || {
                    glp_assert_(
                        b"z[j] == 0 || z[j] == 1\0" as *const u8 as *const i8,
                        b"intopt/covgen.c\0" as *const u8 as *const i8,
                        746 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            let ref mut fresh2 = *z.offset(j as isize);
            *fresh2 = (*fresh2 as i32 ^ 1 as i32) as i8;
            j += 1;
            j;
        }
        s = 0 as i32 as libc::c_double;
        j = 1 as i32;
        while j <= n {
            if *z.offset(j as isize) != 0 {
                s += *a.offset(j as isize);
            }
            j += 1;
            j;
        }
        eps = 0.01f64
            * (if min_aj >= 1 as i32 as libc::c_double {
                min_aj
            } else {
                1 as i32 as libc::c_double
            });
        if !(s >= b + eps) {
            s = 1.7976931348623157e+308f64;
        } else {
            s = 0 as i32 as libc::c_double;
            j = 1 as i32;
            while j <= n {
                if *z.offset(j as isize) != 0 {
                    s += 1 as i32 as libc::c_double - *x.offset(j as isize);
                }
                j += 1;
                j;
            }
        }
    }
    glp_free(aa as *mut libc::c_void);
    glp_free(cc as *mut libc::c_void);
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn glp_cov_gen1(
    mut P: *mut glp_prob,
    mut cov: *mut glp_cov,
    mut pool: *mut glp_prob,
) {
    let mut i: i32 = 0;
    let mut k: i32 = 0;
    let mut len: i32 = 0;
    let mut new_len: i32 = 0;
    let mut ind: *mut i32 = 0 as *mut i32;
    let mut val: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut rhs: libc::c_double = 0.;
    let mut x: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut zeta: libc::c_double = 0.;
    let mut z: *mut i8 = 0 as *mut i8;
    ((*P).n == (*cov).n && (*P).n == (*(*cov).set).n
        || {
            glp_assert_(
                b"P->n == cov->n && P->n == cov->set->n\0" as *const u8 as *const i8,
                b"intopt/covgen.c\0" as *const u8 as *const i8,
                780 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (glp_get_status(P) == 5 as i32
        || {
            glp_assert_(
                b"glp_get_status(P) == GLP_OPT\0" as *const u8 as *const i8,
                b"intopt/covgen.c\0" as *const u8 as *const i8,
                781 as i32,
            );
            1 as i32 != 0
        }) as i32;
    ind = glp_alloc(1 as i32 + (*P).n, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    val = glp_alloc(
        1 as i32 + (*P).n,
        ::core::mem::size_of::<libc::c_double>() as u64 as i32,
    ) as *mut libc::c_double;
    x = glp_alloc(
        1 as i32 + (*P).n,
        ::core::mem::size_of::<libc::c_double>() as u64 as i32,
    ) as *mut libc::c_double;
    z = glp_alloc(1 as i32 + (*P).n, ::core::mem::size_of::<i8>() as u64 as i32)
        as *mut i8;
    i = 1 as i32;
    while i <= (*(*cov).set).m {
        len = glp_get_mat_row((*cov).set, i, ind, val);
        rhs = glp_get_row_ub((*cov).set, i);
        (rhs != 1.7976931348623157e+308f64
            || {
                glp_assert_(
                    b"rhs != +DBL_MAX\0" as *const u8 as *const i8,
                    b"intopt/covgen.c\0" as *const u8 as *const i8,
                    792 as i32,
                );
                1 as i32 != 0
            }) as i32;
        new_len = 0 as i32;
        k = 1 as i32;
        while k <= len {
            if glp_get_col_type(P, *ind.offset(k as isize)) == 5 as i32 {
                rhs
                    -= *val.offset(k as isize)
                        * glp_get_col_prim(P, *ind.offset(k as isize));
            } else {
                new_len += 1;
                new_len;
                *ind.offset(new_len as isize) = *ind.offset(k as isize);
                *val.offset(new_len as isize) = *val.offset(k as isize);
            }
            k += 1;
            k;
        }
        len = new_len;
        if !(len <= 2 as i32) {
            k = 1 as i32;
            while k <= len {
                (glp_get_col_kind(P, *ind.offset(k as isize)) == 3 as i32
                    || {
                        glp_assert_(
                            b"glp_get_col_kind(P, ind[k]) == GLP_BV\0" as *const u8
                                as *const i8,
                            b"intopt/covgen.c\0" as *const u8 as *const i8,
                            814 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                *x.offset(k as isize) = glp_get_col_prim(P, *ind.offset(k as isize));
                if *x.offset(k as isize) < 0.00001f64 {
                    *x.offset(k as isize) = 0 as i32 as libc::c_double;
                } else if *x.offset(k as isize) > 0.99999f64 {
                    *x.offset(k as isize) = 1 as i32 as libc::c_double;
                }
                if *val.offset(k as isize) < 0 as i32 as libc::c_double {
                    *ind.offset(k as isize) = -*ind.offset(k as isize);
                    *val.offset(k as isize) = -*val.offset(k as isize);
                    rhs += *val.offset(k as isize);
                    *x.offset(k as isize) = 1 as i32 as libc::c_double
                        - *x.offset(k as isize);
                }
                k += 1;
                k;
            }
            zeta = simple_cover(
                len,
                val as *const libc::c_double,
                rhs,
                x as *const libc::c_double,
                z,
            );
            if !(zeta > 0.95f64) {
                new_len = 0 as i32;
                rhs = -(1 as i32) as libc::c_double;
                k = 1 as i32;
                while k <= len {
                    if *z.offset(k as isize) != 0 {
                        new_len += 1;
                        new_len;
                        if *ind.offset(k as isize) > 0 as i32 {
                            *ind.offset(new_len as isize) = *ind.offset(k as isize);
                            *val.offset(new_len as isize) = 1 as i32 as libc::c_double;
                            rhs += 1.;
                            rhs;
                        } else {
                            *ind.offset(new_len as isize) = -*ind.offset(k as isize);
                            *val.offset(new_len as isize) = -(1 as i32)
                                as libc::c_double;
                        }
                    }
                    k += 1;
                    k;
                }
                len = new_len;
                k = glp_add_rows(pool, 1 as i32);
                glp_set_mat_row(
                    pool,
                    k,
                    len,
                    ind as *const i32,
                    val as *const libc::c_double,
                );
                glp_set_row_bnds(pool, k, 3 as i32, rhs, rhs);
            }
        }
        i += 1;
        i;
    }
    glp_free(ind as *mut libc::c_void);
    glp_free(val as *mut libc::c_void);
    glp_free(x as *mut libc::c_void);
    glp_free(z as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn glp_cov_free(mut cov: *mut glp_cov) {
    (!cov.is_null()
        || {
            glp_assert_(
                b"cov != NULL\0" as *const u8 as *const i8,
                b"intopt/covgen.c\0" as *const u8 as *const i8,
                877 as i32,
            );
            1 as i32 != 0
        }) as i32;
    glp_delete_prob((*cov).set);
    glp_free(cov as *mut libc::c_void);
}