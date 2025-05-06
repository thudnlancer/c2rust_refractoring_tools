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
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
    fn glp_alloc(n: i32, size: i32) -> *mut libc::c_void;
    fn glp_free(ptr: *mut libc::c_void);
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
pub struct SPXLP {
    pub m: i32,
    pub n: i32,
    pub nnz: i32,
    pub A_ptr: *mut i32,
    pub A_ind: *mut i32,
    pub A_val: *mut libc::c_double,
    pub b: *mut libc::c_double,
    pub c: *mut libc::c_double,
    pub l: *mut libc::c_double,
    pub u: *mut libc::c_double,
    pub head: *mut i32,
    pub flag: *mut i8,
    pub valid: i32,
    pub bfd: *mut BFD,
}
#[no_mangle]
pub unsafe extern "C" fn _glp_spx_init_lp(
    mut lp: *mut SPXLP,
    mut P: *mut glp_prob,
    mut excl: i32,
) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut m: i32 = 0;
    let mut n: i32 = 0;
    let mut nnz: i32 = 0;
    m = (*P).m;
    (m > 0 as i32
        || {
            glp_assert_(
                b"m > 0\0" as *const u8 as *const i8,
                b"simplex/spxprob.c\0" as *const u8 as *const i8,
                40 as i32,
            );
            1 as i32 != 0
        }) as i32;
    n = 0 as i32;
    nnz = (*P).nnz;
    ((*P).valid != 0
        || {
            glp_assert_(
                b"P->valid\0" as *const u8 as *const i8,
                b"simplex/spxprob.c\0" as *const u8 as *const i8,
                43 as i32,
            );
            1 as i32 != 0
        }) as i32;
    i = 1 as i32;
    while i <= m {
        let mut row: *mut GLPROW = *((*P).row).offset(i as isize);
        if !(excl != 0 && (*row).stat == 5 as i32) {
            n += 1;
            n;
            nnz += 1;
            nnz;
        }
        i += 1;
        i;
    }
    j = 1 as i32;
    while j <= (*P).n {
        let mut col: *mut GLPCOL = *((*P).col).offset(j as isize);
        if excl != 0 && (*col).stat == 5 as i32 {
            let mut aij: *mut GLPAIJ = 0 as *mut GLPAIJ;
            aij = (*col).ptr;
            while !aij.is_null() {
                nnz -= 1;
                nnz;
                aij = (*aij).c_next;
            }
        } else {
            n += 1;
            n;
        }
        j += 1;
        j;
    }
    memset(lp as *mut libc::c_void, 0 as i32, ::core::mem::size_of::<SPXLP>() as u64);
    (*lp).m = m;
    (n > 0 as i32
        || {
            glp_assert_(
                b"n > 0\0" as *const u8 as *const i8,
                b"simplex/spxprob.c\0" as *const u8 as *const i8,
                74 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (*lp).n = n;
    (*lp).nnz = nnz;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_spx_alloc_lp(mut lp: *mut SPXLP) {
    let mut m: i32 = (*lp).m;
    let mut n: i32 = (*lp).n;
    let mut nnz: i32 = (*lp).nnz;
    (*lp).A_ptr = glp_alloc(
        1 as i32 + n + 1 as i32,
        ::core::mem::size_of::<i32>() as u64 as i32,
    ) as *mut i32;
    (*lp).A_ind = glp_alloc(1 as i32 + nnz, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    (*lp).A_val = glp_alloc(
        1 as i32 + nnz,
        ::core::mem::size_of::<libc::c_double>() as u64 as i32,
    ) as *mut libc::c_double;
    (*lp).b = glp_alloc(
        1 as i32 + m,
        ::core::mem::size_of::<libc::c_double>() as u64 as i32,
    ) as *mut libc::c_double;
    (*lp).c = glp_alloc(
        1 as i32 + n,
        ::core::mem::size_of::<libc::c_double>() as u64 as i32,
    ) as *mut libc::c_double;
    (*lp).l = glp_alloc(
        1 as i32 + n,
        ::core::mem::size_of::<libc::c_double>() as u64 as i32,
    ) as *mut libc::c_double;
    (*lp).u = glp_alloc(
        1 as i32 + n,
        ::core::mem::size_of::<libc::c_double>() as u64 as i32,
    ) as *mut libc::c_double;
    (*lp).head = glp_alloc(1 as i32 + n, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    (*lp).flag = glp_alloc(1 as i32 + n - m, ::core::mem::size_of::<i8>() as u64 as i32)
        as *mut i8;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_spx_build_lp(
    mut lp: *mut SPXLP,
    mut P: *mut glp_prob,
    mut excl: i32,
    mut shift: i32,
    mut map: *mut i32,
) {
    let mut m: i32 = (*lp).m;
    let mut n: i32 = (*lp).n;
    let mut nnz: i32 = (*lp).nnz;
    let mut A_ptr: *mut i32 = (*lp).A_ptr;
    let mut A_ind: *mut i32 = (*lp).A_ind;
    let mut A_val: *mut libc::c_double = (*lp).A_val;
    let mut b: *mut libc::c_double = (*lp).b;
    let mut c: *mut libc::c_double = (*lp).c;
    let mut l: *mut libc::c_double = (*lp).l;
    let mut u: *mut libc::c_double = (*lp).u;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut kk: i32 = 0;
    let mut ptr: i32 = 0;
    let mut end: i32 = 0;
    let mut dir: libc::c_double = 0.;
    let mut delta: libc::c_double = 0.;
    match (*P).dir {
        1 => {
            dir = 1.0f64;
        }
        2 => {
            dir = -1.0f64;
        }
        _ => {
            (P != P
                || {
                    glp_assert_(
                        b"P != P\0" as *const u8 as *const i8,
                        b"simplex/spxprob.c\0" as *const u8 as *const i8,
                        164 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        }
    }
    *c.offset(0 as i32 as isize) = dir * (*P).c0;
    k = 0 as i32;
    ptr = 1 as i32;
    ((*P).m == m
        || {
            glp_assert_(
                b"P->m == m\0" as *const u8 as *const i8,
                b"simplex/spxprob.c\0" as *const u8 as *const i8,
                171 as i32,
            );
            1 as i32 != 0
        }) as i32;
    i = 1 as i32;
    while i <= m {
        let mut row: *mut GLPROW = *((*P).row).offset(i as isize);
        if excl != 0 && (*row).stat == 5 as i32 {
            ((*row).type_0 == 5 as i32
                || {
                    glp_assert_(
                        b"row->type == GLP_FX\0" as *const u8 as *const i8,
                        b"simplex/spxprob.c\0" as *const u8 as *const i8,
                        177 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            *map.offset(i as isize) = 0 as i32;
            *b.offset(i as isize) = -(*row).lb * (*row).rii;
        } else {
            k += 1;
            *map.offset(i as isize) = k;
            *A_ptr.offset(k as isize) = ptr;
            *A_ind.offset(ptr as isize) = i;
            *A_val.offset(ptr as isize) = 1.0f64;
            ptr += 1;
            ptr;
            let ref mut fresh0 = *c.offset(k as isize);
            *fresh0 = 0.0f64;
            *b.offset(i as isize) = *fresh0;
            match (*row).type_0 {
                1 => {
                    *l.offset(k as isize) = -1.7976931348623157e+308f64;
                    *u.offset(k as isize) = 1.7976931348623157e+308f64;
                }
                2 => {
                    *l.offset(k as isize) = (*row).lb * (*row).rii;
                    *u.offset(k as isize) = 1.7976931348623157e+308f64;
                }
                3 => {
                    *l.offset(k as isize) = -1.7976931348623157e+308f64;
                    *u.offset(k as isize) = (*row).ub * (*row).rii;
                }
                4 => {
                    *l.offset(k as isize) = (*row).lb * (*row).rii;
                    *u.offset(k as isize) = (*row).ub * (*row).rii;
                    (*l.offset(k as isize) != *u.offset(k as isize)
                        || {
                            glp_assert_(
                                b"l[k] != u[k]\0" as *const u8 as *const i8,
                                b"simplex/spxprob.c\0" as *const u8 as *const i8,
                                206 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                }
                5 => {
                    let ref mut fresh1 = *u.offset(k as isize);
                    *fresh1 = (*row).lb * (*row).rii;
                    *l.offset(k as isize) = *fresh1;
                }
                _ => {
                    (row != row
                        || {
                            glp_assert_(
                                b"row != row\0" as *const u8 as *const i8,
                                b"simplex/spxprob.c\0" as *const u8 as *const i8,
                                212 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                }
            }
        }
        i += 1;
        i;
    }
    j = 1 as i32;
    while j <= (*P).n {
        let mut col: *mut GLPCOL = *((*P).col).offset(j as isize);
        let mut aij: *mut GLPAIJ = 0 as *mut GLPAIJ;
        if excl != 0 && (*col).stat == 5 as i32 {
            ((*col).type_0 == 5 as i32
                || {
                    glp_assert_(
                        b"col->type == GLP_FX\0" as *const u8 as *const i8,
                        b"simplex/spxprob.c\0" as *const u8 as *const i8,
                        223 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            *map.offset((m + j) as isize) = 0 as i32;
            if (*col).lb != 0.0f64 {
                aij = (*col).ptr;
                while !aij.is_null() {
                    *b.offset((*(*aij).row).i as isize)
                        += (*(*aij).row).rii * (*aij).val * (*col).lb;
                    aij = (*aij).c_next;
                }
                *c.offset(0 as i32 as isize) += dir * (*col).coef * (*col).lb;
            }
        } else {
            k += 1;
            *map.offset((m + j) as isize) = k;
            *A_ptr.offset(k as isize) = ptr;
            aij = (*col).ptr;
            while !aij.is_null() {
                *A_ind.offset(ptr as isize) = (*(*aij).row).i;
                *A_val.offset(ptr as isize) = -(*(*aij).row).rii * (*aij).val
                    * (*col).sjj;
                ptr += 1;
                ptr;
                aij = (*aij).c_next;
            }
            *c.offset(k as isize) = dir * (*col).coef * (*col).sjj;
            match (*col).type_0 {
                1 => {
                    *l.offset(k as isize) = -1.7976931348623157e+308f64;
                    *u.offset(k as isize) = 1.7976931348623157e+308f64;
                }
                2 => {
                    *l.offset(k as isize) = (*col).lb / (*col).sjj;
                    *u.offset(k as isize) = 1.7976931348623157e+308f64;
                }
                3 => {
                    *l.offset(k as isize) = -1.7976931348623157e+308f64;
                    *u.offset(k as isize) = (*col).ub / (*col).sjj;
                }
                4 => {
                    *l.offset(k as isize) = (*col).lb / (*col).sjj;
                    *u.offset(k as isize) = (*col).ub / (*col).sjj;
                    (*l.offset(k as isize) != *u.offset(k as isize)
                        || {
                            glp_assert_(
                                b"l[k] != u[k]\0" as *const u8 as *const i8,
                                b"simplex/spxprob.c\0" as *const u8 as *const i8,
                                259 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                }
                5 => {
                    let ref mut fresh2 = *u.offset(k as isize);
                    *fresh2 = (*col).lb / (*col).sjj;
                    *l.offset(k as isize) = *fresh2;
                }
                _ => {
                    (col != col
                        || {
                            glp_assert_(
                                b"col != col\0" as *const u8 as *const i8,
                                b"simplex/spxprob.c\0" as *const u8 as *const i8,
                                265 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                }
            }
        }
        j += 1;
        j;
    }
    (k == n
        || {
            glp_assert_(
                b"k == n\0" as *const u8 as *const i8,
                b"simplex/spxprob.c\0" as *const u8 as *const i8,
                269 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (ptr == nnz + 1 as i32
        || {
            glp_assert_(
                b"ptr == nnz+1\0" as *const u8 as *const i8,
                b"simplex/spxprob.c\0" as *const u8 as *const i8,
                270 as i32,
            );
            1 as i32 != 0
        }) as i32;
    *A_ptr.offset((n + 1 as i32) as isize) = ptr;
    if shift != 0 {
        kk = 1 as i32;
        while kk <= m + (*P).n {
            k = *map.offset(kk as isize);
            if !(k == 0 as i32) {
                if *l.offset(k as isize) == -1.7976931348623157e+308f64
                    && *u.offset(k as isize) == 1.7976931348623157e+308f64
                {
                    delta = 0.0f64;
                } else if *l.offset(k as isize) != -1.7976931348623157e+308f64
                    && *u.offset(k as isize) == 1.7976931348623157e+308f64
                {
                    delta = *l.offset(k as isize);
                    *l.offset(k as isize) = 0.0f64;
                } else if *l.offset(k as isize) == -1.7976931348623157e+308f64
                    && *u.offset(k as isize) != 1.7976931348623157e+308f64
                {
                    *map.offset(kk as isize) = -k;
                    delta = *u.offset(k as isize);
                    *u.offset(k as isize) = 0.0f64;
                } else if *l.offset(k as isize) != *u.offset(k as isize) {
                    if fabs(*l.offset(k as isize)) <= fabs(*u.offset(k as isize)) {
                        delta = *l.offset(k as isize);
                        *l.offset(k as isize) = 0.0f64;
                        *u.offset(k as isize) -= delta;
                    } else {
                        *map.offset(kk as isize) = -k;
                        delta = *u.offset(k as isize);
                        *l.offset(k as isize) -= delta;
                        *u.offset(k as isize) = 0.0f64;
                    }
                    (*l.offset(k as isize) != *u.offset(k as isize)
                        || {
                            glp_assert_(
                                b"l[k] != u[k]\0" as *const u8 as *const i8,
                                b"simplex/spxprob.c\0" as *const u8 as *const i8,
                                309 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                } else {
                    delta = *l.offset(k as isize);
                    let ref mut fresh3 = *u.offset(k as isize);
                    *fresh3 = 0.0f64;
                    *l.offset(k as isize) = *fresh3;
                }
                if delta != 0.0f64 {
                    ptr = *A_ptr.offset(k as isize);
                    end = *A_ptr.offset((k + 1 as i32) as isize);
                    while ptr < end {
                        *b.offset(*A_ind.offset(ptr as isize) as isize)
                            -= *A_val.offset(ptr as isize) * delta;
                        ptr += 1;
                        ptr;
                    }
                    *c.offset(0 as i32 as isize) += *c.offset(k as isize) * delta;
                }
            }
            kk += 1;
            kk;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn _glp_spx_build_basis(
    mut lp: *mut SPXLP,
    mut P: *mut glp_prob,
    mut map: *const i32,
) {
    let mut m: i32 = (*lp).m;
    let mut n: i32 = (*lp).n;
    let mut head: *mut i32 = (*lp).head;
    let mut flag: *mut i8 = (*lp).flag;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut ii: i32 = 0;
    let mut jj: i32 = 0;
    ((*P).m == m
        || {
            glp_assert_(
                b"P->m == m\0" as *const u8 as *const i8,
                b"simplex/spxprob.c\0" as *const u8 as *const i8,
                349 as i32,
            );
            1 as i32 != 0
        }) as i32;
    ((*P).valid != 0
        || {
            glp_assert_(
                b"P->valid\0" as *const u8 as *const i8,
                b"simplex/spxprob.c\0" as *const u8 as *const i8,
                350 as i32,
            );
            1 as i32 != 0
        }) as i32;
    memset(
        &mut *head.offset(1 as i32 as isize) as *mut i32 as *mut libc::c_void,
        0 as i32,
        (m as u64).wrapping_mul(::core::mem::size_of::<i32>() as u64),
    );
    jj = 0 as i32;
    ((*P).m == m
        || {
            glp_assert_(
                b"P->m == m\0" as *const u8 as *const i8,
                b"simplex/spxprob.c\0" as *const u8 as *const i8,
                355 as i32,
            );
            1 as i32 != 0
        }) as i32;
    i = 1 as i32;
    while i <= m {
        let mut row: *mut GLPROW = *((*P).row).offset(i as isize);
        k = *map.offset(i as isize);
        if k < 0 as i32 {
            k = -k;
        }
        if !(k == 0 as i32) {
            (1 as i32 <= k && k <= n
                || {
                    glp_assert_(
                        b"1 <= k && k <= n\0" as *const u8 as *const i8,
                        b"simplex/spxprob.c\0" as *const u8 as *const i8,
                        365 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            if (*row).stat == 1 as i32 {
                ii = (*row).bind;
                (1 as i32 <= ii && ii <= m
                    || {
                        glp_assert_(
                            b"1 <= ii && ii <= m\0" as *const u8 as *const i8,
                            b"simplex/spxprob.c\0" as *const u8 as *const i8,
                            369 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                (*head.offset(ii as isize) == 0 as i32
                    || {
                        glp_assert_(
                            b"head[ii] == 0\0" as *const u8 as *const i8,
                            b"simplex/spxprob.c\0" as *const u8 as *const i8,
                            370 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                *head.offset(ii as isize) = k;
            } else {
                jj += 1;
                jj;
                *head.offset((m + jj) as isize) = k;
                *flag.offset(jj as isize) = ((*row).stat == 3 as i32) as i32 as i8;
            }
        }
        i += 1;
        i;
    }
    j = 1 as i32;
    while j <= (*P).n {
        let mut col: *mut GLPCOL = *((*P).col).offset(j as isize);
        k = *map.offset((m + j) as isize);
        if k < 0 as i32 {
            k = -k;
        }
        if !(k == 0 as i32) {
            (1 as i32 <= k && k <= n
                || {
                    glp_assert_(
                        b"1 <= k && k <= n\0" as *const u8 as *const i8,
                        b"simplex/spxprob.c\0" as *const u8 as *const i8,
                        390 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            if (*col).stat == 1 as i32 {
                ii = (*col).bind;
                (1 as i32 <= ii && ii <= m
                    || {
                        glp_assert_(
                            b"1 <= ii && ii <= m\0" as *const u8 as *const i8,
                            b"simplex/spxprob.c\0" as *const u8 as *const i8,
                            394 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                (*head.offset(ii as isize) == 0 as i32
                    || {
                        glp_assert_(
                            b"head[ii] == 0\0" as *const u8 as *const i8,
                            b"simplex/spxprob.c\0" as *const u8 as *const i8,
                            395 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                *head.offset(ii as isize) = k;
            } else {
                jj += 1;
                jj;
                *head.offset((m + jj) as isize) = k;
                *flag.offset(jj as isize) = ((*col).stat == 3 as i32) as i32 as i8;
            }
        }
        j += 1;
        j;
    }
    (m + jj == n
        || {
            glp_assert_(
                b"m+jj == n\0" as *const u8 as *const i8,
                b"simplex/spxprob.c\0" as *const u8 as *const i8,
                405 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (*lp).valid = 1 as i32;
    (*lp).bfd = (*P).bfd;
    (*P).valid = 0 as i32;
    (*P).bfd = 0 as *mut BFD;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_spx_store_basis(
    mut lp: *mut SPXLP,
    mut P: *mut glp_prob,
    mut map: *const i32,
    mut daeh: *mut i32,
) {
    let mut m: i32 = (*lp).m;
    let mut n: i32 = (*lp).n;
    let mut head: *mut i32 = (*lp).head;
    let mut flag: *mut i8 = (*lp).flag;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut kk: i32 = 0;
    kk = 1 as i32;
    while kk <= n {
        *daeh.offset(*head.offset(kk as isize) as isize) = kk;
        kk += 1;
        kk;
    }
    ((*P).m == m
        || {
            glp_assert_(
                b"P->m == m\0" as *const u8 as *const i8,
                b"simplex/spxprob.c\0" as *const u8 as *const i8,
                441 as i32,
            );
            1 as i32 != 0
        }) as i32;
    i = 1 as i32;
    while i <= m {
        let mut row: *mut GLPROW = *((*P).row).offset(i as isize);
        k = *map.offset(i as isize);
        if k < 0 as i32 {
            k = -k;
        }
        if k == 0 as i32 {
            ((*row).type_0 == 5 as i32
                || {
                    glp_assert_(
                        b"row->type == GLP_FX\0" as *const u8 as *const i8,
                        b"simplex/spxprob.c\0" as *const u8 as *const i8,
                        448 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            (*row).stat = 5 as i32;
            (*row).bind = 0 as i32;
        } else {
            kk = *daeh.offset(k as isize);
            if kk <= m {
                *((*P).head).offset(kk as isize) = i;
                (*row).stat = 1 as i32;
                (*row).bind = kk;
            } else {
                match (*row).type_0 {
                    1 => {
                        (*row).stat = 4 as i32;
                    }
                    2 => {
                        (*row).stat = 2 as i32;
                    }
                    3 => {
                        (*row).stat = 3 as i32;
                    }
                    4 => {
                        (*row).stat = if *flag.offset((kk - m) as isize) as i32 != 0 {
                            3 as i32
                        } else {
                            2 as i32
                        };
                    }
                    5 => {
                        (*row).stat = 5 as i32;
                    }
                    _ => {
                        (row != row
                            || {
                                glp_assert_(
                                    b"row != row\0" as *const u8 as *const i8,
                                    b"simplex/spxprob.c\0" as *const u8 as *const i8,
                                    480 as i32,
                                );
                                1 as i32 != 0
                            }) as i32;
                    }
                }
                (*row).bind = 0 as i32;
            }
        }
        i += 1;
        i;
    }
    j = 1 as i32;
    while j <= (*P).n {
        let mut col: *mut GLPCOL = *((*P).col).offset(j as isize);
        k = *map.offset((m + j) as isize);
        if k < 0 as i32 {
            k = -k;
        }
        if k == 0 as i32 {
            ((*col).type_0 == 5 as i32
                || {
                    glp_assert_(
                        b"col->type == GLP_FX\0" as *const u8 as *const i8,
                        b"simplex/spxprob.c\0" as *const u8 as *const i8,
                        493 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            (*col).stat = 5 as i32;
            (*col).bind = 0 as i32;
        } else {
            kk = *daeh.offset(k as isize);
            if kk <= m {
                *((*P).head).offset(kk as isize) = m + j;
                (*col).stat = 1 as i32;
                (*col).bind = kk;
            } else {
                match (*col).type_0 {
                    1 => {
                        (*col).stat = 4 as i32;
                    }
                    2 => {
                        (*col).stat = 2 as i32;
                    }
                    3 => {
                        (*col).stat = 3 as i32;
                    }
                    4 => {
                        (*col).stat = if *flag.offset((kk - m) as isize) as i32 != 0 {
                            3 as i32
                        } else {
                            2 as i32
                        };
                    }
                    5 => {
                        (*col).stat = 5 as i32;
                    }
                    _ => {
                        (col != col
                            || {
                                glp_assert_(
                                    b"col != col\0" as *const u8 as *const i8,
                                    b"simplex/spxprob.c\0" as *const u8 as *const i8,
                                    525 as i32,
                                );
                                1 as i32 != 0
                            }) as i32;
                    }
                }
                (*col).bind = 0 as i32;
            }
        }
        j += 1;
        j;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _glp_spx_store_sol(
    mut lp: *mut SPXLP,
    mut P: *mut glp_prob,
    mut shift: i32,
    mut map: *const i32,
    mut daeh: *const i32,
    mut beta: *const libc::c_double,
    mut pi: *const libc::c_double,
    mut d: *const libc::c_double,
) {
    let mut m: i32 = (*lp).m;
    let mut flag: *mut i8 = (*lp).flag;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut kk: i32 = 0;
    let mut dir: libc::c_double = 0.;
    match (*P).dir {
        1 => {
            dir = 1.0f64;
        }
        2 => {
            dir = -1.0f64;
        }
        _ => {
            (P != P
                || {
                    glp_assert_(
                        b"P != P\0" as *const u8 as *const i8,
                        b"simplex/spxprob.c\0" as *const u8 as *const i8,
                        584 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        }
    }
    ((*P).m == m
        || {
            glp_assert_(
                b"P->m == m\0" as *const u8 as *const i8,
                b"simplex/spxprob.c\0" as *const u8 as *const i8,
                587 as i32,
            );
            1 as i32 != 0
        }) as i32;
    i = 1 as i32;
    while i <= m {
        let mut row: *mut GLPROW = *((*P).row).offset(i as isize);
        k = *map.offset(i as isize);
        if k < 0 as i32 {
            k = -k;
        }
        if k == 0 as i32 {
            ((*row).type_0 == 5 as i32
                || {
                    glp_assert_(
                        b"row->type == GLP_FX\0" as *const u8 as *const i8,
                        b"simplex/spxprob.c\0" as *const u8 as *const i8,
                        594 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            (*row).prim = (*row).lb;
            (*row).dual = -dir * *pi.offset(i as isize) * (*row).rii;
        } else {
            kk = *daeh.offset(k as isize);
            if kk <= m {
                (*row).prim = *beta.offset(kk as isize) / (*row).rii;
                if shift != 0 {
                    (*row).prim
                        += if *map.offset(i as isize) < 0 as i32 {
                            (*row).ub
                        } else {
                            (*row).lb
                        };
                }
                (*row).dual = 0.0f64;
            } else {
                (*row).prim = if *flag.offset((kk - m) as isize) as i32 != 0 {
                    (*row).ub
                } else {
                    (*row).lb
                };
                (*row).dual = dir * *d.offset((kk - m) as isize) * (*row).rii;
            }
        }
        i += 1;
        i;
    }
    (*P).obj_val = (*P).c0;
    j = 1 as i32;
    while j <= (*P).n {
        let mut col: *mut GLPCOL = *((*P).col).offset(j as isize);
        k = *map.offset((m + j) as isize);
        if k < 0 as i32 {
            k = -k;
        }
        if k == 0 as i32 {
            let mut aij: *mut GLPAIJ = 0 as *mut GLPAIJ;
            let mut dk: libc::c_double = 0.;
            ((*col).type_0 == 5 as i32
                || {
                    glp_assert_(
                        b"col->type == GLP_FX\0" as *const u8 as *const i8,
                        b"simplex/spxprob.c\0" as *const u8 as *const i8,
                        627 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            (*col).prim = (*col).lb;
            dk = dir * (*col).coef;
            aij = (*col).ptr;
            while !aij.is_null() {
                dk
                    += (*(*aij).row).rii * (*aij).val
                        * *pi.offset((*(*aij).row).i as isize);
                aij = (*aij).c_next;
            }
            (*col).dual = dir * dk;
        } else {
            kk = *daeh.offset(k as isize);
            if kk <= m {
                (*col).prim = *beta.offset(kk as isize) * (*col).sjj;
                if shift != 0 {
                    (*col).prim
                        += if *map.offset((m + j) as isize) < 0 as i32 {
                            (*col).ub
                        } else {
                            (*col).lb
                        };
                }
                (*col).dual = 0.0f64;
            } else {
                (*col).prim = if *flag.offset((kk - m) as isize) as i32 != 0 {
                    (*col).ub
                } else {
                    (*col).lb
                };
                (*col).dual = dir * *d.offset((kk - m) as isize) / (*col).sjj;
            }
        }
        (*P).obj_val += (*col).coef * (*col).prim;
        j += 1;
        j;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _glp_spx_free_lp(mut lp: *mut SPXLP) {
    glp_free((*lp).A_ptr as *mut libc::c_void);
    glp_free((*lp).A_ind as *mut libc::c_void);
    glp_free((*lp).A_val as *mut libc::c_void);
    glp_free((*lp).b as *mut libc::c_void);
    glp_free((*lp).c as *mut libc::c_void);
    glp_free((*lp).l as *mut libc::c_void);
    glp_free((*lp).u as *mut libc::c_void);
    glp_free((*lp).head as *mut libc::c_void);
    glp_free((*lp).flag as *mut libc::c_void);
}