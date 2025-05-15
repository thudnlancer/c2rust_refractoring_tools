use std::ptr;
use std::mem;
use std::alloc::{alloc, dealloc, Layout};
use libzahl::{z_t, zzero, znegative, zabs, zdivmod, BITS_PER_CHAR};

const S1: &str = "0123456789";
const S2: &str = concat!(
    "00010203040506070809",
    "10111213141516171819",
    "20212223242526272829",
    "30313233343536373839",
    "40414243444546474849",
    "50515253545556575859",
    "60616263646566676869",
    "70717273747576777879",
    "80818283848586878889",
    "90919293949596979899"
);

fn sprintint_fix(buf: &mut [u8; 20], v: u64) {
    let partials = S2.as_bytes();
    let buffer = unsafe { &mut *(buf.as_mut_ptr().add(1) as *mut [u16; 9]) };

    let mut v = v;
    for i in (0..9).rev() {
        buffer[i] = u16::from_ne_bytes([partials[(v % 100) as usize * 2], partials[(v % 100) as usize * 2 + 1]]);
        v /= 100;
    }
    buf[0] = b'0' + v as u8;
    buf[19] = 0;
}

fn cmemmove(d: &mut [u8], s: &[u8], n: usize) {
    d[..n].copy_from_slice(&s[..n]);
}

fn sprintint_min(buf: &mut [u8; 20], v: u64) -> usize {
    sprintint_fix(buf, v);
    let mut i = 0;
    while i < 19 && buf[i] == b'0' {
        i += 1;
    }
    let j = 19 - i;
    cmemmove(&mut buf[..j], &buf[i..i+j], j);
    buf[j] = 0;
    j
}

pub fn zstr(a: &z_t, b: Option<&mut [u8]>, n: usize) -> Result<Vec<u8>, &'static str> {
    let mut buf = [0u8; 20];
    let mut neg = false;
    let mut last;
    let mut tot = 0;
    let mut overridden = 0u8;

    if zzero(a) {
        return if b.is_none() {
            let mut v = vec![0u8; 2];
            v[0] = b'0';
            v[1] = 0;
            Ok(v)
        } else {
            let b = b.unwrap();
            if b.len() < 2 {
                return Err("Buffer too small");
            }
            b[0] = b'0';
            b[1] = 0;
            Ok(b[..2].to_vec())
        };
    }

    let mut n = if n == 0 {
        (20 * BITS_PER_CHAR / 64 + if BITS_PER_CHAR == 8 { 1 } else { 0 }) * a.used
    } else {
        n
    };

    let mut result = if b.is_none() {
        let layout = Layout::from_size_align(n + 1, 1).map_err(|_| "Invalid layout")?;
        unsafe { Vec::from_raw_parts(alloc(layout) as *mut u8, n + 1, n + 1) }
    } else {
        let b = b.unwrap();
        if b.len() < n + 1 {
            return Err("Buffer too small");
        }
        b.to_vec()
    };

    neg = znegative(a);
    let mut num = z_t::new();
    zabs(&mut num, a);
    
    if neg {
        result[0] = b'-';
    }
    let mut b = if neg { &mut result[1..] } else { &mut result[..] };
    n -= if neg { 1 } else { 0 };
    last = n;
    n = if n > 19 { n - 19 } else { 0 };

    let mut rem = z_t::new();
    loop {
        zdivmod(&mut num, &mut rem, &num, &libzahl_const_1e19);
        if !zzero(&num) {
            let rem_val = if zzero(&rem) { 0 } else { rem.chars[0] };
            sprintint_fix(&mut buf, rem_val);
            if n + 19 < b.len() {
                b[n..n+19].copy_from_slice(&buf[..19]);
            }
            if n + 19 < b.len() {
                b[n + 19] = overridden;
            }
            overridden = b[n];
            last = n;
            n = if n > 19 { n - 19 } else { 0 };
            tot += 19;
        } else {
            let len = sprintint_min(&mut buf, rem.chars[0]);
            if tot > 0 {
                b[..len].copy_from_slice(&buf[..len]);
                b[len..len+tot].copy_from_slice(&b[last..last+tot]);
            } else {
                b[..len+1].copy_from_slice(&buf[..len+1]);
            }
            break;
        }
    }

    if neg {
        result.insert(0, b'-');
    }
    Ok(result)
}