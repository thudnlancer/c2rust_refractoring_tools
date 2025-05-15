use std::f64;

/// This is the Unix rand48() generator. The generator returns the
/// upper 32 bits from each term of the sequence,
///
/// x_{n+1} = (a x_n + c) mod m 
///
/// using 48-bit unsigned arithmetic, with a = 0x5DEECE66D , c = 0xB
/// and m = 2^48. The seed specifies the upper 32 bits of the initial
/// value, x_1, with the lower 16 bits set to 0x330E.
///
/// The theoretical value of x_{10001} is 244131582646046.
///
/// The period of this generator is ? FIXME (probably around 2^48).
const A0: u16 = 0xE66D;
const A1: u16 = 0xDEEC;
const A2: u16 = 0x0005;
const C0: u16 = 0x000B;

#[derive(Clone)]
pub struct Rand48State {
    x0: u16,
    x1: u16,
    x2: u16,
}

impl Rand48State {
    fn advance(&mut self) {
        // work with u32 throughout to get correct integer promotions
        let x0 = u32::from(self.x0);
        let x1 = u32::from(self.x1);
        let x2 = u32::from(self.x2);

        let mut a = u32::from(A0) * x0 + u32::from(C0);
        self.x0 = (a & 0xFFFF) as u16;

        a >>= 16;

        // although the next line may overflow we only need the top 16 bits
        // in the following stage, so it does not matter
        a += u32::from(A0) * x1 + u32::from(A1) * x0;
        self.x1 = (a & 0xFFFF) as u16;

        a >>= 16;
        a += u32::from(A0) * x2 + u32::from(A1) * x1 + u32::from(A2) * x0;
        self.x2 = (a & 0xFFFF) as u16;
    }

    fn get(&mut self) -> u32 {
        self.advance();
        (u32::from(self.x2) << 16 | u32::from(self.x1)
    }

    fn get_double(&mut self) -> f64 {
        self.advance();
        f64::from(self.x2) * f64::powi(2.0, -16)
            + f64::from(self.x1) * f64::powi(2.0, -32)
            + f64::from(self.x0) * f64::powi(2.0, -48)
    }

    fn set(&mut self, s: u32) {
        if s == 0 {
            // default seed
            self.x0 = 0x330E;
            self.x1 = 0xABCD;
            self.x2 = 0x1234;
        } else {
            self.x0 = 0x330E;
            self.x1 = (s & 0xFFFF) as u16;
            self.x2 = ((s >> 16) & 0xFFFF) as u16;
        }
    }
}

pub struct Rand48 {
    state: Rand48State,
}

impl Rand48 {
    pub fn new() -> Self {
        let mut state = Rand48State {
            x0: 0,
            x1: 0,
            x2: 0,
        };
        state.set(0);
        Rand48 { state }
    }

    pub fn with_seed(seed: u32) -> Self {
        let mut state = Rand48State {
            x0: 0,
            x1: 0,
            x2: 0,
        };
        state.set(seed);
        Rand48 { state }
    }

    pub fn next_u32(&mut self) -> u32 {
        self.state.get()
    }

    pub fn next_f64(&mut self) -> f64 {
        self.state.get_double()
    }
}

impl Default for Rand48 {
    fn default() -> Self {
        Self::new()
    }
}