use std::mem;

#[derive(Copy, Clone, Default)]
pub struct Md4Context {
    pub a: u32,
    pub b: u32,
    pub c: u32,
    pub d: u32,
    pub total: [u32; 2],
    pub buflen: u32,
    pub buffer: [u32; 32],
}

const FILLBUF: [u8; 64] = [
    0x80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

impl Md4Context {
    pub fn new() -> Self {
        let mut ctx = Md4Context::default();
        ctx.a = 0x67452301;
        ctx.b = 0xefcdab89;
        ctx.c = 0x98badcfe;
        ctx.d = 0x10325476;
        ctx
    }

    pub fn update(&mut self, input: &[u8]) {
        let mut input = input;
        let mut left = self.buflen as usize;
        let len = input.len();

        if left > 0 {
            let add = (128 - left).min(len);
            self.buffer[left/4..(left + add + 3)/4]
                .copy_from_slice(&input[..add]);
            self.buflen += add as u32;

            if self.buflen > 64 {
                let process_len = self.buflen & !63;
                self.process_block(&self.buffer[..(process_len/4) as usize]);
                self.buflen &= 63;
                left = (left + add) & !63;
                self.buffer.copy_within((left/4)..(left/4 + 16), 0);
            }

            input = &input[add..];
        }

        while input.len() >= 64 {
            let (block, rest) = input.split_at(64);
            self.process_block(block);
            input = rest;
        }

        if !input.is_empty() {
            let left = self.buflen as usize;
            self.buffer[left/4..(left + input.len() + 3)/4]
                .copy_from_slice(input);
            self.buflen += input.len() as u32;
        }
    }

    pub fn finalize(mut self) -> [u8; 16] {
        let bytes = self.buflen;
        let pad = if bytes >= 56 { 64 + 56 - bytes } else { 56 - bytes } as usize;

        self.total[0] = self.total[0].wrapping_add(bytes);
        if self.total[0] < bytes {
            self.total[1] = self.total[1].wrapping_add(1);
        }

        let mut padding = [0u8; 64];
        padding[..pad].copy_from_slice(&FILLBUF[..pad]);
        self.update(&padding[..pad]);

        let mut block = [0u32; 16];
        block[0] = self.total[0] << 3;
        block[1] = (self.total[1] << 3) | (self.total[0] >> 29);
        self.process_block(&block);

        let mut result = [0u8; 16];
        result[0..4].copy_from_slice(&self.a.to_le_bytes());
        result[4..8].copy_from_slice(&self.b.to_le_bytes());
        result[8..12].copy_from_slice(&self.c.to_le_bytes());
        result[12..16].copy_from_slice(&self.d.to_le_bytes());
        result
    }

    fn process_block(&mut self, block: &[u8]) {
        let mut x = [0u32; 16];
        for (i, chunk) in block.chunks_exact(4).enumerate() {
            x[i] = u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
        }

        let mut a = self.a;
        let mut b = self.b;
        let mut c = self.c;
        let mut d = self.d;

        // Round 1
        for i in [0, 4, 8, 12] {
            a = a.wrapping_add(d ^ (b & (c ^ d))).wrapping_add(x[i]).rotate_left(3);
            d = d.wrapping_add(c ^ (a & (b ^ c))).wrapping_add(x[i+1]).rotate_left(7);
            c = c.wrapping_add(b ^ (d & (a ^ b))).wrapping_add(x[i+2]).rotate_left(11);
            b = b.wrapping_add(a ^ (c & (d ^ a))).wrapping_add(x[i+3]).rotate_left(19);
        }

        // Round 2
        for i in [0, 1, 2, 3] {
            a = a.wrapping_add((b & c) | (b & d) | (c & d))
                .wrapping_add(x[i])
                .wrapping_add(0x5a827999)
                .rotate_left(3);
            d = d.wrapping_add((a & b) | (a & c) | (b & c))
                .wrapping_add(x[i+4])
                .wrapping_add(0x5a827999)
                .rotate_left(5);
            c = c.wrapping_add((d & a) | (d & b) | (a & b))
                .wrapping_add(x[i+8])
                .wrapping_add(0x5a827999)
                .rotate_left(9);
            b = b.wrapping_add((c & d) | (c & a) | (d & a))
                .wrapping_add(x[i+12])
                .wrapping_add(0x5a827999)
                .rotate_left(13);
        }

        // Round 3
        for i in [0, 2, 1, 3] {
            a = a.wrapping_add(b ^ c ^ d)
                .wrapping_add(x[i])
                .wrapping_add(0x6ed9eba1)
                .rotate_left(3);
            d = d.wrapping_add(a ^ b ^ c)
                .wrapping_add(x[i+8])
                .wrapping_add(0x6ed9eba1)
                .rotate_left(9);
            c = c.wrapping_add(d ^ a ^ b)
                .wrapping_add(x[i+4])
                .wrapping_add(0x6ed9eba1)
                .rotate_left(11);
            b = b.wrapping_add(c ^ d ^ a)
                .wrapping_add(x[i+12])
                .wrapping_add(0x6ed9eba1)
                .rotate_left(15);
        }

        self.a = self.a.wrapping_add(a);
        self.b = self.b.wrapping_add(b);
        self.c = self.c.wrapping_add(c);
        self.d = self.d.wrapping_add(d);
    }
}

pub fn md4(input: &[u8]) -> [u8; 16] {
    let mut ctx = Md4Context::new();
    ctx.update(input);
    ctx.finalize()
}