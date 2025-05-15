/* glpapi13.rs (branch-and-bound interface routines) */

/***********************************************************************
*  This code is part of GLPK (GNU Linear Programming Kit).
*  Copyright (C) 2000-2018 Free Software Foundation, Inc.
*  Written by Andrew Makhorin <mao@gnu.org>.
*
*  GLPK is free software: you can redistribute it and/or modify it
*  under the terms of the GNU General Public License as published by
*  the Free Software Foundation, either version 3 of the License, or
*  (at your option) any later version.
*
*  GLPK is distributed in the hope that it will be useful, but WITHOUT
*  ANY WARRANTY; without even the implied warranty of MERCHANTABILITY
*  or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public
*  License for more details.
*
*  You should have received a copy of the GNU General Public License
*  along with GLPK. If not, see <http://www.gnu.org/licenses/>.
***********************************************************************/

use std::os::raw::{c_int, c_double};
use std::ptr;

#[repr(C)]
pub struct glp_tree {
    reason: c_int,
    mip: *mut glp_prob,
    a_cnt: c_int,
    n_cnt: c_int,
    t_cnt: c_int,
    curr: *mut IOSNPD,
    head: *mut IOSNPD,
    tail: *mut IOSNPD,
    nslots: c_int,
    slot: *mut IOSS,
    non_int: *mut c_int,
    br_var: c_int,
    br_sel: c_int,
    next_p: c_int,
    orig_m: c_int,
    n: c_int,
    parm: *mut glp_iocp,
    stop: c_int,
    local: *mut glp_local,
}

#[repr(C)]
pub struct glp_prob {
    // ... fields omitted for brevity
}

#[repr(C)]
pub struct IOSNPD {
    p: c_int,
    up: *mut IOSNPD,
    next: *mut IOSNPD,
    prev: *mut IOSNPD,
    count: c_int,
    level: c_int,
    bound: c_double,
    data: *mut std::ffi::c_void,
}

#[repr(C)]
pub struct IOSS {
    node: *mut IOSNPD,
}

#[repr(C)]
pub struct glp_iocp {
    msg_lev: c_int,
    // ... other fields omitted
}

#[repr(C)]
pub struct glp_local {
    m: c_int,
    size: c_int,
    // ... other fields omitted
}

#[repr(C)]
pub struct glp_attr {
    level: c_int,
    origin: c_int,
    klass: c_int,
}

#[repr(C)]
pub struct GLPROW {
    level: c_int,
    origin: c_int,
    klass: c_int,
    ptr: *mut GLPAIJ,
    mipx: c_double,
}

#[repr(C)]
pub struct GLPAIJ {
    val: c_double,
    col: *mut GLPCOL,
    r_next: *mut GLPAIJ,
}

#[repr(C)]
pub struct GLPCOL {
    kind: c_int,
    coef: c_double,
    mipx: c_double,
}

pub const GLP_ICUTGEN: c_int = 1;
pub const GLP_IBRANCH: c_int = 2;
pub const GLP_ISELECT: c_int = 3;
pub const GLP_IHEUR: c_int = 4;
pub const GLP_DN_BRNCH: c_int = 1;
pub const GLP_UP_BRNCH: c_int = 2;
pub const GLP_NO_BRNCH: c_int = 3;
pub const GLP_FEAS: c_int = 1;
pub const GLP_MIN: c_int = 1;
pub const GLP_MAX: c_int = 2;
pub const GLP_MSG_ON: c_int = 1;
pub const GLP_MSG_DBG: c_int = 3;
pub const GLP_IV: c_int = 1;

pub fn glp_ios_reason(tree: *mut glp_tree) -> c_int {
    unsafe { (*tree).reason }
}

pub fn glp_ios_get_prob(tree: *mut glp_tree) -> *mut glp_prob {
    unsafe { (*tree).mip }
}

pub fn glp_ios_tree_size(
    tree: *mut glp_tree,
    a_cnt: *mut c_int,
    n_cnt: *mut c_int,
    t_cnt: *mut c_int,
) {
    unsafe {
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
}

pub fn glp_ios_curr_node(tree: *mut glp_tree) -> c_int {
    unsafe {
        let node = (*tree).curr;
        if node.is_null() {
            0
        } else {
            (*node).p
        }
    }
}

pub fn glp_ios_next_node(tree: *mut glp_tree, p: c_int) -> c_int {
    unsafe {
        let node = if p == 0 {
            (*tree).head
        } else {
            if p < 1 || p > (*tree).nslots {
                panic!("glp_ios_next_node: p = {}; invalid subproblem reference number", p);
            }
            let node = (*(*tree).slot.offset((p - 1) as isize)).node;
            if node.is_null() {
                panic!("glp_ios_next_node: p = {}; invalid subproblem reference number", p);
            }
            if (*node).count != 0 {
                panic!("glp_ios_next_node: p = {}; subproblem not in the active list", p);
            }
            (*node).next
        };
        if node.is_null() {
            0
        } else {
            (*node).p
        }
    }
}

pub fn glp_ios_prev_node(tree: *mut glp_tree, p: c_int) -> c_int {
    unsafe {
        let node = if p == 0 {
            (*tree).tail
        } else {
            if p < 1 || p > (*tree).nslots {
                panic!("glp_ios_prev_node: p = {}; invalid subproblem reference number", p);
            }
            let node = (*(*tree).slot.offset((p - 1) as isize)).node;
            if node.is_null() {
                panic!("glp_ios_prev_node: p = {}; invalid subproblem reference number", p);
            }
            if (*node).count != 0 {
                panic!("glp_ios_prev_node: p = {}; subproblem not in the active list", p);
            }
            (*node).prev
        };
        if node.is_null() {
            0
        } else {
            (*node).p
        }
    }
}

pub fn glp_ios_up_node(tree: *mut glp_tree, p: c_int) -> c_int {
    unsafe {
        if p < 1 || p > (*tree).nslots {
            panic!("glp_ios_up_node: p = {}; invalid subproblem reference number", p);
        }
        let node = (*(*tree).slot.offset((p - 1) as isize)).node;
        if node.is_null() {
            panic!("glp_ios_up_node: p = {}; invalid subproblem reference number", p);
        }
        let node = (*node).up;
        if node.is_null() {
            0
        } else {
            (*node).p
        }
    }
}

pub fn glp_ios_node_level(tree: *mut glp_tree, p: c_int) -> c_int {
    unsafe {
        if p < 1 || p > (*tree).nslots {
            panic!("glp_ios_node_level: p = {}; invalid subproblem reference number", p);
        }
        let node = (*(*tree).slot.offset((p - 1) as isize)).node;
        if node.is_null() {
            panic!("glp_ios_node_level: p = {}; invalid subproblem reference number", p);
        }
        (*node).level
    }
}

pub fn glp_ios_node_bound(tree: *mut glp_tree, p: c_int) -> c_double {
    unsafe {
        if p < 1 || p > (*tree).nslots {
            panic!("glp_ios_node_bound: p = {}; invalid subproblem reference number", p);
        }
        let node = (*(*tree).slot.offset((p - 1) as isize)).node;
        if node.is_null() {
            panic!("glp_ios_node_bound: p = {}; invalid subproblem reference number", p);
        }
        (*node).bound
    }
}

pub fn glp_ios_best_node(tree: *mut glp_tree) -> c_int {
    unsafe { ios_best_node(tree) }
}

pub fn glp_ios_mip_gap(tree: *mut glp_tree) -> c_double {
    unsafe { ios_relative_gap(tree) }
}

pub fn glp_ios_node_data(tree: *mut glp_tree, p: c_int) -> *mut std::ffi::c_void {
    unsafe {
        if p < 1 || p > (*tree).nslots {
            panic!("glp_ios_node_data: p = {}; invalid subproblem reference number", p);
        }
        let node = (*(*tree).slot.offset((p - 1) as isize)).node;
        if node.is_null() {
            panic!("glp_ios_node_data: p = {}; invalid subproblem reference number", p);
        }
        (*node).data
    }
}

pub fn glp_ios_row_attr(tree: *mut glp_tree, i: c_int, attr: *mut glp_attr) {
    unsafe {
        if i < 1 || i > (*(*tree).mip).m {
            panic!("glp_ios_row_attr: i = {}; row number out of range", i);
        }
        let row = &*(*(*tree).mip).row.offset((i - 1) as isize);
        (*attr).level = row.level;
        (*attr).origin = row.origin;
        (*attr).klass = row.klass;
    }
}

pub fn glp_ios_pool_size(tree: *mut glp_tree) -> c_int {
    unsafe {
        if (*tree).reason != GLP_ICUTGEN {
            panic!("glp_ios_pool_size: operation not allowed");
        }
        assert!(!(*tree).local.is_null());
        (*(*tree).local).m
    }
}

pub fn glp_ios_add_row(
    tree: *mut glp_tree,
    name: *const std::ffi::c_char,
    klass: c_int,
    flags: c_int,
    len: c_int,
    ind: *const c_int,
    val: *const c_double,
    type_: c_int,
    rhs: c_double,
) -> c_int {
    unsafe {
        if (*tree).reason != GLP_ICUTGEN {
            panic!("glp_ios_add_row: operation not allowed");
        }
        assert!(!(*tree).local.is_null());
        ios_add_row(tree, (*tree).local, name, klass, flags, len, ind, val, type_, rhs)
    }
}

pub fn glp_ios_del_row(tree: *mut glp_tree, i: c_int) {
    unsafe {
        if (*tree).reason != GLP_ICUTGEN {
            panic!("glp_ios_del_row: operation not allowed");
        }
        ios_del_row(tree, (*tree).local, i);
    }
}

pub fn glp_ios_clear_pool(tree: *mut glp_tree) {
    unsafe {
        if (*tree).reason != GLP_ICUTGEN {
            panic!("glp_ios_clear_pool: operation not allowed");
        }
        ios_clear_pool(tree, (*tree).local);
    }
}

pub fn glp_ios_can_branch(tree: *mut glp_tree, j: c_int) -> c_int {
    unsafe {
        if j < 1 || j > (*(*tree).mip).n {
            panic!("glp_ios_can_branch: j = {}; column number out of range", j);
        }
        *(*tree).non_int.offset((j - 1) as isize)
    }
}

pub fn glp_ios_branch_upon(tree: *mut glp_tree, j: c_int, sel: c_int) {
    unsafe {
        if j < 1 || j > (*(*tree).mip).n {
            panic!("glp_ios_branch_upon: j = {}; column number out of range", j);
        }
        if sel != GLP_DN_BRNCH && sel != GLP_UP_BRNCH && sel != GLP_NO_BRNCH {
            panic!("glp_ios_branch_upon: sel = {}: invalid branch selection flag", sel);
        }
        if *(*tree).non_int.offset((j - 1) as isize) == 0 {
            panic!("glp_ios_branch_upon: j = {}; variable cannot be used to branch upon", j);
        }
        if (*tree).br_var != 0 {
            panic!("glp_ios_branch_upon: branching variable already chosen");
        }
        (*tree).br_var = j;
        (*tree).br_sel = sel;
    }
}

pub fn glp_ios_select_node(tree: *mut glp_tree, p: c_int) {
    unsafe {
        if p < 1 || p > (*tree).nslots {
            panic!("glp_ios_select_node: p = {}; invalid subproblem reference number", p);
        }
        let node = (*(*tree).slot.offset((p - 1) as isize)).node;
        if node.is_null() {
            panic!("glp_ios_select_node: p = {}; invalid subproblem reference number", p);
        }
        if (*node).count != 0 {
            panic!("glp_ios_select_node: p = {}; subproblem not in the active list", p);
        }
        if (*tree).next_p != 0 {
            panic!("glp_ios_select_node: subproblem already selected");
        }
        (*tree).next_p = p;
    }
}

pub fn glp_ios_heur_sol(tree: *mut glp_tree, x: *const c_double) -> c_int {
    unsafe {
        let mip = (*tree).mip;
        let m = (*tree).orig_m;
        let n = (*tree).n;
        let mut obj = (*mip).c0;

        assert!((*mip).m >= m);
        assert!((*mip).n == n);

        for j in 1..=n {
            let col = &*(*mip).col.offset((j - 1) as isize);
            if col.kind == GLP_IV {
                if *x.offset((j - 1) as isize) != (*x.offset((j - 1) as isize)).floor() {
                    return 1;
                }
            }
            obj += col.coef * *x.offset((j - 1) as isize);
        }

        if (*mip).mip_stat == GLP_FEAS {
            match (*mip).dir {
                GLP_MIN => {
                    if obj >= (*(*tree).mip).mip_obj {
                        return 1;
                    }
                }
                GLP_MAX => {
                    if obj <= (*(*tree).mip).mip_obj {
                        return 1;
                    }
                }
                _ => unreachable!(),
            }
        }

        if (*tree).parm.msg_lev >= GLP_MSG_ON {
            println!("Solution found by heuristic: {:.12}", obj);
        }

        (*mip).mip_stat = GLP_FEAS;
        (*mip).mip_obj = obj;

        for j in 1..=n {
            (*(*mip).col.offset((j - 1) as isize)).mipx = *x.offset((j - 1) as isize);
        }

        for i in 1..=m {
            let row = &mut *(*mip).row.offset((i - 1) as isize);
            row.mipx = 0.0;
            let mut aij = row.ptr;
            while !aij.is_null() {
                row.mipx += (*aij).val * (*(*aij).col).mipx;
                aij = (*aij).r_next;
            }
        }

        ios_process_sol(tree);
        0
    }
}

pub fn glp_ios_terminate(tree: *mut glp_tree) {
    unsafe {
        if (*tree).parm.msg_lev >= GLP_MSG_DBG {
            println!("The search is prematurely terminated due to application request");
        }
        (*tree).stop = 1;
    }
}

// Placeholder for external functions
extern "C" {
    fn ios_best_node(tree: *mut glp_tree) -> c_int;
    fn ios_relative_gap(tree: *mut glp_tree) -> c_double;
    fn ios_add_row(
        tree: *mut glp_tree,
        local: *mut glp_local,
        name: *const std::ffi::c_char,
        klass: c_int,
        flags: c_int,
        len: c_int,
        ind: *const c_int,
        val: *const c_double,
        type_: c_int,
        rhs: c_double,
    ) -> c_int;
    fn ios_del_row(tree: *mut glp_tree, local: *mut glp_local, i: c_int);
    fn ios_clear_pool(tree: *mut glp_tree, local: *mut glp_local);
    fn ios_process_sol(tree: *mut glp_tree);
}