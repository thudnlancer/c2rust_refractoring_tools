/*
 * This file is part of GNU cflow
 * Copyright (C) 1997, 2005, 2006, 2007, 2009, 2010 Sergey Poznyakoff
 *
 * GNU cflow is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 *
 * GNU cflow is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public
 * License along with GNU cflow; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston,
 * MA 02110-1301 USA
 */

use std::ptr::null_mut;
use std::mem;

type LinkedListFreeDataFn<T> = Option<Box<dyn FnMut(T)>>;

struct LinkedListEntry<T> {
    list: *mut LinkedList<T>,
    data: T,
    next: *mut LinkedListEntry<T>,
    prev: *mut LinkedListEntry<T>,
}

struct LinkedList<T> {
    free_data: LinkedListFreeDataFn<T>,
    head: *mut LinkedListEntry<T>,
    tail: *mut LinkedListEntry<T>,
}

impl<T> LinkedList<T> {
    fn deref_linked_list(plist: &mut Option<Box<LinkedList<T>>>) -> &mut LinkedList<T> {
        if plist.is_none() {
            *plist = Some(Box::new(LinkedList {
                free_data: None,
                head: null_mut(),
                tail: null_mut(),
            }));
        }
        plist.as_mut().unwrap()
    }

    fn create(fun: LinkedListFreeDataFn<T>) -> Box<LinkedList<T>> {
        Box::new(LinkedList {
            free_data: fun,
            head: null_mut(),
            tail: null_mut(),
        })
    }

    fn append(&mut self, data: T) {
        let entry = Box::into_raw(Box::new(LinkedListEntry {
            list: self,
            data,
            next: null_mut(),
            prev: self.tail,
        }));

        if !self.tail.is_null() {
            unsafe { (*self.tail).next = entry; }
        } else {
            self.head = entry;
        }
        self.tail = entry;
    }

    fn destroy(&mut self) {
        let mut current = self.head;
        while !current.is_null() {
            let next = unsafe { (*current).next };
            if let Some(ref mut free_fn) = self.free_data {
                let data = unsafe { mem::replace(&mut (*current).data, mem::zeroed()) };
                free_fn(data);
            }
            unsafe { Box::from_raw(current); }
            current = next;
        }
        self.head = null_mut();
        self.tail = null_mut();
    }

    fn unlink(&mut self, ent: *mut LinkedListEntry<T>) {
        unsafe {
            if !(*ent).prev.is_null() {
                (*(*ent).prev).next = (*ent).next;
            } else {
                self.head = (*ent).next;
            }

            if !(*ent).next.is_null() {
                (*(*ent).next).prev = (*ent).prev;
            } else {
                self.tail = (*ent).prev;
            }

            if let Some(ref mut free_fn) = self.free_data {
                let data = mem::replace(&mut (*ent).data, mem::zeroed());
                free_fn(data);
            }
            Box::from_raw(ent);
        }
    }

    fn iterate<F, U>(&mut self, mut itr: F, data: U)
    where
        F: FnMut(&mut T, U) -> bool,
    {
        let mut current = self.head;
        while !current.is_null() {
            let next = unsafe { (*current).next };
            if itr(unsafe { &mut (*current).data }, data) {
                self.unlink(current);
            }
            current = next;
        }
        if self.head.is_null() {
            self.destroy();
        }
    }

    fn contains(&self, data: &T) -> bool
    where
        T: PartialEq,
    {
        let mut current = self.head;
        while !current.is_null() {
            unsafe {
                if &(*current).data == data {
                    return true;
                }
                current = (*current).next;
            }
        }
        false
    }

    fn head(&self) -> *mut LinkedListEntry<T> {
        self.head
    }
}