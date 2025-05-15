use std::num::Wrapping;

const _ISalnum: u32 = 8;
const _ISpunct: u32 = 4;
const _IScntrl: u32 = 2;
const _ISblank: u32 = 1;
const _ISgraph: u32 = 32768;
const _ISprint: u32 = 16384;
const _ISspace: u32 = 8192;
const _ISxdigit: u32 = 4096;
const _ISdigit: u32 = 2048;
const _ISalpha: u32 = 1024;
const _ISlower: u32 = 512;
const _ISupper: u32 = 256;

fn is_ctype(c: u8, mask: u32) -> bool {
    unsafe {
        let table = *__ctype_b_loc();
        (table.offset(c as isize) as *const u16).read() as u32 & mask) != 0
    }
}

fn to_upper(c: u8) -> u8 {
    if c.is_ascii_lowercase() {
        c.to_ascii_uppercase()
    } else {
        c
    }
}

pub fn rpl_strtoll(nptr: &str, endptr: Option<&mut *const u8>, base: i32) -> i64 {
    if base < 0 || base == 1 || base > 36 {
        std::ptr::write_volatile(unsafe { __errno_location() }, 22);
        return 0;
    }

    let mut s = nptr.as_ptr();
    let mut negative = false;
    let mut overflow = false;
    let mut i: Wrapping<u64> = Wrapping(0);

    // Skip whitespace
    while unsafe { is_ctype(*s, _ISspace) } {
        s = unsafe { s.offset(1) };
    }

    // Handle optional sign
    if unsafe { *s } == b'-' {
        negative = true;
        s = unsafe { s.offset(1) };
    } else if unsafe { *s } == b'+' {
        s = unsafe { s.offset(1) };
    }

    let save = s;
    let mut base = base;

    // Detect base
    if unsafe { *s } == b'0' {
        let next = unsafe { s.offset(1) };
        if (base == 0 || base == 16) && unsafe { to_upper(*next) } == b'X' {
            s = unsafe { s.offset(2) };
            base = 16;
        } else if (base == 0 || base == 2) && unsafe { to_upper(*next) } == b'B' {
            s = unsafe { s.offset(2) };
            base = 2;
        } else if base == 0 {
            base = 8;
        }
    } else if base == 0 {
        base = 10;
    }

    let base = base as u64;
    let cutoff = (i64::MAX as u64 * 2 + 1) / base;
    let cutlim = (i64::MAX as u64 * 2 + 1) % base;

    let mut c = unsafe { *s };
    while c != 0 {
        let digit = if c.is_ascii_digit() {
            (c - b'0') as u64
        } else if is_ctype(c, _ISalpha) {
            (to_upper(c) - b'A' + 10) as u64
        } else {
            break;
        };

        if digit >= base {
            break;
        }

        if i.0 > cutoff || (i.0 == cutoff && digit > cutlim) {
            overflow = true;
        } else {
            i = i * Wrapping(base) + Wrapping(digit);
        }

        s = unsafe { s.offset(1) };
        c = unsafe { *s };
    }

    if s != save {
        if let Some(endptr) = endptr {
            *endptr = s;
        }

        if !overflow && i.0 > (if negative { i64::MIN as u64 } else { i64::MAX as u64 }) {
            overflow = true;
        }

        if overflow {
            std::ptr::write_volatile(unsafe { __errno_location() }, 34);
            return if negative { i64::MIN } else { i64::MAX };
        }

        return if negative { -(i.0 as i64) } else { i.0 as i64 };
    }

    if let Some(endptr) = endptr {
        let offset = unsafe { s.offset_from(nptr.as_ptr()) };
        if offset >= 2 {
            let prev = unsafe { *s.offset(-1) };
            let prev_upper = to_upper(prev);
            if (prev_upper == b'X' || prev_upper == b'B') && unsafe { *s.offset(-2) } == b'0' {
                *endptr = unsafe { s.offset(-1) };
            } else {
                *endptr = nptr.as_ptr();
            }
        } else {
            *endptr = nptr.as_ptr();
        }
    }

    0
}

// These are still needed for the C interop but isolated to this module
extern "C" {
    fn __ctype_b_loc() -> *mut *const u16;
    fn __ctype_toupper_loc() -> *mut *const i32;
    fn __errno_location() -> *mut i32;
}