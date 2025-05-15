use std::num::Wrapping;

// State structs
#[derive(Clone)]
struct Random8State {
    x: u32,
}

#[derive(Clone)]
struct Random32State {
    i: usize,
    j: usize,
    x: [u32; 7],
}

#[derive(Clone)]
struct Random64State {
    i: usize,
    j: usize,
    x: [u32; 15],
}

#[derive(Clone)]
struct Random128State {
    i: usize,
    j: usize,
    x: [u32; 31],
}

#[derive(Clone)]
struct Random256State {
    i: usize,
    j: usize,
    x: [u32; 63],
}

// RNG trait
trait RandomGenerator {
    fn get(&mut self) -> u32;
    fn get_double(&mut self) -> f64 {
        self.get() as f64 / 2147483648.0
    }
    fn set(&mut self, seed: u32);
}

// Implementations
impl RandomGenerator for Random8State {
    fn get(&mut self) -> u32 {
        self.x = ((Wrapping(1103515245) * Wrapping(self.x) + Wrapping(12345)).0 & 0x7FFFFFFF;
        self.x
    }

    fn set(&mut self, seed: u32) {
        self.x = if seed == 0 { 1 } else { seed };
    }
}

fn random_get(i: &mut usize, j: &mut usize, n: usize, x: &mut [u32]) -> u32 {
    x[*i] = x[*i].wrapping_add(x[*j]);
    let k = (x[*i] >> 1) & 0x7FFFFFFF;
    
    *i += 1;
    if *i == n {
        *i = 0;
    }
    
    *j += 1;
    if *j == n {
        *j = 0;
    }
    
    k
}

impl RandomGenerator for Random32State {
    fn get(&mut self) -> u32 {
        random_get(&mut self.i, &mut self.j, 7, &mut self.x)
    }

    fn set(&mut self, seed: u32) {
        bsd_initialize(&mut self.x, seed);
        self.i = 3;
        self.j = 0;
        for _ in 0..70 {
            self.get();
        }
    }
}

impl RandomGenerator for Random64State {
    fn get(&mut self) -> u32 {
        random_get(&mut self.i, &mut self.j, 15, &mut self.x)
    }

    fn set(&mut self, seed: u32) {
        bsd_initialize(&mut self.x, seed);
        self.i = 1;
        self.j = 0;
        for _ in 0..150 {
            self.get();
        }
    }
}

impl RandomGenerator for Random128State {
    fn get(&mut self) -> u32 {
        random_get(&mut self.i, &mut self.j, 31, &mut self.x)
    }

    fn set(&mut self, seed: u32) {
        bsd_initialize(&mut self.x, seed);
        self.i = 3;
        self.j = 0;
        for _ in 0..310 {
            self.get();
        }
    }
}

impl RandomGenerator for Random256State {
    fn get(&mut self) -> u32 {
        random_get(&mut self.i, &mut self.j, 63, &mut self.x)
    }

    fn set(&mut self, seed: u32) {
        bsd_initialize(&mut self.x, seed);
        self.i = 1;
        self.j = 0;
        for _ in 0..630 {
            self.get();
        }
    }
}

// Initialization functions
fn bsd_initialize(x: &mut [u32], mut s: u32) {
    if s == 0 {
        s = 1;
    }
    
    x[0] = s;
    for i in 1..x.len() {
        x[i] = ((Wrapping(1103515245) * Wrapping(x[i-1]) + Wrapping(12345)).0;
    }
}

fn libc5_initialize(x: &mut [u32], mut s: u32) {
    if s == 0 {
        s = 1;
    }
    
    x[0] = s;
    for i in 1..x.len() {
        x[i] = ((Wrapping(1103515145) * Wrapping(x[i-1])) + Wrapping(12345)).0;
    }
}

fn glibc2_initialize(x: &mut [u32], mut s: u32) {
    if s == 0 {
        s = 1;
    }
    
    x[0] = s;
    for i in 1..x.len() {
        let h = s / 127773;
        let t = (16807 * (s - h * 127773)).wrapping_sub(h * 2836);
        s = if t < 0 { t.wrapping_add(2147483647) } else { t };
        x[i] = s;
    }
}

// RNG types
pub struct RandomType {
    pub name: &'static str,
    pub max: u32,
    pub min: u32,
    pub state_size: usize,
    pub set_fn: fn(&mut dyn RandomGenerator, u32),
    pub get_fn: fn(&mut dyn RandomGenerator) -> u32,
    pub get_double_fn: fn(&mut dyn RandomGenerator) -> f64,
}

// RNG implementations
pub const RANDOM_GLIBC2: RandomType = RandomType {
    name: "random-glibc2",
    max: 0x7FFFFFFF,
    min: 0,
    state_size: std::mem::size_of::<Random128State>(),
    set_fn: |state, seed| {
        if let Some(s) = state.as_mut().downcast_mut::<Random128State>() {
            glibc2_initialize(&mut s.x, seed);
            s.i = 3;
            s.j = 0;
            for _ in 0..310 {
                s.get();
            }
        }
    },
    get_fn: |state| {
        state.as_mut().downcast_mut::<Random128State>().unwrap().get()
    },
    get_double_fn: |state| {
        state.as_mut().downcast_mut::<Random128State>().unwrap().get_double()
    },
};

pub const RANDOM8_GLIBC2: RandomType = RandomType {
    name: "random8-glibc2",
    max: 0x7FFFFFFF,
    min: 0,
    state_size: std::mem::size_of::<Random8State>(),
    set_fn: |state, seed| {
        state.as_mut().downcast_mut::<Random8State>().unwrap().set(seed);
    },
    get_fn: |state| {
        state.as_mut().downcast_mut::<Random8State>().unwrap().get()
    },
    get_double_fn: |state| {
        state.as_mut().downcast_mut::<Random8State>().unwrap().get_double()
    },
};

// Similar implementations for other variants (random32, random64, random256)
// and for libc5 and bsd versions would follow the same pattern

// Public API
pub struct GslRng {
    rng_type: &'static RandomType,
    state: Box<dyn RandomGenerator>,
}

impl GslRng {
    pub fn new(rng_type: &'static RandomType) -> Self {
        let state: Box<dyn RandomGenerator> = match rng_type.state_size {
            size if size == std::mem::size_of::<Random8State>() => Box::new(Random8State { x: 1 }),
            size if size == std::mem::size_of::<Random32State>() => Box::new(Random32State {
                i: 0,
                j: 0,
                x: [0; 7],
            }),
            size if size == std::mem::size_of::<Random64State>() => Box::new(Random64State {
                i: 0,
                j: 0,
                x: [0; 15],
            }),
            size if size == std::mem::size_of::<Random128State>() => Box::new(Random128State {
                i: 0,
                j: 0,
                x: [0; 31],
            }),
            size if size == std::mem::size_of::<Random256State>() => Box::new(Random256State {
                i: 0,
                j: 0,
                x: [0; 63],
            }),
            _ => panic!("Unknown state size"),
        };
        
        Self { rng_type, state }
    }

    pub fn set(&mut self, seed: u32) {
        (self.rng_type.set_fn)(&mut *self.state, seed);
    }

    pub fn get(&mut self) -> u32 {
        (self.rng_type.get_fn)(&mut *self.state)
    }

    pub fn get_double(&mut self) -> f64 {
        (self.rng_type.get_double_fn)(&mut *self.state)
    }
}