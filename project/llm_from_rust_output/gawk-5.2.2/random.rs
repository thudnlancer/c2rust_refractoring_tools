use std::io::{self, Write};

type GawkUint32 = u32;
type GawkInt32 = i32;

const DEGREES: [i32; 5] = [0, 7, 15, 31, 63];
const SEPS: [i32; 5] = [0, 3, 1, 3, 1];
const RANDTBL: [GawkUint32; 32] = [
    3,
    0x991539b1,
    0x16a5bce3,
    0x6774a4cd,
    0x3e01511e,
    0x4e508aaa,
    0x61048c05,
    0xf5500617,
    0x846b7115,
    0x6a19892c,
    0x896a97af,
    0xdb48f936,
    0x14898454,
    0x37ffd106,
    0xb58bff9c,
    0x59e17104,
    0xcf918a49,
    0x09378c83,
    0x52c7a471,
    0x8d293ea9,
    0x1f4fc301,
    0xc3db71be,
    0x39b44e1c,
    0xf8a44ef9,
    0x4c8b80b1,
    0x19edc328,
    0x87bf4bdd,
    0xc9b240e5,
    0xe9ee4b1b,
    0x4382aee7,
    0x535b6b41,
    0xf3bec5da,
];

struct RandomState {
    rand_type: i32,
    rand_deg: i32,
    rand_sep: i32,
    state: Vec<GawkUint32>,
    fptr: usize,
    rptr: usize,
    end_ptr: usize,
    shuffle_init: bool,
    shuffle_buffer: [i64; 512],
    s: i64,
}

impl RandomState {
    fn new() -> Self {
        let mut state = RandomState {
            rand_type: 3,
            rand_deg: 31,
            rand_sep: 3,
            state: RANDTBL[1..].to_vec(),
            fptr: 3 + 1,
            rptr: 1,
            end_ptr: 31 + 1,
            shuffle_init: true,
            shuffle_buffer: [0; 512],
            s: 0xcafefeed,
        };
        state.state.insert(0, 0);
        state
    }

    fn good_rand(&self, mut x: GawkInt32) -> GawkUint32 {
        if x == 0 {
            x = 123459876;
        }
        let hi = x / 127773;
        let lo = x % 127773;
        x = 16807 * lo - 2836 * hi;
        if x < 0 {
            x += 0x7fffffff;
        }
        x as GawkUint32
    }

    fn srandom(&mut self, seed: u64) {
        self.shuffle_init = true;
        self.state[0] = seed as GawkUint32;

        if self.rand_type == 0 {
            for _ in 0..50 {
                self.random_old();
            }
        } else {
            for i in 1..self.rand_deg {
                self.state[i as usize] = self.good_rand(self.state[(i - 1) as usize] as GawkInt32);
            }
            self.fptr = self.rand_sep as usize;
            self.rptr = 0;

            for _ in 0..(10 * self.rand_deg) {
                self.random_old();
            }
        }
    }

    fn init_state(&mut self, seed: u64, arg_state: &mut [GawkUint32]) -> io::Result<()> {
        if arg_state.len() < 8 {
            writeln!(
                io::stderr(),
                "random: not enough state ({} bytes); ignored.",
                arg_state.len()
            )?;
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Not enough state",
            ));
        }

        if arg_state.len() < 32 {
            self.rand_type = 0;
            self.rand_deg = 0;
            self.rand_sep = 0;
        } else if arg_state.len() < 64 {
            self.rand_type = 1;
            self.rand_deg = 7;
            self.rand_sep = 3;
        } else if arg_state.len() < 128 {
            self.rand_type = 2;
            self.rand_deg = 15;
            self.rand_sep = 1;
        } else if arg_state.len() < 256 {
            self.rand_type = 3;
            self.rand_deg = 31;
            self.rand_sep = 3;
        } else {
            self.rand_type = 4;
            self.rand_deg = 63;
            self.rand_sep = 1;
        }

        self.state = arg_state[1..].to_vec();
        self.state.insert(0, 0);
        self.end_ptr = self.rand_deg as usize;
        self.srandom(seed);

        if self.rand_type == 0 {
            arg_state[0] = self.rand_type as GawkUint32;
        } else {
            arg_state[0] = (5 * (self.rptr as i64) + self.rand_type as i64) as GawkUint32;
        }

        Ok(())
    }

    fn set_state(&mut self, arg_state: &[GawkUint32]) -> io::Result<()> {
        let type_ = arg_state[0] % 5;
        let rear = arg_state[0] / 5;

        match type_ {
            0 | 1 | 2 | 3 | 4 => {
                self.rand_type = type_ as i32;
                self.rand_deg = DEGREES[type_ as usize];
                self.rand_sep = SEPS[type_ as usize];
            }
            _ => {
                writeln!(io::stderr(), "random: state info corrupted; not changed.")?;
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Corrupted state info",
                ));
            }
        }

        self.state = arg_state[1..].to_vec();
        self.state.insert(0, 0);

        if self.rand_type != 0 {
            self.rptr = rear as usize;
            self.fptr = (rear + self.rand_sep as GawkUint32) % self.rand_deg as GawkUint32;
            self.fptr = self.fptr as usize;
        }

        self.end_ptr = self.rand_deg as usize;
        Ok(())
    }

    fn random_old(&mut self) -> i64 {
        if self.rand_type == 0 {
            let i = self.good_rand(self.state[0] as GawkInt32) & 0x7fffffff;
            self.state[0] = i;
            i as i64
        } else {
            let f = self.fptr;
            let r = self.rptr;

            self.state[f] = self.state[f].wrapping_add(self.state[r]);
            let i = (self.state[f] >> 1) & 0x7fffffff;

            self.fptr = if f + 1 >= self.end_ptr {
                0
            } else {
                f + 1
            };

            self.rptr = if r + 1 >= self.end_ptr {
                0
            } else {
                r + 1
            };

            i as i64
        }
    }

    fn random(&mut self) -> i64 {
        if self.shuffle_init {
            for k in 0..(1 << 9) {
                self.shuffle_buffer[k] = self.random_old();
            }
            self.s = self.random_old();
            self.shuffle_init = false;
        }

        let r = self.random_old();
        let k = (self.s & ((1 << 9) - 1) as i64) as usize;
        self.s = self.shuffle_buffer[k];
        self.shuffle_buffer[k] = r;
        self.s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_good_rand() {
        let state = RandomState::new();
        assert_ne!(state.good_rand(0), 0);
        assert_ne!(state.good_rand(123), 0);
    }

    #[test]
    fn test_random() {
        let mut state = RandomState::new();
        let r1 = state.random();
        let r2 = state.random();
        assert_ne!(r1, r2);
    }
}