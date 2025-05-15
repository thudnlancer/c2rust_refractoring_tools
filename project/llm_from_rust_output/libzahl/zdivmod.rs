use std::mem;
use std::ptr;

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

#[derive(Debug, PartialEq, Eq)]
pub enum ZError {
    ErrnoSet,
    ZeroPowZero,
    ZeroDivZero,
    DivZero,
    Negative,
    InvalidRadix,
}

impl Zahl {
    pub fn zero() -> Self {
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

    pub fn set(&mut self, other: &Zahl) -> Result<(), ZError> {
        if other.is_zero() {
            self.sign = 0;
            Ok(())
        } else {
            self.sign = other.sign;
            self.used = other.used;
            // TODO: Handle allocation properly
            unsafe {
                ptr::copy_nonoverlapping(other.chars, self.chars, other.used);
            }
            Ok(())
        }
    }

    pub fn set_u64(&mut self, val: u64) -> Result<(), ZError> {
        if val == 0 {
            self.sign = 0;
            return Ok(());
        }
        // TODO: Handle allocation properly
        self.sign = 1;
        unsafe {
            *self.chars = val;
        }
        self.used = 1;
        Ok(())
    }

    pub fn set_i64(&mut self, val: i64) -> Result<(), ZError> {
        if val >= 0 {
            self.set_u64(val as u64)
        } else {
            // TODO: Handle allocation properly
            self.sign = -1;
            unsafe {
                *self.chars = (-val) as u64;
            }
            self.used = 1;
            Ok(())
        }
    }

    pub fn cmp_mag(&self, other: &Zahl) -> i32 {
        if self.is_zero() {
            return -(other.is_zero() as i32);
        }
        if other.is_zero() {
            return 1;
        }

        let mut i = self.used - 1;
        let mut j = other.used - 1;

        while i > j {
            unsafe {
                if *self.chars.add(i) != 0 {
                    return 1;
                }
            }
            i -= 1;
        }

        while j > i {
            unsafe {
                if *other.chars.add(j) != 0 {
                    return -1;
                }
            }
            j -= 1;
        }

        while i != 0 && unsafe { *self.chars.add(i) == *other.chars.add(i) } {
            i -= 1;
        }

        unsafe {
            if *self.chars.add(i) < *other.chars.add(i) {
                -1
            } else {
                (*self.chars.add(i) > *other.chars.add(i)) as i32
            }
        }
    }

    pub fn signum(&self) -> i32 {
        self.sign
    }

    pub fn abs(&mut self) {
        self.sign &= 1;
    }

    pub fn bits(&self) -> usize {
        if self.is_zero() {
            return 1;
        }

        let mut used = self.used;
        while used > 0 && unsafe { *self.chars.add(used - 1) } == 0 {
            used -= 1;
        }

        if used == 0 {
            return 1;
        }

        let last_char = unsafe { *self.chars.add(used - 1) };
        used * 64 - last_char.leading_zeros() as usize
    }
}

pub fn zdivmod(
    a: &mut Zahl,
    b: &mut Zahl,
    c: &Zahl,
    d: &Zahl,
) -> Result<(), ZError> {
    let c_sign = c.signum();
    let sign = c_sign * d.signum();

    if sign == 0 {
        if !c.is_zero() {
            return Err(ZError::DivZero);
        } else if d.is_zero() {
            return Err(ZError::ZeroDivZero);
        } else {
            a.sign = 0;
            b.sign = 0;
            return Ok(());
        }
    }

    let cmpmag = c.cmp_mag(d);
    if cmpmag <= 0 {
        if cmpmag == 0 {
            a.set_i64(sign as i64)?;
            b.sign = 0;
        } else {
            b.set(c)?;
            a.sign = 0;
        }
        return Ok(());
    }

    // TODO: Implement zdivmod_impl safely
    unimplemented!("zdivmod_impl needs safe implementation");
}

// TODO: Implement remaining functions safely