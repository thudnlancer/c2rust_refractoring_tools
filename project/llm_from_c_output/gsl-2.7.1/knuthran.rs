/* 
 * This generator is taken from
 *
 * Donald E. Knuth
 * The Art of Computer Programming
 * Volume 2
 * Third Edition
 * Addison-Wesley
 * Section 3.6
 *
 * The comments are taken from the book
 * Our comments are signed
 */

const BUFLEN: usize = 2009;     // [Brian]: length of the buffer aa[]
const KK: usize = 100;          // the long lag
const LL: usize = 37;           // the short lag
const MM: u64 = 1 << 30;        // the modulus
const TT: usize = 70;           // guaranteed separation between streams

// Helper functions
#[inline]
fn evenize(x: u64) -> u64 {
    x & (MM - 2)                // make x even
}

#[inline]
fn is_odd(x: u64) -> bool {
    (x & 1) != 0                // the units bit of x
}

#[inline]
fn mod_diff(x: u64, y: u64) -> u64 {
    ((x as i64 - y as i64) & (MM as i64 - 1)) as u64  // (x - y) mod MM
}

struct RanState {
    i: usize,
    aa: [u64; BUFLEN],          // [Carlo]: I can't pass n to ran_array like Knuth does
    ran_x: [u64; KK],           // the generator state
}

impl RanState {
    fn new() -> Self {
        Self {
            i: 0,
            aa: [0; BUFLEN],
            ran_x: [0; KK],
        }
    }

    fn ran_array(&mut self, n: usize) {
        let mut j = 0;
        
        // First fill aa with ran_x values
        while j < KK {
            self.aa[j] = self.ran_x[j];
            j += 1;
        }
        
        // Continue filling aa with differences
        while j < n {
            self.aa[j] = mod_diff(self.aa[j - KK], self.aa[j - LL]);
            j += 1;
        }
        
        // Update first part of ran_x
        let mut i = 0;
        while i < LL {
            self.ran_x[i] = mod_diff(self.aa[j - KK], self.aa[j - LL]);
            i += 1;
            j += 1;
        }
        
        // Update remaining ran_x
        while i < KK {
            self.ran_x[i] = mod_diff(self.aa[j - KK], self.ran_x[i - LL]);
            i += 1;
            j += 1;
        }
    }

    fn ran_get(&mut self) -> u64 {
        if self.i == 0 {
            // fill buffer with new random numbers
            self.ran_array(BUFLEN);
        }
        
        let result = self.aa[self.i];
        self.i = (self.i + 1) % BUFLEN;
        result
    }

    fn ran_get_double(&mut self) -> f64 {
        self.ran_get() as f64 / 1073741824.0  // [Carlo]: RAND_MAX + 1
    }

    fn ran_set(&mut self, mut s: u64) {
        let mut x = [0; KK + KK - 1];  // the preparation buffer
        let mut ss = evenize(s + 2);

        // Bootstrap the buffer
        for j in 0..KK {
            x[j] = ss;
            ss <<= 1;
            if ss >= MM {       // cyclic shift 29 bits
                ss -= MM - 2;
            }
        }

        // Initialize remaining elements
        for j in KK..(KK + KK - 1) {
            x[j] = 0;
        }

        x[1] += 1;              // make x[1] (and only x[1]) odd
        ss = s & (MM - 1);
        let mut t = TT - 1;

        while t > 0 {
            // Square operation
            for j in (1..KK).rev() {
                x[j + j] = x[j];
            }

            for j in ((KK - LL + 1)..(KK + KK - 1)).rev().step_by(2) {
                x[KK + KK - 1 - j] = evenize(x[j]);
            }

            for j in (KK..(KK + KK - 1)).rev() {
                if is_odd(x[j]) {
                    x[j - (KK - LL)] = mod_diff(x[j - (KK - LL)], x[j]);
                    x[j - KK] = mod_diff(x[j - KK], x[j]);
                }
            }

            if is_odd(ss) {
                // Multiply by "z"
                for j in (1..=KK).rev() {
                    x[j] = x[j - 1];
                }
                x[0] = x[KK];   // shift the buffer cyclically
                if is_odd(x[KK]) {
                    x[LL] = mod_diff(x[LL], x[KK]);
                }
            }

            if ss != 0 {
                ss >>= 1;
            } else {
                t -= 1;
            }
        }

        self.i = 0;

        // Update ran_x from preparation buffer
        for j in 0..LL {
            self.ran_x[j + KK - LL] = x[j];
        }
        for j in LL..KK {
            self.ran_x[j - LL] = x[j];
        }
    }
}

pub struct KnuthRan {
    state: RanState,
}

impl KnuthRan {
    pub fn new(seed: u64) -> Self {
        let mut state = RanState::new();
        state.ran_set(seed);
        Self { state }
    }

    pub fn get(&mut self) -> u64 {
        self.state.ran_get()
    }

    pub fn get_double(&mut self) -> f64 {
        self.state.ran_get_double()
    }
}

// GSL-compatible interface
pub const GSL_RNG_MAX: u64 = 0x3fffffff;  // (2^30) - 1
pub const GSL_RNG_MIN: u64 = 0;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_operations() {
        let mut rng = KnuthRan::new(12345);
        let _ = rng.get();
        let _ = rng.get_double();
    }
}