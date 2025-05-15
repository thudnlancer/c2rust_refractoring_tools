use std::num::Wrapping;

const M1: i64 = 2147483647;
const M2: i64 = 2145483479;

const A2: i64 = 63308;
const QA2: i64 = 33921;
const RA2: i64 = 12979;
const A3: i64 = -183326;
const QA3: i64 = 11714;
const RA3: i64 = 2883;
const B1: i64 = 86098;
const QB1: i64 = 24919;
const RB1: i64 = 7417;
const B3: i64 = -539608;
const QB3: i64 = 3976;
const RB3: i64 = 2071;

#[derive(Debug, Clone)]
pub struct CmrgState {
    x1: i64,
    x2: i64,
    x3: i64,
    y1: i64,
    y2: i64,
    y3: i64,
}

impl CmrgState {
    pub fn new() -> Self {
        Self {
            x1: 0,
            x2: 0,
            x3: 0,
            y1: 0,
            y2: 0,
            y3: 0,
        }
    }

    pub fn set(&mut self, s: u64) {
        let mut s = s;
        if s == 0 {
            s = 1;
        }

        fn lcg(n: u64) -> u64 {
            (Wrapping(69069) * Wrapping(n)).0 & 0xffffffff
        }

        s = lcg(s);
        self.x1 = (s as i64) % M1;
        s = lcg(s);
        self.x2 = (s as i64) % M1;
        s = lcg(s);
        self.x3 = (s as i64) % M1;

        s = lcg(s);
        self.y1 = (s as i64) % M2;
        s = lcg(s);
        self.y2 = (s as i64) % M2;
        s = lcg(s);
        self.y3 = (s as i64) % M2;

        // Warm it up
        for _ in 0..7 {
            self.get();
        }
    }

    pub fn get(&mut self) -> u64 {
        // Component 1
        let h3 = self.x3 / QA3;
        let mut p3 = -A3 * (self.x3 - h3 * QA3) - h3 * RA3;

        let h2 = self.x2 / QA2;
        let mut p2 = A2 * (self.x2 - h2 * QA2) - h2 * RA2;

        if p3 < 0 {
            p3 += M1;
        }
        if p2 < 0 {
            p2 += M1;
        }

        self.x3 = self.x2;
        self.x2 = self.x1;
        self.x1 = p2 - p3;
        if self.x1 < 0 {
            self.x1 += M1;
        }

        // Component 2
        let h3 = self.y3 / QB3;
        let mut p3 = -B3 * (self.y3 - h3 * QB3) - h3 * RB3;

        let h1 = self.y1 / QB1;
        let mut p1 = B1 * (self.y1 - h1 * QB1) - h1 * RB1;

        if p3 < 0 {
            p3 += M2;
        }
        if p1 < 0 {
            p1 += M2;
        }

        self.y3 = self.y2;
        self.y2 = self.y1;
        self.y1 = p1 - p3;
        if self.y1 < 0 {
            self.y1 += M2;
        }

        if self.x1 < self.y1 {
            (self.x1 - self.y1 + M1) as u64
        } else {
            (self.x1 - self.y1) as u64
        }
    }

    pub fn get_double(&mut self) -> f64 {
        self.get() as f64 / 2147483647.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cmrg() {
        let mut rng = CmrgState::new();
        rng.set(1);
        
        // Test after warm-up (7 iterations) + 10000 iterations
        let mut z = 0;
        for _ in 0..10007 {
            z = rng.get();
        }
        assert_eq!(z, 719452880);
    }
}