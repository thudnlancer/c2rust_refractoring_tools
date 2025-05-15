use std::convert::TryFrom;

const BASE64_DECODE_TABLE: [i8; 256] = [
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -2, -2, -2, -2, -2, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 62, -1, -1, -1, 63,
    52, 53, 54, 55, 56, 57, 58, 59, 60, 61, -1, -1, -1, -3, -1, -1,
    -1,  0,  1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14,
    15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, -1, -1, -1, -1, -1,
    -1, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40,
    41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
];

#[derive(Debug, Clone, Copy)]
pub struct Base64DecodeCtx {
    table: &'static [i8; 256],
    word: u16,
    bits: u8,
    padding: u8,
}

impl Base64DecodeCtx {
    pub fn new() -> Self {
        Self {
            table: &BASE64_DECODE_TABLE,
            word: 0,
            bits: 0,
            padding: 0,
        }
    }

    pub fn decode_single(&mut self, dst: &mut u8, src: char) -> Result<Option<()>, ()> {
        let src = src as u8;
        let data = self.table[src as usize];
        
        match data {
            -1 => Err(()),
            -2 => Ok(None),
            -3 => {
                if self.bits == 0 || self.padding > 2 {
                    return Err(());
                }
                if self.word & ((1 << self.bits) - 1) != 0 {
                    return Err(());
                }
                self.padding += 1;
                self.bits = self.bits.saturating_sub(2);
                Ok(None)
            }
            data if data >= 0 && data < 0x40 => {
                if self.padding != 0 {
                    return Err(());
                }
                self.word = (self.word << 6) | data as u16;
                self.bits += 6;
                
                if self.bits >= 8 {
                    self.bits -= 8;
                    *dst = (self.word >> self.bits) as u8;
                    Ok(Some(()))
                } else {
                    Ok(None)
                }
            }
            _ => unreachable!("Invalid base64 data"),
        }
    }

    pub fn decode_update(
        &mut self,
        dst: &mut [u8],
        src: &[u8],
    ) -> Result<usize, ()> {
        let mut done = 0;
        
        for &c in src {
            match self.decode_single(&mut dst[done], c as char) {
                Ok(Some(())) => done += 1,
                Ok(None) => (),
                Err(()) => return Err(()),
            }
        }
        
        let max_len = (src.len() + 1) * 6 / 8;
        assert!(done <= max_len, "Output length exceeds maximum possible");
        
        Ok(done)
    }

    pub fn decode_final(&self) -> bool {
        self.bits == 0
    }
}

pub fn base64_decode_init(ctx: &mut Base64DecodeCtx) {
    *ctx = Base64DecodeCtx::new();
}

pub fn base64_decode_update(
    ctx: &mut Base64DecodeCtx,
    dst: &mut [u8],
    src: &[u8],
) -> Result<usize, ()> {
    ctx.decode_update(dst, src)
}

pub fn base64_decode_final(ctx: &Base64DecodeCtx) -> bool {
    ctx.decode_final()
}