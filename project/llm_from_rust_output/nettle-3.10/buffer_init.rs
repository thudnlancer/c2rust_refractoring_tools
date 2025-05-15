use std::alloc::{alloc, dealloc, realloc, Layout};
use std::ptr::NonNull;

pub type size_t = usize;
pub type uint8_t = u8;

pub struct NettleBuffer {
    contents: Option<NonNull<u8>>,
    alloc: size_t,
    size: size_t,
}

impl NettleBuffer {
    pub fn new() -> Self {
        NettleBuffer {
            contents: None,
            alloc: 0,
            size: 0,
        }
    }

    fn realloc(&mut self, new_size: size_t) -> bool {
        unsafe {
            let layout = Layout::from_size_align(self.alloc, 1).unwrap();
            let new_layout = Layout::from_size_align(new_size, 1).unwrap();

            let new_ptr = if self.alloc == 0 {
                alloc(new_layout)
            } else {
                let old_ptr = self.contents.unwrap().as_ptr() as *mut u8;
                realloc(old_ptr as *mut _, layout, new_size)
            };

            if new_ptr.is_null() {
                return false;
            }

            if self.alloc != 0 {
                self.contents = Some(NonNull::new(new_ptr as *mut u8).unwrap();
            } else {
                self.contents = NonNull::new(new_ptr as *mut u8);
            }

            self.alloc = new_size;
            true
        }
    }
}

pub fn nettle_buffer_init(buffer: &mut NettleBuffer) {
    *buffer = NettleBuffer::new();
}