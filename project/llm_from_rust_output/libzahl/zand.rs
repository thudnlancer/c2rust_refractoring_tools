use std::cmp::Ordering;

type SizeT = usize;
type Uint64T = u64;
type ZahlCharT = Uint64T;

#[derive(Clone)]
pub struct Zahl {
    pub sign: i32,
    pub used: SizeT,
    pub alloced: SizeT,
    pub chars: Vec<ZahlCharT>,
}

impl Zahl {
    fn zero(&self) -> bool {
        self.sign == 0
    }

    fn signum(&self) -> i32 {
        self.sign
    }

    pub fn zand(&mut self, b: &Zahl, c: &Zahl) {
        if b.zero() || c.zero() {
            self.sign = 0;
            return;
        }

        self.used = b.used.min(c.used);
        self.chars.resize(self.used, 0);

        if std::ptr::eq(self, b) {
            for i in 0..self.used {
                self.chars[i] &= c.chars[i];
            }
        } else if std::ptr::eq(self, c) {
            for i in 0..self.used {
                self.chars[i] &= b.chars[i];
            }
        } else {
            for i in 0..self.used {
                self.chars[i] = b.chars[i] & c.chars[i];
            }
        }

        while self.used > 0 && self.chars[self.used - 1] == 0 {
            self.used -= 1;
        }

        self.sign = if self.used != 0 {
            let b_pos = b.signum() > 0;
            let c_pos = c.signum() > 0;
            match (b_pos, c_pos) {
                (true, true) => 1,
                (false, false) => -1,
                _ => 0,
            }
        } else {
            0
        };
    }
}