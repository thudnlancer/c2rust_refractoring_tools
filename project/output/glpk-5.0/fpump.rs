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
    pub type glp_cfg;
    pub type glp_mir;
    pub type glp_cov;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn glp_difftime(t1: libc::c_double, t0: libc::c_double) -> libc::c_double;
    fn glp_time() -> libc::c_double;
    fn glp_free(ptr: *mut libc::c_void);
    fn glp_alloc(n: i32, size: i32) -> *mut libc::c_void;
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
    fn glp_printf(fmt: *const i8, _: ...);
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn glp_create_prob() -> *mut glp_prob;
    fn glp_add_rows(P: *mut glp_prob, nrs: i32) -> i32;
    fn glp_set_row_bnds(
        P: *mut glp_prob,
        i: i32,
        type_0: i32,
        lb: libc::c_double,
        ub: libc::c_double,
    );
    fn glp_set_col_bnds(
        P: *mut glp_prob,
        j: i32,
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
    fn glp_copy_prob(dest: *mut glp_prob, prob: *mut glp_prob, names: i32);
    fn glp_delete_prob(P: *mut glp_prob);
    fn glp_simplex(P: *mut glp_prob, parm: *const glp_smcp) -> i32;
    fn glp_init_smcp(parm: *mut glp_smcp);
    fn glp_get_status(P: *mut glp_prob) -> i32;
    fn _glp_ios_is_hopeful(tree: *mut glp_tree, bound: libc::c_double) -> i32;
    fn glp_ios_heur_sol(T: *mut glp_tree, x: *const libc::c_double) -> i32;
    fn _glp_rng_create_rand() -> *mut RNG;
    fn _glp_rng_delete_rand(rand: *mut RNG);
    fn _glp_rng_uniform(
        rand: *mut RNG,
        a: libc::c_double,
        b: libc::c_double,
    ) -> libc::c_double;
}
pub type size_t = u64;
pub type __compar_fn_t = Option<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> i32,
>;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glp_smcp {
    pub msg_lev: i32,
    pub meth: i32,
    pub pricing: i32,
    pub r_test: i32,
    pub tol_bnd: libc::c_double,
    pub tol_dj: libc::c_double,
    pub tol_piv: libc::c_double,
    pub obj_ll: libc::c_double,
    pub obj_ul: libc::c_double,
    pub it_lim: i32,
    pub tm_lim: i32,
    pub out_frq: i32,
    pub out_dly: i32,
    pub presolve: i32,
    pub excl: i32,
    pub shift: i32,
    pub aorn: i32,
    pub foo_bar: [libc::c_double; 33],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RNG {
    pub A: [i32; 56],
    pub fptr: *mut i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VAR {
    pub j: i32,
    pub x: i32,
    pub d: libc::c_double,
}
unsafe extern "C" fn fcmp(
    mut x: *const libc::c_void,
    mut y: *const libc::c_void,
) -> i32 {
    let mut vx: *const VAR = x as *const VAR;
    let mut vy: *const VAR = y as *const VAR;
    if (*vx).d > (*vy).d {
        return -(1 as i32)
    } else if (*vx).d < (*vy).d {
        return 1 as i32
    } else {
        return 0 as i32
    };
}
#[no_mangle]
pub unsafe extern "C" fn _glp_ios_feas_pump(mut T: *mut glp_tree) {
    let mut current_block: u64;
    let mut P: *mut glp_prob = (*T).mip;
    let mut n: i32 = (*P).n;
    let mut lp: *mut glp_prob = 0 as *mut glp_prob;
    let mut var: *mut VAR = 0 as *mut VAR;
    let mut rand: *mut RNG = 0 as *mut RNG;
    let mut col: *mut GLPCOL = 0 as *mut GLPCOL;
    let mut parm: glp_smcp = glp_smcp {
        msg_lev: 0,
        meth: 0,
        pricing: 0,
        r_test: 0,
        tol_bnd: 0.,
        tol_dj: 0.,
        tol_piv: 0.,
        obj_ll: 0.,
        obj_ul: 0.,
        it_lim: 0,
        tm_lim: 0,
        out_frq: 0,
        out_dly: 0,
        presolve: 0,
        excl: 0,
        shift: 0,
        aorn: 0,
        foo_bar: [0.; 33],
    };
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut new_x: i32 = 0;
    let mut nfail: i32 = 0;
    let mut npass: i32 = 0;
    let mut nv: i32 = 0;
    let mut ret: i32 = 0;
    let mut stalling: i32 = 0;
    let mut dist: libc::c_double = 0.;
    let mut tol: libc::c_double = 0.;
    (glp_get_status(P) == 5 as i32
        || {
            glp_assert_(
                b"glp_get_status(P) == GLP_OPT\0" as *const u8 as *const i8,
                b"intopt/fpump.c\0" as *const u8 as *const i8,
                77 as i32,
            );
            1 as i32 != 0
        }) as i32;
    if (*(*T).curr).level == 0 as i32 && (*(*T).curr).solved == 1 as i32 {
        nv = 0 as i32;
        j = 1 as i32;
        loop {
            if !(j <= n) {
                current_block = 1054647088692577877;
                break;
            }
            col = *((*P).col).offset(j as isize);
            if !((*col).kind == 1 as i32) {
                if !((*col).type_0 == 5 as i32) {
                    ((*col).kind == 2 as i32
                        || {
                            glp_assert_(
                                b"col->kind == GLP_IV\0" as *const u8 as *const i8,
                                b"intopt/fpump.c\0" as *const u8 as *const i8,
                                89 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                    if (*col).type_0 == 4 as i32 && (*col).lb == 0.0f64
                        && (*col).ub == 1.0f64
                    {
                        nv += 1;
                        nv;
                    } else {
                        if (*(*T).parm).msg_lev >= 3 as i32 {
                            glp_printf(
                                b"FPUMP heuristic cannot be applied due to general integer variables\n\0"
                                    as *const u8 as *const i8,
                            );
                        }
                        current_block = 15113620276463314052;
                        break;
                    }
                }
            }
            j += 1;
            j;
        }
        match current_block {
            15113620276463314052 => {}
            _ => {
                if !(nv == 0 as i32) {
                    if (*(*T).parm).msg_lev >= 3 as i32 {
                        glp_printf(
                            b"Applying FPUMP heuristic...\n\0" as *const u8 as *const i8,
                        );
                    }
                    var = glp_alloc(
                        1 as i32 + nv,
                        ::core::mem::size_of::<VAR>() as u64 as i32,
                    ) as *mut VAR;
                    k = 0 as i32;
                    j = 1 as i32;
                    while j <= n {
                        col = *((*P).col).offset(j as isize);
                        if (*col).kind == 2 as i32 && (*col).type_0 == 4 as i32 {
                            k += 1;
                            (*var.offset(k as isize)).j = j;
                        }
                        j += 1;
                        j;
                    }
                    (k == nv
                        || {
                            glp_assert_(
                                b"k == nv\0" as *const u8 as *const i8,
                                b"intopt/fpump.c\0" as *const u8 as *const i8,
                                114 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                    lp = glp_create_prob();
                    '_more: loop {
                        glp_copy_prob(lp, P, 0 as i32);
                        if (*P).mip_stat == 2 as i32 {
                            let mut ind: *mut i32 = 0 as *mut i32;
                            let mut val: *mut libc::c_double = 0 as *mut libc::c_double;
                            let mut bnd: libc::c_double = 0.;
                            glp_add_rows(lp, 1 as i32);
                            ind = glp_alloc(
                                1 as i32 + n,
                                ::core::mem::size_of::<i32>() as u64 as i32,
                            ) as *mut i32;
                            val = glp_alloc(
                                1 as i32 + n,
                                ::core::mem::size_of::<libc::c_double>() as u64 as i32,
                            ) as *mut libc::c_double;
                            j = 1 as i32;
                            while j <= n {
                                *ind.offset(j as isize) = j;
                                *val.offset(j as isize) = (**((*P).col).offset(j as isize))
                                    .coef;
                                j += 1;
                                j;
                            }
                            glp_set_mat_row(
                                lp,
                                (*lp).m,
                                n,
                                ind as *const i32,
                                val as *const libc::c_double,
                            );
                            glp_free(ind as *mut libc::c_void);
                            glp_free(val as *mut libc::c_void);
                            bnd = 0.1f64 * (*P).obj_val + 0.9f64 * (*P).mip_obj;
                            if (*P).dir == 1 as i32 {
                                glp_set_row_bnds(
                                    lp,
                                    (*lp).m,
                                    3 as i32,
                                    0.0f64,
                                    bnd - (*P).c0,
                                );
                            } else if (*P).dir == 2 as i32 {
                                glp_set_row_bnds(
                                    lp,
                                    (*lp).m,
                                    2 as i32,
                                    bnd - (*P).c0,
                                    0.0f64,
                                );
                            } else {
                                (P != P
                                    || {
                                        glp_assert_(
                                            b"P != P\0" as *const u8 as *const i8,
                                            b"intopt/fpump.c\0" as *const u8 as *const i8,
                                            160 as i32,
                                        );
                                        1 as i32 != 0
                                    }) as i32;
                            }
                        }
                        npass = 0 as i32;
                        k = 1 as i32;
                        while k <= nv {
                            (*var.offset(k as isize)).x = -(1 as i32);
                            k += 1;
                            k;
                        }
                        '_pass: loop {
                            npass += 1;
                            npass;
                            if (*(*T).parm).msg_lev >= 3 as i32 {
                                glp_printf(b"Pass %d\n\0" as *const u8 as *const i8, npass);
                            }
                            dist = 1.7976931348623157e+308f64;
                            nfail = 0 as i32;
                            if npass > 1 as i32 {
                                let mut rho: libc::c_double = 0.;
                                let mut temp: libc::c_double = 0.;
                                if rand.is_null() {
                                    rand = _glp_rng_create_rand();
                                }
                                k = 1 as i32;
                                while k <= nv {
                                    j = (*var.offset(k as isize)).j;
                                    col = *((*lp).col).offset(j as isize);
                                    rho = _glp_rng_uniform(rand, -0.3f64, 0.7f64);
                                    if rho < 0.0f64 {
                                        rho = 0.0f64;
                                    }
                                    temp = fabs(
                                        (*var.offset(k as isize)).x as libc::c_double - (*col).prim,
                                    );
                                    if temp + rho > 0.5f64 {
                                        (*var.offset(k as isize)).x = 1 as i32
                                            - (*var.offset(k as isize)).x;
                                    }
                                    k += 1;
                                    k;
                                }
                                current_block = 16511827554221136656;
                            } else {
                                current_block = 16451874224452147654;
                            }
                            loop {
                                match current_block {
                                    16451874224452147654 => {
                                        stalling = 1 as i32;
                                        k = 1 as i32;
                                        while k <= nv {
                                            col = *((*lp).col)
                                                .offset((*var.offset(k as isize)).j as isize);
                                            if (*col).prim < 0.5f64 {
                                                new_x = 0 as i32;
                                            } else {
                                                new_x = 1 as i32;
                                            }
                                            if (*var.offset(k as isize)).x != new_x {
                                                stalling = 0 as i32;
                                                (*var.offset(k as isize)).x = new_x;
                                            }
                                            k += 1;
                                            k;
                                        }
                                        if stalling != 0 {
                                            k = 1 as i32;
                                            while k <= nv {
                                                col = *((*lp).col)
                                                    .offset((*var.offset(k as isize)).j as isize);
                                                (*var.offset(k as isize)).d = fabs(
                                                    (*col).prim - (*var.offset(k as isize)).x as libc::c_double,
                                                );
                                                k += 1;
                                                k;
                                            }
                                            qsort(
                                                &mut *var.offset(1 as i32 as isize) as *mut VAR
                                                    as *mut libc::c_void,
                                                nv as size_t,
                                                ::core::mem::size_of::<VAR>() as u64,
                                                Some(
                                                    fcmp
                                                        as unsafe extern "C" fn(
                                                            *const libc::c_void,
                                                            *const libc::c_void,
                                                        ) -> i32,
                                                ),
                                            );
                                            k = 1 as i32;
                                            while k <= nv {
                                                if k >= 5 as i32 && (*var.offset(k as isize)).d < 0.35f64
                                                    || k >= 10 as i32
                                                {
                                                    break;
                                                }
                                                (*var.offset(k as isize)).x = 1 as i32
                                                    - (*var.offset(k as isize)).x;
                                                k += 1;
                                                k;
                                            }
                                        }
                                        current_block = 16511827554221136656;
                                    }
                                    _ => {
                                        if (*(*T).parm).tm_lim < 2147483647 as i32
                                            && ((*(*T).parm).tm_lim - 1 as i32) as libc::c_double
                                                <= 1000.0f64 * glp_difftime(glp_time(), (*T).tm_beg)
                                        {
                                            break '_more;
                                        }
                                        (*lp).dir = 1 as i32;
                                        (*lp).c0 = 0.0f64;
                                        j = 1 as i32;
                                        while j <= n {
                                            (**((*lp).col).offset(j as isize)).coef = 0.0f64;
                                            j += 1;
                                            j;
                                        }
                                        k = 1 as i32;
                                        while k <= nv {
                                            j = (*var.offset(k as isize)).j;
                                            if (*var.offset(k as isize)).x == 0 as i32 {
                                                (**((*lp).col).offset(j as isize)).coef = 1.0f64;
                                            } else {
                                                (**((*lp).col).offset(j as isize)).coef = -1.0f64;
                                                (*lp).c0 += 1.0f64;
                                            }
                                            k += 1;
                                            k;
                                        }
                                        glp_init_smcp(&mut parm);
                                        if (*(*T).parm).msg_lev <= 1 as i32 {
                                            parm.msg_lev = (*(*T).parm).msg_lev;
                                        } else if (*(*T).parm).msg_lev <= 3 as i32 {
                                            parm.msg_lev = 2 as i32;
                                            parm.out_dly = 10000 as i32;
                                        }
                                        ret = glp_simplex(lp, &mut parm);
                                        if ret != 0 as i32 {
                                            if (*(*T).parm).msg_lev >= 1 as i32 {
                                                glp_printf(
                                                    b"Warning: glp_simplex returned %d\n\0" as *const u8
                                                        as *const i8,
                                                    ret,
                                                );
                                            }
                                            break '_more;
                                        } else {
                                            ret = glp_get_status(lp);
                                            if ret != 5 as i32 {
                                                if (*(*T).parm).msg_lev >= 1 as i32 {
                                                    glp_printf(
                                                        b"Warning: glp_get_status returned %d\n\0" as *const u8
                                                            as *const i8,
                                                        ret,
                                                    );
                                                }
                                                break '_more;
                                            } else {
                                                if (*(*T).parm).msg_lev >= 4 as i32 {
                                                    glp_printf(
                                                        b"delta = %g\n\0" as *const u8 as *const i8,
                                                        (*lp).obj_val,
                                                    );
                                                }
                                                tol = 0.3f64 * (*(*T).parm).tol_int;
                                                k = 1 as i32;
                                                while k <= nv {
                                                    col = *((*lp).col)
                                                        .offset((*var.offset(k as isize)).j as isize);
                                                    if tol < (*col).prim && (*col).prim < 1.0f64 - tol {
                                                        break;
                                                    }
                                                    k += 1;
                                                    k;
                                                }
                                                if k > nv {
                                                    let mut x: *mut libc::c_double = glp_alloc(
                                                        1 as i32 + n,
                                                        ::core::mem::size_of::<libc::c_double>() as u64 as i32,
                                                    ) as *mut libc::c_double;
                                                    j = 1 as i32;
                                                    while j <= n {
                                                        *x.offset(j as isize) = (**((*lp).col).offset(j as isize))
                                                            .prim;
                                                        if (**((*P).col).offset(j as isize)).kind == 2 as i32 {
                                                            *x.offset(j as isize) = floor(
                                                                *x.offset(j as isize) + 0.5f64,
                                                            );
                                                        }
                                                        j += 1;
                                                        j;
                                                    }
                                                    (*lp).c0 = (*P).c0;
                                                    (*lp).dir = (*P).dir;
                                                    k = 1 as i32;
                                                    while k <= nv {
                                                        glp_set_col_bnds(
                                                            lp,
                                                            (*var.offset(k as isize)).j,
                                                            5 as i32,
                                                            *x.offset((*var.offset(k as isize)).j as isize),
                                                            0.0f64,
                                                        );
                                                        k += 1;
                                                        k;
                                                    }
                                                    j = 1 as i32;
                                                    while j <= n {
                                                        (**((*lp).col).offset(j as isize)).coef = (**((*P).col)
                                                            .offset(j as isize))
                                                            .coef;
                                                        j += 1;
                                                        j;
                                                    }
                                                    ret = glp_simplex(lp, &mut parm);
                                                    if ret != 0 as i32 {
                                                        if (*(*T).parm).msg_lev >= 1 as i32 {
                                                            glp_printf(
                                                                b"Warning: glp_simplex returned %d\n\0" as *const u8
                                                                    as *const i8,
                                                                ret,
                                                            );
                                                        }
                                                        glp_free(x as *mut libc::c_void);
                                                        break '_more;
                                                    } else {
                                                        ret = glp_get_status(lp);
                                                        if ret != 5 as i32 {
                                                            if (*(*T).parm).msg_lev >= 1 as i32 {
                                                                glp_printf(
                                                                    b"Warning: glp_get_status returned %d\n\0" as *const u8
                                                                        as *const i8,
                                                                    ret,
                                                                );
                                                            }
                                                            glp_free(x as *mut libc::c_void);
                                                            break '_more;
                                                        } else {
                                                            j = 1 as i32;
                                                            while j <= n {
                                                                if (**((*P).col).offset(j as isize)).kind != 2 as i32 {
                                                                    *x.offset(j as isize) = (**((*lp).col).offset(j as isize))
                                                                        .prim;
                                                                }
                                                                j += 1;
                                                                j;
                                                            }
                                                            ret = glp_ios_heur_sol(T, x as *const libc::c_double);
                                                            glp_free(x as *mut libc::c_void);
                                                            if ret == 0 as i32 {
                                                                if _glp_ios_is_hopeful(T, (*(*T).curr).bound) != 0 {
                                                                    break '_pass;
                                                                } else {
                                                                    break '_more;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                if dist == 1.7976931348623157e+308f64
                                                    || (*lp).obj_val <= dist - 1e-6f64 * (1.0f64 + dist)
                                                {
                                                    nfail = 0 as i32;
                                                    dist = (*lp).obj_val;
                                                } else {
                                                    nfail += 1;
                                                    nfail;
                                                }
                                                if nfail < 3 as i32 {
                                                    current_block = 16451874224452147654;
                                                    continue;
                                                }
                                                if npass < 5 as i32 {
                                                    break;
                                                } else {
                                                    break '_more;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if !lp.is_null() {
        glp_delete_prob(lp);
    }
    if !var.is_null() {
        glp_free(var as *mut libc::c_void);
    }
    if !rand.is_null() {
        _glp_rng_delete_rand(rand);
    }
}