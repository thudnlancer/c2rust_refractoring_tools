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
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
    fn glp_eval_tab_row(
        P: *mut glp_prob,
        k: i32,
        ind: *mut i32,
        val: *mut libc::c_double,
    ) -> i32;
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
pub unsafe extern "C" fn glp_gmi_cut(
    mut P: *mut glp_prob,
    mut j: i32,
    mut ind: *mut i32,
    mut val: *mut libc::c_double,
    mut phi: *mut libc::c_double,
) -> i32 {
    let mut current_block: u64;
    let mut m: i32 = (*P).m;
    let mut n: i32 = (*P).n;
    let mut row: *mut GLPROW = 0 as *mut GLPROW;
    let mut col: *mut GLPCOL = 0 as *mut GLPCOL;
    let mut aij: *mut GLPAIJ = 0 as *mut GLPAIJ;
    let mut i: i32 = 0;
    let mut k: i32 = 0;
    let mut len: i32 = 0;
    let mut kind: i32 = 0;
    let mut stat: i32 = 0;
    let mut lb: libc::c_double = 0.;
    let mut ub: libc::c_double = 0.;
    let mut alfa: libc::c_double = 0.;
    let mut beta: libc::c_double = 0.;
    let mut ksi: libc::c_double = 0.;
    let mut phi1: libc::c_double = 0.;
    let mut rhs: libc::c_double = 0.;
    if !((*P).m == 0 as i32 || (*P).valid != 0) {
        return -(1 as i32);
    }
    if !((*P).pbs_stat == 2 as i32 && (*P).dbs_stat == 2 as i32) {
        return -(2 as i32);
    }
    if !(1 as i32 <= j && j <= n) {
        return -(3 as i32);
    }
    col = *((*P).col).offset(j as isize);
    if (*col).kind != 2 as i32 {
        return -(4 as i32);
    }
    if (*col).type_0 == 5 as i32 || (*col).stat != 1 as i32 {
        return -(5 as i32);
    }
    if fabs((*col).prim - floor((*col).prim + 0.5f64)) < 0.001f64 {
        return -(6 as i32);
    }
    len = glp_eval_tab_row(P, m + j, ind, val);
    beta = (**((*P).col).offset(j as isize)).prim;
    k = 1 as i32;
    while k <= m + n {
        *phi.offset(k as isize) = 0.0f64;
        k += 1;
        k;
    }
    rhs = beta - floor(beta);
    j = 1 as i32;
    while j <= len {
        k = *ind.offset(j as isize);
        (1 as i32 <= k && k <= m + n
            || {
                glp_assert_(
                    b"1 <= k && k <= m+n\0" as *const u8 as *const i8,
                    b"intopt/gmicut.c\0" as *const u8 as *const i8,
                    150 as i32,
                );
                1 as i32 != 0
            }) as i32;
        if k <= m {
            row = *((*P).row).offset(k as isize);
            kind = 1 as i32;
            lb = (*row).lb;
            ub = (*row).ub;
            stat = (*row).stat;
        } else {
            col = *((*P).col).offset((k - m) as isize);
            kind = (*col).kind;
            lb = (*col).lb;
            ub = (*col).ub;
            stat = (*col).stat;
        }
        (stat != 1 as i32
            || {
                glp_assert_(
                    b"stat != GLP_BS\0" as *const u8 as *const i8,
                    b"intopt/gmicut.c\0" as *const u8 as *const i8,
                    170 as i32,
                );
                1 as i32 != 0
            }) as i32;
        ksi = *val.offset(j as isize);
        if fabs(ksi) > 1e+05f64 {
            return -(7 as i32);
        }
        if !(fabs(ksi) < 1e-10f64) {
            match stat {
                4 => return -(8 as i32),
                2 => {
                    alfa = -ksi;
                    current_block = 12199444798915819164;
                }
                3 => {
                    alfa = ksi;
                    current_block = 12199444798915819164;
                }
                5 => {
                    current_block = 11584701595673473500;
                }
                _ => {
                    (stat != stat
                        || {
                            glp_assert_(
                                b"stat != stat\0" as *const u8 as *const i8,
                                b"intopt/gmicut.c\0" as *const u8 as *const i8,
                                197 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                    current_block = 12199444798915819164;
                }
            }
            match current_block {
                11584701595673473500 => {}
                _ => {
                    match kind {
                        2 => {
                            if fabs(alfa - floor(alfa + 0.5f64)) < 1e-10f64 {
                                current_block = 11584701595673473500;
                            } else {
                                if alfa - floor(alfa) <= beta - floor(beta) {
                                    phi1 = alfa - floor(alfa);
                                } else {
                                    phi1 = (beta - floor(beta))
                                        / (1.0f64 - (beta - floor(beta)))
                                        * (1.0f64 - (alfa - floor(alfa)));
                                }
                                current_block = 7420279277351916581;
                            }
                        }
                        1 => {
                            if alfa >= 0.0f64 {
                                phi1 = alfa;
                            } else {
                                phi1 = (beta - floor(beta))
                                    / (1.0f64 - (beta - floor(beta))) * -alfa;
                            }
                            current_block = 7420279277351916581;
                        }
                        _ => {
                            (kind != kind
                                || {
                                    glp_assert_(
                                        b"kind != kind\0" as *const u8 as *const i8,
                                        b"intopt/gmicut.c\0" as *const u8 as *const i8,
                                        220 as i32,
                                    );
                                    1 as i32 != 0
                                }) as i32;
                            current_block = 7420279277351916581;
                        }
                    }
                    match current_block {
                        11584701595673473500 => {}
                        _ => {
                            match stat {
                                2 => {
                                    *phi.offset(k as isize) = phi1;
                                    rhs += phi1 * lb;
                                }
                                3 => {
                                    *phi.offset(k as isize) = -phi1;
                                    rhs -= phi1 * ub;
                                }
                                _ => {
                                    (stat != stat
                                        || {
                                            glp_assert_(
                                                b"stat != stat\0" as *const u8 as *const i8,
                                                b"intopt/gmicut.c\0" as *const u8 as *const i8,
                                                236 as i32,
                                            );
                                            1 as i32 != 0
                                        }) as i32;
                                }
                            }
                        }
                    }
                }
            }
        }
        j += 1;
        j;
    }
    i = 1 as i32;
    while i <= m {
        if !(fabs(*phi.offset(i as isize)) < 1e-10f64) {
            row = *((*P).row).offset(i as isize);
            ((*row).type_0 != 5 as i32
                || {
                    glp_assert_(
                        b"row->type != GLP_FX\0" as *const u8 as *const i8,
                        b"intopt/gmicut.c\0" as *const u8 as *const i8,
                        252 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            aij = (*row).ptr;
            while !aij.is_null() {
                *phi.offset((m + (*(*aij).col).j) as isize)
                    += *phi.offset(i as isize) * (*aij).val;
                aij = (*aij).r_next;
            }
        }
        i += 1;
        i;
    }
    len = 0 as i32;
    j = 1 as i32;
    while j <= n {
        if !(fabs(*phi.offset((m + j) as isize)) < 1e-10f64) {
            col = *((*P).col).offset(j as isize);
            if (*col).type_0 == 5 as i32 {
                rhs -= *phi.offset((m + j) as isize) * (*col).lb;
            } else {
                len += 1;
                len;
                *ind.offset(len as isize) = j;
                *val.offset(len as isize) = *phi.offset((m + j) as isize);
            }
        }
        j += 1;
        j;
    }
    if fabs(rhs) < 1e-12f64 {
        rhs = 0.0f64;
    }
    *ind.offset(0 as i32 as isize) = 0 as i32;
    *val.offset(0 as i32 as isize) = rhs;
    return len;
}