use std::mem;
use std::ptr;
use std::cmp;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Zahl {
    pub sign: i32,
    padding__: i32,
    pub used: usize,
    pub alloced: usize,
    pub chars: *mut u64,
}

pub type ZT = [Zahl; 1];

impl Zahl {
    pub fn new() -> Self {
        Zahl {
            sign: 0,
            padding__: 0,
            used: 0,
            alloced: 0,
            chars: ptr::null_mut(),
        }
    }

    pub fn is_zero(&self) -> bool {
        self.sign == 0
    }

    pub fn bits(&mut self) -> usize {
        if self.is_zero() {
            return 1;
        }

        while unsafe { *self.chars.add(self.used - 1) } == 0 {
            self.used -= 1;
        }

        let bits_per_char = mem::size_of::<u64>() * 8;
        let total_bits = self.used * bits_per_char;
        let leading_zeros = unsafe { self.chars.add(self.used - 1) }.leading_zeros() as usize;
        total_bits - leading_zeros
    }
}

pub struct ZContext {
    temp_stack: Vec<*mut Zahl>,
    temp_allocation: Option<Box<[u8]>>,
    error: i32,
}

impl ZContext {
    pub fn new() -> Self {
        ZContext {
            temp_stack: Vec::new(),
            temp_allocation: None,
            error: 0,
        }
    }

    pub fn init_temp(&mut self, z: &mut Zahl) {
        *z = Zahl::new();
        self.temp_stack.push(z);
    }

    pub fn free_temp(&mut self, z: *mut Zahl) {
        unsafe {
            if !z.is_null() {
                Box::from_raw((*z).chars);
            }
        }
        self.temp_stack.pop();
    }

    fn failure(&mut self, error: i32) -> ! {
        self.error = error;
        while let Some(z) = self.temp_stack.pop() {
            unsafe {
                Box::from_raw((*z).chars);
            Box::from_raw(z);
            }
        }
        self.temp_allocation = None;
        panic!("Zahl operation failed");
    }
}

pub fn zmul_ll_single_char(a: &mut Zahl, b: &Zahl, c: &Zahl) {
    if a.alloced < 1 {
        a.chars = Box::into_raw(Box::new([0u64; 1])) as *mut u64;
        a.alloced = 1;
    }
    a.used = 1;
    unsafe {
        *a.chars = (*b.chars) * (*c.chars);
    }
    a.sign = 1;
}

pub fn zsplit(high: &mut Zahl, low: &mut Zahl, a: &mut Zahl, delim: usize) {
    if ptr::eq(high, a) {
        ztrunc(low, a, delim);
        zrsh(high, a, delim);
    } else {
        zrsh(high, a, delim);
        ztrunc(low, a, delim);
    }
}

pub fn zsplit_pz(high: &mut Zahl, low: &mut Zahl, a: &mut Zahl, delim: usize) {
    if a.is_zero() {
        high.sign = 0;
        low.sign = 0;
    } else {
        zsplit(high, low, a, delim);
    }
}

pub fn zmul_ll(a: &mut Zahl, b: &mut Zahl, c: &mut Zahl, ctx: &mut ZContext) {
    if b.is_zero() || c.is_zero() {
        a.sign = 0;
        return;
    }

    let m = b.bits();
    let m2 = if ptr::eq(b, c) { m } else { c.bits() };

    if m + m2 <= 64 {
        zmul_ll_single_char(a, b, c);
        return;
    }

    let m = cmp::max(m, m2);
    let m2 = m >> 1;

    let mut b_high = Box::new(Zahl::new());
    let mut b_low = Box::new(Zahl::new());
    let mut c_high = Box::new(Zahl::new());
    let mut c_low = Box::new(Zahl::new());

    ctx.init_temp(&mut b_high);
    ctx.init_temp(&mut b_low);
    ctx.init_temp(&mut c_high);
    ctx.init_temp(&mut c_low);

    zsplit_pz(&mut b_high, &mut b_low, b, m2);
    zsplit_pz(&mut c_high, &mut c_low, c, m2);

    zmul_ll(a, &mut b_low, &mut c_low, ctx);
    zadd_unsigned_assign(&mut b_low, &mut b_high);
    zadd_unsigned_assign(&mut c_low, &mut c_high);
    zmul_ll(&mut b_low, &mut b_low, &mut c_low, ctx);
    zmul_ll(&mut c_low, &mut b_high, &mut c_high, ctx);
    zsub_nonnegative_assign(&mut b_low, a);
    zsub_nonnegative_assign(&mut b_low, &mut c_low);
    zlsh(&mut b_low, &mut b_low, m2);
    let m2 = m2 << 1;
    zlsh(&mut c_low, &mut c_low, m2);
    zadd_unsigned_assign(a, &mut b_low);
    zadd_unsigned_assign(a, &mut c_low);

    ctx.free_temp(&mut c_low);
    ctx.free_temp(&mut c_high);
    ctx.free_temp(&mut b_low);
    ctx.free_temp(&mut b_high);
}

// Helper functions that would need proper Rust implementations
fn zadd_unsigned_assign(a: &mut Zahl, b: &mut Zahl) {
    // Implementation needed
}

fn zsub_nonnegative_assign(a: &mut Zahl, b: &mut Zahl) {
    // Implementation needed
}

fn zlsh(a: &mut Zahl, b: &mut Zahl, bits: usize) {
    // Implementation needed
}

fn zrsh(a: &mut Zahl, b: &mut Zahl, bits: usize) {
    // Implementation needed
}

fn ztrunc(a: &mut Zahl, b: &mut Zahl, bits: usize) {
    // Implementation needed
}