use ::libc;
extern "C" {
    pub type AVL;
    pub type AVLNODE;
    pub type BFD;
    pub type DMP;
    pub type glp_cfg;
    pub type glp_mir;
    pub type glp_cov;
    fn glp_free(ptr: *mut libc::c_void);
    fn glp_alloc(n: libc::c_int, size: libc::c_int) -> *mut libc::c_void;
    fn glp_printf(fmt: *const libc::c_char, _: ...);
    fn glp_create_prob() -> *mut glp_prob;
    fn glp_copy_prob(dest: *mut glp_prob, prob: *mut glp_prob, names: libc::c_int);
    fn glp_delete_prob(P: *mut glp_prob);
    fn glp_check_kkt(
        P: *mut glp_prob,
        sol: libc::c_int,
        cond: libc::c_int,
        ae_max: *mut libc::c_double,
        ae_ind: *mut libc::c_int,
        re_max: *mut libc::c_double,
        re_ind: *mut libc::c_int,
    );
    fn glp_ios_heur_sol(T: *mut glp_tree, x: *const libc::c_double) -> libc::c_int;
    fn _glp_proxy(
        lp: *mut glp_prob,
        zstar: *mut libc::c_double,
        xstar: *mut libc::c_double,
        initsol: *const libc::c_double,
        rel_impr: libc::c_double,
        tlim: libc::c_int,
        verbose: libc::c_int,
    ) -> libc::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glp_prob {
    pub pool: *mut DMP,
    pub tree: *mut glp_tree,
    pub name: *mut libc::c_char,
    pub obj: *mut libc::c_char,
    pub dir: libc::c_int,
    pub c0: libc::c_double,
    pub m_max: libc::c_int,
    pub n_max: libc::c_int,
    pub m: libc::c_int,
    pub n: libc::c_int,
    pub nnz: libc::c_int,
    pub row: *mut *mut GLPROW,
    pub col: *mut *mut GLPCOL,
    pub r_tree: *mut AVL,
    pub c_tree: *mut AVL,
    pub valid: libc::c_int,
    pub head: *mut libc::c_int,
    pub bfd: *mut BFD,
    pub pbs_stat: libc::c_int,
    pub dbs_stat: libc::c_int,
    pub obj_val: libc::c_double,
    pub it_cnt: libc::c_int,
    pub some: libc::c_int,
    pub ipt_stat: libc::c_int,
    pub ipt_obj: libc::c_double,
    pub mip_stat: libc::c_int,
    pub mip_obj: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GLPCOL {
    pub j: libc::c_int,
    pub name: *mut libc::c_char,
    pub node: *mut AVLNODE,
    pub kind: libc::c_int,
    pub type_0: libc::c_int,
    pub lb: libc::c_double,
    pub ub: libc::c_double,
    pub coef: libc::c_double,
    pub ptr: *mut GLPAIJ,
    pub sjj: libc::c_double,
    pub stat: libc::c_int,
    pub bind: libc::c_int,
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
    pub i: libc::c_int,
    pub name: *mut libc::c_char,
    pub node: *mut AVLNODE,
    pub level: libc::c_int,
    pub origin: libc::c_uchar,
    pub klass: libc::c_uchar,
    pub type_0: libc::c_int,
    pub lb: libc::c_double,
    pub ub: libc::c_double,
    pub ptr: *mut GLPAIJ,
    pub rii: libc::c_double,
    pub stat: libc::c_int,
    pub bind: libc::c_int,
    pub prim: libc::c_double,
    pub dual: libc::c_double,
    pub pval: libc::c_double,
    pub dval: libc::c_double,
    pub mipx: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glp_tree {
    pub magic: libc::c_int,
    pub pool: *mut DMP,
    pub n: libc::c_int,
    pub orig_m: libc::c_int,
    pub orig_type: *mut libc::c_uchar,
    pub orig_lb: *mut libc::c_double,
    pub orig_ub: *mut libc::c_double,
    pub orig_stat: *mut libc::c_uchar,
    pub orig_prim: *mut libc::c_double,
    pub orig_dual: *mut libc::c_double,
    pub orig_obj: libc::c_double,
    pub nslots: libc::c_int,
    pub avail: libc::c_int,
    pub slot: *mut IOSLOT,
    pub head: *mut IOSNPD,
    pub tail: *mut IOSNPD,
    pub a_cnt: libc::c_int,
    pub n_cnt: libc::c_int,
    pub t_cnt: libc::c_int,
    pub root_m: libc::c_int,
    pub root_type: *mut libc::c_uchar,
    pub root_lb: *mut libc::c_double,
    pub root_ub: *mut libc::c_double,
    pub root_stat: *mut libc::c_uchar,
    pub curr: *mut IOSNPD,
    pub mip: *mut glp_prob,
    pub non_int: *mut libc::c_uchar,
    pub pred_m: libc::c_int,
    pub pred_max: libc::c_int,
    pub pred_type: *mut libc::c_uchar,
    pub pred_lb: *mut libc::c_double,
    pub pred_ub: *mut libc::c_double,
    pub pred_stat: *mut libc::c_uchar,
    pub local: *mut IOSPOOL,
    pub cov_gen: *mut glp_cov,
    pub mir_gen: *mut glp_mir,
    pub clq_gen: *mut glp_cfg,
    pub pcost: *mut libc::c_void,
    pub iwrk: *mut libc::c_int,
    pub dwrk: *mut libc::c_double,
    pub parm: *const glp_iocp,
    pub tm_beg: libc::c_double,
    pub tm_lag: libc::c_double,
    pub sol_cnt: libc::c_int,
    pub P: *mut libc::c_void,
    pub npp: *mut libc::c_void,
    pub save_sol: *const libc::c_char,
    pub save_cnt: libc::c_int,
    pub reason: libc::c_int,
    pub stop: libc::c_int,
    pub next_p: libc::c_int,
    pub reopt: libc::c_int,
    pub reinv: libc::c_int,
    pub br_var: libc::c_int,
    pub br_sel: libc::c_int,
    pub child: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glp_iocp {
    pub msg_lev: libc::c_int,
    pub br_tech: libc::c_int,
    pub bt_tech: libc::c_int,
    pub tol_int: libc::c_double,
    pub tol_obj: libc::c_double,
    pub tm_lim: libc::c_int,
    pub out_frq: libc::c_int,
    pub out_dly: libc::c_int,
    pub cb_func: Option::<unsafe extern "C" fn(*mut glp_tree, *mut libc::c_void) -> ()>,
    pub cb_info: *mut libc::c_void,
    pub cb_size: libc::c_int,
    pub pp_tech: libc::c_int,
    pub mip_gap: libc::c_double,
    pub mir_cuts: libc::c_int,
    pub gmi_cuts: libc::c_int,
    pub cov_cuts: libc::c_int,
    pub clq_cuts: libc::c_int,
    pub presolve: libc::c_int,
    pub binarize: libc::c_int,
    pub fp_heur: libc::c_int,
    pub ps_heur: libc::c_int,
    pub ps_tm_lim: libc::c_int,
    pub sr_heur: libc::c_int,
    pub use_sol: libc::c_int,
    pub save_sol: *const libc::c_char,
    pub alien: libc::c_int,
    pub flip: libc::c_int,
    pub foo_bar: [libc::c_double; 23],
}
pub type IOSPOOL = glp_prob;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IOSNPD {
    pub p: libc::c_int,
    pub up: *mut IOSNPD,
    pub level: libc::c_int,
    pub count: libc::c_int,
    pub b_ptr: *mut IOSBND,
    pub s_ptr: *mut IOSTAT,
    pub r_ptr: *mut IOSROW,
    pub solved: libc::c_int,
    pub lp_obj: libc::c_double,
    pub bound: libc::c_double,
    pub ii_cnt: libc::c_int,
    pub ii_sum: libc::c_double,
    pub changed: libc::c_int,
    pub br_var: libc::c_int,
    pub br_val: libc::c_double,
    pub data: *mut libc::c_void,
    pub temp: *mut IOSNPD,
    pub prev: *mut IOSNPD,
    pub next: *mut IOSNPD,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IOSROW {
    pub name: *mut libc::c_char,
    pub origin: libc::c_uchar,
    pub klass: libc::c_uchar,
    pub type_0: libc::c_uchar,
    pub lb: libc::c_double,
    pub ub: libc::c_double,
    pub ptr: *mut IOSAIJ,
    pub rii: libc::c_double,
    pub stat: libc::c_uchar,
    pub next: *mut IOSROW,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IOSAIJ {
    pub j: libc::c_int,
    pub val: libc::c_double,
    pub next: *mut IOSAIJ,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IOSTAT {
    pub k: libc::c_int,
    pub stat: libc::c_uchar,
    pub next: *mut IOSTAT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IOSBND {
    pub k: libc::c_int,
    pub type_0: libc::c_uchar,
    pub lb: libc::c_double,
    pub ub: libc::c_double,
    pub next: *mut IOSBND,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IOSLOT {
    pub node: *mut IOSNPD,
    pub next: libc::c_int,
}
#[no_mangle]
pub unsafe extern "C" fn _glp_ios_proxy_heur(mut T: *mut glp_tree) {
    let mut prob: *mut glp_prob = 0 as *mut glp_prob;
    let mut j: libc::c_int = 0;
    let mut status: libc::c_int = 0;
    let mut xstar: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut zstar: libc::c_double = 0.;
    if (*(*T).curr).level == 0 as libc::c_int && (*(*T).curr).solved == 1 as libc::c_int
    {
        prob = glp_create_prob();
        glp_copy_prob(prob, (*T).mip, 0 as libc::c_int);
        xstar = glp_alloc(
            1 as libc::c_int + (*prob).n,
            ::core::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
        ) as *mut libc::c_double;
        j = 1 as libc::c_int;
        while j <= (*prob).n {
            *xstar.offset(j as isize) = 0.0f64;
            j += 1;
            j;
        }
        if (*(*T).mip).mip_stat != 2 as libc::c_int {
            status = _glp_proxy(
                prob,
                &mut zstar,
                xstar,
                0 as *const libc::c_double,
                0.0f64,
                (*(*T).parm).ps_tm_lim,
                1 as libc::c_int,
            );
        } else {
            let mut xinit: *mut libc::c_double = glp_alloc(
                1 as libc::c_int + (*prob).n,
                ::core::mem::size_of::<libc::c_double>() as libc::c_ulong as libc::c_int,
            ) as *mut libc::c_double;
            j = 1 as libc::c_int;
            while j <= (*prob).n {
                *xinit
                    .offset(j as isize) = (**((*(*T).mip).col).offset(j as isize)).mipx;
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
                1 as libc::c_int,
            );
            glp_free(xinit as *mut libc::c_void);
        }
        if status == 0 as libc::c_int {
            let mut i: libc::c_int = 0;
            let mut feas1: libc::c_int = 0;
            let mut feas2: libc::c_int = 0;
            let mut ae_ind: libc::c_int = 0;
            let mut re_ind: libc::c_int = 0;
            let mut ae_max: libc::c_double = 0.;
            let mut re_max: libc::c_double = 0.;
            glp_copy_prob(prob, (*T).mip, 0 as libc::c_int);
            j = 1 as libc::c_int;
            while j <= (*prob).n {
                (**((*prob).col).offset(j as isize)).mipx = *xstar.offset(j as isize);
                j += 1;
                j;
            }
            i = 1 as libc::c_int;
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
                3 as libc::c_int,
                1 as libc::c_int,
                &mut ae_max,
                &mut ae_ind,
                &mut re_max,
                &mut re_ind,
            );
            feas1 = (re_max <= 1e-6f64) as libc::c_int;
            glp_check_kkt(
                prob,
                3 as libc::c_int,
                2 as libc::c_int,
                &mut ae_max,
                &mut ae_ind,
                &mut re_max,
                &mut re_ind,
            );
            feas2 = (re_max <= 1e-6f64) as libc::c_int;
            if feas1 != 0 && feas2 != 0 {
                glp_ios_heur_sol(T, xstar as *const libc::c_double);
            } else {
                glp_printf(
                    b"WARNING: PROXY HEURISTIC REPORTED WRONG SOLUTION; SOLUTION REJECTED\n\0"
                        as *const u8 as *const libc::c_char,
                );
            }
        }
        glp_free(xstar as *mut libc::c_void);
        glp_delete_prob(prob);
    }
}
