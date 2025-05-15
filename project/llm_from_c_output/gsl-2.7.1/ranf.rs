/* rng/ranf.rs

 * This is a Rust translation of the CRAY RANF generator from GSL.
 * Original C code Copyright (C) 1996-2007 James Theiler, Brian Gough
 * under GNU General Public License.
 */

use std::f64;

const A0: u32 = 0xB175;
const A1: u32 = 0xA2E7;
const A2: u32 = 0x2875;

const B0: u32 = 0xD6DD;
const B1: u32 = 0xB894;
const B2: u32 = 0x5CEE;

const DEFAULT_X0: u16 = 0x9CD1;
const DEFAULT_X1: u16 = 0x53FC;
const DEFAULT_X2: u16 = 0x9482;

#[derive(Clone)]
struct RanfState {
    x0: u16,
    x1: u16,
    x2: u16,
}

impl RanfState {
    fn advance(&mut self) {
        let x0 = self.x0 as u32;
        let x1 = self.x1 as u32;
        let x2 = self.x2 as u32;

        let mut r = A0 * x0;
        self.x0 = (r & 0xFFFF) as u16;

        r >>= 16;
        r += A0 * x1 + A1 * x0;
        self.x1 = (r & 0xFFFF) as u16;

        r >>= 16;
        r += A0 * x2 + A1 * x1 + A2 * x0;
        self.x2 = (r & 0xFFFF) as u16;
    }

    fn get(&mut self) -> u32 {
        self.advance();
        ((self.x2 as u32) << 16) | (self.x1 as u32)
    }

    fn get_double(&mut self) -> f64 {
        self.advance();
        f64::from(self.x2) / 65536.0 + 
        f64::from(self.x1) / 4294967296.0 + 
        f64::from(self.x0) / 281474976710656.0
    }

    fn set(&mut self, seed: Option<u32>) {
        let (x0, x1, x2) = match seed {
            None => (DEFAULT_X0, DEFAULT_X1, DEFAULT_X2),
            Some(s) => ((s | 1) as u16, (s >> 16) as u16, 0),
        };

        let mut r = B0 * u32::from(x0);
        self.x0 = (r & 0xFFFF) as u16;

        r >>= 16;
        r += B0 * u32::from(x1) + B1 * u32::from(x0);
        self.x1 = (r & 0xFFFF) as u16;

        r >>= 16;
        r += B0 * u32::from(x2) + B1 * u32::from(x1) + B2 * u32::from(x0);
        self.x2 = (r & 0xFFFF) as u16;
    }
}

pub struct RanfRng {
    state: RanfState,
}

impl RanfRng {
    pub fn new(seed: Option<u32>) -> Self {
        let mut state = RanfState {
            x0: 0,
            x1: 0,
            x2: 0,
        };
        state.set(seed);
        Self { state }
    }

    pub fn next_u32(&mut self) -> u32 {
        self.state.get()
    }

    pub fn next_f64(&mut self) -> f64 {
        self.state.get_double()
    }
}

impl Default for RanfRng {
    fn default() -> Self {
        Self::new(None)
    }
}