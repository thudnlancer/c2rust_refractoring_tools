use std::mem;
use std::ptr;

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
    fn new() -> Self {
        Zahl {
            sign: 0,
            used: 0,
            alloced: 0,
            chars: Vec::new(),
        }
    }

    fn is_zero(&self) -> bool {
        self.sign == 0
    }

    fn realloc(&mut self, new_size: SizeT) {
        self.chars.resize(new_size, 0);
        self.alloced = new_size;
    }

    fn truncate(&mut self, other: &mut Zahl, bits: SizeT) {
        if other.is_zero() {
            self.sign = 0;
            return;
        }

        let chars = (bits + 63) >> 6;
        self.used = if chars < other.used { chars } else { other.used };

        let mut remaining_bits = if self.used < chars { 0 } else { bits };

        if !ptr::eq(self, other) {
            self.sign = other.sign;
            if self.alloced < self.used {
                self.realloc(self.used);
            }
            self.chars[..self.used].copy_from_slice(&other.chars[..self.used]);
        }

        remaining_bits &= 63;
        if remaining_bits != 0 {
            let last = self.used - 1;
            self.chars[last] &= (1 << remaining_bits) - 1;
        }

        while self.used != 0 && self.chars[self.used - 1] == 0 {
            self.used -= 1;
        }

        if self.used == 0 {
            self.sign = 0;
        }
    }
}

fn libzahl_memcpy(d: &mut [ZahlCharT], s: &[ZahlCharT], n: SizeT) {
    if n <= 20 {
        d[..n].copy_from_slice(&s[..n]);
    } else {
        let chunks = n / 4;
        let remainder = n % 4;
        
        for i in 0..chunks {
            let offset = i * 4;
            d[offset..offset+4].copy_from_slice(&s[offset..offset+4]);
        }
        
        if remainder > 0 {
            let offset = chunks * 4;
            d[offset..offset+remainder].copy_from_slice(&s[offset..offset+remainder]);
        }
    }
}