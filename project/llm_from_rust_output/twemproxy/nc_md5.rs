use std::convert::TryInto;
use std::mem;

type MD5_u32plus = u32;

#[derive(Clone, Copy)]
struct MD5_CTX {
    lo: MD5_u32plus,
    hi: MD5_u32plus,
    a: MD5_u32plus,
    b: MD5_u32plus,
    c: MD5_u32plus,
    d: MD5_u32plus,
    buffer: [u8; 64],
    block: [MD5_u32plus; 16],
}

fn body(ctx: &mut MD5_CTX, data: &[u8]) -> usize {
    let mut a = ctx.a;
    let mut b = ctx.b;
    let mut c = ctx.c;
    let mut d = ctx.d;
    
    for chunk in data.chunks(64) {
        let saved_a = a;
        let saved_b = b;
        let saved_c = c;
        let saved_d = d;
        
        let mut words = [0u32; 16];
        for (i, word) in chunk.chunks(4).take(16).enumerate() {
            words[i] = u32::from_le_bytes(word.try_into().unwrap());
        }
        
        // Round 1
        macro_rules! F {
            ($x:expr, $y:expr, $z:expr) => {
                ($z ^ ($x & ($y ^ $z)))
            };
        }
        macro_rules! FF {
            ($a:expr, $b:expr, $c:expr, $d:expr, $x:expr, $s:expr, $ac:expr) => {
                $a = $a.wrapping_add(F!($b, $c, $d).wrapping_add($x).wrapping_add($ac);
                $a = $a.rotate_left($s).wrapping_add($b);
            };
        }
        
        FF!(a, b, c, d, words[0], 7, 0xd76aa478);
        FF!(d, a, b, c, words[1], 12, 0xe8c7b756);
        FF!(c, d, a, b, words[2], 17, 0x242070db);
        FF!(b, c, d, a, words[3], 22, 0xc1bdceee);
        FF!(a, b, c, d, words[4], 7, 0xf57c0faf);
        FF!(d, a, b, c, words[5], 12, 0x4787c62a);
        FF!(c, d, a, b, words[6], 17, 0xa8304613);
        FF!(b, c, d, a, words[7], 22, 0xfd469501);
        FF!(a, b, c, d, words[8], 7, 0x698098d8);
        FF!(d, a, b, c, words[9], 12, 0x8b44f7af);
        FF!(c, d, a, b, words[10], 17, 0xffff5bb1);
        FF!(b, c, d, a, words[11], 22, 0x895cd7be);
        FF!(a, b, c, d, words[12], 7, 0x6b901122);
        FF!(d, a, b, c, words[13], 12, 0xfd987193);
        FF!(c, d, a, b, words[14], 17, 0xa679438e);
        FF!(b, c, d, a, words[15], 22, 0x49b40821);

        // Round 2
        macro_rules! G {
            ($x:expr, $y:expr, $z:expr) => {
                ($y ^ ($z & ($x ^ $y)))
            };
        }
        macro_rules! GG {
            ($a:expr, $b:expr, $c:expr, $d:expr, $x:expr, $s:expr, $ac:expr) => {
                $a = $a.wrapping_add(G!($b, $c, $d)).wrapping_add($x).wrapping_add($ac);
                $a = $a.rotate_left($s).wrapping_add($b);
            };
        }
        
        GG!(a, b, c, d, words[1], 5, 0xf61e2562);
        GG!(d, a, b, c, words[6], 9, 0xc040b340);
        GG!(c, d, a, b, words[11], 14, 0x265e5a51);
        GG!(b, c, d, a, words[0], 20, 0xe9b6c7aa);
        GG!(a, b, c, d, words[5], 5, 0xd62f105d);
        GG!(d, a, b, c, words[10], 9, 0x02441453);
        GG!(c, d, a, b, words[15], 14, 0xd8a1e681);
        GG!(b, c, d, a, words[4], 20, 0xe7d3fbc8);
        GG!(a, b, c, d, words[9], 5, 0x21e1cde6);
        GG!(d, a, b, c, words[14], 9, 0xc33707d6);
        GG!(c, d, a, b, words[3], 14, 0xf4d50d87);
        GG!(b, c, d, a, words[8], 20, 0x455a14ed);
        GG!(a, b, c, d, words[13], 5, 0xa9e3e905);
        GG!(d, a, b, c, words[2], 9, 0xfcefa3f8);
        GG!(c, d, a, b, words[7], 14, 0x676f02d9);
        GG!(b, c, d, a, words[12], 20, 0x8d2a4c8a);

        // Round 3
        macro_rules! H {
            ($x:expr, $y:expr, $z:expr) => {
                ($x ^ $y ^ $z)
            };
        }
        macro_rules! HH {
            ($a:expr, $b:expr, $c:expr, $d:expr, $x:expr, $s:expr, $ac:expr) => {
                $a = $a.wrapping_add(H!($b, $c, $d)).wrapping_add($x).wrapping_add($ac);
                $a = $a.rotate_left($s).wrapping_add($b);
            };
        }
        
        HH!(a, b, c, d, words[5], 4, 0xfffa3942);
        HH!(d, a, b, c, words[8], 11, 0x8771f681);
        HH!(c, d, a, b, words[11], 16, 0x6d9d6122);
        HH!(b, c, d, a, words[14], 23, 0xfde5380c);
        HH!(a, b, c, d, words[1], 4, 0xa4beea44);
        HH!(d, a, b, c, words[4], 11, 0x4bdecfa9);
        HH!(c, d, a, b, words[7], 16, 0xf6bb4b60);
        HH!(b, c, d, a, words[10], 23, 0xbebfbc70);
        HH!(a, b, c, d, words[13], 4, 0x289b7ec6);
        HH!(d, a, b, c, words[0], 11, 0xeaa127fa);
        HH!(c, d, a, b, words[3], 16, 0xd4ef3085);
        HH!(b, c, d, a, words[6], 23, 0x04881d05);
        HH!(a, b, c, d, words[9], 4, 0xd9d4d039);
        HH!(d, a, b, c, words[12], 11, 0xe6db99e5);
        HH!(c, d, a, b, words[15], 16, 0x1fa27cf8);
        HH!(b, c, d, a, words[2], 23, 0xc4ac5665);

        // Round 4
        macro_rules! I {
            ($x:expr, $y:expr, $z:expr) => {
                ($y ^ ($x | !$z))
            };
        }
        macro_rules! II {
            ($a:expr, $b:expr, $c:expr, $d:expr, $x:expr, $s:expr, $ac:expr) => {
                $a = $a.wrapping_add(I!($b, $c, $d)).wrapping_add($x).wrapping_add($ac);
                $a = $a.rotate_left($s).wrapping_add($b);
            };
        }
        
        II!(a, b, c, d, words[0], 6, 0xf4292244);
        II!(d, a, b, c, words[7], 10, 0x432aff97);
        II!(c, d, a, b, words[14], 15, 0xab9423a7);
        II!(b, c, d, a, words[5], 21, 0xfc93a039);
        II!(a, b, c, d, words[12], 6, 0x655b59c3);
        II!(d, a, b, c, words[3], 10, 0x8f0ccc92);
        II!(c, d, a, b, words[10], 15, 0xffeff47d);
        II!(b, c, d, a, words[1], 21, 0x85845dd1);
        II!(a, b, c, d, words[8], 6, 0x6fa87e4f);
        II!(d, a, b, c, words[15], 10, 0xfe2ce6e0);
        II!(c, d, a, b, words[6], 15, 0xa3014314);
        II!(b, c, d, a, words[13], 21, 0x4e0811a1);
        II!(a, b, c, d, words[4], 6, 0xf7537e82);
        II!(d, a, b, c, words[11], 10, 0xbd3af235);
        II!(c, d, a, b, words[2], 15, 0x2ad7d2bb);
        II!(b, c, d, a, words[9], 21, 0xeb86d391);

        a = a.wrapping_add(saved_a);
        b = b.wrapping_add(saved_b);
        c = c.wrapping_add(saved_c);
        d = d.wrapping_add(saved_d);
    }

    ctx.a = a;
    ctx.b = b;
    ctx.c = c;
    ctx.d = d;

    data.len()
}

pub fn md5_init(ctx: &mut MD5_CTX) {
    ctx.a = 0x67452301;
    ctx.b = 0xefcdab89;
    ctx.c = 0x98badcfe;
    ctx.d = 0x10325476;
    ctx.lo = 0;
    ctx.hi = 0;
}

pub fn md5_update(ctx: &mut MD5_CTX, data: &[u8]) {
    let saved_lo = ctx.lo;
    ctx.lo = (saved_lo as u64 + data.len() as u64) as u32;
    if ctx.lo < saved_lo {
        ctx.hi += 1;
    }
    ctx.hi = (ctx.hi as u64 + (data.len() >> 29) as u64) as u32;

    let used = (saved_lo & 0x3f) as usize;
    if used != 0 {
        let free = 64 - used;
        if data.len() < free {
            ctx.buffer[used..used + data.len()].copy_from_slice(data);
            return;
        }

        ctx.buffer[used..used + free].copy_from_slice(&data[..free]);
        body(ctx, &ctx.buffer[..64]);
    }

    let aligned_len = data.len() & !0x3f;
    if aligned_len > 0 {
        body(ctx, &data[..aligned_len]);
    }

    let remaining = data.len() - aligned_len;
    if remaining > 0 {
        ctx.buffer[..remaining].copy_from_slice(&data[aligned_len..]);
    }
}

pub fn md5_final(ctx: &mut MD5_CTX, result: &mut [u8; 16]) {
    let used = (ctx.lo & 0x3f) as usize;
    ctx.buffer[used] = 0x80;
    let free = 64 - used;

    if free < 8 {
        ctx.buffer[used + 1..used + free].fill(0);
        body(ctx, &ctx.buffer[..64]);
        ctx.buffer[..56].fill(0);
    } else {
        ctx.buffer[used + 1..used + free - 8].fill(0);
    }

    let bits = (ctx.lo as u64) << 3 | (ctx.hi as u64) << 35;
    ctx.buffer[56..64].copy_from_slice(&bits.to_le_bytes());

    body(ctx, &ctx.buffer[..64]);

    result[0..4].copy_from_slice(&ctx.a.to_le_bytes());
    result[4..8].copy_from_slice(&ctx.b.to_le_bytes());
    result[8..12].copy_from_slice(&ctx.c.to_le_bytes());
    result[12..16].copy_from_slice(&ctx.d.to_le_bytes());

    *ctx = MD5_CTX {
        lo: 0,
        hi: 0,
        a: 0,
        b: 0,
        c: 0,
        d: 0,
        buffer: [0; 64],
        block: [0; 16],
    };
}

pub fn md5_signature(key: &[u8]) -> [u8; 16] {
    let mut ctx = MD5_CTX {
        lo: 0,
        hi: 0,
        a: 0,
        b: 0,
        c: 0,
        d: 0,
        buffer: [0; 64],
        block: [0; 16],
    };
    md5_init(&mut ctx);
    md5_update(&mut ctx, key);
    let mut result = [0u8; 16];
    md5_final(&mut ctx, &mut result);
    result
}

pub fn hash_md5(key: &str) -> u32 {
    let results = md5_signature(key.as_bytes());
    u32::from_le_bytes(results[0..4].try_into().unwrap())
}