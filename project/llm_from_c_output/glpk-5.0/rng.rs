/*!
 * A Rust implementation of Knuth's portable pseudo-random number generator.
 * Original C code from GLPK (GNU Linear Programming Kit).
 */

use std::ops::Range;

/// Knuth's portable pseudo-random number generator
pub struct Rng {
    a: [i32; 56],    // pseudo-random values
    fptr: usize,     // index of the next A value to be exported
}

impl Rng {
    /// Create a new pseudo-random number generator
    pub fn new() -> Self {
        let mut rng = Self {
            a: [0; 56],
            fptr: 0,
        };
        rng.a[0] = -1;
        rng.init(1);
        rng
    }

    /// Initialize the pseudo-random number generator with a seed
    pub fn init(&mut self, seed: i32) {
        let mut prev = seed;
        let mut next = 1;
        let mut seed = mod_diff(prev, 0);
        
        self.a[55] = prev;
        let mut i = 21;
        for _ in 0..55 {
            self.a[i] = next;
            next = mod_diff(prev, next);
            
            if seed & 1 != 0 {
                seed = 0x40000000 + (seed >> 1);
            } else {
                seed >>= 1;
            }
            
            next = mod_diff(next, seed);
            prev = self.a[i];
            i = (i + 21) % 55;
        }
        
        for _ in 0..5 {
            self.flip_cycle();
        }
    }

    /// Obtain pseudo-random integer in the range [0, 2^31-1]
    pub fn next(&mut self) -> i32 {
        if self.fptr == 0 || self.a[self.fptr] < 0 {
            self.flip_cycle();
        }
        let val = self.a[self.fptr];
        self.fptr -= 1;
        val
    }

    /// Obtain pseudo-random integer in the range [0, m-1]
    pub fn uniform_int(&mut self, m: i32) -> i32 {
        assert!(m > 0, "m must be positive");
        let t = 0x80000000u32 - (0x80000000u32 % m as u32);
        loop {
            let r = self.next() as u32;
            if r < t {
                return (r % m as u32) as i32;
            }
        }
    }

    /// Obtain pseudo-random number in the range [0, 1]
    pub fn uniform_01(&mut self) -> f64 {
        self.next() as f64 / 2147483647.0
    }

    /// Obtain pseudo-random number in the range [a, b]
    pub fn uniform_range(&mut self, range: Range<f64>) -> f64 {
        range.start + (range.end - range.start) * self.uniform_01()
    }

    fn flip_cycle(&mut self) {
        let mut ii = 1;
        let mut jj = 32;
        
        while jj <= 55 {
            self.a[ii] = mod_diff(self.a[ii], self.a[jj]);
            ii += 1;
            jj += 1;
        }
        
        jj = 1;
        while ii <= 55 {
            self.a[ii] = mod_diff(self.a[ii], self.a[jj]);
            ii += 1;
            jj += 1;
        }
        
        self.fptr = 54;
    }
}

/// Difference modulo 2^31
fn mod_diff(x: i32, y: i32) -> i32 {
    (x - y) & 0x7FFFFFFF
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rng() {
        let mut rng = Rng::new();
        rng.init(-314159);
        
        assert_eq!(rng.next(), 119318998);
        
        for _ in 0..133 {
            rng.next();
        }
        
        assert_eq!(rng.uniform_int(0x55555555), 748103812);
    }
}