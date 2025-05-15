//! MD5 hash function implementation in Rust.
//!
//! Based on the original C code from GNU Nettle, which is:
//! Copyright (C) 2001 Niels Möller
//! Licensed under either GNU Lesser General Public License or GNU General Public License.

use std::convert::TryInto;

pub const MD5_DIGEST_SIZE: usize = 16;
pub const MD5_BLOCK_SIZE: usize = 64;
pub const MD5_DATA_SIZE: usize = MD5_BLOCK_SIZE;

const DIGEST_LENGTH: usize = 4;

/// MD5 context structure
pub struct Md5Context {
    state: [u32; DIGEST_LENGTH],
    count: u64,
    index: usize,
    block: [u8; MD5_BLOCK_SIZE],
}

impl Md5Context {
    /// Initialize MD5 context
    pub fn new() -> Self {
        Md5Context {
            state: [0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476],
            count: 0,
            index: 0,
            block: [0; MD5_BLOCK_SIZE],
        }
    }

    /// Update context with new data
    pub fn update(&mut self, data: &[u8]) {
        let mut length = data.len();
        let mut offset = 0;

        // Update index
        let index = self.index as usize;
        let part_len = MD5_BLOCK_SIZE - index;

        // Transform as many times as possible
        if length >= part_len {
            self.block[index..].copy_from_slice(&data[..part_len]);
            self.compress(&self.block);
            offset += part_len;
            length -= part_len;

            while length >= MD5_BLOCK_SIZE {
                self.compress(&data[offset..offset + MD5_BLOCK_SIZE]);
                offset += MD5_BLOCK_SIZE;
                length -= MD5_BLOCK_SIZE;
                self.count += 1;
            }

            self.index = 0;
        } else {
            self.index += length;
            return;
        }

        // Buffer remaining input
        if length > 0 {
            self.block[..length].copy_from_slice(&data[offset..]);
            self.index = length;
        }
    }

    /// Finalize and output digest
    pub fn digest(mut self, digest: &mut [u8]) -> Result<(), &'static str> {
        if digest.len() < MD5_DIGEST_SIZE {
            return Err("Digest buffer too small");
        }

        let bit_count = (self.count << 9) | (self.index as u64) << 3;

        // Pad with 1 followed by zeros
        let pad_len = if self.index < 56 {
            56 - self.index
        } else {
            120 - self.index
        };

        let padding = [0x80u8];
        self.update(&padding);
        self.update(&vec![0; pad_len - 1]);

        // Append length (before padding)
        let bit_count_bytes = bit_count.to_le_bytes();
        self.update(&bit_count_bytes);

        // Write the digest
        for (i, &word) in self.state.iter().enumerate() {
            digest[i*4..i*4+4].copy_from_slice(&word.to_le_bytes());
        }

        Ok(())
    }

    /// MD5 compression function
    fn compress(&mut self, input: &[u8]) {
        assert!(input.len() >= MD5_BLOCK_SIZE);

        let mut data = [0u32; 16];
        for (i, chunk) in input.chunks_exact(4).take(16).enumerate() {
            data[i] = u32::from_le_bytes(chunk.try_into().unwrap());
        }

        let mut a = self.state[0];
        let mut b = self.state[1];
        let mut c = self.state[2];
        let mut d = self.state[3];

        macro_rules! round {
            ($f:expr, $a:ident, $b:ident, $c:ident, $d:ident, $x:expr, $s:expr, $ac:expr) => {
                $a = $a.wrapping_add($f($b, $c, $d).wrapping_add($x).wrapping_add($ac);
                $a = $a.rotate_left($s);
                $a = $a.wrapping_add($b);
            };
        }

        macro_rules! F {
            ($x:expr, $y:expr, $z:expr) => {
                ($z ^ ($x & ($y ^ $z)))
            };
        }
        macro_rules! G {
            ($x:expr, $y:expr, $z:expr) => {
                ($y ^ ($z & ($x ^ $y)))
            };
        }
        macro_rules! H {
            ($x:expr, $y:expr, $z:expr) => {
                ($x ^ $y ^ $z)
            };
        }
        macro_rules! I {
            ($x:expr, $y:expr, $z:expr) => {
                ($y ^ ($x | !$z))
            };
        }

        // Round 1
        round!(F, a, b, c, d, data[0], 7, 0xd76aa478);
        round!(F, d, a, b, c, data[1], 12, 0xe8c7b756);
        round!(F, c, d, a, b, data[2], 17, 0x242070db);
        round!(F, b, c, d, a, data[3], 22, 0xc1bdceee);
        round!(F, a, b, c, d, data[4], 7, 0xf57c0faf);
        round!(F, d, a, b, c, data[5], 12, 0x4787c62a);
        round!(F, c, d, a, b, data[6], 17, 0xa8304613);
        round!(F, b, c, d, a, data[7], 22, 0xfd469501);
        round!(F, a, b, c, d, data[8], 7, 0x698098d8);
        round!(F, d, a, b, c, data[9], 12, 0x8b44f7af);
        round!(F, c, d, a, b, data[10], 17, 0xffff5bb1);
        round!(F, b, c, d, a, data[11], 22, 0x895cd7be);
        round!(F, a, b, c, d, data[12], 7, 0x6b901122);
        round!(F, d, a, b, c, data[13], 12, 0xfd987193);
        round!(F, c, d, a, b, data[14], 17, 0xa679438e);
        round!(F, b, c, d, a, data[15], 22, 0x49b40821);

        // Round 2
        round!(G, a, b, c, d, data[1], 5, 0xf61e2562);
        round!(G, d, a, b, c, data[6], 9, 0xc040b340);
        round!(G, c, d, a, b, data[11], 14, 0x265e5a51);
        round!(G, b, c, d, a, data[0], 20, 0xe9b6c7aa);
        round!(G, a, b, c, d, data[5], 5, 0xd62f105d);
        round!(G, d, a, b, c, data[10], 9, 0x02441453);
        round!(G, c, d, a, b, data[15], 14, 0xd8a1e681);
        round!(G, b, c, d, a, data[4], 20, 0xe7d3fbc8);
        round!(G, a, b, c, d, data[9], 5, 0x21e1cde6);
        round!(G, d, a, b, c, data[14], 9, 0xc33707d6);
        round!(G, c, d, a, b, data[3], 14, 0xf4d50d87);
        round!(G, b, c, d, a, data[8], 20, 0x455a14ed);
        round!(G, a, b, c, d, data[13], 5, 0xa9e3e905);
        round!(G, d, a, b, c, data[2], 9, 0xfcefa3f8);
        round!(G, c, d, a, b, data[7], 14, 0x676f02d9);
        round!(G, b, c, d, a, data[12], 20, 0x8d2a4c8a);

        // Round 3
        round!(H, a, b, c, d, data[5], 4, 0xfffa3942);
        round!(H, d, a, b, c, data[8], 11, 0x8771f681);
        round!(H, c, d, a, b, data[11], 16, 0x6d9d6122);
        round!(H, b, c, d, a, data[14], 23, 0xfde5380c);
        round!(H, a, b, c, d, data[1], 4, 0xa4beea44);
        round!(H, d, a, b, c, data[4], 11, 0x4bdecfa9);
        round!(H, c, d, a, b, data[7], 16, 0xf6bb4b60);
        round!(H, b, c, d, a, data[10], 23, 0xbebfbc70);
        round!(H, a, b, c, d, data[13], 4, 0x289b7ec6);
        round!(H, d, a, b, c, data[0], 11, 0xeaa127fa);
        round!(H, c, d, a, b, data[3], 16, 0xd4ef3085);
        round!(H, b, c, d, a, data[6], 23, 0x04881d05);
        round!(H, a, b, c, d, data[9], 4, 0xd9d4d039);
        round!(H, d, a, b, c, data[12], 11, 0xe6db99e5);
        round!(H, c, d, a, b, data[15], 16, 0x1fa27cf8);
        round!(H, b, c, d, a, data[2], 23, 0xc4ac5665);

        // Round 4
        round!(I, a, b, c, d, data[0], 6, 0xf4292244);
        round!(I, d, a, b, c, data[7], 10, 0x432aff97);
        round!(I, c, d, a, b, data[14], 15, 0xab9423a7);
        round!(I, b, c, d, a, data[5], 21, 0xfc93a039);
        round!(I, a, b, c, d, data[12], 6, 0x655b59c3);
        round!(I, d, a, b, c, data[3], 10, 0x8f0ccc92);
        round!(I, c, d, a, b, data[10], 15, 0xffeff47d);
        round!(I, b, c, d, a, data[1], 21, 0x85845dd1);
        round!(I, a, b, c, d, data[8], 6, 0x6fa87e4f);
        round!(I, d, a, b, c, data[15], 10, 0xfe2ce6e0);
        round!(I, c, d, a, b, data[6], 15, 0xa3014314);
        round!(I, b, c, d, a, data[13], 21, 0x4e0811a1);
        round!(I, a, b, c, d, data[4], 6, 0xf7537e82);
        round!(I, d, a, b, c, data[11], 10, 0xbd3af235);
        round!(I, c, d, a, b, data[2], 15, 0x2ad7d2bb);
        round!(I, b, c, d, a, data[9], 21, 0xeb86d391);

        self.state[0] = self.state[0].wrapping_add(a);
        self.state[1] = self.state[1].wrapping_add(b);
        self.state[2] = self.state[2].wrapping_add(c);
        self.state[3] = self.state[3].wrapping_add(d);
    }
}

/// Compute MD5 hash of input data
pub fn md5(data: &[u8]) -> [u8; MD5_DIGEST_SIZE] {
    let mut ctx = Md5Context::new();
    ctx.update(data);
    let mut digest = [0u8; MD5_DIGEST_SIZE];
    ctx.digest(&mut digest).unwrap();
    digest
}