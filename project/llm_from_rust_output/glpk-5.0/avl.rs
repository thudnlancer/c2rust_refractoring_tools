use std::cmp::Ordering;
use std::ptr;

struct DMP;

struct AVL {
    pool: *mut DMP,
    root: *mut AVLNODE,
    fcmp: Option<Box<dyn FnMut(&mut (), &(), &()) -> i32>>,
    info: *mut (),
    size: i32,
    height: i32,
}

struct AVLNODE {
    key: *const (),
    rank: i32,
    type_: i32,
    link: *mut (),
    up: *mut AVLNODE,
    flag: i16,
    bal: i16,
    left: *mut AVLNODE,
    right: *mut AVLNODE,
}

impl AVL {
    fn new(
        fcmp: Option<Box<dyn FnMut(&mut (), &(), &()) -> i32>>,
        info: *mut (),
    ) -> Self {
        unsafe {
            AVL {
                pool: _glp_dmp_create_pool(),
                root: ptr::null_mut(),
                fcmp,
                info,
                size: 0,
                height: 0,
            }
        }
    }

    fn insert_node(&mut self, key: *const ()) -> *mut AVLNODE {
        unsafe {
            let mut p = ptr::null_mut();
            let mut q = self.root;
            let mut flag = 0i16;

            while !q.is_null() {
                p = q;
                let cmp = (self.fcmp.as_mut().unwrap())(&mut *self.info, &*key, &*(*p).key);
                match cmp.cmp(&0) {
                    Ordering::Less | Ordering::Equal => {
                        flag = 0;
                        q = (*p).left;
                        (*p).rank += 1;
                    }
                    Ordering::Greater => {
                        flag = 1;
                        q = (*p).right;
                    }
                }
            }

            let r = _glp_dmp_get_atom(
                self.pool,
                std::mem::size_of::<AVLNODE>() as i32,
            ) as *mut AVLNODE;

            (*r).key = key;
            (*r).type_ = 0;
            (*r).link = ptr::null_mut();
            (*r).rank = 1;
            (*r).up = p;
            (*r).flag = if p.is_null() { 0 } else { flag } as i16;
            (*r).bal = 0;
            (*r).left = ptr::null_mut();
            (*r).right = ptr::null_mut();

            self.size += 1;

            if p.is_null() {
                self.root = r;
            } else if flag == 0 {
                (*p).left = r;
            } else {
                (*p).right = r;
            }

            let mut current = p;
            while !current.is_null() {
                if flag == 0 {
                    if (*current).bal > 0 {
                        (*current).bal = 0;
                        break;
                    } else if (*current).bal < 0 {
                        self.rotate_subtree(current);
                        break;
                    } else {
                        (*current).bal = -1;
                        flag = (*current).flag;
                        current = (*current).up;
                    }
                } else if (*current).bal < 0 {
                    (*current).bal = 0;
                    break;
                } else if (*current).bal > 0 {
                    self.rotate_subtree(current);
                    break;
                } else {
                    (*current).bal = 1;
                    flag = (*current).flag;
                    current = (*current).up;
                }
            }

            if current.is_null() {
                self.height += 1;
            }

            r
        }
    }

    fn rotate_subtree(&mut self, node: *mut AVLNODE) -> *mut AVLNODE {
        unsafe {
            let p = node;
            let f = (*p).up;
            let mut new_root = ptr::null_mut();

            if (*p).bal < 0 {
                let q = (*p).left;
                let r = (*q).right;

                if (*q).bal <= 0 {
                    if f.is_null() {
                        self.root = q;
                    } else if (*p).flag == 0 {
                        (*f).left = q;
                    } else {
                        (*f).right = q;
                    }

                    (*p).rank -= (*q).rank;
                    (*q).up = f;
                    (*q).flag = (*p).flag;
                    (*q).bal += 1;
                    (*q).right = p;
                    (*p).up = q;
                    (*p).flag = 1;
                    (*p).bal = -(*q).bal;
                    (*p).left = r;

                    if !r.is_null() {
                        (*r).up = p;
                        (*r).flag = 0;
                    }

                    new_root = q;
                } else {
                    let x = (*r).left;
                    let y = (*r).right;

                    if f.is_null() {
                        self.root = r;
                    } else if (*p).flag == 0 {
                        (*f).left = r;
                    } else {
                        (*f).right = r;
                    }

                    (*p).rank -= (*q).rank + (*r).rank;
                    (*r).rank += (*q).rank;
                    (*p).bal = if (*r).bal >= 0 { 0 } else { 1 };
                    (*q).bal = if (*r).bal <= 0 { 0 } else { -1 };
                    (*r).up = f;
                    (*r).flag = (*p).flag;
                    (*r).bal = 0;
                    (*r).left = q;
                    (*r).right = p;
                    (*p).up = r;
                    (*p).flag = 1;
                    (*p).left = y;
                    (*q).up = r;
                    (*q).flag = 0;
                    (*q).right = x;

                    if !x.is_null() {
                        (*x).up = q;
                        (*x).flag = 1;
                    }
                    if !y.is_null() {
                        (*y).up = p;
                        (*y).flag = 0;
                    }

                    new_root = r;
                }
            } else {
                let q = (*p).right;
                let r = (*q).left;

                if (*q).bal >= 0 {
                    if f.is_null() {
                        self.root = q;
                    } else if (*p).flag == 0 {
                        (*f).left = q;
                    } else {
                        (*f).right = q;
                    }

                    (*q).rank += (*p).rank;
                    (*q).up = f;
                    (*q).flag = (*p).flag;
                    (*q).bal -= 1;
                    (*q).left = p;
                    (*p).up = q;
                    (*p).flag = 0;
                    (*p).bal = -(*q).bal;
                    (*p).right = r;

                    if !r.is_null() {
                        (*r).up = p;
                        (*r).flag = 1;
                    }

                    new_root = q;
                } else {
                    let x = (*r).left;
                    let y = (*r).right;

                    if f.is_null() {
                        self.root = r;
                    } else if (*p).flag == 0 {
                        (*f).left = r;
                    } else {
                        (*f).right = r;
                    }

                    (*q).rank -= (*r).rank;
                    (*r).rank += (*p).rank;
                    (*p).bal = if (*r).bal <= 0 { 0 } else { -1 };
                    (*q).bal = if (*r).bal >= 0 { 0 } else { 1 };
                    (*r).up = f;
                    (*r).flag = (*p).flag;
                    (*r).bal = 0;
                    (*r).left = p;
                    (*r).right = q;
                    (*p).up = r;
                    (*p).flag = 0;
                    (*p).right = x;
                    (*q).up = r;
                    (*q).flag = 1;
                    (*q).left = y;

                    if !x.is_null() {
                        (*x).up = p;
                        (*x).flag = 1;
                    }
                    if !y.is_null() {
                        (*y).up = q;
                        (*y).flag = 0;
                    }

                    new_root = r;
                }
            }

            new_root
        }
    }
}

extern "C" {
    fn _glp_dmp_create_pool() -> *mut DMP;
    fn _glp_dmp_delete_pool(pool: *mut DMP);
    fn _glp_dmp_get_atom(pool: *mut DMP, size: i32) -> *mut ();
    fn _glp_dmp_free_atom(pool: *mut DMP, atom: *mut (), size: i32);
    fn glp_alloc(n: i32, size: i32) -> *mut ();
    fn glp_free(ptr: *mut ());
}