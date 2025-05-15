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
**  pth_pqueue.rs: Pth thread priority queues (Rust implementation)
*/

use std::ptr::NonNull;

pub struct Pth {
    q_prev: Option<NonNull<Pth>>,
    q_next: Option<NonNull<Pth>>,
    q_prio: i32,
    // Other Pth fields...
}

pub struct PthPqueue {
    q_head: Option<NonNull<Pth>>,
    q_num: i32,
}

impl PthPqueue {
    /// Initialize a priority queue; O(1)
    pub fn init(&mut self) {
        self.q_head = None;
        self.q_num = 0;
    }

    /// Insert thread into priority queue; O(n)
    pub fn insert(&mut self, prio: i32, mut t: NonNull<Pth>) {
        unsafe {
            if self.q_head.is_none() || self.q_num == 0 {
                // Add as first element
                t.as_mut().q_prev = Some(t);
                t.as_mut().q_next = Some(t);
                t.as_mut().q_prio = prio;
                self.q_head = Some(t);
            } else if self.q_head.unwrap().as_ref().q_prio < prio {
                // Add as new head of queue
                let head = self.q_head.unwrap();
                t.as_mut().q_prev = head.as_ref().q_prev;
                t.as_mut().q_next = Some(head);
                t.as_mut().q_prev.unwrap().as_mut().q_next = Some(t);
                t.as_mut().q_next.unwrap().as_mut().q_prev = Some(t);
                t.as_mut().q_prio = prio;
                t.as_mut().q_next.unwrap().as_mut().q_prio = prio - t.as_ref().q_next.unwrap().as_ref().q_prio;
                self.q_head = Some(t);
            } else {
                // Insert after elements with greater or equal priority
                let mut c = self.q_head.unwrap();
                let mut p = c.as_ref().q_prio;
                while (p - c.as_ref().q_next.unwrap().as_ref().q_prio) >= prio 
                    && c.as_ref().q_next != self.q_head {
                    c = c.as_ref().q_next.unwrap();
                    p -= c.as_ref().q_prio;
                }
                t.as_mut().q_prev = Some(c);
                t.as_mut().q_next = c.as_ref().q_next;
                t.as_mut().q_prev.unwrap().as_mut().q_next = Some(t);
                t.as_mut().q_next.unwrap().as_mut().q_prev = Some(t);
                t.as_mut().q_prio = p - prio;
                if t.as_ref().q_next != self.q_head {
                    t.as_mut().q_next.unwrap().as_mut().q_prio -= t.as_ref().q_prio;
                }
            }
            self.q_num += 1;
        }
    }

    /// Remove thread with maximum priority from priority queue; O(1)
    pub fn delmax(&mut self) -> Option<NonNull<Pth>> {
        if self.q_head.is_none() {
            return None;
        }

        unsafe {
            let t = self.q_head?;
            
            if t.as_ref().q_next == Some(t) {
                // Remove the last element and make queue empty
                t.as_mut().q_next = None;
                t.as_mut().q_prev = None;
                t.as_mut().q_prio = 0;
                self.q_head = None;
                self.q_num = 0;
            } else {
                // Remove head of queue
                t.as_mut().q_prev.unwrap().as_mut().q_next = t.as_ref().q_next;
                t.as_mut().q_next.unwrap().as_mut().q_prev = t.as_ref().q_prev;
                t.as_mut().q_next.unwrap().as_mut().q_prio = t.as_ref().q_prio - t.as_ref().q_next.unwrap().as_ref().q_prio;
                t.as_mut().q_prio = 0;
                self.q_head = t.as_ref().q_next;
                self.q_num -= 1;
            }
            Some(t)
        }
    }

    /// Remove thread from priority queue; O(n)
    pub fn delete(&mut self, mut t: NonNull<Pth>) {
        unsafe {
            if self.q_head.is_none() {
                return;
            }

            if Some(t) == self.q_head {
                if t.as_ref().q_next == Some(t) {
                    // Remove the last element and make queue empty
                    t.as_mut().q_next = None;
                    t.as_mut().q_prev = None;
                    t.as_mut().q_prio = 0;
                    self.q_head = None;
                    self.q_num = 0;
                } else {
                    // Remove head of queue
                    t.as_mut().q_prev.unwrap().as_mut().q_next = t.as_ref().q_next;
                    t.as_mut().q_next.unwrap().as_mut().q_prev = t.as_ref().q_prev;
                    t.as_mut().q_next.unwrap().as_mut().q_prio = t.as_ref().q_prio - t.as_ref().q_next.unwrap().as_ref().q_prio;
                    t.as_mut().q_prio = 0;
                    self.q_head = t.as_ref().q_next;
                    self.q_num -= 1;
                }
            } else {
                t.as_mut().q_prev.unwrap().as_mut().q_next = t.as_ref().q_next;
                t.as_mut().q_next.unwrap().as_mut().q_prev = t.as_ref().q_prev;
                if t.as_ref().q_next != self.q_head {
                    t.as_mut().q_next.unwrap().as_mut().q_prio += t.as_ref().q_prio;
                }
                t.as_mut().q_prio = 0;
                self.q_num -= 1;
            }
        }
    }

    /// Determine priority required to favorite a thread; O(1)
    pub fn favorite_prio(&self) -> i32 {
        match self.q_head {
            Some(head) => unsafe { head.as_ref().q_prio + 1 },
            None => i32::MAX, // PTH_PRIO_MAX
        }
    }

    /// Move a thread inside queue to the top; O(n)
    pub fn favorite(&mut self, t: NonNull<Pth>) -> bool {
        if self.q_head.is_none() || self.q_num == 0 {
            return false;
        }
        // Element is already at top
        if self.q_num == 1 {
            return true;
        }
        // Move to top
        self.delete(t);
        self.insert(self.favorite_prio(), t);
        true
    }

    /// Increase priority of all(!) threads in queue; O(1)
    pub fn increase(&mut self) {
        unsafe {
            if let Some(mut head) = self.q_head {
                head.as_mut().q_prio += 1;
            }
        }
    }

    /// Return number of elements in priority queue: O(1)
    pub fn elements(&self) -> i32 {
        self.q_num
    }

    /// Walk to first thread in queue; O(1)
    pub fn head(&self) -> Option<NonNull<Pth>> {
        self.q_head
    }

    /// Walk to last thread in queue
    pub fn tail(&self) -> Option<NonNull<Pth>> {
        unsafe {
            self.q_head.map(|head| head.as_ref().q_prev.unwrap())
        }
    }

    /// Walk to next or previous thread in queue; O(1)
    pub fn walk(&self, t: NonNull<Pth>, direction: i32) -> Option<NonNull<Pth>> {
        unsafe {
            let tn = match direction {
                // PTH_WALK_PREV
                0 if t != self.q_head.unwrap() => t.as_ref().q_prev,
                // PTH_WALK_NEXT
                1 => {
                    let next = t.as_ref().q_next?;
                    if next == self.q_head.unwrap() {
                        None
                    } else {
                        Some(next)
                    }
                },
                _ => None,
            };
            tn
        }
    }

    /// Check whether a thread is in a queue; O(n)
    pub fn contains(&self, t: NonNull<Pth>) -> bool {
        let mut tc = self.head();
        while let Some(current) = tc {
            if current == t {
                return true;
            }
            tc = self.walk(current, 1); // PTH_WALK_NEXT
        }
        false
    }
}