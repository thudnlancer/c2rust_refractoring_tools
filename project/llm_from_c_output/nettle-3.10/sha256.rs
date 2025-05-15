use std::mem;

const SHA256_DIGEST_SIZE: usize = 32;
const SHA256_BLOCK_SIZE: usize = 64;

const K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 
    0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5, 
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 
    0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174, 
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 
    0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 
    0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 
    0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 
    0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 
    0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3, 
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 
    0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

#[derive(Clone)]
pub struct Sha256Ctx {
    state: [u32; 8],
    count: u64,
    index: usize,
    block: [u8; SHA256_BLOCK_SIZE],
}

fn sha256_compress(state: &mut [u32; 8], input: &[u8]) {
    nettle_sha256_compress_n(state, &K, 1, input);
}

fn nettle_sha256_compress_n(state: &mut [u32; 8], k: &[u32; 64], blocks: usize, input: &[u8]) -> &[u8] {
    // Implementation of the compression function would go here
    // This is a placeholder for the actual implementation
    &input[blocks * SHA256_BLOCK_SIZE..]
}

fn write_be32(length: usize, digest: &mut [u8], state: &[u32; 8]) {
    for (i, &word) in state.iter().enumerate().take(length / 4) {
        digest[i*4..i*4+4].copy_from_slice(&word.to_be_bytes());
    }
}

impl Sha256Ctx {
    pub fn new() -> Self {
        let h0 = [
            0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 
            0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
        ];

        Sha256Ctx {
            state: h0,
            count: 0,
            index: 0,
            block: [0; SHA256_BLOCK_SIZE],
        }
    }

    pub fn update(&mut self, data: &[u8]) {
        let mut length = data.len();
        if length == 0 {
            return;
        }

        let mut data_ptr = data;

        if self.index > 0 {
            let fill = std::cmp::min(SHA256_BLOCK_SIZE - self.index, length);
            self.block[self.index..self.index + fill].copy_from_slice(&data_ptr[..fill]);
            data_ptr = &data_ptr[fill..];
            length -= fill;
            self.index += fill;

            if self.index == SHA256_BLOCK_SIZE {
                sha256_compress(&mut self.state, &self.block);
                self.count += 1;
                self.index = 0;
            }
        }

        let blocks = length / SHA256_BLOCK_SIZE;
        data_ptr = nettle_sha256_compress_n(&mut self.state, &K, blocks, data_ptr);
        self.count += blocks as u64;
        let remaining = length % SHA256_BLOCK_SIZE;

        if remaining > 0 {
            self.block[..remaining].copy_from_slice(&data_ptr[..remaining]);
            self.index = remaining;
        }
    }

    fn pad(&mut self) {
        let index = self.index;
        let pad_len = if index < 56 { 56 - index } else { 120 - index };

        self.block[index] = 0x80;
        self.block[index + 1..index + pad_len].fill(0);

        if pad_len < 8 {
            sha256_compress(&mut self.state, &self.block);
            self.block[..56].fill(0);
        }

        let bit_count = (self.count << 9) | ((self.index as u64) << 3);
        self.block[56..64].copy_from_slice(&bit_count.to_be_bytes());
    }

    pub fn digest(&mut self, digest: &mut [u8]) {
        assert!(digest.len() <= SHA256_DIGEST_SIZE);

        self.pad();
        sha256_compress(&mut self.state, &self.block);
        write_be32(digest.len(), digest, &self.state);
        *self = Self::new();
    }
}

pub struct Sha224Ctx(Sha256Ctx);

impl Sha224Ctx {
    pub fn new() -> Self {
        let h0 = [
            0xc1059ed8, 0x367cd507, 0x3070dd17, 0xf70e5939,
            0xffc00b31, 0x68581511, 0x64f98fa7, 0xbefa4fa4,
        ];

        Sha224Ctx(Sha256Ctx {
            state: h0,
            count: 0,
            index: 0,
            block: [0; SHA256_BLOCK_SIZE],
        })
    }

    pub fn update(&mut self, data: &[u8]) {
        self.0.update(data);
    }

    pub fn digest(&mut self, digest: &mut [u8]) {
        assert!(digest.len() <= 28); // SHA-224 produces 28-byte digests
        self.0.pad();
        sha256_compress(&mut self.0.state, &self.0.block);
        write_be32(digest.len(), digest, &self.0.state);
        *self = Self::new();
    }
}