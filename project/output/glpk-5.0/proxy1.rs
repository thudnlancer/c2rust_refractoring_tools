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
    fn glp_free(ptr: *mut libc::c_void);
    fn glp_alloc(n: i32, size: i32) -> *mut libc::c_void;
    fn glp_printf(fmt: *const i8, _: ...);
    fn glp_create_prob() -> *mut glp_prob;
    fn glp_copy_prob(dest: *mut glp_prob, prob: *mut glp_prob, names: i32);
    fn glp_delete_prob(P: *mut glp_prob);
    fn glp_check_kkt(
        P: *mut glp_prob,
        sol: i32,
        cond: i32,
        ae_max: *mut libc::c_double,
        ae_ind: *mut i32,
        re_max: *mut libc::c_double,
        re_ind: *mut i32,
    );
    fn glp_ios_heur_sol(T: *mut glp_tree, x: *const libc::c_double) -> i32;
    fn _glp_proxy(
        lp: *mut glp_prob,
        zstar: *mut libc::c_double,
        xstar: *mut libc::c_double,
        initsol: *const libc::c_double,
        rel_impr: libc::c_double,
        tlim: i32,
        verbose: i32,
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
#[no_mangle]
pub unsafe extern "C" fn _glp_ios_proxy_heur(mut T: *mut glp_tree) {
    let mut prob: *mut glp_prob = 0 as *mut glp_prob;
    let mut j: i32 = 0;
    let mut status: i32 = 0;
    let mut xstar: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut zstar: libc::c_double = 0.;
    if (*(*T).curr).level == 0 as i32 && (*(*T).curr).solved == 1 as i32 {
        prob = glp_create_prob();
        glp_copy_prob(prob, (*T).mip, 0 as i32);
        xstar = glp_alloc(
            1 as i32 + (*prob).n,
            ::core::mem::size_of::<libc::c_double>() as u64 as i32,
        ) as *mut libc::c_double;
        j = 1 as i32;
        while j <= (*prob).n {
            *xstar.offset(j as isize) = 0.0f64;
            j += 1;
            j;
        }
        if (*(*T).mip).mip_stat != 2 as i32 {
            status = _glp_proxy(
                prob,
                &mut zstar,
                xstar,
                0 as *const libc::c_double,
                0.0f64,
                (*(*T).parm).ps_tm_lim,
                1 as i32,
            );
        } else {
            let mut xinit: *mut libc::c_double = glp_alloc(
                1 as i32 + (*prob).n,
                ::core::mem::size_of::<libc::c_double>() as u64 as i32,
            ) as *mut libc::c_double;
            j = 1 as i32;
            while j <= (*prob).n {
                *xinit.offset(j as isize) = (**((*(*T).mip).col).offset(j as isize))
                    .mipx;
                j += 1;
                j;
            }
            status = _glp_proxy(
                prob,
                &mut zstar,
                xstar,
                xinit as *const libc::c_double,
                0.0f64,
                (*(*T).parm).ps_tm_lim,
                1 as i32,
            );
            glp_free(xinit as *mut libc::c_void);
        }
        if status == 0 as i32 {
            let mut i: i32 = 0;
            let mut feas1: i32 = 0;
            let mut feas2: i32 = 0;
            let mut ae_ind: i32 = 0;
            let mut re_ind: i32 = 0;
            let mut ae_max: libc::c_double = 0.;
            let mut re_max: libc::c_double = 0.;
            glp_copy_prob(prob, (*T).mip, 0 as i32);
            j = 1 as i32;
            while j <= (*prob).n {
                (**((*prob).col).offset(j as isize)).mipx = *xstar.offset(j as isize);
                j += 1;
                j;
            }
            i = 1 as i32;
            while i <= (*prob).m {
                let mut row: *mut GLPROW = 0 as *mut GLPROW;
                let mut aij: *mut GLPAIJ = 0 as *mut GLPAIJ;
                row = *((*prob).row).offset(i as isize);
                (*row).mipx = 0.0f64;
                aij = (*row).ptr;
                while !aij.is_null() {
                    (*row).mipx += (*aij).val * (*(*aij).col).mipx;
                    aij = (*aij).r_next;
                }
                i += 1;
                i;
            }
            glp_check_kkt(
                prob,
                3 as i32,
                1 as i32,
                &mut ae_max,
                &mut ae_ind,
                &mut re_max,
                &mut re_ind,
            );
            feas1 = (re_max <= 1e-6f64) as i32;
            glp_check_kkt(
                prob,
                3 as i32,
                2 as i32,
                &mut ae_max,
                &mut ae_ind,
                &mut re_max,
                &mut re_ind,
            );
            feas2 = (re_max <= 1e-6f64) as i32;
            if feas1 != 0 && feas2 != 0 {
                glp_ios_heur_sol(T, xstar as *const libc::c_double);
            } else {
                glp_printf(
                    b"WARNING: PROXY HEURISTIC REPORTED WRONG SOLUTION; SOLUTION REJECTED\n\0"
                        as *const u8 as *const i8,
                );
            }
        }
        glp_free(xstar as *mut libc::c_void);
        glp_delete_prob(prob);
    }
}