use std::mem;

type SizeT = usize;
type Uint64T = u64;
type ZahlCharT = Uint64T;

#[derive(Debug, Clone)]
pub struct Zahl {
    pub sign: i32,
    pub used: SizeT,
    pub alloced: SizeT,
    pub chars: Vec<ZahlCharT>,
}

impl Zahl {
    pub fn zero() -> Self {
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

    pub fn signum(&self) -> i32 {
        self.sign
    }

    pub fn bits(&mut self) -> SizeT {
        if self.is_zero() {
            return 1;
        }

        while self.used > 0 && self.chars[self.used - 1] == 0 {
            self.used -= 1;
        }

        let mut rc = self.used * 8 * mem::size_of::<ZahlCharT>();
        if self.used > 0 {
            rc -= self.chars[self.used - 1].leading_zeros() as SizeT;
        }
        rc
    }

    pub fn not(&mut self, other: &mut Zahl) {
        if other.is_zero() {
            self.sign = 0;
            return;
        }

        let bits = other.bits();
        self.used = other.used;
        self.sign = -other.signum();

        self.chars = other.chars.iter().map(|&x| !x).collect();

        let bits = bits & (64 - 1);
        if bits != 0 && !self.chars.is_empty() {
            let last = self.used - 1;
            self.chars[last] &= (1 << bits) - 1;
        }

        while self.used > 0 && self.chars[self.used - 1] == 0 {
            self.used -= 1;
        }

        if self.used == 0 {
            self.sign = 0;
        }
    }
}