use std::mem;
use std::ptr;

type SizeT = usize;
type ZahlCharT = u64;

#[derive(Clone, Copy)]
pub struct Zahl {
    pub sign: i32,
    padding__: i32,
    pub used: SizeT,
    pub alloced: SizeT,
    chars: *mut ZahlCharT,
}

impl Zahl {
    pub fn zero(&mut self) {
        self.sign = 0;
    }

    pub fn signum(&self) -> i32 {
        self.sign
    }

    pub fn is_zero(&self) -> bool {
        self.sign == 0
    }

    pub fn realloc(&mut self, new_size: SizeT) {
        unsafe {
            let new_ptr = libc::realloc(
                self.chars as *mut libc::c_void,
                new_size * mem::size_of::<ZahlCharT>(),
            );
            self.chars = new_ptr as *mut ZahlCharT;
            self.alloced = new_size;
        }
    }

    fn memcpy(dest: &mut [ZahlCharT], src: &[ZahlCharT]) {
        dest.copy_from_slice(src);
    }

    fn memset(arr: &mut [ZahlCharT], value: ZahlCharT) {
        for elem in arr.iter_mut() {
            *elem = value;
        }
    }

    fn memmove(dest: &mut [ZahlCharT], src: &[ZahlCharT]) {
        if src.as_ptr() < dest.as_ptr() {
            for i in (0..src.len()).rev() {
                dest[i] = src[i];
            }
        } else {
            dest.copy_from_slice(src);
        }
    }

    pub fn lsh(&mut self, b: &Zahl, bits: SizeT) {
        if b.is_zero() {
            self.zero();
            return;
        }

        let chars = bits >> 6;
        let bits = bits & (64 - 1);
        let cbits = 64 - bits;

        let required_size = b.used + chars + 1;
        if self.alloced < required_size {
            self.realloc(required_size);
        }

        unsafe {
            let self_chars = std::slice::from_raw_parts_mut(self.chars, self.alloced);
            let b_chars = std::slice::from_raw_parts(b.chars, b.used);

            if ptr::eq(self, b) {
                Self::memmove(
                    &mut self_chars[chars..chars + b.used],
                    &b_chars[..b.used],
                );
            } else {
                Self::memcpy(
                    &mut self_chars[chars..chars + b.used],
                    &b_chars[..b.used],
                );
            }

            Self::memset(&mut self_chars[..chars], 0);
            self.used = b.used + chars;

            if bits != 0 {
                let mut carry = 0;
                for i in chars..self.used {
                    let tcarry = self_chars[i] >> cbits;
                    self_chars[i] <<= bits;
                    self_chars[i] |= carry;
                    carry = tcarry;
                }

                if carry != 0 {
                    self_chars[self.used] = carry;
                    self.used += 1;
                }
            }
        }

        self.sign = b.signum();
    }
}