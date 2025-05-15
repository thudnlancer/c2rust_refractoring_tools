use std::alloc::{alloc, dealloc, realloc, Layout};
use std::mem;
use std::ptr;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Zahl {
    pub sign: i32,
    pub padding__: i32,
    pub used: usize,
    pub alloced: usize,
    pub chars: *mut u64,
}

#[derive(Debug)]
pub struct LibZahl {
    pool_n: [usize; 64],
    pool: [Vec<*mut u64>; 64],
    temp_stack: Vec<*mut Zahl>,
    temp_stack_head: usize,
    temp_allocation: Option<ptr::NonNull<u8>>,
    error: i32,
    jmp_buf: [std::ffi::c_long; 8],
}

impl LibZahl {
    pub fn new() -> Self {
        Self {
            pool_n: [0; 64],
            pool: [(); 64].map(|_| Vec::new()),
            temp_stack: Vec::new(),
            temp_stack_head: 0,
            temp_allocation: None,
            error: 0,
            jmp_buf: [0; 8],
        }
    }

    pub fn realloc(&mut self, a: &mut Zahl, need: usize) -> Result<(), i32> {
        let mut i = (8 * mem::size_of::<usize>() - 1).saturating_sub(need.leading_zeros() as usize);
        let mut new_size = 1 << i;

        if new_size != need {
            i += 1;
            new_size <<= 1;
        }

        if self.pool_n[i] > 0 {
            self.pool_n[i] -= 1;
            let new = self.pool[i][self.pool_n[i]];
            unsafe {
                ptr::copy_nonoverlapping(a.chars, new, a.alloced);
            }
            self.zfree(a);
            a.chars = new;
        } else {
            let layout = Layout::array::<u64>(new_size + 4).map_err(|_| 2)?;
            let new_ptr = unsafe {
                realloc(
                    a.chars as *mut u8,
                    layout,
                    (new_size + 4) * mem::size_of::<u64>(),
                )
            };

            if new_ptr.is_null() {
                return Err(2);
            }

            a.chars = new_ptr as *mut u64;
        }

        a.alloced = new_size;
        Ok(())
    }

    pub fn zfree(&mut self, z: &mut Zahl) {
        if !z.chars.is_null() {
            let layout = Layout::array::<u64>(z.alloced + 4).unwrap();
            unsafe {
                dealloc(z.chars as *mut u8, layout);
            }
            z.chars = ptr::null_mut();
        }
    }

    fn memcpy(d: *mut u64, s: *const u64, n: usize) {
        if n == 0 {
            return;
        }

        unsafe {
            ptr::copy_nonoverlapping(s, d, n);
        }
    }

    fn failure(&mut self, error: i32) -> ! {
        self.error = error;
        
        while self.temp_stack_head > 0 {
            self.temp_stack_head -= 1;
            let z = unsafe { &mut *self.temp_stack[self.temp_stack_head] };
            self.zfree(z);
        }

        if let Some(ptr) = self.temp_allocation.take() {
            let layout = Layout::array::<u8>(1).unwrap();
            unsafe {
                dealloc(ptr.as_ptr(), layout);
            }
        }

        panic!("libzahl failure");
    }
}