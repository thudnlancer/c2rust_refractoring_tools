use std::ptr;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Zahl {
    pub sign: i32,
    pub padding__: i32,
    pub used: usize,
    pub alloced: usize,
    pub chars: *mut u64,
}

pub type Z = [Zahl; 1];

static mut LIBZAHL_TMP_MOD: Z = [Zahl {
    sign: 0,
    padding__: 0,
    used: 0,
    alloced: 0,
    chars: ptr::null_mut(),
}; 1];

static mut LIBZAHL_TMP_MODMUL: Z = [Zahl {
    sign: 0,
    padding__: 0,
    used: 0,
    alloced: 0,
    chars: ptr::null_mut(),
}; 1];

fn libzahl_memcpy(d: &mut [u64], s: &[u64]) {
    d.copy_from_slice(s);
}

fn zset(a: &mut Zahl, b: &Zahl) {
    if b.sign == 0 {
        a.sign = 0;
    } else {
        a.sign = b.sign;
        a.used = b.used;
        if a.alloced < b.used {
            // Safe because we're reallocating to a larger size
            unsafe {
                libzahl_realloc(a, b.used);
            }
        }
        unsafe {
            libzahl_memcpy(
                std::slice::from_raw_parts_mut(a.chars, a.used),
                std::slice::from_raw_parts(b.chars, b.used),
            );
        }
    }
}

fn zsignum(a: &Zahl) -> i32 {
    a.sign
}

fn zmul(a: &mut Zahl, b: &mut Zahl, c: &mut Zahl) {
    let b_sign = b.sign;
    b.sign *= b_sign;
    let c_sign = c.sign;
    c.sign *= c_sign;
    
    unsafe {
        zmul_ll(a, b, c);
    }
    
    c.sign = c_sign;
    b.sign = b_sign;
    a.sign = zsignum(b) * zsignum(c);
}

fn zmod(a: &mut Zahl, b: &mut Zahl, c: &mut Zahl) {
    unsafe {
        zdivmod(&mut LIBZAHL_TMP_MOD[0], a, b, c);
    }
}

pub fn zmodmul(a: &mut Zahl, b: &mut Zahl, c: &mut Zahl, d: &mut Zahl) {
    if ptr::eq(a, d) {
        unsafe {
            zset(&mut LIBZAHL_TMP_MODMUL[0], d);
            zmul(a, b, c);
            zmod(a, a, &mut LIBZAHL_TMP_MODMUL[0]);
        }
    } else {
        zmul(a, b, c);
        zmod(a, a, d);
    }
}

// These would need to be implemented safely or marked as unsafe extern
extern "C" {
    fn libzahl_realloc(z: *mut Zahl, size: usize);
    fn zmul_ll(a: *mut Zahl, b: *mut Zahl, c: *mut Zahl);
    fn zdivmod(q: *mut Zahl, r: *mut Zahl, a: *mut Zahl, b: *mut Zahl);
}