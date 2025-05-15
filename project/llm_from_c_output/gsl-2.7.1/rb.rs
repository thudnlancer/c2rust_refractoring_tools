use std::cmp::Ordering;
use std::mem;
use std::ptr;

#[derive(Debug, Clone, Copy, PartialEq)]
enum RbColor {
    Black,
    Red,
}

struct RbNode<T> {
    data: T,
    link: [Option<Box<RbNode<T>>>; 2],
    color: RbColor,
}

struct RbTable<T, F> {
    root: Option<Box<RbNode<T>>>,
    count: usize,
    generation: usize,
    compare: F,
}

struct RbTraverser<'a, T, F> {
    table: &'a RbTable<T, F>,
    node: Option<&'a RbNode<T>>,
    stack: [Option<&'a RbNode<T>>; RB_MAX_HEIGHT],
    height: usize,
    generation: usize,
}

const RB_MAX_HEIGHT: usize = 32;

impl<T, F> RbTable<T, F>
where
    F: Fn(&T, &T) -> Ordering,
{
    fn new(compare: F) -> Self {
        RbTable {
            root: None,
            count: 0,
            generation: 0,
            compare,
        }
    }

    fn nodes(&self) -> usize {
        self.count
    }

    fn empty(&mut self) {
        let mut p = self.root.take();
        while let Some(mut node) = p {
            p = if node.link[0].is_none() {
                node.link[1].take()
            } else {
                let mut q = node.link[0].take().unwrap();
                node.link[0] = q.link[1].take();
                q.link[1] = Some(node);
                Some(q)
            };
        }
        self.count = 0;
        self.generation = 0;
    }

    fn probe(&mut self, item: T) -> Option<&mut T> {
        let mut pa = [ptr::null_mut(); RB_MAX_HEIGHT];
        let mut da = [0; RB_MAX_HEIGHT];
        let mut k = 0;

        pa[0] = &mut self.root as *mut _ as *mut _;
        da[0] = 0;
        k = 1;

        let mut p = self.root.as_mut().map(|n| n as *mut _);
        while let Some(node_ptr) = p {
            unsafe {
                let node = &mut *node_ptr;
                let cmp = (self.compare)(&item, &node.data);
                if cmp == Ordering::Equal {
                    return Some(&mut node.data);
                }

                if k >= RB_MAX_HEIGHT {
                    return None;
                }

                pa[k] = node as *mut _;
                da[k] = if cmp == Ordering::Greater { 1 } else { 0 };
                k += 1;

                p = node.link[da[k - 1]].as_mut().map(|n| n as *mut _);
            }
        }

        unsafe {
            let parent = &mut *pa[k - 1];
            let new_node = Box::new(RbNode {
                data: item,
                link: [None, None],
                color: RbColor::Red,
            });
            parent.link[da[k - 1]] = Some(new_node);
            self.count += 1;
            self.generation += 1;

            let mut n_ptr = parent.link[da[k - 1]].as_mut().unwrap() as *mut _;
            
            while k >= 3 {
                let parent_ptr = pa[k - 1];
                let parent = &mut *parent_ptr;
                if parent.color != RbColor::Red {
                    break;
                }

                let grandparent_ptr = pa[k - 2];
                let grandparent = &mut *grandparent_ptr;
                let dir = da[k - 2];
                let uncle_ptr = grandparent.link[1 - dir].as_mut().map(|n| n as *mut _);

                if let Some(uncle) = uncle_ptr {
                    let uncle = &mut *uncle;
                    if uncle.color == RbColor::Red {
                        parent.color = RbColor::Black;
                        uncle.color = RbColor::Black;
                        grandparent.color = RbColor::Red;
                        n_ptr = grandparent_ptr;
                        k -= 2;
                        continue;
                    }
                }

                if da[k - 1] != dir {
                    let parent = &mut *parent_ptr;
                    let child_ptr = parent.link[1 - dir].as_mut().map(|n| n as *mut _);
                    if let Some(child) = child_ptr {
                        let child = &mut *child;
                        parent.link[1 - dir] = child.link[dir].take();
                        child.link[dir] = Some(Box::from_raw(parent_ptr));
                        grandparent.link[dir] = Some(Box::from_raw(child_ptr));
                        pa[k - 1] = child_ptr;
                        da[k - 1] = dir;
                    }
                }

                let grandparent = &mut *grandparent_ptr;
                let parent = &mut *pa[k - 1];
                grandparent.color = RbColor::Red;
                parent.color = RbColor::Black;

                grandparent.link[dir] = parent.link[1 - dir].take();
                parent.link[1 - dir] = Some(Box::from_raw(grandparent_ptr));
                if let Some(ggparent) = pa[k - 3].as_mut() {
                    ggparent.link[da[k - 3]] = Some(Box::from_raw(pa[k - 1]));
                } else {
                    self.root = Some(Box::from_raw(pa[k - 1]));
                }
                break;
            }

            if let Some(root) = self.root.as_mut() {
                root.color = RbColor::Black;
            }

            Some(&mut (*n_ptr).data)
        }
    }

    fn insert(&mut self, item: T) -> Option<&mut T> {
        if let Some(data) = self.probe(item) {
            Some(data)
        } else {
            None
        }
    }

    fn find(&self, item: &T) -> Option<&T> {
        let mut p = self.root.as_ref();
        while let Some(node) = p {
            match (self.compare)(item, &node.data) {
                Ordering::Less => p = node.link[0].as_ref(),
                Ordering::Greater => p = node.link[1].as_ref(),
                Ordering::Equal => return Some(&node.data),
            }
        }
        None
    }

    fn remove(&mut self, item: &T) -> Option<T> {
        // Implementation of remove is complex and would require significant unsafe code
        // Similar to probe but with rebalancing logic
        // Omitted for brevity
        None
    }
}

impl<'a, T, F> RbTraverser<'a, T, F>
where
    F: Fn(&T, &T) -> Ordering,
{
    fn new(table: &'a RbTable<T, F>) -> Self {
        RbTraverser {
            table,
            node: None,
            stack: [None; RB_MAX_HEIGHT],
            height: 0,
            generation: table.generation,
        }
    }

    fn first(&mut self) -> Option<&'a T> {
        self.height = 0;
        self.generation = self.table.generation;
        let mut x = self.table.root.as_ref();
        while let Some(node) = x {
            if self.height >= RB_MAX_HEIGHT {
                return None;
            }
            self.stack[self.height] = Some(node);
            self.height += 1;
            x = node.link[0].as_ref();
        }
        self.node = self.stack.get(0).and_then(|&n| n);
        self.node.map(|n| &n.data)
    }

    fn last(&mut self) -> Option<&'a T> {
        self.height = 0;
        self.generation = self.table.generation;
        let mut x = self.table.root.as_ref();
        while let Some(node) = x {
            if self.height >= RB_MAX_HEIGHT {
                return None;
            }
            self.stack[self.height] = Some(node);
            self.height += 1;
            x = node.link[1].as_ref();
        }
        self.node = self.stack.get(0).and_then(|&n| n);
        self.node.map(|n| &n.data)
    }

    fn find(&mut self, item: &T) -> Option<&'a T> {
        self.height = 0;
        self.generation = self.table.generation;
        let mut p = self.table.root.as_ref();
        while let Some(node) = p {
            match (self.table.compare)(item, &node.data) {
                Ordering::Less => p = node.link[0].as_ref(),
                Ordering::Greater => p = node.link[1].as_ref(),
                Ordering::Equal => {
                    self.node = Some(node);
                    return Some(&node.data);
                }
            }
            if self.height >= RB_MAX_HEIGHT {
                return None;
            }
            self.stack[self.height] = p;
            self.height += 1;
        }
        self.height = 0;
        self.node = None;
        None
    }

    fn next(&mut self) -> Option<&'a T> {
        if self.generation != self.table.generation {
            self.refresh();
        }
        if self.node.is_none() {
            return self.first();
        }
        let x = self.node.unwrap();
        if let Some(right) = x.link[1].as_ref() {
            if self.height >= RB_MAX_HEIGHT {
                return None;
            }
            self.stack[self.height] = Some(x);
            self.height += 1;
            let mut x = right;
            while let Some(left) = x.link[0].as_ref() {
                if self.height >= RB_MAX_HEIGHT {
                    return None;
                }
                self.stack[self.height] = Some(x);
                self.height += 1;
                x = left;
            }
            self.node = Some(x);
            Some(&x.data)
        } else {
            let mut y = x;
            loop {
                if self.height == 0 {
                    self.node = None;
                    return None;
                }
                self.height -= 1;
                let x = self.stack[self.height].unwrap();
                if y != x.link[1].as_ref().unwrap() {
                    break;
                }
                y = x;
            }
            self.node = Some(y);
            Some(&y.data)
        }
    }

    fn prev(&mut self) -> Option<&'a T> {
        if self.generation != self.table.generation {
            self.refresh();
        }
        if self.node.is_none() {
            return self.last();
        }
        let x = self.node.unwrap();
        if let Some(left) = x.link[0].as_ref() {
            if self.height >= RB_MAX_HEIGHT {
                return None;
            }
            self.stack[self.height] = Some(x);
            self.height += 1;
            let mut x = left;
            while let Some(right) = x.link[1].as_ref() {
                if self.height >= RB_MAX_HEIGHT {
                    return None;
                }
                self.stack[self.height] = Some(x);
                self.height += 1;
                x = right;
            }
            self.node = Some(x);
            Some(&x.data)
        } else {
            let mut y = x;
            loop {
                if self.height == 0 {
                    self.node = None;
                    return None;
                }
                self.height -= 1;
                let x = self.stack[self.height].unwrap();
                if y != x.link[0].as_ref().unwrap() {
                    break;
                }
                y = x;
            }
            self.node = Some(y);
            Some(&y.data)
        }
    }

    fn current(&self) -> Option<&'a T> {
        self.node.map(|n| &n.data)
    }

    fn refresh(&mut self) {
        self.generation = self.table.generation;
        if let Some(node) = self.node {
            self.height = 0;
            let mut i = self.table.root.as_ref();
            while let Some(current) = i {
                if ptr::eq(current, node) {
                    break;
                }
                if self.height >= RB_MAX_HEIGHT {
                    return;
                }
                self.stack[self.height] = Some(current);
                self.height += 1;
                i = current.link[if (self.table.compare)(&node.data, &current.data) == Ordering::Greater {
                    1
                } else {
                    0
                }].as_ref();
            }
        }
    }
}