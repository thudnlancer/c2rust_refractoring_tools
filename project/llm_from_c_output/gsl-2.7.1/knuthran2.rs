/*
 * This generator is taken from
 *
 * Donald E. Knuth
 * The Art of Computer Programming
 * Volume 2
 * Third Edition
 * Addison-Wesley
 * Page 108
 *
 * This implementation copyright (C) 2001 Carlo Perassi
 * and (C) 2003 Heiko Bauke.
 */

const AA1: u64 = 271828183;
const AA2: u64 = 1833324378;    /* = -314159269 mod (2 ^ 31 -1) */
const MM: u64 = 0x7fffffff;     /* 2 ^ 31 - 1 */
const CEIL_SQRT_MM: u64 = 46341; /* sqrt(2 ^ 31 - 1) */

pub struct KnuthRan2 {
    x0: u64,
    x1: u64,
}

impl KnuthRan2 {
    pub fn new(seed: u64) -> Self {
        let mut s = seed;
        if s % MM == 0 {
            s = 1; // default seed is 1
        }
        KnuthRan2 {
            x0: s % MM,
            x1: s % MM,
        }
    }

    pub fn get(&mut self) -> u64 {
        let xtmp = self.x1;
        self.x1 = schrage_mult(AA1, self.x1, MM, CEIL_SQRT_MM)
            .wrapping_add(schrage_mult(AA2, self.x0, MM, CEIL_SQRT_MM));
        
        if self.x1 >= MM {
            self.x1 -= MM;
        }
        
        self.x0 = xtmp;
        self.x1
    }

    pub fn get_double(&mut self) -> f64 {
        self.get() as f64 / 2147483647.0
    }
}

fn schrage_mult(a: u64, b: u64, m: u64, ceil_sqrt_m: u64) -> u64 {
    let q = m / a;
    let r = m % a;
    
    let t = a * (b % q) - r * (b / q);
    
    if t < 0 {
        t + m
    } else {
        t
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_knuthran2() {
        let mut rng = KnuthRan2::new(12345);
        let val1 = rng.get();
        let val2 = rng.get();
        assert_ne!(val1, val2);
        
        let dval = rng.get_double();
        assert!(dval >= 0.0 && dval <= 1.0);
    }
}