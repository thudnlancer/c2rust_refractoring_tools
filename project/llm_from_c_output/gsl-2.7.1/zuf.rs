use std::num::Wrapping;

const ZUF_RANDMAX: u32 = 16777216; // 2^24

#[derive(Debug)]
struct ZufState {
    n: usize,
    u: [u32; 607],
}

impl ZufState {
    fn new() -> Self {
        Self {
            n: 0,
            u: [0; 607],
        }
    }

    fn get(&mut self) -> u32 {
        let n = self.n;
        let m = (n + 334) % 607; // 607-273=334
        let mut t = self.u[n] + self.u[m];

        while t > ZUF_RANDMAX {
            t -= ZUF_RANDMAX;
        }

        self.u[n] = t;

        self.n = if n == 606 { 0 } else { n + 1 };

        t
    }

    fn get_double(&mut self) -> f64 {
        f64::from(self.get()) / 16777216.0
    }

    fn set(&mut self, mut s: u32) {
        // A very elaborate seeding procedure is provided with the
        // zufall package; this is virtually a copy of that procedure

        // Initialized data
        let mut kl = 9373;
        let mut ij = 1802;

        // Local variables
        let mut i: i32;
        let mut j: i32;
        let mut k: i32;
        let mut l: i32;
        let mut m: i32;
        let mut x: f64;
        let mut y: f64;

        self.n = 0;

        // generates initial seed buffer by linear congruential
        // method. Taken from Marsaglia, FSU report FSU-SCRI-87-50
        // variable seed should be 0 < seed <31328

        if s == 0 {
            s = 1802; // default seed is 1802
        }

        ij = s as i32;

        i = ij / 177 % 177 + 2;
        j = ij % 177 + 2;
        k = kl / 169 % 178 + 1;
        l = kl % 169;

        for ii in 0..607 {
            x = 0.0;
            y = 0.5;
            // 24 bits??
            for _ in 1..=24 {
                m = (Wrapping(i) * Wrapping(j) % Wrapping(179) * Wrapping(k) % Wrapping(179)).0;
                i = j;
                j = k;
                k = m;
                l = (Wrapping(l) * Wrapping(53) + Wrapping(1)).0 % 169;
                if (Wrapping(l) * Wrapping(m)).0 % 64 >= 32 {
                    x += y;
                }
                y *= 0.5;
            }
            self.u[ii] = (x * f64::from(ZUF_RANDMAX)) as u32;
        }
    }
}

pub struct ZufRng {
    state: ZufState,
}

impl ZufRng {
    pub fn new() -> Self {
        let mut state = ZufState::new();
        state.set(0);
        Self { state }
    }

    pub fn new_with_seed(seed: u32) -> Self {
        let mut state = ZufState::new();
        state.set(seed);
        Self { state }
    }

    pub fn next(&mut self) -> u32 {
        self.state.get()
    }

    pub fn next_double(&mut self) -> f64 {
        self.state.get_double()
    }
}

impl Default for ZufRng {
    fn default() -> Self {
        Self::new()
    }
}