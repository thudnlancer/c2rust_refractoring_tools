use std::alloc::{self, Layout};
use std::ptr::NonNull;
use std::mem::MaybeUninit;

#[derive(Clone)]
pub struct ScratchBuffer {
    data: Option<NonNull<u8>>,
    length: usize,
    space: MaybeUninit<[u8; 1024]>,
}

impl ScratchBuffer {
    pub fn new() -> Self {
        Self {
            data: None,
            length: 0,
            space: MaybeUninit::uninit(),
        }
    }

    pub fn init(&mut self) {
        self.data = NonNull::new(self.space.as_mut_ptr() as *mut u8);
        self.length = std::mem::size_of_val(&self.space);
    }

    pub fn free(&mut self) {
        if let Some(ptr) = self.data {
            if ptr.as_ptr() as *const u8 != self.space.as_ptr() as *const u8 {
                unsafe {
                    alloc::dealloc(ptr.as_ptr(), Layout::from_size_align(self.length, 1).unwrap());
                }
            }
        }
    }

    pub fn grow(&mut self) -> bool {
        let new_length = match self.length.checked_mul(2) {
            Some(len) => len,
            None => {
                std::ptr::write_volatile(std::ptr::addr_of_mut!(errno()), 12);
                return false;
            }
        };

        self.free();

        let layout = match Layout::from_size_align(new_length, 1) {
            Ok(layout) => layout,
            Err(_) => {
                std::ptr::write_volatile(std::ptr::addr_of_mut!(errno()), 12);
                self.init();
                return false;
            }
        };

        let new_ptr = unsafe { alloc::alloc(layout) };
        if new_ptr.is_null() {
            self.init();
            return false;
        }

        self.data = NonNull::new(new_ptr);
        self.length = new_length;
        true
    }
}

fn errno() -> i32 {
    unsafe { *libc::__errno_location() }
}