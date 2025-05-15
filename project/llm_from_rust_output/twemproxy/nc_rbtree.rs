use std::ptr;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Color {
    Red,
    Black,
}

#[derive(Debug)]
struct RBNode<K, V> {
    key: K,
    value: V,
    color: Color,
    left: Option<Box<RBNode<K, V>>>,
    right: Option<Box<RBNode<K, V>>>,
    parent: Option<*mut RBNode<K, V>>,
}

impl<K: Ord, V> RBNode<K, V> {
    fn new(key: K, value: V) -> Self {
        RBNode {
            key,
            value,
            color: Color::Red,
            left: None,
            right: None,
            parent: None,
        }
    }

    fn is_red(&self) -> bool {
        self.color == Color::Red
    }

    fn is_black(&self) -> bool {
        self.color == Color::Black
    }

    fn set_color(&mut self, color: Color) {
        self.color = color;
    }
}

#[derive(Debug)]
pub struct RBTree<K: Ord, V> {
    root: Option<Box<RBNode<K, V>>>,
    sentinel: Box<RBNode<K, V>>,
}

impl<K: Ord, V> RBTree<K, V> {
    pub fn new() -> Self {
        let sentinel = Box::new(RBNode {
            key: unsafe { std::mem::zeroed() },
            value: unsafe { std::mem::zeroed() },
            color: Color::Black,
            left: None,
            right: None,
            parent: None,
        });

        RBTree {
            root: None,
            sentinel,
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        let mut new_node = Box::new(RBNode::new(key, value));
        new_node.left = Some(self.sentinel.clone());
        new_node.right = Some(self.sentinel.clone());

        if self.root.is_none() {
            new_node.color = Color::Black;
            self.root = Some(new_node);
            return;
        }

        let mut current = self.root.as_mut().unwrap();
        loop {
            if new_node.key < current.key {
                if current.left.as_ref().unwrap().ptr_eq(&self.sentinel) {
                    new_node.parent = Some(current.as_mut() as *mut _);
                    current.left = Some(new_node);
                    break;
                }
                current = current.left.as_mut().unwrap();
            } else {
                if current.right.as_ref().unwrap().ptr_eq(&self.sentinel) {
                    new_node.parent = Some(current.as_mut() as *mut _);
                    current.right = Some(new_node);
                    break;
                }
                current = current.right.as_mut().unwrap();
            }
        }

        self.fix_insert(new_node.as_mut() as *mut _);
    }

    fn fix_insert(&mut self, mut node: *mut RBNode<K, V>) {
        unsafe {
            while let Some(parent) = (*node).parent {
                if (*parent).is_black() {
                    break;
                }

                let grandparent = (*parent).parent.expect("Red node must have grandparent");
                if parent == (*grandparent).left.as_mut().unwrap().as_mut() as *mut _ {
                    let uncle = (*grandparent).right.as_mut().unwrap().as_mut() as *mut _;
                    if (*uncle).is_red() {
                        (*parent).set_color(Color::Black);
                        (*uncle).set_color(Color::Black);
                        (*grandparent).set_color(Color::Red);
                        node = grandparent;
                    } else {
                        if node == (*parent).right.as_mut().unwrap().as_mut() as *mut _ {
                            node = parent;
                            self.left_rotate(node);
                        }
                        (*parent).set_color(Color::Black);
                        (*grandparent).set_color(Color::Red);
                        self.right_rotate(grandparent);
                    }
                } else {
                    let uncle = (*grandparent).left.as_mut().unwrap().as_mut() as *mut _;
                    if (*uncle).is_red() {
                        (*parent).set_color(Color::Black);
                        (*uncle).set_color(Color::Black);
                        (*grandparent).set_color(Color::Red);
                        node = grandparent;
                    } else {
                        if node == (*parent).left.as_mut().unwrap().as_mut() as *mut _ {
                            node = parent;
                            self.right_rotate(node);
                        }
                        (*parent).set_color(Color::Black);
                        (*grandparent).set_color(Color::Red);
                        self.left_rotate(grandparent);
                    }
                }
            }
        }
        self.root.as_mut().unwrap().set_color(Color::Black);
    }

    fn left_rotate(&mut self, x: *mut RBNode<K, V>) {
        unsafe {
            let y = (*x).right.take().unwrap();
            (*x).right = y.left;
            if let Some(ref mut left) = (*x).right {
                left.parent = Some(x);
            }
            y.parent = (*x).parent;
            if let Some(parent) = (*x).parent {
                if x == (*parent).left.as_mut().unwrap().as_mut() as *mut _ {
                    (*parent).left = Some(y);
                } else {
                    (*parent).right = Some(y);
                }
            } else {
                self.root = Some(y);
            }
            y.left = Some(Box::from_raw(x));
            (*x).parent = Some(y.as_mut() as *mut _);
        }
    }

    fn right_rotate(&mut self, x: *mut RBNode<K, V>) {
        unsafe {
            let y = (*x).left.take().unwrap();
            (*x).left = y.right;
            if let Some(ref mut right) = (*x).left {
                right.parent = Some(x);
            }
            y.parent = (*x).parent;
            if let Some(parent) = (*x).parent {
                if x == (*parent).right.as_mut().unwrap().as_mut() as *mut _ {
                    (*parent).right = Some(y);
                } else {
                    (*parent).left = Some(y);
                }
            } else {
                self.root = Some(y);
            }
            y.right = Some(Box::from_raw(x));
            (*x).parent = Some(y.as_mut() as *mut _);
        }
    }

    pub fn min(&self) -> Option<&V> {
        self.root.as_ref().map(|root| {
            let mut current = root;
            while let Some(left) = current.left.as_ref() {
                if left.ptr_eq(&self.sentinel) {
                    break;
                }
                current = left;
            }
            &current.value
        })
    }
}

impl<K: Ord, V> Default for RBTree<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

trait PtrEq {
    fn ptr_eq(&self, other: &Self) -> bool;
}

impl<K, V> PtrEq for RBNode<K, V> {
    fn ptr_eq(&self, other: &Self) -> bool {
        ptr::eq(self, other)
    }
}