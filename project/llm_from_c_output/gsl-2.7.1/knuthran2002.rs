use std::num::Wrapping;

const BUFLEN: usize = 1009;    // length of the buffer aa[]
const KK: usize = 100;          // the long lag
const LL: usize = 37;           // the short lag
const MM: u64 = 1 << 30;        // the modulus
const TT: usize = 70;           // guaranteed separation between streams

// The units bit of x
fn is_odd(x: u64) -> bool {
    x & 1 != 0
}

// (x - y) mod MM
fn mod_diff(x: u64, y: u64) -> u64 {
    (Wrapping(x) - Wrapping(y)).0 & (MM - 1)
}

pub struct KnuthRan2002 {
    i: usize,
    aa: [u64; BUFLEN],
    ran_x: [u64; KK],  // the generator state
}

impl KnuthRan2002 {
    pub fn new() -> Self {
        let mut rng = KnuthRan2002 {
            i: 0,
            aa: [0; BUFLEN],
            ran_x: [0; KK],
        };
        rng.set(314159); // default seed used by Knuth
        rng
    }

    pub fn with_seed(seed: u64) -> Self {
        let mut rng = KnuthRan2002 {
            i: 0,
            aa: [0; BUFLEN],
            ran_x: [0; KK],
        };
        rng.set(seed);
        rng
    }

    fn ran_array(&mut self, n: usize) {
        let mut j = 0;
        
        for j in 0..KK {
            self.aa[j] = self.ran_x[j];
        }

        for j in KK..n {
            self.aa[j] = mod_diff(self.aa[j - KK], self.aa[j - LL]);
        }

        let mut i = 0;
        j = n;
        while i < LL {
            self.ran_x[i] = mod_diff(self.aa[j - KK], self.aa[j - LL]);
            i += 1;
            j += 1;
        }

        while i < KK {
            self.ran_x[i] = mod_diff(self.aa[j - KK], self.ran_x[i - LL]);
            i += 1;
            j += 1;
        }
    }

    pub fn set(&mut self, s: u64) {
        let mut x = [0u64; KK + KK - 1]; // the preparation buffer

        let mut ss = (s + 2) & (MM - 2);

        for j in 0..KK {
            x[j] = ss; // bootstrap the buffer
            ss <<= 1;
            if ss >= MM {
                // cyclic shift 29 bits
                ss -= MM - 2;
            }
        }
        x[1] += 1; // make x[1] (and only x[1]) odd

        let mut ss = s & (MM - 1);
        let mut t = TT - 1;
        while t > 0 {
            // square
            for j in (1..KK).rev() {
                x[j + j] = x[j];
                x[j + j - 1] = 0;
            }

            for j in (KK..=(KK + KK - 2)).rev() {
                x[j - (KK - LL)] = mod_diff(x[j - (KK - LL)], x[j]);
                x[j - KK] = mod_diff(x[j - KK], x[j]);
            }

            if is_odd(ss) {
                // multiply by "z"
                for j in (1..=KK).rev() {
                    x[j] = x[j - 1];
                }
                x[0] = x[KK]; // shift the buffer cyclically
                x[LL] = mod_diff(x[LL], x[KK]);
            }

            if ss != 0 {
                ss >>= 1;
            } else {
                t -= 1;
            }
        }

        for j in 0..LL {
            self.ran_x[j + KK - LL] = x[j];
        }
        for j in LL..KK {
            self.ran_x[j - LL] = x[j];
        }

        // warm things up
        for _ in 0..10 {
            self.ran_array(KK + KK - 1);
        }

        self.i = 0;
    }

    pub fn get(&mut self) -> u64 {
        if self.i == 0 {
            // fill buffer with new random numbers
            self.ran_array(BUFLEN);
        }

        let v = self.aa[self.i];
        self.i = (self.i + 1) % KK;
        v
    }

    pub fn get_double(&mut self) -> f64 {
        self.get() as f64 / 1073741824.0 // RAND_MAX + 1
    }
}

impl Default for KnuthRan2002 {
    fn default() -> Self {
        Self::new()
    }
}