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
    pub type DMP;
    pub type AVLNODE;
    pub type BFD;
    pub type AVL;
    pub type glp_tree;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn _glp_cfg_create_graph(n: i32, nv_max: i32) -> *mut CFG;
    fn _glp_cfg_add_clique(G: *mut CFG, size: i32, ind: *const i32);
    fn _glp_cfg_get_adjacent(G: *mut CFG, v: i32, ind: *mut i32) -> i32;
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
    fn glp_alloc(n: i32, size: i32) -> *mut libc::c_void;
    fn glp_free(ptr: *mut libc::c_void);
    fn glp_get_mat_row(
        P: *mut glp_prob,
        i: i32,
        ind: *mut i32,
        val: *mut libc::c_double,
    ) -> i32;
    fn glp_get_num_bin(P: *mut glp_prob) -> i32;
    fn _glp_wclique(n: i32, w: *const i32, a: *const u8, ind: *mut i32) -> i32;
    fn _glp_wclique1(
        n: i32,
        w: *const libc::c_double,
        func_0: Option<unsafe extern "C" fn(*mut libc::c_void, i32, *mut i32) -> i32>,
        info: *mut libc::c_void,
        c: *mut i32,
    ) -> i32;
}
pub type size_t = u64;
pub type __compar_fn_t = Option<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glp_cfg {
    pub n: i32,
    pub pos: *mut i32,
    pub neg: *mut i32,
    pub pool: *mut DMP,
    pub nv_max: i32,
    pub nv: i32,
    pub ref_0: *mut i32,
    pub vptr: *mut *mut CFGVLE,
    pub cptr: *mut *mut CFGCLE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CFGCLE {
    pub vptr: *mut CFGVLE,
    pub next: *mut CFGCLE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CFGVLE {
    pub v: i32,
    pub next: *mut CFGVLE,
}
pub type CFG = glp_cfg;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct term {
    pub ind: i32,
    pub val: libc::c_double,
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
pub struct csa {
    pub P: *mut glp_prob,
    pub G: *mut CFG,
    pub ind: *mut i32,
    pub nn: i32,
    pub vtoi: *mut i32,
    pub itov: *mut i32,
    pub wgt: *mut libc::c_double,
}
unsafe extern "C" fn fcmp(
    mut e1: *const libc::c_void,
    mut e2: *const libc::c_void,
) -> i32 {
    let mut t1: *const term = e1 as *const term;
    let mut t2: *const term = e2 as *const term;
    if (*t1).val > (*t2).val {
        return -(1 as i32)
    } else if (*t1).val < (*t2).val {
        return 1 as i32
    } else {
        return 0 as i32
    };
}
unsafe extern "C" fn analyze_ineq(
    mut P: *mut glp_prob,
    mut G: *mut CFG,
    mut len: i32,
    mut ind: *mut i32,
    mut val: *mut libc::c_double,
    mut rhs: libc::c_double,
    mut t: *mut term,
) {
    let mut current_block: u64;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut kk: i32 = 0;
    let mut p: i32 = 0;
    let mut q: i32 = 0;
    let mut type_0: i32 = 0;
    let mut new_len: i32 = 0;
    new_len = 0 as i32;
    k = 1 as i32;
    loop {
        if !(k <= len) {
            current_block = 17860125682698302841;
            break;
        }
        j = *ind.offset(k as isize);
        if (**((*P).col).offset(j as isize)).kind == 2 as i32
            && (**((*P).col).offset(j as isize)).type_0 == 4 as i32
            && (**((*P).col).offset(j as isize)).lb == 0.0f64
            && (**((*P).col).offset(j as isize)).ub == 1.0f64
        {
            new_len += 1;
            new_len;
            *ind.offset(new_len as isize) = j;
            *val.offset(new_len as isize) = *val.offset(k as isize);
        } else if *val.offset(k as isize) > 0.0f64 {
            type_0 = (**((*P).col).offset(j as isize)).type_0;
            if type_0 == 1 as i32 || type_0 == 3 as i32 {
                current_block = 10051727868454315572;
                break;
            } else {
                rhs -= *val.offset(k as isize) * (**((*P).col).offset(j as isize)).lb;
            }
        } else {
            type_0 = (**((*P).col).offset(j as isize)).type_0;
            if type_0 == 1 as i32 || type_0 == 2 as i32 {
                current_block = 10051727868454315572;
                break;
            } else {
                rhs -= *val.offset(k as isize) * (**((*P).col).offset(j as isize)).ub;
            }
        }
        k += 1;
        k;
    }
    match current_block {
        17860125682698302841 => {
            len = new_len;
            if !(len <= 1 as i32) {
                k = 1 as i32;
                while k <= len {
                    if *val.offset(k as isize) < 0.0f64 {
                        *ind.offset(k as isize) = -*ind.offset(k as isize);
                        *val.offset(k as isize) = -*val.offset(k as isize);
                        rhs += *val.offset(k as isize);
                    }
                    k += 1;
                    k;
                }
                rhs += 0.001f64 * (1.0f64 + fabs(rhs));
                p = 0 as i32;
                k = 1 as i32;
                while k <= len {
                    if p == 0 as i32 || *val.offset(p as isize) > *val.offset(k as isize)
                    {
                        p = k;
                    }
                    k += 1;
                    k;
                }
                q = 0 as i32;
                k = 1 as i32;
                while k <= len {
                    if k != p
                        && (q == 0 as i32
                            || *val.offset(q as isize) > *val.offset(k as isize))
                    {
                        q = k;
                    }
                    k += 1;
                    k;
                }
                (p != 0 as i32 && q != 0 as i32 && p != q
                    || {
                        glp_assert_(
                            b"p != 0 && q != 0 && p != q\0" as *const u8 as *const i8,
                            b"intopt/cfg1.c\0" as *const u8 as *const i8,
                            284 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                if *val.offset(p as isize) + *val.offset(q as isize) > rhs {
                    _glp_cfg_add_clique(G, len, ind as *const i32);
                } else {
                    p = 0 as i32;
                    k = 1 as i32;
                    while k <= len {
                        if p == 0 as i32
                            || *val.offset(p as isize) < *val.offset(k as isize)
                        {
                            p = k;
                        }
                        k += 1;
                        k;
                    }
                    q = 0 as i32;
                    k = 1 as i32;
                    while k <= len {
                        if k != p
                            && (q == 0 as i32
                                || *val.offset(q as isize) < *val.offset(k as isize))
                        {
                            q = k;
                        }
                        k += 1;
                        k;
                    }
                    (p != 0 as i32 && q != 0 as i32 && p != q
                        || {
                            glp_assert_(
                                b"p != 0 && q != 0 && p != q\0" as *const u8 as *const i8,
                                b"intopt/cfg1.c\0" as *const u8 as *const i8,
                                303 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                    if !(*val.offset(p as isize) + *val.offset(q as isize) <= rhs) {
                        (len >= 3 as i32
                            || {
                                glp_assert_(
                                    b"len >= 3\0" as *const u8 as *const i8,
                                    b"intopt/cfg1.c\0" as *const u8 as *const i8,
                                    310 as i32,
                                );
                                1 as i32 != 0
                            }) as i32;
                        k = 1 as i32;
                        while k <= len {
                            (*t.offset(k as isize)).ind = *ind.offset(k as isize);
                            (*t.offset(k as isize)).val = *val.offset(k as isize);
                            k += 1;
                            k;
                        }
                        qsort(
                            &mut *t.offset(1 as i32 as isize) as *mut term
                                as *mut libc::c_void,
                            len as size_t,
                            ::core::mem::size_of::<term>() as u64,
                            Some(
                                fcmp
                                    as unsafe extern "C" fn(
                                        *const libc::c_void,
                                        *const libc::c_void,
                                    ) -> i32,
                            ),
                        );
                        k = 1 as i32;
                        while k <= len {
                            *ind.offset(k as isize) = (*t.offset(k as isize)).ind;
                            *val.offset(k as isize) = (*t.offset(k as isize)).val;
                            k += 1;
                            k;
                        }
                        (*val.offset(1 as i32 as isize) + *val.offset(2 as i32 as isize)
                            > rhs
                            || {
                                glp_assert_(
                                    b"val[1] + val[2] > rhs\0" as *const u8 as *const i8,
                                    b"intopt/cfg1.c\0" as *const u8 as *const i8,
                                    324 as i32,
                                );
                                1 as i32 != 0
                            }) as i32;
                        (*val.offset((len - 1 as i32) as isize)
                            + *val.offset(len as isize) <= rhs
                            || {
                                glp_assert_(
                                    b"val[len-1] + val[len] <= rhs\0" as *const u8 as *const i8,
                                    b"intopt/cfg1.c\0" as *const u8 as *const i8,
                                    325 as i32,
                                );
                                1 as i32 != 0
                            }) as i32;
                        p = 2 as i32;
                        while p < len {
                            if *val.offset(p as isize)
                                + *val.offset((p + 1 as i32) as isize) <= rhs
                            {
                                break;
                            }
                            p += 1;
                            p;
                        }
                        (p < len
                            || {
                                glp_assert_(
                                    b"p < len\0" as *const u8 as *const i8,
                                    b"intopt/cfg1.c\0" as *const u8 as *const i8,
                                    331 as i32,
                                );
                                1 as i32 != 0
                            }) as i32;
                        _glp_cfg_add_clique(G, p, ind as *const i32);
                        k = 1 as i32;
                        while k <= p {
                            kk = p;
                            while kk <= len {
                                if k != kk
                                    && *val.offset(k as isize) + *val.offset(kk as isize) > rhs
                                {
                                    let mut iii: [i32; 3] = [0; 3];
                                    iii[1 as i32 as usize] = *ind.offset(k as isize);
                                    iii[2 as i32 as usize] = *ind.offset(kk as isize);
                                    _glp_cfg_add_clique(
                                        G,
                                        2 as i32,
                                        iii.as_mut_ptr() as *const i32,
                                    );
                                }
                                kk += 1;
                                kk;
                            }
                            k += 1;
                            k;
                        }
                    }
                }
            }
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn _glp_cfg_build_graph(mut P_: *mut libc::c_void) -> *mut CFG {
    let mut P: *mut glp_prob = P_ as *mut glp_prob;
    let mut m: i32 = (*P).m;
    let mut n: i32 = (*P).n;
    let mut G: *mut CFG = 0 as *mut CFG;
    let mut i: i32 = 0;
    let mut k: i32 = 0;
    let mut type_0: i32 = 0;
    let mut len: i32 = 0;
    let mut ind: *mut i32 = 0 as *mut i32;
    let mut val: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut t: *mut term = 0 as *mut term;
    G = _glp_cfg_create_graph(n, 2 as i32 * glp_get_num_bin(P));
    ind = glp_alloc(1 as i32 + n, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    val = glp_alloc(1 as i32 + n, ::core::mem::size_of::<libc::c_double>() as u64 as i32)
        as *mut libc::c_double;
    t = glp_alloc(1 as i32 + n, ::core::mem::size_of::<term>() as u64 as i32)
        as *mut term;
    i = 1 as i32;
    while i <= m {
        type_0 = (**((*P).row).offset(i as isize)).type_0;
        if type_0 == 2 as i32 || type_0 == 4 as i32 || type_0 == 5 as i32 {
            len = glp_get_mat_row(P, i, ind, val);
            k = 1 as i32;
            while k <= len {
                *val.offset(k as isize) = -*val.offset(k as isize);
                k += 1;
                k;
            }
            analyze_ineq(P, G, len, ind, val, -(**((*P).row).offset(i as isize)).lb, t);
        }
        if type_0 == 3 as i32 || type_0 == 4 as i32 || type_0 == 5 as i32 {
            len = glp_get_mat_row(P, i, ind, val);
            analyze_ineq(P, G, len, ind, val, (**((*P).row).offset(i as isize)).ub, t);
        }
        i += 1;
        i;
    }
    glp_free(ind as *mut libc::c_void);
    glp_free(val as *mut libc::c_void);
    glp_free(t as *mut libc::c_void);
    return G;
}
unsafe extern "C" fn build_subgraph(mut csa: *mut csa) {
    let mut P: *mut glp_prob = (*csa).P;
    let mut n: i32 = (*P).n;
    let mut G: *mut CFG = (*csa).G;
    let mut ind: *mut i32 = (*csa).ind;
    let mut pos: *mut i32 = (*G).pos;
    let mut neg: *mut i32 = (*G).neg;
    let mut nv: i32 = (*G).nv;
    let mut ref_0: *mut i32 = (*G).ref_0;
    let mut vtoi: *mut i32 = (*csa).vtoi;
    let mut itov: *mut i32 = (*csa).itov;
    let mut wgt: *mut libc::c_double = (*csa).wgt;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut v: i32 = 0;
    let mut w: i32 = 0;
    let mut nn: i32 = 0;
    let mut len: i32 = 0;
    let mut z: libc::c_double = 0.;
    let mut sum: libc::c_double = 0.;
    nn = 0 as i32;
    v = 1 as i32;
    while v <= nv {
        j = *ref_0.offset(v as isize);
        (1 as i32 <= j && j <= n
            || {
                glp_assert_(
                    b"1 <= j && j <= n\0" as *const u8 as *const i8,
                    b"intopt/cfg1.c\0" as *const u8 as *const i8,
                    477 as i32,
                );
                1 as i32 != 0
            }) as i32;
        if *pos.offset(j as isize) == v {
            z = (**((*P).col).offset(j as isize)).prim;
        } else if *neg.offset(j as isize) == v {
            z = 1.0f64 - (**((*P).col).offset(j as isize)).prim;
        } else {
            (v != v
                || {
                    glp_assert_(
                        b"v != v\0" as *const u8 as *const i8,
                        b"intopt/cfg1.c\0" as *const u8 as *const i8,
                        487 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        }
        if z < 0.001f64 {
            *vtoi.offset(v as isize) = 0 as i32;
        } else {
            sum = z;
            len = _glp_cfg_get_adjacent(G, v, ind);
            k = 1 as i32;
            while k <= len {
                w = *ind.offset(k as isize);
                (w != v
                    || {
                        glp_assert_(
                            b"w != v\0" as *const u8 as *const i8,
                            b"intopt/cfg1.c\0" as *const u8 as *const i8,
                            501 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                j = *ref_0.offset(w as isize);
                (1 as i32 <= j && j <= n
                    || {
                        glp_assert_(
                            b"1 <= j && j <= n\0" as *const u8 as *const i8,
                            b"intopt/cfg1.c\0" as *const u8 as *const i8,
                            504 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                if *pos.offset(j as isize) == w {
                    sum += (**((*P).col).offset(j as isize)).prim;
                } else if *neg.offset(j as isize) == w {
                    sum += 1.0f64 - (**((*P).col).offset(j as isize)).prim;
                } else {
                    (w != w
                        || {
                            glp_assert_(
                                b"w != w\0" as *const u8 as *const i8,
                                b"intopt/cfg1.c\0" as *const u8 as *const i8,
                                510 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                }
                k += 1;
                k;
            }
            if sum < 1.010f64 {
                *vtoi.offset(v as isize) = 0 as i32;
            } else {
                nn += 1;
                nn;
                *vtoi.offset(v as isize) = nn;
                *itov.offset(nn as isize) = v;
                *wgt.offset(nn as isize) = z;
            }
        }
        v += 1;
        v;
    }
    (*csa).nn = nn;
}
unsafe extern "C" fn sub_adjacent(
    mut csa: *mut csa,
    mut i: i32,
    mut adj: *mut i32,
) -> i32 {
    let mut G: *mut CFG = (*csa).G;
    let mut nv: i32 = (*G).nv;
    let mut ind: *mut i32 = (*csa).ind;
    let mut nn: i32 = (*csa).nn;
    let mut vtoi: *mut i32 = (*csa).vtoi;
    let mut itov: *mut i32 = (*csa).itov;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut v: i32 = 0;
    let mut w: i32 = 0;
    let mut len: i32 = 0;
    let mut len1: i32 = 0;
    (1 as i32 <= i && i <= nn
        || {
            glp_assert_(
                b"1 <= i && i <= nn\0" as *const u8 as *const i8,
                b"intopt/cfg1.c\0" as *const u8 as *const i8,
                541 as i32,
            );
            1 as i32 != 0
        }) as i32;
    v = *itov.offset(i as isize);
    len1 = _glp_cfg_get_adjacent(G, v, ind);
    len = 0 as i32;
    k = 1 as i32;
    while k <= len1 {
        w = *ind.offset(k as isize);
        (1 as i32 <= w && w <= nv && w != v
            || {
                glp_assert_(
                    b"1 <= w && w <= nv && w != v\0" as *const u8 as *const i8,
                    b"intopt/cfg1.c\0" as *const u8 as *const i8,
                    551 as i32,
                );
                1 as i32 != 0
            }) as i32;
        j = *vtoi.offset(w as isize);
        if j != 0 as i32 {
            (1 as i32 <= j && j <= nn && j != i
                || {
                    glp_assert_(
                        b"1 <= j && j <= nn && j != i\0" as *const u8 as *const i8,
                        b"intopt/cfg1.c\0" as *const u8 as *const i8,
                        555 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            len += 1;
            *adj.offset(len as isize) = j;
        }
        k += 1;
        k;
    }
    return len;
}
unsafe extern "C" fn find_clique(mut csa: *mut csa, mut c_ind: *mut i32) -> i32 {
    let mut nn: i32 = (*csa).nn;
    let mut wgt: *mut libc::c_double = (*csa).wgt;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut p: i32 = 0;
    let mut q: i32 = 0;
    let mut t: i32 = 0;
    let mut ne: i32 = 0;
    let mut nb: i32 = 0;
    let mut len: i32 = 0;
    let mut iwt: *mut i32 = 0 as *mut i32;
    let mut ind: *mut i32 = 0 as *mut i32;
    let mut a: *mut u8 = 0 as *mut u8;
    (nn >= 2 as i32
        || {
            glp_assert_(
                b"nn >= 2\0" as *const u8 as *const i8,
                b"intopt/cfg1.c\0" as *const u8 as *const i8,
                569 as i32,
            );
            1 as i32 != 0
        }) as i32;
    ind = glp_alloc(1 as i32 + nn, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    ne = nn * (nn - 1 as i32) / 2 as i32;
    nb = (ne + (8 as i32 - 1 as i32)) / 8 as i32;
    a = glp_alloc(nb, ::core::mem::size_of::<u8>() as u64 as i32) as *mut u8;
    memset(a as *mut libc::c_void, 0 as i32, nb as u64);
    p = 1 as i32;
    while p <= nn {
        len = sub_adjacent(csa, p, ind);
        k = 1 as i32;
        while k <= len {
            q = *ind.offset(k as isize);
            (1 as i32 <= q && q <= nn && q != p
                || {
                    glp_assert_(
                        b"1 <= q && q <= nn && q != p\0" as *const u8 as *const i8,
                        b"intopt/cfg1.c\0" as *const u8 as *const i8,
                        588 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            if p > q {
                i = p;
                j = q;
            } else {
                i = q;
                j = p;
            }
            t = (i - 1 as i32) * (i - 2 as i32) / 2 as i32 + (j - 1 as i32);
            let ref mut fresh0 = *a.offset((t / 8 as i32) as isize);
            *fresh0 = (*fresh0 as i32
                | ((1 as i32) << 8 as i32 - 1 as i32 - t % 8 as i32) as u8 as i32) as u8;
            k += 1;
            k;
        }
        p += 1;
        p;
    }
    iwt = ind;
    i = 1 as i32;
    while i <= nn {
        t = (1000.0f64 * *wgt.offset(i as isize) + 0.5f64) as i32;
        if t < 0 as i32 {
            t = 0 as i32;
        } else if t > 1000 as i32 {
            t = 1000 as i32;
        }
        *iwt.offset(i as isize) = t;
        i += 1;
        i;
    }
    len = _glp_wclique(nn, iwt as *const i32, a as *const u8, c_ind);
    glp_free(ind as *mut libc::c_void);
    glp_free(a as *mut libc::c_void);
    return len;
}
unsafe extern "C" fn func(
    mut info: *mut libc::c_void,
    mut i: i32,
    mut ind: *mut i32,
) -> i32 {
    let mut csa: *mut csa = info as *mut csa;
    (1 as i32 <= i && i <= (*csa).nn
        || {
            glp_assert_(
                b"1 <= i && i <= csa->nn\0" as *const u8 as *const i8,
                b"intopt/cfg1.c\0" as *const u8 as *const i8,
                625 as i32,
            );
            1 as i32 != 0
        }) as i32;
    return sub_adjacent(csa, i, ind);
}
unsafe extern "C" fn find_clique1(mut csa: *mut csa, mut c_ind: *mut i32) -> i32 {
    let mut nn: i32 = (*csa).nn;
    let mut wgt: *mut libc::c_double = (*csa).wgt;
    let mut len: i32 = 0;
    (nn >= 2 as i32
        || {
            glp_assert_(
                b"nn >= 2\0" as *const u8 as *const i8,
                b"intopt/cfg1.c\0" as *const u8 as *const i8,
                635 as i32,
            );
            1 as i32 != 0
        }) as i32;
    len = _glp_wclique1(
        nn,
        wgt as *const libc::c_double,
        Some(func as unsafe extern "C" fn(*mut libc::c_void, i32, *mut i32) -> i32),
        csa as *mut libc::c_void,
        c_ind,
    );
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_cfg_find_clique(
    mut P: *mut libc::c_void,
    mut G: *mut CFG,
    mut ind: *mut i32,
    mut sum_: *mut libc::c_double,
) -> i32 {
    let mut nv: i32 = (*G).nv;
    let mut csa: csa = csa {
        P: 0 as *mut glp_prob,
        G: 0 as *mut CFG,
        ind: 0 as *mut i32,
        nn: 0,
        vtoi: 0 as *mut i32,
        itov: 0 as *mut i32,
        wgt: 0 as *mut libc::c_double,
    };
    let mut i: i32 = 0;
    let mut k: i32 = 0;
    let mut len: i32 = 0;
    let mut sum: libc::c_double = 0.;
    csa.P = P as *mut glp_prob;
    csa.G = G;
    csa.ind = glp_alloc(1 as i32 + nv, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    csa.nn = -(1 as i32);
    csa.vtoi = glp_alloc(1 as i32 + nv, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    csa.itov = glp_alloc(1 as i32 + nv, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    csa.wgt = glp_alloc(
        1 as i32 + nv,
        ::core::mem::size_of::<libc::c_double>() as u64 as i32,
    ) as *mut libc::c_double;
    build_subgraph(&mut csa);
    if csa.nn < 2 as i32 {
        len = 0 as i32;
        sum = 0.0f64;
    } else {
        if csa.nn <= 50 as i32 {
            len = find_clique(&mut csa, ind);
        } else {
            len = find_clique1(&mut csa, ind);
        }
        if len < 2 as i32 {
            len = 0 as i32;
            sum = 0.0f64;
        } else {
            sum = 0.0f64;
            k = 1 as i32;
            while k <= len {
                i = *ind.offset(k as isize);
                (1 as i32 <= i && i <= csa.nn
                    || {
                        glp_assert_(
                            b"1 <= i && i <= csa.nn\0" as *const u8 as *const i8,
                            b"intopt/cfg1.c\0" as *const u8 as *const i8,
                            687 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                sum += *(csa.wgt).offset(i as isize);
                *ind.offset(k as isize) = *(csa.itov).offset(i as isize);
                k += 1;
                k;
            }
        }
    }
    glp_free(csa.ind as *mut libc::c_void);
    glp_free(csa.vtoi as *mut libc::c_void);
    glp_free(csa.itov as *mut libc::c_void);
    glp_free(csa.wgt as *mut libc::c_void);
    *sum_ = sum;
    return len;
}