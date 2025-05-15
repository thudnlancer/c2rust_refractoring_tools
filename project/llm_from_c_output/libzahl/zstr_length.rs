use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ZError {
    InvalidRadix,
}

impl fmt::Display for ZError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ZError::InvalidRadix => write!(f, "invalid radix"),
        }
    }
}

impl Error for ZError {}

pub struct Z {
    // Assuming this is a big integer type in Rust
    value: i32, // Placeholder - replace with actual big integer implementation
}

impl Z {
    pub fn new(value: i32) -> Self {
        Z { value }
    }

    pub fn is_zero(&self) -> bool {
        self.value == 0
    }

    pub fn signum(&self) -> i32 {
        self.value.signum()
    }

    pub fn set(&mut self, other: &Z) {
        self.value = other.value;
    }

    pub fn set_u64(&mut self, value: u64) {
        self.value = value as i32;
    }

    pub fn cmp_mag(&self, other: &Z) -> i32 {
        self.value.abs().cmp(&other.value.abs()) as i32
    }

    pub fn sqr(&mut self, other: &Z) {
        self.value = other.value * other.value;
    }

    pub fn div(&mut self, dividend: &Z, divisor: &Z) {
        self.value = dividend.value / divisor.value;
    }
}

pub fn zstr_length(a: &Z, radix: u64) -> Result<usize, ZError> {
    if radix < 2 {
        return Err(ZError::InvalidRadix);
    }

    let mut num = Z::new(0);
    num.set(a);
    let mut size_total = 1;

    while !num.is_zero() {
        let mut mag = Z::new(0);
        mag.set_u64(radix);
        let mut div = Z::new(0);
        div.set(&mag);
        
        let mut size_temp = 1;
        
        while num.cmp_mag(&mag) <= 0 {
            div.set(&mag);
            mag.sqr(&mag);
            size_temp <<= 1;
        }
        
        size_temp >>= 1;
        size_total += size_temp;
        num.div(&num, &div);
    }

    Ok(size_total + (a.signum() < 0) as usize)
}