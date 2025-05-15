use std::mem;
use std::ptr;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __sigset_t {
    pub __val: [u64; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: [i64; 8],
    pub __mask_was_saved: i32,
    pub __saved_mask: __sigset_t,
}

pub type jmp_buf = [__jmp_buf_tag; 1];

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Zahl {
    pub sign: i32,
    pub padding__: i32,
    pub used: usize,
    pub alloced: usize,
    pub chars: *mut u64,
}

pub type z_t = [Zahl; 1];

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ZError {
    ErrnoSet = 0,
    ZeroPowZero = 1,
    ZeroDivZero = 2,
    DivByZero = 3,
    Negative = 4,
    InvalidRadix = 5,
}

pub struct LibZahl {
    tmp_pow_b: z_t,
    jmp_buf: jmp_buf,
    temp_allocation: *mut std::ffi::c_void,
    temp_stack: Vec<*mut Zahl>,
    error: Option<ZError>,
}

impl LibZahl {
    pub fn new() -> Self {
        Self {
            tmp_pow_b: [Zahl {
                sign: 0,
                padding__: 0,
                used: 0,
                alloced: 0,
                chars: ptr::null_mut(),
            }; 1],
            jmp_buf: [__jmp_buf_tag {
                __jmpbuf: [0; 8],
                __mask_was_saved: 0,
                __saved_mask: __sigset_t { __val: [0; 16] },
            }; 1],
            temp_allocation: ptr::null_mut(),
            temp_stack: Vec::new(),
            error: None,
        }
    }

    fn memcpy(d: &mut [u64], s: &[u64]) {
        d.copy_from_slice(s);
    }

    fn zzero(&self, a: &Zahl) -> bool {
        a.sign == 0
    }

    fn zset(&mut self, a: &mut Zahl, b: &Zahl) {
        if b.sign == 0 {
            a.sign = 0;
        } else {
            a.sign = b.sign;
            a.used = b.used;
            if a.alloced < b.used {
                self.libzahl_realloc(a, b.used);
            }
            unsafe {
                Self::memcpy(
                    std::slice::from_raw_parts_mut(a.chars, a.used),
                    std::slice::from_raw_parts(b.chars, b.used),
                );
            }
        }
    }

    fn zsetu(&mut self, a: &mut Zahl, b: u64) {
        if b == 0 {
            a.sign = 0;
            return;
        }
        if a.alloced < 1 {
            self.libzahl_realloc(a, 1);
        }
        a.sign = 1;
        unsafe {
            *a.chars = b;
        }
        a.used = 1;
    }

    fn zsignum(&self, a: &Zahl) -> i32 {
        a.sign
    }

    fn zabs(&mut self, a: &mut Zahl, b: &Zahl) {
        if !ptr::eq(a, b) {
            self.zset(a, b);
        }
        a.sign &= 1;
    }

    fn zneg(&mut self, a: &mut Zahl, b: &Zahl) {
        if !ptr::eq(a, b) {
            self.zset(a, b);
        }
        a.sign = -a.sign;
    }

    fn libzahl_realloc(&mut self, a: &mut Zahl, size: usize) {
        if a.alloced < size {
            let new_chars = unsafe {
                libc::realloc(
                    a.chars as *mut libc::c_void,
                    size * mem::size_of::<u64>(),
                ) as *mut u64
            };
            if new_chars.is_null() {
                self.libzahl_failure(ZError::ErrnoSet);
            }
            a.chars = new_chars;
            a.alloced = size;
        }
    }

    fn libzahl_failure(&mut self, error: ZError) {
        self.error = Some(error);
        while let Some(ptr) = self.temp_stack.pop() {
            unsafe {
                libc::free(ptr as *mut libc::c_void);
            }
        }
        unsafe {
            libc::free(self.temp_allocation);
        }
        self.temp_allocation = ptr::null_mut();
        // longjmp would be unsafe here, in real code we'd use proper error handling
        panic!("libzahl failure: {:?}", error);
    }

    pub fn zpowu(&mut self, a: &mut Zahl, b: &Zahl, c: u64) {
        let mut neg = false;
        if c == 0 {
            if self.zzero(b) {
                self.libzahl_failure(ZError::ZeroPowZero);
            }
            self.zsetu(a, 1);
            return;
        } else if self.zzero(b) {
            a.sign = 0;
            return;
        }

        neg = self.zsignum(b) < 0 && (c & 1) != 0;
        self.zabs(&mut self.tmp_pow_b[0], b);

        if c & 1 != 0 {
            self.zset(a, &self.tmp_pow_b[0]);
        } else {
            self.zsetu(a, 1);
        }

        let mut c = c;
        loop {
            c >>= 1;
            if c == 0 {
                break;
            }
            // zsqr_ll would need to be implemented safely
            // zmul_ll would need to be implemented safely
            if c & 1 != 0 {
                // zmul_ll(a, a, &self.tmp_pow_b[0]);
            }
        }

        if neg {
            self.zneg(a, a);
        }
    }
}