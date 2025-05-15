/*
 * This generator is taken from
 *
 * Donald E. Knuth
 * The Art of Computer Programming
 * Volume 2
 * Third Edition
 * Addison-Wesley
 * Page 106-108
 *
 * It is called "Fishman - Moore III".
 *
 * This implementation copyright (C) 2001 Carlo Perassi
 * and (C) 2003 Heiko Bauke.
 */

const AA: u64 = 62089911;
const MM: u64 = 0x7fffffff;       /* 2 ^ 31 - 1 */
const CEIL_SQRT_MM: u64 = 46341;  /* ceil(sqrt(2 ^ 31 - 1)) */

pub struct Fishman18Rng {
    x: u64,
}

impl Fishman18Rng {
    pub fn new(seed: u64) -> Self {
        let mut s = seed;
        if s % MM == 0 {
            s = 1; // default seed is 1
        }
        Fishman18Rng { x: s % MM }
    }

    pub fn next(&mut self) -> u64 {
        self.x = schrage_mult(AA, self.x, MM, CEIL_SQRT_MM);
        self.x
    }

    pub fn next_double(&mut self) -> f64 {
        self.next() as f64 / 2147483647.0
    }
}

fn schrage_mult(a: u64, x: u64, m: u64, ceil_sqrt_m: u64) -> u64 {
    let q = m / a;
    let r = m % a;
    let t = a * (x % q) - r * (x / q);
    if t < 0 { t + m } else { t }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rng() {
        let mut rng = Fishman18Rng::new(1);
        let val1 = rng.next();
        let val2 = rng.next();
        assert_ne!(val1, val2);
        
        let dval = rng.next_double();
        assert!(dval >= 0.0 && dval <= 1.0);
    }

    #[test]
    fn test_seed_zero() {
        let mut rng = Fishman18Rng::new(0);
        assert_eq!(rng.x, 1); // Should default to 1
    }
}