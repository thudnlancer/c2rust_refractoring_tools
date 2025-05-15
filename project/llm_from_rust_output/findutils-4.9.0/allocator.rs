use std::alloc::{alloc, realloc, dealloc, Layout};

pub type size_t = usize;

#[derive(Copy, Clone)]
pub struct Allocator {
    pub allocate: Option<fn(size_t) -> *mut u8>,
    pub reallocate: Option<fn(*mut u8, size_t) -> *mut u8>,
    pub free: Option<fn(*mut u8)>,
    pub die: Option<fn(size_t)>,
}

fn stdlib_allocate(size: size_t) -> *mut u8 {
    unsafe {
        if size == 0 {
            return std::ptr::null_mut();
        }
        let layout = Layout::from_size_align_unchecked(size, 1);
        alloc(layout)
    }
}

fn stdlib_reallocate(ptr: *mut u8, size: size_t) -> *mut u8 {
    unsafe {
        if size == 0 {
            if !ptr.is_null() {
                let layout = Layout::from_size_align_unchecked(1, 1);
                dealloc(ptr, layout);
            }
            return std::ptr::null_mut();
        }
        let layout = Layout::from_size_align_unchecked(size, 1);
        realloc(ptr, layout, size)
    }
}

fn stdlib_free(ptr: *mut u8) {
    if !ptr.is_null() {
        unsafe {
            let layout = Layout::from_size_align_unchecked(1, 1);
            dealloc(ptr, layout);
        }
    }
}

pub static STDLIB_ALLOCATOR: Allocator = Allocator {
    allocate: Some(stdlib_allocate),
    reallocate: Some(stdlib_reallocate),
    free: Some(stdlib_free),
    die: None,
};