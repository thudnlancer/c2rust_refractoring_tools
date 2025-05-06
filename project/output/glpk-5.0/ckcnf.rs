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
#[no_mangle]
pub unsafe extern "C" fn glp_check_cnfsat(mut P: *mut glp_prob) -> i32 {
    let mut m: i32 = (*P).m;
    let mut n: i32 = (*P).n;
    let mut row: *mut GLPROW = 0 as *mut GLPROW;
    let mut col: *mut GLPCOL = 0 as *mut GLPCOL;
    let mut aij: *mut GLPAIJ = 0 as *mut GLPAIJ;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut neg: i32 = 0;
    j = 1 as i32;
    while j <= n {
        col = *((*P).col).offset(j as isize);
        if !((*col).kind == 2 as i32 && (*col).type_0 == 4 as i32 && (*col).lb == 0.0f64
            && (*col).ub == 1.0f64)
        {
            return 1 as i32;
        }
        j += 1;
        j;
    }
    if (*P).c0 != 0.0f64 {
        return 2 as i32;
    }
    j = 1 as i32;
    while j <= n {
        col = *((*P).col).offset(j as isize);
        if (*col).coef != 0.0f64 {
            return 3 as i32;
        }
        j += 1;
        j;
    }
    i = 1 as i32;
    while i <= m {
        row = *((*P).row).offset(i as isize);
        if (*row).type_0 != 2 as i32 {
            return 4 as i32;
        }
        neg = 0 as i32;
        aij = (*row).ptr;
        while !aij.is_null() {
            if !((*aij).val == 1.0f64) {
                if (*aij).val == -1.0f64 {
                    neg += 1;
                    neg;
                } else {
                    return 5 as i32
                }
            }
            aij = (*aij).r_next;
        }
        if (*row).lb != (1 as i32 - neg) as libc::c_double {
            return 6 as i32;
        }
        i += 1;
        i;
    }
    return 0 as i32;
}