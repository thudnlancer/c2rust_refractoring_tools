use std::mem::size_of;

pub type SizeT = usize;
pub type Uint64T = u64;
pub type ZahlCharT = Uint64T;

#[derive(Debug, Clone)]
pub struct Zahl {
    pub sign: i32,
    pub used: SizeT,
    pub alloced: SizeT,
    pub chars: Vec<ZahlCharT>,
}

impl Zahl {
    pub fn new() -> Self {
        Zahl {
            sign: 0,
            used: 0,
            alloced: 0,
            chars: Vec::new(),
        }
    }

    pub fn is_zero(&self) -> bool {
        self.sign == 0
    }

    pub fn resize(&mut self, new_size: SizeT) {
        if new_size > self.alloced {
            self.chars.resize(new_size, 0);
            self.alloced = new_size;
        }
    }

    pub fn set_bit(&mut self, bit: SizeT) {
        let chars = bit / (size_of::<ZahlCharT>() * 8);
        let bit = bit % (size_of::<ZahlCharT>() * 8);

        if self.is_zero() {
            self.used = 0;
            self.sign = 1;
        }

        if chars >= self.used {
            self.resize(chars + 1);
            for i in self.used..=chars {
                self.chars[i] = 0;
            }
            self.used = chars + 1;
        }

        self.chars[chars] |= 1 << bit;
    }

    pub fn clear_bit(&mut self, bit: SizeT) {
        let chars = bit / (size_of::<ZahlCharT>() * 8);
        let bit = bit % (size_of::<ZahlCharT>() * 8);

        if chars >= self.used {
            return;
        }

        self.chars[chars] &= !(1 << bit);

        while self.used > 0 && self.chars[self.used - 1] == 0 {
            self.used -= 1;
        }

        if self.used == 0 {
            self.sign = 0;
        }
    }

    pub fn flip_bit(&mut self, bit: SizeT) {
        let chars = bit / (size_of::<ZahlCharT>() * 8);
        let bit = bit % (size_of::<ZahlCharT>() * 8);

        if self.is_zero() {
            self.used = 0;
            self.sign = 1;
        }

        if chars >= self.used {
            self.resize(chars + 1);
            for i in self.used..=chars {
                self.chars[i] = 0;
            }
            self.used = chars + 1;
        }

        self.chars[chars] ^= 1 << bit;

        while self.used > 0 && self.chars[self.used - 1] == 0 {
            self.used -= 1;
        }

        if self.used == 0 {
            self.sign = 0;
        }
    }
}