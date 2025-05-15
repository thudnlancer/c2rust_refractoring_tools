use std::ptr;
use std::ffi::CStr;
use std::os::raw::{c_char, c_int, c_ulong, c_ulonglong};
use std::num::NonZeroUsize;

type size_t = c_ulong;
type uint64_t = c_ulonglong;
type zahl_char_t = uint64_t;

#[derive(Clone, Copy)]
#[repr(C)]
struct Zahl {
    sign: c_int,
    padding__: c_int,
    used: size_t,
    alloced: size_t,
    chars: *mut zahl_char_t,
}

type z_t = [Zahl; 1];

const _ISdigit: u16 = 2048;

extern "C" {
    fn libzahl_realloc(a: *mut Zahl, size: size_t);
    fn zadd(a: *mut Zahl, b: *mut Zahl, c: *mut Zahl);
    fn zmul_ll(a: *mut Zahl, b: *mut Zahl, c: *mut Zahl);
    static mut libzahl_tmp_str_num: z_t;
    static mut libzahl_const_1e19: z_t;
    fn __errno_location() -> *mut c_int;
    fn __ctype_b_loc() -> *mut *const u16;
}

fn libzahl_memcpy(d: *mut zahl_char_t, s: *const zahl_char_t, n: size_t) {
    unsafe {
        if n == 0 {
            return;
        }
        ptr::copy_nonoverlapping(s, d, n as usize);
    }
}

fn zset(a: *mut Zahl, b: *mut Zahl) {
    unsafe {
        if (*b).sign == 0 {
            (*a).sign = 0;
        } else {
            (*a).sign = (*b).sign;
            (*a).used = (*b).used;
            if (*a).alloced < (*b).used {
                libzahl_realloc(a, (*b).used);
            }
            libzahl_memcpy((*a).chars, (*b).chars, (*b).used);
        }
    }
}

fn zsignum(a: *mut Zahl) -> c_int {
    unsafe { (*a).sign }
}

fn zmul(a: *mut Zahl, b: *mut Zahl, c: *mut Zahl) {
    unsafe {
        let b_sign = (*b).sign;
        (*b).sign *= b_sign;
        let c_sign = (*c).sign;
        (*c).sign *= c_sign;
        zmul_ll(a, b, c);
        (*c).sign = c_sign;
        (*b).sign = b_sign;
        (*a).sign = zsignum(b) * zsignum(c);
    }
}

pub fn zsets(a: *mut Zahl, str: *const c_char) -> c_int {
    unsafe {
        let mut temp: c_ulonglong = 0;
        let cstr = CStr::from_ptr(str);
        let bytes = cstr.to_bytes();
        let mut neg = false;
        let mut start = 0;

        if bytes.is_empty() {
            *__errno_location() = 22;
            return -1;
        }

        match bytes[0] {
            b'-' => {
                neg = true;
                start = 1;
            }
            b'+' => {
                start = 1;
            }
            _ => {}
        }

        if start >= bytes.len() {
            *__errno_location() = 22;
            return -1;
        }

        for &c in &bytes[start..] {
            if !c.is_ascii_digit() {
                *__errno_location() = 22;
                return -1;
            }
        }

        (*a).sign = 0;
        zset(libzahl_tmp_str_num.as_mut_ptr(), libzahl_const_1e19.as_mut_ptr());

        let mut pos = start;
        while pos < bytes.len() {
            let chunk_size = (bytes.len() - pos).min(19);
            temp = 0;

            for i in 0..chunk_size {
                temp = temp.wrapping_mul(10);
                temp = temp.wrapping_add((bytes[pos + i] - b'0') as c_ulonglong);
            }

            if temp != 0 {
                *((*libzahl_tmp_str_num.as_mut_ptr()).chars).offset(0) = temp as zahl_char_t;
                zadd(a, a, libzahl_tmp_str_num.as_mut_ptr());
            }

            pos += chunk_size;
            if pos < bytes.len() {
                zmul(a, a, libzahl_const_1e19.as_mut_ptr());
            }
        }

        if neg {
            (*a).sign = -zsignum(a);
        }

        0
    }
}