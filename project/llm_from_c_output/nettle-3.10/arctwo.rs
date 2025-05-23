use std::convert::TryInto;

pub const ARCTWO_BLOCK_SIZE: usize = 8;
pub const ARCTWO_MIN_KEY_SIZE: usize = 1;
pub const ARCTWO_MAX_KEY_SIZE: usize = 128;
pub const ARCTWO_KEY_SIZE: usize = 8;

#[derive(Clone)]
pub struct ArctwoCtx {
    s: [u16; 64],
}

const ARCTWO_SBOX: [u8; 256] = [
    0xd9, 0x78, 0xf9, 0xc4, 0x19, 0xdd, 0xb5, 0xed,
    0x28, 0xe9, 0xfd, 0x79, 0x4a, 0xa0, 0xd8, 0x9d,
    0xc6, 0x7e, 0x37, 0x83, 0x2b, 0x76, 0x53, 0x8e,
    0x62, 0x4c, 0x64, 0x88, 0x44, 0x8b, 0xfb, 0xa2,
    0x17, 0x9a, 0x59, 0xf5, 0x87, 0xb3, 0x4f, 0x13,
    0x61, 0x45, 0x6d, 0x8d, 0x09, 0x81, 0x7d, 0x32,
    0xbd, 0x8f, 0x40, 0xeb, 0x86, 0xb7, 0x7b, 0x0b,
    0xf0, 0x95, 0x21, 0x22, 0x5c, 0x6b, 0x4e, 0x82,
    0x54, 0xd6, 0x65, 0x93, 0xce, 0x60, 0xb2, 0x1c,
    0x73, 0x56, 0xc0, 0x14, 0xa7, 0x8c, 0xf1, 0xdc,
    0x12, 0x75, 0xca, 0x1f, 0x3b, 0xbe, 0xe4, 0xd1,
    0x42, 0x3d, 0xd4, 0x30, 0xa3, 0x3c, 0xb6, 0x26,
    0x6f, 0xbf, 0x0e, 0xda, 0x46, 0x69, 0x07, 0x57,
    0x27, 0xf2, 0x1d, 0x9b, 0xbc, 0x94, 0x43, 0x03,
    0xf8, 0x11, 0xc7, 0xf6, 0x90, 0xef, 0x3e, 0xe7,
    0x06, 0xc3, 0xd5, 0x2f, 0xc8, 0x66, 0x1e, 0xd7,
    0x08, 0xe8, 0xea, 0xde, 0x80, 0x52, 0xee, 0xf7,
    0x84, 0xaa, 0x72, 0xac, 0x35, 0x4d, 0x6a, 0x2a,
    0x96, 0x1a, 0xd2, 0x71, 0x5a, 0x15, 0x49, 0x74,
    0x4b, 0x9f, 0xd0, 0x5e, 0x04, 0x18, 0xa4, 0xec,
    0xc2, 0xe0, 0x41, 0x6e, 0x0f, 0x51, 0xcb, 0xcc,
    0x24, 0x91, 0xaf, 0x50, 0xa1, 0xf4, 0x70, 0x39,
    0x99, 0x7c, 0x3a, 0x85, 0x23, 0xb8, 0xb4, 0x7a,
    0xfc, 0x02, 0x36, 0x5b, 0x25, 0x55, 0x97, 0x31,
    0x2d, 0x5d, 0xfa, 0x98, 0xe3, 0x8a, 0x92, 0xae,
    0x05, 0xdf, 0x29, 0x10, 0x67, 0x6c, 0xba, 0xc9,
    0xd3, 0x00, 0xe6, 0xcf, 0xe1, 0x9e, 0xa8, 0x2c,
    0x63, 0x16, 0x01, 0x3f, 0x58, 0xe2, 0x89, 0xa9,
    0x0d, 0x38, 0x34, 0x1b, 0xab, 0x33, 0xff, 0xb0,
    0xbb, 0x48, 0x0c, 0x5f, 0xb9, 0xb1, 0xcd, 0x2e,
    0xc5, 0xf3, 0xdb, 0x47, 0xe5, 0xa5, 0x9c, 0x77,
    0x0a, 0xa6, 0x20, 0x68, 0xfe, 0x7f, 0xc1, 0xad,
];

fn rotl16(x: u16, n: u16) -> u16 {
    x.rotate_left(n.into())
}

fn rotr16(x: u16, n: u16) -> u16 {
    x.rotate_right(n.into())
}

impl ArctwoCtx {
    pub fn encrypt(&self, data: &[u8]) -> Vec<u8> {
        let mut dst = vec![0; data.len()];
        for block in data.chunks(ARCTWO_BLOCK_SIZE) {
            if block.len() != ARCTWO_BLOCK_SIZE {
                continue;
            }
            
            let mut w0 = u16::from_le_bytes(block[0..2].try_into().unwrap());
            let mut w1 = u16::from_le_bytes(block[2..4].try_into().unwrap());
            let mut w2 = u16::from_le_bytes(block[4..6].try_into().unwrap());
            let mut w3 = u16::from_le_bytes(block[6..8].try_into().unwrap());

            for i in 0..16 {
                let j = i * 4;
                w0 = rotl16(
                    w0.wrapping_add((w1 & !w3).wrapping_add(w2 & w3).wrapping_add(self.s[j]),
                    1,
                );
                w1 = rotl16(
                    w1.wrapping_add((w2 & !w0).wrapping_add(w3 & w0).wrapping_add(self.s[j + 1]),
                    2,
                );
                w2 = rotl16(
                    w2.wrapping_add((w3 & !w1).wrapping_add(w0 & w1).wrapping_add(self.s[j + 2]),
                    3,
                );
                w3 = rotl16(
                    w3.wrapping_add((w0 & !w2).wrapping_add(w1 & w2).wrapping_add(self.s[j + 3]),
                    5,
                );

                if i == 4 || i == 10 {
                    w0 = w0.wrapping_add(self.s[(w3 & 63) as usize]);
                    w1 = w1.wrapping_add(self.s[(w0 & 63) as usize]);
                    w2 = w2.wrapping_add(self.s[(w1 & 63) as usize]);
                    w3 = w3.wrapping_add(self.s[(w2 & 63) as usize]);
                }
            }

            dst[..8].copy_from_slice(&[
                w0.to_le_bytes(),
                w1.to_le_bytes(),
                w2.to_le_bytes(),
                w3.to_le_bytes(),
            ].concat());
        }
        dst
    }

    pub fn decrypt(&self, data: &[u8]) -> Vec<u8> {
        let mut dst = vec![0; data.len()];
        for block in data.chunks(ARCTWO_BLOCK_SIZE) {
            if block.len() != ARCTWO_BLOCK_SIZE {
                continue;
            }
            
            let mut w0 = u16::from_le_bytes(block[0..2].try_into().unwrap());
            let mut w1 = u16::from_le_bytes(block[2..4].try_into().unwrap());
            let mut w2 = u16::from_le_bytes(block[4..6].try_into().unwrap());
            let mut w3 = u16::from_le_bytes(block[6..8].try_into().unwrap());

            for i in (0..16).rev() {
                let j = i * 4;

                w3 = rotr16(w3, 5);
                w3 = w3.wrapping_sub(
                    (w0 & !w2).wrapping_add(w1 & w2).wrapping_add(self.s[j + 3])
                );

                w2 = rotr16(w2, 3);
                w2 = w2.wrapping_sub(
                    (w3 & !w1).wrapping_add(w0 & w1).wrapping_add(self.s[j + 2])
                );

                w1 = rotr16(w1, 2);
                w1 = w1.wrapping_sub(
                    (w2 & !w0).wrapping_add(w3 & w0).wrapping_add(self.s[j + 1])
                );

                w0 = rotr16(w0, 1);
                w0 = w0.wrapping_sub(
                    (w1 & !w3).wrapping_add(w2 & w3).wrapping_add(self.s[j])
                );

                if i == 5 || i == 11 {
                    w3 = w3.wrapping_sub(self.s[(w2 & 63) as usize]);
                    w2 = w2.wrapping_sub(self.s[(w1 & 63) as usize]);
                    w1 = w1.wrapping_sub(self.s[(w0 & 63) as usize]);
                    w0 = w0.wrapping_sub(self.s[(w3 & 63) as usize]);
                }
            }

            dst[..8].copy_from_slice(&[
                w0.to_le_bytes(),
                w1.to_le_bytes(),
                w2.to_le_bytes(),
                w3.to_le_bytes(),
            ].concat());
        }
        dst
    }

    pub fn set_key_ekb(key: &[u8], ekb: u16) -> Result<Self, &'static str> {
        if key.len() < ARCTWO_MIN_KEY_SIZE || key.len() > ARCTWO_MAX_KEY_SIZE {
            return Err("Invalid key length");
        }
        if ekb > 1024 {
            return Err("Effective key bits must be <= 1024");
        }

        let mut s = [0u8; 128];
        s[..key.len()].copy_from_slice(key);

        for i in key.len()..ARCTWO_MAX_KEY_SIZE {
            s[i] = ARCTWO_SBOX[(s[i - key.len()].wrapping_add(s[i - 1])) as usize];
        }

        s[0] = ARCTWO_SBOX[s[0] as usize];

        if ekb > 0 && ekb < 1024 {
            let len = ((ekb + 7) >> 3) as usize;
            let mut i = 128 - len;
            let mut x = ARCTWO_SBOX[(s[i] & (255 >> (7 & !(ekb as u8))) as usize];
            s[i] = x;

            while i > 0 {
                i -= 1;
                x = ARCTWO_SBOX[(x ^ s[i + len]) as usize];
                s[i] = x;
            }
        }

        let mut ctx_s = [0u16; 64];
        for i in 0..64 {
            ctx_s[i] = u16::from_le_bytes([s[i * 2], s[i * 2 + 1]]);
        }

        Ok(Self { s: ctx_s })
    }

    pub fn set_key(key: &[u8]) -> Result<Self, &'static str> {
        Self::set_key_ekb(key, (key.len() * 8) as u16)
    }

    pub fn set_key_gutmann(key: &[u8]) -> Result<Self, &'static str> {
        Self::set_key_ekb(key, 1024)
    }

    pub fn set_key_40(key: &[u8]) -> Result<Self, &'static str> {
        if key.len() != 5 {
            return Err("Key must be 5 bytes for ARCTWO40");
        }
        Self::set_key_ekb(key, 40)
    }

    pub fn set_key_64(key: &[u8]) -> Result<Self, &'static str> {
        if key.len() != 8 {
            return Err("Key must be 8 bytes for ARCTWO64");
        }
        Self::set_key_ekb(key, 64)
    }

    pub fn set_key_128(key: &[u8]) -> Result<Self, &'static str> {
        if key.len() != 16 {
            return Err("Key must be 16 bytes for ARCTWO128");
        }
        Self::set_key_ekb(key, 128)
    }

    pub fn set_key_128_gutmann(key: &[u8]) -> Result<Self, &'static str> {
        if key.len() != 16 {
            return Err("Key must be 16 bytes for ARCTWO128");
        }
        Self::set_key_ekb(key, 1024)
    }
}