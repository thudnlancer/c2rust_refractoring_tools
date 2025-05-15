use std::alloc::{alloc, realloc, dealloc, Layout};
use std::ptr::NonNull;

pub type size_t = usize;

#[derive(Copy, Clone)]
pub struct Allocator {
    pub allocate: Option<fn(size_t) -> Option<NonNull<u8>>>,
    pub reallocate: Option<fn(NonNull<u8>, size_t) -> Option<NonNull<u8>>>,
    pub free: Option<fn(NonNull<u8>)>,
    pub die: Option<fn(size_t)>,
}

fn stdlib_allocate(size: size_t) -> Option<NonNull<u8>> {
    if size == 0 {
        return None;
    }
    unsafe {
        let layout = Layout::from_size_align_unchecked(size, 1);
        let ptr = alloc(layout);
        NonNull::new(ptr)
    }
}

fn stdlib_reallocate(ptr: NonNull<u8>, new_size: size_t) -> Option<NonNull<u8>> {
    if new_size == 0 {
        return None;
    }
    unsafe {
        let old_layout = Layout::from_size_align_unchecked(1, 1); // Minimal layout
        let new_layout = Layout::from_size_align_unchecked(new_size, 1);
        let new_ptr = realloc(ptr.as_ptr(), old_layout, new_size);
        NonNull::new(new_ptr)
    }
}

fn stdlib_free(ptr: NonNull<u8>) {
    unsafe {
        let layout = Layout::from_size_align_unchecked(1, 1); // Minimal layout
        dealloc(ptr.as_ptr(), layout);
    }
}

pub static STDLIB_ALLOCATOR: Allocator = Allocator {
    allocate: Some(stdlib_allocate),
    reallocate: Some(stdlib_reallocate),
    free: Some(stdlib_free),
    die: None,
};