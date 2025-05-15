use std::cmp::Ordering;
use std::mem;
use std::ptr;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Color {
    Red,
    Black,
}

#[derive(Debug)]
struct Node<T> {
    data: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
    color: Color,
}

impl<T> Node<T> {
    fn new(data: T) -> Self {
        Node {
            data,
            left: None,
            right: None,
            color: Color::Red,
        }
    }
}

#[derive(Debug)]
pub struct RBTree<T, F>
where
    F: Fn(&T, &T) -> Ordering,
{
    root: Option<Box<Node<T>>>,
    compare: F,
    count: usize,
}

impl<T, F> RBTree<T, F>
where
    F: Fn(&T, &T) -> Ordering,
{
    pub fn new(compare: F) -> Self {
        RBTree {
            root: None,
            compare,
            count: 0,
        }
    }

    pub fn insert(&mut self, value: T) -> Option<&T> {
        let mut new_root = None;
        let result = self.insert_helper(&mut new_root, value);
        self.root = new_root;
        if let Some(node) = &mut self.root {
            node.color = Color::Black;
        }
        result
    }

    fn insert_helper(&mut self, node: &mut Option<Box<Node<T>>>, value: T) -> Option<&T> {
        match node {
            None => {
                *node = Some(Box::new(Node::new(value)));
                self.count += 1;
                Some(&node.as_ref().unwrap().data)
            }
            Some(ref mut current) => {
                let cmp = (self.compare)(&value, &current.data);
                match cmp {
                    Ordering::Less => {
                        let result = self.insert_helper(&mut current.left, value);
                        self.balance(node);
                        result
                    }
                    Ordering::Greater => {
                        let result = self.insert_helper(&mut current.right, value);
                        self.balance(node);
                        result
                    }
                    Ordering::Equal => None,
                }
            }
        }
    }

    fn balance(&mut self, node: &mut Option<Box<Node<T>>>) {
        if let Some(current) = node {
            if is_red(&current.right) && !is_red(&current.left) {
                self.rotate_left(node);
            }
            if is_red(&current.left) && is_red(&current.left.as_ref().unwrap().left) {
                self.rotate_right(node);
            }
            if is_red(&current.left) && is_red(&current.right) {
                flip_colors(current);
            }
        }
    }

    fn rotate_left(&mut self, node: &mut Option<Box<Node<T>>>) {
        if let Some(mut current) = node.take() {
            if let Some(mut new_root) = current.right.take() {
                current.right = new_root.left.take();
                new_root.color = current.color;
                current.color = Color::Red;
                new_root.left = Some(current);
                *node = Some(new_root);
            }
        }
    }

    fn rotate_right(&mut self, node: &mut Option<Box<Node<T>>>) {
        if let Some(mut current) = node.take() {
            if let Some(mut new_root) = current.left.take() {
                current.left = new_root.right.take();
                new_root.color = current.color;
                current.color = Color::Red;
                new_root.right = Some(current);
                *node = Some(new_root);
            }
        }
    }
}

fn is_red<T>(node: &Option<Box<Node<T>>>) -> bool {
    node.as_ref().map_or(false, |n| n.color == Color::Red)
}

fn flip_colors<T>(node: &mut Node<T>) {
    node.color = Color::Red;
    if let Some(ref mut left) = node.left {
        left.color = Color::Black;
    }
    if let Some(ref mut right) = node.right {
        right.color = Color::Black;
    }
}

#[derive(Debug)]
pub struct RBTreeIterator<'a, T> {
    stack: Vec<&'a Node<T>>,
    current: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for RBTreeIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(node) = self.current {
            self.stack.push(node);
            self.current = node.left.as_deref();
        }

        if let Some(node) = self.stack.pop() {
            self.current = node.right.as_deref();
            Some(&node.data)
        } else {
            None
        }
    }
}

impl<T, F> RBTree<T, F>
where
    F: Fn(&T, &T) -> Ordering,
{
    pub fn iter(&self) -> RBTreeIterator<'_, T> {
        RBTreeIterator {
            stack: Vec::new(),
            current: self.root.as_deref(),
        }
    }
}

pub struct RBTreeTraverser<'a, T> {
    tree: &'a RBTree<T, Box<dyn Fn(&T, &T) -> Ordering>>,
    stack: Vec<&'a Node<T>>,
    current: Option<&'a Node<T>>,
}

impl<'a, T> RBTreeTraverser<'a, T> {
    pub fn new(tree: &'a RBTree<T, Box<dyn Fn(&T, &T) -> Ordering>>) -> Self {
        RBTreeTraverser {
            tree,
            stack: Vec::new(),
            current: None,
        }
    }

    pub fn first(&mut self) -> Option<&'a T> {
        self.stack.clear();
        self.current = self.tree.root.as_deref();
        self.next()
    }

    pub fn last(&mut self) -> Option<&'a T> {
        self.stack.clear();
        self.current = self.tree.root.as_deref();
        while let Some(node) = self.current {
            self.stack.push(node);
            self.current = node.right.as_deref();
        }
        self.stack.pop().map(|node| &node.data)
    }

    pub fn next(&mut self) -> Option<&'a T> {
        while let Some(node) = self.current {
            self.stack.push(node);
            self.current = node.left.as_deref();
        }

        if let Some(node) = self.stack.pop() {
            self.current = node.right.as_deref();
            Some(&node.data)
        } else {
            None
        }
    }

    pub fn prev(&mut self) -> Option<&'a T> {
        while let Some(node) = self.current {
            self.stack.push(node);
            self.current = node.right.as_deref();
        }

        if let Some(node) = self.stack.pop() {
            self.current = node.left.as_deref();
            Some(&node.data)
        } else {
            None
        }
    }
}