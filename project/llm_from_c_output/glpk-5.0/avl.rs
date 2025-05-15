use std::cmp::Ordering;
use std::ptr;

struct AvlTree<T> {
    root: Option<Box<AvlNode<T>>>,
    size: usize,
    height: usize,
    comparator: Box<dyn Fn(&T, &T) -> Ordering>,
}

struct AvlNode<T> {
    key: T,
    rank: usize,
    node_type: i32,
    link: Option<Box<dyn std::any::Any>>,
    parent: *mut AvlNode<T>,
    flag: i8,
    balance: i8,
    left: Option<Box<AvlNode<T>>>,
    right: Option<Box<AvlNode<T>>>,
}

impl<T> AvlTree<T> {
    pub fn new(comparator: impl Fn(&T, &T) -> Ordering + 'static) -> Self {
        AvlTree {
            root: None,
            size: 0,
            height: 0,
            comparator: Box::new(comparator),
        }
    }

    pub fn strcmp(_info: &(), key1: &str, key2: &str) -> Ordering {
        key1.cmp(key2)
    }

    pub fn insert(&mut self, key: T) -> &mut AvlNode<T> {
        let mut parent = ptr::null_mut();
        let mut current = &mut self.root;
        let mut flag = 0i8;

        while let Some(node) = current {
            parent = node.as_mut() as *mut _;
            match (self.comparator)(&key, &node.key) {
                Ordering::Less | Ordering::Equal => {
                    flag = 0;
                    node.rank += 1;
                    current = &mut node.left;
                }
                Ordering::Greater => {
                    flag = 1;
                    current = &mut node.right;
                }
            }
        }

        let mut new_node = Box::new(AvlNode {
            key,
            rank: 1,
            node_type: 0,
            link: None,
            parent,
            flag,
            balance: 0,
            left: None,
            right: None,
        });

        let new_node_ptr = new_node.as_mut() as *mut _;

        if parent.is_null() {
            self.root = Some(new_node);
        } else {
            unsafe {
                if flag == 0 {
                    (*parent).left = Some(new_node);
                } else {
                    (*parent).right = Some(new_node);
                }
            }
        }

        self.size += 1;
        self.rebalance_after_insert(new_node_ptr);
        unsafe { &mut *new_node_ptr }
    }

    fn rebalance_after_insert(&mut self, mut node: *mut AvlNode<T>) {
        let mut parent = unsafe { (*node).parent };
        let mut flag = unsafe { (*node).flag };

        while !parent.is_null() {
            if flag == 0 {
                unsafe {
                    if (*parent).balance > 0 {
                        (*parent).balance = 0;
                        break;
                    }
                    if (*parent).balance < 0 {
                        self.rotate(parent);
                        break;
                    }
                    (*parent).balance = -1;
                    flag = (*parent).flag;
                    parent = (*parent).parent;
                }
            } else {
                unsafe {
                    if (*parent).balance < 0 {
                        (*parent).balance = 0;
                        break;
                    }
                    if (*parent).balance > 0 {
                        self.rotate(parent);
                        break;
                    }
                    (*parent).balance = 1;
                    flag = (*parent).flag;
                    parent = (*parent).parent;
                }
            }
        }

        if parent.is_null() {
            self.height += 1;
        }
    }

    pub fn set_node_type(&mut self, node: &mut AvlNode<T>, node_type: i32) {
        node.node_type = node_type;
    }

    pub fn set_node_link(&mut self, node: &mut AvlNode<T>, link: Box<dyn std::any::Any>) {
        node.link = Some(link);
    }

    pub fn find(&self, key: &T) -> Option<&AvlNode<T>> {
        let mut current = self.root.as_ref();
        while let Some(node) = current {
            match (self.comparator)(key, &node.key) {
                Ordering::Equal => return Some(node),
                Ordering::Less => current = node.left.as_ref(),
                Ordering::Greater => current = node.right.as_ref(),
            }
        }
        None
    }

    pub fn get_node_type(node: &AvlNode<T>) -> i32 {
        node.node_type
    }

    pub fn get_node_link(node: &AvlNode<T>) -> Option<&Box<dyn std::any::Any>> {
        node.link.as_ref()
    }

    fn find_next_node(&self, node: Option<&AvlNode<T>>) -> Option<&AvlNode<T>> {
        if self.root.is_none() {
            return None;
        }

        if let Some(p) = node {
            if let Some(right) = p.right.as_ref() {
                let mut q = right;
                while let Some(left) = q.left.as_ref() {
                    q = left;
                }
                Some(q)
            } else {
                let mut p = p;
                loop {
                    if p.flag == 0 {
                        return p.parent.as_ref().map(|parent| unsafe { &*parent });
                    }
                    p = unsafe { &*p.parent };
                }
            }
        } else {
            let mut q = self.root.as_ref().unwrap();
            while let Some(left) = q.left.as_ref() {
                q = left;
            }
            Some(q)
        }
    }

    pub fn delete(&mut self, node: &mut AvlNode<T>) {
        let mut p = node as *mut _;
        let mut f = unsafe { (*p).parent };
        let mut flag = unsafe { (*p).flag };

        // Adjust ranks
        let mut q = p;
        let mut parent = unsafe { (*q).parent };
        while !parent.is_null() {
            unsafe {
                if (*q).flag == 0 {
                    (*parent).rank -= 1;
                }
                q = parent;
                parent = (*q).parent;
            }
        }

        // Remove node
        let child = if unsafe { (*p).left.is_none() } {
            unsafe { (*p).right.take() }
        } else {
            unsafe { (*p).left.take() }
        };

        if f.is_null() {
            self.root = child;
        } else {
            unsafe {
                if flag == 0 {
                    (*f).left = child;
                } else {
                    (*f).right = child;
                }
            }
        }

        if let Some(mut child) = child {
            unsafe {
                child.parent = f;
                child.flag = flag;
            }
        }

        self.size -= 1;
        self.rebalance_after_delete(f, flag);

        if f.is_null() {
            self.height -= 1;
        }
    }

    fn rebalance_after_delete(&mut self, mut f: *mut AvlNode<T>, mut flag: i8) {
        while !f.is_null() {
            if flag == 0 {
                unsafe {
                    if (*f).balance == 0 {
                        (*f).balance = 1;
                        break;
                    }
                    if (*f).balance < 0 {
                        (*f).balance = 0;
                    } else {
                        f = self.rotate(f);
                        if (*f).balance < 0 {
                            break;
                        }
                    }
                    flag = (*f).flag;
                    f = (*f).parent;
                }
            } else {
                unsafe {
                    if (*f).balance == 0 {
                        (*f).balance = -1;
                        break;
                    }
                    if (*f).balance > 0 {
                        (*f).balance = 0;
                    } else {
                        f = self.rotate(f);
                        if (*f).balance > 0 {
                            break;
                        }
                    }
                    flag = (*f).flag;
                    f = (*f).parent;
                }
            }
        }
    }

    fn rotate(&mut self, node: *mut AvlNode<T>) -> *mut AvlNode<T> {
        unsafe {
            let p = node;
            if (*p).balance < 0 {
                // Left rotation
                let f = (*p).parent;
                let q = (*p).left.as_mut().unwrap();
                let r = q.right.as_mut();

                if q.balance <= 0 {
                    // Single left rotation
                    if f.is_null() {
                        self.root = Some(Box::from_raw(q));
                    } else {
                        if (*p).flag == 0 {
                            (*f).left = Some(Box::from_raw(q));
                        } else {
                            (*f).right = Some(Box::from_raw(q));
                        }
                    }

                    (*p).rank -= q.rank;
                    q.parent = f;
                    q.flag = (*p).flag;
                    q.balance += 1;
                    q.right = Some(Box::from_raw(p));

                    (*p).parent = q as *mut _;
                    (*p).flag = 1;
                    (*p).balance = -q.balance;
                    (*p).left = r.map(|r| Box::from_raw(r));

                    if let Some(r) = r {
                        r.parent = p;
                        r.flag = 0;
                    }

                    q as *mut _
                } else {
                    // Double left rotation
                    let r = q.right.as_mut().unwrap();
                    let x = r.left.as_mut();
                    let y = r.right.as_mut();

                    if f.is_null() {
                        self.root = Some(Box::from_raw(r));
                    } else {
                        if (*p).flag == 0 {
                            (*f).left = Some(Box::from_raw(r));
                        } else {
                            (*f).right = Some(Box::from_raw(r));
                        }
                    }

                    (*p).rank -= q.rank + r.rank;
                    r.rank += q.rank;
                    (*p).balance = if r.balance >= 0 { 0 } else { 1 };
                    q.balance = if r.balance <= 0 { 0 } else { -1 };
                    r.parent = f;
                    r.flag = (*p).flag;
                    r.balance = 0;
                    r.left = Some(Box::from_raw(q));
                    r.right = Some(Box::from_raw(p));

                    (*p).parent = r as *mut _;
                    (*p).flag = 1;
                    (*p).left = y.map(|y| Box::from_raw(y));

                    (*q).parent = r as *mut _;
                    (*q).flag = 0;
                    (*q).right = x.map(|x| Box::from_raw(x));

                    if let Some(x) = x {
                        x.parent = q as *mut _;
                        x.flag = 1;
                    }

                    if let Some(y) = y {
                        y.parent = p;
                        y.flag = 0;
                    }

                    r as *mut _
                }
            } else {
                // Right rotation
                let f = (*p).parent;
                let q = (*p).right.as_mut().unwrap();
                let r = q.left.as_mut();

                if q.balance >= 0 {
                    // Single right rotation
                    if f.is_null() {
                        self.root = Some(Box::from_raw(q));
                    } else {
                        if (*p).flag == 0 {
                            (*f).left = Some(Box::from_raw(q));
                        } else {
                            (*f).right = Some(Box::from_raw(q));
                        }
                    }

                    q.rank += (*p).rank;
                    q.parent = f;
                    q.flag = (*p).flag;
                    q.balance -= 1;
                    q.left = Some(Box::from_raw(p));

                    (*p).parent = q as *mut _;
                    (*p).flag = 0;
                    (*p).balance = -q.balance;
                    (*p).right = r.map(|r| Box::from_raw(r));

                    if let Some(r) = r {
                        r.parent = p;
                        r.flag = 1;
                    }

                    q as *mut _
                } else {
                    // Double right rotation
                    let r = q.left.as_mut().unwrap();
                    let x = r.left.as_mut();
                    let y = r.right.as_mut();

                    if f.is_null() {
                        self.root = Some(Box::from_raw(r));
                    } else {
                        if (*p).flag == 0 {
                            (*f).left = Some(Box::from_raw(r));
                        } else {
                            (*f).right = Some(Box::from_raw(r));
                        }
                    }

                    q.rank -= r.rank;
                    r.rank += (*p).rank;
                    (*p).balance = if r.balance <= 0 { 0 } else { -1 };
                    q.balance = if r.balance >= 0 { 0 } else { 1 };
                    r.parent = f;
                    r.flag = (*p).flag;
                    r.balance = 0;
                    r.left = Some(Box::from_raw(p));
                    r.right = Some(Box::from_raw(q));

                    (*p).parent = r as *mut _;
                    (*p).flag = 0;
                    (*p).right = x.map(|x| Box::from_raw(x));

                    (*q).parent = r as *mut _;
                    (*q).flag = 1;
                    (*q).left = y.map(|y| Box::from_raw(y));

                    if let Some(x) = x {
                        x.parent = p;
                        x.flag = 1;
                    }

                    if let Some(y) = y {
                        y.parent = q as *mut _;
                        y.flag = 0;
                    }

                    r as *mut _
                }
            }
        }
    }
}

impl<T> Drop for AvlTree<T> {
    fn drop(&mut self) {
        // Rust's ownership system will automatically clean up the tree
    }
}