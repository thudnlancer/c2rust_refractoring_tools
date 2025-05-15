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
 * It is called "Waterman".
 *
 * This implementation copyright (C) 2001 Carlo Perassi
 * and (C) 2003 Heiko Bauke.
 */

const AA: u64 = 1566083941;
const MM: u64 = 0xffffffff; /* 2 ^ 32 - 1 */

pub struct Waterman14 {
    x: u64,
}

impl Waterman14 {
    pub fn new(s: u64) -> Self {
        let mut seed = s;
        if seed == 0 {
            seed = 1; // default seed is 1
        }
        Waterman14 { x: seed & MM }
    }

    pub fn get(&mut self) -> u64 {
        self.x = (AA * self.x) & MM;
        self.x
    }

    pub fn get_double(&mut self) -> f64 {
        self.get() as f64 / 4294967296.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_waterman14() {
        let mut rng = Waterman14::new(1);
        let val = rng.get();
        assert_ne!(val, 0);
        let dval = rng.get_double();
        assert!(dval >= 0.0 && dval < 1.0);
    }

    #[test]
    fn test_seed_zero() {
        let rng = Waterman14::new(0);
        assert_eq!(rng.x, 1);
    }
}