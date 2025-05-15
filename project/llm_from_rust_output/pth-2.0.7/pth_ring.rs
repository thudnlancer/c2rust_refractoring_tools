use std::ptr;
use std::cell::UnsafeCell;

type PthRingNodePtr = Option<Box<PthRingNode>>;

#[derive(Debug)]
struct PthRingNode {
    next: UnsafeCell<PthRingNodePtr>,
    prev: UnsafeCell<PthRingNodePtr>,
}

impl PthRingNode {
    fn new() -> Self {
        PthRingNode {
            next: UnsafeCell::new(None),
            prev: UnsafeCell::new(None),
        }
    }
}

#[derive(Debug)]
pub struct PthRing {
    hook: UnsafeCell<PthRingNodePtr>,
    nodes: usize,
}

impl PthRing {
    pub fn new() -> Self {
        PthRing {
            hook: UnsafeCell::new(None),
            nodes: 0,
        }
    }

    pub fn init(&mut self) {
        *self.hook.get_mut() = None;
        self.nodes = 0;
    }

    pub fn insert_after(&mut self, rn1: &mut PthRingNode, rn2: &mut PthRingNode) {
        unsafe {
            *rn2.prev.get() = Some(Box::from_raw(rn1));
            *rn2.next.get() = (*rn1.next.get()).take();
            if let Some(ref mut next) = *rn2.next.get() {
                *next.prev.get() = Some(Box::from_raw(rn2));
            }
            *rn1.next.get() = Some(Box::from_raw(rn2));
            self.nodes += 1;
        }
    }

    pub fn insert_before(&mut self, rn1: &mut PthRingNode, rn2: &mut PthRingNode) {
        unsafe {
            *rn2.next.get() = Some(Box::from_raw(rn1));
            *rn2.prev.get() = (*rn1.prev.get()).take();
            if let Some(ref mut prev) = *rn2.prev.get() {
                *prev.next.get() = Some(Box::from_raw(rn2));
            }
            *rn1.prev.get() = Some(Box::from_raw(rn2));
            self.nodes += 1;
        }
    }

    pub fn delete(&mut self, rn: &mut PthRingNode) {
        unsafe {
            let hook = self.hook.get_mut();
            if let Some(h) = hook {
                if ptr::eq(h.as_ref(), rn) && 
                   ptr::eq((*rn.prev.get()).as_ref().unwrap().as_ref(), rn) && 
                   ptr::eq((*rn.next.get()).as_ref().unwrap().as_ref(), rn) {
                    *hook = None;
                } else {
                    if ptr::eq(h.as_ref(), rn) {
                        *hook = (*rn.next.get()).take();
                    }
                    if let Some(ref mut prev) = *rn.prev.get() {
                        *prev.next.get() = (*rn.next.get()).take();
                    }
                    if let Some(ref mut next) = *rn.next.get() {
                        *next.prev.get() = (*rn.prev.get()).take();
                    }
                }
            }
            self.nodes -= 1;
        }
    }

    pub fn prepend(&mut self, rn: &mut PthRingNode) {
        unsafe {
            let hook = self.hook.get_mut();
            if hook.is_none() {
                *hook = Some(Box::from_raw(rn));
                *rn.next.get() = Some(Box::from_raw(rn));
                *rn.prev.get() = Some(Box::from_raw(rn));
            } else {
                *rn.next.get() = hook.take();
                *rn.prev.get() = (*hook.as_ref().unwrap().prev.get()).take();
                if let Some(ref mut next) = *rn.next.get() {
                    *next.prev.get() = Some(Box::from_raw(rn));
                }
                if let Some(ref mut prev) = *rn.prev.get() {
                    *prev.next.get() = Some(Box::from_raw(rn));
                }
                *hook = Some(Box::from_raw(rn));
            }
            self.nodes += 1;
        }
    }

    pub fn append(&mut self, rn: &mut PthRingNode) {
        unsafe {
            let hook = self.hook.get_mut();
            if hook.is_none() {
                *hook = Some(Box::from_raw(rn));
                *rn.next.get() = Some(Box::from_raw(rn));
                *rn.prev.get() = Some(Box::from_raw(rn));
            } else {
                *rn.next.get() = hook.take();
                *rn.prev.get() = (*hook.as_ref().unwrap().prev.get()).take();
                if let Some(ref mut next) = *rn.next.get() {
                    *next.prev.get() = Some(Box::from_raw(rn));
                }
                if let Some(ref mut prev) = *rn.prev.get() {
                    *prev.next.get() = Some(Box::from_raw(rn));
                }
            }
            self.nodes += 1;
        }
    }

    pub fn pop(&mut self) -> Option<Box<PthRingNode>> {
        let rn = self.hook.get_mut().take();
        if let Some(ref mut node) = rn {
            self.delete(node);
        }
        rn
    }

    pub fn favorite(&mut self, rn: &mut PthRingNode) -> bool {
        if self.hook.get_mut().is_none() {
            return false;
        }
        unsafe {
            if ptr::eq(self.hook.get_mut().as_ref().unwrap().as_ref(), rn) {
                return true;
            }
            self.delete(rn);
            self.prepend(rn);
            true
        }
    }

    pub fn dequeue(&mut self) -> Option<Box<PthRingNode>> {
        unsafe {
            let hook = self.hook.get_mut();
            if hook.is_none() {
                return None;
            }
            let rn = (*hook.as_ref().unwrap().prev.get()).take();
            if let Some(ref mut node) = rn {
                self.delete(node);
            }
            rn
        }
    }

    pub fn contains(&self, rns: &PthRingNode) -> bool {
        unsafe {
            let mut rn = self.hook.get();
            if rn.is_none() {
                return false;
            }
            let start = rn.unwrap();
            loop {
                if ptr::eq(rn.unwrap().as_ref(), rns) {
                    return true;
                }
                rn = (*rn.unwrap().next.get());
                if ptr::eq(rn.unwrap().as_ref(), start.as_ref()) {
                    break;
                }
            }
            false
        }
    }
}