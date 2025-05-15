use std::mem;

const RIPEMD160_DIGEST_SIZE: usize = 20;
const BLOCK_SIZE: usize = 64;

#[derive(Copy, Clone)]
pub struct Ripemd160Ctx {
    state: [u32; 5],
    count: u64,
    index: usize,
    block: [u8; BLOCK_SIZE],
}

impl Ripemd160Ctx {
    pub fn new() -> Self {
        let mut ctx = Ripemd160Ctx {
            state: [0; 5],
            count: 0,
            index: 0,
            block: [0; BLOCK_SIZE],
        };
        ctx.init();
        ctx
    }

    fn init(&mut self) {
        self.state = [
            0x67452301,
            0xefcdab89,
            0x98badcfe,
            0x10325476,
            0xc3d2e1f0,
        ];
        self.count = 0;
        self.index = 0;
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
            } else {
                let (left, right) = data.split_at(remaining);
                self.block[self.index..].copy_from_slice(left);
                self.compress();
                self.count += 1;
                self.update(right);
                return;
            }
        }

        let mut chunks = data.chunks_exact(BLOCK_SIZE);
        for chunk in chunks.by_ref() {
            self.state = compress(self.state, chunk);
            self.count += 1;
        }

        let remainder = chunks.remainder();
        if !remainder.is_empty() {
            self.block[..remainder.len()].copy_from_slice(remainder);
            self.index = remainder.len();
        }
    }

    pub fn digest(&mut self, output: &mut [u8]) {
        assert!(output.len() <= RIPEMD160_DIGEST_SIZE);

        let mut block = self.block;
        let index = self.index;

        block[index] = 0x80;
        if index > BLOCK_SIZE - 8 {
            block[index + 1..].fill(0);
            self.compress();
            block.fill(0);
        } else {
            block[index + 1..BLOCK_SIZE - 8].fill(0);
        }

        let bit_count = self.count * 512 + (self.index as u64) * 8;
        block[56..].copy_from_slice(&bit_count.to_le_bytes());

        self.compress();

        for (i, &word) in self.state.iter().enumerate() {
            let bytes = word.to_le_bytes();
            let start = i * 4;
            let end = start + 4;
            if end <= output.len() {
                output[start..end].copy_from_slice(&bytes);
            } else if start < output.len() {
                output[start..].copy_from_slice(&bytes[..output.len() - start]);
            }
        }

        self.init();
    }

    fn compress(&mut self) {
        self.state = compress(self.state, &self.block);
    }
}

fn compress(state: [u32; 5], block: &[u8]) -> [u32; 5] {
    // This would be implemented using the actual RIPEMD-160 compression function
    // For now we'll just return the state unchanged as a placeholder
    state
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let ctx = Ripemd160Ctx::new();
        assert_eq!(ctx.state, [0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476, 0xc3d2e1f0]);
        assert_eq!(ctx.count, 0);
        assert_eq!(ctx.index, 0);
    }

    #[test]
    fn test_update() {
        let mut ctx = Ripemd160Ctx::new();
        ctx.update(b"test");
        assert_eq!(ctx.index, 4);
    }
}