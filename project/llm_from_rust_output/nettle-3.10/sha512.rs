use std::mem;

const BLOCK_SIZE: usize = 128;
const DIGEST_SIZE: usize = 64;
const SHA384_DIGEST_SIZE: usize = 48;
const SHA224_DIGEST_SIZE: usize = 28;
const SHA256_DIGEST_SIZE: usize = 32;

const K: [u64; 80] = [
    0x428a2f98d728ae22, 0x7137449123ef65cd, 0xb5c0fbcfec4d3b2f, 0xe9b5dba58189dbbc,
    0x3956c25bf348b538, 0x59f111f1b605d019, 0x923f82a4af194f9b, 0xab1c5ed5da6d8118,
    0xd807aa98a3030242, 0x12835b0145706fbe, 0x243185be4ee4b28c, 0x550c7dc3d5ffb4e2,
    0x72be5d74f27b896f, 0x80deb1fe3b1696b1, 0x9bdc06a725c71235, 0xc19bf174cf692694,
    0xe49b69c19ef14ad2, 0xefbe4786384f25e3, 0x0fc19dc68b8cd5b5, 0x240ca1cc77ac9c65,
    0x2de92c6f592b0275, 0x4a7484aa6ea6e483, 0x5cb0a9dcbd41fbd4, 0x76f988da831153b5,
    0x983e5152ee66dfab, 0xa831c66d2db43210, 0xb00327c898fb213f, 0xbf597fc7beef0ee4,
    0xc6e00bf33da88fc2, 0xd5a79147930aa725, 0x06ca6351e003826f, 0x142929670a0e6e70,
    0x27b70a8546d22ffc, 0x2e1b21385c26c926, 0x4d2c6dfc5ac42aed, 0x53380d139d95b3df,
    0x650a73548baf63de, 0x766a0abb3c77b2a8, 0x81c2c92e47edaee6, 0x92722c851482353b,
    0xa2bfe8a14cf10364, 0xa81a664bbc423001, 0xc24b8b70d0f89791, 0xc76c51a30654be30,
    0xd192e819d6ef5218, 0xd69906245565a910, 0xf40e35855771202a, 0x106aa07032bbd1b8,
    0x19a4c116b8d2d0c8, 0x1e376c085141ab53, 0x2748774cdf8eeb99, 0x34b0bcb5e19b48a8,
    0x391c0cb3c5c95a63, 0x4ed8aa4ae3418acb, 0x5b9cca4f7763e373, 0x682e6ff3d6b2b8a3,
    0x748f82ee5defb2fc, 0x78a5636f43172f60, 0x84c87814a1f0ab72, 0x8cc702081a6439ec,
    0x90befffa23631e28, 0xa4506cebde82bde9, 0xbef9a3f7b2c67915, 0xc67178f2e372532b,
    0xca273eceea26619c, 0xd186b8c721c0c207, 0xeada7dd6cde0eb1e, 0xf57d4f7fee6ed178,
    0x06f067aa72176fba, 0x0a637dc5a2c898a6, 0x113f9804bef90dae, 0x1b710b35131c471b,
    0x28db77f523047d84, 0x32caab7b40c72493, 0x3c9ebe0a15c9bebc, 0x431d67c49c100d4c,
    0x4cc5d4becb3e42b6, 0x597f299cfc657e2a, 0x5fcb6fab3ad6faec, 0x6c44198c4a475817,
];

#[derive(Clone)]
pub struct Sha512Ctx {
    state: [u64; 8],
    count_low: u64,
    count_high: u64,
    index: usize,
    block: [u8; BLOCK_SIZE],
}

impl Sha512Ctx {
    pub fn new() -> Self {
        Sha512Ctx {
            state: [0; 8],
            count_low: 0,
            count_high: 0,
            index: 0,
            block: [0; BLOCK_SIZE],
        }
    }

    pub fn init(&mut self, initial_state: &[u64; 8]) {
        self.state.copy_from_slice(initial_state);
        self.count_low = 0;
        self.count_high = 0;
        self.index = 0;
    }

    pub fn update(&mut self, data: &[u8]) {
        let mut i = 0;
        while i < data.len() {
            if self.index != 0 {
                let remaining = BLOCK_SIZE - self.index;
                let to_copy = remaining.min(data.len() - i);
                self.block[self.index..self.index + to_copy].copy_from_slice(&data[i..i + to_copy]);
                self.index += to_copy;
                i += to_copy;

                if self.index == BLOCK_SIZE {
                    self.compress();
                    self.index = 0;
                }
            } else {
                let remaining_blocks = (data.len() - i) / BLOCK_SIZE;
                for _ in 0..remaining_blocks {
                    self.block.copy_from_slice(&data[i..i + BLOCK_SIZE]);
                    self.compress();
                    i += BLOCK_SIZE;
                }

                let remaining = data.len() - i;
                if remaining > 0 {
                    self.block[..remaining].copy_from_slice(&data[i..]);
                    self.index = remaining;
                    i += remaining;
                }
            }
        }
    }

    pub fn digest(&mut self, length: usize, output: &mut [u8]) {
        assert!(length <= DIGEST_SIZE);
        self.write_digest(length, output);
        self.init(&self.state);
    }

    fn write_digest(&mut self, length: usize, digest: &mut [u8]) {
        assert!(length <= DIGEST_SIZE);

        let mut block = self.block;
        block[self.index] = 0x80;
        block[self.index + 1..].fill(0);

        if self.index > BLOCK_SIZE - 16 {
            self.compress();
            block.fill(0);
        }

        let bit_count = (self.count_high << 10) | (self.count_low >> 54);
        let bit_count_low = (self.count_low << 10) | ((self.index << 3) as u64);

        block[BLOCK_SIZE - 16..BLOCK_SIZE - 8].copy_from_slice(&bit_count.to_be_bytes());
        block[BLOCK_SIZE - 8..].copy_from_slice(&bit_count_low.to_be_bytes());

        self.compress();

        let words = length / 8;
        let leftover = length % 8;

        for i in 0..words {
            digest[i * 8..(i + 1) * 8].copy_from_slice(&self.state[i].to_be_bytes());
        }

        if leftover > 0 {
            let word = self.state[words] >> (8 * (8 - leftover));
            digest[words * 8..words * 8 + leftover].copy_from_slice(&word.to_be_bytes()[..leftover]);
        }
    }

    fn compress(&mut self) {
        let mut w = [0u64; 80];
        for i in 0..16 {
            w[i] = u64::from_be_bytes([
                self.block[i * 8],
                self.block[i * 8 + 1],
                self.block[i * 8 + 2],
                self.block[i * 8 + 3],
                self.block[i * 8 + 4],
                self.block[i * 8 + 5],
                self.block[i * 8 + 6],
                self.block[i * 8 + 7],
            ]);
        }

        for i in 16..80 {
            let s0 = w[i - 15].rotate_right(1) ^ w[i - 15].rotate_right(8) ^ (w[i - 15] >> 7);
            let s1 = w[i - 2].rotate_right(19) ^ w[i - 2].rotate_right(61) ^ (w[i - 2] >> 6);
            w[i] = w[i - 16]
                .wrapping_add(s0)
                .wrapping_add(w[i - 7])
                .wrapping_add(s1);
        }

        let mut a = self.state[0];
        let mut b = self.state[1];
        let mut c = self.state[2];
        let mut d = self.state[3];
        let mut e = self.state[4];
        let mut f = self.state[5];
        let mut g = self.state[6];
        let mut h = self.state[7];

        for i in 0..80 {
            let s1 = e.rotate_right(14) ^ e.rotate_right(18) ^ e.rotate_right(41);
            let ch = (e & f) ^ (!e & g);
            let temp1 = h
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(K[i])
                .wrapping_add(w[i]);
            let s0 = a.rotate_right(28) ^ a.rotate_right(34) ^ a.rotate_right(39);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = s0.wrapping_add(maj);

            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);
        }

        self.state[0] = self.state[0].wrapping_add(a);
        self.state[1] = self.state[1].wrapping_add(b);
        self.state[2] = self.state[2].wrapping_add(c);
        self.state[3] = self.state[3].wrapping_add(d);
        self.state[4] = self.state[4].wrapping_add(e);
        self.state[5] = self.state[5].wrapping_add(f);
        self.state[6] = self.state[6].wrapping_add(g);
        self.state[7] = self.state[7].wrapping_add(h);

        self.count_low = self.count_low.wrapping_add(1);
        if self.count_low == 0 {
            self.count_high = self.count_high.wrapping_add(1);
        }
    }
}

pub fn sha512_init(ctx: &mut Sha512Ctx) {
    const H0: [u64; 8] = [
        0x6a09e667f3bcc908,
        0xbb67ae8584caa73b,
        0x3c6ef372fe94f82b,
        0xa54ff53a5f1d36f1,
        0x510e527fade682d1,
        0x9b05688c2b3e6c1f,
        0x1f83d9abfb41bd6b,
        0x5be0cd19137e2179,
    ];
    ctx.init(&H0);
}

pub fn sha384_init(ctx: &mut Sha512Ctx) {
    const H0: [u64; 8] = [
        0xcbbb9d5dc1059ed8,
        0x629a292a367cd507,
        0x9159015a3070dd17,
        0x152fecd8f70e5939,
        0x67332667ffc00b31,
        0x8eb44a8768581511,
        0xdb0c2e0d64f98fa7,
        0x47b5481dbefa4fa4,
    ];
    ctx.init(&H0);
}

pub fn sha512_224_init(ctx: &mut Sha512Ctx) {
    const H0: [u64; 8] = [
        0x8c3d37c819544da2,
        0x73e1996689dcd4d6,
        0x1dfab7ae32ff9c82,
        0x679dd514582f9fcf,
        0x0f6d2b697bd44da8,
        0x77e36f7304c48942,
        0x3f9d85a86a1d36c8,
        0x1112e6ad91d692a1,
    ];
    ctx.init(&H0);
}

pub fn sha512_256_init(ctx: &mut Sha512Ctx) {
    const H0: [u64; 8] = [
        0x22312194fc2bf72c,
        0x9f555fa3c84c64c2,
        0x2393b86b6f53b151,
        0x963877195940eabd,
        0x96283ee2a88effe3,
        0xbe5e1e2553863992,
        0x2b0199fc2c85b8aa,
        0x0eb72ddc81c52ca2,
    ];
    ctx.init(&H0);
}

pub fn sha512_digest(ctx: &mut Sha512Ctx, length: usize, digest: &mut [u8]) {
    assert!(length <= DIGEST_SIZE);
    ctx.digest(length, digest);
}

pub fn sha384_digest(ctx: &mut Sha512Ctx, length: usize, digest: &mut [u8]) {
    assert!(length <= SHA384_DIGEST_SIZE);
    ctx.digest(length, digest);
}

pub fn sha512_224_digest(ctx: &mut Sha512Ctx, length: usize, digest: &mut [u8]) {
    assert!(length <= SHA224_DIGEST_SIZE);
    ctx.digest(length, digest);
}

pub fn sha512_256_digest(ctx: &mut Sha512Ctx, length: usize, digest: &mut [u8]) {
    assert!(length <= SHA256_DIGEST_SIZE);
    ctx.digest(length, digest);
}