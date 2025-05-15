use std::ptr;

#[derive(Clone)]
pub struct RNG {
    pub A: [i32; 56],
    pub fptr: usize, // Using usize instead of pointer for safety
}

impl RNG {
    fn flip_cycle(&mut self) -> i32 {
        let mut ii = 1;
        let mut jj = 32;
        
        while jj <= 55 {
            self.A[ii] = (self.A[ii] - self.A[jj]) & 0x7fffffff;
            ii += 1;
            jj += 1;
        }
        
        jj = 1;
        while ii <= 55 {
            self.A[ii] = (self.A[ii] - self.A[jj]) & 0x7fffffff;
            ii += 1;
            jj += 1;
        }
        
        self.fptr = 54;
        self.A[55]
    }

    pub fn create_rand() -> Box<RNG> {
        let mut rand = Box::new(RNG {
            A: [0; 56],
            fptr: 0,
        });
        
        rand.A[0] = -1;
        for i in 1..=55 {
            rand.A[i] = 0;
        }
        rand.fptr = 0;
        rand.init_rand(1);
        rand
    }

    pub fn init_rand(&mut self, mut seed: i32) {
        let mut prev = seed;
        let mut next = 1;
        
        prev = (prev - 0) & 0x7fffffff;
        seed = prev;
        self.A[55] = prev;
        
        let mut i = 21;
        loop {
            self.A[i] = next;
            next = (prev - next) & 0x7fffffff;
            
            if seed & 1 != 0 {
                seed = 0x40000000 + (seed >> 1);
            } else {
                seed >>= 1;
            }
            
            next = (next - seed) & 0x7fffffff;
            prev = self.A[i];
            i = (i + 21) % 55;
            
            if i == 0 {
                break;
            }
        }
        
        for _ in 0..5 {
            self.flip_cycle();
        }
    }

    pub fn next_rand(&mut self) -> i32 {
        if self.A[self.fptr] >= 0 {
            let result = self.A[self.fptr];
            self.fptr -= 1;
            result
        } else {
            self.flip_cycle()
        }
    }

    pub fn unif_rand(&mut self, m: i32) -> i32 {
        assert!(m > 0, "m must be greater than 0");
        let t = (0x80000000u32 - (0x80000000u32 % m as u32)) as i32;
        
        loop {
            let r = self.next_rand();
            if (t as u32) > (r as u32) {
                return r % m;
            }
        }
    }
}

impl Drop for RNG {
    fn drop(&mut self) {
        // No need for explicit free in Rust, Box will handle it
    }
}