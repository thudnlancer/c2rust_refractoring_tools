//! TT800 random number generator implementation in Rust.
//!
//! This is a translation of the TT800 twisted GSFR generator from C to Rust.
//! The original C code was part of the GNU Scientific Library (GSL).
//!
//! The period is 2^800. This implementation is based on tt800.c, July 8th 1996 version
//! by M. Matsumoto, email: matumoto@math.keio.ac.jp
//!
//! From: Makoto Matsumoto and Yoshiharu Kurita, "Twisted GFSR Generators II",
//! ACM Transactions on Modelling and Computer Simulation, Vol. 4, No. 3, 1994, pages 254-266.

const N: usize = 25;
const M: usize = 7;

/// State for the TT800 random number generator
#[derive(Clone)]
pub struct TtState {
    n: usize,
    x: [u32; N],
}

impl TtState {
    /// Creates a new TT800 generator with the given seed
    pub fn new(seed: u32) -> Self {
        if seed == 0 {
            return Self::default();
        }

        let mut state = TtState {
            n: 0,
            x: [0; N],
        };

        state.x[0] = seed;
        for i in 1..N {
            state.x[i] = (69069_u32.wrapping_mul(state.x[i - 1])) & 0xffff_ffff;
        }

        state
    }

    /// Generates the next random number in the sequence
    pub fn get(&mut self) -> u32 {
        const MAG01: [u32; 2] = [0x0000_0000, 0x8ebf_d028];
        
        if self.n >= N {
            let mut i = 0;
            while i < N - M {
                self.x[i] = self.x[i + M] ^ (self.x[i] >> 1) ^ MAG01[(self.x[i] % 2) as usize];
                i += 1;
            }
            while i < N {
                self.x[i] = self.x[i + (M - N)] ^ (self.x[i] >> 1) ^ MAG01[(self.x[i] % 2) as usize];
                i += 1;
            }
            self.n = 0;
        }

        let mut y = self.x[self.n];
        y ^= (y << 7) & 0x2b5b_2500;   // s and b, magic vectors
        y ^= (y << 15) & 0xdb8b_0000;   // t and c, magic vectors
        y &= 0xffff_ffff;               // ensure 32-bit
        
        // Improvement added in 1996 version
        y ^= y >> 16;
        
        self.n += 1;
        y
    }

    /// Generates a random double in [0,1)
    pub fn get_double(&mut self) -> f64 {
        self.get() as f64 / 4294967296.0
    }
}

impl Default for TtState {
    fn default() -> Self {
        TtState {
            n: 0,
            x: [
                0x95f2_4dab, 0x0b68_5215, 0xe76c_cae7,
                0xaf3e_c239, 0x715f_ad23, 0x24a5_90ad,
                0x69e4_b5ef, 0xbf45_6141, 0x96bc_1b7b,
                0xa7bd_f825, 0xc1de_75b7, 0x8858_a9c9,
                0x2da8_7693, 0xb657_f9dd, 0xffdc_8a9f,
                0x8121_da71, 0x8b82_3ecb, 0x885d_05f5,
                0x4e20_cd47, 0x5a9a_d5d9, 0x512c_0c03,
                0xea85_7ccd, 0x4cc1_d30f, 0x8891_a8a1,
                0xa6b7_aadb,
            ],
        }
    }
}

/// TT800 random number generator type
pub struct Tt800;

impl Tt800 {
    /// Creates a new TT800 random number generator
    pub fn new(seed: u32) -> TtState {
        TtState::new(seed)
    }

    /// Maximum value that can be generated
    pub const fn max_value() -> u32 {
        0xffff_ffff
    }

    /// Minimum value that can be generated
    pub const fn min_value() -> u32 {
        0
    }
}