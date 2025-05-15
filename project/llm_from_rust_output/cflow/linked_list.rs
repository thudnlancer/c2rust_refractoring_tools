use std::ptr;

type LinkedListFreeDataFn = Option<Box<dyn FnMut(*mut libc::c_void)>>;

struct LinkedListEntry {
    next: Option<Box<LinkedListEntry>>,
    prev: *mut LinkedListEntry,
    list: *mut LinkedList,
    data: *mut libc::c_void,
}

struct LinkedList {
    free_data: LinkedListFreeDataFn,
    head: Option<Box<LinkedListEntry>>,
    tail: *mut LinkedListEntry,
}

impl LinkedList {
    fn new(free_data: LinkedListFreeDataFn) -> Self {
        LinkedList {
            free_data,
            head: None,
            tail: ptr::null_mut(),
        }
    }

    fn append(&mut self, data: *mut libc::c_void) {
        let mut entry = Box::new(LinkedListEntry {
            next: None,
            prev: self.tail,
            list: self as *mut _,
            data,
        });

        let entry_ptr = &mut *entry as *mut LinkedListEntry;

        if !self.tail.is_null() {
            unsafe {
                (*self.tail).next = Some(entry);
            }
        } else {
            self.head = Some(entry);
        }

        self.tail = entry_ptr;
    }

    fn destroy(&mut self) {
        while let Some(mut entry) = self.head.take() {
            self.head = entry.next.take();

            if let Some(ref mut free_fn) = self.free_data {
                free_fn(entry.data);
            }

            // Box will be automatically dropped here
        }
        self.tail = ptr::null_mut();
    }

    fn unlink(&mut self, ent: *mut LinkedListEntry) {
        unsafe {
            let prev = (*ent).prev;
            let next = (*ent).next.take();

            if !prev.is_null() {
                (*prev).next = next;
            } else {
                self.head = next;
            }

            if let Some(mut next) = next {
                next.prev = prev;
                self.tail = if next.next.is_none() {
                    &mut *next as *mut _
                } else {
                    self.tail
                };
            } else {
                self.tail = prev;
            }

            if let Some(ref mut free_fn) = self.free_data {
                free_fn((*ent).data);
            }
        }
    }

    fn iterate<F>(&mut self, mut itr: F, data: *mut libc::c_void)
    where
        F: FnMut(*mut libc::c_void, *mut libc::c_void) -> bool,
    {
        let mut current = self.head.take();
        while let Some(mut entry) = current {
            let next = entry.next.take();
            if itr(entry.data, data) {
                self.unlink(&mut *entry);
            } else {
                if self.head.is_none() {
                    self.head = Some(entry);
                    self.tail = &mut *self.head.as_mut().unwrap() as *mut _;
                } else {
                    unsafe {
                        (*self.tail).next = Some(entry);
                        entry.prev = self.tail;
                        self.tail = &mut *entry as *mut _;
                    }
                }
            }
            current = next;
        }
    }

    fn contains(&self, data: *mut libc::c_void) -> bool {
        let mut current = &self.head;
        while let Some(entry) = current {
            if entry.data == data {
                return true;
            }
            current = &entry.next;
        }
        false
    }
}

impl Drop for LinkedList {
    fn drop(&mut self) {
        self.destroy();
    }
}

fn linked_list_create(fun: LinkedListFreeDataFn) -> LinkedList {
    LinkedList::new(fun)
}