use std::f64;

pub type size_t = usize;

#[derive(Clone)]
pub struct GslRngType {
    pub name: &'static str,
    pub max: u64,
    pub min: u64,
    pub size: size_t,
    pub set: Option<fn(&mut RanfState, u64)>,
    pub get: Option<fn(&mut RanfState) -> u64>,
    pub get_double: Option<fn(&mut RanfState) -> f64>,
}

#[derive(Clone)]
pub struct RanfState {
    x0: u16,
    x1: u16,
    x2: u16,
}

const A0: u16 = 0xb175;
const A1: u16 = 0xa2e7;
const A2: u16 = 0x2875;

impl RanfState {
    fn advance(&mut self) {
        let x0 = u64::from(self.x0);
        let x1 = u64::from(self.x1);
        let x2 = u64::from(self.x2);

        let mut r = u64::from(A0) * x0;
        self.x0 = (r & 0xffff) as u16;
        r >>= 16;

        r += u64::from(A0) * x1 + u64::from(A1) * x0;
        self.x1 = (r & 0xffff) as u16;
        r >>= 16;

        r += u64::from(A0) * x2 + u64::from(A1) * x1 + u64::from(A2) * x0;
        self.x2 = (r & 0xffff) as u16;
    }

    fn get(&mut self) -> u64 {
        self.advance();
        (u64::from(self.x2) << 16) + u64::from(self.x1)
    }

    fn get_double(&mut self) -> f64 {
        self.advance();
        f64::from(self.x2) * f64::powi(2.0, -16) 
            + f64::from(self.x1) * f64::powi(2.0, -32)
            + f64::from(self.x0) * f64::powi(2.0, -48)
    }

    fn set(&mut self, s: u64) {
        let (x0, x1, x2) = if s == 0 {
            (0x9cd1, 0x53fc, 0x9482)
        } else {
            (
                ((s | 1) & 0xffff) as u16,
                ((s >> 16) & 0xffff) as u16,
                0,
            )
        };

        let b0 = 0xd6dd;
        let b1 = 0xb894;
        let b2 = 0x5cee;

        let mut r = u64::from(b0) * u64::from(x0);
        self.x0 = (r & 0xffff) as u16;
        r >>= 16;

        r += u64::from(b0) * u64::from(x1) + u64::from(b1) * u64::from(x0);
        self.x1 = (r & 0xffff) as u16;
        r >>= 16;

        r += u64::from(b0) * u64::from(x2) + u64::from(b1) * u64::from(x1) + u64::from(b2) * u64::from(x0);
        self.x2 = (r & 0xffff) as u16;
    }
}

pub static GSL_RNG_RANF: GslRngType = GslRngType {
    name: "ranf",
    max: 0xffffffff,
    min: 0,
    size: std::mem::size_of::<RanfState>(),
    set: Some(RanfState::set),
    get: Some(RanfState::get),
    get_double: Some(RanfState::get_double),
};