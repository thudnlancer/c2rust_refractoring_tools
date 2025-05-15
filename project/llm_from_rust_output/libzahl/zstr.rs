use std::cmp::Ordering;
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

pub type Z = [Zahl; 1];

const PARTIALS: &[u8] = b"00010203040506070809101112131415161718192021222324252627282930313233343536373839404142434445464748495051525354555657585960616263646566676869707172737475767778798081828384858687888990919293949596979899";

fn zzero(a: &Zahl) -> bool {
    a.sign == 0
}

fn zsignum(a: &Zahl) -> i32 {
    a.sign
}

fn zabs(a: &mut Zahl, b: &Zahl) {
    if a as *const _ != b as *const _ {
        zset(a, b);
    }
    a.sign &= 1;
}

fn zset(a: &mut Zahl, b: &Zahl) {
    if b.sign == 0 {
        a.sign = 0;
    } else {
        a.sign = b.sign;
        a.used = b.used;
        if a.alloced < b.used {
            libzahl_realloc(a, b.used);
        }
        unsafe {
            ptr::copy_nonoverlapping(b.chars, a.chars, b.used);
        }
    }
}

fn libzahl_realloc(a: &mut Zahl, new_size: usize) {
    unsafe {
        let new_chars = libc::realloc(a.chars as *mut libc::c_void, new_size * mem::size_of::<u64>());
        if new_chars.is_null() {
            libzahl_memfailure();
        }
        a.chars = new_chars as *mut u64;
        a.alloced = new_size;
    }
}

fn libzahl_memfailure() -> ! {
    unsafe {
        if libc::__errno_location().read() == 0 {
            *libc::__errno_location() = 2;
        }
        libzahl_failure(*libc::__errno_location());
    }
}

fn libzahl_failure(error: i32) -> ! {
    unsafe {
        libzahl_error = error;
        if !libzahl_temp_stack.is_null() {
            while libzahl_temp_stack_head != libzahl_temp_stack {
                libzahl_temp_stack_head = libzahl_temp_stack_head.offset(-1);
                zfree(&mut **libzahl_temp_stack_head);
            }
        }
        libc::free(libzahl_temp_allocation);
        libzahl_temp_allocation = ptr::null_mut();
        libc::longjmp(libzahl_jmp_buf.as_mut_ptr(), 1);
    }
}

fn zfree(a: &mut Zahl) {
    unsafe {
        libc::free(a.chars as *mut libc::c_void);
        a.chars = ptr::null_mut();
        a.used = 0;
        a.alloced = 0;
    }
}

fn sprintint_fix(buf: &mut [u8; 20], v: u64) {
    let mut v = v;
    let buffer = unsafe { &mut *(buf.as_mut_ptr().add(1) as *mut [u16; 9]) };

    for i in (0..9).rev() {
        let idx = (v % 100) as usize * 2;
        buffer[i] = u16::from_le_bytes([PARTIALS[idx], PARTIALS[idx + 1]]);
        v /= 100;
    }

    buf[0] = b'0' + v as u8;
    buf[19] = 0;
}

fn sprintint_min(buf: &mut [u8; 20], v: u64) -> usize {
    sprintint_fix(buf, v);
    let mut i = 0;
    while buf[i] == b'0' {
        i += 1;
    }
    let j = 19 - i;
    buf.copy_within(i..19, 0);
    buf[j] = 0;
    j
}

pub fn zstr(a: &mut Zahl, b: Option<&mut [u8]>, n: usize) -> Result<Vec<u8>, ()> {
    let mut buf = [0u8; 20];
    let mut neg = false;
    let mut last = 0;
    let mut tot = 0;
    let mut overridden = 0;

    if zzero(a) {
        return if let Some(b) = b {
            if b.len() < 2 {
                return Err(());
            }
            b[0] = b'0';
            b[1] = 0;
            Ok(b[..2].to_vec())
        } else {
            Ok(b"0\0".to_vec())
        };
    }

    let n = if n == 0 {
        (20 * 64 / 64 + (64 == 8) as usize) * a.used
    } else {
        n
    };

    let mut result = if let Some(b) = b {
        if b.len() < n + 1 {
            return Err(());
        }
        b.to_vec()
    } else {
        vec![0; n + 1]
    };

    neg = zsignum(a) < 0;
    zabs(&mut libzahl_tmp_str_num[0], a);

    if neg {
        result[0] = b'-';
    }

    let mut b_ptr = if neg { 1 } else { 0 };
    let mut remaining = n - if neg { 1 } else { 0 };

    last = remaining;
    remaining = if last > 19 { remaining - 19 } else { 0 };

    loop {
        zdivmod(
            &mut libzahl_tmp_str_num[0],
            &mut libzahl_tmp_str_rem[0],
            &mut libzahl_tmp_str_num[0],
            &mut libzahl_const_1e19[0],
        );

        if !zzero(&libzahl_tmp_str_num[0]) {
            let rem = if zzero(&libzahl_tmp_str_rem[0]) {
                0
            } else {
                unsafe { *libzahl_tmp_str_rem[0].chars }
            };

            sprintint_fix(&mut buf, rem);
            result[b_ptr + remaining..b_ptr + remaining + 19].copy_from_slice(&buf[..19]);
            overridden = result[b_ptr + remaining];
            last = remaining;
            remaining = if last > 19 { remaining - 19 } else { 0 };
            tot += 19;
        } else {
            let rem = unsafe { *libzahl_tmp_str_rem[0].chars };
            let len = sprintint_min(&mut buf, rem);

            if tot != 0 {
                result[..len].copy_from_slice(&buf[..len]);
                result[len..len + tot + 1].copy_from_slice(&result[last..last + tot + 1]);
            } else {
                result[..len + 1].copy_from_slice(&buf[..len + 1]);
            }
            break;
        }
    }

    Ok(result)
}

// Global variables (would need proper Rust-safe wrappers)
static mut libzahl_error: i32 = 0;
static mut libzahl_temp_stack: *mut *mut Zahl = ptr::null_mut();
static mut libzahl_temp_stack_head: *mut *mut Zahl = ptr::null_mut();
static mut libzahl_temp_allocation: *mut libc::c_void = ptr::null_mut();
static mut libzahl_jmp_buf: [libc::jmp_buf; 1] = [libc::jmp_buf { __jmpbuf: [0; 8], __mask_was_saved: 0, __saved_mask: libc::__sigset_t { __val: [0; 16] } }];
static mut libzahl_tmp_str_rem: Z = [Zahl { sign: 0, padding__: 0, used: 0, alloced: 0, chars: ptr::null_mut() }];
static mut libzahl_tmp_str_num: Z = [Zahl { sign: 0, padding__: 0, used: 0, alloced: 0, chars: ptr::null_mut() }];
static mut libzahl_const_1e19: Z = [Zahl { sign: 0, padding__: 0, used: 0, alloced: 0, chars: ptr::null_mut() }];

// External C functions (would need proper Rust bindings)
extern "C" {
    fn zdivmod(a: *mut Zahl, b: *mut Zahl, c: *mut Zahl, d: *mut Zahl);
}