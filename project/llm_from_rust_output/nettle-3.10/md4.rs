use std::convert::TryInto;

const MD4_DIGEST_SIZE: usize = 16;
const BLOCK_SIZE: usize = 64;

#[derive(Clone, Copy)]
pub struct Md4Ctx {
    state: [u32; 4],
    count: u64,
    index: usize,
    block: [u8; BLOCK_SIZE],
}

impl Md4Ctx {
    pub fn new() -> Self {
        let iv = [
            0x67452301u32,
            0xefcdab89,
            0x98badcfe,
            0x10325476,
        ];
        Md4Ctx {
            state: iv,
            count: 0,
            index: 0,
            block: [0; BLOCK_SIZE],
        }
    }

    pub fn update(&mut self, data: &[u8]) {
        if data.is_empty() {
            return;
        }

        if self.index != 0 {
            let remaining = BLOCK_SIZE - self.index;
            if data.len() < remaining {
                self.block[self.index..self.index + data.len()].copy_from_slice(data);
                self.index += data.len();
                return;
            }

            let (left, right) = data.split_at(remaining);
            self.block[self.index..].copy_from_slice(left);
            self.compress(&self.block);
            self.count += 1;
            self.index = 0;
            self.update(right);
        } else {
            let chunks = data.chunks_exact(BLOCK_SIZE);
            let remainder = chunks.remainder();
            for chunk in chunks {
                self.compress(chunk.try_into().unwrap());
                self.count += 1;
            }
            if !remainder.is_empty() {
                self.block[..remainder.len()].copy_from_slice(remainder);
                self.index = remainder.len();
            }
        }
    }

    pub fn digest(&mut self, output: &mut [u8]) {
        assert!(output.len() <= MD4_DIGEST_SIZE);

        let mut bit_count = self.count << 9 | (self.index << 3) as u64;
        let mut data = [0u32; 16];

        self.block[self.index] = 0x80;
        self.index += 1;

        if self.index > BLOCK_SIZE - 8 {
            self.block[self.index..].fill(0);
            self.compress(&self.block);
            self.index = 0;
        }

        self.block[self.index..BLOCK_SIZE - 8].fill(0);

        for i in 0..14 {
            data[i] = u32::from_le_bytes([
                self.block[i * 4],
                self.block[i * 4 + 1],
                self.block[i * 4 + 2],
                self.block[i * 4 + 3],
            ]);
        }

        data[14] = bit_count as u32;
        data[15] = (bit_count >> 32) as u32;

        self.transform(&data);

        for (i, &word) in self.state.iter().enumerate() {
            output[i * 4..(i + 1) * 4].copy_from_slice(&word.to_le_bytes());
        }

        *self = Self::new();
    }

    fn compress(&mut self, block: &[u8; BLOCK_SIZE]) {
        let mut data = [0u32; 16];
        for (i, chunk) in block.chunks_exact(4).enumerate() {
            data[i] = u32::from_le_bytes(chunk.try_into().unwrap());
        }
        self.transform(&data);
    }

    fn transform(&mut self, data: &[u32; 16]) {
        let mut a = self.state[0];
        let mut b = self.state[1];
        let mut c = self.state[2];
        let mut d = self.state[3];

        // Round 1
        for &i in &[0, 4, 8, 12] {
            a = a.wrapping_add((b & c) | (!b & d)).wrapping_add(data[i]).rotate_left(3);
            d = d.wrapping_add((a & b) | (!a & c)).wrapping_add(data[i + 1]).rotate_left(7);
            c = c.wrapping_add((d & a) | (!d & b)).wrapping_add(data[i + 2]).rotate_left(11);
            b = b.wrapping_add((c & d) | (!c & a)).wrapping_add(data[i + 3]).rotate_left(19);
        }

        // Round 2
        for &i in &[0, 1, 2, 3] {
            a = a.wrapping_add((b & c) | (b & d) | (c & d))
                .wrapping_add(data[i])
                .wrapping_add(0x5a827999)
                .rotate_left(3);
            d = d.wrapping_add((a & b) | (a & c) | (b & c))
                .wrapping_add(data[i + 4])
                .wrapping_add(0x5a827999)
                .rotate_left(5);
            c = c.wrapping_add((d & a) | (d & b) | (a & b))
                .wrapping_add(data[i + 8])
                .wrapping_add(0x5a827999)
                .rotate_left(9);
            b = b.wrapping_add((c & d) | (c & a) | (d & a))
                .wrapping_add(data[i + 12])
                .wrapping_add(0x5a827999)
                .rotate_left(13);
        }

        // Round 3
        for &i in &[0, 2, 1, 3] {
            a = a.wrapping_add(b ^ c ^ d)
                .wrapping_add(data[i])
                .wrapping_add(0x6ed9eba1)
                .rotate_left(3);
            d = d.wrapping_add(a ^ b ^ c)
                .wrapping_add(data[i + 8])
                .wrapping_add(0x6ed9eba1)
                .rotate_left(9);
            c = c.wrapping_add(d ^ a ^ b)
                .wrapping_add(data[i + 4])
                .wrapping_add(0x6ed9eba1)
                .rotate_left(11);
            b = b.wrapping_add(c ^ d ^ a)
                .wrapping_add(data[i + 12])
                .wrapping_add(0x6ed9eba1)
                .rotate_left(15);
        }

        self.state[0] = self.state[0].wrapping_add(a);
        self.state[1] = self.state[1].wrapping_add(b);
        self.state[2] = self.state[2].wrapping_add(c);
        self.state[3] = self.state[3].wrapping_add(d);
    }
}

pub fn md4(data: &[u8]) -> [u8; MD4_DIGEST_SIZE] {
    let mut ctx = Md4Ctx::new();
    ctx.update(data);
    let mut digest = [0; MD4_DIGEST_SIZE];
    ctx.digest(&mut digest);
    digest
}