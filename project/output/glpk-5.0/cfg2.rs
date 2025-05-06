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
    pub type DMP;
    pub type AVL;
    pub type AVLNODE;
    pub type BFD;
    pub type glp_tree;
    fn _glp_cfg_delete_graph(G: *mut CFG);
    fn _glp_cfg_build_graph(P: *mut libc::c_void) -> *mut CFG;
    fn glp_printf(fmt: *const i8, _: ...);
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glp_cfg {
    pub n: i32,
    pub pos: *mut i32,
    pub neg: *mut i32,
    pub pool: *mut DMP,
    pub nv_max: i32,
    pub nv: i32,
    pub ref_0: *mut i32,
    pub vptr: *mut *mut CFGVLE,
    pub cptr: *mut *mut CFGCLE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CFGCLE {
    pub vptr: *mut CFGVLE,
    pub next: *mut CFGCLE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CFGVLE {
    pub v: i32,
    pub next: *mut CFGVLE,
}
pub type CFG = glp_cfg;
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
pub unsafe extern "C" fn glp_cfg_init(mut P: *mut glp_prob) -> *mut glp_cfg {
    let mut G: *mut glp_cfg = 0 as *mut glp_cfg;
    let mut j: i32 = 0;
    let mut n1: i32 = 0;
    let mut n2: i32 = 0;
    glp_printf(b"Constructing conflict graph...\n\0" as *const u8 as *const i8);
    G = _glp_cfg_build_graph(P as *mut libc::c_void);
    n2 = 0 as i32;
    n1 = n2;
    j = 1 as i32;
    while j <= (*P).n {
        if *((*G).pos).offset(j as isize) != 0 {
            n1 += 1;
            n1;
        }
        if *((*G).neg).offset(j as isize) != 0 {
            n2 += 1;
            n2;
        }
        j += 1;
        j;
    }
    if n1 == 0 as i32 && n2 == 0 as i32 {
        glp_printf(b"No conflicts found\n\0" as *const u8 as *const i8);
        _glp_cfg_delete_graph(G);
        G = 0 as *mut glp_cfg;
    } else {
        glp_printf(
            b"Conflict graph has %d + %d = %d vertices\n\0" as *const u8 as *const i8,
            n1,
            n2,
            (*G).nv,
        );
    }
    return G;
}
#[no_mangle]
pub unsafe extern "C" fn glp_cfg_free(mut G: *mut glp_cfg) {
    (!G.is_null()
        || {
            glp_assert_(
                b"G != NULL\0" as *const u8 as *const i8,
                b"intopt/cfg2.c\0" as *const u8 as *const i8,
                84 as i32,
            );
            1 as i32 != 0
        }) as i32;
    _glp_cfg_delete_graph(G);
}