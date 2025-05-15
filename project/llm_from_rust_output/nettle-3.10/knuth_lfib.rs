#[derive(Clone, Copy)]
pub struct KnuthLfibCtx {
    x: [u32; 100],
    index: u32,
}

impl KnuthLfibCtx {
    pub fn new(seed: u32) -> Self {
        let mut ctx = KnuthLfibCtx {
            x: [0; 100],
            index: 0,
        };
        ctx.init(seed);
        ctx
    }

    fn init(&mut self, seed: u32) {
        let mut t: u32;
        let mut j: u32;
        let mut x: [u32; 199] = [0; 199];
        let mut ss = (seed.wrapping_add(2) as u64 & ((1 << 30) - 2)) as u32;

        j = 0;
        while j < 100 {
            x[j as usize] = ss;
            ss <<= 1;
            if ss as u64 >= (1 << 30) {
                ss = (ss as u64).wrapping_sub((1 << 30) - 2) as u32;
            }
            j = j.wrapping_add(1);
        }

        while j < (2 * 100 - 1) as u32 {
            x[j as usize] = 0;
            j = j.wrapping_add(1);
        }

        x[1] = x[1].wrapping_add(1);
        ss = (seed as u64 & ((1 << 30) - 1)) as u32;
        t = (70 - 1) as u32;

        while t != 0 {
            j = (100 - 1) as u32;
            while j > 0 {
                x[(j + j) as usize] = x[j as usize];
                j = j.wrapping_sub(1);
            }

            j = (2 * 100 - 2) as u32;
            while j > (100 - 37) as u32 {
                x[(2 * 100 - 1 - j) as usize] = x[j as usize] & !1;
                j = j.wrapping_sub(2);
            }

            j = (2 * 100 - 2) as u32;
            while j >= 100 {
                if x[j as usize] & 1 != 0 {
                    x[(j - (100 - 37)) as usize] = (x[(j - (100 - 37)) as usize]
                        .wrapping_sub(x[j as usize]))
                        & ((1 << 30) - 1);
                    x[(j - 100) as usize] = (x[(j - 100) as usize].wrapping_sub(x[j as usize]))
                        & ((1 << 30) - 1);
                }
                j = j.wrapping_sub(1);
            }

            if ss & 1 != 0 {
                j = 100;
                while j > 0 {
                    x[j as usize] = x[(j - 1) as usize];
                    j = j.wrapping_sub(1);
                }
                x[0] = x[100];
                if x[100] & 1 != 0 {
                    x[37] = (x[37].wrapping_sub(x[100])) & ((1 << 30) - 1);
                }
            }

            if ss != 0 {
                ss >>= 1;
            } else {
                t = t.wrapping_sub(1);
            }
        }

        j = 0;
        while j < 37 {
            self.x[(j + 100 - 37) as usize] = x[j as usize];
            j = j.wrapping_add(1);
        }

        while j < 100 {
            self.x[(j - 37) as usize] = x[j as usize];
            j = j.wrapping_add(1);
        }
    }

    pub fn get(&mut self) -> u32 {
        assert!(self.index < 100, "ctx->index < KK");

        let value = self.x[self.index as usize];
        self.x[self.index as usize] = (self.x[self.index as usize]
            .wrapping_sub(
                self.x[(self.index.wrapping_add(100).wrapping_sub(37) % 100 as usize],
            ))
            & ((1 << 30) - 1);
        self.index = (self.index + 1) % 100;
        value
    }

    pub fn get_array(&mut self, a: &mut [u32]) {
        for item in a.iter_mut() {
            *item = self.get();
        }
    }

    pub fn random(&mut self, dst: &mut [u8]) {
        let mut n = dst.len();
        let mut i = 0;

        while n >= 3 {
            let mut value = self.get();
            value ^= value >> 24;
            dst[i] = (value >> 16 & 0xff) as u8;
            dst[i + 1] = (value >> 8 & 0xff) as u8;
            dst[i + 2] = (value & 0xff) as u8;
            n -= 3;
            i += 3;
        }

        if n != 0 {
            let value = self.get();
            match n {
                1 => dst[i] = (value & 0xff) as u8,
                2 => {
                    dst[i] = (value >> 8 & 0xff) as u8;
                    dst[i + 1] = (value & 0xff) as u8;
                }
                _ => unreachable!(),
            }
        }
    }
}