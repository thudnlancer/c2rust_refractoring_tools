use std::ptr;

#[derive(Debug)]
enum Color {
    Red,
    Black,
}

#[derive(Debug)]
struct RBNode<T> {
    left: *mut RBNode<T>,
    right: *mut RBNode<T>,
    parent: *mut RBNode<T>,
    key: i64,
    data: Option<T>,
    color: Color,
}

impl<T> RBNode<T> {
    fn new() -> Self {
        RBNode {
            left: ptr::null_mut(),
            right: ptr::null_mut(),
            parent: ptr::null_mut(),
            key: 0,
            data: None,
            color: Color::Black, // Default to black for sentinel nodes
        }
    }

    fn is_red(&self) -> bool {
        matches!(self.color, Color::Red)
    }

    fn is_black(&self) -> bool {
        !self.is_red()
    }

    fn set_red(&mut self) {
        self.color = Color::Red;
    }

    fn set_black(&mut self) {
        self.color = Color::Black;
    }

    fn copy_color(&mut self, other: &Self) {
        self.color = match other.color {
            Color::Red => Color::Red,
            Color::Black => Color::Black,
        };
    }
}

#[derive(Debug)]
struct RBTree<T> {
    root: *mut RBNode<T>,
    sentinel: *mut RBNode<T>,
}

impl<T> RBTree<T> {
    fn new() -> Self {
        let sentinel = Box::into_raw(Box::new(RBNode::new()));
        RBTree {
            root: sentinel,
            sentinel,
        }
    }

    fn min(&self) -> Option<&RBNode<T>> {
        if self.root == self.sentinel {
            return None;
        }

        let mut node = unsafe { &*self.root };
        while unsafe { (*node.left) != *self.sentinel } {
            node = unsafe { &*node.left };
        }
        Some(node)
    }

    fn insert(&mut self, mut node: Box<RBNode<T>>) {
        unsafe {
            if self.root == self.sentinel {
                node.parent = ptr::null_mut();
                node.left = self.sentinel;
                node.right = self.sentinel;
                node.set_black();
                self.root = Box::into_raw(node);
                return;
            }

            let mut temp = self.root;
            let mut p = &mut self.root;

            loop {
                let current = &mut *temp;
                p = if node.key < current.key {
                    &mut current.left
                } else {
                    &mut current.right
                };

                if *p == self.sentinel {
                    break;
                }
                temp = *p;
            }

            *p = Box::into_raw(node);
            let node_ptr = *p;
            (*node_ptr).parent = temp;
            (*node_ptr).left = self.sentinel;
            (*node_ptr).right = self.sentinel;
            (*node_ptr).set_red();

            self.insert_fixup(node_ptr);
        }
    }

    unsafe fn insert_fixup(&mut self, mut node: *mut RBNode<T>) {
        while node != self.root && (*node).parent != ptr::null_mut() && (*node).parent != self.sentinel && (*(*node).parent).is_red() {
            let parent = (*node).parent;
            let grandparent = (*parent).parent;

            if parent == (*grandparent).left {
                let uncle = (*grandparent).right;
                if uncle != self.sentinel && (*uncle).is_red() {
                    (*parent).set_black();
                    (*uncle).set_black();
                    (*grandparent).set_red();
                    node = grandparent;
                } else {
                    if node == (*parent).right {
                        node = parent;
                        self.left_rotate(node);
                    }
                    (*parent).set_black();
                    (*grandparent).set_red();
                    self.right_rotate(grandparent);
                }
            } else {
                let uncle = (*grandparent).left;
                if uncle != self.sentinel && (*uncle).is_red() {
                    (*parent).set_black();
                    (*uncle).set_black();
                    (*grandparent).set_red();
                    node = grandparent;
                } else {
                    if node == (*parent).left {
                        node = parent;
                        self.right_rotate(node);
                    }
                    (*parent).set_black();
                    (*grandparent).set_red();
                    self.left_rotate(grandparent);
                }
            }
        }
        (*(self.root)).set_black();
    }

    unsafe fn left_rotate(&mut self, x: *mut RBNode<T>) {
        let y = (*x).right;
        (*x).right = (*y).left;

        if (*y).left != self.sentinel {
            (*(*y).left).parent = x;
        }

        (*y).parent = (*x).parent;

        if x == self.root {
            self.root = y;
        } else if x == (*(*x).parent).left {
            (*(*x).parent).left = y;
        } else {
            (*(*x).parent).right = y;
        }

        (*y).left = x;
        (*x).parent = y;
    }

    unsafe fn right_rotate(&mut self, x: *mut RBNode<T>) {
        let y = (*x).left;
        (*x).left = (*y).right;

        if (*y).right != self.sentinel {
            (*(*y).right).parent = x;
        }

        (*y).parent = (*x).parent;

        if x == self.root {
            self.root = y;
        } else if x == (*(*x).parent).right {
            (*(*x).parent).right = y;
        } else {
            (*(*x).parent).left = y;
        }

        (*y).right = x;
        (*x).parent = y;
    }

    fn delete(&mut self, node: *mut RBNode<T>) {
        unsafe {
            let mut subst;
            let mut temp;

            if (*node).left == self.sentinel {
                temp = (*node).right;
                subst = node;
            } else if (*node).right == self.sentinel {
                temp = (*node).left;
                subst = node;
            } else {
                subst = self.node_min((*node).right);
                temp = (*subst).right;
            }

            if subst == self.root {
                self.root = temp;
                (*temp).set_black();
                return;
            }

            let red = (*subst).is_red();

            if subst == (*(*subst).parent).left {
                (*(*subst).parent).left = temp;
            } else {
                (*(*subst).parent).right = temp;
            }

            if subst == node {
                (*temp).parent = (*subst).parent;
            } else {
                if (*subst).parent == node {
                    (*temp).parent = subst;
                } else {
                    (*temp).parent = (*subst).parent;
                }

                (*subst).left = (*node).left;
                (*subst).right = (*node).right;
                (*subst).parent = (*node).parent;
                (*subst).copy_color(&*node);

                if node == self.root {
                    self.root = subst;
                } else {
                    if node == (*(*node).parent).left {
                        (*(*node).parent).left = subst;
                    } else {
                        (*(*node).parent).right = subst;
                    }
                }

                if (*subst).left != self.sentinel {
                    (*(*subst).left).parent = subst;
                }

                if (*subst).right != self.sentinel {
                    (*(*subst).right).parent = subst;
                }
            }

            if red {
                return;
            }

            self.delete_fixup(temp);
        }
    }

    unsafe fn delete_fixup(&mut self, mut temp: *mut RBNode<T>) {
        while temp != self.root && (*temp).is_black() {
            let parent = (*temp).parent;
            if temp == (*parent).left {
                let mut w = (*parent).right;

                if (*w).is_red() {
                    (*w).set_black();
                    (*parent).set_red();
                    self.left_rotate(parent);
                    w = (*parent).right;
                }

                if (*(*w).left).is_black() && (*(*w).right).is_black() {
                    (*w).set_red();
                    temp = parent;
                } else {
                    if (*(*w).right).is_black() {
                        (*(*w).left).set_black();
                        (*w).set_red();
                        self.right_rotate(w);
                        w = (*parent).right;
                    }

                    (*w).copy_color(&*parent);
                    (*parent).set_black();
                    (*(*w).right).set_black();
                    self.left_rotate(parent);
                    temp = self.root;
                }
            } else {
                let mut w = (*parent).left;

                if (*w).is_red() {
                    (*w).set_black();
                    (*parent).set_red();
                    self.right_rotate(parent);
                    w = (*parent).left;
                }

                if (*(*w).left).is_black() && (*(*w).right).is_black() {
                    (*w).set_red();
                    temp = parent;
                } else {
                    if (*(*w).left).is_black() {
                        (*(*w).right).set_black();
                        (*w).set_red();
                        self.left_rotate(w);
                        w = (*parent).left;
                    }

                    (*w).copy_color(&*parent);
                    (*parent).set_black();
                    (*(*w).left).set_black();
                    self.right_rotate(parent);
                    temp = self.root;
                }
            }
        }

        (*temp).set_black();
    }

    unsafe fn node_min(&self, mut node: *mut RBNode<T>) -> *mut RBNode<T> {
        while (*node).left != self.sentinel {
            node = (*node).left;
        }
        node
    }
}

impl<T> Drop for RBTree<T> {
    fn drop(&mut self) {
        unsafe {
            if self.root != self.sentinel {
                self.free_tree(self.root);
            }
            Box::from_raw(self.sentinel);
        }
    }
}

impl<T> RBTree<T> {
    unsafe fn free_tree(&mut self, node: *mut RBNode<T>) {
        if (*node).left != self.sentinel {
            self.free_tree((*node).left);
        }
        if (*node).right != self.sentinel {
            self.free_tree((*node).right);
        }
        Box::from_raw(node);
    }
}