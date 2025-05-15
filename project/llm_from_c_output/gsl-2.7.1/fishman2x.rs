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
 * It is called "Fishman - L'Ecuyer"
 *
 * This implementation copyright (C) 2001 Carlo Perassi
 * and (C) 2003 Heiko Bauke.
 */

// Fishman constants
const AAA_F: u64 = 48271;
const MMM_F: u64 = 0x7fffffff;      // 2 ^ 31 - 1
const QQQ_F: u64 = 44488;
const RRR_F: u64 = 3399;

// L'Ecuyer constants
const AAA_L: u64 = 40692;
const MMM_L: u64 = 0x7fffff07;      // 2 ^ 31 - 249
const QQQ_L: u64 = 52774;
const RRR_L: u64 = 3791;

#[derive(Debug, Clone)]
pub struct Fishman2xRng {
    x: u64,
    y: u64,
    z: u64,
}

impl Fishman2xRng {
    pub fn new(s: u64) -> Self {
        let mut state = Fishman2xRng { x: 0, y: 0, z: 0 };
        state.set(s);
        state
    }

    pub fn set(&mut self, s: u64) {
        let s_f = if s % MMM_F == 0 { 1 } else { s % MMM_F };
        let s_l = if s % MMM_L == 0 { 1 } else { s % MMM_L };

        self.x = s_f;
        self.y = s_l;
        self.z = if self.x > self.y {
            self.x - self.y
        } else {
            MMM_F + self.x - self.y
        };
    }

    pub fn get(&mut self) -> u64 {
        let r_f = RRR_F * (self.x / QQQ_F);
        let mut y_f = AAA_F * (self.x % QQQ_F) - r_f;
        if y_f > MMM_F {
            y_f = y_f.wrapping_add(MMM_F);
        }
        self.x = y_f;

        let r_l = RRR_L * (self.y / QQQ_L);
        let mut y_l = AAA_L * (self.y % QQQ_L) - r_l;
        if y_l > MMM_L {
            y_l = y_l.wrapping_add(MMM_L);
        }
        self.y = y_l;

        self.z = if self.x > self.y {
            self.x - self.y
        } else {
            MMM_F + self.x - self.y
        };

        self.z
    }

    pub fn get_double(&mut self) -> f64 {
        self.get() as f64 / 2147483647.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rng_sequence() {
        let mut rng = Fishman2xRng::new(1);
        let v1 = rng.get();
        let v2 = rng.get();
        assert_ne!(v1, v2);
    }

    #[test]
    fn test_double_range() {
        let mut rng = Fishman2xRng::new(1);
        let v = rng.get_double();
        assert!(v >= 0.0 && v <= 1.0);
    }
}