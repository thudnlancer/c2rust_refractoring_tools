use std::convert::TryInto;

pub const MD4_DIGEST_SIZE: usize = 16;
pub const MD4_BLOCK_SIZE: usize = 64;
pub const MD4_DATA_SIZE: usize = MD4_BLOCK_SIZE;
const _MD4_DIGEST_LENGTH: usize = 4;
const MD4_DATA_LENGTH: usize = 16;

pub struct Md4Context {
    state: [u32; _MD4_DIGEST_LENGTH],
    count: u64,
    index: usize,
    block: [u8; MD4_BLOCK_SIZE],
}

impl Md4Context {
    pub fn new() -> Self {
        let iv = [
            0x67452301,
            0xefcdab89,
            0x98badcfe,
            0x10325476,
        ];
        Md4Context {
            state: iv,
            count: 0,
            index: 0,
            block: [0; MD4_BLOCK_SIZE],
        }
    }

    pub fn update(&mut self, data: &[u8]) {
        let mut length = data.len();
        let mut offset = 0;

        while length > 0 {
            let available = MD4_BLOCK_SIZE - self.index;
            let to_copy = std::cmp::min(available, length);

            self.block[self.index..self.index + to_copy].copy_from_slice(&data[offset..offset + to_copy]);
            self.index += to_copy;
            offset += to_copy;
            length -= to_copy;

            if self.index == MD4_BLOCK_SIZE {
                self.compress();
                self.count += 1;
                self.index = 0;
            }
        }
    }

    pub fn digest(&mut self, output: &mut [u8]) {
        assert!(output.len() <= MD4_DIGEST_SIZE);

        // Pad the message
        let pad_byte = 0x80;
        self.block[self.index] = pad_byte;
        self.index += 1;

        if self.index > MD4_BLOCK_SIZE - 8 {
            if self.index < MD4_BLOCK_SIZE {
                self.block[self.index..MD4_BLOCK_SIZE].fill(0);
            }
            self.compress();
            self.index = 0;
        }

        self.block[self.index..MD4_BLOCK_SIZE - 8].fill(0);

        // Append the bit count
        let bit_count = (self.count << 9) | (self.index as u64) << 3;
        for (i, byte) in bit_count.to_le_bytes().iter().enumerate() {
            self.block[MD4_BLOCK_SIZE - 8 + i] = *byte;
        }

        self.compress();

        // Write the output in little-endian
        for (i, word) in self.state.iter().enumerate() {
            let bytes = word.to_le_bytes();
            let start = i * 4;
            let end = start + 4;
            if end <= output.len() {
                output[start..end].copy_from_slice(&bytes);
            }
        }

        // Reset the context
        *self = Md4Context::new();
    }

    fn compress(&mut self) {
        let mut data = [0u32; MD4_DATA_LENGTH];
        
        for i in 0..MD4_DATA_LENGTH {
            let start = i * 4;
            data[i] = u32::from_le_bytes([
                self.block[start],
                self.block[start + 1],
                self.block[start + 2],
                self.block[start + 3],
            ]);
        }

        md4_transform(&mut self.state, &data);
    }
}

fn md4_transform(digest: &mut [u32; _MD4_DIGEST_LENGTH], data: &[u32; MD4_DATA_LENGTH]) {
    let mut a = digest[0];
    let mut b = digest[1];
    let mut c = digest[2];
    let mut d = digest[3];

    macro_rules! round_f {
        ($w:ident, $x:ident, $y:ident, $z:ident, $data:expr, $s:expr) => {
            $w = $w.wrapping_add(f($x, $y, $z).wrapping_add($data))
                .rotate_left($s);
        };
    }

    macro_rules! round_g {
        ($w:ident, $x:ident, $y:ident, $z:ident, $data:expr, $s:expr) => {
            $w = $w.wrapping_add(g($x, $y, $z).wrapping_add($data))
                .rotate_left($s);
        };
    }

    macro_rules! round_h {
        ($w:ident, $x:ident, $y:ident, $z:ident, $data:expr, $s:expr) => {
            $w = $w.wrapping_add(h($x, $y, $z).wrapping_add($data))
                .rotate_left($s);
        };
    }

    // Round 1
    round_f!(a, b, c, d, data[0], 3);
    round_f!(d, a, b, c, data[1], 7);
    round_f!(c, d, a, b, data[2], 11);
    round_f!(b, c, d, a, data[3], 19);
    round_f!(a, b, c, d, data[4], 3);
    round_f!(d, a, b, c, data[5], 7);
    round_f!(c, d, a, b, data[6], 11);
    round_f!(b, c, d, a, data[7], 19);
    round_f!(a, b, c, d, data[8], 3);
    round_f!(d, a, b, c, data[9], 7);
    round_f!(c, d, a, b, data[10], 11);
    round_f!(b, c, d, a, data[11], 19);
    round_f!(a, b, c, d, data[12], 3);
    round_f!(d, a, b, c, data[13], 7);
    round_f!(c, d, a, b, data[14], 11);
    round_f!(b, c, d, a, data[15], 19);

    // Round 2
    round_g!(a, b, c, d, data[0].wrapping_add(0x5a827999), 3);
    round_g!(d, a, b, c, data[4].wrapping_add(0x5a827999), 5);
    round_g!(c, d, a, b, data[8].wrapping_add(0x5a827999), 9);
    round_g!(b, c, d, a, data[12].wrapping_add(0x5a827999), 13);
    round_g!(a, b, c, d, data[1].wrapping_add(0x5a827999), 3);
    round_g!(d, a, b, c, data[5].wrapping_add(0x5a827999), 5);
    round_g!(c, d, a, b, data[9].wrapping_add(0x5a827999), 9);
    round_g!(b, c, d, a, data[13].wrapping_add(0x5a827999), 13);
    round_g!(a, b, c, d, data[2].wrapping_add(0x5a827999), 3);
    round_g!(d, a, b, c, data[6].wrapping_add(0x5a827999), 5);
    round_g!(c, d, a, b, data[10].wrapping_add(0x5a827999), 9);
    round_g!(b, c, d, a, data[14].wrapping_add(0x5a827999), 13);
    round_g!(a, b, c, d, data[3].wrapping_add(0x5a827999), 3);
    round_g!(d, a, b, c, data[7].wrapping_add(0x5a827999), 5);
    round_g!(c, d, a, b, data[11].wrapping_add(0x5a827999), 9);
    round_g!(b, c, d, a, data[15].wrapping_add(0x5a827999), 13);

    // Round 3
    round_h!(a, b, c, d, data[0].wrapping_add(0x6ed9eba1), 3);
    round_h!(d, a, b, c, data[8].wrapping_add(0x6ed9eba1), 9);
    round_h!(c, d, a, b, data[4].wrapping_add(0x6ed9eba1), 11);
    round_h!(b, c, d, a, data[12].wrapping_add(0x6ed9eba1), 15);
    round_h!(a, b, c, d, data[2].wrapping_add(0x6ed9eba1), 3);
    round_h!(d, a, b, c, data[10].wrapping_add(0x6ed9eba1), 9);
    round_h!(c, d, a, b, data[6].wrapping_add(0x6ed9eba1), 11);
    round_h!(b, c, d, a, data[14].wrapping_add(0x6ed9eba1), 15);
    round_h!(a, b, c, d, data[1].wrapping_add(0x6ed9eba1), 3);
    round_h!(d, a, b, c, data[9].wrapping_add(0x6ed9eba1), 9);
    round_h!(c, d, a, b, data[5].wrapping_add(0x6ed9eba1), 11);
    round_h!(b, c, d, a, data[13].wrapping_add(0x6ed9eba1), 15);
    round_h!(a, b, c, d, data[3].wrapping_add(0x6ed9eba1), 3);
    round_h!(d, a, b, c, data[11].wrapping_add(0x6ed9eba1), 9);
    round_h!(c, d, a, b, data[7].wrapping_add(0x6ed9eba1), 11);
    round_h!(b, c, d, a, data[15].wrapping_add(0x6ed9eba1), 15);

    digest[0] = digest[0].wrapping_add(a);
    digest[1] = digest[1].wrapping_add(b);
    digest[2] = digest[2].wrapping_add(c);
    digest[3] = digest[3].wrapping_add(d);
}

#[inline]
fn f(x: u32, y: u32, z: u32) -> u32 {
    (y & x) | (z & !x)
}

#[inline]
fn g(x: u32, y: u32, z: u32) -> u32 {
    (y & x) | (z & x) | (y & z)
}

#[inline]
fn h(x: u32, y: u32, z: u32) -> u32 {
    x ^ y ^ z
}