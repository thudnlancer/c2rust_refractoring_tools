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
 * It is called "Fishman"
 *
 * This implementation copyright (C) 2001 Carlo Perassi
 * and (C) 2003 Heiko Bauke.
 */

pub struct Fishman20Rng {
    x: u32,
}

impl Fishman20Rng {
    const M: u32 = 2147483647;
    const A: u32 = 48271;
    const Q: u32 = 44488;
    const R: u32 = 3399;

    pub fn new(seed: u32) -> Self {
        let mut s = seed;
        if s % Self::M == 0 {
            s = 1; // default seed is 1
        }
        Fishman20Rng { x: s & Self::M }
    }

    pub fn next(&mut self) -> u32 {
        let x = self.x;
        let h = x / Self::Q;
        let t = (Self::A as i64) * ((x - h * Self::Q) as i64) - (h as i64) * (Self::R as i64);

        self.x = if t < 0 {
            (t + Self::M as i64) as u32
        } else {
            t as u32
        };

        self.x
    }

    pub fn next_double(&mut self) -> f64 {
        self.next() as f64 / 2147483647.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fishman20() {
        let mut rng = Fishman20Rng::new(1);
        let _ = rng.next();
        let _ = rng.next_double();
    }
}