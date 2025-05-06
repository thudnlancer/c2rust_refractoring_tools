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
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn glp_free(ptr: *mut libc::c_void);
    fn glp_alloc(n: i32, size: i32) -> *mut libc::c_void;
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
    fn glp_error_(file: *const i8, line: i32) -> glp_errfunc;
    fn strlen(_: *const i8) -> u64;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn glp_get_mat_col(
        P: *mut glp_prob,
        j: i32,
        ind: *mut i32,
        val: *mut libc::c_double,
    ) -> i32;
    fn glp_get_bfcp(P: *mut glp_prob, parm: *mut glp_bfcp);
    fn glp_set_bfcp(P: *mut glp_prob, parm: *const glp_bfcp);
    fn _glp_avl_insert_node(tree: *mut AVL, key: *const libc::c_void) -> *mut AVLNODE;
    fn _glp_avl_set_node_link(node: *mut AVLNODE, link: *mut libc::c_void);
    fn _glp_avl_delete_tree(tree: *mut AVL);
    fn _glp_avl_delete_node(tree: *mut AVL, node: *mut AVLNODE);
    fn _glp_dmp_free_atom(pool: *mut DMP, atom: *mut libc::c_void, size: i32);
    fn _glp_dmp_get_atom(pool: *mut DMP, size: i32) -> *mut libc::c_void;
    fn _glp_dmp_create_pool() -> *mut DMP;
    fn _glp_bfd_delete_it(bfd: *mut BFD);
    fn _glp_dmp_delete_pool(pool: *mut DMP);
}
pub type C2RustUnnamed = u32;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
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
pub struct glp_bfcp {
    pub msg_lev: i32,
    pub type_0: i32,
    pub lu_size: i32,
    pub piv_tol: libc::c_double,
    pub piv_lim: i32,
    pub suhl: i32,
    pub eps_tol: libc::c_double,
    pub max_gro: libc::c_double,
    pub nfs_max: i32,
    pub upd_tol: libc::c_double,
    pub nrs_max: i32,
    pub rs_size: i32,
    pub foo_bar: [libc::c_double; 38],
}
unsafe extern "C" fn create_prob(mut lp: *mut glp_prob) {
    (*lp).pool = _glp_dmp_create_pool();
    (*lp).tree = 0 as *mut glp_tree;
    (*lp).name = 0 as *mut i8;
    (*lp).obj = 0 as *mut i8;
    (*lp).dir = 1 as i32;
    (*lp).c0 = 0.0f64;
    (*lp).m_max = 100 as i32;
    (*lp).n_max = 200 as i32;
    (*lp).n = 0 as i32;
    (*lp).m = (*lp).n;
    (*lp).nnz = 0 as i32;
    (*lp).row = glp_alloc(
        1 as i32 + (*lp).m_max,
        ::core::mem::size_of::<*mut GLPROW>() as u64 as i32,
    ) as *mut *mut GLPROW;
    (*lp).col = glp_alloc(
        1 as i32 + (*lp).n_max,
        ::core::mem::size_of::<*mut GLPCOL>() as u64 as i32,
    ) as *mut *mut GLPCOL;
    (*lp).c_tree = 0 as *mut AVL;
    (*lp).r_tree = (*lp).c_tree;
    (*lp).valid = 0 as i32;
    (*lp).head = glp_alloc(
        1 as i32 + (*lp).m_max,
        ::core::mem::size_of::<i32>() as u64 as i32,
    ) as *mut i32;
    (*lp).bfd = 0 as *mut BFD;
    (*lp).dbs_stat = 1 as i32;
    (*lp).pbs_stat = (*lp).dbs_stat;
    (*lp).obj_val = 0.0f64;
    (*lp).it_cnt = 0 as i32;
    (*lp).some = 0 as i32;
    (*lp).ipt_stat = 1 as i32;
    (*lp).ipt_obj = 0.0f64;
    (*lp).mip_stat = 1 as i32;
    (*lp).mip_obj = 0.0f64;
}
#[no_mangle]
pub unsafe extern "C" fn glp_create_prob() -> *mut glp_prob {
    let mut lp: *mut glp_prob = 0 as *mut glp_prob;
    lp = glp_alloc(1 as i32, ::core::mem::size_of::<glp_prob>() as u64 as i32)
        as *mut glp_prob;
    create_prob(lp);
    return lp;
}
#[no_mangle]
pub unsafe extern "C" fn glp_set_prob_name(mut lp: *mut glp_prob, mut name: *const i8) {
    let mut tree: *mut glp_tree = (*lp).tree;
    if !tree.is_null() && (*tree).reason != 0 as i32 {
        (glp_error_(b"api/prob1.c\0" as *const u8 as *const i8, 135 as i32))
            .expect(
                "non-null function pointer",
            )(b"glp_set_prob_name: operation not allowed\n\0" as *const u8 as *const i8);
    }
    if !((*lp).name).is_null() {
        _glp_dmp_free_atom(
            (*lp).pool,
            (*lp).name as *mut libc::c_void,
            (strlen((*lp).name)).wrapping_add(1 as i32 as u64) as i32,
        );
        (*lp).name = 0 as *mut i8;
    }
    if !(name.is_null() || *name.offset(0 as i32 as isize) as i32 == '\0' as i32) {
        let mut k: i32 = 0;
        k = 0 as i32;
        while *name.offset(k as isize) as i32 != '\0' as i32 {
            if k == 256 as i32 {
                (glp_error_(b"api/prob1.c\0" as *const u8 as *const i8, 144 as i32))
                    .expect(
                        "non-null function pointer",
                    )(
                    b"glp_set_prob_name: problem name too long\n\0" as *const u8
                        as *const i8,
                );
            }
            if *(*__ctype_b_loc()).offset(*name.offset(k as isize) as u8 as i32 as isize)
                as i32 & _IScntrl as i32 as libc::c_ushort as i32 != 0
            {
                (glp_error_(b"api/prob1.c\0" as *const u8 as *const i8, 146 as i32))
                    .expect(
                        "non-null function pointer",
                    )(
                    b"glp_set_prob_name: problem name contains invalid character(s)\n\0"
                        as *const u8 as *const i8,
                );
            }
            k += 1;
            k;
        }
        (*lp).name = _glp_dmp_get_atom(
            (*lp).pool,
            (strlen(name)).wrapping_add(1 as i32 as u64) as i32,
        ) as *mut i8;
        strcpy((*lp).name, name);
    }
}
#[no_mangle]
pub unsafe extern "C" fn glp_set_obj_name(mut lp: *mut glp_prob, mut name: *const i8) {
    let mut tree: *mut glp_tree = (*lp).tree;
    if !tree.is_null() && (*tree).reason != 0 as i32 {
        (glp_error_(b"api/prob1.c\0" as *const u8 as *const i8, 176 as i32))
            .expect(
                "non-null function pointer",
            )(b"glp_set_obj_name: operation not allowed\n\0" as *const u8 as *const i8);
    }
    if !((*lp).obj).is_null() {
        _glp_dmp_free_atom(
            (*lp).pool,
            (*lp).obj as *mut libc::c_void,
            (strlen((*lp).obj)).wrapping_add(1 as i32 as u64) as i32,
        );
        (*lp).obj = 0 as *mut i8;
    }
    if !(name.is_null() || *name.offset(0 as i32 as isize) as i32 == '\0' as i32) {
        let mut k: i32 = 0;
        k = 0 as i32;
        while *name.offset(k as isize) as i32 != '\0' as i32 {
            if k == 256 as i32 {
                (glp_error_(b"api/prob1.c\0" as *const u8 as *const i8, 185 as i32))
                    .expect(
                        "non-null function pointer",
                    )(
                    b"glp_set_obj_name: objective name too long\n\0" as *const u8
                        as *const i8,
                );
            }
            if *(*__ctype_b_loc()).offset(*name.offset(k as isize) as u8 as i32 as isize)
                as i32 & _IScntrl as i32 as libc::c_ushort as i32 != 0
            {
                (glp_error_(b"api/prob1.c\0" as *const u8 as *const i8, 187 as i32))
                    .expect(
                        "non-null function pointer",
                    )(
                    b"glp_set_obj_name: objective name contains invalid character(s)\n\0"
                        as *const u8 as *const i8,
                );
            }
            k += 1;
            k;
        }
        (*lp).obj = _glp_dmp_get_atom(
            (*lp).pool,
            (strlen(name)).wrapping_add(1 as i32 as u64) as i32,
        ) as *mut i8;
        strcpy((*lp).obj, name);
    }
}
#[no_mangle]
pub unsafe extern "C" fn glp_set_obj_dir(mut lp: *mut glp_prob, mut dir: i32) {
    let mut tree: *mut glp_tree = (*lp).tree;
    if !tree.is_null() && (*tree).reason != 0 as i32 {
        (glp_error_(b"api/prob1.c\0" as *const u8 as *const i8, 217 as i32))
            .expect(
                "non-null function pointer",
            )(b"glp_set_obj_dir: operation not allowed\n\0" as *const u8 as *const i8);
    }
    if !(dir == 1 as i32 || dir == 2 as i32) {
        (glp_error_(b"api/prob1.c\0" as *const u8 as *const i8, 219 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_set_obj_dir: dir = %d; invalid direction flag\n\0" as *const u8
                as *const i8,
            dir,
        );
    }
    (*lp).dir = dir;
}
#[no_mangle]
pub unsafe extern "C" fn glp_add_rows(mut lp: *mut glp_prob, mut nrs: i32) -> i32 {
    let mut tree: *mut glp_tree = (*lp).tree;
    let mut row: *mut GLPROW = 0 as *mut GLPROW;
    let mut m_new: i32 = 0;
    let mut i: i32 = 0;
    if nrs < 1 as i32 {
        (glp_error_(b"api/prob1.c\0" as *const u8 as *const i8, 254 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_add_rows: nrs = %d; invalid number of rows\n\0" as *const u8
                as *const i8,
            nrs,
        );
    }
    if nrs > 100000000 as i32 - (*lp).m {
        (glp_error_(b"api/prob1.c\0" as *const u8 as *const i8, 257 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_add_rows: nrs = %d; too many rows\n\0" as *const u8 as *const i8,
            nrs,
        );
    }
    m_new = (*lp).m + nrs;
    if (*lp).m_max < m_new {
        let mut save: *mut *mut GLPROW = (*lp).row;
        while (*lp).m_max < m_new {
            (*lp).m_max += (*lp).m_max;
            ((*lp).m_max > 0 as i32
                || {
                    glp_assert_(
                        b"lp->m_max > 0\0" as *const u8 as *const i8,
                        b"api/prob1.c\0" as *const u8 as *const i8,
                        264 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        }
        (*lp).row = glp_alloc(
            1 as i32 + (*lp).m_max,
            ::core::mem::size_of::<*mut GLPROW>() as u64 as i32,
        ) as *mut *mut GLPROW;
        memcpy(
            &mut *((*lp).row).offset(1 as i32 as isize) as *mut *mut GLPROW
                as *mut libc::c_void,
            &mut *save.offset(1 as i32 as isize) as *mut *mut GLPROW
                as *const libc::c_void,
            ((*lp).m as u64).wrapping_mul(::core::mem::size_of::<*mut GLPROW>() as u64),
        );
        glp_free(save as *mut libc::c_void);
        glp_free((*lp).head as *mut libc::c_void);
        (*lp).head = glp_alloc(
            1 as i32 + (*lp).m_max,
            ::core::mem::size_of::<i32>() as u64 as i32,
        ) as *mut i32;
    }
    i = (*lp).m + 1 as i32;
    while i <= m_new {
        row = _glp_dmp_get_atom(
            (*lp).pool,
            ::core::mem::size_of::<GLPROW>() as u64 as i32,
        ) as *mut GLPROW;
        let ref mut fresh0 = *((*lp).row).offset(i as isize);
        *fresh0 = row;
        (*row).i = i;
        (*row).name = 0 as *mut i8;
        (*row).node = 0 as *mut AVLNODE;
        (*row).level = 0 as i32;
        (*row).origin = 0 as i32 as u8;
        (*row).klass = 0 as i32 as u8;
        if !tree.is_null() {
            match (*tree).reason {
                0 => {}
                1 => {
                    (!((*tree).curr).is_null()
                        || {
                            glp_assert_(
                                b"tree->curr != NULL\0" as *const u8 as *const i8,
                                b"api/prob1.c\0" as *const u8 as *const i8,
                                289 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                    (*row).level = (*(*tree).curr).level;
                    (*row).origin = 1 as i32 as u8;
                }
                4 => {
                    (!((*tree).curr).is_null()
                        || {
                            glp_assert_(
                                b"tree->curr != NULL\0" as *const u8 as *const i8,
                                b"api/prob1.c\0" as *const u8 as *const i8,
                                294 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                    (*row).level = (*(*tree).curr).level;
                    (*row).origin = 2 as i32 as u8;
                }
                _ => {
                    (tree != tree
                        || {
                            glp_assert_(
                                b"tree != tree\0" as *const u8 as *const i8,
                                b"api/prob1.c\0" as *const u8 as *const i8,
                                299 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                }
            }
        }
        (*row).type_0 = 1 as i32;
        (*row).ub = 0.0f64;
        (*row).lb = (*row).ub;
        (*row).ptr = 0 as *mut GLPAIJ;
        (*row).rii = 1.0f64;
        (*row).stat = 1 as i32;
        (*row).bind = 0 as i32;
        (*row).dual = 0.0f64;
        (*row).prim = (*row).dual;
        (*row).dval = 0.0f64;
        (*row).pval = (*row).dval;
        (*row).mipx = 0.0f64;
        i += 1;
        i;
    }
    (*lp).m = m_new;
    (*lp).valid = 0 as i32;
    if !tree.is_null() && (*tree).reason != 0 as i32 {
        (*tree).reopt = 1 as i32;
    }
    return m_new - nrs + 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn glp_add_cols(mut lp: *mut glp_prob, mut ncs: i32) -> i32 {
    let mut tree: *mut glp_tree = (*lp).tree;
    let mut col: *mut GLPCOL = 0 as *mut GLPCOL;
    let mut n_new: i32 = 0;
    let mut j: i32 = 0;
    if !tree.is_null() && (*tree).reason != 0 as i32 {
        (glp_error_(b"api/prob1.c\0" as *const u8 as *const i8, 357 as i32))
            .expect(
                "non-null function pointer",
            )(b"glp_add_cols: operation not allowed\n\0" as *const u8 as *const i8);
    }
    if ncs < 1 as i32 {
        (glp_error_(b"api/prob1.c\0" as *const u8 as *const i8, 360 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_add_cols: ncs = %d; invalid number of columns\n\0" as *const u8
                as *const i8,
            ncs,
        );
    }
    if ncs > 100000000 as i32 - (*lp).n {
        (glp_error_(b"api/prob1.c\0" as *const u8 as *const i8, 363 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_add_cols: ncs = %d; too many columns\n\0" as *const u8 as *const i8,
            ncs,
        );
    }
    n_new = (*lp).n + ncs;
    if (*lp).n_max < n_new {
        let mut save: *mut *mut GLPCOL = (*lp).col;
        while (*lp).n_max < n_new {
            (*lp).n_max += (*lp).n_max;
            ((*lp).n_max > 0 as i32
                || {
                    glp_assert_(
                        b"lp->n_max > 0\0" as *const u8 as *const i8,
                        b"api/prob1.c\0" as *const u8 as *const i8,
                        370 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
        }
        (*lp).col = glp_alloc(
            1 as i32 + (*lp).n_max,
            ::core::mem::size_of::<*mut GLPCOL>() as u64 as i32,
        ) as *mut *mut GLPCOL;
        memcpy(
            &mut *((*lp).col).offset(1 as i32 as isize) as *mut *mut GLPCOL
                as *mut libc::c_void,
            &mut *save.offset(1 as i32 as isize) as *mut *mut GLPCOL
                as *const libc::c_void,
            ((*lp).n as u64).wrapping_mul(::core::mem::size_of::<*mut GLPCOL>() as u64),
        );
        glp_free(save as *mut libc::c_void);
    }
    j = (*lp).n + 1 as i32;
    while j <= n_new {
        col = _glp_dmp_get_atom(
            (*lp).pool,
            ::core::mem::size_of::<GLPCOL>() as u64 as i32,
        ) as *mut GLPCOL;
        let ref mut fresh1 = *((*lp).col).offset(j as isize);
        *fresh1 = col;
        (*col).j = j;
        (*col).name = 0 as *mut i8;
        (*col).node = 0 as *mut AVLNODE;
        (*col).kind = 1 as i32;
        (*col).type_0 = 5 as i32;
        (*col).ub = 0.0f64;
        (*col).lb = (*col).ub;
        (*col).coef = 0.0f64;
        (*col).ptr = 0 as *mut GLPAIJ;
        (*col).sjj = 1.0f64;
        (*col).stat = 5 as i32;
        (*col).bind = 0 as i32;
        (*col).dual = 0.0f64;
        (*col).prim = (*col).dual;
        (*col).dval = 0.0f64;
        (*col).pval = (*col).dval;
        (*col).mipx = 0.0f64;
        j += 1;
        j;
    }
    (*lp).n = n_new;
    return n_new - ncs + 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn glp_set_row_name(
    mut lp: *mut glp_prob,
    mut i: i32,
    mut name: *const i8,
) {
    let mut tree: *mut glp_tree = (*lp).tree;
    let mut row: *mut GLPROW = 0 as *mut GLPROW;
    if !(1 as i32 <= i && i <= (*lp).m) {
        (glp_error_(b"api/prob1.c\0" as *const u8 as *const i8, 427 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_set_row_name: i = %d; row number out of range\n\0" as *const u8
                as *const i8,
            i,
        );
    }
    row = *((*lp).row).offset(i as isize);
    if !tree.is_null() && (*tree).reason != 0 as i32 {
        (!((*tree).curr).is_null()
            || {
                glp_assert_(
                    b"tree->curr != NULL\0" as *const u8 as *const i8,
                    b"api/prob1.c\0" as *const u8 as *const i8,
                    431 as i32,
                );
                1 as i32 != 0
            }) as i32;
        ((*row).level == (*(*tree).curr).level
            || {
                glp_assert_(
                    b"row->level == tree->curr->level\0" as *const u8 as *const i8,
                    b"api/prob1.c\0" as *const u8 as *const i8,
                    432 as i32,
                );
                1 as i32 != 0
            }) as i32;
    }
    if !((*row).name).is_null() {
        if !((*row).node).is_null() {
            (!((*lp).r_tree).is_null()
                || {
                    glp_assert_(
                        b"lp->r_tree != NULL\0" as *const u8 as *const i8,
                        b"api/prob1.c\0" as *const u8 as *const i8,
                        436 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            _glp_avl_delete_node((*lp).r_tree, (*row).node);
            (*row).node = 0 as *mut AVLNODE;
        }
        _glp_dmp_free_atom(
            (*lp).pool,
            (*row).name as *mut libc::c_void,
            (strlen((*row).name)).wrapping_add(1 as i32 as u64) as i32,
        );
        (*row).name = 0 as *mut i8;
    }
    if !(name.is_null() || *name.offset(0 as i32 as isize) as i32 == '\0' as i32) {
        let mut k: i32 = 0;
        k = 0 as i32;
        while *name.offset(k as isize) as i32 != '\0' as i32 {
            if k == 256 as i32 {
                (glp_error_(b"api/prob1.c\0" as *const u8 as *const i8, 447 as i32))
                    .expect(
                        "non-null function pointer",
                    )(
                    b"glp_set_row_name: i = %d; row name too long\n\0" as *const u8
                        as *const i8,
                    i,
                );
            }
            if *(*__ctype_b_loc()).offset(*name.offset(k as isize) as u8 as i32 as isize)
                as i32 & _IScntrl as i32 as libc::c_ushort as i32 != 0
            {
                (glp_error_(b"api/prob1.c\0" as *const u8 as *const i8, 450 as i32))
                    .expect(
                        "non-null function pointer",
                    )(
                    b"glp_set_row_name: i = %d: row name contains invalid character(s)\n\0"
                        as *const u8 as *const i8,
                    i,
                );
            }
            k += 1;
            k;
        }
        (*row).name = _glp_dmp_get_atom(
            (*lp).pool,
            (strlen(name)).wrapping_add(1 as i32 as u64) as i32,
        ) as *mut i8;
        strcpy((*row).name, name);
        if !((*lp).r_tree).is_null() {
            (((*row).node).is_null()
                || {
                    glp_assert_(
                        b"row->node == NULL\0" as *const u8 as *const i8,
                        b"api/prob1.c\0" as *const u8 as *const i8,
                        456 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            (*row).node = _glp_avl_insert_node(
                (*lp).r_tree,
                (*row).name as *const libc::c_void,
            );
            _glp_avl_set_node_link((*row).node, row as *mut libc::c_void);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn glp_set_col_name(
    mut lp: *mut glp_prob,
    mut j: i32,
    mut name: *const i8,
) {
    let mut tree: *mut glp_tree = (*lp).tree;
    let mut col: *mut GLPCOL = 0 as *mut GLPCOL;
    if !tree.is_null() && (*tree).reason != 0 as i32 {
        (glp_error_(b"api/prob1.c\0" as *const u8 as *const i8, 486 as i32))
            .expect(
                "non-null function pointer",
            )(b"glp_set_col_name: operation not allowed\n\0" as *const u8 as *const i8);
    }
    if !(1 as i32 <= j && j <= (*lp).n) {
        (glp_error_(b"api/prob1.c\0" as *const u8 as *const i8, 488 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_set_col_name: j = %d; column number out of range\n\0" as *const u8
                as *const i8,
            j,
        );
    }
    col = *((*lp).col).offset(j as isize);
    if !((*col).name).is_null() {
        if !((*col).node).is_null() {
            (!((*lp).c_tree).is_null()
                || {
                    glp_assert_(
                        b"lp->c_tree != NULL\0" as *const u8 as *const i8,
                        b"api/prob1.c\0" as *const u8 as *const i8,
                        493 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            _glp_avl_delete_node((*lp).c_tree, (*col).node);
            (*col).node = 0 as *mut AVLNODE;
        }
        _glp_dmp_free_atom(
            (*lp).pool,
            (*col).name as *mut libc::c_void,
            (strlen((*col).name)).wrapping_add(1 as i32 as u64) as i32,
        );
        (*col).name = 0 as *mut i8;
    }
    if !(name.is_null() || *name.offset(0 as i32 as isize) as i32 == '\0' as i32) {
        let mut k: i32 = 0;
        k = 0 as i32;
        while *name.offset(k as isize) as i32 != '\0' as i32 {
            if k == 256 as i32 {
                (glp_error_(b"api/prob1.c\0" as *const u8 as *const i8, 504 as i32))
                    .expect(
                        "non-null function pointer",
                    )(
                    b"glp_set_col_name: j = %d; column name too long\n\0" as *const u8
                        as *const i8,
                    j,
                );
            }
            if *(*__ctype_b_loc()).offset(*name.offset(k as isize) as u8 as i32 as isize)
                as i32 & _IScntrl as i32 as libc::c_ushort as i32 != 0
            {
                (glp_error_(b"api/prob1.c\0" as *const u8 as *const i8, 507 as i32))
                    .expect(
                        "non-null function pointer",
                    )(
                    b"glp_set_col_name: j = %d: column name contains invalid character(s)\n\0"
                        as *const u8 as *const i8,
                    j,
                );
            }
            k += 1;
            k;
        }
        (*col).name = _glp_dmp_get_atom(
            (*lp).pool,
            (strlen(name)).wrapping_add(1 as i32 as u64) as i32,
        ) as *mut i8;
        strcpy((*col).name, name);
        if !((*lp).c_tree).is_null() && !((*col).name).is_null() {
            (((*col).node).is_null()
                || {
                    glp_assert_(
                        b"col->node == NULL\0" as *const u8 as *const i8,
                        b"api/prob1.c\0" as *const u8 as *const i8,
                        513 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            (*col).node = _glp_avl_insert_node(
                (*lp).c_tree,
                (*col).name as *const libc::c_void,
            );
            _glp_avl_set_node_link((*col).node, col as *mut libc::c_void);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn glp_set_row_bnds(
    mut lp: *mut glp_prob,
    mut i: i32,
    mut type_0: i32,
    mut lb: libc::c_double,
    mut ub: libc::c_double,
) {
    let mut row: *mut GLPROW = 0 as *mut GLPROW;
    if !(1 as i32 <= i && i <= (*lp).m) {
        (glp_error_(b"api/prob1.c\0" as *const u8 as *const i8, 559 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_set_row_bnds: i = %d; row number out of range\n\0" as *const u8
                as *const i8,
            i,
        );
    }
    row = *((*lp).row).offset(i as isize);
    (*row).type_0 = type_0;
    match type_0 {
        1 => {
            (*row).ub = 0.0f64;
            (*row).lb = (*row).ub;
            if (*row).stat != 1 as i32 {
                (*row).stat = 4 as i32;
            }
        }
        2 => {
            (*row).lb = lb;
            (*row).ub = 0.0f64;
            if (*row).stat != 1 as i32 {
                (*row).stat = 2 as i32;
            }
        }
        3 => {
            (*row).lb = 0.0f64;
            (*row).ub = ub;
            if (*row).stat != 1 as i32 {
                (*row).stat = 3 as i32;
            }
        }
        4 => {
            (*row).lb = lb;
            (*row).ub = ub;
            if !((*row).stat == 1 as i32 || (*row).stat == 2 as i32
                || (*row).stat == 3 as i32)
            {
                (*row).stat = if fabs(lb) <= fabs(ub) { 2 as i32 } else { 3 as i32 };
            }
        }
        5 => {
            (*row).ub = lb;
            (*row).lb = (*row).ub;
            if (*row).stat != 1 as i32 {
                (*row).stat = 5 as i32;
            }
        }
        _ => {
            (glp_error_(b"api/prob1.c\0" as *const u8 as *const i8, 587 as i32))
                .expect(
                    "non-null function pointer",
                )(
                b"glp_set_row_bnds: i = %d; type = %d; invalid row type\n\0" as *const u8
                    as *const i8,
                i,
                type_0,
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn glp_set_col_bnds(
    mut lp: *mut glp_prob,
    mut j: i32,
    mut type_0: i32,
    mut lb: libc::c_double,
    mut ub: libc::c_double,
) {
    let mut col: *mut GLPCOL = 0 as *mut GLPCOL;
    if !(1 as i32 <= j && j <= (*lp).n) {
        (glp_error_(b"api/prob1.c\0" as *const u8 as *const i8, 630 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_set_col_bnds: j = %d; column number out of range\n\0" as *const u8
                as *const i8,
            j,
        );
    }
    col = *((*lp).col).offset(j as isize);
    (*col).type_0 = type_0;
    match type_0 {
        1 => {
            (*col).ub = 0.0f64;
            (*col).lb = (*col).ub;
            if (*col).stat != 1 as i32 {
                (*col).stat = 4 as i32;
            }
        }
        2 => {
            (*col).lb = lb;
            (*col).ub = 0.0f64;
            if (*col).stat != 1 as i32 {
                (*col).stat = 2 as i32;
            }
        }
        3 => {
            (*col).lb = 0.0f64;
            (*col).ub = ub;
            if (*col).stat != 1 as i32 {
                (*col).stat = 3 as i32;
            }
        }
        4 => {
            (*col).lb = lb;
            (*col).ub = ub;
            if !((*col).stat == 1 as i32 || (*col).stat == 2 as i32
                || (*col).stat == 3 as i32)
            {
                (*col).stat = if fabs(lb) <= fabs(ub) { 2 as i32 } else { 3 as i32 };
            }
        }
        5 => {
            (*col).ub = lb;
            (*col).lb = (*col).ub;
            if (*col).stat != 1 as i32 {
                (*col).stat = 5 as i32;
            }
        }
        _ => {
            (glp_error_(b"api/prob1.c\0" as *const u8 as *const i8, 658 as i32))
                .expect(
                    "non-null function pointer",
                )(
                b"glp_set_col_bnds: j = %d; type = %d; invalid column type\n\0"
                    as *const u8 as *const i8,
                j,
                type_0,
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn glp_set_obj_coef(
    mut lp: *mut glp_prob,
    mut j: i32,
    mut coef: libc::c_double,
) {
    let mut tree: *mut glp_tree = (*lp).tree;
    if !tree.is_null() && (*tree).reason != 0 as i32 {
        (glp_error_(b"api/prob1.c\0" as *const u8 as *const i8, 684 as i32))
            .expect(
                "non-null function pointer",
            )(b"glp_set_obj_coef: operation not allowed\n\0" as *const u8 as *const i8);
    }
    if !(0 as i32 <= j && j <= (*lp).n) {
        (glp_error_(b"api/prob1.c\0" as *const u8 as *const i8, 686 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_set_obj_coef: j = %d; column number out of range\n\0" as *const u8
                as *const i8,
            j,
        );
    }
    if j == 0 as i32 {
        (*lp).c0 = coef;
    } else {
        (**((*lp).col).offset(j as isize)).coef = coef;
    };
}
#[no_mangle]
pub unsafe extern "C" fn glp_set_mat_row(
    mut lp: *mut glp_prob,
    mut i: i32,
    mut len: i32,
    mut ind: *const i32,
    mut val: *const libc::c_double,
) {
    let mut tree: *mut glp_tree = (*lp).tree;
    let mut row: *mut GLPROW = 0 as *mut GLPROW;
    let mut col: *mut GLPCOL = 0 as *mut GLPCOL;
    let mut aij: *mut GLPAIJ = 0 as *mut GLPAIJ;
    let mut next: *mut GLPAIJ = 0 as *mut GLPAIJ;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    if !(1 as i32 <= i && i <= (*lp).m) {
        (glp_error_(b"api/prob1.c\0" as *const u8 as *const i8, 729 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_set_mat_row: i = %d; row number out of range\n\0" as *const u8
                as *const i8,
            i,
        );
    }
    row = *((*lp).row).offset(i as isize);
    if !tree.is_null() && (*tree).reason != 0 as i32 {
        (!((*tree).curr).is_null()
            || {
                glp_assert_(
                    b"tree->curr != NULL\0" as *const u8 as *const i8,
                    b"api/prob1.c\0" as *const u8 as *const i8,
                    733 as i32,
                );
                1 as i32 != 0
            }) as i32;
        ((*row).level == (*(*tree).curr).level
            || {
                glp_assert_(
                    b"row->level == tree->curr->level\0" as *const u8 as *const i8,
                    b"api/prob1.c\0" as *const u8 as *const i8,
                    734 as i32,
                );
                1 as i32 != 0
            }) as i32;
    }
    while !((*row).ptr).is_null() {
        aij = (*row).ptr;
        (*row).ptr = (*aij).r_next;
        col = (*aij).col;
        if ((*aij).c_prev).is_null() {
            (*col).ptr = (*aij).c_next;
        } else {
            (*(*aij).c_prev).c_next = (*aij).c_next;
        }
        if !((*aij).c_next).is_null() {
            (*(*aij).c_next).c_prev = (*aij).c_prev;
        }
        _glp_dmp_free_atom(
            (*lp).pool,
            aij as *mut libc::c_void,
            ::core::mem::size_of::<GLPAIJ>() as u64 as i32,
        );
        (*lp).nnz -= 1;
        (*lp).nnz;
        if (*col).stat == 1 as i32 {
            (*lp).valid = 0 as i32;
        }
    }
    if !(0 as i32 <= len && len <= (*lp).n) {
        (glp_error_(b"api/prob1.c\0" as *const u8 as *const i8, 761 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_set_mat_row: i = %d; len = %d; invalid row length \n\0" as *const u8
                as *const i8,
            i,
            len,
        );
    }
    if len > 500000000 as i32 - (*lp).nnz {
        (glp_error_(b"api/prob1.c\0" as *const u8 as *const i8, 764 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_set_mat_row: i = %d; len = %d; too many constraint coefficients\n\0"
                as *const u8 as *const i8,
            i,
            len,
        );
    }
    k = 1 as i32;
    while k <= len {
        j = *ind.offset(k as isize);
        if !(1 as i32 <= j && j <= (*lp).n) {
            (glp_error_(b"api/prob1.c\0" as *const u8 as *const i8, 771 as i32))
                .expect(
                    "non-null function pointer",
                )(
                b"glp_set_mat_row: i = %d; ind[%d] = %d; column index out of range\n\0"
                    as *const u8 as *const i8,
                i,
                k,
                j,
            );
        }
        col = *((*lp).col).offset(j as isize);
        if !((*col).ptr).is_null() && (*(*(*col).ptr).row).i == i {
            (glp_error_(b"api/prob1.c\0" as *const u8 as *const i8, 777 as i32))
                .expect(
                    "non-null function pointer",
                )(
                b"glp_set_mat_row: i = %d; ind[%d] = %d; duplicate column indices not allowed\n\0"
                    as *const u8 as *const i8,
                i,
                k,
                j,
            );
        }
        aij = _glp_dmp_get_atom(
            (*lp).pool,
            ::core::mem::size_of::<GLPAIJ>() as u64 as i32,
        ) as *mut GLPAIJ;
        (*lp).nnz += 1;
        (*lp).nnz;
        (*aij).row = row;
        (*aij).col = col;
        (*aij).val = *val.offset(k as isize);
        (*aij).r_prev = 0 as *mut GLPAIJ;
        (*aij).r_next = (*row).ptr;
        (*aij).c_prev = 0 as *mut GLPAIJ;
        (*aij).c_next = (*col).ptr;
        if !((*aij).r_next).is_null() {
            (*(*aij).r_next).r_prev = aij;
        }
        if !((*aij).c_next).is_null() {
            (*(*aij).c_next).c_prev = aij;
        }
        (*col).ptr = aij;
        (*row).ptr = (*col).ptr;
        if (*col).stat == 1 as i32 && (*aij).val != 0.0f64 {
            (*lp).valid = 0 as i32;
        }
        k += 1;
        k;
    }
    aij = (*row).ptr;
    while !aij.is_null() {
        next = (*aij).r_next;
        if (*aij).val == 0.0f64 {
            if ((*aij).r_prev).is_null() {
                (*row).ptr = next;
            } else {
                (*(*aij).r_prev).r_next = next;
            }
            if !next.is_null() {
                (*next).r_prev = (*aij).r_prev;
            }
            (((*aij).c_prev).is_null()
                || {
                    glp_assert_(
                        b"aij->c_prev == NULL\0" as *const u8 as *const i8,
                        b"api/prob1.c\0" as *const u8 as *const i8,
                        811 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            (*(*aij).col).ptr = (*aij).c_next;
            if !((*aij).c_next).is_null() {
                (*(*aij).c_next).c_prev = 0 as *mut GLPAIJ;
            }
            _glp_dmp_free_atom(
                (*lp).pool,
                aij as *mut libc::c_void,
                ::core::mem::size_of::<GLPAIJ>() as u64 as i32,
            );
            (*lp).nnz -= 1;
            (*lp).nnz;
        }
        aij = next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn glp_set_mat_col(
    mut lp: *mut glp_prob,
    mut j: i32,
    mut len: i32,
    mut ind: *const i32,
    mut val: *const libc::c_double,
) {
    let mut tree: *mut glp_tree = (*lp).tree;
    let mut row: *mut GLPROW = 0 as *mut GLPROW;
    let mut col: *mut GLPCOL = 0 as *mut GLPCOL;
    let mut aij: *mut GLPAIJ = 0 as *mut GLPAIJ;
    let mut next: *mut GLPAIJ = 0 as *mut GLPAIJ;
    let mut i: i32 = 0;
    let mut k: i32 = 0;
    if !tree.is_null() && (*tree).reason != 0 as i32 {
        (glp_error_(b"api/prob1.c\0" as *const u8 as *const i8, 854 as i32))
            .expect(
                "non-null function pointer",
            )(b"glp_set_mat_col: operation not allowed\n\0" as *const u8 as *const i8);
    }
    if !(1 as i32 <= j && j <= (*lp).n) {
        (glp_error_(b"api/prob1.c\0" as *const u8 as *const i8, 857 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_set_mat_col: j = %d; column number out of range\n\0" as *const u8
                as *const i8,
            j,
        );
    }
    col = *((*lp).col).offset(j as isize);
    while !((*col).ptr).is_null() {
        aij = (*col).ptr;
        (*col).ptr = (*aij).c_next;
        row = (*aij).row;
        if ((*aij).r_prev).is_null() {
            (*row).ptr = (*aij).r_next;
        } else {
            (*(*aij).r_prev).r_next = (*aij).r_next;
        }
        if !((*aij).r_next).is_null() {
            (*(*aij).r_next).r_prev = (*aij).r_prev;
        }
        _glp_dmp_free_atom(
            (*lp).pool,
            aij as *mut libc::c_void,
            ::core::mem::size_of::<GLPAIJ>() as u64 as i32,
        );
        (*lp).nnz -= 1;
        (*lp).nnz;
    }
    if !(0 as i32 <= len && len <= (*lp).m) {
        (glp_error_(b"api/prob1.c\0" as *const u8 as *const i8, 882 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_set_mat_col: j = %d; len = %d; invalid column length\n\0" as *const u8
                as *const i8,
            j,
            len,
        );
    }
    if len > 500000000 as i32 - (*lp).nnz {
        (glp_error_(b"api/prob1.c\0" as *const u8 as *const i8, 885 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_set_mat_col: j = %d; len = %d; too many constraint coefficients\n\0"
                as *const u8 as *const i8,
            j,
            len,
        );
    }
    k = 1 as i32;
    while k <= len {
        i = *ind.offset(k as isize);
        if !(1 as i32 <= i && i <= (*lp).m) {
            (glp_error_(b"api/prob1.c\0" as *const u8 as *const i8, 892 as i32))
                .expect(
                    "non-null function pointer",
                )(
                b"glp_set_mat_col: j = %d; ind[%d] = %d; row index out of range\n\0"
                    as *const u8 as *const i8,
                j,
                k,
                i,
            );
        }
        row = *((*lp).row).offset(i as isize);
        if !((*row).ptr).is_null() && (*(*(*row).ptr).col).j == j {
            (glp_error_(b"api/prob1.c\0" as *const u8 as *const i8, 898 as i32))
                .expect(
                    "non-null function pointer",
                )(
                b"glp_set_mat_col: j = %d; ind[%d] = %d; duplicate row indices not allowed\n\0"
                    as *const u8 as *const i8,
                j,
                k,
                i,
            );
        }
        aij = _glp_dmp_get_atom(
            (*lp).pool,
            ::core::mem::size_of::<GLPAIJ>() as u64 as i32,
        ) as *mut GLPAIJ;
        (*lp).nnz += 1;
        (*lp).nnz;
        (*aij).row = row;
        (*aij).col = col;
        (*aij).val = *val.offset(k as isize);
        (*aij).r_prev = 0 as *mut GLPAIJ;
        (*aij).r_next = (*row).ptr;
        (*aij).c_prev = 0 as *mut GLPAIJ;
        (*aij).c_next = (*col).ptr;
        if !((*aij).r_next).is_null() {
            (*(*aij).r_next).r_prev = aij;
        }
        if !((*aij).c_next).is_null() {
            (*(*aij).c_next).c_prev = aij;
        }
        (*col).ptr = aij;
        (*row).ptr = (*col).ptr;
        k += 1;
        k;
    }
    aij = (*col).ptr;
    while !aij.is_null() {
        next = (*aij).c_next;
        if (*aij).val == 0.0f64 {
            (((*aij).r_prev).is_null()
                || {
                    glp_assert_(
                        b"aij->r_prev == NULL\0" as *const u8 as *const i8,
                        b"api/prob1.c\0" as *const u8 as *const i8,
                        920 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            (*(*aij).row).ptr = (*aij).r_next;
            if !((*aij).r_next).is_null() {
                (*(*aij).r_next).r_prev = 0 as *mut GLPAIJ;
            }
            if ((*aij).c_prev).is_null() {
                (*col).ptr = next;
            } else {
                (*(*aij).c_prev).c_next = next;
            }
            if !next.is_null() {
                (*next).c_prev = (*aij).c_prev;
            }
            _glp_dmp_free_atom(
                (*lp).pool,
                aij as *mut libc::c_void,
                ::core::mem::size_of::<GLPAIJ>() as u64 as i32,
            );
            (*lp).nnz -= 1;
            (*lp).nnz;
        }
        aij = next;
    }
    if (*col).stat == 1 as i32 {
        (*lp).valid = 0 as i32;
    }
}
#[no_mangle]
pub unsafe extern "C" fn glp_load_matrix(
    mut lp: *mut glp_prob,
    mut ne: i32,
    mut ia: *const i32,
    mut ja: *const i32,
    mut ar: *const libc::c_double,
) {
    let mut tree: *mut glp_tree = (*lp).tree;
    let mut row: *mut GLPROW = 0 as *mut GLPROW;
    let mut col: *mut GLPCOL = 0 as *mut GLPCOL;
    let mut aij: *mut GLPAIJ = 0 as *mut GLPAIJ;
    let mut next: *mut GLPAIJ = 0 as *mut GLPAIJ;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    if !tree.is_null() && (*tree).reason != 0 as i32 {
        (glp_error_(b"api/prob1.c\0" as *const u8 as *const i8, 977 as i32))
            .expect(
                "non-null function pointer",
            )(b"glp_load_matrix: operation not allowed\n\0" as *const u8 as *const i8);
    }
    i = 1 as i32;
    while i <= (*lp).m {
        row = *((*lp).row).offset(i as isize);
        while !((*row).ptr).is_null() {
            aij = (*row).ptr;
            (*row).ptr = (*aij).r_next;
            _glp_dmp_free_atom(
                (*lp).pool,
                aij as *mut libc::c_void,
                ::core::mem::size_of::<GLPAIJ>() as u64 as i32,
            );
            (*lp).nnz -= 1;
            (*lp).nnz;
        }
        i += 1;
        i;
    }
    ((*lp).nnz == 0 as i32
        || {
            glp_assert_(
                b"lp->nnz == 0\0" as *const u8 as *const i8,
                b"api/prob1.c\0" as *const u8 as *const i8,
                987 as i32,
            );
            1 as i32 != 0
        }) as i32;
    j = 1 as i32;
    while j <= (*lp).n {
        let ref mut fresh2 = (**((*lp).col).offset(j as isize)).ptr;
        *fresh2 = 0 as *mut GLPAIJ;
        j += 1;
        j;
    }
    if ne < 0 as i32 {
        (glp_error_(b"api/prob1.c\0" as *const u8 as *const i8, 992 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_load_matrix: ne = %d; invalid number of constraint coefficients\n\0"
                as *const u8 as *const i8,
            ne,
        );
    }
    if ne > 500000000 as i32 {
        (glp_error_(b"api/prob1.c\0" as *const u8 as *const i8, 995 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_load_matrix: ne = %d; too many constraint coefficients\n\0"
                as *const u8 as *const i8,
            ne,
        );
    }
    k = 1 as i32;
    while k <= ne {
        i = *ia.offset(k as isize);
        j = *ja.offset(k as isize);
        if !(1 as i32 <= i && i <= (*lp).m) {
            (glp_error_(b"api/prob1.c\0" as *const u8 as *const i8, 1002 as i32))
                .expect(
                    "non-null function pointer",
                )(
                b"glp_load_matrix: ia[%d] = %d; row index out of range\n\0" as *const u8
                    as *const i8,
                k,
                i,
            );
        }
        row = *((*lp).row).offset(i as isize);
        if !(1 as i32 <= j && j <= (*lp).n) {
            (glp_error_(b"api/prob1.c\0" as *const u8 as *const i8, 1007 as i32))
                .expect(
                    "non-null function pointer",
                )(
                b"glp_load_matrix: ja[%d] = %d; column index out of range\n\0"
                    as *const u8 as *const i8,
                k,
                j,
            );
        }
        col = *((*lp).col).offset(j as isize);
        aij = _glp_dmp_get_atom(
            (*lp).pool,
            ::core::mem::size_of::<GLPAIJ>() as u64 as i32,
        ) as *mut GLPAIJ;
        (*lp).nnz += 1;
        (*lp).nnz;
        (*aij).row = row;
        (*aij).col = col;
        (*aij).val = *ar.offset(k as isize);
        (*aij).r_prev = 0 as *mut GLPAIJ;
        (*aij).r_next = (*row).ptr;
        if !((*aij).r_next).is_null() {
            (*(*aij).r_next).r_prev = aij;
        }
        (*row).ptr = aij;
        k += 1;
        k;
    }
    ((*lp).nnz == ne
        || {
            glp_assert_(
                b"lp->nnz == ne\0" as *const u8 as *const i8,
                b"api/prob1.c\0" as *const u8 as *const i8,
                1021 as i32,
            );
            1 as i32 != 0
        }) as i32;
    i = 1 as i32;
    while i <= (*lp).m {
        aij = (**((*lp).row).offset(i as isize)).ptr;
        while !aij.is_null() {
            col = (*aij).col;
            if !((*col).ptr).is_null() && (*(*(*col).ptr).row).i == i {
                k = 1 as i32;
                while k <= ne {
                    if *ia.offset(k as isize) == i && *ja.offset(k as isize) == (*col).j
                    {
                        break;
                    }
                    k += 1;
                    k;
                }
                (glp_error_(b"api/prob1.c\0" as *const u8 as *const i8, 1033 as i32))
                    .expect(
                        "non-null function pointer",
                    )(
                    b"glp_load_mat: ia[%d] = %d; ja[%d] = %d; duplicate indices not allowed\n\0"
                        as *const u8 as *const i8,
                    k,
                    i,
                    k,
                    (*col).j,
                );
            }
            (*aij).c_prev = 0 as *mut GLPAIJ;
            (*aij).c_next = (*col).ptr;
            if !((*aij).c_next).is_null() {
                (*(*aij).c_next).c_prev = aij;
            }
            (*col).ptr = aij;
            aij = (*aij).r_next;
        }
        i += 1;
        i;
    }
    i = 1 as i32;
    while i <= (*lp).m {
        row = *((*lp).row).offset(i as isize);
        aij = (*row).ptr;
        while !aij.is_null() {
            next = (*aij).r_next;
            if (*aij).val == 0.0f64 {
                if ((*aij).r_prev).is_null() {
                    (*row).ptr = next;
                } else {
                    (*(*aij).r_prev).r_next = next;
                }
                if !next.is_null() {
                    (*next).r_prev = (*aij).r_prev;
                }
                if ((*aij).c_prev).is_null() {
                    (*(*aij).col).ptr = (*aij).c_next;
                } else {
                    (*(*aij).c_prev).c_next = (*aij).c_next;
                }
                if !((*aij).c_next).is_null() {
                    (*(*aij).c_next).c_prev = (*aij).c_prev;
                }
                _glp_dmp_free_atom(
                    (*lp).pool,
                    aij as *mut libc::c_void,
                    ::core::mem::size_of::<GLPAIJ>() as u64 as i32,
                );
                (*lp).nnz -= 1;
                (*lp).nnz;
            }
            aij = next;
        }
        i += 1;
        i;
    }
    (*lp).valid = 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn glp_check_dup(
    mut m: i32,
    mut n: i32,
    mut ne: i32,
    mut ia: *const i32,
    mut ja: *const i32,
) -> i32 {
    let mut current_block: u64;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut ptr: *mut i32 = 0 as *mut i32;
    let mut next: *mut i32 = 0 as *mut i32;
    let mut ret: i32 = 0;
    let mut flag: *mut i8 = 0 as *mut i8;
    if m < 0 as i32 {
        (glp_error_(b"api/prob1.c\0" as *const u8 as *const i8, 1120 as i32))
            .expect(
                "non-null function pointer",
            )(b"glp_check_dup: m = %d; invalid parameter\n\0" as *const u8 as *const i8);
    }
    if n < 0 as i32 {
        (glp_error_(b"api/prob1.c\0" as *const u8 as *const i8, 1122 as i32))
            .expect(
                "non-null function pointer",
            )(b"glp_check_dup: n = %d; invalid parameter\n\0" as *const u8 as *const i8);
    }
    if ne < 0 as i32 {
        (glp_error_(b"api/prob1.c\0" as *const u8 as *const i8, 1124 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_check_dup: ne = %d; invalid parameter\n\0" as *const u8 as *const i8,
        );
    }
    if ne > 0 as i32 && ia.is_null() {
        (glp_error_(b"api/prob1.c\0" as *const u8 as *const i8, 1126 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_check_dup: ia = %p; invalid parameter\n\0" as *const u8 as *const i8,
            ia,
        );
    }
    if ne > 0 as i32 && ja.is_null() {
        (glp_error_(b"api/prob1.c\0" as *const u8 as *const i8, 1128 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_check_dup: ja = %p; invalid parameter\n\0" as *const u8 as *const i8,
            ja,
        );
    }
    k = 1 as i32;
    loop {
        if !(k <= ne) {
            current_block = 4166486009154926805;
            break;
        }
        i = *ia.offset(k as isize);
        j = *ja.offset(k as isize);
        if !(1 as i32 <= i && i <= m && 1 as i32 <= j && j <= n) {
            ret = -k;
            current_block = 2565565653680186462;
            break;
        } else {
            k += 1;
            k;
        }
    }
    match current_block {
        4166486009154926805 => {
            if m == 0 as i32 || n == 0 as i32 {
                ret = 0 as i32;
            } else {
                ptr = glp_alloc(
                    1 as i32 + m,
                    ::core::mem::size_of::<i32>() as u64 as i32,
                ) as *mut i32;
                next = glp_alloc(
                    1 as i32 + ne,
                    ::core::mem::size_of::<i32>() as u64 as i32,
                ) as *mut i32;
                flag = glp_alloc(
                    1 as i32 + n,
                    ::core::mem::size_of::<i8>() as u64 as i32,
                ) as *mut i8;
                i = 1 as i32;
                while i <= m {
                    *ptr.offset(i as isize) = 0 as i32;
                    i += 1;
                    i;
                }
                k = 1 as i32;
                while k <= ne {
                    i = *ia.offset(k as isize);
                    *next.offset(k as isize) = *ptr.offset(i as isize);
                    *ptr.offset(i as isize) = k;
                    k += 1;
                    k;
                }
                j = 1 as i32;
                while j <= n {
                    *flag.offset(j as isize) = 0 as i32 as i8;
                    j += 1;
                    j;
                }
                i = 1 as i32;
                's_143: loop {
                    if !(i <= m) {
                        current_block = 3160140712158701372;
                        break;
                    }
                    k = *ptr.offset(i as isize);
                    while k != 0 as i32 {
                        j = *ja.offset(k as isize);
                        if *flag.offset(j as isize) != 0 {
                            k = 1 as i32;
                            while k <= ne {
                                if *ia.offset(k as isize) == i
                                    && *ja.offset(k as isize) == j
                                {
                                    break;
                                }
                                k += 1;
                                k;
                            }
                            (k <= ne
                                || {
                                    glp_assert_(
                                        b"k <= ne\0" as *const u8 as *const i8,
                                        b"api/prob1.c\0" as *const u8 as *const i8,
                                        1163 as i32,
                                    );
                                    1 as i32 != 0
                                }) as i32;
                            k += 1;
                            k;
                            while k <= ne {
                                if *ia.offset(k as isize) == i
                                    && *ja.offset(k as isize) == j
                                {
                                    break;
                                }
                                k += 1;
                                k;
                            }
                            (k <= ne
                                || {
                                    glp_assert_(
                                        b"k <= ne\0" as *const u8 as *const i8,
                                        b"api/prob1.c\0" as *const u8 as *const i8,
                                        1167 as i32,
                                    );
                                    1 as i32 != 0
                                }) as i32;
                            ret = k;
                            current_block = 18324072715605596488;
                            break 's_143;
                        } else {
                            *flag.offset(j as isize) = 1 as i32 as i8;
                            k = *next.offset(k as isize);
                        }
                    }
                    k = *ptr.offset(i as isize);
                    while k != 0 as i32 {
                        *flag.offset(*ja.offset(k as isize) as isize) = 0 as i32 as i8;
                        k = *next.offset(k as isize);
                    }
                    i += 1;
                    i;
                }
                match current_block {
                    3160140712158701372 => {
                        ret = 0 as i32;
                    }
                    _ => {}
                }
                glp_free(ptr as *mut libc::c_void);
                glp_free(next as *mut libc::c_void);
                glp_free(flag as *mut libc::c_void);
            }
        }
        _ => {}
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn glp_sort_matrix(mut P: *mut glp_prob) {
    let mut aij: *mut GLPAIJ = 0 as *mut GLPAIJ;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    i = (*P).m;
    while i >= 1 as i32 {
        let ref mut fresh3 = (**((*P).row).offset(i as isize)).ptr;
        *fresh3 = 0 as *mut GLPAIJ;
        i -= 1;
        i;
    }
    j = (*P).n;
    while j >= 1 as i32 {
        aij = (**((*P).col).offset(j as isize)).ptr;
        while !aij.is_null() {
            i = (*(*aij).row).i;
            (*aij).r_prev = 0 as *mut GLPAIJ;
            (*aij).r_next = (**((*P).row).offset(i as isize)).ptr;
            if !((*aij).r_next).is_null() {
                (*(*aij).r_next).r_prev = aij;
            }
            let ref mut fresh4 = (**((*P).row).offset(i as isize)).ptr;
            *fresh4 = aij;
            aij = (*aij).c_next;
        }
        j -= 1;
        j;
    }
    j = (*P).n;
    while j >= 1 as i32 {
        let ref mut fresh5 = (**((*P).col).offset(j as isize)).ptr;
        *fresh5 = 0 as *mut GLPAIJ;
        j -= 1;
        j;
    }
    i = (*P).m;
    while i >= 1 as i32 {
        aij = (**((*P).row).offset(i as isize)).ptr;
        while !aij.is_null() {
            j = (*(*aij).col).j;
            (*aij).c_prev = 0 as *mut GLPAIJ;
            (*aij).c_next = (**((*P).col).offset(j as isize)).ptr;
            if !((*aij).c_next).is_null() {
                (*(*aij).c_next).c_prev = aij;
            }
            let ref mut fresh6 = (**((*P).col).offset(j as isize)).ptr;
            *fresh6 = aij;
            aij = (*aij).r_next;
        }
        i -= 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn glp_del_rows(
    mut lp: *mut glp_prob,
    mut nrs: i32,
    mut num: *const i32,
) {
    let mut tree: *mut glp_tree = (*lp).tree;
    let mut row: *mut GLPROW = 0 as *mut GLPROW;
    let mut i: i32 = 0;
    let mut k: i32 = 0;
    let mut m_new: i32 = 0;
    if !(1 as i32 <= nrs && nrs <= (*lp).m) {
        (glp_error_(b"api/prob1.c\0" as *const u8 as *const i8, 1265 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_del_rows: nrs = %d; invalid number of rows\n\0" as *const u8
                as *const i8,
            nrs,
        );
    }
    k = 1 as i32;
    while k <= nrs {
        i = *num.offset(k as isize);
        if !(1 as i32 <= i && i <= (*lp).m) {
            (glp_error_(b"api/prob1.c\0" as *const u8 as *const i8, 1272 as i32))
                .expect(
                    "non-null function pointer",
                )(
                b"glp_del_rows: num[%d] = %d; row number out of range\n\0" as *const u8
                    as *const i8,
                k,
                i,
            );
        }
        row = *((*lp).row).offset(i as isize);
        if !tree.is_null() && (*tree).reason != 0 as i32 {
            if !((*tree).reason == 0x1 as i32 || (*tree).reason == 0x4 as i32) {
                (glp_error_(b"api/prob1.c\0" as *const u8 as *const i8, 1278 as i32))
                    .expect(
                        "non-null function pointer",
                    )(
                    b"glp_del_rows: operation not allowed\n\0" as *const u8 as *const i8,
                );
            }
            (!((*tree).curr).is_null()
                || {
                    glp_assert_(
                        b"tree->curr != NULL\0" as *const u8 as *const i8,
                        b"api/prob1.c\0" as *const u8 as *const i8,
                        1279 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            if (*row).level != (*(*tree).curr).level {
                (glp_error_(b"api/prob1.c\0" as *const u8 as *const i8, 1281 as i32))
                    .expect(
                        "non-null function pointer",
                    )(
                    b"glp_del_rows: num[%d] = %d; invalid attempt to delete row created not in current subproblem\n\0"
                        as *const u8 as *const i8,
                    k,
                    i,
                );
            }
            if (*row).stat != 1 as i32 {
                (glp_error_(b"api/prob1.c\0" as *const u8 as *const i8, 1284 as i32))
                    .expect(
                        "non-null function pointer",
                    )(
                    b"glp_del_rows: num[%d] = %d; invalid attempt to delete active row (constraint)\n\0"
                        as *const u8 as *const i8,
                    k,
                    i,
                );
            }
            (*tree).reinv = 1 as i32;
        }
        if (*row).i == 0 as i32 {
            (glp_error_(b"api/prob1.c\0" as *const u8 as *const i8, 1290 as i32))
                .expect(
                    "non-null function pointer",
                )(
                b"glp_del_rows: num[%d] = %d; duplicate row numbers not allowed\n\0"
                    as *const u8 as *const i8,
                k,
                i,
            );
        }
        glp_set_row_name(lp, i, 0 as *const i8);
        (((*row).node).is_null()
            || {
                glp_assert_(
                    b"row->node == NULL\0" as *const u8 as *const i8,
                    b"api/prob1.c\0" as *const u8 as *const i8,
                    1294 as i32,
                );
                1 as i32 != 0
            }) as i32;
        glp_set_mat_row(lp, i, 0 as i32, 0 as *const i32, 0 as *const libc::c_double);
        (((*row).ptr).is_null()
            || {
                glp_assert_(
                    b"row->ptr == NULL\0" as *const u8 as *const i8,
                    b"api/prob1.c\0" as *const u8 as *const i8,
                    1297 as i32,
                );
                1 as i32 != 0
            }) as i32;
        (*row).i = 0 as i32;
        k += 1;
        k;
    }
    m_new = 0 as i32;
    i = 1 as i32;
    while i <= (*lp).m {
        row = *((*lp).row).offset(i as isize);
        if (*row).i == 0 as i32 {
            _glp_dmp_free_atom(
                (*lp).pool,
                row as *mut libc::c_void,
                ::core::mem::size_of::<GLPROW>() as u64 as i32,
            );
        } else {
            m_new += 1;
            (*row).i = m_new;
            let ref mut fresh7 = *((*lp).row).offset((*row).i as isize);
            *fresh7 = row;
        }
        i += 1;
        i;
    }
    (*lp).m = m_new;
    (*lp).valid = 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn glp_del_cols(
    mut lp: *mut glp_prob,
    mut ncs: i32,
    mut num: *const i32,
) {
    let mut tree: *mut glp_tree = (*lp).tree;
    let mut col: *mut GLPCOL = 0 as *mut GLPCOL;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut n_new: i32 = 0;
    if !tree.is_null() && (*tree).reason != 0 as i32 {
        (glp_error_(b"api/prob1.c\0" as *const u8 as *const i8, 1349 as i32))
            .expect(
                "non-null function pointer",
            )(b"glp_del_cols: operation not allowed\n\0" as *const u8 as *const i8);
    }
    if !(1 as i32 <= ncs && ncs <= (*lp).n) {
        (glp_error_(b"api/prob1.c\0" as *const u8 as *const i8, 1352 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_del_cols: ncs = %d; invalid number of columns\n\0" as *const u8
                as *const i8,
            ncs,
        );
    }
    k = 1 as i32;
    while k <= ncs {
        j = *num.offset(k as isize);
        if !(1 as i32 <= j && j <= (*lp).n) {
            (glp_error_(b"api/prob1.c\0" as *const u8 as *const i8, 1359 as i32))
                .expect(
                    "non-null function pointer",
                )(
                b"glp_del_cols: num[%d] = %d; column number out of range\0" as *const u8
                    as *const i8,
                k,
                j,
            );
        }
        col = *((*lp).col).offset(j as isize);
        if (*col).j == 0 as i32 {
            (glp_error_(b"api/prob1.c\0" as *const u8 as *const i8, 1364 as i32))
                .expect(
                    "non-null function pointer",
                )(
                b"glp_del_cols: num[%d] = %d; duplicate column numbers not allowed\n\0"
                    as *const u8 as *const i8,
                k,
                j,
            );
        }
        glp_set_col_name(lp, j, 0 as *const i8);
        (((*col).node).is_null()
            || {
                glp_assert_(
                    b"col->node == NULL\0" as *const u8 as *const i8,
                    b"api/prob1.c\0" as *const u8 as *const i8,
                    1368 as i32,
                );
                1 as i32 != 0
            }) as i32;
        glp_set_mat_col(lp, j, 0 as i32, 0 as *const i32, 0 as *const libc::c_double);
        (((*col).ptr).is_null()
            || {
                glp_assert_(
                    b"col->ptr == NULL\0" as *const u8 as *const i8,
                    b"api/prob1.c\0" as *const u8 as *const i8,
                    1371 as i32,
                );
                1 as i32 != 0
            }) as i32;
        (*col).j = 0 as i32;
        if (*col).stat == 1 as i32 {
            (*lp).valid = 0 as i32;
        }
        k += 1;
        k;
    }
    n_new = 0 as i32;
    j = 1 as i32;
    while j <= (*lp).n {
        col = *((*lp).col).offset(j as isize);
        if (*col).j == 0 as i32 {
            _glp_dmp_free_atom(
                (*lp).pool,
                col as *mut libc::c_void,
                ::core::mem::size_of::<GLPCOL>() as u64 as i32,
            );
        } else {
            n_new += 1;
            (*col).j = n_new;
            let ref mut fresh8 = *((*lp).col).offset((*col).j as isize);
            *fresh8 = col;
        }
        j += 1;
        j;
    }
    (*lp).n = n_new;
    if (*lp).valid != 0 {
        let mut m: i32 = (*lp).m;
        let mut head: *mut i32 = (*lp).head;
        j = 1 as i32;
        while j <= n_new {
            k = (**((*lp).col).offset(j as isize)).bind;
            if k != 0 as i32 {
                (1 as i32 <= k && k <= m
                    || {
                        glp_assert_(
                            b"1 <= k && k <= m\0" as *const u8 as *const i8,
                            b"api/prob1.c\0" as *const u8 as *const i8,
                            1402 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                *head.offset(k as isize) = m + j;
            }
            j += 1;
            j;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn glp_copy_prob(
    mut dest: *mut glp_prob,
    mut prob: *mut glp_prob,
    mut names: i32,
) {
    let mut tree: *mut glp_tree = (*dest).tree;
    let mut bfcp: glp_bfcp = glp_bfcp {
        msg_lev: 0,
        type_0: 0,
        lu_size: 0,
        piv_tol: 0.,
        piv_lim: 0,
        suhl: 0,
        eps_tol: 0.,
        max_gro: 0.,
        nfs_max: 0,
        upd_tol: 0.,
        nrs_max: 0,
        rs_size: 0,
        foo_bar: [0.; 38],
    };
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut len: i32 = 0;
    let mut ind: *mut i32 = 0 as *mut i32;
    let mut val: *mut libc::c_double = 0 as *mut libc::c_double;
    if !tree.is_null() && (*tree).reason != 0 as i32 {
        (glp_error_(b"api/prob1.c\0" as *const u8 as *const i8, 1434 as i32))
            .expect(
                "non-null function pointer",
            )(b"glp_copy_prob: operation not allowed\n\0" as *const u8 as *const i8);
    }
    if dest == prob {
        (glp_error_(b"api/prob1.c\0" as *const u8 as *const i8, 1436 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_copy_prob: copying problem object to itself not allowed\n\0"
                as *const u8 as *const i8,
        );
    }
    if !(names == 1 as i32 || names == 0 as i32) {
        (glp_error_(b"api/prob1.c\0" as *const u8 as *const i8, 1439 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_copy_prob: names = %d; invalid parameter\n\0" as *const u8
                as *const i8,
            names,
        );
    }
    glp_erase_prob(dest);
    if names != 0 && !((*prob).name).is_null() {
        glp_set_prob_name(dest, (*prob).name);
    }
    if names != 0 && !((*prob).obj).is_null() {
        glp_set_obj_name(dest, (*prob).obj);
    }
    (*dest).dir = (*prob).dir;
    (*dest).c0 = (*prob).c0;
    if (*prob).m > 0 as i32 {
        glp_add_rows(dest, (*prob).m);
    }
    if (*prob).n > 0 as i32 {
        glp_add_cols(dest, (*prob).n);
    }
    glp_get_bfcp(prob, &mut bfcp);
    glp_set_bfcp(dest, &mut bfcp);
    (*dest).pbs_stat = (*prob).pbs_stat;
    (*dest).dbs_stat = (*prob).dbs_stat;
    (*dest).obj_val = (*prob).obj_val;
    (*dest).some = (*prob).some;
    (*dest).ipt_stat = (*prob).ipt_stat;
    (*dest).ipt_obj = (*prob).ipt_obj;
    (*dest).mip_stat = (*prob).mip_stat;
    (*dest).mip_obj = (*prob).mip_obj;
    i = 1 as i32;
    while i <= (*prob).m {
        let mut to: *mut GLPROW = *((*dest).row).offset(i as isize);
        let mut from: *mut GLPROW = *((*prob).row).offset(i as isize);
        if names != 0 && !((*from).name).is_null() {
            glp_set_row_name(dest, i, (*from).name);
        }
        (*to).type_0 = (*from).type_0;
        (*to).lb = (*from).lb;
        (*to).ub = (*from).ub;
        (*to).rii = (*from).rii;
        (*to).stat = (*from).stat;
        (*to).prim = (*from).prim;
        (*to).dual = (*from).dual;
        (*to).pval = (*from).pval;
        (*to).dval = (*from).dval;
        (*to).mipx = (*from).mipx;
        i += 1;
        i;
    }
    ind = glp_alloc(1 as i32 + (*prob).m, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    val = glp_alloc(
        1 as i32 + (*prob).m,
        ::core::mem::size_of::<libc::c_double>() as u64 as i32,
    ) as *mut libc::c_double;
    j = 1 as i32;
    while j <= (*prob).n {
        let mut to_0: *mut GLPCOL = *((*dest).col).offset(j as isize);
        let mut from_0: *mut GLPCOL = *((*prob).col).offset(j as isize);
        if names != 0 && !((*from_0).name).is_null() {
            glp_set_col_name(dest, j, (*from_0).name);
        }
        (*to_0).kind = (*from_0).kind;
        (*to_0).type_0 = (*from_0).type_0;
        (*to_0).lb = (*from_0).lb;
        (*to_0).ub = (*from_0).ub;
        (*to_0).coef = (*from_0).coef;
        len = glp_get_mat_col(prob, j, ind, val);
        glp_set_mat_col(dest, j, len, ind as *const i32, val as *const libc::c_double);
        (*to_0).sjj = (*from_0).sjj;
        (*to_0).stat = (*from_0).stat;
        (*to_0).prim = (*from_0).prim;
        (*to_0).dual = (*from_0).dual;
        (*to_0).pval = (*from_0).pval;
        (*to_0).dval = (*from_0).dval;
        (*to_0).mipx = (*from_0).mipx;
        j += 1;
        j;
    }
    glp_free(ind as *mut libc::c_void);
    glp_free(val as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn glp_erase_prob(mut lp: *mut glp_prob) {
    let mut tree: *mut glp_tree = (*lp).tree;
    if !tree.is_null() && (*tree).reason != 0 as i32 {
        (glp_error_(b"api/prob1.c\0" as *const u8 as *const i8, 1527 as i32))
            .expect(
                "non-null function pointer",
            )(b"glp_erase_prob: operation not allowed\n\0" as *const u8 as *const i8);
    }
    delete_prob(lp);
    create_prob(lp);
}
unsafe extern "C" fn delete_prob(mut lp: *mut glp_prob) {
    _glp_dmp_delete_pool((*lp).pool);
    (((*lp).tree).is_null()
        || {
            glp_assert_(
                b"lp->tree == NULL\0" as *const u8 as *const i8,
                b"api/prob1.c\0" as *const u8 as *const i8,
                1561 as i32,
            );
            1 as i32 != 0
        }) as i32;
    glp_free((*lp).row as *mut libc::c_void);
    glp_free((*lp).col as *mut libc::c_void);
    if !((*lp).r_tree).is_null() {
        _glp_avl_delete_tree((*lp).r_tree);
    }
    if !((*lp).c_tree).is_null() {
        _glp_avl_delete_tree((*lp).c_tree);
    }
    glp_free((*lp).head as *mut libc::c_void);
    if !((*lp).bfd).is_null() {
        _glp_bfd_delete_it((*lp).bfd);
    }
}
#[no_mangle]
pub unsafe extern "C" fn glp_delete_prob(mut lp: *mut glp_prob) {
    let mut tree: *mut glp_tree = (*lp).tree;
    if !tree.is_null() && (*tree).reason != 0 as i32 {
        (glp_error_(b"api/prob1.c\0" as *const u8 as *const i8, 1580 as i32))
            .expect(
                "non-null function pointer",
            )(b"glp_delete_prob: operation not allowed\n\0" as *const u8 as *const i8);
    }
    delete_prob(lp);
    glp_free(lp as *mut libc::c_void);
}