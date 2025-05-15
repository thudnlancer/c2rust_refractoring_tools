use std::num::Wrapping;

pub struct SlatecState {
    x0: i64,
    x1: i64,
}

pub struct SlatecRng {
    state: SlatecState,
}

impl SlatecRng {
    const P: i64 = 4_194_304;
    const A1: i64 = 1_536;
    const A0: i64 = 1_029;
    const A1MA0: i64 = 507;
    const C: i64 = 1_731;

    pub fn new() -> Self {
        SlatecRng {
            state: SlatecState { x0: 0, x1: 0 },
        }
    }

    pub fn with_seed(s: u64) -> Self {
        let mut rng = SlatecRng::new();
        rng.set(s);
        rng
    }

    pub fn set(&mut self, s: u64) {
        // Only eight seeds are permitted
        let s = (s % 8) as i64;
        let s = s * (Self::P / 8);

        self.state.x0 = s % 2048;
        self.state.x1 = (s - self.state.x0) / 2048;
    }

    pub fn get(&mut self) -> u64 {
        let y0 = Wrapping(Self::A0) * Wrapping(self.state.x0);
        let mut y1 = Wrapping(Self::A1) * Wrapping(self.state.x1) 
            + Wrapping(Self::A1MA0) * Wrapping(self.state.x0 - self.state.x1) 
            + y0;
        
        let mut y0 = y0 + Wrapping(Self::C);
        self.state.x0 = (y0 % Wrapping(2048)).0;
        y1 = y1 + Wrapping((y0.0 - self.state.x0) / 2048);
        self.state.x1 = (y1 % Wrapping(2048)).0;

        (self.state.x1 * 2048 + self.state.x0) as u64
    }

    pub fn get_double(&mut self) -> f64 {
        self.get() as f64 / Self::P as f64
    }
}

impl Default for SlatecRng {
    fn default() -> Self {
        Self::new()
    }
}