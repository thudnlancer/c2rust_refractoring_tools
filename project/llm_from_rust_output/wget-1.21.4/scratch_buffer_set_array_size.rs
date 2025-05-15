use std::alloc::{alloc, dealloc, Layout};
use std::ptr::NonNull;
use std::mem::{size_of, MaybeUninit};

const BUFFER_SIZE: usize = 1024;

struct ScratchBuffer {
    data: Option<NonNull<u8>>,
    length: usize,
    space: MaybeUninit<[u8; BUFFER_SIZE]>,
}

impl ScratchBuffer {
    fn new() -> Self {
        Self {
            data: None,
            length: 0,
            space: MaybeUninit::uninit(),
        }
    }

    fn init(&mut self) {
        self.data = NonNull::new(unsafe { self.space.as_mut_ptr() as *mut u8 });
        self.length = size_of::<[u8; BUFFER_SIZE]>();
    }

    fn free(&mut self) {
        if let Some(ptr) = self.data {
            if ptr.as_ptr() as *const u8 != unsafe { self.space.as_ptr() } as *const u8 {
                unsafe {
                    dealloc(ptr.as_ptr(), Layout::from_size_align(self.length, 1).unwrap());
                }
            }
        }
    }

    fn set_array_size(&mut self, nelem: usize, size: usize) -> Result<(), ()> {
        let new_length = nelem.checked_mul(size).ok_or(())?;

        if (nelem | size) > (size_of::<usize>() * 8 / 2) 
            && nelem != 0 
            && size != new_length / nelem 
        {
            self.free();
            self.init();
            return Err(());
        }

        if new_length <= self.length {
            return Ok(());
        }

        self.free();
        
        let layout = Layout::from_size_align(new_length, 1).map_err(|_| ())?;
        let new_ptr = unsafe { alloc(layout) };
        
        if new_ptr.is_null() {
            self.init();
            return Err(());
        }

        self.data = NonNull::new(new_ptr);
        self.length = new_length;
        Ok(())
    }
}

fn gl_scratch_buffer_set_array_size(buffer: &mut ScratchBuffer, nelem: usize, size: usize) -> bool {
    buffer.set_array_size(nelem, size).is_ok()
}