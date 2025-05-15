use std::mem::size_of;
use std::ops::{Shr, Shl, BitOrAssign};

type SizeT = usize;
type ZahlCharT = u64;

#[derive(Clone)]
struct Zahl {
    sign: i32,
    used: SizeT,
    alloced: SizeT,
    chars: Vec<ZahlCharT>,
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

    fn signum(&self) -> i32 {
        self.sign
    }

    fn bits(&mut self) -> SizeT {
        if self.is_zero() {
            return 1;
        }

        while self.chars[self.used - 1] == 0 {
            self.used -= 1;
        }

        let mut rc = self.used * 8 * size_of::<ZahlCharT>();
        rc -= self.chars[self.used - 1].leading_zeros() as SizeT;
        rc
    }

    fn set(&mut self, other: &Zahl) {
        if other.is_zero() {
            self.sign = 0;
        } else {
            self.sign = other.sign;
            self.used = other.used;
            if self.alloced < other.used {
                self.chars.resize(other.used, 0);
                self.alloced = other.used;
            }
            self.chars[..other.used].copy_from_slice(&other.chars[..other.used]);
        }
    }
}

fn memcpy(dest: &mut [ZahlCharT], src: &[ZahlCharT], n: SizeT) {
    dest[..n].copy_from_slice(&src[..n]);
}

fn memmovef(dest: &mut [ZahlCharT], src: &[ZahlCharT], n: SizeT) {
    if n != 0 && n < 4 {
        dest[0] = src[0];
        if n > 1 {
            dest[1] = src[1];
            if n > 2 {
                dest[2] = src[2];
            }
        }
    } else {
        for i in (0..n).step_by(4) {
            let end = i + 4;
            if end > n {
                break;
            }
            dest[i] = src[i];
            dest[i + 1] = src[i + 1];
            dest[i + 2] = src[i + 2];
            dest[i + 3] = src[i + 3];
        }
    }
}

fn memmoveb(dest: &mut [ZahlCharT], src: &[ZahlCharT], n: SizeT) {
    if n <= 20 {
        match n {
            20 => dest[19] = src[19],
            19 => dest[18] = src[18],
            18 => dest[17] = src[17],
            17 => dest[16] = src[16],
            16 => dest[15] = src[15],
            15 => dest[14] = src[14],
            14 => dest[13] = src[13],
            13 => dest[12] = src[12],
            12 => dest[11] = src[11],
            11 => dest[10] = src[10],
            10 => dest[9] = src[9],
            9 => dest[8] = src[8],
            8 => dest[7] = src[7],
            7 => dest[6] = src[6],
            6 => dest[5] = src[5],
            5 => dest[4] = src[4],
            4 => dest[3] = src[3],
            3 => dest[2] = src[2],
            2 => dest[1] = src[1],
            1 => dest[0] = src[0],
            _ => (),
        }
    } else {
        let mut i = (n + 3) & !3;
        while i > 0 {
            i -= 4;
            dest[i] = src[i];
            dest[i + 1] = src[i + 1];
            dest[i + 2] = src[i + 2];
            dest[i + 3] = src[i + 3];
        }
    }
}

fn memmove(dest: &mut [ZahlCharT], src: &[ZahlCharT], n: SizeT) {
    if dest.as_ptr() < src.as_ptr() {
        memmovef(dest, src, n);
    } else {
        memmoveb(dest, src, n);
    }
}

pub fn zrsh(a: &mut Zahl, b: &Zahl, bits: SizeT) {
    if bits == 0 {
        if !std::ptr::eq(a, b) {
            a.set(b);
        }
        return;
    }

    let chars = bits >> 6;
    if b.is_zero() || chars >= b.used || b.bits() <= bits {
        a.sign = 0;
        return;
    }

    let bits = bits & (64 - 1);
    let cbits = 64 - bits;

    if chars != 0 && std::ptr::eq(a, b) {
        a.used -= chars;
        memmove(&mut a.chars[..a.used], &a.chars[chars..chars + a.used], a.used);
    } else if !std::ptr::eq(a, b) {
        a.used = b.used - chars;
        if a.alloced < a.used {
            a.chars.resize(a.used, 0);
            a.alloced = a.used;
        }
        memcpy(&mut a.chars[..a.used], &b.chars[chars..chars + a.used], a.used);
    }

    if bits != 0 {
        a.chars[0] >>= bits;
        for i in 1..a.used {
            a.chars[i - 1] |= a.chars[i] << cbits;
            a.chars[i] >>= bits;
        }

        while a.chars[a.used - 1] == 0 {
            a.used -= 1;
        }
    }

    a.sign = b.signum();
}