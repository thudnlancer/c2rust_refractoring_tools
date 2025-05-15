use std::convert::TryFrom;

const TABLE_INVALID: i8 = -1;
const TABLE_SPACE: i8 = -2;
const TABLE_END: i8 = -3;

struct Base64DecodeCtx {
    word: u32,
    bits: u32,
    padding: u32,
    table: [i8; 256],
}

impl Base64DecodeCtx {
    fn new() -> Self {
        let table = [
            // White space is HT, VT, FF, CR, LF and SPC
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
        
        Self {
            word: 0,
            bits: 0,
            padding: 0,
            table,
        }
    }

    fn decode_single(&mut self, dst: &mut [u8], src: u8) -> Result<Option<u8>, ()> {
        let data = self.table[src as usize];

        match data {
            x if x >= 0 && x < 0x40 => {
                if self.padding > 0 {
                    return Err(());
                }
                
                self.word = (self.word << 6) | (x as u32);
                self.bits += 6;

                if self.bits >= 8 {
                    self.bits -= 8;
                    let byte = (self.word >> self.bits) as u8;
                    Ok(Some(byte))
                } else {
                    Ok(None)
                }
            }
            TABLE_INVALID => Err(()),
            TABLE_SPACE => Ok(None),
            TABLE_END => {
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
            _ => unreachable!(),
        }
    }

    fn decode_update(
        &mut self,
        dst: &mut [u8],
        src: &[u8],
    ) -> Result<usize, ()> {
        let mut done = 0;

        for &byte in src {
            match self.decode_single(&mut dst[done..], byte)? {
                Some(_) => {
                    done += 1;
                }
                None => {}
            }
        }

        Ok(done)
    }

    fn decode_final(&self) -> bool {
        self.bits == 0
    }
}

fn base64_decode_length(src_length: usize) -> usize {
    (src_length * 3 + 3) / 4
}

pub fn base64_decode(
    input: &[u8],
    output: &mut [u8],
) -> Result<usize, ()> {
    let mut ctx = Base64DecodeCtx::new();
    let decoded = ctx.decode_update(output, input)?;
    if !ctx.decode_final() {
        return Err(());
    }
    Ok(decoded)
}