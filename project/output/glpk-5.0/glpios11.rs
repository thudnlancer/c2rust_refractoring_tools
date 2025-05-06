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
    pub type glp_cfg;
    pub type glp_mir;
    pub type glp_cov;
    pub type DMP;
    fn glp_add_rows(P: *mut glp_prob, nrs: i32) -> i32;
    fn glp_set_row_name(P: *mut glp_prob, i: i32, name: *const i8);
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
    fn glp_transform_row(
        P: *mut glp_prob,
        len: i32,
        ind: *mut i32,
        val: *mut libc::c_double,
    ) -> i32;
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
    fn glp_alloc(n: i32, size: i32) -> *mut libc::c_void;
    fn glp_free(ptr: *mut libc::c_void);
    fn _glp_analyze_row(
        P: *mut glp_prob,
        len: i32,
        ind: *const i32,
        val: *const libc::c_double,
        type_0: i32,
        rhs: libc::c_double,
        eps: libc::c_double,
        _piv: *mut i32,
        _x: *mut libc::c_double,
        _dx: *mut libc::c_double,
        _y: *mut libc::c_double,
        _dy: *mut libc::c_double,
        _dz: *mut libc::c_double,
    ) -> i32;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
}
pub type size_t = u64;
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
pub struct glp_tree {
    pub magic: i32,
    pub pool: *mut DMP,
    pub n: i32,
    pub orig_m: i32,
    pub orig_type: *mut u8,
    pub orig_lb: *mut libc::c_double,
    pub orig_ub: *mut libc::c_double,
    pub orig_stat: *mut u8,
    pub orig_prim: *mut libc::c_double,
    pub orig_dual: *mut libc::c_double,
    pub orig_obj: libc::c_double,
    pub nslots: i32,
    pub avail: i32,
    pub slot: *mut IOSLOT,
    pub head: *mut IOSNPD,
    pub tail: *mut IOSNPD,
    pub a_cnt: i32,
    pub n_cnt: i32,
    pub t_cnt: i32,
    pub root_m: i32,
    pub root_type: *mut u8,
    pub root_lb: *mut libc::c_double,
    pub root_ub: *mut libc::c_double,
    pub root_stat: *mut u8,
    pub curr: *mut IOSNPD,
    pub mip: *mut glp_prob,
    pub non_int: *mut u8,
    pub pred_m: i32,
    pub pred_max: i32,
    pub pred_type: *mut u8,
    pub pred_lb: *mut libc::c_double,
    pub pred_ub: *mut libc::c_double,
    pub pred_stat: *mut u8,
    pub local: *mut IOSPOOL,
    pub cov_gen: *mut glp_cov,
    pub mir_gen: *mut glp_mir,
    pub clq_gen: *mut glp_cfg,
    pub pcost: *mut libc::c_void,
    pub iwrk: *mut i32,
    pub dwrk: *mut libc::c_double,
    pub parm: *const glp_iocp,
    pub tm_beg: libc::c_double,
    pub tm_lag: libc::c_double,
    pub sol_cnt: i32,
    pub P: *mut libc::c_void,
    pub npp: *mut libc::c_void,
    pub save_sol: *const i8,
    pub save_cnt: i32,
    pub reason: i32,
    pub stop: i32,
    pub next_p: i32,
    pub reopt: i32,
    pub reinv: i32,
    pub br_var: i32,
    pub br_sel: i32,
    pub child: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glp_iocp {
    pub msg_lev: i32,
    pub br_tech: i32,
    pub bt_tech: i32,
    pub tol_int: libc::c_double,
    pub tol_obj: libc::c_double,
    pub tm_lim: i32,
    pub out_frq: i32,
    pub out_dly: i32,
    pub cb_func: Option<unsafe extern "C" fn(*mut glp_tree, *mut libc::c_void) -> ()>,
    pub cb_info: *mut libc::c_void,
    pub cb_size: i32,
    pub pp_tech: i32,
    pub mip_gap: libc::c_double,
    pub mir_cuts: i32,
    pub gmi_cuts: i32,
    pub cov_cuts: i32,
    pub clq_cuts: i32,
    pub presolve: i32,
    pub binarize: i32,
    pub fp_heur: i32,
    pub ps_heur: i32,
    pub ps_tm_lim: i32,
    pub sr_heur: i32,
    pub use_sol: i32,
    pub save_sol: *const i8,
    pub alien: i32,
    pub flip: i32,
    pub foo_bar: [libc::c_double; 23],
}
pub type IOSPOOL = glp_prob;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IOSNPD {
    pub p: i32,
    pub up: *mut IOSNPD,
    pub level: i32,
    pub count: i32,
    pub b_ptr: *mut IOSBND,
    pub s_ptr: *mut IOSTAT,
    pub r_ptr: *mut IOSROW,
    pub solved: i32,
    pub lp_obj: libc::c_double,
    pub bound: libc::c_double,
    pub ii_cnt: i32,
    pub ii_sum: libc::c_double,
    pub changed: i32,
    pub br_var: i32,
    pub br_val: libc::c_double,
    pub data: *mut libc::c_void,
    pub temp: *mut IOSNPD,
    pub prev: *mut IOSNPD,
    pub next: *mut IOSNPD,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IOSROW {
    pub name: *mut i8,
    pub origin: u8,
    pub klass: u8,
    pub type_0: u8,
    pub lb: libc::c_double,
    pub ub: libc::c_double,
    pub ptr: *mut IOSAIJ,
    pub rii: libc::c_double,
    pub stat: u8,
    pub next: *mut IOSROW,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IOSAIJ {
    pub j: i32,
    pub val: libc::c_double,
    pub next: *mut IOSAIJ,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IOSTAT {
    pub k: i32,
    pub stat: u8,
    pub next: *mut IOSTAT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IOSBND {
    pub k: i32,
    pub type_0: u8,
    pub lb: libc::c_double,
    pub ub: libc::c_double,
    pub next: *mut IOSBND,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IOSLOT {
    pub node: *mut IOSNPD,
    pub next: i32,
}
pub type __compar_fn_t = Option<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32,
>;
pub type IOSCUT = GLPROW;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct info {
    pub cut: *mut IOSCUT,
    pub flag: i8,
    pub eff: libc::c_double,
    pub deg: libc::c_double,
}
unsafe extern "C" fn fcmp(
    mut arg1: *const libc::c_void,
    mut arg2: *const libc::c_void,
) -> i32 {
    let mut info1: *const info = arg1 as *const info;
    let mut info2: *const info = arg2 as *const info;
    if (*info1).deg == 0.0f64 && (*info2).deg == 0.0f64 {
        if (*info1).eff > (*info2).eff {
            return -(1 as i32);
        }
        if (*info1).eff < (*info2).eff {
            return 1 as i32;
        }
    } else {
        if (*info1).deg > (*info2).deg {
            return -(1 as i32);
        }
        if (*info1).deg < (*info2).deg {
            return 1 as i32;
        }
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_ios_process_cuts(mut T: *mut glp_tree) {
    let mut pool: *mut IOSPOOL = 0 as *mut IOSPOOL;
    let mut cut: *mut IOSCUT = 0 as *mut IOSCUT;
    let mut aij: *mut GLPAIJ = 0 as *mut GLPAIJ;
    let mut info: *mut info = 0 as *mut info;
    let mut k: i32 = 0;
    let mut kk: i32 = 0;
    let mut max_cuts: i32 = 0;
    let mut len: i32 = 0;
    let mut ret: i32 = 0;
    let mut ind: *mut i32 = 0 as *mut i32;
    let mut val: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut work: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut rhs: libc::c_double = 0.;
    (!((*T).curr).is_null()
        || {
            glp_assert_(
                b"T->curr != NULL\0" as *const u8 as *const i8,
                b"draft/glpios11.c\0" as *const u8 as *const i8,
                90 as i32,
            );
            1 as i32 != 0
        }) as i32;
    pool = (*T).local;
    (!pool.is_null()
        || {
            glp_assert_(
                b"pool != NULL\0" as *const u8 as *const i8,
                b"draft/glpios11.c\0" as *const u8 as *const i8,
                93 as i32,
            );
            1 as i32 != 0
        }) as i32;
    ((*pool).m > 0 as i32
        || {
            glp_assert_(
                b"pool->m > 0\0" as *const u8 as *const i8,
                b"draft/glpios11.c\0" as *const u8 as *const i8,
                94 as i32,
            );
            1 as i32 != 0
        }) as i32;
    info = glp_alloc(1 as i32 + (*pool).m, ::core::mem::size_of::<info>() as u64 as i32)
        as *mut info;
    ind = glp_alloc(1 as i32 + (*T).n, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    val = glp_alloc(
        1 as i32 + (*T).n,
        ::core::mem::size_of::<libc::c_double>() as u64 as i32,
    ) as *mut libc::c_double;
    work = glp_alloc(
        1 as i32 + (*T).n,
        ::core::mem::size_of::<libc::c_double>() as u64 as i32,
    ) as *mut libc::c_double;
    k = 1 as i32;
    while k <= (*T).n {
        *work.offset(k as isize) = 0.0f64;
        k += 1;
        k;
    }
    k = 1 as i32;
    while k <= (*pool).m {
        let ref mut fresh0 = (*info.offset(k as isize)).cut;
        *fresh0 = *((*pool).row).offset(k as isize);
        (*info.offset(k as isize)).flag = 0 as i32 as i8;
        k += 1;
        k;
    }
    k = 1 as i32;
    while k <= (*pool).m {
        let mut temp: libc::c_double = 0.;
        let mut dy: libc::c_double = 0.;
        let mut dz: libc::c_double = 0.;
        cut = (*info.offset(k as isize)).cut;
        len = 0 as i32;
        temp = 0.0f64;
        aij = (*cut).ptr;
        while !aij.is_null() {
            (1 as i32 <= (*(*aij).col).j && (*(*aij).col).j <= (*T).n
                || {
                    glp_assert_(
                        b"1 <= aij->col->j && aij->col->j <= T->n\0" as *const u8
                            as *const i8,
                        b"draft/glpios11.c\0" as *const u8 as *const i8,
                        112 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            len += 1;
            len;
            *ind.offset(len as isize) = (*(*aij).col).j;
            *val.offset(len as isize) = (*aij).val;
            temp += (*aij).val * (*aij).val;
            aij = (*aij).r_next;
        }
        if temp < 2.2204460492503131e-16f64 * 2.2204460492503131e-16f64 {
            temp = 2.2204460492503131e-16f64;
        }
        len = glp_transform_row((*T).mip, len, ind, val);
        match (*cut).type_0 {
            2 => {
                rhs = (*cut).lb;
            }
            3 => {
                rhs = (*cut).ub;
            }
            _ => {
                (cut != cut
                    || {
                        glp_assert_(
                            b"cut != cut\0" as *const u8 as *const i8,
                            b"draft/glpios11.c\0" as *const u8 as *const i8,
                            126 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
            }
        }
        ret = _glp_analyze_row(
            (*T).mip,
            len,
            ind as *const i32,
            val as *const libc::c_double,
            (*cut).type_0,
            rhs,
            1e-9f64,
            0 as *mut i32,
            0 as *mut libc::c_double,
            0 as *mut libc::c_double,
            0 as *mut libc::c_double,
            &mut dy,
            &mut dz,
        );
        if ret == 0 as i32 {
            (*info.offset(k as isize)).eff = fabs(dy) / sqrt(temp);
            if (*(*T).mip).dir == 1 as i32 {
                if dz < 0.0f64 {
                    dz = 0.0f64;
                }
                (*info.offset(k as isize)).deg = dz;
            } else {
                if dz > 0.0f64 {
                    dz = 0.0f64;
                }
                (*info.offset(k as isize)).deg = -dz;
            }
        } else if ret == 1 as i32 {
            let ref mut fresh1 = (*info.offset(k as isize)).deg;
            *fresh1 = 0.0f64;
            (*info.offset(k as isize)).eff = *fresh1;
        } else if ret == 2 as i32 {
            (*info.offset(k as isize)).eff = 1.0f64;
            (*info.offset(k as isize)).deg = 1.7976931348623157e+308f64;
        } else {
            (ret != ret
                || {
                    glp_assert_(
                        b"ret != ret\0" as *const u8 as *const i8,
                        b"draft/glpios11.c\0" as *const u8 as *const i8,
                        156 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        }
        if (*info.offset(k as isize)).deg < 0.01f64 {
            (*info.offset(k as isize)).deg = 0.0f64;
        }
        k += 1;
        k;
    }
    qsort(
        &mut *info.offset(1 as i32 as isize) as *mut info as *mut libc::c_void,
        (*pool).m as size_t,
        ::core::mem::size_of::<info>() as u64,
        Some(
            fcmp as unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32,
        ),
    );
    max_cuts = if (*(*T).curr).level == 0 as i32 { 90 as i32 } else { 10 as i32 };
    if max_cuts > (*pool).m {
        max_cuts = (*pool).m;
    }
    k = 1 as i32;
    while k <= max_cuts {
        let mut i: i32 = 0;
        let mut len_0: i32 = 0;
        if !((*info.offset(k as isize)).deg < 0.01f64
            && (*info.offset(k as isize)).eff < 0.01f64)
        {
            kk = 1 as i32;
            while kk < k {
                if (*info.offset(kk as isize)).flag != 0 {
                    if parallel(
                        (*info.offset(k as isize)).cut,
                        (*info.offset(kk as isize)).cut,
                        work,
                    ) > 0.90f64
                    {
                        break;
                    }
                }
                kk += 1;
                kk;
            }
            if !(kk < k) {
                cut = (*info.offset(k as isize)).cut;
                (*info.offset(k as isize)).flag = 1 as i32 as i8;
                i = glp_add_rows((*T).mip, 1 as i32);
                if !((*cut).name).is_null() {
                    glp_set_row_name((*T).mip, i, (*cut).name);
                }
                ((**((*(*T).mip).row).offset(i as isize)).origin as i32 == 2 as i32
                    || {
                        glp_assert_(
                            b"T->mip->row[i]->origin == GLP_RF_CUT\0" as *const u8
                                as *const i8,
                            b"draft/glpios11.c\0" as *const u8 as *const i8,
                            192 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                (**((*(*T).mip).row).offset(i as isize)).klass = (*cut).klass;
                len_0 = 0 as i32;
                aij = (*cut).ptr;
                while !aij.is_null() {
                    len_0 += 1;
                    len_0;
                    *ind.offset(len_0 as isize) = (*(*aij).col).j;
                    *val.offset(len_0 as isize) = (*aij).val;
                    aij = (*aij).r_next;
                }
                glp_set_mat_row(
                    (*T).mip,
                    i,
                    len_0,
                    ind as *const i32,
                    val as *const libc::c_double,
                );
                match (*cut).type_0 {
                    2 => {
                        rhs = (*cut).lb;
                    }
                    3 => {
                        rhs = (*cut).ub;
                    }
                    _ => {
                        (cut != cut
                            || {
                                glp_assert_(
                                    b"cut != cut\0" as *const u8 as *const i8,
                                    b"draft/glpios11.c\0" as *const u8 as *const i8,
                                    201 as i32,
                                );
                                1 as i32 != 0
                            }) as i32;
                    }
                }
                glp_set_row_bnds((*T).mip, i, (*cut).type_0, rhs, rhs);
            }
        }
        k += 1;
        k;
    }
    glp_free(info as *mut libc::c_void);
    glp_free(ind as *mut libc::c_void);
    glp_free(val as *mut libc::c_void);
    glp_free(work as *mut libc::c_void);
}
unsafe extern "C" fn parallel(
    mut a: *mut IOSCUT,
    mut b: *mut IOSCUT,
    mut work: *mut libc::c_double,
) -> libc::c_double {
    let mut aij: *mut GLPAIJ = 0 as *mut GLPAIJ;
    let mut s: libc::c_double = 0.0f64;
    let mut sa: libc::c_double = 0.0f64;
    let mut sb: libc::c_double = 0.0f64;
    let mut temp: libc::c_double = 0.;
    aij = (*a).ptr;
    while !aij.is_null() {
        *work.offset((*(*aij).col).j as isize) = (*aij).val;
        sa += (*aij).val * (*aij).val;
        aij = (*aij).r_next;
    }
    aij = (*b).ptr;
    while !aij.is_null() {
        s += *work.offset((*(*aij).col).j as isize) * (*aij).val;
        sb += (*aij).val * (*aij).val;
        aij = (*aij).r_next;
    }
    aij = (*a).ptr;
    while !aij.is_null() {
        *work.offset((*(*aij).col).j as isize) = 0.0f64;
        aij = (*aij).r_next;
    }
    temp = sqrt(sa) * sqrt(sb);
    if temp < 2.2204460492503131e-16f64 * 2.2204460492503131e-16f64 {
        temp = 2.2204460492503131e-16f64;
    }
    return s / temp;
}