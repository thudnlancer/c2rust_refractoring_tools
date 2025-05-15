/*
**  GNU Pth - The GNU Portable Threads
**  Copyright (c) 1999-2006 Ralf S. Engelschall <rse@engelschall.com>
**
**  This file is part of GNU Pth, a non-preemptive thread scheduling
**  library which can be found at http://www.gnu.org/software/pth/.
**
**  This library is free software; you can redistribute it and/or
**  modify it under the terms of the GNU Lesser General Public
**  License as published by the Free Software Foundation; either
**  version 2.1 of the License, or (at your option) any later version.
**
**  This library is distributed in the hope that it will be useful,
**  but WITHOUT ANY WARRANTY; without even the implied warranty of
**  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
**  Lesser General Public License for more details.
**
**  You should have received a copy of the GNU Lesser General Public
**  License along with this library; if not, write to the Free Software
**  Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA 02111-1307
**  USA, or contact Ralf S. Engelschall <rse@engelschall.com>.
**
**  pth_ring.rs: Pth ring data structure (Rust implementation)
*/

                             /* ``Unix was not designed to stop people
                                  from doing stupid things, because that
                                  would also stop them from doing clever
                                  things.''         --Doug Gwyn          */

use std::ptr::NonNull;

#[derive(Debug)]
pub struct PthRingNode<T> {
    pub data: T,
    pub next: Option<NonNull<PthRingNode<T>>>,
    pub prev: Option<NonNull<PthRingNode<T>>>,
}

#[derive(Debug)]
pub struct PthRing<T> {
    hook: Option<NonNull<PthRingNode<T>>>,
    nodes: usize,
}

impl<T> PthRing<T> {
    /// Initialize ring; O(1)
    pub fn new() -> Self {
        PthRing {
            hook: None,
            nodes: 0,
        }
    }

    /// Return number of nodes in ring; O(1)
    pub fn elements(&self) -> usize {
        self.nodes
    }

    /// Return first node in ring; O(1)
    pub fn first(&self) -> Option<NonNull<PthRingNode<T>>> {
        self.hook
    }

    /// Return last node in ring; O(1)
    pub fn last(&self) -> Option<NonNull<PthRingNode<T>>> {
        self.hook.and_then(|hook| unsafe { hook.as_ref().prev })
    }

    /// Walk to next node in ring; O(1)
    pub fn next(&self, node: NonNull<PthRingNode<T>>) -> Option<NonNull<PthRingNode<T>>> {
        unsafe {
            let next = node.as_ref().next?;
            if next == self.hook? {
                None
            } else {
                Some(next)
            }
        }
    }

    /// Walk to previous node in ring; O(1)
    pub fn prev(&self, node: NonNull<PthRingNode<T>>) -> Option<NonNull<PthRingNode<T>>> {
        unsafe {
            let prev = node.as_ref().prev?;
            if prev == self.hook?.as_ref().prev? {
                None
            } else {
                Some(prev)
            }
        }
    }

    /// Insert node into ring; O(1)
    pub fn insert(&mut self, node: NonNull<PthRingNode<T>>) {
        self.append(node);
    }

    /// Insert node after a second node in ring; O(1)
    pub fn insert_after(
        &mut self,
        rn1: NonNull<PthRingNode<T>>,
        rn2: NonNull<PthRingNode<T>>,
    ) {
        unsafe {
            rn2.as_ref().prev = Some(rn1);
            rn2.as_ref().next = rn1.as_ref().next;
            rn2.as_ref().prev.unwrap().as_ref().next = Some(rn2);
            rn2.as_ref().next.unwrap().as_ref().prev = Some(rn2);
            self.nodes += 1;
        }
    }

    /// Insert node before a second node in ring; O(1)
    pub fn insert_before(
        &mut self,
        rn1: NonNull<PthRingNode<T>>,
        rn2: NonNull<PthRingNode<T>>,
    ) {
        unsafe {
            rn2.as_ref().next = Some(rn1);
            rn2.as_ref().prev = rn1.as_ref().prev;
            rn2.as_ref().prev.unwrap().as_ref().next = Some(rn2);
            rn2.as_ref().next.unwrap().as_ref().prev = Some(rn2);
            self.nodes += 1;
        }
    }

    /// Delete a node from ring; O(1)
    pub fn delete(&mut self, rn: NonNull<PthRingNode<T>>) {
        unsafe {
            if self.hook == Some(rn) && rn.as_ref().prev == Some(rn) && rn.as_ref().next == Some(rn) {
                self.hook = None;
            } else {
                if self.hook == Some(rn) {
                    self.hook = rn.as_ref().next;
                }
                rn.as_ref().prev.unwrap().as_ref().next = rn.as_ref().next;
                rn.as_ref().next.unwrap().as_ref().prev = rn.as_ref().prev;
            }
            self.nodes -= 1;
        }
    }

    /// Prepend a node to ring; O(1)
    pub fn prepend(&mut self, rn: NonNull<PthRingNode<T>>) {
        unsafe {
            if self.hook.is_none() {
                rn.as_ref().next = Some(rn);
                rn.as_ref().prev = Some(rn);
                self.hook = Some(rn);
            } else {
                let hook = self.hook.unwrap();
                rn.as_ref().next = Some(hook);
                rn.as_ref().prev = hook.as_ref().prev;
                rn.as_ref().next.unwrap().as_ref().prev = Some(rn);
                rn.as_ref().prev.unwrap().as_ref().next = Some(rn);
                self.hook = Some(rn);
            }
            self.nodes += 1;
        }
    }

    /// Append a node to ring; O(1)
    pub fn append(&mut self, rn: NonNull<PthRingNode<T>>) {
        unsafe {
            if self.hook.is_none() {
                rn.as_ref().next = Some(rn);
                rn.as_ref().prev = Some(rn);
                self.hook = Some(rn);
            } else {
                let hook = self.hook.unwrap();
                rn.as_ref().next = Some(hook);
                rn.as_ref().prev = hook.as_ref().prev;
                rn.as_ref().next.unwrap().as_ref().prev = Some(rn);
                rn.as_ref().prev.unwrap().as_ref().next = Some(rn);
            }
            self.nodes += 1;
        }
    }

    /// Treat ring as stack: push node onto stack; O(1)
    pub fn push(&mut self, rn: NonNull<PthRingNode<T>>) {
        self.prepend(rn);
    }

    /// Treat ring as stack: pop node from stack; O(1)
    pub fn pop(&mut self) -> Option<NonNull<PthRingNode<T>>> {
        let rn = self.first();
        if let Some(rn) = rn {
            self.delete(rn);
        }
        rn
    }

    /// Treat ring as queue: favorite a node in the ring; O(1)
    pub fn favorite(&mut self, rn: NonNull<PthRingNode<T>>) -> bool {
        if self.hook.is_none() {
            return false;
        }
        if self.hook == Some(rn) {
            return true;
        }
        self.delete(rn);
        self.prepend(rn);
        true
    }

    /// Treat ring as queue: enqueue node; O(1)
    pub fn enqueue(&mut self, rn: NonNull<PthRingNode<T>>) {
        self.prepend(rn);
    }

    /// Treat ring as queue: dequeue node; O(1)
    pub fn dequeue(&mut self) -> Option<NonNull<PthRingNode<T>>> {
        let rn = self.last();
        if let Some(rn) = rn {
            self.delete(rn);
        }
        rn
    }

    /// Check whether node is contained in ring; O(n)
    pub fn contains(&self, rns: NonNull<PthRingNode<T>>) -> bool {
        let mut rn = match self.hook {
            Some(rn) => rn,
            None => return false,
        };

        loop {
            if rn == rns {
                return true;
            }
            unsafe {
                rn = match rn.as_ref().next {
                    Some(next) => next,
                    None => break,
                };
            }
            if rn == self.hook.unwrap() {
                break;
            }
        }
        false
    }
}

impl<T> Default for PthRing<T> {
    fn default() -> Self {
        Self::new()
    }
}