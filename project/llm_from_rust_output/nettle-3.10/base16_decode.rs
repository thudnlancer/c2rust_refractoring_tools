use std::convert::TryFrom;

#[derive(Copy, Clone, Default)]
pub struct Base16DecodeCtx {
    word: u8,
    bits: u8,
}

impl Base16DecodeCtx {
    pub fn init(&mut self) {
        self.bits = 0;
        self.word = self.bits;
    }

    pub fn decode_single(&mut self, dst: &mut u8, src: char) -> Result<Option<()>, ()> {
        const HEX_DECODE_TABLE: [i8; 128] = [
            -1, -1, -1, -1, -1, -1, -1, -1, -1, -2, -2, -1, -1, -2, -1, -1, -1, -1, -1, -1, -1,
            -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
            -1, -1, -1, -1, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, -1, -1, -1, -1, -1, -1, -1, 10, 11,
            12, 13, 14, 15, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
            -1, -1, -1, -1, -1, -1, -1, 10, 11, 12, 13, 14, 15, -1, -1, -1, -1, -1, -1, -1, -1, -1,
            -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
        ];

        let usrc = src as usize;
        if usrc >= 128 {
            return Err(());
        }

        let digit = HEX_DECODE_TABLE[usrc];
        match digit {
            -1 => Err(()),
            -2 => Ok(None),
            d if d >= 0 && d < 16 => {
                let digit = d as u8;
                if self.bits != 0 {
                    *dst = (self.word << 4) | digit;
                    self.bits = 0;
                    Ok(Some(()))
                } else {
                    self.word = digit;
                    self.bits = 4;
                    Ok(None)
                }
            }
            _ => unreachable!(),
        }
    }

    pub fn decode_update(
        &mut self,
        dst: &mut [u8],
        src: &str,
    ) -> Result<usize, ()> {
        let mut done = 0;
        for c in src.chars() {
            match self.decode_single(
                dst.get_mut(done).ok_or(())?,
                c,
            )? {
                Some(_) => done += 1,
                None => (),
            }
        }
        Ok(done)
    }

    pub fn decode_final(&self) -> bool {
        self.bits == 0
    }
}

pub fn base16_decode_init(ctx: &mut Base16DecodeCtx) {
    ctx.init();
}

pub fn base16_decode_single(ctx: &mut Base16DecodeCtx, dst: &mut u8, src: char) -> Result<Option<()>, ()> {
    ctx.decode_single(dst, src)
}

pub fn base16_decode_update(
    ctx: &mut Base16DecodeCtx,
    dst: &mut [u8],
    src: &str,
) -> Result<usize, ()> {
    ctx.decode_update(dst, src)
}

pub fn base16_decode_final(ctx: &Base16DecodeCtx) -> bool {
    ctx.decode_final()
}