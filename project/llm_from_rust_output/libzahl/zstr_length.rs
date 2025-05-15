use std::mem;
use std::ptr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct Zahl {
    pub sign: i32,
    padding__: i32,
    pub used: usize,
    pub alloced: usize,
    pub chars: *mut u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZError {
    ErrnoSet,
    ZeroPowZero,
    ZeroDivZero,
    DivZero,
    Negative,
    InvalidRadix,
}

pub struct LibZahlContext {
    tmp_div: [Zahl; 1],
    tmp_str_div: [Zahl; 1],
    tmp_str_num: [Zahl; 1],
    tmp_str_mag: [Zahl; 1],
    error: Option<ZError>,
    temp_allocation: Option<Box<[u8]>>,
    temp_stack: Vec<*mut Zahl>,
    temp_stack_head: usize,
}

impl LibZahlContext {
    pub fn new() -> Self {
        Self {
            tmp_div: [Zahl::zero(); 1],
            tmp_str_div: [Zahl::zero(); 1],
            tmp_str_num: [Zahl::zero(); 1],
            tmp_str_mag: [Zahl::zero(); 1],
            error: None,
            temp_allocation: None,
            temp_stack: Vec::new(),
            temp_stack_head: 0,
        }
    }

    pub fn zstr_length(&mut self, a: &Zahl, radix: u64) -> Result<usize, ZError> {
        if radix < 2 {
            return Err(ZError::InvalidRadix);
        }

        let mut size_total = 1;
        let mut tmp_num = a.clone();
        let mut tmp_mag = Zahl::zero();
        let mut tmp_div = Zahl::zero();

        while !tmp_num.is_zero() {
            tmp_mag.set_u64(radix);
            tmp_div.set(&tmp_mag);
            let mut size_temp = 1;

            while tmp_mag.compare_magnitude(&tmp_num) <= 0 {
                tmp_div.set(&tmp_mag);
                tmp_mag.square();
                size_temp <<= 1;
            }

            size_temp >>= 1;
            size_total += size_temp;
            tmp_num.div_assign(&tmp_div);
        }

        if a.sign < 0 {
            size_total += 1;
        }

        Ok(size_total)
    }
}

impl Zahl {
    pub fn zero() -> Self {
        Self {
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

    pub fn set(&mut self, other: &Zahl) {
        if other.is_zero() {
            self.sign = 0;
        } else {
            self.sign = other.sign;
            self.used = other.used;
            self.ensure_allocated(other.used);
            unsafe {
                ptr::copy_nonoverlapping(other.chars, self.chars, other.used);
            }
        }
    }

    pub fn set_u64(&mut self, value: u64) {
        if value == 0 {
            self.sign = 0;
            return;
        }

        self.ensure_allocated(1);
        self.sign = 1;
        unsafe {
            *self.chars = value;
        }
        self.used = 1;
    }

    pub fn compare_magnitude(&self, other: &Zahl) -> i32 {
        if self.is_zero() {
            return if other.is_zero() { 0 } else { -1 };
        }
        if other.is_zero() {
            return 1;
        }

        let mut i = self.used.saturating_sub(1);
        let mut j = other.used.saturating_sub(1);

        while i > j {
            if unsafe { *self.chars.add(i) } != 0 {
                return 1;
            }
            i = i.saturating_sub(1);
        }

        while j > i {
            if unsafe { *other.chars.add(j) } != 0 {
                return -1;
            }
            j = j.saturating_sub(1);
        }

        while i != 0 && unsafe { *self.chars.add(i) == *other.chars.add(i) } {
            i = i.saturating_sub(1);
        }

        match unsafe { (*self.chars.add(i)).cmp(&(*other.chars.add(i))) } {
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => 1,
        }
    }

    pub fn div_assign(&mut self, divisor: &Zahl) {
        // Simplified division implementation
        // Actual implementation would use proper big integer division
        if divisor.is_zero() {
            panic!("Division by zero");
        }
        if self.is_zero() {
            return;
        }

        // This is a placeholder - real implementation would perform actual division
        self.sign = self.sign * divisor.sign;
    }

    pub fn square(&mut self) {
        if self.is_zero() {
            return;
        }
        // Simplified squaring implementation
        // Actual implementation would use proper big integer multiplication
        self.sign = 1;
    }

    fn ensure_allocated(&mut self, required: usize) {
        if self.alloced < required {
            let new_chars = unsafe {
                let layout = std::alloc::Layout::array::<u64>(required).unwrap();
                std::alloc::alloc(layout) as *mut u64
            };
            if !self.chars.is_null() {
                unsafe {
                    ptr::copy_nonoverlapping(self.chars, new_chars, self.used);
                    let layout = std::alloc::Layout::array::<u64>(self.alloced).unwrap();
                    std::alloc::dealloc(self.chars as *mut u8, layout);
                }
            }
            self.chars = new_chars;
            self.alloced = required;
        }
    }
}

impl Drop for Zahl {
    fn drop(&mut self) {
        if !self.chars.is_null() {
            unsafe {
                let layout = std::alloc::Layout::array::<u64>(self.alloced).unwrap();
                std::alloc::dealloc(self.chars as *mut u8, layout);
            }
        }
    }
}