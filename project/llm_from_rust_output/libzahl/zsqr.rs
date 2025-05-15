use std::mem;
use std::ptr;
use std::cmp;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Zahl {
    pub sign: i32,
    pub padding__: i32,
    pub used: usize,
    pub alloced: usize,
    pub chars: *mut u64,
}

pub type ZT = [Zahl; 1];

pub struct LibZahlContext {
    temp_stack_head: *mut *mut Zahl,
    temp_stack: *mut *mut Zahl,
    temp_stack_end: *mut *mut Zahl,
    temp_allocation: *mut std::ffi::c_void,
    error: i32,
    jmp_buf: [std::ffi::c_long; 8],
}

impl LibZahlContext {
    pub fn new() -> Self {
        Self {
            temp_stack_head: ptr::null_mut(),
            temp_stack: ptr::null_mut(),
            temp_stack_end: ptr::null_mut(),
            temp_allocation: ptr::null_mut(),
            error: 0,
            jmp_buf: [0; 8],
        }
    }

    pub fn zinit(&mut self, a: &mut Zahl) {
        a.alloced = 0;
        a.chars = ptr::null_mut();
    }

    pub fn zzero(&self, a: &Zahl) -> bool {
        a.sign == 0
    }

    pub fn zfree_temp(&mut self, a: &mut Zahl) {
        unsafe {
            zfree(a);
            if !self.temp_stack_head.is_null() {
                self.temp_stack_head = self.temp_stack_head.offset(-1);
            }
        }
    }

    pub fn zinit_temp(&mut self, a: &mut Zahl) {
        self.zinit(a);
        unsafe {
            if self.temp_stack_head == self.temp_stack_end {
                let n = self.temp_stack_end.offset_from(self.temp_stack) as usize;
                let old = self.temp_stack as *mut std::ffi::c_void;
                self.temp_stack = libc::realloc(
                    old,
                    2 * n * mem::size_of::<*mut Zahl>(),
                ) as *mut *mut Zahl;
                if self.temp_stack.is_null() {
                    self.temp_stack = old as *mut *mut Zahl;
                    self.libzahl_memfailure();
                }
                self.temp_stack_head = self.temp_stack.offset(n as isize);
                self.temp_stack_end = self.temp_stack_head.offset(n as isize);
            }
            *self.temp_stack_head = a;
            self.temp_stack_head = self.temp_stack_head.offset(1);
        }
    }

    fn libzahl_memfailure(&mut self) {
        unsafe {
            if *libc::__errno_location() == 0 {
                *libc::__errno_location() = 2;
            }
            self.libzahl_failure(*libc::__errno_location());
        }
    }

    fn libzahl_failure(&mut self, error: i32) {
        self.error = error;
        unsafe {
            if !self.temp_stack.is_null() {
                while self.temp_stack_head != self.temp_stack {
                    self.temp_stack_head = self.temp_stack_head.offset(-1);
                    zfree(*self.temp_stack_head);
                }
            }
            libc::free(self.temp_allocation);
            self.temp_allocation = ptr::null_mut();
            libc::longjmp(self.jmp_buf.as_mut_ptr() as *mut libc::c_void, 1);
        }
    }

    pub fn zsplit_unsigned_fast_large_taint(
        &mut self,
        high: &mut Zahl,
        low: &mut Zahl,
        a: &mut Zahl,
        n: usize,
    ) {
        let n = n >> 6;
        high.sign = 1;
        high.used = a.used - n;
        high.chars = unsafe { a.chars.offset(n as isize) };
        low.sign = 1;
        low.used = n;
        low.chars = a.chars;
        while low.used != 0 && unsafe { *low.chars.offset((low.used - 1) as isize) } == 0 {
            low.used -= 1;
        }
        if low.used == 0 {
            low.sign = 0;
        }
    }

    pub fn zsplit_unsigned_fast_small_auto(
        &mut self,
        high: &mut Zahl,
        low: &mut Zahl,
        a: &mut Zahl,
        n: usize,
    ) {
        let mut mask: u64 = 1;
        mask = (mask << n) - 1;
        high.sign = 1;
        high.used = 1;
        unsafe {
            *high.chars.offset(0) = *a.chars.offset(0) >> n;
            if a.used == 2 {
                *high.chars.offset(1) = *a.chars.offset(1) >> n;
                high.used += (*high.chars.offset(1) != 0) as usize;
                let n = 64 - n;
                *high.chars.offset(0) |= (*a.chars.offset(1) & mask) << n;
            }
            low.sign = 1;
            low.used = 1;
            *low.chars.offset(0) = *a.chars.offset(0) & mask;
            if *low.chars.offset(0) == 0 {
                low.sign = 0;
            }
        }
    }

    pub fn zbits(&mut self, a: &mut Zahl) -> usize {
        let mut rc = 0;
        if self.zzero(a) {
            return 1;
        }
        unsafe {
            while *a.chars.offset((a.used - 1) as isize) == 0 {
                a.used -= 1;
            }
            rc = a.used * 8 * mem::size_of::<u64>();
            rc -= (*a.chars.offset((a.used - 1) as isize) as u64).leading_zeros() as usize;
        }
        rc
    }

    pub fn zsqr_ll_single_char(&mut self, a: &mut Zahl, b: &mut Zahl) {
        if a.alloced < 1 {
            unsafe {
                libzahl_realloc(a, 1);
            }
        }
        a.used = 1;
        unsafe {
            *a.chars.offset(0) = (*b.chars.offset(0)).wrapping_mul(*b.chars.offset(0));
        }
        a.sign = 1;
    }

    pub fn zsqr_ll(&mut self, a: &mut Zahl, b: &mut Zahl) {
        let mut z0 = Zahl {
            sign: 0,
            padding__: 0,
            used: 0,
            alloced: 0,
            chars: ptr::null_mut(),
        };
        let mut z1 = Zahl {
            sign: 0,
            padding__: 0,
            used: 0,
            alloced: 0,
            chars: ptr::null_mut(),
        };
        let mut high = Zahl {
            sign: 0,
            padding__: 0,
            used: 0,
            alloced: 0,
            chars: ptr::null_mut(),
        };
        let mut low = Zahl {
            sign: 0,
            padding__: 0,
            used: 0,
            alloced: 0,
            chars: ptr::null_mut(),
        };
        let mut auxchars: [u64; 12] = [0; 12];
        let mut bits = self.zbits(b);
        if bits <= (64 / 2) as usize {
            self.zsqr_ll_single_char(a, b);
            return;
        }
        bits >>= 1;
        if bits < 64 {
            low.chars = auxchars.as_mut_ptr();
            high.chars = unsafe { auxchars.as_mut_ptr().offset(4) };
            self.zsplit_unsigned_fast_small_auto(&mut high, &mut low, b, bits);
        } else {
            bits = bits & !(64 - 1) as usize;
            self.zsplit_unsigned_fast_large_taint(&mut high, &mut low, b, bits);
        }
        if self.zzero(&low) {
            self.zsqr_ll(a, &mut high);
            unsafe {
                zlsh(a, a, bits << 1);
            }
        } else {
            self.zinit_temp(&mut z0);
            self.zinit_temp(&mut z1);
            self.zsqr_ll(&mut z0, &mut low);
            unsafe {
                zmul_ll(&mut z1, &mut low, &mut high);
                zlsh(&mut z1, &mut z1, bits + 1);
                self.zsqr_ll(a, &mut high);
                zlsh(a, a, bits << 1);
                zadd_unsigned_assign(a, &mut z1);
                zadd_unsigned_assign(a, &mut z0);
                self.zfree_temp(&mut z1);
                self.zfree_temp(&mut z0);
            }
        }
    }
}

// External C functions
extern "C" {
    fn zfree(a: *mut Zahl);
    fn zadd_unsigned_assign(a: *mut Zahl, b: *mut Zahl);
    fn zlsh(a: *mut Zahl, b: *mut Zahl, n: usize);
    fn libzahl_realloc(a: *mut Zahl, n: usize);
    fn zmul_ll(a: *mut Zahl, b: *mut Zahl, c: *mut Zahl);
}