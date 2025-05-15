use std::alloc::{self, Layout};
use std::process;
use std::ptr;

pub type size_t = usize;

pub struct NettleReallocContext;

pub trait Realloc {
    fn realloc(&mut self, ptr: *mut u8, old_size: usize, new_size: usize) -> *mut u8;
}

impl Realloc for NettleReallocContext {
    fn realloc(&mut self, ptr: *mut u8, _old_size: usize, new_size: usize) -> *mut u8 {
        if new_size == 0 {
            if !ptr.is_null() {
                unsafe {
                    alloc::dealloc(ptr, Layout::from_size_align(1, 1).unwrap());
                }
            }
            return ptr::null_mut();
        }

        let layout = Layout::from_size_align(new_size, 1).unwrap();
        unsafe {
            if ptr.is_null() {
                alloc::alloc(layout)
            } else {
                alloc::realloc(ptr, Layout::from_size_align(1, 1).unwrap(), new_size)
            }
        }
    }
}

pub fn nettle_xrealloc(
    ctx: &mut dyn Realloc,
    ptr: *mut u8,
    length: usize,
) -> *mut u8 {
    if length == 0 {
        ctx.realloc(ptr, 0, 0)
    } else {
        let new_ptr = ctx.realloc(ptr, 0, length);
        if new_ptr.is_null() {
            eprintln!("Virtual memory exhausted.");
            process::abort();
        }
        new_ptr
    }
}