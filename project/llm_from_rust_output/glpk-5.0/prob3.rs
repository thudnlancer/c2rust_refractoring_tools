use std::ffi::{CStr, CString};
use std::ptr;
use std::cmp::Ordering;

pub type AVL = *mut libc::c_void;
pub type AVLNODE = *mut libc::c_void;
pub type BFD = *mut libc::c_void;
pub type DMP = *mut libc::c_void;
pub type glp_tree = *mut libc::c_void;

pub type glp_errfunc = Option<unsafe extern "C" fn(*const libc::c_char, ...)>;

#[derive(Debug)]
pub struct GlpProb {
    pool: *mut DMP,
    tree: *mut glp_tree,
    name: Option<CString>,
    obj: Option<CString>,
    dir: i32,
    c0: f64,
    m_max: i32,
    n_max: i32,
    m: i32,
    n: i32,
    nnz: i32,
    row: Vec<Option<Box<GLPROW>>>,
    col: Vec<Option<Box<GLPCOL>>>,
    r_tree: Option<AVLTree>,
    c_tree: Option<AVLTree>,
    valid: i32,
    head: *mut i32,
    bfd: *mut BFD,
    pbs_stat: i32,
    dbs_stat: i32,
    obj_val: f64,
    it_cnt: i32,
    some: i32,
    ipt_stat: i32,
    ipt_obj: f64,
    mip_stat: i32,
    mip_obj: f64,
}

#[derive(Debug)]
pub struct GLPCOL {
    j: i32,
    name: Option<CString>,
    node: Option<*mut AVLNODE>,
    kind: i32,
    type_: i32,
    lb: f64,
    ub: f64,
    coef: f64,
    ptr: Option<Box<GLPAIJ>>,
    sjj: f64,
    stat: i32,
    bind: i32,
    prim: f64,
    dual: f64,
    pval: f64,
    dval: f64,
    mipx: f64,
}

#[derive(Debug)]
pub struct GLPAIJ {
    row: Option<Box<GLPROW>>,
    col: Option<Box<GLPCOL>>,
    val: f64,
    r_prev: Option<Box<GLPAIJ>>,
    r_next: Option<Box<GLPAIJ>>,
    c_prev: Option<Box<GLPAIJ>>,
    c_next: Option<Box<GLPAIJ>>,
}

#[derive(Debug)]
pub struct GLPROW {
    i: i32,
    name: Option<CString>,
    node: Option<*mut AVLNODE>,
    level: i32,
    origin: u8,
    klass: u8,
    type_: i32,
    lb: f64,
    ub: f64,
    ptr: Option<Box<GLPAIJ>>,
    rii: f64,
    stat: i32,
    bind: i32,
    prim: f64,
    dual: f64,
    pval: f64,
    dval: f64,
    mipx: f64,
}

struct AVLTree {
    tree: *mut AVL,
}

impl AVLTree {
    fn new() -> Option<Self> {
        unsafe {
            let tree = _glp_avl_create_tree(
                Some(_glp_avl_strcmp),
                ptr::null_mut(),
            );
            if tree.is_null() {
                None
            } else {
                Some(Self { tree })
            }
        }
    }

    fn find_node(&self, key: &CStr) -> Option<*mut AVLNODE> {
        unsafe {
            let node = _glp_avl_find_node(self.tree, key.as_ptr() as *const _);
            if node.is_null() {
                None
            } else {
                Some(node)
            }
        }
    }

    fn insert_node(&mut self, key: &CStr) -> Option<*mut AVLNODE> {
        unsafe {
            let node = _glp_avl_insert_node(self.tree, key.as_ptr() as *const _);
            if node.is_null() {
                None
            } else {
                Some(node)
            }
        }
    }

    fn set_node_link(&self, node: *mut AVLNODE, link: *mut libc::c_void) {
        unsafe {
            _glp_avl_set_node_link(node, link);
        }
    }
}

impl Drop for AVLTree {
    fn drop(&mut self) {
        unsafe {
            _glp_avl_delete_tree(self.tree);
        }
    }
}

impl GlpProb {
    pub fn create_index(&mut self) {
        if self.r_tree.is_none() {
            self.r_tree = AVLTree::new();
            if let Some(ref mut tree) = self.r_tree {
                for i in 1..=self.m {
                    if let Some(ref mut row) = self.row[i as usize] {
                        assert!(row.node.is_none(), "row->node == NULL");
                        if let Some(ref name) = row.name {
                            if let Some(node) = tree.insert_node(name) {
                                row.node = Some(node);
                                tree.set_node_link(node, row as *mut _ as *mut _);
                            }
                        }
                    }
                }
            }
        }

        if self.c_tree.is_none() {
            self.c_tree = AVLTree::new();
            if let Some(ref mut tree) = self.c_tree {
                for j in 1..=self.n {
                    if let Some(ref mut col) = self.col[j as usize] {
                        assert!(col.node.is_none(), "col->node == NULL");
                        if let Some(ref name) = col.name {
                            if let Some(node) = tree.insert_node(name) {
                                col.node = Some(node);
                                tree.set_node_link(node, col as *mut _ as *mut _);
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn find_row(&self, name: &CStr) -> i32 {
        if self.r_tree.is_none() {
            unsafe {
                (glp_error_(
                    b"api/prob3.c\0".as_ptr() as *const _,
                    94,
                )).expect("non-null function pointer")(
                    b"glp_find_row: row name index does not exist\n\0".as_ptr() as *const _,
                );
            }
            return 0;
        }

        if name.to_bytes().is_empty() || name.to_bytes().len() > 255 {
            return 0;
        }

        if let Some(ref tree) = self.r_tree {
            if let Some(node) = tree.find_node(name) {
                unsafe {
                    let link = _glp_avl_get_node_link(node);
                    return (*(link as *mut GLPROW)).i;
                }
            }
        }
        0
    }

    pub fn find_col(&self, name: &CStr) -> i32 {
        if self.c_tree.is_none() {
            unsafe {
                (glp_error_(
                    b"api/prob3.c\0".as_ptr() as *const _,
                    122,
                )).expect("non-null function pointer")(
                    b"glp_find_col: column name index does not exist\n\0".as_ptr() as *const _,
                );
            }
            return 0;
        }

        if name.to_bytes().is_empty() || name.to_bytes().len() > 255 {
            return 0;
        }

        if let Some(ref tree) = self.c_tree {
            if let Some(node) = tree.find_node(name) {
                unsafe {
                    let link = _glp_avl_get_node_link(node);
                    return (*(link as *mut GLPCOL)).j;
                }
            }
        }
        0
    }

    pub fn delete_index(&mut self) {
        if let Some(_) = self.r_tree.take() {
            for i in 1..=self.m {
                if let Some(ref mut row) = self.row[i as usize] {
                    row.node = None;
                }
            }
        }

        if let Some(_) = self.c_tree.take() {
            for j in 1..=self.n {
                if let Some(ref mut col) = self.col[j as usize] {
                    col.node = None;
                }
            }
        }
    }
}

extern "C" {
    fn glp_assert_(expr: *const libc::c_char, file: *const libc::c_char, line: i32);
    fn glp_error_(file: *const libc::c_char, line: i32) -> glp_errfunc;
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
        fcmp: Option<unsafe extern "C" fn(*mut libc::c_void, *const libc::c_void, *const libc::c_void) -> i32>,
        info: *mut libc::c_void,
    ) -> *mut AVL;
    fn _glp_avl_delete_tree(tree: *mut AVL);
}