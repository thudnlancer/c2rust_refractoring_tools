use std::mem;

const NEXT: [usize; 12] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 0];
const SNEXT: [usize; 24] = [
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 0,
];

const SBASE: f64 = 16777216.0; // 2^24
const SONE_BIT: f64 = 1.0 / 16777216.0; // 1/2^24
const ONE_BIT: f64 = 1.0 / 281474976710656.0; // 1/2^48

const SHIFT: f64 = 268435456.0; // 2^28

macro_rules! ranlux_step {
    ($x1:expr, $x2:expr, $i1:expr, $i2:expr, $i3:expr) => {
        $x1 = xdbl[$i1] - xdbl[$i2];
        if $x2 < 0.0 {
            $x1 -= ONE_BIT;
            $x2 += 1.0;
        }
        xdbl[$i3] = $x2;
    };
}

#[derive(Clone)]
struct RanlxsState {
    xdbl: [f64; 12],
    ydbl: [f64; 12],
    carry: f64,
    xflt: [f32; 24],
    ir: usize,
    jr: usize,
    is: usize,
    is_old: usize,
    pr: usize,
}

impl RanlxsState {
    fn new() -> Self {
        Self {
            xdbl: [0.0; 12],
            ydbl: [0.0; 12],
            carry: 0.0,
            xflt: [0.0; 24],
            ir: 0,
            jr: 0,
            is: 0,
            is_old: 0,
            pr: 0,
        }
    }

    fn increment_state(&mut self) {
        let mut k = 0;
        let mut kmax;
        let mut m;
        let mut x;
        let mut y1;
        let mut y2;
        let mut y3;

        let xflt = &mut self.xflt;
        let xdbl = &mut self.xdbl;
        let ydbl = &mut self.ydbl;
        let mut carry = self.carry;
        let mut ir = self.ir;
        let mut jr = self.jr;

        while ir > 0 {
            y1 = xdbl[jr] - xdbl[ir];
            y2 = y1 - carry;
            if y2 < 0.0 {
                carry = ONE_BIT;
                y2 += 1.0;
            } else {
                carry = 0.0;
            }
            xdbl[ir] = y2;
            ir = NEXT[ir];
            jr = NEXT[jr];
            k += 1;
        }

        kmax = self.pr - 12;

        while k <= kmax {
            y1 = xdbl[7] - xdbl[0];
            y1 -= carry;

            ranlux_step!(y2, y1, 8, 1, 0);
            ranlux_step!(y3, y2, 9, 2, 1);
            ranlux_step!(y1, y3, 10, 3, 2);
            ranlux_step!(y2, y1, 11, 4, 3);
            ranlux_step!(y3, y2, 0, 5, 4);
            ranlux_step!(y1, y3, 1, 6, 5);
            ranlux_step!(y2, y1, 2, 7, 6);
            ranlux_step!(y3, y2, 3, 8, 7);
            ranlux_step!(y1, y3, 4, 9, 8);
            ranlux_step!(y2, y1, 5, 10, 9);
            ranlux_step!(y3, y2, 6, 11, 10);

            if y3 < 0.0 {
                carry = ONE_BIT;
                y3 += 1.0;
            } else {
                carry = 0.0;
            }
            xdbl[11] = y3;
            k += 12;
        }

        kmax = self.pr;

        while k < kmax {
            y1 = xdbl[jr] - xdbl[ir];
            y2 = y1 - carry;
            if y2 < 0.0 {
                carry = ONE_BIT;
                y2 += 1.0;
            } else {
                carry = 0.0;
            }
            xdbl[ir] = y2;
            ydbl[ir] = y2 + SHIFT;
            ir = NEXT[ir];
            jr = NEXT[jr];
            k += 1;
        }

        ydbl[ir] = xdbl[ir] + SHIFT;

        let mut k = NEXT[ir];
        while k > 0 {
            ydbl[k] = xdbl[k] + SHIFT;
            k = NEXT[k];
        }

        m = 0;
        for k in 0..12 {
            x = xdbl[k];
            y2 = ydbl[k] - SHIFT;
            if y2 > x {
                y2 -= SONE_BIT;
            }
            y1 = (x - y2) * SBASE;

            xflt[m] = y1 as f32;
            m += 1;
            xflt[m] = y2 as f32;
            m += 1;
        }

        self.ir = ir;
        self.is = 2 * ir;
        self.is_old = 2 * ir;
        self.jr = jr;
        self.carry = carry;
    }

    fn get_double(&mut self) -> f64 {
        let is = SNEXT[self.is];
        self.is = is;

        if is == self.is_old {
            self.increment_state();
        }

        f64::from(self.xflt[self.is])
    }

    fn get(&mut self) -> u32 {
        (self.get_double() * SBASE) as u32
    }

    fn set_lux(&mut self, s: u32, luxury: usize) {
        let mut ibit = 0;
        let mut jbit = 18;
        let mut xbit = [0; 31];
        let mut x;
        let mut y;

        let mut seed = s;

        if seed == 0 {
            seed = 1;
        }

        let mut i = seed & 0x7FFFFFFF;

        for k in 0..31 {
            xbit[k] = i % 2;
            i /= 2;
        }

        for k in 0..12 {
            x = 0.0;

            for _ in 1..=48 {
                y = f64::from(xbit[ibit]);
                x += x + y;
                xbit[ibit] = (xbit[ibit] + xbit[jbit]) % 2;
                ibit = (ibit + 1) % 31;
                jbit = (jbit + 1) % 31;
            }
            self.xdbl[k] = ONE_BIT * x;
        }

        self.carry = 0.0;
        self.ir = 0;
        self.jr = 7;
        self.is = 23;
        self.is_old = 0;
        self.pr = luxury;
    }
}

pub struct Ranlxs0 {
    state: RanlxsState,
}

impl Ranlxs0 {
    pub fn new(s: u32) -> Self {
        let mut state = RanlxsState::new();
        state.set_lux(s, 109);
        Self { state }
    }

    pub fn get(&mut self) -> u32 {
        self.state.get()
    }

    pub fn get_double(&mut self) -> f64 {
        self.state.get_double()
    }
}

pub struct Ranlxs1 {
    state: RanlxsState,
}

impl Ranlxs1 {
    pub fn new(s: u32) -> Self {
        let mut state = RanlxsState::new();
        state.set_lux(s, 202);
        Self { state }
    }

    pub fn get(&mut self) -> u32 {
        self.state.get()
    }

    pub fn get_double(&mut self) -> f64 {
        self.state.get_double()
    }
}

pub struct Ranlxs2 {
    state: RanlxsState,
}

impl Ranlxs2 {
    pub fn new(s: u32) -> Self {
        let mut state = RanlxsState::new();
        state.set_lux(s, 397);
        Self { state }
    }

    pub fn get(&mut self) -> u32 {
        self.state.get()
    }

    pub fn get_double(&mut self) -> f64 {
        self.state.get_double()
    }
}