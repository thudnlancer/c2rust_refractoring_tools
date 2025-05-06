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
    fn _glp_dmp_delete_pool(pool: *mut DMP);
    fn _glp_dmp_create_pool() -> *mut DMP;
    fn _glp_dmp_free_atom(pool: *mut DMP, atom: *mut libc::c_void, size: i32);
    fn _glp_dmp_get_atom(pool: *mut DMP, size: i32) -> *mut libc::c_void;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
    fn glp_alloc(n: i32, size: i32) -> *mut libc::c_void;
    fn glp_free(ptr: *mut libc::c_void);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVL {
    pub pool: *mut DMP,
    pub root: *mut AVLNODE,
    pub fcmp: Option<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const libc::c_void,
            *const libc::c_void,
        ) -> i32,
    >,
    pub info: *mut libc::c_void,
    pub size: i32,
    pub height: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVLNODE {
    pub key: *const libc::c_void,
    pub rank: i32,
    pub type_0: i32,
    pub link: *mut libc::c_void,
    pub up: *mut AVLNODE,
    pub flag: libc::c_short,
    pub bal: libc::c_short,
    pub left: *mut AVLNODE,
    pub right: *mut AVLNODE,
}
#[no_mangle]
pub unsafe extern "C" fn _glp_avl_create_tree(
    mut fcmp: Option<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const libc::c_void,
            *const libc::c_void,
        ) -> i32,
    >,
    mut info: *mut libc::c_void,
) -> *mut AVL {
    let mut tree: *mut AVL = 0 as *mut AVL;
    tree = glp_alloc(1 as i32, ::core::mem::size_of::<AVL>() as u64 as i32) as *mut AVL;
    (*tree).pool = _glp_dmp_create_pool();
    (*tree).root = 0 as *mut AVLNODE;
    (*tree).fcmp = fcmp;
    (*tree).info = info;
    (*tree).size = 0 as i32;
    (*tree).height = 0 as i32;
    return tree;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_avl_strcmp(
    mut info: *mut libc::c_void,
    mut key1: *const libc::c_void,
    mut key2: *const libc::c_void,
) -> i32 {
    (info == info
        || {
            glp_assert_(
                b"info == info\0" as *const u8 as *const i8,
                b"misc/avl.c\0" as *const u8 as *const i8,
                89 as i32,
            );
            1 as i32 != 0
        }) as i32;
    return strcmp(key1 as *const i8, key2 as *const i8);
}
#[no_mangle]
pub unsafe extern "C" fn _glp_avl_insert_node(
    mut tree: *mut AVL,
    mut key: *const libc::c_void,
) -> *mut AVLNODE {
    let mut p: *mut AVLNODE = 0 as *mut AVLNODE;
    let mut q: *mut AVLNODE = 0 as *mut AVLNODE;
    let mut r: *mut AVLNODE = 0 as *mut AVLNODE;
    let mut flag: libc::c_short = 0;
    p = 0 as *mut AVLNODE;
    q = (*tree).root;
    while !q.is_null() {
        p = q;
        if ((*tree).fcmp)
            .expect("non-null function pointer")((*tree).info, key, (*p).key) <= 0 as i32
        {
            flag = 0 as i32 as libc::c_short;
            q = (*p).left;
            (*p).rank += 1;
            (*p).rank;
        } else {
            flag = 1 as i32 as libc::c_short;
            q = (*p).right;
        }
    }
    r = _glp_dmp_get_atom((*tree).pool, ::core::mem::size_of::<AVLNODE>() as u64 as i32)
        as *mut AVLNODE;
    (*r).key = key;
    (*r).type_0 = 0 as i32;
    (*r).link = 0 as *mut libc::c_void;
    (*r).rank = 1 as i32;
    (*r).up = p;
    (*r).flag = (if p.is_null() { 0 as i32 } else { flag as i32 }) as libc::c_short;
    (*r).bal = 0 as i32 as libc::c_short;
    (*r).left = 0 as *mut AVLNODE;
    (*r).right = 0 as *mut AVLNODE;
    (*tree).size += 1;
    (*tree).size;
    if p.is_null() {
        (*tree).root = r;
    } else if flag as i32 == 0 as i32 {
        (*p).left = r;
    } else {
        (*p).right = r;
    }
    while !p.is_null() {
        if flag as i32 == 0 as i32 {
            if (*p).bal as i32 > 0 as i32 {
                (*p).bal = 0 as i32 as libc::c_short;
                break;
            } else if ((*p).bal as i32) < 0 as i32 {
                rotate_subtree(tree, p);
                break;
            } else {
                (*p).bal = -(1 as i32) as libc::c_short;
                flag = (*p).flag;
                p = (*p).up;
            }
        } else if ((*p).bal as i32) < 0 as i32 {
            (*p).bal = 0 as i32 as libc::c_short;
            break;
        } else if (*p).bal as i32 > 0 as i32 {
            rotate_subtree(tree, p);
            break;
        } else {
            (*p).bal = 1 as i32 as libc::c_short;
            flag = (*p).flag;
            p = (*p).up;
        }
    }
    if p.is_null() {
        (*tree).height += 1;
        (*tree).height;
    }
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_avl_set_node_type(
    mut node: *mut AVLNODE,
    mut type_0: i32,
) {
    (*node).type_0 = type_0;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_avl_set_node_link(
    mut node: *mut AVLNODE,
    mut link: *mut libc::c_void,
) {
    (*node).link = link;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_avl_find_node(
    mut tree: *mut AVL,
    mut key: *const libc::c_void,
) -> *mut AVLNODE {
    let mut p: *mut AVLNODE = 0 as *mut AVLNODE;
    let mut c: i32 = 0;
    p = (*tree).root;
    while !p.is_null() {
        c = ((*tree).fcmp)
            .expect("non-null function pointer")((*tree).info, key, (*p).key);
        if c == 0 as i32 {
            break;
        }
        p = if c < 0 as i32 { (*p).left } else { (*p).right };
    }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_avl_get_node_type(mut node: *mut AVLNODE) -> i32 {
    return (*node).type_0;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_avl_get_node_link(
    mut node: *mut AVLNODE,
) -> *mut libc::c_void {
    return (*node).link;
}
unsafe extern "C" fn find_next_node(
    mut tree: *mut AVL,
    mut node: *mut AVLNODE,
) -> *mut AVLNODE {
    let mut p: *mut AVLNODE = 0 as *mut AVLNODE;
    let mut q: *mut AVLNODE = 0 as *mut AVLNODE;
    if ((*tree).root).is_null() {
        return 0 as *mut AVLNODE;
    }
    p = node;
    q = if p.is_null() { (*tree).root } else { (*p).right };
    if q.is_null() {
        loop {
            q = (*p).up;
            if q.is_null() {
                break;
            }
            if (*p).flag as i32 == 0 as i32 {
                break;
            }
            p = q;
        }
    } else {
        loop {
            p = (*q).left;
            if p.is_null() {
                break;
            }
            q = p;
        }
    }
    return q;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_avl_delete_node(
    mut tree: *mut AVL,
    mut node: *mut AVLNODE,
) {
    let mut f: *mut AVLNODE = 0 as *mut AVLNODE;
    let mut p: *mut AVLNODE = 0 as *mut AVLNODE;
    let mut q: *mut AVLNODE = 0 as *mut AVLNODE;
    let mut r: *mut AVLNODE = 0 as *mut AVLNODE;
    let mut s: *mut AVLNODE = 0 as *mut AVLNODE;
    let mut x: *mut AVLNODE = 0 as *mut AVLNODE;
    let mut y: *mut AVLNODE = 0 as *mut AVLNODE;
    let mut flag: libc::c_short = 0;
    p = node;
    if !(((*p).left).is_null() || ((*p).right).is_null()) {
        f = (*p).up;
        q = (*p).left;
        r = find_next_node(tree, p);
        s = (*r).right;
        if (*p).right == r {
            if f.is_null() {
                (*tree).root = r;
            } else if (*p).flag as i32 == 0 as i32 {
                (*f).left = r;
            } else {
                (*f).right = r;
            }
            (*r).rank = (*p).rank;
            (*r).up = f;
            (*r).flag = (*p).flag;
            (*r).bal = (*p).bal;
            (*r).left = q;
            (*r).right = p;
            (*q).up = r;
            (*p).rank = 1 as i32;
            (*p).up = r;
            (*p).flag = 1 as i32 as libc::c_short;
            (*p).bal = (if s.is_null() { 0 as i32 } else { 1 as i32 }) as libc::c_short;
            (*p).left = 0 as *mut AVLNODE;
            (*p).right = s;
            if !s.is_null() {
                (*s).up = p;
            }
        } else {
            x = (*p).right;
            y = (*r).up;
            if f.is_null() {
                (*tree).root = r;
            } else if (*p).flag as i32 == 0 as i32 {
                (*f).left = r;
            } else {
                (*f).right = r;
            }
            (*r).rank = (*p).rank;
            (*r).up = f;
            (*r).flag = (*p).flag;
            (*r).bal = (*p).bal;
            (*r).left = q;
            (*r).right = x;
            (*q).up = r;
            (*x).up = r;
            (*y).left = p;
            (*p).rank = 1 as i32;
            (*p).up = y;
            (*p).flag = 0 as i32 as libc::c_short;
            (*p).bal = (if s.is_null() { 0 as i32 } else { 1 as i32 }) as libc::c_short;
            (*p).left = 0 as *mut AVLNODE;
            (*p).right = s;
            if !s.is_null() {
                (*s).up = p;
            }
        }
    }
    q = p;
    f = (*q).up;
    while !f.is_null() {
        if (*q).flag as i32 == 0 as i32 {
            (*f).rank -= 1;
            (*f).rank;
        }
        q = f;
        f = (*q).up;
    }
    f = (*p).up;
    flag = (*p).flag;
    q = if !((*p).left).is_null() { (*p).left } else { (*p).right };
    if f.is_null() {
        (*tree).root = q;
    } else if flag as i32 == 0 as i32 {
        (*f).left = q;
    } else {
        (*f).right = q;
    }
    if !q.is_null() {
        (*q).up = f;
        (*q).flag = flag;
    }
    (*tree).size -= 1;
    (*tree).size;
    while !f.is_null() {
        if flag as i32 == 0 as i32 {
            if (*f).bal as i32 == 0 as i32 {
                (*f).bal = 1 as i32 as libc::c_short;
                break;
            } else {
                if ((*f).bal as i32) < 0 as i32 {
                    (*f).bal = 0 as i32 as libc::c_short;
                } else {
                    f = rotate_subtree(tree, f);
                    if ((*f).bal as i32) < 0 as i32 {
                        break;
                    }
                }
                flag = (*f).flag;
                f = (*f).up;
            }
        } else if (*f).bal as i32 == 0 as i32 {
            (*f).bal = -(1 as i32) as libc::c_short;
            break;
        } else {
            if (*f).bal as i32 > 0 as i32 {
                (*f).bal = 0 as i32 as libc::c_short;
            } else {
                f = rotate_subtree(tree, f);
                if (*f).bal as i32 > 0 as i32 {
                    break;
                }
            }
            flag = (*f).flag;
            f = (*f).up;
        }
    }
    if f.is_null() {
        (*tree).height -= 1;
        (*tree).height;
    }
    _glp_dmp_free_atom(
        (*tree).pool,
        p as *mut libc::c_void,
        ::core::mem::size_of::<AVLNODE>() as u64 as i32,
    );
}
unsafe extern "C" fn rotate_subtree(
    mut tree: *mut AVL,
    mut node: *mut AVLNODE,
) -> *mut AVLNODE {
    let mut f: *mut AVLNODE = 0 as *mut AVLNODE;
    let mut p: *mut AVLNODE = 0 as *mut AVLNODE;
    let mut q: *mut AVLNODE = 0 as *mut AVLNODE;
    let mut r: *mut AVLNODE = 0 as *mut AVLNODE;
    let mut x: *mut AVLNODE = 0 as *mut AVLNODE;
    let mut y: *mut AVLNODE = 0 as *mut AVLNODE;
    (!node.is_null()
        || {
            glp_assert_(
                b"node != NULL\0" as *const u8 as *const i8,
                b"misc/avl.c\0" as *const u8 as *const i8,
                319 as i32,
            );
            1 as i32 != 0
        }) as i32;
    p = node;
    if ((*p).bal as i32) < 0 as i32 {
        f = (*p).up;
        q = (*p).left;
        r = (*q).right;
        if (*q).bal as i32 <= 0 as i32 {
            if f.is_null() {
                (*tree).root = q;
            } else if (*p).flag as i32 == 0 as i32 {
                (*f).left = q;
            } else {
                (*f).right = q;
            }
            (*p).rank -= (*q).rank;
            (*q).up = f;
            (*q).flag = (*p).flag;
            (*q).bal += 1;
            (*q).bal;
            (*q).right = p;
            (*p).up = q;
            (*p).flag = 1 as i32 as libc::c_short;
            (*p).bal = -((*q).bal as i32) as libc::c_short;
            (*p).left = r;
            if !r.is_null() {
                (*r).up = p;
                (*r).flag = 0 as i32 as libc::c_short;
            }
            node = q;
        } else {
            x = (*r).left;
            y = (*r).right;
            if f.is_null() {
                (*tree).root = r;
            } else if (*p).flag as i32 == 0 as i32 {
                (*f).left = r;
            } else {
                (*f).right = r;
            }
            (*p).rank -= (*q).rank + (*r).rank;
            (*r).rank += (*q).rank;
            (*p).bal = (if (*r).bal as i32 >= 0 as i32 { 0 as i32 } else { 1 as i32 })
                as libc::c_short;
            (*q).bal = (if (*r).bal as i32 <= 0 as i32 { 0 as i32 } else { -(1 as i32) })
                as libc::c_short;
            (*r).up = f;
            (*r).flag = (*p).flag;
            (*r).bal = 0 as i32 as libc::c_short;
            (*r).left = q;
            (*r).right = p;
            (*p).up = r;
            (*p).flag = 1 as i32 as libc::c_short;
            (*p).left = y;
            (*q).up = r;
            (*q).flag = 0 as i32 as libc::c_short;
            (*q).right = x;
            if !x.is_null() {
                (*x).up = q;
                (*x).flag = 1 as i32 as libc::c_short;
            }
            if !y.is_null() {
                (*y).up = p;
                (*y).flag = 0 as i32 as libc::c_short;
            }
            node = r;
        }
    } else {
        f = (*p).up;
        q = (*p).right;
        r = (*q).left;
        if (*q).bal as i32 >= 0 as i32 {
            if f.is_null() {
                (*tree).root = q;
            } else if (*p).flag as i32 == 0 as i32 {
                (*f).left = q;
            } else {
                (*f).right = q;
            }
            (*q).rank += (*p).rank;
            (*q).up = f;
            (*q).flag = (*p).flag;
            (*q).bal -= 1;
            (*q).bal;
            (*q).left = p;
            (*p).up = q;
            (*p).flag = 0 as i32 as libc::c_short;
            (*p).bal = -((*q).bal as i32) as libc::c_short;
            (*p).right = r;
            if !r.is_null() {
                (*r).up = p;
                (*r).flag = 1 as i32 as libc::c_short;
            }
            node = q;
        } else {
            x = (*r).left;
            y = (*r).right;
            if f.is_null() {
                (*tree).root = r;
            } else if (*p).flag as i32 == 0 as i32 {
                (*f).left = r;
            } else {
                (*f).right = r;
            }
            (*q).rank -= (*r).rank;
            (*r).rank += (*p).rank;
            (*p).bal = (if (*r).bal as i32 <= 0 as i32 { 0 as i32 } else { -(1 as i32) })
                as libc::c_short;
            (*q).bal = (if (*r).bal as i32 >= 0 as i32 { 0 as i32 } else { 1 as i32 })
                as libc::c_short;
            (*r).up = f;
            (*r).flag = (*p).flag;
            (*r).bal = 0 as i32 as libc::c_short;
            (*r).left = p;
            (*r).right = q;
            (*p).up = r;
            (*p).flag = 0 as i32 as libc::c_short;
            (*p).right = x;
            (*q).up = r;
            (*q).flag = 1 as i32 as libc::c_short;
            (*q).left = y;
            if !x.is_null() {
                (*x).up = p;
                (*x).flag = 1 as i32 as libc::c_short;
            }
            if !y.is_null() {
                (*y).up = q;
                (*y).flag = 0 as i32 as libc::c_short;
            }
            node = r;
        }
    }
    return node;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_avl_delete_tree(mut tree: *mut AVL) {
    _glp_dmp_delete_pool((*tree).pool);
    glp_free(tree as *mut libc::c_void);
}