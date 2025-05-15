use std::convert::TryInto;
use std::mem::size_of;

pub const SM3_DIGEST_SIZE: usize = 32;
pub const SM3_BLOCK_SIZE: usize = 64;
const _SM3_DIGEST_LENGTH: usize = 8;

pub struct Sm3Ctx {
    state: [u32; _SM3_DIGEST_LENGTH],
    count: u64,
    index: usize,
    block: [u8; SM3_BLOCK_SIZE],
}

const K: [u32; 64] = [
    0x79cc4519, 0xf3988a32, 0xe7311465, 0xce6228cb,
    0x9cc45197, 0x3988a32f, 0x7311465e, 0xe6228cbc,
    0xcc451979, 0x988a32f3, 0x311465e7, 0x6228cbce,
    0xc451979c, 0x88a32f39, 0x11465e73, 0x228cbce6,
    0x9d8a7a87, 0x3b14f50f, 0x7629ea1e, 0xec53d43c,
    0xd8a7a879, 0xb14f50f3, 0x629ea1e7, 0xc53d43ce,
    0x8a7a879d, 0x14f50f3b, 0x29ea1e76, 0x53d43cec,
    0xa7a879d8, 0x4f50f3b1, 0x9ea1e762, 0x3d43cec5,
    0x7a879d8a, 0xf50f3b14, 0xea1e7629, 0xd43cec53,
    0xa879d8a7, 0x50f3b14f, 0xa1e7629e, 0x43cec53d,
    0x879d8a7a, 0x0f3b14f5, 0x1e7629ea, 0x3cec53d4,
    0x79d8a7a8, 0xf3b14f50, 0xe7629ea1, 0xcec53d43,
    0x9d8a7a87, 0x3b14f50f, 0x7629ea1e, 0xec53d43c,
    0xd8a7a879, 0xb14f50f3, 0x629ea1e7, 0xc53d43ce,
    0x8a7a879d, 0x14f50f3b, 0x29ea1e76, 0x53d43cec,
    0xa7a879d8, 0x4f50f3b1, 0x9ea1e762, 0x3d43cec5,
];

fn rotl32(x: u32, n: u32) -> u32 {
    x.rotate_left(n)
}

fn ff1(x: u32, y: u32, z: u32) -> u32 {
    x ^ y ^ z
}

fn ff2(x: u32, y: u32, z: u32) -> u32 {
    (x & y) | (x & z) | (y & z)
}

fn gg1(x: u32, y: u32, z: u32) -> u32 {
    x ^ y ^ z
}

fn gg2(x: u32, y: u32, z: u32) -> u32 {
    (x & y) | (!x & z)
}

fn p0(x: u32) -> u32 {
    x ^ rotl32(x, 9) ^ rotl32(x, 17)
}

fn p1(x: u32) -> u32 {
    x ^ rotl32(x, 15) ^ rotl32(x, 23)
}

fn read_uint32(input: &[u8]) -> u32 {
    u32::from_be_bytes(input[..4].try_into().unwrap())
}

fn write_be32(length: usize, digest: &mut [u8], state: &[u32; _SM3_DIGEST_LENGTH]) {
    for (i, &word) in state.iter().enumerate().take(length / 4) {
        digest[i * 4..(i + 1) * 4].copy_from_slice(&word.to_be_bytes());
    }
}

fn write_u64(dst: &mut [u8], n: u64) {
    dst.copy_from_slice(&n.to_be_bytes());
}

fn sm3_compress(state: &mut [u32; _SM3_DIGEST_LENGTH], input: &[u8]) {
    let mut a = state[0];
    let mut b = state[1];
    let mut c = state[2];
    let mut d = state[3];
    let mut e = state[4];
    let mut f = state[5];
    let mut g = state[6];
    let mut h = state[7];
    let mut ss1;
    let mut ss2;
    let mut w = [0u32; 16];

    macro_rules! i {
        ($i:expr) => {
            w[$i] = read_uint32(&input[$i * 4..]);
        };
    }

    macro_rules! w1 {
        ($i:expr) => {
            w[$i & 0x0f]
        };
    }

    macro_rules! w2 {
        ($i:expr) => {
            w[$i & 0x0f] = p1(w[$i & 0x0f] ^ w[($i - 9) & 0x0f] ^ rotl32(w[($i - 3) & 0x0f], 15))
                ^ rotl32(w[($i - 13) & 0x0f], 7)
                ^ w[($i - 6) & 0x0f]
        };
    }

    macro_rules! r {
        ($i:expr, $a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr, $h:expr, $t:expr, $w1:expr, $w2:expr) => {
            ss1 = rotl32(7, rotl32(12, $a).wrapping_add($e).wrapping_add($t));
            ss2 = ss1 ^ rotl32(12, $a);
            $d = $d.wrapping_add(if $i == 1 { ff1($a, $b, $c) } else { ff2($a, $b, $c) })
                .wrapping_add(ss2)
                .wrapping_add($w1 ^ $w2);
            $h = $h.wrapping_add(if $i == 1 { gg1($e, $f, $g) } else { gg2($e, $f, $g) })
                .wrapping_add(ss1)
                .wrapping_add($w1);
            $b = rotl32(9, $b);
            $f = rotl32(19, $f);
            $h = p0($h);
        };
    }

    macro_rules! r1 {
        ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr, $h:expr, $t:expr, $w1:expr, $w2:expr) => {
            r!(1, $a, $b, $c, $d, $e, $f, $g, $h, $t, $w1, $w2)
        };
    }

    macro_rules! r2 {
        ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr, $h:expr, $t:expr, $w1:expr, $w2:expr) => {
            r!(2, $a, $b, $c, $d, $e, $f, $g, $h, $t, $w1, $w2)
        };
    }

    r1!(a, b, c, d, e, f, g, h, K[0], i!(0), i!(4));
    r1!(d, a, b, c, h, e, f, g, K[1], i!(1), i!(5));
    r1!(c, d, a, b, g, h, e, f, K[2], i!(2), i!(6));
    r1!(b, c, d, a, f, g, h, e, K[3], i!(3), i!(7));
    r1!(a, b, c, d, e, f, g, h, K[4], w1!(4), i!(8));
    r1!(d, a, b, c, h, e, f, g, K[5], w1!(5), i!(9));
    r1!(c, d, a, b, g, h, e, f, K[6], w1!(6), i!(10));
    r1!(b, c, d, a, f, g, h, e, K[7], w1!(7), i!(11));
    r1!(a, b, c, d, e, f, g, h, K[8], w1!(8), i!(12));
    r1!(d, a, b, c, h, e, f, g, K[9], w1!(9), i!(13));
    r1!(c, d, a, b, g, h, e, f, K[10], w1!(10), i!(14));
    r1!(b, c, d, a, f, g, h, e, K[11], w1!(11), i!(15));
    r1!(a, b, c, d, e, f, g, h, K[12], w1!(12), w2!(16));
    r1!(d, a, b, c, h, e, f, g, K[13], w1!(13), w2!(17));
    r1!(c, d, a, b, g, h, e, f, K[14], w1!(14), w2!(18));
    r1!(b, c, d, a, f, g, h, e, K[15], w1!(15), w2!(19));

    r2!(a, b, c, d, e, f, g, h, K[16], w1!(16), w2!(20));
    r2!(d, a, b, c, h, e, f, g, K[17], w1!(17), w2!(21));
    r2!(c, d, a, b, g, h, e, f, K[18], w1!(18), w2!(22));
    r2!(b, c, d, a, f, g, h, e, K[19], w1!(19), w2!(23));
    r2!(a, b, c, d, e, f, g, h, K[20], w1!(20), w2!(24));
    r2!(d, a, b, c, h, e, f, g, K[21], w1!(21), w2!(25));
    r2!(c, d, a, b, g, h, e, f, K[22], w1!(22), w2!(26));
    r2!(b, c, d, a, f, g, h, e, K[23], w1!(23), w2!(27));
    r2!(a, b, c, d, e, f, g, h, K[24], w1!(24), w2!(28));
    r2!(d, a, b, c, h, e, f, g, K[25], w1!(25), w2!(29));
    r2!(c, d, a, b, g, h, e, f, K[26], w1!(26), w2!(30));
    r2!(b, c, d, a, f, g, h, e, K[27], w1!(27), w2!(31));
    r2!(a, b, c, d, e, f, g, h, K[28], w1!(28), w2!(32));
    r2!(d, a, b, c, h, e, f, g, K[29], w1!(29), w2!(33));
    r2!(c, d, a, b, g, h, e, f, K[30], w1!(30), w2!(34));
    r2!(b, c, d, a, f, g, h, e, K[31], w1!(31), w2!(35));

    r2!(a, b, c, d, e, f, g, h, K[32], w1!(32), w2!(36));
    r2!(d, a, b, c, h, e, f, g, K[33], w1!(33), w2!(37));
    r2!(c, d, a, b, g, h, e, f, K[34], w1!(34), w2!(38));
    r2!(b, c, d, a, f, g, h, e, K[35], w1!(35), w2!(39));
    r2!(a, b, c, d, e, f, g, h, K[36], w1!(36), w2!(40));
    r2!(d, a, b, c, h, e, f, g, K[37], w1!(37), w2!(41));
    r2!(c, d, a, b, g, h, e, f, K[38], w1!(38), w2!(42));
    r2!(b, c, d, a, f, g, h, e, K[39], w1!(39), w2!(43));
    r2!(a, b, c, d, e, f, g, h, K[40], w1!(40), w2!(44));
    r2!(d, a, b, c, h, e, f, g, K[41], w1!(41), w2!(45));
    r2!(c, d, a, b, g, h, e, f, K[42], w1!(42), w2!(46));
    r2!(b, c, d, a, f, g, h, e, K[43], w1!(43), w2!(47));
    r2!(a, b, c, d, e, f, g, h, K[44], w1!(44), w2!(48));
    r2!(d, a, b, c, h, e, f, g, K[45], w1!(45), w2!(49));
    r2!(c, d, a, b, g, h, e, f, K[46], w1!(46), w2!(50));
    r2!(b, c, d, a, f, g, h, e, K[47], w1!(47), w2!(51));

    r2!(a, b, c, d, e, f, g, h, K[48], w1!(48), w2!(52));
    r2!(d, a, b, c, h, e, f, g, K[49], w1!(49), w2!(53));
    r2!(c, d, a, b, g, h, e, f, K[50], w1!(50), w2!(54));
    r2!(b, c, d, a, f, g, h, e, K[51], w1!(51), w2!(55));
    r2!(a, b, c, d, e, f, g, h, K[52], w1!(52), w2!(56));
    r2!(d, a, b, c, h, e, f, g, K[53], w1!(53), w2!(57));
    r2!(c, d, a, b, g, h, e, f, K[54], w1!(54), w2!(58));
    r2!(b, c, d, a, f, g, h, e, K[55], w1!(55), w2!(59));
    r2!(a, b, c, d, e, f, g, h, K[56], w1!(56), w2!(60));
    r2!(d, a, b, c, h, e, f, g, K[57], w1!(57), w2!(61));
    r2!(c, d, a, b, g, h, e, f, K[58], w1!(58), w2!(62));
    r2!(b, c, d, a, f, g, h, e, K[59], w1!(59), w2!(63));
    r2!(a, b, c, d, e, f, g, h, K[60], w1!(60), w2!(64));
    r2!(d, a, b, c, h, e, f, g, K[61], w1!(61), w2!(65));
    r2!(c, d, a, b, g, h, e, f, K[62], w1!(62), w2!(66));
    r2!(b, c, d, a, f, g, h, e, K[63], w1!(63), w2!(67));

    state[0] ^= a;
    state[1] ^= b;
    state[2] ^= c;
   