use std::cmp::Ordering;
use std::mem;
use std::ptr;

#[derive(Debug, Clone)]
struct MpzSeg {
    d: [u16; 6],
    next: Option<Box<MpzSeg>>,
}

#[derive(Debug, Clone)]
struct Mpz {
    val: i32,
    ptr: Option<Box<MpzSeg>>,
}

#[derive(Debug, Clone)]
struct Mpq {
    p: Mpz,
    q: Mpz,
}

impl MpzSeg {
    fn new() -> Self {
        MpzSeg {
            d: [0; 6],
            next: None,
        }
    }
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
            let mut seg = Box::new(MpzSeg::new());
            seg.d[1] = 0x8000;
            self.ptr = Some(seg);
        } else {
            self.val = val;
        }
    }

    fn clear(&mut self) {
        self.ptr = None;
        self.val = 0;
    }

    fn set(&mut self, other: &Mpz) {
        if ptr::eq(self, other) {
            return;
        }
        self.set_si(0);
        self.val = other.val;
        let mut current = &other.ptr;
        let mut last_seg = None;
        
        while let Some(seg) = current {
            let new_seg = Box::new(MpzSeg {
                d: seg.d,
                next: None,
            });
            
            if let Some(last) = last_seg {
                last.next = Some(new_seg);
                last_seg = last.next.as_mut();
            } else {
                self.ptr = Some(new_seg);
                last_seg = self.ptr.as_mut();
            }
            current = &seg.next;
        }
    }

    fn get_d(&self) -> f64 {
        if self.ptr.is_none() {
            return self.val as f64;
        }
        
        assert_ne!(self.val, 0);
        let mut val = 0.0;
        let mut deg = 1.0;
        let mut current = &self.ptr;
        
        while let Some(seg) = current {
            for &digit in &seg.d {
                val += deg * digit as f64;
                deg *= 65536.0;
            }
            current = &seg.next;
        }
        
        if self.val < 0 {
            -val
        } else {
            val
        }
    }

    fn get_d_2exp(&self, exp: &mut i32) -> f64 {
        if self.ptr.is_none() {
            *exp = 0;
            return self.val as f64;
        }
        
        assert_ne!(self.val, 0);
        let mut val = 0.0;
        let mut n = 0;
        let mut current = &self.ptr;
        
        while let Some(seg) = current {
            for &digit in &seg.d {
                val += digit as f64;
                val /= 65536.0;
                n += 16;
            }
            current = &seg.next;
        }
        
        if self.val < 0 {
            val = -val;
        }
        
        let n1 = val.frexp().1;
        *exp = n + n1 as i32;
        val
    }

    fn swap(&mut self, other: &mut Mpz) {
        mem::swap(&mut self.val, &mut other.val);
        mem::swap(&mut self.ptr, &mut other.ptr);
    }

    fn normalize(&mut self) {
        if self.ptr.is_none() {
            assert_ne!(self.val, i32::MIN);
            return;
        }
        
        assert!(self.val == 1 || self.val == -1);
        
        let mut last_nonzero = None;
        let mut prev = None;
        let mut current = self.ptr.take();
        
        while let Some(mut seg) = current {
            let mut all_zero = true;
            for &digit in &seg.d {
                if digit != 0 {
                    all_zero = false;
                    break;
                }
            }
            
            if !all_zero {
                last_nonzero = Some(seg.d);
            }
            
            prev = Some(seg);
            current = prev.unwrap().next.take();
        }
        
        if last_nonzero.is_none() {
            self.set_si(0);
            return;
        }
        
        // Restore the chain up to last_nonzero
        // ... (implementation omitted for brevity)
        
        // Check if we can convert back to single integer
        if let Some(seg) = &self.ptr {
            if seg.next.is_none() && seg.d[1] <= 0x7fff && seg.d[2..].iter().all(|&x| x == 0) {
                let val = seg.d[0] as i32 + ((seg.d[1] as i32) << 16;
                self.set_si(if self.val < 0 { -val } else { val });
            }
        }
    }

    fn add(&mut self, x: &Mpz, y: &Mpz) {
        // Implementation omitted for brevity
    }

    fn sub(&mut self, x: &Mpz, y: &Mpz) {
        if ptr::eq(x, y) {
            self.set_si(0);
        } else {
            let mut y_copy = y.clone();
            y_copy.neg();
            self.add(x, &y_copy);
        }
    }

    fn mul(&mut self, x: &Mpz, y: &Mpz) {
        // Implementation omitted for brevity
    }

    fn neg(&mut self, x: &Mpz) {
        self.set(x);
        self.val = -self.val;
    }

    fn abs(&mut self, x: &Mpz) {
        self.set(x);
        if self.val < 0 {
            self.val = -self.val;
        }
    }

    fn div(&mut self, rem: Option<&mut Mpz>, x: &Mpz, y: &Mpz) {
        // Implementation omitted for brevity
    }

    fn gcd(&mut self, x: &Mpz, y: &Mpz) {
        // Implementation omitted for brevity
    }

    fn cmp(&self, other: &Mpz) -> Ordering {
        // Implementation omitted for brevity
        Ordering::Equal
    }

    fn sgn(&self) -> i32 {
        if self.val > 0 {
            1
        } else if self.val < 0 {
            -1
        } else {
            0
        }
    }
}

impl Mpq {
    fn new() -> Self {
        Mpq {
            p: Mpz::new(),
            q: Mpz {
                val: 1,
                ptr: None,
            },
        }
    }

    fn clear(&mut self) {
        self.p.clear();
        self.q.set_si(1);
    }

    fn canonicalize(&mut self) {
        assert_ne!(self.q.val, 0);
        
        if self.q.val < 0 {
            self.p.neg(&self.p);
            self.q.neg(&self.q);
        }
        
        let mut f = Mpz::new();
        f.gcd(&self.p, &self.q);
        
        if !(f.val == 1 && f.ptr.is_none()) {
            self.p.div(None, &self.p, &f);
            self.q.div(None, &self.q, &f);
        }
    }

    fn set(&mut self, other: &Mpq) {
        if ptr::eq(self, other) {
            return;
        }
        self.p.set(&other.p);
        self.q.set(&other.q);
    }

    fn set_si(&mut self, p: i32, q: u32) {
        if q == 0 {
            panic!("mpq_set_si: zero denominator not allowed");
        }
        self.p.set_si(p);
        assert!(q <= i32::MAX as u32);
        self.q.set_si(q as i32);
    }

    fn get_d(&self) -> f64 {
        let mut np = 0;
        let mut nq = 0;
        let p = self.p.get_d_2exp(&mut np);
        let q = self.q.get_d_2exp(&mut nq);
        (p / q).ldexp(np - nq)
    }

    fn set_d(&mut self, val: f64) {
        assert!(val.is_finite());
        self.set_si(0, 1);
        
        if val > 0.0 || val < 0.0 {
            let s = if val > 0.0 { 1 } else { -1 };
            let (mut f, mut n) = val.abs().frexp();
            
            let mut temp = Mpz::new();
            while f != 0.0 {
                f *= 16.0;
                n -= 4;
                let d = f as i32;
                assert!((0..=15).contains(&d));
                f -= d as f64;
                
                temp.set_si(16);
                self.p.mul(&self.p, &temp);
                temp.set_si(d);
                self.p.add(&self.p, &temp);
            }
            
            if n > 0 {
                for _ in 0..n {
                    self.p.add(&self.p, &self.p);
                }
            } else if n < 0 {
                for _ in 0..-n {
                    self.q.add(&self.q, &self.q);
                }
                self.canonicalize();
            }
            
            if s < 0 {
                self.neg(self);
            }
        }
    }

    fn add(&mut self, x: &Mpq, y: &Mpq) {
        let mut p = Mpz::new();
        let mut q = Mpz::new();
        
        p.mul(&x.p, &y.q);
        q.mul(&x.q, &y.p);
        p.add(&p, &q);
        q.mul(&x.q, &y.q);
        
        self.p.set(&p);
        self.q.set(&q);
        self.canonicalize();
    }

    fn sub(&mut self, x: &Mpq, y: &Mpq) {
        let mut p = Mpz::new();
        let mut q = Mpz::new();
        
        p.mul(&x.p, &y.q);
        q.mul(&x.q, &y.p);
        p.sub(&p, &q);
        q.mul(&x.q, &y.q);
        
        self.p.set(&p);
        self.q.set(&q);
        self.canonicalize();
    }

    fn mul(&mut self, x: &Mpq, y: &Mpq) {
        self.p.mul(&x.p, &y.p);
        self.q.mul(&x.q, &y.q);
        self.canonicalize();
    }

    fn div(&mut self, x: &Mpq, y: &Mpq) {
        if y.sgn() == 0 {
            panic!("mpq_div: zero divisor not allowed");
        }
        
        let mut p = Mpz::new();
        let mut q = Mpz::new();
        
        p.mul(&x.p, &y.q);
        q.mul(&x.q, &y.p);
        
        self.p.set(&p);
        self.q.set(&q);
        self.canonicalize();
    }

    fn neg(&mut self, x: &Mpq) {
        self.set(x);
        self.p.neg(&self.p);
    }

    fn abs(&mut self, x: &Mpq) {
        self.set(x);
        self.p.abs(&self.p);
        assert!(self.q.sgn() > 0);
    }

    fn cmp(&self, other: &Mpq) -> Ordering {
        let mut temp = Mpq::new();
        temp.sub(self, other);
        temp.sgn().cmp(&0)
    }

    fn sgn(&self) -> i32 {
        let s = self.p.sgn();
        assert!(self.q.sgn() > 0);
        s
    }
}