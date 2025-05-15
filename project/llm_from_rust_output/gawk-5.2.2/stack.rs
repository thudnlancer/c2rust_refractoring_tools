use std::ptr::NonNull;
use std::alloc::{self, Layout};

pub struct Stack {
    data: Vec<NonNull<()>>,
    index: isize,
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            data: Vec::with_capacity(20),
            index: -1,
        }
    }

    pub fn empty(&self) -> bool {
        self.index < 0
    }

    pub fn top(&self) -> Option<NonNull<()>> {
        if self.empty() {
            None
        } else {
            Some(self.data[self.index as usize])
        }
    }

    pub fn pop(&mut self) -> Option<NonNull<()>> {
        if self.empty() {
            None
        } else {
            let val = self.data[self.index as usize];
            self.index -= 1;
            Some(val)
        }
    }

    pub fn push(&mut self, object: NonNull<()>) -> bool {
        if (self.index + 1) as usize >= self.data.capacity() {
            let new_capacity = self.data.capacity() * 2;
            self.data.reserve(new_capacity);
        }

        self.index += 1;
        if self.index as usize >= self.data.len() {
            self.data.push(object);
        } else {
            self.data[self.index as usize] = object;
        }
        true
    }
}

#[no_mangle]
pub extern "C" fn stack_empty() -> i32 {
    let stack = unsafe { STACK.as_ref().unwrap() };
    stack.empty() as i32
}

#[no_mangle]
pub extern "C" fn stack_top() -> *mut std::ffi::c_void {
    let stack = unsafe { STACK.as_ref().unwrap() };
    stack.top().map_or(std::ptr::null_mut(), |p| p.as_ptr())
}

#[no_mangle]
pub extern "C" fn stack_pop() -> *mut std::ffi::c_void {
    let stack = unsafe { STACK.as_mut().unwrap() };
    stack.pop().map_or(std::ptr::null_mut(), |p| p.as_ptr())
}

#[no_mangle]
pub extern "C" fn stack_push(object: *mut std::ffi::c_void) -> i32 {
    let stack = unsafe { STACK.as_mut().unwrap() };
    if let Some(non_null) = NonNull::new(object) {
        stack.push(non_null) as i32
    } else {
        0
    }
}

static mut STACK: Option<Stack> = None;

#[no_mangle]
pub extern "C" fn stack_init() {
    unsafe {
        STACK = Some(Stack::new());
    }
}

#[no_mangle]
pub extern "C" fn stack_cleanup() {
    unsafe {
        STACK = None;
    }
}