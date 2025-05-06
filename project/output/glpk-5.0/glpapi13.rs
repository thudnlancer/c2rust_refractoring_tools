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
    fn floor(_: libc::c_double) -> libc::c_double;
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
    fn glp_error_(file: *const i8, line: i32) -> glp_errfunc;
    fn glp_printf(fmt: *const i8, _: ...);
    fn _glp_ios_best_node(tree: *mut glp_tree) -> i32;
    fn _glp_ios_relative_gap(tree: *mut glp_tree) -> libc::c_double;
    fn _glp_ios_add_row(
        tree: *mut glp_tree,
        pool: *mut IOSPOOL,
        name: *const i8,
        klass: i32,
        flags: i32,
        len: i32,
        ind: *const i32,
        val: *const libc::c_double,
        type_0: i32,
        rhs: libc::c_double,
    ) -> i32;
    fn _glp_ios_del_row(tree: *mut glp_tree, pool: *mut IOSPOOL, i: i32);
    fn _glp_ios_clear_pool(tree: *mut glp_tree, pool: *mut IOSPOOL);
    fn _glp_ios_process_sol(T: *mut glp_tree);
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
pub struct glp_attr {
    pub level: i32,
    pub origin: i32,
    pub klass: i32,
    pub foo_bar: [libc::c_double; 7],
}
#[no_mangle]
pub unsafe extern "C" fn glp_ios_reason(mut tree: *mut glp_tree) -> i32 {
    return (*tree).reason;
}
#[no_mangle]
pub unsafe extern "C" fn glp_ios_get_prob(mut tree: *mut glp_tree) -> *mut glp_prob {
    return (*tree).mip;
}
#[no_mangle]
pub unsafe extern "C" fn glp_ios_tree_size(
    mut tree: *mut glp_tree,
    mut a_cnt: *mut i32,
    mut n_cnt: *mut i32,
    mut t_cnt: *mut i32,
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
#[no_mangle]
pub unsafe extern "C" fn glp_ios_curr_node(mut tree: *mut glp_tree) -> i32 {
    let mut node: *mut IOSNPD = 0 as *mut IOSNPD;
    node = (*tree).curr;
    return if node.is_null() { 0 as i32 } else { (*node).p };
}
#[no_mangle]
pub unsafe extern "C" fn glp_ios_next_node(mut tree: *mut glp_tree, mut p: i32) -> i32 {
    let mut node: *mut IOSNPD = 0 as *mut IOSNPD;
    if p == 0 as i32 {
        node = (*tree).head;
    } else {
        let mut current_block_6: u64;
        if !(1 as i32 <= p && p <= (*tree).nslots) {
            current_block_6 = 4457692755016659906;
        } else {
            current_block_6 = 17179679302217393232;
        }
        loop {
            match current_block_6 {
                4457692755016659906 => {
                    (glp_error_(
                        b"draft/glpapi13.c\0" as *const u8 as *const i8,
                        163 as i32,
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        b"glp_ios_next_node: p = %d; invalid subproblem reference number\n\0"
                            as *const u8 as *const i8,
                        p,
                    );
                    current_block_6 = 17179679302217393232;
                }
                _ => {
                    node = (*((*tree).slot).offset(p as isize)).node;
                    if node.is_null() {
                        current_block_6 = 4457692755016659906;
                    } else {
                        break;
                    }
                }
            }
        }
        if (*node).count != 0 as i32 {
            (glp_error_(b"draft/glpapi13.c\0" as *const u8 as *const i8, 169 as i32))
                .expect(
                    "non-null function pointer",
                )(
                b"glp_ios_next_node: p = %d; subproblem not in the active list\n\0"
                    as *const u8 as *const i8,
                p,
            );
        }
        node = (*node).next;
    }
    return if node.is_null() { 0 as i32 } else { (*node).p };
}
#[no_mangle]
pub unsafe extern "C" fn glp_ios_prev_node(mut tree: *mut glp_tree, mut p: i32) -> i32 {
    let mut node: *mut IOSNPD = 0 as *mut IOSNPD;
    if p == 0 as i32 {
        node = (*tree).tail;
    } else {
        let mut current_block_6: u64;
        if !(1 as i32 <= p && p <= (*tree).nslots) {
            current_block_6 = 4839069504723007948;
        } else {
            current_block_6 = 17179679302217393232;
        }
        loop {
            match current_block_6 {
                4839069504723007948 => {
                    (glp_error_(
                        b"draft/glpapi13.c\0" as *const u8 as *const i8,
                        210 as i32,
                    ))
                        .expect(
                            "non-null function pointer",
                        )(
                        b"glp_ios_prev_node: p = %d; invalid subproblem reference number\n\0"
                            as *const u8 as *const i8,
                        p,
                    );
                    current_block_6 = 17179679302217393232;
                }
                _ => {
                    node = (*((*tree).slot).offset(p as isize)).node;
                    if node.is_null() {
                        current_block_6 = 4839069504723007948;
                    } else {
                        break;
                    }
                }
            }
        }
        if (*node).count != 0 as i32 {
            (glp_error_(b"draft/glpapi13.c\0" as *const u8 as *const i8, 216 as i32))
                .expect(
                    "non-null function pointer",
                )(
                b"glp_ios_prev_node: p = %d; subproblem not in the active list\n\0"
                    as *const u8 as *const i8,
                p,
            );
        }
        node = (*node).prev;
    }
    return if node.is_null() { 0 as i32 } else { (*node).p };
}
#[no_mangle]
pub unsafe extern "C" fn glp_ios_up_node(mut tree: *mut glp_tree, mut p: i32) -> i32 {
    let mut current_block: u64;
    let mut node: *mut IOSNPD = 0 as *mut IOSNPD;
    if !(1 as i32 <= p && p <= (*tree).nslots) {
        current_block = 6605715998616123096;
    } else {
        current_block = 16559507199688588974;
    }
    loop {
        match current_block {
            6605715998616123096 => {
                (glp_error_(b"draft/glpapi13.c\0" as *const u8 as *const i8, 246 as i32))
                    .expect(
                        "non-null function pointer",
                    )(
                    b"glp_ios_up_node: p = %d; invalid subproblem reference number\n\0"
                        as *const u8 as *const i8,
                    p,
                );
                current_block = 16559507199688588974;
            }
            _ => {
                node = (*((*tree).slot).offset(p as isize)).node;
                if node.is_null() {
                    current_block = 6605715998616123096;
                } else {
                    break;
                }
            }
        }
    }
    node = (*node).up;
    return if node.is_null() { 0 as i32 } else { (*node).p };
}
#[no_mangle]
pub unsafe extern "C" fn glp_ios_node_level(mut tree: *mut glp_tree, mut p: i32) -> i32 {
    let mut current_block: u64;
    let mut node: *mut IOSNPD = 0 as *mut IOSNPD;
    if !(1 as i32 <= p && p <= (*tree).nslots) {
        current_block = 16972782986771149647;
    } else {
        current_block = 16559507199688588974;
    }
    loop {
        match current_block {
            16972782986771149647 => {
                (glp_error_(b"draft/glpapi13.c\0" as *const u8 as *const i8, 276 as i32))
                    .expect(
                        "non-null function pointer",
                    )(
                    b"glp_ios_node_level: p = %d; invalid subproblem reference number\n\0"
                        as *const u8 as *const i8,
                    p,
                );
                current_block = 16559507199688588974;
            }
            _ => {
                node = (*((*tree).slot).offset(p as isize)).node;
                if node.is_null() {
                    current_block = 16972782986771149647;
                } else {
                    break;
                }
            }
        }
    }
    return (*node).level;
}
#[no_mangle]
pub unsafe extern "C" fn glp_ios_node_bound(
    mut tree: *mut glp_tree,
    mut p: i32,
) -> libc::c_double {
    let mut current_block: u64;
    let mut node: *mut IOSNPD = 0 as *mut IOSNPD;
    if !(1 as i32 <= p && p <= (*tree).nslots) {
        current_block = 10955586933610770304;
    } else {
        current_block = 16559507199688588974;
    }
    loop {
        match current_block {
            10955586933610770304 => {
                (glp_error_(b"draft/glpapi13.c\0" as *const u8 as *const i8, 319 as i32))
                    .expect(
                        "non-null function pointer",
                    )(
                    b"glp_ios_node_bound: p = %d; invalid subproblem reference number\n\0"
                        as *const u8 as *const i8,
                    p,
                );
                current_block = 16559507199688588974;
            }
            _ => {
                node = (*((*tree).slot).offset(p as isize)).node;
                if node.is_null() {
                    current_block = 10955586933610770304;
                } else {
                    break;
                }
            }
        }
    }
    return (*node).bound;
}
#[no_mangle]
pub unsafe extern "C" fn glp_ios_best_node(mut tree: *mut glp_tree) -> i32 {
    return _glp_ios_best_node(tree);
}
#[no_mangle]
pub unsafe extern "C" fn glp_ios_mip_gap(mut tree: *mut glp_tree) -> libc::c_double {
    return _glp_ios_relative_gap(tree);
}
#[no_mangle]
pub unsafe extern "C" fn glp_ios_node_data(
    mut tree: *mut glp_tree,
    mut p: i32,
) -> *mut libc::c_void {
    let mut current_block: u64;
    let mut node: *mut IOSNPD = 0 as *mut IOSNPD;
    if !(1 as i32 <= p && p <= (*tree).nslots) {
        current_block = 16398156374735032991;
    } else {
        current_block = 16559507199688588974;
    }
    loop {
        match current_block {
            16398156374735032991 => {
                (glp_error_(b"draft/glpapi13.c\0" as *const u8 as *const i8, 416 as i32))
                    .expect(
                        "non-null function pointer",
                    )(
                    b"glp_ios_node_level: p = %d; invalid subproblem reference number\n\0"
                        as *const u8 as *const i8,
                    p,
                );
                current_block = 16559507199688588974;
            }
            _ => {
                node = (*((*tree).slot).offset(p as isize)).node;
                if node.is_null() {
                    current_block = 16398156374735032991;
                } else {
                    break;
                }
            }
        }
    }
    return (*node).data;
}
#[no_mangle]
pub unsafe extern "C" fn glp_ios_row_attr(
    mut tree: *mut glp_tree,
    mut i: i32,
    mut attr: *mut glp_attr,
) {
    let mut row: *mut GLPROW = 0 as *mut GLPROW;
    if !(1 as i32 <= i && i <= (*(*tree).mip).m) {
        (glp_error_(b"draft/glpapi13.c\0" as *const u8 as *const i8, 441 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_ios_row_attr: i = %d; row number out of range\n\0" as *const u8
                as *const i8,
            i,
        );
    }
    row = *((*(*tree).mip).row).offset(i as isize);
    (*attr).level = (*row).level;
    (*attr).origin = (*row).origin as i32;
    (*attr).klass = (*row).klass as i32;
}
#[no_mangle]
pub unsafe extern "C" fn glp_ios_pool_size(mut tree: *mut glp_tree) -> i32 {
    if (*tree).reason != 0x4 as i32 {
        (glp_error_(b"draft/glpapi13.c\0" as *const u8 as *const i8, 455 as i32))
            .expect(
                "non-null function pointer",
            )(b"glp_ios_pool_size: operation not allowed\n\0" as *const u8 as *const i8);
    }
    (!((*tree).local).is_null()
        || {
            glp_assert_(
                b"tree->local != NULL\0" as *const u8 as *const i8,
                b"draft/glpapi13.c\0" as *const u8 as *const i8,
                456 as i32,
            );
            1 as i32 != 0
        }) as i32;
    return (*(*tree).local).m;
}
#[no_mangle]
pub unsafe extern "C" fn glp_ios_add_row(
    mut tree: *mut glp_tree,
    mut name: *const i8,
    mut klass: i32,
    mut flags: i32,
    mut len: i32,
    mut ind: *const i32,
    mut val: *const libc::c_double,
    mut type_0: i32,
    mut rhs: libc::c_double,
) -> i32 {
    let mut num: i32 = 0;
    if (*tree).reason != 0x4 as i32 {
        (glp_error_(b"draft/glpapi13.c\0" as *const u8 as *const i8, 472 as i32))
            .expect(
                "non-null function pointer",
            )(b"glp_ios_add_row: operation not allowed\n\0" as *const u8 as *const i8);
    }
    (!((*tree).local).is_null()
        || {
            glp_assert_(
                b"tree->local != NULL\0" as *const u8 as *const i8,
                b"draft/glpapi13.c\0" as *const u8 as *const i8,
                473 as i32,
            );
            1 as i32 != 0
        }) as i32;
    num = _glp_ios_add_row(
        tree,
        (*tree).local,
        name,
        klass,
        flags,
        len,
        ind,
        val,
        type_0,
        rhs,
    );
    return num;
}
#[no_mangle]
pub unsafe extern "C" fn glp_ios_del_row(mut tree: *mut glp_tree, mut i: i32) {
    if (*tree).reason != 0x4 as i32 {
        (glp_error_(b"draft/glpapi13.c\0" as *const u8 as *const i8, 484 as i32))
            .expect(
                "non-null function pointer",
            )(b"glp_ios_del_row: operation not allowed\n\0" as *const u8 as *const i8);
    }
    _glp_ios_del_row(tree, (*tree).local, i);
}
#[no_mangle]
pub unsafe extern "C" fn glp_ios_clear_pool(mut tree: *mut glp_tree) {
    if (*tree).reason != 0x4 as i32 {
        (glp_error_(b"draft/glpapi13.c\0" as *const u8 as *const i8, 494 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_ios_clear_pool: operation not allowed\n\0" as *const u8 as *const i8,
        );
    }
    _glp_ios_clear_pool(tree, (*tree).local);
}
#[no_mangle]
pub unsafe extern "C" fn glp_ios_can_branch(mut tree: *mut glp_tree, mut j: i32) -> i32 {
    if !(1 as i32 <= j && j <= (*(*tree).mip).n) {
        (glp_error_(b"draft/glpapi13.c\0" as *const u8 as *const i8, 515 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_ios_can_branch: j = %d; column number out of range\n\0" as *const u8
                as *const i8,
            j,
        );
    }
    return *((*tree).non_int).offset(j as isize) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn glp_ios_branch_upon(
    mut tree: *mut glp_tree,
    mut j: i32,
    mut sel: i32,
) {
    if !(1 as i32 <= j && j <= (*(*tree).mip).n) {
        (glp_error_(b"draft/glpapi13.c\0" as *const u8 as *const i8, 546 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_ios_branch_upon: j = %d; column number out of range\n\0" as *const u8
                as *const i8,
            j,
        );
    }
    if !(sel == 1 as i32 || sel == 2 as i32 || sel == 0 as i32) {
        (glp_error_(b"draft/glpapi13.c\0" as *const u8 as *const i8, 550 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_ios_branch_upon: sel = %d: invalid branch selection flag\n\0"
                as *const u8 as *const i8,
            sel,
        );
    }
    if *((*tree).non_int).offset(j as isize) == 0 {
        (glp_error_(b"draft/glpapi13.c\0" as *const u8 as *const i8, 553 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_ios_branch_upon: j = %d; variable cannot be used to branch upon\n\0"
                as *const u8 as *const i8,
            j,
        );
    }
    if (*tree).br_var != 0 as i32 {
        (glp_error_(b"draft/glpapi13.c\0" as *const u8 as *const i8, 556 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_ios_branch_upon: branching variable already chosen\n\0" as *const u8
                as *const i8,
        );
    }
    (*tree).br_var = j;
    (*tree).br_sel = sel;
}
#[no_mangle]
pub unsafe extern "C" fn glp_ios_select_node(mut tree: *mut glp_tree, mut p: i32) {
    let mut current_block: u64;
    let mut node: *mut IOSNPD = 0 as *mut IOSNPD;
    if !(1 as i32 <= p && p <= (*tree).nslots) {
        current_block = 2099700737878324941;
    } else {
        current_block = 16559507199688588974;
    }
    loop {
        match current_block {
            2099700737878324941 => {
                (glp_error_(b"draft/glpapi13.c\0" as *const u8 as *const i8, 583 as i32))
                    .expect(
                        "non-null function pointer",
                    )(
                    b"glp_ios_select_node: p = %d; invalid subproblem reference number\n\0"
                        as *const u8 as *const i8,
                    p,
                );
                current_block = 16559507199688588974;
            }
            _ => {
                node = (*((*tree).slot).offset(p as isize)).node;
                if node.is_null() {
                    current_block = 2099700737878324941;
                } else {
                    break;
                }
            }
        }
    }
    if (*node).count != 0 as i32 {
        (glp_error_(b"draft/glpapi13.c\0" as *const u8 as *const i8, 589 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_ios_select_node: p = %d; subproblem not in the active list\n\0"
                as *const u8 as *const i8,
            p,
        );
    }
    if (*tree).next_p != 0 as i32 {
        (glp_error_(b"draft/glpapi13.c\0" as *const u8 as *const i8, 593 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_ios_select_node: subproblem already selected\n\0" as *const u8
                as *const i8,
        );
    }
    (*tree).next_p = p;
}
#[no_mangle]
pub unsafe extern "C" fn glp_ios_heur_sol(
    mut tree: *mut glp_tree,
    mut x: *const libc::c_double,
) -> i32 {
    let mut mip: *mut glp_prob = (*tree).mip;
    let mut m: i32 = (*tree).orig_m;
    let mut n: i32 = (*tree).n;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut obj: libc::c_double = 0.;
    ((*mip).m >= m
        || {
            glp_assert_(
                b"mip->m >= m\0" as *const u8 as *const i8,
                b"draft/glpapi13.c\0" as *const u8 as *const i8,
                638 as i32,
            );
            1 as i32 != 0
        }) as i32;
    ((*mip).n == n
        || {
            glp_assert_(
                b"mip->n == n\0" as *const u8 as *const i8,
                b"draft/glpapi13.c\0" as *const u8 as *const i8,
                639 as i32,
            );
            1 as i32 != 0
        }) as i32;
    obj = (*mip).c0;
    j = 1 as i32;
    while j <= n {
        let mut col: *mut GLPCOL = *((*mip).col).offset(j as isize);
        if (*col).kind == 2 as i32 {
            if *x.offset(j as isize) != floor(*x.offset(j as isize)) {
                return 1 as i32;
            }
        }
        obj += (*col).coef * *x.offset(j as isize);
        j += 1;
        j;
    }
    if (*mip).mip_stat == 2 as i32 {
        match (*mip).dir {
            1 => {
                if obj >= (*(*tree).mip).mip_obj {
                    return 1 as i32;
                }
            }
            2 => {
                if obj <= (*(*tree).mip).mip_obj {
                    return 1 as i32;
                }
            }
            _ => {
                (mip != mip
                    || {
                        glp_assert_(
                            b"mip != mip\0" as *const u8 as *const i8,
                            b"draft/glpapi13.c\0" as *const u8 as *const i8,
                            662 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
            }
        }
    }
    if (*(*tree).parm).msg_lev >= 2 as i32 {
        glp_printf(
            b"Solution found by heuristic: %.12g\n\0" as *const u8 as *const i8,
            obj,
        );
    }
    (*mip).mip_stat = 2 as i32;
    (*mip).mip_obj = obj;
    j = 1 as i32;
    while j <= n {
        (**((*mip).col).offset(j as isize)).mipx = *x.offset(j as isize);
        j += 1;
        j;
    }
    i = 1 as i32;
    while i <= m {
        let mut row: *mut GLPROW = *((*mip).row).offset(i as isize);
        let mut aij: *mut GLPAIJ = 0 as *mut GLPAIJ;
        (*row).mipx = 0.0f64;
        aij = (*row).ptr;
        while !aij.is_null() {
            (*row).mipx += (*aij).val * (*(*aij).col).mipx;
            aij = (*aij).r_next;
        }
        i += 1;
        i;
    }
    _glp_ios_process_sol(tree);
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn glp_ios_terminate(mut tree: *mut glp_tree) {
    if (*(*tree).parm).msg_lev >= 4 as i32 {
        glp_printf(
            b"The search is prematurely terminated due to application request\n\0"
                as *const u8 as *const i8,
        );
    }
    (*tree).stop = 1 as i32;
}