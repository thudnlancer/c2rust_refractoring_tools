use std::os::raw::{c_int, c_double, c_char, c_uchar, c_void};
use std::ptr;

#[repr(C)]
pub struct glp_prob {
    pub pool: *mut c_void,
    pub tree: *mut glp_tree,
    pub name: *mut c_char,
    pub obj: *mut c_char,
    pub dir: c_int,
    pub c0: c_double,
    pub m_max: c_int,
    pub n_max: c_int,
    pub m: c_int,
    pub n: c_int,
    pub nnz: c_int,
    pub row: *mut *mut GLPROW,
    pub col: *mut *mut GLPCOL,
    pub r_tree: *mut c_void,
    pub c_tree: *mut c_void,
    pub valid: c_int,
    pub head: *mut c_int,
    pub bfd: *mut c_void,
    pub pbs_stat: c_int,
    pub dbs_stat: c_int,
    pub obj_val: c_double,
    pub it_cnt: c_int,
    pub some: c_int,
    pub ipt_stat: c_int,
    pub ipt_obj: c_double,
    pub mip_stat: c_int,
    pub mip_obj: c_double,
}

#[repr(C)]
pub struct GLPCOL {
    pub j: c_int,
    pub name: *mut c_char,
    pub node: *mut c_void,
    pub kind: c_int,
    pub type_: c_int,
    pub lb: c_double,
    pub ub: c_double,
    pub coef: c_double,
    pub ptr: *mut GLPAIJ,
    pub sjj: c_double,
    pub stat: c_int,
    pub bind: c_int,
    pub prim: c_double,
    pub dual: c_double,
    pub pval: c_double,
    pub dval: c_double,
    pub mipx: c_double,
}

#[repr(C)]
pub struct GLPAIJ {
    pub row: *mut GLPROW,
    pub col: *mut GLPCOL,
    pub val: c_double,
    pub r_prev: *mut GLPAIJ,
    pub r_next: *mut GLPAIJ,
    pub c_prev: *mut GLPAIJ,
    pub c_next: *mut GLPAIJ,
}

#[repr(C)]
pub struct GLPROW {
    pub i: c_int,
    pub name: *mut c_char,
    pub node: *mut c_void,
    pub level: c_int,
    pub origin: c_uchar,
    pub klass: c_uchar,
    pub type_: c_int,
    pub lb: c_double,
    pub ub: c_double,
    pub ptr: *mut GLPAIJ,
    pub rii: c_double,
    pub stat: c_int,
    pub bind: c_int,
    pub prim: c_double,
    pub dual: c_double,
    pub pval: c_double,
    pub dval: c_double,
    pub mipx: c_double,
}

#[repr(C)]
pub struct glp_tree {
    pub magic: c_int,
    pub pool: *mut c_void,
    pub n: c_int,
    pub orig_m: c_int,
    pub orig_type: *mut c_uchar,
    pub orig_lb: *mut c_double,
    pub orig_ub: *mut c_double,
    pub orig_stat: *mut c_uchar,
    pub orig_prim: *mut c_double,
    pub orig_dual: *mut c_double,
    pub orig_obj: c_double,
    pub nslots: c_int,
    pub avail: c_int,
    pub slot: *mut IOSLOT,
    pub head: *mut IOSNPD,
    pub tail: *mut IOSNPD,
    pub a_cnt: c_int,
    pub n_cnt: c_int,
    pub t_cnt: c_int,
    pub root_m: c_int,
    pub root_type: *mut c_uchar,
    pub root_lb: *mut c_double,
    pub root_ub: *mut c_double,
    pub root_stat: *mut c_uchar,
    pub curr: *mut IOSNPD,
    pub mip: *mut glp_prob,
    pub non_int: *mut c_uchar,
    pub pred_m: c_int,
    pub pred_max: c_int,
    pub pred_type: *mut c_uchar,
    pub pred_lb: *mut c_double,
    pub pred_ub: *mut c_double,
    pub pred_stat: *mut c_uchar,
    pub local: *mut c_void,
    pub cov_gen: *mut c_void,
    pub mir_gen: *mut c_void,
    pub clq_gen: *mut c_void,
    pub pcost: *mut c_void,
    pub iwrk: *mut c_int,
    pub dwrk: *mut c_double,
    pub parm: *const glp_iocp,
    pub tm_beg: c_double,
    pub tm_lag: c_double,
    pub sol_cnt: c_int,
    pub P: *mut c_void,
    pub npp: *mut c_void,
    pub save_sol: *const c_char,
    pub save_cnt: c_int,
    pub reason: c_int,
    pub stop: c_int,
    pub next_p: c_int,
    pub reopt: c_int,
    pub reinv: c_int,
    pub br_var: c_int,
    pub br_sel: c_int,
    pub child: c_int,
}

#[repr(C)]
pub struct glp_iocp {
    pub msg_lev: c_int,
    pub br_tech: c_int,
    pub bt_tech: c_int,
    pub tol_int: c_double,
    pub tol_obj: c_double,
    pub tm_lim: c_int,
    pub out_frq: c_int,
    pub out_dly: c_int,
    pub cb_func: Option<unsafe extern "C" fn(*mut glp_tree, *mut c_void)>,
    pub cb_info: *mut c_void,
    pub cb_size: c_int,
    pub pp_tech: c_int,
    pub mip_gap: c_double,
    pub mir_cuts: c_int,
    pub gmi_cuts: c_int,
    pub cov_cuts: c_int,
    pub clq_cuts: c_int,
    pub presolve: c_int,
    pub binarize: c_int,
    pub fp_heur: c_int,
    pub ps_heur: c_int,
    pub ps_tm_lim: c_int,
    pub sr_heur: c_int,
    pub use_sol: c_int,
    pub save_sol: *const c_char,
    pub alien: c_int,
    pub flip: c_int,
    pub foo_bar: [c_double; 23],
}

#[repr(C)]
pub struct IOSNPD {
    pub p: c_int,
    pub up: *mut IOSNPD,
    pub level: c_int,
    pub count: c_int,
    pub b_ptr: *mut IOSBND,
    pub s_ptr: *mut IOSTAT,
    pub r_ptr: *mut IOSROW,
    pub solved: c_int,
    pub lp_obj: c_double,
    pub bound: c_double,
    pub ii_cnt: c_int,
    pub ii_sum: c_double,
    pub changed: c_int,
    pub br_var: c_int,
    pub br_val: c_double,
    pub data: *mut c_void,
    pub temp: *mut IOSNPD,
    pub prev: *mut IOSNPD,
    pub next: *mut IOSNPD,
}

#[repr(C)]
pub struct IOSROW {
    pub name: *mut c_char,
    pub origin: c_uchar,
    pub klass: c_uchar,
    pub type_: c_uchar,
    pub lb: c_double,
    pub ub: c_double,
    pub ptr: *mut IOSAIJ,
    pub rii: c_double,
    pub stat: c_uchar,
    pub next: *mut IOSROW,
}

#[repr(C)]
pub struct IOSAIJ {
    pub j: c_int,
    pub val: c_double,
    pub next: *mut IOSAIJ,
}

#[repr(C)]
pub struct IOSTAT {
    pub k: c_int,
    pub stat: c_uchar,
    pub next: *mut IOSTAT,
}

#[repr(C)]
pub struct IOSBND {
    pub k: c_int,
    pub type_: c_uchar,
    pub lb: c_double,
    pub ub: c_double,
    pub next: *mut IOSBND,
}

#[repr(C)]
pub struct IOSLOT {
    pub node: *mut IOSNPD,
    pub next: c_int,
}

#[repr(C)]
pub struct glp_attr {
    pub level: c_int,
    pub origin: c_int,
    pub klass: c_int,
    pub foo_bar: [c_double; 7],
}

pub unsafe fn glp_ios_reason(tree: *mut glp_tree) -> c_int {
    (*tree).reason
}

pub unsafe fn glp_ios_get_prob(tree: *mut glp_tree) -> *mut glp_prob {
    (*tree).mip
}

pub unsafe fn glp_ios_tree_size(
    tree: *mut glp_tree,
    a_cnt: *mut c_int,
    n_cnt: *mut c_int,
    t_cnt: *mut c_int,
) {
    if !a_cnt.is_null() {
        *a_cnt = (*tree).a_cnt;
    }
    if !n_cnt.is_null() {
        *n_cnt = (*tree).n_cnt;
    }
    if !t_cnt.is_null() {
        *t_cnt = (*tree).t_cnt;
    }
}

pub unsafe fn glp_ios_curr_node(tree: *mut glp_tree) -> c_int {
    let node = (*tree).curr;
    if node.is_null() { 0 } else { (*node).p }
}

pub unsafe fn glp_ios_next_node(tree: *mut glp_tree, p: c_int) -> c_int {
    let node = if p == 0 {
        (*tree).head
    } else {
        if !(1 <= p && p <= (*tree).nslots) {
            panic!("Invalid subproblem reference number");
        }
        let node = *(*tree).slot.offset(p as isize).node;
        if node.is_null() {
            panic!("Invalid subproblem reference number");
        }
        if (*node).count != 0 {
            panic!("Subproblem not in the active list");
        }
        (*node).next
    };
    if node.is_null() { 0 } else { (*node).p }
}

pub unsafe fn glp_ios_prev_node(tree: *mut glp_tree, p: c_int) -> c_int {
    let node = if p == 0 {
        (*tree).tail
    } else {
        if !(1 <= p && p <= (*tree).nslots) {
            panic!("Invalid subproblem reference number");
        }
        let node = *(*tree).slot.offset(p as isize).node;
        if node.is_null() {
            panic!("Invalid subproblem reference number");
        }
        if (*node).count != 0 {
            panic!("Subproblem not in the active list");
        }
        (*node).prev
    };
    if node.is_null() { 0 } else { (*node).p }
}

pub unsafe fn glp_ios_up_node(tree: *mut glp_tree, p: c_int) -> c_int {
    if !(1 <= p && p <= (*tree).nslots) {
        panic!("Invalid subproblem reference number");
    }
    let node = *(*tree).slot.offset(p as isize).node;
    if node.is_null() {
        panic!("Invalid subproblem reference number");
    }
    let node = (*node).up;
    if node.is_null() { 0 } else { (*node).p }
}

pub unsafe fn glp_ios_node_level(tree: *mut glp_tree, p: c_int) -> c_int {
    if !(1 <= p && p <= (*tree).nslots) {
        panic!("Invalid subproblem reference number");
    }
    let node = *(*tree).slot.offset(p as isize).node;
    if node.is_null() {
        panic!("Invalid subproblem reference number");
    }
    (*node).level
}

pub unsafe fn glp_ios_node_bound(tree: *mut glp_tree, p: c_int) -> c_double {
    if !(1 <= p && p <= (*tree).nslots) {
        panic!("Invalid subproblem reference number");
    }
    let node = *(*tree).slot.offset(p as isize).node;
    if node.is_null() {
        panic!("Invalid subproblem reference number");
    }
    (*node).bound
}

pub unsafe fn glp_ios_best_node(tree: *mut glp_tree) -> c_int {
    // Implementation would call _glp_ios_best_node
    0
}

pub unsafe fn glp_ios_mip_gap(tree: *mut glp_tree) -> c_double {
    // Implementation would call _glp_ios_relative_gap
    0.0
}

pub unsafe fn glp_ios_node_data(tree: *mut glp_tree, p: c_int) -> *mut c_void {
    if !(1 <= p && p <= (*tree).nslots) {
        panic!("Invalid subproblem reference number");
    }
    let node = *(*tree).slot.offset(p as isize).node;
    if node.is_null() {
        panic!("Invalid subproblem reference number");
    }
    (*node).data
}

pub unsafe fn glp_ios_row_attr(tree: *mut glp_tree, i: c_int, attr: *mut glp_attr) {
    if !(1 <= i && i <= (*(*tree).mip).m) {
        panic!("Row number out of range");
    }
    let row = *(*(*tree).mip).row.offset(i as isize);
    (*attr).level = (*row).level;
    (*attr).origin = (*row).origin as c_int;
    (*attr).klass = (*row).klass as c_int;
}

pub unsafe fn glp_ios_pool_size(tree: *mut glp_tree) -> c_int {
    if (*tree).reason != 0x4 {
        panic!("Operation not allowed");
    }
    if (*tree).local.is_null() {
        panic!("Local pool is null");
    }
    (*(*tree).local).m
}

pub unsafe fn glp_ios_add_row(
    tree: *mut glp_tree,
    name: *const c_char,
    klass: c_int,
    flags: c_int,
    len: c_int,
    ind: *const c_int,
    val: *const c_double,
    type_: c_int,
    rhs: c_double,
) -> c_int {
    if (*tree).reason != 0x4 {
        panic!("Operation not allowed");
    }
    if (*tree).local.is_null() {
        panic!("Local pool is null");
    }
    // Implementation would call _glp_ios_add_row
    0
}

pub unsafe fn glp_ios_del_row(tree: *mut glp_tree, i: c_int) {
    if (*tree).reason != 0x4 {
        panic!("Operation not allowed");
    }
    // Implementation would call _glp_ios_del_row
}

pub unsafe fn glp_ios_clear_pool(tree: *mut glp_tree) {
    if (*tree).reason != 0x4 {
        panic!("Operation not allowed");
    }
    // Implementation would call _glp_ios_clear_pool
}

pub unsafe fn glp_ios_can_branch(tree: *mut glp_tree, j: c_int) -> c_int {
    if !(1 <= j && j <= (*(*tree).mip).n) {
        panic!("Column number out of range");
    }
    *(*tree).non_int.offset(j as isize) as c_int
}

pub unsafe fn glp_ios_branch_upon(tree: *mut glp_tree, j: c_int, sel: c_int) {
    if !(1 <= j && j <= (*(*tree).mip).n) {
        panic!("Column number out of range");
    }
    if !(sel == 1 || sel == 2 || sel == 0) {
        panic!("Invalid branch selection flag");
    }
    if *(*tree).non_int.offset(j as isize) == 0 {
        panic!("Variable cannot be used to branch upon");
    }
    if (*tree).br_var != 0 {
        panic!("Branching variable already chosen");
    }
    (*tree).br_var = j;
    (*tree).br_sel = sel;
}

pub unsafe fn glp_ios_select_node(tree: *mut glp_tree, p: c_int) {
    if !(1 <= p && p <= (*tree).nslots) {
        panic!("Invalid subproblem reference number");
    }
    let node = *(*tree).slot.offset(p as isize).node;
    if node.is_null() {
        panic!("Invalid subproblem reference number");
    }
    if (*node).count != 0 {
        panic!("Subproblem not in the active list");
    }
    if (*tree).next_p != 0 {
        panic!("Subproblem already selected");
    }
    (*tree).next_p = p;
}

pub unsafe fn glp_ios_terminate(tree: *mut glp_tree) {
    if (*(*tree).parm).msg_lev >= 4 {
        println!("The search is prematurely terminated due to application request");
    }
    (*tree).stop = 1;
}