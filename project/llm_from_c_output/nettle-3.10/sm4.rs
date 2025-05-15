// sm4.rs

use std::convert::TryInto;

pub const SM4_BLOCK_SIZE: usize = 16;
pub const SM4_KEY_SIZE: usize = 16;

#[derive(Clone)]
pub struct Sm4Ctx {
    rkey: [u32; 32],
}

const FK: [u32; 4] = [
    0xa3b1bac6, 0x56aa3350, 0x677d9197, 0xb27022dc
];

const CK: [u32; 32] = [
    0x00070e15, 0x1c232a31, 0x383f464d, 0x545b6269,
    0x70777e85, 0x8c939aa1, 0xa8afb6bd, 0xc4cbd2d9,
    0xe0e7eef5, 0xfc030a11, 0x181f262d, 0x343b4249,
    0x50575e65, 0x6c737a81, 0x888f969d, 0xa4abb2b9,
    0xc0c7ced5, 0xdce3eaf1, 0xf8ff060d, 0x141b2229,
    0x30373e45, 0x4c535a61, 0x686f767d, 0x848b9299,
    0xa0a7aeb5, 0xbcc3cad1, 0xd8dfe6ed, 0xf4fb0209,
    0x10171e25, 0x2c333a41, 0x484f565d, 0x646b7279
];

const SBOX: [u8; 256] = [
    0xd6, 0x90, 0xe9, 0xfe, 0xcc, 0xe1, 0x3d, 0xb7,
    0x16, 0xb6, 0x14, 0xc2, 0x28, 0xfb, 0x2c, 0x05,
    0x2b, 0x67, 0x9a, 0x76, 0x2a, 0xbe, 0x04, 0xc3,
    0xaa, 0x44, 0x13, 0x26, 0x49, 0x86, 0x06, 0x99,
    0x9c, 0x42, 0x50, 0xf4, 0x91, 0xef, 0x98, 0x7a,
    0x33, 0x54, 0x0b, 0x43, 0xed, 0xcf, 0xac, 0x62,
    0xe4, 0xb3, 0x1c, 0xa9, 0xc9, 0x08, 0xe8, 0x95,
    0x80, 0xdf, 0x94, 0xfa, 0x75, 0x8f, 0x3f, 0xa6,
    0x47, 0x07, 0xa7, 0xfc, 0xf3, 0x73, 0x17, 0xba,
    0x83, 0x59, 0x3c, 0x19, 0xe6, 0x85, 0x4f, 0xa8,
    0x68, 0x6b, 0x81, 0xb2, 0x71, 0x64, 0xda, 0x8b,
    0xf8, 0xeb, 0x0f, 0x4b, 0x70, 0x56, 0x9d, 0x35,
    0x1e, 0x24, 0x0e, 0x5e, 0x63, 0x58, 0xd1, 0xa2,
    0x25, 0x22, 0x7c, 0x3b, 0x01, 0x21, 0x78, 0x87,
    0xd4, 0x00, 0x46, 0x57, 0x9f, 0xd3, 0x27, 0x52,
    0x4c, 0x36, 0x02, 0xe7, 0xa0, 0xc4, 0xc8, 0x9e,
    0xea, 0xbf, 0x8a, 0xd2, 0x40, 0xc7, 0x38, 0xb5,
    0xa3, 0xf7, 0xf2, 0xce, 0xf9, 0x61, 0x15, 0xa1,
    0xe0, 0xae, 0x5d, 0xa4, 0x9b, 0x34, 0x1a, 0x55,
    0xad, 0x93, 0x32, 0x30, 0xf5, 0x8c, 0xb1, 0xe3,
    0x1d, 0xf6, 0xe2, 0x2e, 0x82, 0x66, 0xca, 0x60,
    0xc0, 0x29, 0x23, 0xab, 0x0d, 0x53, 0x4e, 0x6f,
    0xd5, 0xdb, 0x37, 0x45, 0xde, 0xfd, 0x8e, 0x2f,
    0x03, 0xff, 0x6a, 0x72, 0x6d, 0x6c, 0x5b, 0x51,
    0x8d, 0x1b, 0xaf, 0x92, 0xbb, 0xdd, 0xbc, 0x7f,
    0x11, 0xd9, 0x5c, 0x41, 0x1f, 0x10, 0x5a, 0xd8,
    0x0a, 0xc1, 0x31, 0x88, 0xa5, 0xcd, 0x7b, 0xbd,
    0x2d, 0x74, 0xd0, 0x12, 0xb8, 0xe5, 0xb4, 0xb0,
    0x89, 0x69, 0x97, 0x4a, 0x0c, 0x96, 0x77, 0x7e,
    0x65, 0xb9, 0xf1, 0x09, 0xc5, 0x6e, 0xc6, 0x84,
    0x18, 0xf0, 0x7d, 0xec, 0x3a, 0xdc, 0x4d, 0x20,
    0x79, 0xee, 0x5f, 0x3e, 0xd7, 0xcb, 0x39, 0x48
];

fn rotl32(n: u32, x: u32) -> u32 {
    x.rotate_left(n)
}

fn sm4_t_non_lin_sub(x: u32) -> u32 {
    let mut out = 0;
    out |= u32::from(SBOX[(x & 0xff) as usize]);
    out |= u32::from(SBOX[((x >> 8) & 0xff) as usize]) << 8;
    out |= u32::from(SBOX[((x >> 16) & 0xff) as usize]) << 16;
    out |= u32::from(SBOX[((x >> 24) & 0xff) as usize]) << 24;
    out
}

fn sm4_key_lin_sub(x: u32) -> u32 {
    x ^ rotl32(13, x) ^ rotl32(23, x)
}

fn sm4_enc_lin_sub(x: u32) -> u32 {
    x ^ rotl32(2, x) ^ rotl32(10, x) ^ rotl32(18, x) ^ rotl32(24, x)
}

fn sm4_key_sub(x: u32) -> u32 {
    sm4_key_lin_sub(sm4_t_non_lin_sub(x))
}

fn sm4_enc_sub(x: u32) -> u32 {
    sm4_enc_lin_sub(sm4_t_non_lin_sub(x))
}

fn sm4_round(x0: u32, x1: u32, x2: u32, x3: u32, rk: u32) -> u32 {
    x0 ^ sm4_enc_sub(x1 ^ x2 ^ x3 ^ rk)
}

fn read_u32_be(bytes: &[u8]) -> u32 {
    u32::from_be_bytes(bytes[..4].try_into().unwrap())
}

fn write_u32_be(bytes: &mut [u8], value: u32) {
    bytes[..4].copy_from_slice(&value.to_be_bytes());
}

impl Sm4Ctx {
    fn set_key(&mut self, key: &[u8], encrypt: bool) {
        assert_eq!(key.len(), SM4_KEY_SIZE);

        let mut rk0 = read_u32_be(&key[0..4]) ^ FK[0];
        let mut rk1 = read_u32_be(&key[4..8]) ^ FK[1];
        let mut rk2 = read_u32_be(&key[8..12]) ^ FK[2];
        let mut rk3 = read_u32_be(&key[12..16]) ^ FK[3];

        for i in 0..32 {
            if i % 4 == 0 {
                rk0 ^= sm4_key_sub(rk1 ^ rk2 ^ rk3 ^ CK[i]);
                rk1 ^= sm4_key_sub(rk2 ^ rk3 ^ rk0 ^ CK[i + 1]);
                rk2 ^= sm4_key_sub(rk3 ^ rk0 ^ rk1 ^ CK[i + 2]);
                rk3 ^= sm4_key_sub(rk0 ^ rk1 ^ rk2 ^ CK[i + 3]);
            }

            if encrypt {
                self.rkey[i] = rk0;
                self.rkey[i + 1] = rk1;
                self.rkey[i + 2] = rk2;
                self.rkey[i + 3] = rk3;
            } else {
                self.rkey[31 - i] = rk0;
                self.rkey[31 - (i + 1)] = rk1;
                self.rkey[31 - (i + 2)] = rk2;
                self.rkey[31 - (i + 3)] = rk3;
            }
        }
    }

    pub fn set_encrypt_key(&mut self, key: &[u8]) {
        self.set_key(key, true);
    }

    pub fn set_decrypt_key(&mut self, key: &[u8]) {
        self.set_key(key, false);
    }

    pub fn crypt(&self, length: usize, dst: &mut [u8], src: &[u8]) {
        assert_eq!(length % SM4_BLOCK_SIZE, 0);
        assert!(dst.len() >= length);
        assert!(src.len() >= length);

        let mut src_pos = 0;
        let mut dst_pos = 0;
        let rk = &self.rkey;

        for _ in 0..length / SM4_BLOCK_SIZE {
            let mut x0 = read_u32_be(&src[src_pos..src_pos + 4]);
            let mut x1 = read_u32_be(&src[src_pos + 4..src_pos + 8]);
            let mut x2 = read_u32_be(&src[src_pos + 8..src_pos + 12]);
            let mut x3 = read_u32_be(&src[src_pos + 12..src_pos + 16]);

            for i in (0..32).step_by(4) {
                x0 = sm4_round(x0, x1, x2, x3, rk[i]);
                x1 = sm4_round(x1, x2, x3, x0, rk[i + 1]);
                x2 = sm4_round(x2, x3, x0, x1, rk[i + 2]);
                x3 = sm4_round(x3, x0, x1, x2, rk[i + 3]);
            }

            write_u32_be(&mut dst[dst_pos..dst_pos + 4], x3);
            write_u32_be(&mut dst[dst_pos + 4..dst_pos + 8], x2);
            write_u32_be(&mut dst[dst_pos + 8..dst_pos + 12], x1);
            write_u32_be(&mut dst[dst_pos + 12..dst_pos + 16], x0);

            src_pos += SM4_BLOCK_SIZE;
            dst_pos += SM4_BLOCK_SIZE;
        }
    }
}

impl Default for Sm4Ctx {
    fn default() -> Self {
        Sm4Ctx { rkey: [0; 32] }
    }
}