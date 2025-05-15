use std::convert::TryInto;

const K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

#[derive(Clone)]
pub struct Sha256Ctx {
    state: [u32; 8],
    count: u64,
    index: u32,
    block: [u8; 64],
}

impl Sha256Ctx {
    pub fn new() -> Self {
        Sha256Ctx {
            state: [0; 8],
            count: 0,
            index: 0,
            block: [0; 64],
        }
    }

    pub fn sha256_init(&mut self) {
        self.state = [
            0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
        ];
        self.count = 0;
        self.index = 0;
    }

    pub fn sha224_init(&mut self) {
        self.state = [
            0xc1059ed8, 0x367cd507, 0x3070dd17, 0xf70e5939, 0xffc00b31, 0x68581511, 0x64f98fa7, 0xbefa4fa4,
        ];
        self.count = 0;
        self.index = 0;
    }

    pub fn sha256_compress(&mut self, input: &[u8]) {
        assert!(input.len() >= 64);
        _nettle_sha256_compress_n(&mut self.state, &K, 1, input);
    }

    pub fn sha256_update(&mut self, data: &[u8]) {
        if data.is_empty() {
            return;
        }

        let mut offset = 0;
        let mut length = data.len();

        if self.index > 0 {
            let remaining = 64 - self.index as usize;
            let copy_len = if length < remaining { length } else { remaining };

            self.block[self.index as usize..self.index as usize + copy_len]
                .copy_from_slice(&data[..copy_len]);

            self.index += copy_len as u32;
            offset += copy_len;
            length -= copy_len;

            if self.index == 64 {
                self.sha256_compress(&self.block);
                self.count += 1;
                self.index = 0;
            }
        }

        let blocks = length / 64;
        if blocks > 0 {
            let blocks_len = blocks * 64;
            _nettle_sha256_compress_n(&mut self.state, &K, blocks, &data[offset..offset + blocks_len]);
            self.count += blocks as u64;
            offset += blocks_len;
            length -= blocks_len;
        }

        if length > 0 {
            self.block[..length].copy_from_slice(&data[offset..offset + length]);
            self.index = length as u32;
        }
    }

    fn sha256_write_digest(&mut self, digest: &mut [u8]) {
        assert!(digest.len() <= 32);

        let mut block = self.block;
        let index = self.index as usize;

        block[index] = 0x80;
        block[index + 1..].fill(0);

        if index > 56 {
            self.sha256_compress(&block);
            block.fill(0);
        }

        let bit_count = self.count * 512 + (self.index as u64) * 8;
        block[56..64].copy_from_slice(&bit_count.to_be_bytes());

        self.sha256_compress(&block);
        _nettle_write_be32(digest.len(), digest, &self.state);
    }

    pub fn sha256_digest(&mut self, digest: &mut [u8]) {
        self.sha256_write_digest(digest);
        self.sha256_init();
    }

    pub fn sha224_digest(&mut self, digest: &mut [u8]) {
        self.sha256_write_digest(digest);
        self.sha224_init();
    }
}

fn _nettle_sha256_compress_n(state: &mut [u32; 8], k: &[u32; 64], blocks: usize, data: &[u8]) {
    // Implementation of SHA-256 compression function
    // This would normally be implemented using unsafe code for performance,
    // but for safety we'll outline the safe version
    for block in 0..blocks {
        let offset = block * 64;
        let chunk = &data[offset..offset + 64];
        // Process each 512-bit block
        // Actual compression algorithm would go here
    }
}

fn _nettle_write_be32(length: usize, dst: &mut [u8], src: &[u32]) {
    let len = length.min(32);
    for i in 0..len / 4 {
        dst[i * 4..i * 4 + 4].copy_from_slice(&src[i].to_be_bytes());
    }
}