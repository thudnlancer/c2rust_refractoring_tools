use std::convert::TryInto;

const SHA512_DIGEST_LENGTH: usize = 8;
const SHA512_BLOCK_SIZE: usize = 128;
const SHA384_DIGEST_SIZE: usize = 48;
const SHA224_DIGEST_SIZE: usize = 28;
const SHA256_DIGEST_SIZE: usize = 32;

const K: [u64; 80] = [
    0x428A2F98D728AE22, 0x7137449123EF65CD,
    0xB5C0FBCFEC4D3B2F, 0xE9B5DBA58189DBBC,
    0x3956C25BF348B538, 0x59F111F1B605D019,
    0x923F82A4AF194F9B, 0xAB1C5ED5DA6D8118,
    0xD807AA98A3030242, 0x12835B0145706FBE,
    0x243185BE4EE4B28C, 0x550C7DC3D5FFB4E2,
    0x72BE5D74F27B896F, 0x80DEB1FE3B1696B1,
    0x9BDC06A725C71235, 0xC19BF174CF692694,
    0xE49B69C19EF14AD2, 0xEFBE4786384F25E3,
    0x0FC19DC68B8CD5B5, 0x240CA1CC77AC9C65,
    0x2DE92C6F592B0275, 0x4A7484AA6EA6E483,
    0x5CB0A9DCBD41FBD4, 0x76F988DA831153B5,
    0x983E5152EE66DFAB, 0xA831C66D2DB43210,
    0xB00327C898FB213F, 0xBF597FC7BEEF0EE4,
    0xC6E00BF33DA88FC2, 0xD5A79147930AA725,
    0x06CA6351E003826F, 0x142929670A0E6E70,
    0x27B70A8546D22FFC, 0x2E1B21385C26C926,
    0x4D2C6DFC5AC42AED, 0x53380D139D95B3DF,
    0x650A73548BAF63DE, 0x766A0ABB3C77B2A8,
    0x81C2C92E47EDAEE6, 0x92722C851482353B,
    0xA2BFE8A14CF10364, 0xA81A664BBC423001,
    0xC24B8B70D0F89791, 0xC76C51A30654BE30,
    0xD192E819D6EF5218, 0xD69906245565A910,
    0xF40E35855771202A, 0x106AA07032BBD1B8,
    0x19A4C116B8D2D0C8, 0x1E376C085141AB53,
    0x2748774CDF8EEB99, 0x34B0BCB5E19B48A8,
    0x391C0CB3C5C95A63, 0x4ED8AA4AE3418ACB,
    0x5B9CCA4F7763E373, 0x682E6FF3D6B2B8A3,
    0x748F82EE5DEFB2FC, 0x78A5636F43172F60,
    0x84C87814A1F0AB72, 0x8CC702081A6439EC,
    0x90BEFFFA23631E28, 0xA4506CEBDE82BDE9,
    0xBEF9A3F7B2C67915, 0xC67178F2E372532B,
    0xCA273ECEEA26619C, 0xD186B8C721C0C207,
    0xEADA7DD6CDE0EB1E, 0xF57D4F7FEE6ED178,
    0x06F067AA72176FBA, 0x0A637DC5A2C898A6,
    0x113F9804BEF90DAE, 0x1B710B35131C471B,
    0x28DB77F523047D84, 0x32CAAB7B40C72493,
    0x3C9EBE0A15C9BEBC, 0x431D67C49C100D4C,
    0x4CC5D4BECB3E42B6, 0x597F299CFC657E2A,
    0x5FCB6FAB3AD6FAEC, 0x6C44198C4A475817,
];

#[derive(Debug, Clone)]
pub struct Sha512Ctx {
    state: [u64; SHA512_DIGEST_LENGTH],
    count_low: u64,
    count_high: u64,
    index: usize,
    block: [u8; SHA512_BLOCK_SIZE],
}

impl Default for Sha512Ctx {
    fn default() -> Self {
        Self::new()
    }
}

impl Sha512Ctx {
    pub fn new() -> Self {
        let h0 = [
            0x6A09E667F3BCC908,
            0xBB67AE8584CAA73B,
            0x3C6EF372FE94F82B,
            0xA54FF53A5F1D36F1,
            0x510E527FADE682D1,
            0x9B05688C2B3E6C1F,
            0x1F83D9ABFB41BD6B,
            0x5BE0CD19137E2179,
        ];

        Self {
            state: h0,
            count_low: 0,
            count_high: 0,
            index: 0,
            block: [0; SHA512_BLOCK_SIZE],
        }
    }

    pub fn update(&mut self, data: &[u8]) {
        let mut length = data.len();
        let mut offset = 0;

        while length > 0 {
            let space = SHA512_BLOCK_SIZE - self.index;
            let copy_len = if length > space { space } else { length };

            self.block[self.index..self.index + copy_len]
                .copy_from_slice(&data[offset..offset + copy_len]);

            self.index += copy_len;
            offset += copy_len;
            length -= copy_len;

            if self.index == SHA512_BLOCK_SIZE {
                sha512_compress(&mut self.state, &self.block);
                self.index = 0;

                let (low, high) = self.count_low.overflowing_add((SHA512_BLOCK_SIZE as u64) << 3);
                self.count_low = low;
                self.count_high += high as u64;
            }
        }
    }

    pub fn digest(&mut self, length: usize) -> Vec<u8> {
        assert!(length <= SHA512_DIGEST_SIZE * 8);

        self.pad();
        self.finalize();

        let mut digest = Vec::with_capacity(length);
        let words = length / 8;
        let leftover = length % 8;

        for i in 0..words {
            digest.extend_from_slice(&self.state[i].to_be_bytes());
        }

        if leftover > 0 {
            let word = self.state[words] >> (8 * (8 - leftover));
            let mut bytes = word.to_be_bytes();
            digest.extend_from_slice(&bytes[8 - leftover..8]);
        }

        self.reset();
        digest
    }

    fn pad(&mut self) {
        let pad_len = if self.index < 112 {
            112 - self.index
        } else {
            240 - self.index
        };

        let padding = [0u8; 128];
        self.update(&padding[..pad_len]);
    }

    fn finalize(&mut self) {
        let high = (self.count_high << 10) | (self.count_low >> 54);
        let low = (self.count_low << 10) | ((self.index as u64) << 3);

        let mut block = [0u8; SHA512_BLOCK_SIZE];
        block[..16].copy_from_slice(&high.to_be_bytes());
        block[16..24].copy_from_slice(&low.to_be_bytes());

        sha512_compress(&mut self.state, &block);
    }

    fn reset(&mut self) {
        *self = Self::new();
    }
}

pub struct Sha384Ctx(Sha512Ctx);

impl Sha384Ctx {
    pub fn new() -> Self {
        let h0 = [
            0xCBBB9D5DC1059ED8,
            0x629A292A367CD507,
            0x9159015A3070DD17,
            0x152FECD8F70E5939,
            0x67332667FFC00B31,
            0x8EB44A8768581511,
            0xDB0C2E0D64F98FA7,
            0x47B5481DBEFA4FA4,
        ];

        let mut ctx = Sha512Ctx::new();
        ctx.state = h0;
        Self(ctx)
    }

    pub fn update(&mut self, data: &[u8]) {
        self.0.update(data);
    }

    pub fn digest(&mut self, length: usize) -> Vec<u8> {
        assert!(length <= SHA384_DIGEST_SIZE);
        self.0.digest(length)
    }
}

pub struct Sha512_224Ctx(Sha512Ctx);

impl Sha512_224Ctx {
    pub fn new() -> Self {
        let h0 = [
            0x8c3d37c819544da2,
            0x73e1996689dcd4d6,
            0x1dfab7ae32ff9c82,
            0x679dd514582f9fcf,
            0x0f6d2b697bd44da8,
            0x77e36f7304c48942,
            0x3f9d85a86a1d36c8,
            0x1112e6ad91d692a1,
        ];

        let mut ctx = Sha512Ctx::new();
        ctx.state = h0;
        Self(ctx)
    }

    pub fn update(&mut self, data: &[u8]) {
        self.0.update(data);
    }

    pub fn digest(&mut self, length: usize) -> Vec<u8> {
        assert!(length <= SHA224_DIGEST_SIZE);
        self.0.digest(length)
    }
}

pub struct Sha512_256Ctx(Sha512Ctx);

impl Sha512_256Ctx {
    pub fn new() -> Self {
        let h0 = [
            0x22312194fc2bf72c,
            0x9f555fa3c84c64c2,
            0x2393b86b6f53b151,
            0x963877195940eabd,
            0x96283ee2a88effe3,
            0xbe5e1e2553863992,
            0x2b0199fc2c85b8aa,
            0x0eb72ddc81c52ca2,
        ];

        let mut ctx = Sha512Ctx::new();
        ctx.state = h0;
        Self(ctx)
    }

    pub fn update(&mut self, data: &[u8]) {
        self.0.update(data);
    }

    pub fn digest(&mut self, length: usize) -> Vec<u8> {
        assert!(length <= SHA256_DIGEST_SIZE);
        self.0.digest(length)
    }
}

fn sha512_compress(state: &mut [u64; SHA512_DIGEST_LENGTH], input: &[u8; SHA512_BLOCK_SIZE]) {
    // Implementation of the SHA-512 compression function would go here
    // This is a placeholder as the actual implementation is quite complex
    // and would require significant additional code
    unimplemented!("SHA-512 compression function not implemented");
}