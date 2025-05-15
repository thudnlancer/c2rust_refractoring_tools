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
 * This implementation copyright (C) 2001 Brian Gough, Carlo Perassi
 * and (C) 2003 Heiko Bauke.
 */

const AAA: i64 = 40692;
const MMM: u64 = 2147483399;
const QQQ: i64 = 52774;
const RRR: i64 = 3791;

#[derive(Clone)]
pub struct Lecuyer21Rng {
    x: u64,
}

impl Lecuyer21Rng {
    pub fn new(seed: u64) -> Self {
        let mut s = seed;
        if s % MMM == 0 {
            s = 1; // default seed is 1
        }
        Lecuyer21Rng { x: s % MMM }
    }

    pub fn next(&mut self) -> u64 {
        let y = self.x as i64;
        let r = RRR * (y / QQQ);

        let mut y = AAA * (y % QQQ) - r;
        if y < 0 {
            y += MMM as i64;
        }

        self.x = y as u64;
        self.x
    }

    pub fn next_double(&mut self) -> f64 {
        self.next() as f64 / MMM as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rng() {
        let mut rng = Lecuyer21Rng::new(1);
        let _ = rng.next();
        let _ = rng.next_double();
    }
}