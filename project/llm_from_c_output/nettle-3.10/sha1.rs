use std::mem;

const SHA1_DIGEST_SIZE: usize = 20;
const SHA1_BLOCK_SIZE: usize = 64;
const SHA1_DATA_SIZE: usize = SHA1_BLOCK_SIZE;
const _SHA1_DIGEST_LENGTH: usize = 5;

#[derive(Clone)]
pub struct Sha1Ctx {
    state: [u32; _SHA1_DIGEST_LENGTH],
    count: u64,
    index: usize,
    block: [u8; SHA1_BLOCK_SIZE],
}

impl Sha1Ctx {
    pub fn new() -> Self {
        let iv: [u32; _SHA1_DIGEST_LENGTH] = [
            0x67452301,
            0xEFCDAB89,
            0x98BADCFE,
            0x10325476,
            0xC3D2E1F0,
        ];

        Sha1Ctx {
            state: iv,
            count: 0,
            index: 0,
            block: [0; SHA1_BLOCK_SIZE],
        }
    }

    pub fn update(&mut self, data: &[u8]) {
        let mut length = data.len();
        let mut offset = 0;

        while length > 0 {
            let available = SHA1_BLOCK_SIZE - self.index;
            let to_copy = if length < available {
                length
            } else {
                available
            };

            self.block[self.index..self.index + to_copy].copy_from_slice(&data[offset..offset + to_copy]);
            self.index += to_copy;
            offset += to_copy;
            length -= to_copy;

            if self.index == SHA1_BLOCK_SIZE {
                sha1_compress(&mut self.state, &self.block);
                self.count += 1;
                self.index = 0;
            }
        }
    }

    pub fn digest(&mut self, digest: &mut [u8]) -> Result<(), &'static str> {
        if digest.len() > SHA1_DIGEST_SIZE {
            return Err("Digest buffer too large");
        }

        // Padding
        let pad_byte = 0x80;
        self.block[self.index] = pad_byte;
        self.index += 1;

        if self.index > SHA1_BLOCK_SIZE - 8 {
            if self.index < SHA1_BLOCK_SIZE {
                self.block[self.index..].fill(0);
            }
            sha1_compress(&mut self.state, &self.block);
            self.count += 1;
            self.index = 0;
        }

        if self.index < SHA1_BLOCK_SIZE - 8 {
            self.block[self.index..SHA1_BLOCK_SIZE - 8].fill(0);
        }

        // Append bit count
        let bit_count = (self.count << 9) | (self.index as u64) << 3;
        self.block[SHA1_BLOCK_SIZE - 8..].copy_from_slice(&bit_count.to_be_bytes());
        sha1_compress(&mut self.state, &self.block);

        // Write output
        for (i, &word) in self.state.iter().enumerate() {
            let bytes = word.to_be_bytes();
            let start = i * 4;
            let end = start + 4;
            if end <= digest.len() {
                digest[start..end].copy_from_slice(&bytes);
            } else if start < digest.len() {
                digest[start..].copy_from_slice(&bytes[..digest.len() - start]);
            }
        }

        // Reset context
        *self = Sha1Ctx::new();
        Ok(())
    }
}

fn sha1_compress(state: &mut [u32; _SHA1_DIGEST_LENGTH], data: &[u8; SHA1_BLOCK_SIZE]) {
    let mut w = [0u32; 80];
    
    // Initialize the first 16 words
    for i in 0..16 {
        w[i] = u32::from_be_bytes([
            data[i * 4],
            data[i * 4 + 1],
            data[i * 4 + 2],
            data[i * 4 + 3],
        ]);
    }

    // Expand to 80 words
    for i in 16..80 {
        w[i] = (w[i-3] ^ w[i-8] ^ w[i-14] ^ w[i-16]).rotate_left(1);
    }

    let mut a = state[0];
    let mut b = state[1];
    let mut c = state[2];
    let mut d = state[3];
    let mut e = state[4];

    for i in 0..80 {
        let (f, k) = match i {
            0..=19 => ((b & c) | ((!b) & d), 0x5A827999),
            20..=39 => (b ^ c ^ d, 0x6ED9EBA1),
            40..=59 => ((b & c) | (b & d) | (c & d), 0x8F1BBCDC),
            60..=79 => (b ^ c ^ d, 0xCA62C1D6),
            _ => unreachable!(),
        };

        let temp = a.rotate_left(5)
            .wrapping_add(f)
            .wrapping_add(e)
            .wrapping_add(k)
            .wrapping_add(w[i]);
        e = d;
        d = c;
        c = b.rotate_left(30);
        b = a;
        a = temp;
    }

    state[0] = state[0].wrapping_add(a);
    state[1] = state[1].wrapping_add(b);
    state[2] = state[2].wrapping_add(c);
    state[3] = state[3].wrapping_add(d);
    state[4] = state[4].wrapping_add(e);
}