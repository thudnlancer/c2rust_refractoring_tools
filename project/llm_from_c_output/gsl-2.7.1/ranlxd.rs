/* rng/ranlxd.rs

   This is a Rust implementation of Martin Luescher's second generation
   double-precision (48-bit) version of the RANLUX generator.

   Translated from the original C code from GNU Scientific Library.
*/

const ONE_BIT: f64 = 1.0 / 281474976710656.0;  // 1/2^48
const NEXT: [usize; 12] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 0];

struct RanlxdState {
    xdbl: [f64; 12],
    carry: f64,
    ir: usize,
    jr: usize,
    ir_old: usize,
    pr: usize,
}

impl RanlxdState {
    fn increment_state(&mut self) {
        let mut k = 0;
        let mut ir = self.ir;
        let mut jr = self.jr;
        let mut carry = self.carry;

        // First loop
        while ir > 0 {
            let y1 = self.xdbl[jr] - self.xdbl[ir];
            let mut y2 = y1 - carry;
            if y2 < 0.0 {
                carry = ONE_BIT;
                y2 += 1.0;
            } else {
                carry = 0.0;
            }
            self.xdbl[ir] = y2;
            ir = NEXT[ir];
            jr = NEXT[jr];
            k += 1;
        }

        // Middle section with macro-equivalent steps
        let kmax = self.pr - 12;
        while k <= kmax {
            let mut y1 = self.xdbl[7] - self.xdbl[0];
            y1 -= carry;

            macro_rules! ranlux_step {
                ($y_out:ident, $y_in:ident, $i1:expr, $i2:expr, $i3:expr) => {
                    let $y_out = self.xdbl[$i1] - self.xdbl[$i2];
                    let $y_out = if $y_in < 0.0 {
                        $y_out - ONE_BIT
                    } else {
                        $y_out
                    };
                    self.xdbl[$i3] = $y_out;
                };
            }

            let y2;
            ranlux_step!(y2, y1, 8, 1, 0);
            let y3;
            ranlux_step!(y3, y2, 9, 2, 1);
            let y1;
            ranlux_step!(y1, y3, 10, 3, 2);
            let y2;
            ranlux_step!(y2, y1, 11, 4, 3);
            let y3;
            ranlux_step!(y3, y2, 0, 5, 4);
            let y1;
            ranlux_step!(y1, y3, 1, 6, 5);
            let y2;
            ranlux_step!(y2, y1, 2, 7, 6);
            let y3;
            ranlux_step!(y3, y2, 3, 8, 7);
            let y1;
            ranlux_step!(y1, y3, 4, 9, 8);
            let y2;
            ranlux_step!(y2, y1, 5, 10, 9);
            let y3;
            ranlux_step!(y3, y2, 6, 11, 10);

            if y3 < 0.0 {
                carry = ONE_BIT;
            } else {
                carry = 0.0;
            }
            k += 12;
        }

        // Final loop
        while k < self.pr {
            let y1 = self.xdbl[jr] - self.xdbl[ir];
            let mut y2 = y1 - carry;
            if y2 < 0.0 {
                carry = ONE_BIT;
                y2 += 1.0;
            } else {
                carry = 0.0;
            }
            self.xdbl[ir] = y2;
            ir = NEXT[ir];
            jr = NEXT[jr];
            k += 1;
        }

        self.ir = ir;
        self.ir_old = ir;
        self.jr = jr;
        self.carry = carry;
    }

    fn get_double(&mut self) -> f64 {
        let ir = self.ir;
        self.ir = NEXT[ir];

        if self.ir == self.ir_old {
            self.increment_state();
        }

        self.xdbl[self.ir]
    }

    fn get(&mut self) -> u32 {
        (self.get_double() * 4294967296.0) as u32  // 2^32
    }

    fn set_lux(&mut self, s: u32, luxury: usize) {
        let mut seed = if s == 0 { 1 } else { s }; // default seed is 1

        let mut xbit = [0; 31];
        for k in 0..31 {
            xbit[k] = (seed % 2) as i32;
            seed /= 2;
        }

        let mut ibit = 0;
        let mut jbit = 18;

        for k in 0..12 {
            let mut x = 0.0;

            for _ in 1..=48 {
                let y = ((xbit[ibit] + 1) % 2) as f64;
                x += x + y;
                xbit[ibit] = (xbit[ibit] + xbit[jbit]) % 2;
                ibit = (ibit + 1) % 31;
                jbit = (jbit + 1) % 31;
            }
            self.xdbl[k] = ONE_BIT * x;
        }

        self.carry = 0.0;
        self.ir = 11;
        self.jr = 7;
        self.ir_old = 0;
        self.pr = luxury;
    }
}

pub struct Ranlxd1 {
    state: RanlxdState,
}

impl Ranlxd1 {
    pub fn new(seed: u32) -> Self {
        let mut state = RanlxdState {
            xdbl: [0.0; 12],
            carry: 0.0,
            ir: 0,
            jr: 0,
            ir_old: 0,
            pr: 202,
        };
        state.set_lux(seed, 202);
        Self { state }
    }

    pub fn get(&mut self) -> u32 {
        self.state.get()
    }

    pub fn get_double(&mut self) -> f64 {
        self.state.get_double()
    }
}

pub struct Ranlxd2 {
    state: RanlxdState,
}

impl Ranlxd2 {
    pub fn new(seed: u32) -> Self {
        let mut state = RanlxdState {
            xdbl: [0.0; 12],
            carry: 0.0,
            ir: 0,
            jr: 0,
            ir_old: 0,
            pr: 397,
        };
        state.set_lux(seed, 397);
        Self { state }
    }

    pub fn get(&mut self) -> u32 {
        self.state.get()
    }

    pub fn get_double(&mut self) -> f64 {
        self.state.get_double()
    }
}