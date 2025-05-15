use std::cmp::Ordering;
use std::fmt;
use std::mem;
use std::ops::{Add, Div, Mul, Neg, Sub};
use std::ptr;

const BASE: u32 = 65536;
const SEG_SIZE: usize = 6;

#[derive(Debug, Clone)]
struct MpzSeg {
    d: [u16; SEG_SIZE],
    next: Option<Box<MpzSeg>>,
}

impl MpzSeg {
    fn new() -> Self {
        MpzSeg {
            d: [0; SEG_SIZE],
            next: None,
        }
    }
}

#[derive(Debug, Clone)]
struct Mpz {
    val: i32,
    ptr: Option<Box<MpzSeg>>,
}

impl Mpz {
    fn new() -> Self {
        Mpz {
            val: 0,
            ptr: None,
        }
    }

    fn set_si(&mut self, val: i32) {
        self.clear();
        if val == i32::MIN {
            self.val = -1;
            let mut seg = MpzSeg::new();
            seg.d[1] = 0x8000;
            self.ptr = Some(Box::new(seg));
        } else {
            self.val = val;
        }
    }

    fn clear(&mut self) {
        self.val = 0;
        self.ptr = None;
    }

    fn normalize(&mut self) {
        if self.ptr.is_none() {
            return;
        }

        let mut last_nonzero: Option<&mut Box<MpzSeg>> = None;
        let mut current = self.ptr.as_mut();
        
        while let Some(seg) = current {
            if seg.d.iter().any(|&d| d != 0) {
                last_nonzero = current;
            }
            current = seg.next.as_mut();
        }

        if last_nonzero.is_none() {
            self.set_si(0);
            return;
        }

        let last = last_nonzero.unwrap();
        last.next = None;

        if let Some(seg) = &self.ptr {
            if seg.next.is_none() && seg.d[1] <= 0x7FFF && seg.d[2..].iter().all(|&d| d == 0) {
                let val = seg.d[0] as i32 + ((seg.d[1] as i32) << 16;
                let signed_val = if self.val < 0 { -val } else { val };
                self.set_si(signed_val);
            }
        }
    }

    fn abs(&mut self) {
        if self.val < 0 {
            self.val = -self.val;
        }
    }

    fn neg(&mut self) {
        self.val = -self.val;
    }

    fn sgn(&self) -> i32 {
        match self.val.cmp(&0) {
            Ordering::Greater => 1,
            Ordering::Equal => 0,
            Ordering::Less => -1,
        }
    }
}

#[derive(Debug, Clone)]
struct Mpq {
    p: Mpz,
    q: Mpz,
}

impl Mpq {
    fn new() -> Self {
        Mpq {
            p: Mpz::new(),
            q: Mpz { val: 1, ptr: None },
        }
    }

    fn canonicalize(&mut self) {
        if self.q.val < 0 {
            self.p.neg();
            self.q.neg();
        }

        let mut f = Mpz::new();
        gcd(&mut f, &self.p, &self.q);

        if !(f.val == 1 && f.ptr.is_none()) {
            div(&mut self.p, None, &self.p, &f);
            div(&mut self.q, None, &self.q, &f);
        }
    }

    fn set_si(&mut self, p: i32, q: u32) {
        if q == 0 {
            panic!("mpq_set_si: zero denominator not allowed");
        }
        self.p.set_si(p);
        self.q.set_si(q as i32);
    }

    fn sgn(&self) -> i32 {
        self.p.sgn()
    }

    fn neg(&mut self) {
        self.p.neg();
    }

    fn abs(&mut self) {
        self.p.abs();
    }
}

fn add(z: &mut Mpz, x: &Mpz, y: &Mpz) {
    if x.val == 0 {
        *z = y.clone();
        return;
    }
    if y.val == 0 {
        *z = x.clone();
        return;
    }

    // Implementation of addition for long format numbers
    // ... (remaining addition logic)
}

fn sub(z: &mut Mpz, x: &Mpz, y: &Mpz) {
    if ptr::eq(x, y) {
        z.set_si(0);
    } else {
        let mut y_neg = y.clone();
        y_neg.neg();
        add(z, x, &y_neg);
    }
}

fn mul(z: &mut Mpz, x: &Mpz, y: &Mpz) {
    if x.val == 0 || y.val == 0 {
        z.set_si(0);
        return;
    }

    // Implementation of multiplication for long format numbers
    // ... (remaining multiplication logic)
}

fn div(q: Option<&mut Mpz>, r: Option<&mut Mpz>, x: &Mpz, y: &Mpz) {
    if y.val == 0 {
        panic!("mpz_div: divide by zero not allowed");
    }
    if x.val == 0 {
        if let Some(q) = q {
            q.set_si(0);
        }
        if let Some(r) = r {
            r.set_si(0);
        }
        return;
    }

    // Implementation of division for long format numbers
    // ... (remaining division logic)
}

fn gcd(z: &mut Mpz, x: &Mpz, y: &Mpz) {
    let mut u = x.clone();
    u.abs();
    let mut v = y.clone();
    v.abs();
    let mut r = Mpz::new();

    while v.sgn() != 0 {
        div(None, Some(&mut r), &u, &v);
        u = v.clone();
        v = r.clone();
    }

    *z = u;
}

fn cmp(x: &Mpz, y: &Mpz) -> Ordering {
    if ptr::eq(x, y) {
        return Ordering::Equal;
    }

    // Implementation of comparison
    // ... (remaining comparison logic)
    Ordering::Equal
}

// Implement remaining functions and traits as needed...

impl Add for Mpz {
    type Output = Mpz;

    fn add(self, other: Mpz) -> Mpz {
        let mut result = Mpz::new();
        add(&mut result, &self, &other);
        result
    }
}

impl Sub for Mpz {
    type Output = Mpz;

    fn sub(self, other: Mpz) -> Mpz {
        let mut result = Mpz::new();
        sub(&mut result, &self, &other);
        result
    }
}

impl Mul for Mpz {
    type Output = Mpz;

    fn mul(self, other: Mpz) -> Mpz {
        let mut result = Mpz::new();
        mul(&mut result, &self, &other);
        result
    }
}

impl Neg for Mpz {
    type Output = Mpz;

    fn neg(mut self) -> Mpz {
        self.neg();
        self
    }
}

// Similar implementations for Mpq...

impl fmt::Display for Mpz {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Implementation of string representation
        write!(f, "Mpz")
    }
}

impl fmt::Display for Mpq {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", self.p, self.q)
    }
}