use std::convert::TryInto;
use std::mem::size_of;

const RIPEMD160_DIGEST_SIZE: usize = 20;
const RIPEMD160_BLOCK_SIZE: usize = 64;
const _RIPEMD160_DIGEST_LENGTH: usize = 5;

pub struct Ripemd160Ctx {
    state: [u32; _RIPEMD160_DIGEST_LENGTH],
    count: u64,
    index: usize,
    block: [u8; RIPEMD160_BLOCK_SIZE],
}

impl Ripemd160Ctx {
    pub fn new() -> Self {
        let iv = [
            0x67452301,
            0xEFCDAB89,
            0x98BADCFE,
            0x10325476,
            0xC3D2E1F0,
        ];
        
        Ripemd160Ctx {
            state: iv,
            count: 0,
            index: 0,
            block: [0; RIPEMD160_BLOCK_SIZE],
        }
    }

    pub fn update(&mut self, data: &[u8]) {
        let mut length = data.len();
        let mut offset = 0;

        while length > 0 {
            let space = RIPEMD160_BLOCK_SIZE - self.index;
            let copy_len = std::cmp::min(length, space);

            self.block[self.index..self.index + copy_len]
                .copy_from_slice(&data[offset..offset + copy_len]);

            self.index += copy_len;
            offset += copy_len;
            length -= copy_len;

            if self.index == RIPEMD160_BLOCK_SIZE {
                compress(&mut self.state, &self.block);
                self.count += 1;
                self.index = 0;
            }
        }
    }

    pub fn digest(mut self, digest: &mut [u8]) -> Result<(), &'static str> {
        if digest.len() < RIPEMD160_DIGEST_SIZE {
            return Err("Digest buffer too small");
        }

        // Pad the message
        let pad_len = if self.index < 56 {
            56 - self.index
        } else {
            120 - self.index
        };

        let pad = [0x80u8];
        self.update(&pad);
        self.update(&vec![0u8; pad_len - 1]);

        // Append the bit count
        let bit_count = (self.count << 9) | (self.index << 3) as u64;
        let bit_count_bytes = bit_count.to_le_bytes();
        self.update(&bit_count_bytes);

        // Write the digest
        for (i, &word) in self.state.iter().enumerate() {
            digest[i * 4..(i + 1) * 4].copy_from_slice(&word.to_le_bytes());
        }

        Ok(())
    }
}

fn compress(state: &mut [u32; _RIPEMD160_DIGEST_LENGTH], block: &[u8; RIPEMD160_BLOCK_SIZE]) {
    // Implementation of the RIPEMD-160 compression function
    // This would contain the actual algorithm implementation
    // For brevity, we're omitting the full implementation here
    // but it would mirror the C version's compression logic
    unimplemented!("RIPEMD-160 compression function needs to be implemented");
}