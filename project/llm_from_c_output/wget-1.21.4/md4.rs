use std::convert::TryInto;
use std::io::{Read, Result as IoResult};
use std::mem;

pub const MD4_DIGEST_SIZE: usize = 16;

#[derive(Clone, Default)]
pub struct Md4Ctx {
    a: u32,
    b: u32,
    c: u32,
    d: u32,
    total: [u32; 2],
    buflen: usize,
    buffer: [u32; 32],
}

impl Md4Ctx {
    pub fn new() -> Self {
        let mut ctx = Self::default();
        ctx.init();
        ctx
    }

    fn init(&mut self) {
        self.a = 0x67452301;
        self.b = 0xefcdab89;
        self.c = 0x98badcfe;
        self.d = 0x10325476;
        self.total = [0, 0];
        self.buflen = 0;
    }

    pub fn process_bytes(&mut self, buffer: &[u8]) {
        let mut len = buffer.len();
        let mut offset = 0;

        // Process existing buffer if any
        if self.buflen != 0 {
            let left_over = self.buflen;
            let add = (128 - left_over).min(len);

            let buffer_u32 = &mut self.buffer;
            let buffer_u8 = unsafe {
                std::slice::from_raw_parts_mut(
                    buffer_u32.as_mut_ptr() as *mut u8,
                    buffer_u32.len() * 4,
                )
            };

            buffer_u8[left_over..left_over + add].copy_from_slice(&buffer[..add]);
            self.buflen += add;
            offset += add;
            len -= add;

            if self.buflen > 64 {
                let process_len = self.buflen & !63;
                self.process_block(&buffer_u8[..process_len]);
                self.buflen -= process_len;
                buffer_u8.copy_within(process_len..process_len + self.buflen, 0);
            }
        }

        // Process complete blocks
        if len >= 64 {
            let process_len = len & !63;
            self.process_block(&buffer[offset..offset + process_len]);
            offset += process_len;
            len -= process_len;
        }

        // Store remaining bytes
        if len > 0 {
            let buffer_u32 = &mut self.buffer;
            let buffer_u8 = unsafe {
                std::slice::from_raw_parts_mut(
                    buffer_u32.as_mut_ptr() as *mut u8,
                    buffer_u32.len() * 4,
                )
            };
            buffer_u8[self.buflen..self.buflen + len].copy_from_slice(&buffer[offset..]);
            self.buflen += len;
        }
    }

    pub fn finish(mut self, resbuf: &mut [u8; MD4_DIGEST_SIZE]) {
        // Pad with 0x80 and zeros
        let bytes = self.buflen;
        let pad = if bytes >= 56 { 64 + 56 - bytes } else { 56 - bytes };

        let mut padding = [0u8; 64];
        padding[0] = 0x80;
        self.process_bytes(&padding[..pad]);

        // Append length in bits
        let total_bits = (self.total[0] << 3) as u64 | ((self.total[1] as u64) << 35);
        let len_bytes = total_bits.to_le_bytes();
        self.process_bytes(&len_bytes);

        // Write result
        resbuf[..4].copy_from_slice(&self.a.to_le_bytes());
        resbuf[4..8].copy_from_slice(&self.b.to_le_bytes());
        resbuf[8..12].copy_from_slice(&self.c.to_le_bytes());
        resbuf[12..16].copy_from_slice(&self.d.to_le_bytes());
    }

    fn process_block(&mut self, block: &[u8]) {
        let mut x = [0u32; 16];
        for (i, chunk) in block.chunks_exact(4).enumerate() {
            x[i] = u32::from_le_bytes(chunk.try_into().unwrap());
        }

        let mut a = self.a;
        let mut b = self.b;
        let mut c = self.c;
        let mut d = self.d;

        // Round 1
        macro_rules! F {
            ($x:expr, $y:expr, $z:expr) => {
                $z ^ ($x & ($y ^ $z))
            };
        }
        macro_rules! R1 {
            ($a:expr, $b:expr, $c:expr, $d:expr, $k:expr, $s:expr) => {
                $a = $a
                    .wrapping_add(F!($b, $c, $d))
                    .wrapping_add(x[$k])
                    .rotate_left($s);
            };
        }

        R1!(a, b, c, d, 0, 3);
        R1!(d, a, b, c, 1, 7);
        R1!(c, d, a, b, 2, 11);
        R1!(b, c, d, a, 3, 19);
        R1!(a, b, c, d, 4, 3);
        R1!(d, a, b, c, 5, 7);
        R1!(c, d, a, b, 6, 11);
        R1!(b, c, d, a, 7, 19);
        R1!(a, b, c, d, 8, 3);
        R1!(d, a, b, c, 9, 7);
        R1!(c, d, a, b, 10, 11);
        R1!(b, c, d, a, 11, 19);
        R1!(a, b, c, d, 12, 3);
        R1!(d, a, b, c, 13, 7);
        R1!(c, d, a, b, 14, 11);
        R1!(b, c, d, a, 15, 19);

        // Round 2
        macro_rules! G {
            ($x:expr, $y:expr, $z:expr) => {
                ($x & $y) | ($x & $z) | ($y & $z)
            };
        }
        macro_rules! R2 {
            ($a:expr, $b:expr, $c:expr, $d:expr, $k:expr, $s:expr) => {
                $a = $a
                    .wrapping_add(G!($b, $c, $d))
                    .wrapping_add(x[$k])
                    .wrapping_add(0x5a827999)
                    .rotate_left($s);
            };
        }

        R2!(a, b, c, d, 0, 3);
        R2!(d, a, b, c, 4, 5);
        R2!(c, d, a, b, 8, 9);
        R2!(b, c, d, a, 12, 13);
        R2!(a, b, c, d, 1, 3);
        R2!(d, a, b, c, 5, 5);
        R2!(c, d, a, b, 9, 9);
        R2!(b, c, d, a, 13, 13);
        R2!(a, b, c, d, 2, 3);
        R2!(d, a, b, c, 6, 5);
        R2!(c, d, a, b, 10, 9);
        R2!(b, c, d, a, 14, 13);
        R2!(a, b, c, d, 3, 3);
        R2!(d, a, b, c, 7, 5);
        R2!(c, d, a, b, 11, 9);
        R2!(b, c, d, a, 15, 13);

        // Round 3
        macro_rules! H {
            ($x:expr, $y:expr, $z:expr) => {
                $x ^ $y ^ $z
            };
        }
        macro_rules! R3 {
            ($a:expr, $b:expr, $c:expr, $d:expr, $k:expr, $s:expr) => {
                $a = $a
                    .wrapping_add(H!($b, $c, $d))
                    .wrapping_add(x[$k])
                    .wrapping_add(0x6ed9eba1)
                    .rotate_left($s);
            };
        }

        R3!(a, b, c, d, 0, 3);
        R3!(d, a, b, c, 8, 9);
        R3!(c, d, a, b, 4, 11);
        R3!(b, c, d, a, 12, 15);
        R3!(a, b, c, d, 2, 3);
        R3!(d, a, b, c, 10, 9);
        R3!(c, d, a, b, 6, 11);
        R3!(b, c, d, a, 14, 15);
        R3!(a, b, c, d, 1, 3);
        R3!(d, a, b, c, 9, 9);
        R3!(c, d, a, b, 5, 11);
        R3!(b, c, d, a, 13, 15);
        R3!(a, b, c, d, 3, 3);
        R3!(d, a, b, c, 11, 9);
        R3!(c, d, a, b, 7, 11);
        R3!(b, c, d, a, 15, 15);

        // Update state
        self.a = self.a.wrapping_add(a);
        self.b = self.b.wrapping_add(b);
        self.c = self.c.wrapping_add(c);
        self.d = self.d.wrapping_add(d);

        // Update total bytes processed
        let len = block.len() as u32;
        self.total[0] = self.total[0].wrapping_add(len);
        if self.total[0] < len {
            self.total[1] = self.total[1].wrapping_add(1);
        }
    }
}

pub fn md4_buffer(buffer: &[u8]) -> [u8; MD4_DIGEST_SIZE] {
    let mut ctx = Md4Ctx::new();
    ctx.process_bytes(buffer);
    let mut result = [0u8; MD4_DIGEST_SIZE];
    ctx.finish(&mut result);
    result
}

pub fn md4_stream(mut stream: impl Read) -> IoResult<[u8; MD4_DIGEST_SIZE]> {
    let mut ctx = Md4Ctx::new();
    let mut buffer = [0u8; 4096];

    loop {
        let bytes_read = stream.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        ctx.process_bytes(&buffer[..bytes_read]);
    }

    let mut result = [0u8; MD4_DIGEST_SIZE];
    ctx.finish(&mut result);
    Ok(result)
}