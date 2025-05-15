use std::convert::TryInto;

const ARCTWO_SBOX: [u8; 256] = [
    0xd9, 0x78, 0xf9, 0xc4, 0x19, 0xdd, 0xb5, 0xed, 0x28, 0xe9, 0xfd, 0x79, 0x4a, 0xa0, 0xd8, 0x9d,
    0xc6, 0x7e, 0x37, 0x83, 0x2b, 0x76, 0x53, 0x8e, 0x62, 0x4c, 0x64, 0x88, 0x44, 0x8b, 0xfb, 0xa2,
    0x17, 0x9a, 0x59, 0xf5, 0x87, 0xb3, 0x4f, 0x13, 0x61, 0x45, 0x6d, 0x8d, 0x09, 0x81, 0x7d, 0x32,
    0xbd, 0x8f, 0x40, 0xeb, 0x86, 0xb7, 0x7b, 0x0b, 0xf0, 0x95, 0x21, 0x22, 0x5c, 0x6b, 0x4e, 0x82,
    0x54, 0xd6, 0x65, 0x93, 0xce, 0x60, 0xb2, 0x1c, 0x73, 0x56, 0xc0, 0x14, 0xa7, 0x8c, 0xf1, 0xdc,
    0x12, 0x75, 0xca, 0x1f, 0x3b, 0xbe, 0xe4, 0xd1, 0x42, 0x3d, 0xd4, 0x30, 0xa3, 0x3c, 0xb6, 0x26,
    0x6f, 0xbf, 0x0e, 0xda, 0x46, 0x69, 0x07, 0x57, 0x27, 0xf2, 0x1d, 0x9b, 0xbc, 0x94, 0x43, 0x03,
    0xf8, 0x11, 0xc7, 0xf6, 0x90, 0xef, 0x3e, 0xe7, 0x06, 0xc3, 0xd5, 0x2f, 0xc8, 0x66, 0x1e, 0xd7,
    0x08, 0xe8, 0xea, 0xde, 0x80, 0x52, 0xee, 0xf7, 0x84, 0xaa, 0x72, 0xac, 0x35, 0x4d, 0x6a, 0x2a,
    0x96, 0x1a, 0xd2, 0x71, 0x5a, 0x15, 0x49, 0x74, 0x4b, 0x9f, 0xd0, 0x5e, 0x04, 0x18, 0xa4, 0xec,
    0xc2, 0xe0, 0x41, 0x6e, 0x0f, 0x51, 0xcb, 0xcc, 0x24, 0x91, 0xaf, 0x50, 0xa1, 0xf4, 0x70, 0x39,
    0x99, 0x7c, 0x3a, 0x85, 0x23, 0xb8, 0xb4, 0x7a, 0xfc, 0x02, 0x36, 0x5b, 0x25, 0x55, 0x97, 0x31,
    0x2d, 0x5d, 0xfa, 0x98, 0xe3, 0x8a, 0x92, 0xae, 0x05, 0xdf, 0x29, 0x10, 0x67, 0x6c, 0xba, 0xc9,
    0xd3, 0x00, 0xe6, 0xcf, 0xe1, 0x9e, 0xa8, 0x2c, 0x63, 0x16, 0x01, 0x3f, 0x58, 0xe2, 0x89, 0xa9,
    0x0d, 0x38, 0x34, 0x1b, 0xab, 0x33, 0xff, 0xb0, 0xbb, 0x48, 0x0c, 0x5f, 0xb9, 0xb1, 0xcd, 0x2e,
    0xc5, 0xf3, 0xdb, 0x47, 0xe5, 0xa5, 0x9c, 0x77, 0x0a, 0xa6, 0x20, 0x68, 0xfe, 0x7f, 0xc1, 0xad,
];

#[derive(Clone, Copy)]
pub struct ArctwoCtx {
    pub S: [u16; 64],
}

impl ArctwoCtx {
    pub fn new() -> Self {
        Self { S: [0; 64] }
    }

    pub fn encrypt(&mut self, data: &[u8]) -> Vec<u8> {
        assert!(data.len() % 8 == 0, "Data length must be multiple of 8");
        let mut result = vec![0; data.len()];
        for (block, out_block) in data.chunks(8).zip(result.chunks_mut(8)) {
            let mut w = self.process_block(block.try_into().unwrap(), true);
            out_block.copy_from_slice(&w.to_bytes());
        }
        result
    }

    pub fn decrypt(&mut self, data: &[u8]) -> Vec<u8> {
        assert!(data.len() % 8 == 0, "Data length must be multiple of 8");
        let mut result = vec![0; data.len()];
        for (block, out_block) in data.chunks(8).zip(result.chunks_mut(8)) {
            let mut w = self.process_block(block.try_into().unwrap(), false);
            out_block.copy_from_slice(&w.to_bytes());
        }
        result
    }

    fn process_block(&self, block: [u8; 8], encrypt: bool) -> [u16; 4] {
        let mut w = [
            u16::from_be_bytes([block[0], block[1]]),
            u16::from_be_bytes([block[2], block[3]]),
            u16::from_be_bytes([block[4], block[5]]),
            u16::from_be_bytes([block[6], block[7]]),
        ];

        if encrypt {
            for i in 0..16 {
                let j = i * 4;
                w[0] = w[0].wrapping_add(
                    (w[1] & !w[3] | w[2] & w[3])
                        .wrapping_add(self.S[j as usize] as u16)
                ).rotate_left(1);
                w[1] = w[1].wrapping_add(
                    (w[2] & !w[0] | w[3] & w[0])
                        .wrapping_add(self.S[(j + 1) as usize] as u16)
                ).rotate_left(2);
                w[2] = w[2].wrapping_add(
                    (w[3] & !w[1] | w[0] & w[1])
                        .wrapping_add(self.S[(j + 2) as usize] as u16)
                ).rotate_left(3);
                w[3] = w[3].wrapping_add(
                    (w[0] & !w[2] | w[1] & w[2])
                        .wrapping_add(self.S[(j + 3) as usize] as u16)
                ).rotate_left(5);

                if i == 4 || i == 10 {
                    w[0] = w[0].wrapping_add(self.S[(w[3] & 63) as usize] as u16);
                    w[1] = w[1].wrapping_add(self.S[(w[0] & 63) as usize] as u16);
                    w[2] = w[2].wrapping_add(self.S[(w[1] & 63) as usize] as u16);
                    w[3] = w[3].wrapping_add(self.S[(w[2] & 63) as usize] as u16);
                }
            }
        } else {
            for i in (0..16).rev() {
                let j = i * 4;
                if i == 5 || i == 11 {
                    w[3] = w[3].wrapping_sub(self.S[(w[2] & 63) as usize] as u16);
                    w[2] = w[2].wrapping_sub(self.S[(w[1] & 63) as usize] as u16);
                    w[1] = w[1].wrapping_sub(self.S[(w[0] & 63) as usize] as u16);
                    w[0] = w[0].wrapping_sub(self.S[(w[3] & 63) as usize] as u16);
                }

                w[3] = w[3].rotate_right(5).wrapping_sub(
                    (w[0] & !w[2] | w[1] & w[2])
                        .wrapping_add(self.S[(j + 3) as usize] as u16)
                );
                w[2] = w[2].rotate_right(3).wrapping_sub(
                    (w[3] & !w[1] | w[0] & w[1])
                        .wrapping_add(self.S[(j + 2) as usize] as u16)
                );
                w[1] = w[1].rotate_right(2).wrapping_sub(
                    (w[2] & !w[0] | w[3] & w[0])
                        .wrapping_add(self.S[(j + 1) as usize] as u16)
                );
                w[0] = w[0].rotate_right(1).wrapping_sub(
                    (w[1] & !w[3] | w[2] & w[3])
                        .wrapping_add(self.S[j as usize] as u16)
                );
            }
        }
        w
    }

    pub fn set_key_ekb(key: &[u8], ekb: u32) -> Self {
        assert!(key.len() >= 1 && key.len() <= 128, "Invalid key length");
        assert!(ekb <= 1024, "EKB must be <= 1024");

        let mut S = [0; 128];
        S[..key.len()].copy_from_slice(key);

        for i in key.len()..128 {
            S[i] = ARCTWO_SBOX[(S[i - key.len()] as usize + S[i - 1] as usize) & 0xff];
        }

        S[0] = ARCTWO_SBOX[S[0] as usize];

        if ekb > 0 && ekb < 1024 {
            let len = ((ekb + 7) >> 3) as usize;
            let mut i = 128 - len;
            let mut x = ARCTWO_SBOX[(S[i] as usize & (0xff >> (7 & (!ekb as usize)))];
            S[i] = x;

            while i > 0 {
                i -= 1;
                x = ARCTWO_SBOX[(x as usize ^ S[i + len] as usize)];
                S[i] = x;
            }
        }

        let mut ctx = Self::new();
        for i in 0..64 {
            ctx.S[i] = u16::from_be_bytes([S[i * 2], S[i * 2 + 1]]);
        }
        ctx
    }

    pub fn set_key(key: &[u8]) -> Self {
        Self::set_key_ekb(key, (key.len() * 8) as u32)
    }

    pub fn set_key_gutmann(key: &[u8]) -> Self {
        Self::set_key_ekb(key, 0)
    }

    pub fn set_key_40(key: &[u8]) -> Self {
        assert_eq!(key.len(), 5, "Key must be 5 bytes for ARC2-40");
        Self::set_key_ekb(key, 40)
    }

    pub fn set_key_64(key: &[u8]) -> Self {
        assert_eq!(key.len(), 8, "Key must be 8 bytes for ARC2-64");
        Self::set_key_ekb(key, 64)
    }

    pub fn set_key_128(key: &[u8]) -> Self {
        assert_eq!(key.len(), 16, "Key must be 16 bytes for ARC2-128");
        Self::set_key_ekb(key, 128)
    }

    pub fn set_key_128_gutmann(key: &[u8]) -> Self {
        assert_eq!(key.len(), 16, "Key must be 16 bytes for ARC2-128-Gutmann");
        Self::set_key_ekb(key, 1024)
    }
}

trait Block {
    fn to_bytes(&self) -> [u8; 8];
}

impl Block for [u16; 4] {
    fn to_bytes(&self) -> [u8; 8] {
        let mut bytes = [0; 8];
        bytes[0..2].copy_from_slice(&self[0].to_be_bytes());
        bytes[2..4].copy_from_slice(&self[1].to_be_bytes());
        bytes[4..6].copy_from_slice(&self[2].to_be_bytes());
        bytes[6..8].copy_from_slice(&self[3].to_be_bytes());
        bytes
    }
}