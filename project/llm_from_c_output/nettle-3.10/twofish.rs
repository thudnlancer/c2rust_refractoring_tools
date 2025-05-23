use std::convert::TryInto;

pub const TWOFISH_BLOCK_SIZE: usize = 16;
pub const TWOFISH_MIN_KEY_SIZE: usize = 16;
pub const TWOFISH_MAX_KEY_SIZE: usize = 32;
pub const TWOFISH_KEY_SIZE: usize = 32;
pub const TWOFISH128_KEY_SIZE: usize = 16;
pub const TWOFISH192_KEY_SIZE: usize = 24;
pub const TWOFISH256_KEY_SIZE: usize = 32;

#[derive(Clone)]
pub struct TwofishCtx {
    keys: [u32; 40],
    s_box: [[u32; 256]; 4],
}

impl TwofishCtx {
    pub fn new() -> Self {
        Self {
            keys: [0; 40],
            s_box: [[0; 256]; 4],
        }
    }
}

const Q0: [u8; 256] = [
    0xA9,0x67,0xB3,0xE8,0x04,0xFD,0xA3,0x76,0x9A,0x92,0x80,0x78,0xE4,0xDD,0xD1,0x38,
    0x0D,0xC6,0x35,0x98,0x18,0xF7,0xEC,0x6C,0x43,0x75,0x37,0x26,0xFA,0x13,0x94,0x48,
    0xF2,0xD0,0x8B,0x30,0x84,0x54,0xDF,0x23,0x19,0x5B,0x3D,0x59,0xF3,0xAE,0xA2,0x82,
    0x63,0x01,0x83,0x2E,0xD9,0x51,0x9B,0x7C,0xA6,0xEB,0xA5,0xBE,0x16,0x0C,0xE3,0x61,
    0xC0,0x8C,0x3A,0xF5,0x73,0x2C,0x25,0x0B,0xBB,0x4E,0x89,0x6B,0x53,0x6A,0xB4,0xF1,
    0xE1,0xE6,0xBD,0x45,0xE2,0xF4,0xB6,0x66,0xCC,0x95,0x03,0x56,0xD4,0x1C,0x1E,0xD7,
    0xFB,0xC3,0x8E,0xB5,0xE9,0xCF,0xBF,0xBA,0xEA,0x77,0x39,0xAF,0x33,0xC9,0x62,0x71,
    0x81,0x79,0x09,0xAD,0x24,0xCD,0xF9,0xD8,0xE5,0xC5,0xB9,0x4D,0x44,0x08,0x86,0xE7,
    0xA1,0x1D,0xAA,0xED,0x06,0x70,0xB2,0xD2,0x41,0x7B,0xA0,0x11,0x31,0xC2,0x27,0x90,
    0x20,0xF6,0x60,0xFF,0x96,0x5C,0xB1,0xAB,0x9E,0x9C,0x52,0x1B,0x5F,0x93,0x0A,0xEF,
    0x91,0x85,0x49,0xEE,0x2D,0x4F,0x8F,0x3B,0x47,0x87,0x6D,0x46,0xD6,0x3E,0x69,0x64,
    0x2A,0xCE,0xCB,0x2F,0xFC,0x97,0x05,0x7A,0xAC,0x7F,0xD5,0x1A,0x4B,0x0E,0xA7,0x5A,
    0x28,0x14,0x3F,0x29,0x88,0x3C,0x4C,0x02,0xB8,0xDA,0xB0,0x17,0x55,0x1F,0x8A,0x7D,
    0x57,0xC7,0x8D,0x74,0xB7,0xC4,0x9F,0x72,0x7E,0x15,0x22,0x12,0x58,0x07,0x99,0x34,
    0x6E,0x50,0xDE,0x68,0x65,0xBC,0xDB,0xF8,0xC8,0xA8,0x2B,0x40,0xDC,0xFE,0x32,0xA4,
    0xCA,0x10,0x21,0xF0,0xD3,0x5D,0x0F,0x00,0x6F,0x9D,0x36,0x42,0x4A,0x5E,0xC1,0xE0,
];

const Q1: [u8; 256] = [
    0x75,0xF3,0xC6,0xF4,0xDB,0x7B,0xFB,0xC8,0x4A,0xD3,0xE6,0x6B,0x45,0x7D,0xE8,0x4B,
    0xD6,0x32,0xD8,0xFD,0x37,0x71,0xF1,0xE1,0x30,0x0F,0xF8,0x1B,0x87,0xFA,0x06,0x3F,
    0x5E,0xBA,0xAE,0x5B,0x8A,0x00,0xBC,0x9D,0x6D,0xC1,0xB1,0x0E,0x80,0x5D,0xD2,0xD5,
    0xA0,0x84,0x07,0x14,0xB5,0x90,0x2C,0xA3,0xB2,0x73,0x4C,0x54,0x92,0x74,0x36,0x51,
    0x38,0xB0,0xBD,0x5A,0xFC,0x60,0x62,0x96,0x6C,0x42,0xF7,0x10,0x7C,0x28,0x27,0x8C,
    0x13,0x95,0x9C,0xC7,0x24,0x46,0x3B,0x70,0xCA,0xE3,0x85,0xCB,0x11,0xD0,0x93,0xB8,
    0xA6,0x83,0x20,0xFF,0x9F,0x77,0xC3,0xCC,0x03,0x6F,0x08,0xBF,0x40,0xE7,0x2B,0xE2,
    0x79,0x0C,0xAA,0x82,0x41,0x3A,0xEA,0xB9,0xE4,0x9A,0xA4,0x97,0x7E,0xDA,0x7A,0x17,
    0x66,0x94,0xA1,0x1D,0x3D,0xF0,0xDE,0xB3,0x0B,0x72,0xA7,0x1C,0xEF,0xD1,0x53,0x3E,
    0x8F,0x33,0x26,0x5F,0xEC,0x76,0x2A,0x49,0x81,0x88,0xEE,0x21,0xC4,0x1A,0xEB,0xD9,
    0xC5,0x39,0x99,0xCD,0xAD,0x31,0x8B,0x01,0x18,0x23,0xDD,0x1F,0x4E,0x2D,0xF9,0x48,
    0x4F,0xF2,0x65,0x8E,0x78,0x5C,0x58,0x19,0x8D,0xE5,0x98,0x57,0x67,0x7F,0x05,0x64,
    0xAF,0x63,0xB6,0xFE,0xF5,0xB7,0x3C,0xA5,0xCE,0xE9,0x68,0x44,0xE0,0x4D,0x43,0x69,
    0x29,0x2E,0xAC,0x15,0x59,0xA8,0x0A,0x9E,0x6E,0x47,0xDF,0x34,0x35,0x6A,0xCF,0xDC,
    0x22,0xC9,0xC0,0x9B,0x89,0xD4,0xED,0xAB,0x12,0xA2,0x0D,0x52,0xBB,0x02,0x2F,0xA9,
    0xD7,0x61,0x1E,0xB4,0x50,0x04,0xF6,0xC2,0x16,0x25,0x86,0x56,0x55,0x09,0xBE,0x91,
];

const RS_MATRIX: [[u8; 8]; 4] = [
    [0x01, 0xA4, 0x55, 0x87, 0x5A, 0x58, 0xDB, 0x9E],
    [0xA4, 0x56, 0x82, 0xF3, 0x1E, 0xC6, 0x68, 0xE5],
    [0x02, 0xA1, 0xFC, 0xC1, 0x47, 0xAE, 0x3D, 0x19],
    [0xA4, 0x55, 0x87, 0x5A, 0x58, 0xDB, 0x9E, 0x03],
];

const Q_TABLE: [[&'static [u8; 256]; 5]; 4] = [
    [&Q1, &Q1, &Q0, &Q0, &Q1],
    [&Q0, &Q1, &Q1, &Q0, &Q0],
    [&Q0, &Q0, &Q0, &Q1, &Q1],
    [&Q1, &Q0, &Q1, &Q1, &Q0],
];

const MDS_MATRIX: [[u8; 4]; 4] = [
    [0x01, 0xEF, 0x5B, 0x5B],
    [0x5B, 0xEF, 0xEF, 0x01],
    [0xEF, 0x5B, 0x01, 0xEF],
    [0xEF, 0x01, 0xEF, 0x5B],
];

fn rol1(x: u32) -> u32 {
    (x << 1) | (x >> 31)
}

fn rol8(x: u32) -> u32 {
    (x << 8) | (x >> 24)
}

fn rol9(x: u32) -> u32 {
    (x << 9) | (x >> 23)
}

fn ror1(x: u32) -> u32 {
    (x >> 1) | (x << 31)
}

fn gf_multiply(p: u8, a: u8, b: u8) -> u32 {
    let mut shift = b as u32;
    let mut result = 0u32;
    let mut a = a;
    while a != 0 {
        if a & 1 != 0 {
            result ^= shift;
        }
        a >>= 1;
        shift <<= 1;
        if shift & 0x100 != 0 {
            shift ^= p as u32;
        }
    }
    result
}

fn compute_s(m1: u32, m2: u32) -> u32 {
    let mut s = 0u32;
    for i in 0..4 {
        s |= (gf_multiply(0x4D, (m1 >> (i*8)) as u8, RS_MATRIX[i][0]) ^
              gf_multiply(0x4D, (m1 >> (i*8 + 8)) as u8, RS_MATRIX[i][1]) ^
              gf_multiply(0x4D, (m1 >> (i*8 + 16)) as u8, RS_MATRIX[i][2]) ^
              gf_multiply(0x4D, (m1 >> (i*8 + 24)) as u8, RS_MATRIX[i][3]) ^
              gf_multiply(0x4D, (m2 >> (i*8)) as u8, RS_MATRIX[i][4]) ^
              gf_multiply(0x4D, (m2 >> (i*8 + 8)) as u8, RS_MATRIX[i][5]) ^
              gf_multiply(0x4D, (m2 >> (i*8 + 16)) as u8, RS_MATRIX[i][6]) ^
              gf_multiply(0x4D, (m2 >> (i*8 + 24)) as u8, RS_MATRIX[i][7])) << (i*8);
    }
    s
}

fn h_byte(k: usize, i: usize, x: u8, l0: u8, l1: u8, l2: u8, l3: u8) -> u32 {
    let y = Q_TABLE[i][4][(l0 ^ Q_TABLE[i][3][(l1 ^ Q_TABLE[i][2][if k == 2 { x } else { l2 ^ Q_TABLE[i][1][if k == 3 { x } else { l3 ^ Q_TABLE[i][0][x] } }]) as usize];
    
    (gf_multiply(0x69, MDS_MATRIX[0][i], y) |
     (gf_multiply(0x69, MDS_MATRIX[1][i], y) << 8) |
     (gf_multiply(0x69, MDS_MATRIX[2][i], y) << 16) |
     (gf_multiply(0x69, MDS_MATRIX[3][i], y) << 24)
}

fn h(k: usize, x: u8, l0: u32, l1: u32, l2: u32, l3: u32) -> u32 {
    h_byte(k, 0, x, l0 as u8, l1 as u8, l2 as u8, l3 as u8) ^
    h_byte(k, 1, x, (l0 >> 8) as u8, (l1 >> 8) as u8, (l2 >> 8) as u8, (l3 >> 8) as u8) ^
    h_byte(k, 2, x, (l0 >> 16) as u8, (l1 >> 16) as u8, (l2 >> 16) as u8, (l3 >> 16) as u8) ^
    h_byte(k, 3, x, (l0 >> 24) as u8, (l1 >> 24) as u8, (l2 >> 24) as u8, (l3 >> 24) as u8)
}

pub fn twofish_set_key(context: &mut TwofishCtx, keysize: usize, key: &[u8]) {
    assert!(keysize <= TWOFISH_MAX_KEY_SIZE);
    let mut key_copy = [0u8; 32];
    key_copy[..keysize].copy_from_slice(&key[..keysize]);
    
    let mut m = [0u32; 8];
    for i in 0..8 {
        m[i] = u32::from_le_bytes(key_copy[i*4..i*4+4].try_into().unwrap());
    }
    
    let k = if keysize <= 16 {
        2
    } else if keysize <= 24 {
        3
    } else {
        4
    };
    
    for i in 0..20 {
        let t = h(k, (2*i+1) as u8, m[1], m[