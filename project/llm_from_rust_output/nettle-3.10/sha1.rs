use std::convert::TryInto;

const SHA1_DIGEST_SIZE: usize = 20;
const BLOCK_SIZE: usize = 64;

#[derive(Copy, Clone)]
pub struct Sha1Context {
    state: [u32; 5],
    count: u64,
    index: usize,
    block: [u8; BLOCK_SIZE],
}

impl Sha1Context {
    pub fn new() -> Self {
        Sha1Context {
            state: [
                0x67452301,
                0xefcdab89,
                0x98badcfe,
                0x10325476,
                0xc3d2e1f0,
            ],
            count: 0,
            index: 0,
            block: [0; BLOCK_SIZE],
        }
    }

    pub fn update(&mut self, data: &[u8]) {
        if data.is_empty() {
            return;
        }

        let mut data = data;
        let mut length = data.len();

        if self.index != 0 {
            let left = BLOCK_SIZE - self.index;
            if length < left {
                self.block[self.index..self.index + length].copy_from_slice(data);
                self.index += length;
                return;
            } else {
                self.block[self.index..self.index + left].copy_from_slice(&data[..left]);
                self.compress(&self.block);
                self.count += 1;
                data = &data[left..];
                length -= left;
            }
        }

        while length >= BLOCK_SIZE {
            self.compress(&data[..BLOCK_SIZE]);
            self.count += 1;
            data = &data[BLOCK_SIZE..];
            length -= BLOCK_SIZE;
        }

        self.block[..length].copy_from_slice(data);
        self.index = length;
    }

    pub fn digest(&mut self, output: &mut [u8]) {
        assert!(output.len() <= SHA1_DIGEST_SIZE);

        let mut block = self.block;
        let index = self.index;

        block[index] = 0x80;
        if index > BLOCK_SIZE - 8 {
            block[index + 1..].fill(0);
            self.compress(&block);
            block.fill(0);
        } else {
            block[index + 1..BLOCK_SIZE - 8].fill(0);
        }

        let bit_count = (self.count << 9) | ((self.index << 3) as u64);
        block[BLOCK_SIZE - 8..].copy_from_slice(&bit_count.to_be_bytes());

        self.compress(&block);

        for (i, &word) in self.state.iter().enumerate() {
            if i * 4 < output.len() {
                let bytes = word.to_be_bytes();
                let len = std::cmp::min(4, output.len() - i * 4);
                output[i * 4..i * 4 + len].copy_from_slice(&bytes[..len]);
            }
        }

        *self = Sha1Context::new();
    }

    fn compress(&mut self, block: &[u8]) {
        assert!(block.len() == BLOCK_SIZE);
        let mut state = self.state;
        nettle_sha1_compress(&mut state, block);
        self.state = state;
    }
}

fn nettle_sha1_compress(state: &mut [u32; 5], block: &[u8]) {
    // Implementation of SHA-1 compression function would go here
    // This is left as an exercise or would be imported from a safe SHA-1 crate
    unimplemented!("SHA-1 compression function implementation required");
}