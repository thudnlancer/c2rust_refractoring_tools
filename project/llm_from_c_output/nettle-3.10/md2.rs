// md2.rs

use std::mem;

pub const MD2_DIGEST_SIZE: usize = 16;
pub const MD2_BLOCK_SIZE: usize = 16;
pub const MD2_DATA_SIZE: usize = MD2_BLOCK_SIZE;

const S: [u8; 256] = [
    41, 46, 67, 201, 162, 216, 124, 1, 61, 54, 84, 161, 236, 240, 6,
    19, 98, 167, 5, 243, 192, 199, 115, 140, 152, 147, 43, 217, 188,
    76, 130, 202, 30, 155, 87, 60, 253, 212, 224, 22, 103, 66, 111, 24,
    138, 23, 229, 18, 190, 78, 196, 214, 218, 158, 222, 73, 160, 251,
    245, 142, 187, 47, 238, 122, 169, 104, 121, 145, 21, 178, 7, 63,
    148, 194, 16, 137, 11, 34, 95, 33, 128, 127, 93, 154, 90, 144, 50,
    39, 53, 62, 204, 231, 191, 247, 151, 3, 255, 25, 48, 179, 72, 165,
    181, 209, 215, 94, 146, 42, 172, 86, 170, 198, 79, 184, 56, 210,
    150, 164, 125, 182, 118, 252, 107, 226, 156, 116, 4, 241, 69, 157,
    112, 89, 100, 113, 135, 32, 134, 91, 207, 101, 230, 45, 168, 2, 27,
    96, 37, 173, 174, 176, 185, 246, 28, 70, 97, 105, 52, 64, 126, 15,
    85, 71, 163, 35, 221, 81, 175, 58, 195, 92, 249, 206, 186, 197,
    234, 38, 44, 83, 13, 110, 133, 40, 132, 9, 211, 223, 205, 244, 65,
    129, 77, 82, 106, 220, 55, 200, 108, 193, 171, 250, 36, 225, 123,
    8, 12, 189, 177, 74, 120, 136, 149, 139, 227, 99, 232, 109, 233,
    203, 213, 254, 59, 0, 29, 57, 242, 239, 183, 14, 102, 88, 208, 228,
    166, 119, 114, 248, 235, 117, 75, 10, 49, 68, 80, 180, 143, 237,
    31, 26, 219, 153, 141, 51, 159, 17, 131, 20
];

#[derive(Clone)]
pub struct Md2Ctx {
    c: [u8; MD2_BLOCK_SIZE],
    x: [u8; 3 * MD2_BLOCK_SIZE],
    index: usize,
    block: [u8; MD2_BLOCK_SIZE],
}

impl Md2Ctx {
    pub fn new() -> Self {
        Md2Ctx {
            c: [0; MD2_BLOCK_SIZE],
            x: [0; 3 * MD2_BLOCK_SIZE],
            index: 0,
            block: [0; MD2_BLOCK_SIZE],
        }
    }

    fn transform(&mut self, data: &[u8]) {
        assert_eq!(data.len(), MD2_BLOCK_SIZE);

        // Copy data to X[16..32]
        self.x[MD2_BLOCK_SIZE..2 * MD2_BLOCK_SIZE].copy_from_slice(data);

        // First loop
        let mut t = self.c[15];
        for i in 0..MD2_BLOCK_SIZE {
            self.x[2 * MD2_BLOCK_SIZE + i] = self.x[i] ^ self.x[MD2_BLOCK_SIZE + i];
            t = self.c[i] ^ S[(data[i] ^ t) as usize];
            self.c[i] = t;
        }

        // Second loop
        let mut t = 0;
        for i in 0..MD2_BLOCK_SIZE + 2 {
            t = (t + i) & 0xff;
            for j in 0..3 * MD2_BLOCK_SIZE {
                t = self.x[j] ^ S[t as usize];
                self.x[j] = t;
            }
        }
    }

    pub fn update(&mut self, data: &[u8]) {
        let mut length = data.len();
        let mut offset = 0;

        // Process any buffered data first
        if self.index > 0 {
            let available = MD2_BLOCK_SIZE - self.index;
            let to_copy = available.min(length);
            self.block[self.index..self.index + to_copy].copy_from_slice(&data[..to_copy]);
            self.index += to_copy;
            offset += to_copy;
            length -= to_copy;

            if self.index == MD2_BLOCK_SIZE {
                self.transform(&self.block);
                self.index = 0;
            }
        }

        // Process full blocks
        while length >= MD2_BLOCK_SIZE {
            self.transform(&data[offset..offset + MD2_BLOCK_SIZE]);
            offset += MD2_BLOCK_SIZE;
            length -= MD2_BLOCK_SIZE;
        }

        // Buffer any remaining data
        if length > 0 {
            self.block[..length].copy_from_slice(&data[offset..offset + length]);
            self.index = length;
        }
    }

    pub fn digest(&mut self, digest: &mut [u8]) -> Result<(), &'static str> {
        if digest.len() > MD2_DIGEST_SIZE {
            return Err("digest buffer too large");
        }

        // Pad the message
        let left = MD2_BLOCK_SIZE - self.index;
        self.block[self.index..].fill(left as u8);
        self.transform(&self.block);

        // Final transform with checksum
        self.transform(&self.c);

        // Copy the result
        digest.copy_from_slice(&self.x[..digest.len()]);

        // Reset the context
        *self = Self::new();

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_md2() {
        let mut ctx = Md2Ctx::new();
        let mut digest = [0u8; MD2_DIGEST_SIZE];

        // Test empty string
        ctx.digest(&mut digest).unwrap();
        assert_eq!(
            digest,
            [0x83, 0x50, 0xE5, 0xA3, 0xE2, 0x4C, 0x15, 0x3D, 0xF2, 0x27, 0x5C, 0x9F, 0x80, 0x69, 0x27, 0x73]
        );

        // Test "abc"
        let mut ctx = Md2Ctx::new();
        ctx.update(b"abc");
        ctx.digest(&mut digest).unwrap();
        assert_eq!(
            digest,
            [0xDA, 0x85, 0x3B, 0x0D, 0x3F, 0x88, 0xD9, 0x9B, 0x30, 0x28, 0x3A, 0x69, 0xE6, 0xDE, 0xD6, 0xBB]
        );
    }
}