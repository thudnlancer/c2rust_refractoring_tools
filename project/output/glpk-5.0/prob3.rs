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
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
    fn glp_error_(file: *const i8, line: i32) -> glp_errfunc;
    fn strlen(_: *const i8) -> u64;
    fn _glp_avl_get_node_link(node: *mut AVLNODE) -> *mut libc::c_void;
    fn _glp_avl_find_node(tree: *mut AVL, key: *const libc::c_void) -> *mut AVLNODE;
    fn _glp_avl_set_node_link(node: *mut AVLNODE, link: *mut libc::c_void);
    fn _glp_avl_insert_node(tree: *mut AVL, key: *const libc::c_void) -> *mut AVLNODE;
    fn _glp_avl_strcmp(
        info: *mut libc::c_void,
        key1: *const libc::c_void,
        key2: *const libc::c_void,
    ) -> i32;
    fn _glp_avl_create_tree(
        fcmp: Option<
            unsafe extern "C" fn(
                *mut libc::c_void,
                *const libc::c_void,
                *const libc::c_void,
            ) -> i32,
        >,
        info: *mut libc::c_void,
    ) -> *mut AVL;
    fn _glp_avl_delete_tree(tree: *mut AVL);
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
#[no_mangle]
pub unsafe extern "C" fn glp_create_index(mut lp: *mut glp_prob) {
    let mut row: *mut GLPROW = 0 as *mut GLPROW;
    let mut col: *mut GLPCOL = 0 as *mut GLPCOL;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    if ((*lp).r_tree).is_null() {
        (*lp).r_tree = _glp_avl_create_tree(
            Some(
                _glp_avl_strcmp
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> i32,
            ),
            0 as *mut libc::c_void,
        );
        i = 1 as i32;
        while i <= (*lp).m {
            row = *((*lp).row).offset(i as isize);
            (((*row).node).is_null()
                || {
                    glp_assert_(
                        b"row->node == NULL\0" as *const u8 as *const i8,
                        b"api/prob3.c\0" as *const u8 as *const i8,
                        53 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            if !((*row).name).is_null() {
                (*row).node = _glp_avl_insert_node(
                    (*lp).r_tree,
                    (*row).name as *const libc::c_void,
                );
                _glp_avl_set_node_link((*row).node, row as *mut libc::c_void);
            }
            i += 1;
            i;
        }
    }
    if ((*lp).c_tree).is_null() {
        (*lp).c_tree = _glp_avl_create_tree(
            Some(
                _glp_avl_strcmp
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> i32,
            ),
            0 as *mut libc::c_void,
        );
        j = 1 as i32;
        while j <= (*lp).n {
            col = *((*lp).col).offset(j as isize);
            (((*col).node).is_null()
                || {
                    glp_assert_(
                        b"col->node == NULL\0" as *const u8 as *const i8,
                        b"api/prob3.c\0" as *const u8 as *const i8,
                        65 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            if !((*col).name).is_null() {
                (*col).node = _glp_avl_insert_node(
                    (*lp).c_tree,
                    (*col).name as *const libc::c_void,
                );
                _glp_avl_set_node_link((*col).node, col as *mut libc::c_void);
            }
            j += 1;
            j;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn glp_find_row(
    mut lp: *mut glp_prob,
    mut name: *const i8,
) -> i32 {
    let mut node: *mut AVLNODE = 0 as *mut AVLNODE;
    let mut i: i32 = 0 as i32;
    if ((*lp).r_tree).is_null() {
        (glp_error_(b"api/prob3.c\0" as *const u8 as *const i8, 94 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_find_row: row name index does not exist\n\0" as *const u8 as *const i8,
        );
    }
    if !(name.is_null() || *name.offset(0 as i32 as isize) as i32 == '\0' as i32
        || strlen(name) > 255 as i32 as u64)
    {
        node = _glp_avl_find_node((*lp).r_tree, name as *const libc::c_void);
        if !node.is_null() {
            i = (*(_glp_avl_get_node_link(node) as *mut GLPROW)).i;
        }
    }
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn glp_find_col(
    mut lp: *mut glp_prob,
    mut name: *const i8,
) -> i32 {
    let mut node: *mut AVLNODE = 0 as *mut AVLNODE;
    let mut j: i32 = 0 as i32;
    if ((*lp).c_tree).is_null() {
        (glp_error_(b"api/prob3.c\0" as *const u8 as *const i8, 122 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_find_col: column name index does not exist\n\0" as *const u8
                as *const i8,
        );
    }
    if !(name.is_null() || *name.offset(0 as i32 as isize) as i32 == '\0' as i32
        || strlen(name) > 255 as i32 as u64)
    {
        node = _glp_avl_find_node((*lp).c_tree, name as *const libc::c_void);
        if !node.is_null() {
            j = (*(_glp_avl_get_node_link(node) as *mut GLPCOL)).j;
        }
    }
    return j;
}
#[no_mangle]
pub unsafe extern "C" fn glp_delete_index(mut lp: *mut glp_prob) {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    if !((*lp).r_tree).is_null() {
        i = 1 as i32;
        while i <= (*lp).m {
            let ref mut fresh0 = (**((*lp).row).offset(i as isize)).node;
            *fresh0 = 0 as *mut AVLNODE;
            i += 1;
            i;
        }
        _glp_avl_delete_tree((*lp).r_tree);
        (*lp).r_tree = 0 as *mut AVL;
    }
    if !((*lp).c_tree).is_null() {
        j = 1 as i32;
        while j <= (*lp).n {
            let ref mut fresh1 = (**((*lp).col).offset(j as isize)).node;
            *fresh1 = 0 as *mut AVLNODE;
            j += 1;
            j;
        }
        _glp_avl_delete_tree((*lp).c_tree);
        (*lp).c_tree = 0 as *mut AVL;
    }
}