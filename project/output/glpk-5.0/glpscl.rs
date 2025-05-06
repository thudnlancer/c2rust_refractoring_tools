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
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn glp_printf(fmt: *const i8, _: ...);
    fn glp_error_(file: *const i8, line: i32) -> glp_errfunc;
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
    fn _glp_round2n(x: libc::c_double) -> libc::c_double;
    fn glp_set_rii(P: *mut glp_prob, i: i32, rii: libc::c_double);
    fn glp_set_sjj(P: *mut glp_prob, j: i32, sjj: libc::c_double);
    fn glp_get_rii(P: *mut glp_prob, i: i32) -> libc::c_double;
    fn glp_get_sjj(P: *mut glp_prob, j: i32) -> libc::c_double;
    fn glp_unscale_prob(P: *mut glp_prob);
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
unsafe extern "C" fn min_row_aij(
    mut lp: *mut glp_prob,
    mut i: i32,
    mut scaled: i32,
) -> libc::c_double {
    let mut aij: *mut GLPAIJ = 0 as *mut GLPAIJ;
    let mut min_aij: libc::c_double = 0.;
    let mut temp: libc::c_double = 0.;
    (1 as i32 <= i && i <= (*lp).m
        || {
            glp_assert_(
                b"1 <= i && i <= lp->m\0" as *const u8 as *const i8,
                b"draft/glpscl.c\0" as *const u8 as *const i8,
                40 as i32,
            );
            1 as i32 != 0
        }) as i32;
    min_aij = 1.0f64;
    aij = (**((*lp).row).offset(i as isize)).ptr;
    while !aij.is_null() {
        temp = fabs((*aij).val);
        if scaled != 0 {
            temp *= (*(*aij).row).rii * (*(*aij).col).sjj;
        }
        if ((*aij).r_prev).is_null() || min_aij > temp {
            min_aij = temp;
        }
        aij = (*aij).r_next;
    }
    return min_aij;
}
unsafe extern "C" fn max_row_aij(
    mut lp: *mut glp_prob,
    mut i: i32,
    mut scaled: i32,
) -> libc::c_double {
    let mut aij: *mut GLPAIJ = 0 as *mut GLPAIJ;
    let mut max_aij: libc::c_double = 0.;
    let mut temp: libc::c_double = 0.;
    (1 as i32 <= i && i <= (*lp).m
        || {
            glp_assert_(
                b"1 <= i && i <= lp->m\0" as *const u8 as *const i8,
                b"draft/glpscl.c\0" as *const u8 as *const i8,
                65 as i32,
            );
            1 as i32 != 0
        }) as i32;
    max_aij = 1.0f64;
    aij = (**((*lp).row).offset(i as isize)).ptr;
    while !aij.is_null() {
        temp = fabs((*aij).val);
        if scaled != 0 {
            temp *= (*(*aij).row).rii * (*(*aij).col).sjj;
        }
        if ((*aij).r_prev).is_null() || max_aij < temp {
            max_aij = temp;
        }
        aij = (*aij).r_next;
    }
    return max_aij;
}
unsafe extern "C" fn min_col_aij(
    mut lp: *mut glp_prob,
    mut j: i32,
    mut scaled: i32,
) -> libc::c_double {
    let mut aij: *mut GLPAIJ = 0 as *mut GLPAIJ;
    let mut min_aij: libc::c_double = 0.;
    let mut temp: libc::c_double = 0.;
    (1 as i32 <= j && j <= (*lp).n
        || {
            glp_assert_(
                b"1 <= j && j <= lp->n\0" as *const u8 as *const i8,
                b"draft/glpscl.c\0" as *const u8 as *const i8,
                90 as i32,
            );
            1 as i32 != 0
        }) as i32;
    min_aij = 1.0f64;
    aij = (**((*lp).col).offset(j as isize)).ptr;
    while !aij.is_null() {
        temp = fabs((*aij).val);
        if scaled != 0 {
            temp *= (*(*aij).row).rii * (*(*aij).col).sjj;
        }
        if ((*aij).c_prev).is_null() || min_aij > temp {
            min_aij = temp;
        }
        aij = (*aij).c_next;
    }
    return min_aij;
}
unsafe extern "C" fn max_col_aij(
    mut lp: *mut glp_prob,
    mut j: i32,
    mut scaled: i32,
) -> libc::c_double {
    let mut aij: *mut GLPAIJ = 0 as *mut GLPAIJ;
    let mut max_aij: libc::c_double = 0.;
    let mut temp: libc::c_double = 0.;
    (1 as i32 <= j && j <= (*lp).n
        || {
            glp_assert_(
                b"1 <= j && j <= lp->n\0" as *const u8 as *const i8,
                b"draft/glpscl.c\0" as *const u8 as *const i8,
                115 as i32,
            );
            1 as i32 != 0
        }) as i32;
    max_aij = 1.0f64;
    aij = (**((*lp).col).offset(j as isize)).ptr;
    while !aij.is_null() {
        temp = fabs((*aij).val);
        if scaled != 0 {
            temp *= (*(*aij).row).rii * (*(*aij).col).sjj;
        }
        if ((*aij).c_prev).is_null() || max_aij < temp {
            max_aij = temp;
        }
        aij = (*aij).c_next;
    }
    return max_aij;
}
unsafe extern "C" fn min_mat_aij(
    mut lp: *mut glp_prob,
    mut scaled: i32,
) -> libc::c_double {
    let mut i: i32 = 0;
    let mut min_aij: libc::c_double = 0.;
    let mut temp: libc::c_double = 0.;
    min_aij = 1.0f64;
    i = 1 as i32;
    while i <= (*lp).m {
        temp = min_row_aij(lp, i, scaled);
        if i == 1 as i32 || min_aij > temp {
            min_aij = temp;
        }
        i += 1;
        i;
    }
    return min_aij;
}
unsafe extern "C" fn max_mat_aij(
    mut lp: *mut glp_prob,
    mut scaled: i32,
) -> libc::c_double {
    let mut i: i32 = 0;
    let mut max_aij: libc::c_double = 0.;
    let mut temp: libc::c_double = 0.;
    max_aij = 1.0f64;
    i = 1 as i32;
    while i <= (*lp).m {
        temp = max_row_aij(lp, i, scaled);
        if i == 1 as i32 || max_aij < temp {
            max_aij = temp;
        }
        i += 1;
        i;
    }
    return max_aij;
}
unsafe extern "C" fn eq_scaling(mut lp: *mut glp_prob, mut flag: i32) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut pass: i32 = 0;
    let mut temp: libc::c_double = 0.;
    (flag == 0 as i32 || flag == 1 as i32
        || {
            glp_assert_(
                b"flag == 0 || flag == 1\0" as *const u8 as *const i8,
                b"draft/glpscl.c\0" as *const u8 as *const i8,
                202 as i32,
            );
            1 as i32 != 0
        }) as i32;
    pass = 0 as i32;
    while pass <= 1 as i32 {
        if pass == flag {
            i = 1 as i32;
            while i <= (*lp).m {
                temp = max_row_aij(lp, i, 1 as i32);
                glp_set_rii(lp, i, glp_get_rii(lp, i) / temp);
                i += 1;
                i;
            }
        } else {
            j = 1 as i32;
            while j <= (*lp).n {
                temp = max_col_aij(lp, j, 1 as i32);
                glp_set_sjj(lp, j, glp_get_sjj(lp, j) / temp);
                j += 1;
                j;
            }
        }
        pass += 1;
        pass;
    }
}
unsafe extern "C" fn gm_scaling(mut lp: *mut glp_prob, mut flag: i32) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut pass: i32 = 0;
    let mut temp: libc::c_double = 0.;
    (flag == 0 as i32 || flag == 1 as i32
        || {
            glp_assert_(
                b"flag == 0 || flag == 1\0" as *const u8 as *const i8,
                b"draft/glpscl.c\0" as *const u8 as *const i8,
                258 as i32,
            );
            1 as i32 != 0
        }) as i32;
    pass = 0 as i32;
    while pass <= 1 as i32 {
        if pass == flag {
            i = 1 as i32;
            while i <= (*lp).m {
                temp = min_row_aij(lp, i, 1 as i32) * max_row_aij(lp, i, 1 as i32);
                glp_set_rii(lp, i, glp_get_rii(lp, i) / sqrt(temp));
                i += 1;
                i;
            }
        } else {
            j = 1 as i32;
            while j <= (*lp).n {
                temp = min_col_aij(lp, j, 1 as i32) * max_col_aij(lp, j, 1 as i32);
                glp_set_sjj(lp, j, glp_get_sjj(lp, j) / sqrt(temp));
                j += 1;
                j;
            }
        }
        pass += 1;
        pass;
    }
}
unsafe extern "C" fn max_row_ratio(mut lp: *mut glp_prob) -> libc::c_double {
    let mut i: i32 = 0;
    let mut ratio: libc::c_double = 0.;
    let mut temp: libc::c_double = 0.;
    ratio = 1.0f64;
    i = 1 as i32;
    while i <= (*lp).m {
        temp = max_row_aij(lp, i, 1 as i32) / min_row_aij(lp, i, 1 as i32);
        if i == 1 as i32 || ratio < temp {
            ratio = temp;
        }
        i += 1;
        i;
    }
    return ratio;
}
unsafe extern "C" fn max_col_ratio(mut lp: *mut glp_prob) -> libc::c_double {
    let mut j: i32 = 0;
    let mut ratio: libc::c_double = 0.;
    let mut temp: libc::c_double = 0.;
    ratio = 1.0f64;
    j = 1 as i32;
    while j <= (*lp).n {
        temp = max_col_aij(lp, j, 1 as i32) / min_col_aij(lp, j, 1 as i32);
        if j == 1 as i32 || ratio < temp {
            ratio = temp;
        }
        j += 1;
        j;
    }
    return ratio;
}
unsafe extern "C" fn gm_iterate(
    mut lp: *mut glp_prob,
    mut it_max: i32,
    mut tau: libc::c_double,
) {
    let mut k: i32 = 0;
    let mut flag: i32 = 0;
    let mut ratio: libc::c_double = 0.0f64;
    let mut r_old: libc::c_double = 0.;
    flag = (max_row_ratio(lp) > max_col_ratio(lp)) as i32;
    k = 1 as i32;
    while k <= it_max {
        r_old = ratio;
        ratio = max_mat_aij(lp, 1 as i32) / min_mat_aij(lp, 1 as i32);
        if k > 1 as i32 && ratio > tau * r_old {
            break;
        }
        gm_scaling(lp, flag);
        k += 1;
        k;
    }
}
unsafe extern "C" fn scale_prob(mut lp: *mut glp_prob, mut flags: i32) {
    let mut current_block: u64;
    static mut fmt: *const i8 = b"%s: min|aij| = %10.3e  max|aij| = %10.3e  ratio = %10.3e\n\0"
        as *const u8 as *const i8;
    let mut min_aij: libc::c_double = 0.;
    let mut max_aij: libc::c_double = 0.;
    let mut ratio: libc::c_double = 0.;
    glp_printf(b"Scaling...\n\0" as *const u8 as *const i8);
    glp_unscale_prob(lp);
    min_aij = min_mat_aij(lp, 1 as i32);
    max_aij = max_mat_aij(lp, 1 as i32);
    ratio = max_aij / min_aij;
    glp_printf(fmt, b" A\0" as *const u8 as *const i8, min_aij, max_aij, ratio);
    if min_aij >= 0.10f64 && max_aij <= 10.0f64 {
        glp_printf(b"Problem data seem to be well scaled\n\0" as *const u8 as *const i8);
        if flags & 0x40 as i32 != 0 {
            current_block = 1791590821008025756;
        } else {
            current_block = 11006700562992250127;
        }
    } else {
        current_block = 11006700562992250127;
    }
    match current_block {
        11006700562992250127 => {
            if flags & 0x1 as i32 != 0 {
                gm_iterate(lp, 15 as i32, 0.90f64);
                min_aij = min_mat_aij(lp, 1 as i32);
                max_aij = max_mat_aij(lp, 1 as i32);
                ratio = max_aij / min_aij;
                glp_printf(
                    fmt,
                    b"GM\0" as *const u8 as *const i8,
                    min_aij,
                    max_aij,
                    ratio,
                );
            }
            if flags & 0x10 as i32 != 0 {
                eq_scaling(lp, (max_row_ratio(lp) > max_col_ratio(lp)) as i32);
                min_aij = min_mat_aij(lp, 1 as i32);
                max_aij = max_mat_aij(lp, 1 as i32);
                ratio = max_aij / min_aij;
                glp_printf(
                    fmt,
                    b"EQ\0" as *const u8 as *const i8,
                    min_aij,
                    max_aij,
                    ratio,
                );
            }
            if flags & 0x20 as i32 != 0 {
                let mut i: i32 = 0;
                let mut j: i32 = 0;
                i = 1 as i32;
                while i <= (*lp).m {
                    glp_set_rii(lp, i, _glp_round2n(glp_get_rii(lp, i)));
                    i += 1;
                    i;
                }
                j = 1 as i32;
                while j <= (*lp).n {
                    glp_set_sjj(lp, j, _glp_round2n(glp_get_sjj(lp, j)));
                    j += 1;
                    j;
                }
                min_aij = min_mat_aij(lp, 1 as i32);
                max_aij = max_mat_aij(lp, 1 as i32);
                ratio = max_aij / min_aij;
                glp_printf(
                    fmt,
                    b"2N\0" as *const u8 as *const i8,
                    min_aij,
                    max_aij,
                    ratio,
                );
            }
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn glp_scale_prob(mut lp: *mut glp_prob, mut flags: i32) {
    if flags & !(0x1 as i32 | 0x10 as i32 | 0x20 as i32 | 0x40 as i32 | 0x80 as i32) != 0
    {
        (glp_error_(b"draft/glpscl.c\0" as *const u8 as *const i8, 467 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_scale_prob: flags = 0x%02X; invalid scaling options\n\0" as *const u8
                as *const i8,
            flags,
        );
    }
    if flags & 0x80 as i32 != 0 {
        flags = 0x1 as i32 | 0x10 as i32 | 0x40 as i32;
    }
    scale_prob(lp, flags);
}