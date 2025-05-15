use std::convert::TryInto;

const MD5_DIGEST_SIZE: usize = 16;

#[derive(Clone)]
pub struct Md5Context {
    state: [u32; 4],
    count: u64,
    index: usize,
    block: [u8; 64],
}

impl Md5Context {
    pub fn new() -> Self {
        Md5Context {
            state: [0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476],
            count: 0,
            index: 0,
            block: [0; 64],
        }
    }

    pub fn update(&mut self, data: &[u8]) {
        if data.is_empty() {
            return;
        }

        let mut remaining = data.len();
        let mut offset = 0;

        if self.index > 0 {
            let available = 64 - self.index;
            let to_copy = remaining.min(available);

            self.block[self.index..self.index + to_copy].copy_from_slice(&data[..to_copy]);
            self.index += to_copy;
            offset += to_copy;
            remaining -= to_copy;

            if self.index == 64 {
                self.compress();
                self.index = 0;
            }
        }

        while remaining >= 64 {
            let chunk = &data[offset..offset + 64];
            self.state = compress(self.state, chunk);
            self.count += 1;
            offset += 64;
            remaining -= 64;
        }

        if remaining > 0 {
            self.block[..remaining].copy_from_slice(&data[offset..]);
            self.index = remaining;
        }
    }

    pub fn digest(&mut self, output: &mut [u8]) {
        assert!(output.len() <= MD5_DIGEST_SIZE);

        let mut bit_count = (self.count << 9) | ((self.index as u64) << 3);
        let mut padding = [0u8; 64];

        if self.index < 64 {
            padding[0] = 0x80;
        }

        if self.index > 56 {
            self.block[self.index..].copy_from_slice(&padding[..64 - self.index]);
            self.compress();
            self.index = 0;
            padding = [0; 64];
        }

        self.block[self.index..56].copy_from_slice(&padding[..56 - self.index]);
        self.block[56..].copy_from_slice(&bit_count.to_le_bytes());

        self.compress();

        for (i, &word) in self.state.iter().enumerate() {
            output[i * 4..(i + 1) * 4].copy_from_slice(&word.to_le_bytes());
        }

        *self = Self::new();
    }

    fn compress(&mut self) {
        self.state = compress(self.state, &self.block);
    }
}

fn compress(state: [u32; 4], block: &[u8]) -> [u32; 4] {
    let mut a = state[0];
    let mut b = state[1];
    let mut c = state[2];
    let mut d = state[3];
    
    let mut data = [0u32; 16];
    
    for i in 0..16 {
        let offset = i * 4;
        data[i] = u32::from_le_bytes([
            block[offset],
            block[offset + 1],
            block[offset + 2],
            block[offset + 3],
        ]);
    }

    // Round 1
    macro_rules! round1 {
        ($a:expr, $b:expr, $c:expr, $d:expr, $k:expr, $s:expr, $i:expr) => {
            $a = $b.wrapping_add(
                ($a.wrapping_add(F($b, $c, $d))
                    .wrapping_add(data[$i])
                    .wrapping_add($k))
                    .rotate_left($s),
            )
        };
    }

    round1!(a, b, c, d, 0xd76aa478, 7, 0);
    round1!(d, a, b, c, 0xe8c7b756, 12, 1);
    round1!(c, d, a, b, 0x242070db, 17, 2);
    round1!(b, c, d, a, 0xc1bdceee, 22, 3);
    round1!(a, b, c, d, 0xf57c0faf, 7, 4);
    round1!(d, a, b, c, 0x4787c62a, 12, 5);
    round1!(c, d, a, b, 0xa8304613, 17, 6);
    round1!(b, c, d, a, 0xfd469501, 22, 7);
    round1!(a, b, c, d, 0x698098d8, 7, 8);
    round1!(d, a, b, c, 0x8b44f7af, 12, 9);
    round1!(c, d, a, b, 0xffff5bb1, 17, 10);
    round1!(b, c, d, a, 0x895cd7be, 22, 11);
    round1!(a, b, c, d, 0x6b901122, 7, 12);
    round1!(d, a, b, c, 0xfd987193, 12, 13);
    round1!(c, d, a, b, 0xa679438e, 17, 14);
    round1!(b, c, d, a, 0x49b40821, 22, 15);

    // Round 2
    macro_rules! round2 {
        ($a:expr, $b:expr, $c:expr, $d:expr, $k:expr, $s:expr, $i:expr) => {
            $a = $b.wrapping_add(
                ($a.wrapping_add(G($b, $c, $d))
                    .wrapping_add(data[$i])
                    .wrapping_add($k)
                    .rotate_left($s),
            )
        };
    }

    round2!(a, b, c, d, 0xf61e2562, 5, 1);
    round2!(d, a, b, c, 0xc040b340, 9, 6);
    round2!(c, d, a, b, 0x265e5a51, 14, 11);
    round2!(b, c, d, a, 0xe9b6c7aa, 20, 0);
    round2!(a, b, c, d, 0xd62f105d, 5, 5);
    round2!(d, a, b, c, 0x02441453, 9, 10);
    round2!(c, d, a, b, 0xd8a1e681, 14, 15);
    round2!(b, c, d, a, 0xe7d3fbc8, 20, 4);
    round2!(a, b, c, d, 0x21e1cde6, 5, 9);
    round2!(d, a, b, c, 0xc33707d6, 9, 14);
    round2!(c, d, a, b, 0xf4d50d87, 14, 3);
    round2!(b, c, d, a, 0x455a14ed, 20, 8);
    round2!(a, b, c, d, 0xa9e3e905, 5, 13);
    round2!(d, a, b, c, 0xfcefa3f8, 9, 2);
    round2!(c, d, a, b, 0x676f02d9, 14, 7);
    round2!(b, c, d, a, 0x8d2a4c8a, 20, 12);

    // Round 3
    macro_rules! round3 {
        ($a:expr, $b:expr, $c:expr, $d:expr, $k:expr, $s:expr, $i:expr) => {
            $a = $b.wrapping_add(
                ($a.wrapping_add(H($b, $c, $d))
                    .wrapping_add(data[$i])
                    .wrapping_add($k)
                    .rotate_left($s),
            )
        };
    }

    round3!(a, b, c, d, 0xfffa3942, 4, 5);
    round3!(d, a, b, c, 0x8771f681, 11, 8);
    round3!(c, d, a, b, 0x6d9d6122, 16, 11);
    round3!(b, c, d, a, 0xfde5380c, 23, 14);
    round3!(a, b, c, d, 0xa4beea44, 4, 1);
    round3!(d, a, b, c, 0x4bdecfa9, 11, 4);
    round3!(c, d, a, b, 0xf6bb4b60, 16, 7);
    round3!(b, c, d, a, 0xbebfbc70, 23, 10);
    round3!(a, b, c, d, 0x289b7ec6, 4, 13);
    round3!(d, a, b, c, 0xeaa127fa, 11, 0);
    round3!(c, d, a, b, 0xd4ef3085, 16, 3);
    round3!(b, c, d, a, 0x04881d05, 23, 6);
    round3!(a, b, c, d, 0xd9d4d039, 4, 9);
    round3!(d, a, b, c, 0xe6db99e5, 11, 12);
    round3!(c, d, a, b, 0x1fa27cf8, 16, 15);
    round3!(b, c, d, a, 0xc4ac5665, 23, 2);

    // Round 4
    macro_rules! round4 {
        ($a:expr, $b:expr, $c:expr, $d:expr, $k:expr, $s:expr, $i:expr) => {
            $a = $b.wrapping_add(
                ($a.wrapping_add(I($b, $c, $d))
                    .wrapping_add(data[$i])
                    .wrapping_add($k)
                    .rotate_left($s),
            )
        };
    }

    round4!(a, b, c, d, 0xf4292244, 6, 0);
    round4!(d, a, b, c, 0x432aff97, 10, 7);
    round4!(c, d, a, b, 0xab9423a7, 15, 14);
    round4!(b, c, d, a, 0xfc93a039, 21, 5);
    round4!(a, b, c, d, 0x655b59c3, 6, 12);
    round4!(d, a, b, c, 0x8f0ccc92, 10, 3);
    round4!(c, d, a, b, 0xffeff47d, 15, 10);
    round4!(b, c, d, a, 0x85845dd1, 21, 1);
    round4!(a, b, c, d, 0x6fa87e4f, 6, 8);
    round4!(d, a, b, c, 0xfe2ce6e0, 10, 15);
    round4!(c, d, a, b, 0xa3014314, 15, 6);
    round4!(b, c, d, a, 0x4e0811a1, 21, 13);
    round4!(a, b, c, d, 0xf7537e82, 6, 4);
    round4!(d, a, b, c, 0xbd3af235, 10, 11);
    round4!(c, d, a, b, 0x2ad7d2bb, 15, 2);
    round4!(b, c, d, a, 0xeb86d391, 21, 9);

    [
        state[0].wrapping_add(a),
        state[1].wrapping_add(b),
        state[2].wrapping_add(c),
        state[3].wrapping_add(d),
    ]
}

#[inline]
fn F(x: u32, y: u32, z: u32) -> u32 {
    (x & y) | (!x & z)
}

#[inline]
fn G(x: u32, y: u32, z: u32) -> u32 {
    (x & z) | (y & !z)
}

#[inline]
fn H(x: u32, y: u32, z: u32) -> u32 {
    x ^ y ^ z
}

#[inline]
fn I(x: u32, y: u32, z: u32) -> u32 {
    y ^ (x | !z)
}