use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ZError {
    ZeroPowZero,
    DivisionByZero,
}

impl fmt::Display for ZError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ZError::ZeroPowZero => write!(f, "0^0 is undefined"),
            ZError::DivisionByZero => write!(f, "division by zero"),
        }
    }
}

impl Error for ZError {}

pub struct Z {
    signum: i32,
    chars: Vec<u8>,
}

pub fn zmodpow(
    a: &mut Z,
    b: &Z,
    c: &Z,
    d: &Z,
    tb: &mut Z,
    tc: &mut Z,
    td: &mut Z,
) -> Result<(), Box<dyn Error>> {
    /* TODO use zmodpowu when possible */

    if c.signum <= 0 {
        if c.chars.is_empty() || c.chars.iter().all(|&x| x == 0) {
            if b.chars.is_empty() || b.chars.iter().all(|&x| x == 0) {
                return Err(Box::new(ZError::ZeroPowZero));
            } else if d.chars.is_empty() || d.chars.iter().all(|&x| x == 0) {
                return Err(Box::new(ZError::DivisionByZero));
            }
            a.signum = 1;
            a.chars = vec![1];
        } else if (b.chars.len() == 1 && b.chars[0] == 1) 
               || (d.chars.len() == 1 && d.chars[0] == 1) {
            return Err(Box::new(ZError::DivisionByZero));
        } else {
            a.signum = 0;
            a.chars.clear();
        }
        return Ok(());
    } else if d.chars.is_empty() || d.chars.iter().all(|&x| x == 0) {
        return Err(Box::new(ZError::DivisionByZero));
    } else if b.chars.is_empty() || b.chars.iter().all(|&x| x == 0) {
        a.signum = 0;
        a.chars.clear();
        return Ok(());
    }

    let bits = zbits(c);
    let n = bits / 8;

    zmod(tb, b, d)?;
    tc.chars = c.chars.clone();
    tc.signum = c.signum;
    td.chars = d.chars.clone();
    td.signum = d.signum;
    a.signum = 1;
    a.chars = vec![1];

    for i in 0..n {
        let mut x = tc.chars[i];
        for _ in 0..8 {
            if x & 1 != 0 {
                zmodmul(a, a, tb, td)?;
            }
            zmodsqr(tb, tb, td)?;
            x >>= 1;
        }
    }

    if n < tc.chars.len() {
        let mut x = tc.chars[n];
        while x != 0 {
            if x & 1 != 0 {
                zmodmul(a, a, tb, td)?;
            }
            zmodsqr(tb, tb, td)?;
            x >>= 1;
        }
    }

    Ok(())
}

fn zbits(z: &Z) -> usize {
    if z.chars.is_empty() {
        return 0;
    }
    let mut bits = z.chars.len() * 8;
    let mut top_char = z.chars[z.chars.len() - 1];
    while top_char != 0 {
        bits -= 1;
        top_char >>= 1;
    }
    bits
}

fn zmod(dest: &mut Z, src: &Z, modulus: &Z) -> Result<(), Box<dyn Error>> {
    // Implementation of zmod
    unimplemented!()
}

fn zmodmul(dest: &mut Z, a: &Z, b: &Z, modulus: &Z) -> Result<(), Box<dyn Error>> {
    // Implementation of zmodmul
    unimplemented!()
}

fn zmodsqr(dest: &mut Z, src: &Z, modulus: &Z) -> Result<(), Box<dyn Error>> {
    // Implementation of zmodsqr
    unimplemented!()
}

fn zset(dest: &mut Z, src: &Z) {
    dest.signum = src.signum;
    dest.chars = src.chars.clone();
}

fn zsetu(dest: &mut Z, value: u8) {
    if value == 0 {
        dest.signum = 0;
        dest.chars.clear();
    } else {
        dest.signum = 1;
        dest.chars = vec![value];
    }
}

fn zzero(z: &Z) -> bool {
    z.chars.is_empty() || z.chars.iter().all(|&x| x == 0)
}

fn zzero1(z1: &Z, z2: &Z) -> bool {
    (z1.chars.len() == 1 && z1.chars[0] == 1) || (z2.chars.len() == 1 && z2.chars[0] == 1)
}

fn zsignum(z: &Z) -> i32 {
    z.signum
}