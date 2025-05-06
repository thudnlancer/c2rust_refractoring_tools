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
    pub type AVL;
    pub type AVLNODE;
    pub type BFD;
    pub type glp_tree;
    fn _glp_cfg_find_clique(
        P: *mut libc::c_void,
        G: *mut CFG,
        ind: *mut i32,
        sum: *mut libc::c_double,
    ) -> i32;
    fn _glp_cfg_expand_clique(G: *mut CFG, c_len: i32, c_ind: *mut i32) -> i32;
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
}
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
pub unsafe extern "C" fn glp_clq_cut(
    mut P: *mut glp_prob,
    mut G: *mut glp_cfg,
    mut ind: *mut i32,
    mut val: *mut libc::c_double,
) -> i32 {
    let mut n: i32 = (*P).n;
    let mut pos: *mut i32 = (*G).pos;
    let mut neg: *mut i32 = (*G).neg;
    let mut nv: i32 = (*G).nv;
    let mut ref_0: *mut i32 = (*G).ref_0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut v: i32 = 0;
    let mut len: i32 = 0;
    let mut rhs: libc::c_double = 0.;
    let mut sum: libc::c_double = 0.;
    ((*G).n == n
        || {
            glp_assert_(
                b"G->n == n\0" as *const u8 as *const i8,
                b"intopt/clqcut.c\0" as *const u8 as *const i8,
                65 as i32,
            );
            1 as i32 != 0
        }) as i32;
    len = _glp_cfg_find_clique(P as *mut libc::c_void, G, ind, &mut sum);
    if sum < 1.07f64 {
        return 0 as i32;
    }
    len = _glp_cfg_expand_clique(G, len, ind);
    rhs = 1.0f64;
    j = 1 as i32;
    while j <= n {
        *val.offset(j as isize) = 0.0f64;
        j += 1;
        j;
    }
    k = 1 as i32;
    while k <= len {
        v = *ind.offset(k as isize);
        (1 as i32 <= v && v <= nv
            || {
                glp_assert_(
                    b"1 <= v && v <= nv\0" as *const u8 as *const i8,
                    b"intopt/clqcut.c\0" as *const u8 as *const i8,
                    89 as i32,
                );
                1 as i32 != 0
            }) as i32;
        j = *ref_0.offset(v as isize);
        (1 as i32 <= j && j <= n
            || {
                glp_assert_(
                    b"1 <= j && j <= n\0" as *const u8 as *const i8,
                    b"intopt/clqcut.c\0" as *const u8 as *const i8,
                    92 as i32,
                );
                1 as i32 != 0
            }) as i32;
        if *pos.offset(j as isize) == v {
            if (**((*P).col).offset(j as isize)).type_0 == 5 as i32 {
                rhs -= (**((*P).col).offset(j as isize)).prim;
            } else {
                *val.offset(j as isize) += 1.0f64;
            }
        } else if *neg.offset(j as isize) == v {
            if (**((*P).col).offset(j as isize)).type_0 == 5 as i32 {
                rhs -= 1.0f64 - (**((*P).col).offset(j as isize)).prim;
            } else {
                *val.offset(j as isize) -= 1.0f64;
                rhs -= 1.0f64;
            }
        } else {
            (v != v
                || {
                    glp_assert_(
                        b"v != v\0" as *const u8 as *const i8,
                        b"intopt/clqcut.c\0" as *const u8 as *const i8,
                        117 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        }
        k += 1;
        k;
    }
    len = 0 as i32;
    j = 1 as i32;
    while j <= n {
        if *val.offset(j as isize) != 0.0f64 {
            len += 1;
            len;
            *ind.offset(len as isize) = j;
            *val.offset(len as isize) = *val.offset(j as isize);
        }
        j += 1;
        j;
    }
    *ind.offset(0 as i32 as isize) = 0 as i32;
    *val.offset(0 as i32 as isize) = rhs;
    return len;
}