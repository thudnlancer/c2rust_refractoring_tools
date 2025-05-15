use std::convert::TryInto;
use std::io::{self, Read};
use std::mem;

const SHA384_DIGEST_SIZE: usize = 384 / 8;
const SHA512_DIGEST_SIZE: usize = 512 / 8;

#[derive(Clone)]
struct Sha512Context {
    state: [u64; 8],
    total: [u64; 2],
    buflen: usize,
    buffer: [u64; 32],
}

impl Sha512Context {
    fn new() -> Self {
        Sha512Context {
            state: [0; 8],
            total: [0; 2],
            buflen: 0,
            buffer: [0; 32],
        }
    }
}

const SHA512_ROUND_CONSTANTS: [u64; 80] = [
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

fn sha512_init_ctx(ctx: &mut Sha512Context) {
    ctx.state[0] = 0x6a09e667f3bcc908;
    ctx.state[1] = 0xbb67ae8584caa73b;
    ctx.state[2] = 0x3c6ef372fe94f82b;
    ctx.state[3] = 0xa54ff53a5f1d36f1;
    ctx.state[4] = 0x510e527fade682d1;
    ctx.state[5] = 0x9b05688c2b3e6c1f;
    ctx.state[6] = 0x1f83d9abfb41bd6b;
    ctx.state[7] = 0x5be0cd19137e2179;

    ctx.total[0] = 0;
    ctx.total[1] = 0;
    ctx.buflen = 0;
}

fn sha384_init_ctx(ctx: &mut Sha512Context) {
    ctx.state[0] = 0xcbbb9d5dc1059ed8;
    ctx.state[1] = 0x629a292a367cd507;
    ctx.state[2] = 0x9159015a3070dd17;
    ctx.state[3] = 0x152fecd8f70e5939;
    ctx.state[4] = 0x67332667ffc00b31;
    ctx.state[5] = 0x8eb44a8768581511;
    ctx.state[6] = 0xdb0c2e0d64f98fa7;
    ctx.state[7] = 0x47b5481dbefa4fa4;

    ctx.total[0] = 0;
    ctx.total[1] = 0;
    ctx.buflen = 0;
}

fn sha512_read_ctx(ctx: &Sha512Context, resbuf: &mut [u8]) {
    for (i, &word) in ctx.state.iter().enumerate() {
        resbuf[i * 8..(i + 1) * 8].copy_from_slice(&word.to_be_bytes());
    }
}

fn sha384_read_ctx(ctx: &Sha512Context, resbuf: &mut [u8]) {
    for (i, &word) in ctx.state.iter().take(6).enumerate() {
        resbuf[i * 8..(i + 1) * 8].copy_from_slice(&word.to_be_bytes());
    }
}

fn sha512_conclude_ctx(ctx: &mut Sha512Context) {
    let bytes = ctx.buflen;
    let size = if bytes < 112 { 128 / 8 } else { 128 * 2 / 8 };

    ctx.total[0] = ctx.total[0].wrapping_add(bytes as u64);
    if ctx.total[0] < bytes as u64 {
        ctx.total[1] = ctx.total[1].wrapping_add(1);
    }

    let len_bits = (ctx.total[1] << 3) | (ctx.total[0] >> 61);
    ctx.buffer[size - 2] = len_bits.to_be();
    ctx.buffer[size - 1] = (ctx.total[0] << 3).to_be();

    let fillbuf = [0x80u8];
    let buffer_slice = unsafe {
        std::slice::from_raw_parts_mut(
            ctx.buffer.as_mut_ptr() as *mut u8,
            ctx.buffer.len() * 8,
        )
    };
    buffer_slice[bytes..bytes + fillbuf.len()].copy_from_slice(&fillbuf);

    sha512_process_block(&ctx.buffer, size * 8, ctx);
}

fn sha512_finish_ctx(ctx: &mut Sha512Context, resbuf: &mut [u8]) {
    sha512_conclude_ctx(ctx);
    sha512_read_ctx(ctx, resbuf);
}

fn sha384_finish_ctx(ctx: &mut Sha512Context, resbuf: &mut [u8]) {
    sha512_conclude_ctx(ctx);
    sha384_read_ctx(ctx, resbuf);
}

fn sha512_buffer(buffer: &[u8], resblock: &mut [u8]) {
    let mut ctx = Sha512Context::new();
    sha512_init_ctx(&mut ctx);
    sha512_process_bytes(buffer, &mut ctx);
    sha512_finish_ctx(&mut ctx, resblock);
}

fn sha384_buffer(buffer: &[u8], resblock: &mut [u8]) {
    let mut ctx = Sha512Context::new();
    sha384_init_ctx(&mut ctx);
    sha512_process_bytes(buffer, &mut ctx);
    sha384_finish_ctx(&mut ctx, resblock);
}

fn sha512_process_bytes(buffer: &[u8], ctx: &mut Sha512Context) {
    let mut len = buffer.len();
    let mut offset = 0;

    if ctx.buflen != 0 {
        let left_over = ctx.buflen;
        let add = std::cmp::min(256 - left_over, len);

        let buffer_slice = unsafe {
            std::slice::from_raw_parts_mut(
                ctx.buffer.as_mut_ptr() as *mut u8,
                ctx.buffer.len() * 8,
            )
        };
        buffer_slice[left_over..left_over + add].copy_from_slice(&buffer[..add]);

        ctx.buflen += add;
        offset += add;
        len -= add;

        if ctx.buflen > 128 {
            sha512_process_block(&ctx.buffer, ctx.buflen & !127, ctx);
            ctx.buflen &= 127;

            let copy_len = ctx.buflen;
            buffer_slice[..copy_len].copy_from_slice(
                &buffer_slice[(left_over + add) & !127..(left_over + add) & !127 + copy_len],
            );
        }
    }

    if len >= 128 {
        sha512_process_block(&buffer[offset..offset + (len & !127)], len & !127, ctx);
        offset += len & !127;
        len &= 127;
    }

    if len > 0 {
        let buffer_slice = unsafe {
            std::slice::from_raw_parts_mut(
                ctx.buffer.as_mut_ptr() as *mut u8,
                ctx.buffer.len() * 8,
            )
        };
        buffer_slice[ctx.buflen..ctx.buflen + len].copy_from_slice(&buffer[offset..]);
        ctx.buflen += len;
    }
}

fn sha512_process_block(buffer: &[u8], len: usize, ctx: &mut Sha512Context) {
    let words = unsafe {
        std::slice::from_raw_parts(
            buffer.as_ptr() as *const u64,
            len / 8,
        )
    };

    let mut a = ctx.state[0];
    let mut b = ctx.state[1];
    let mut c = ctx.state[2];
    let mut d = ctx.state[3];
    let mut e = ctx.state[4];
    let mut f = ctx.state[5];
    let mut g = ctx.state[6];
    let mut h = ctx.state[7];

    let lolen = len as u64;

    ctx.total[0] = ctx.total[0].wrapping_add(lolen);
    if ctx.total[0] < lolen {
        ctx.total[1] = ctx.total[1].wrapping_add(1);
    }

    macro_rules! S0 {
        ($x:expr) => {
            $x.rotate_right(63) ^ $x.rotate_right(56) ^ ($x >> 7)
        };
    }

    macro_rules! S1 {
        ($x:expr) => {
            $x.rotate_right(45) ^ $x.rotate_right(3) ^ ($x >> 6)
        };
    }

    macro_rules! SS0 {
        ($x:expr) => {
            $x.rotate_right(36) ^ $x.rotate_right(30) ^ $x.rotate_right(25)
        };
    }

    macro_rules! SS1 {
        ($x:expr) => {
            $x.rotate_right(50) ^ $x.rotate_right(46) ^ $x.rotate_right(23)
        };
    }

    macro_rules! F2 {
        ($a:expr, $b:expr, $c:expr) => {
            ($a & $b) | ($c & ($a | $b))
        };
    }

    macro_rules! F1 {
        ($e:expr, $f:expr, $g:expr) => {
            $g ^ ($e & ($f ^ $g))
        };
    }

    macro_rules! M {
        ($i:expr, $x:expr) => {
            $x[$i & 15] = $x[$i & 15].wrapping_add(
                S1!($x[($i - 2) & 15])
                .wrapping_add($x[($i - 7) & 15])
                .wrapping_add(S0!($x[($i - 15) & 15])),
        };
    }

    macro_rules! R {
        ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr, $g:expr, $h:expr, $k:expr, $m:expr) => {
            let t0 = SS0!($a).wrapping_add(F2!($a, $b, $c));
            let t1 = $h
                .wrapping_add(SS1!($e))
                .wrapping_add(F1!($e, $f, $g))
                .wrapping_add($k)
                .wrapping_add($m);
            $d = $d.wrapping_add(t1);
            $h = t0.wrapping_add(t1);
        };
    }

    let mut x = [0u64; 16];
    for chunk in words.chunks(16) {
        for (i, &word) in chunk.iter().enumerate() {
            x[i] = u64::from_be(word);
        }

        for i in 0..16 {
            R!(a, b, c, d, e, f, g, h, SHA512_ROUND_CONSTANTS[i], x[i]);
            mem::swap(&mut h, &mut g);
            mem::swap(&mut g, &mut f);
            mem::swap(&mut f, &mut e);
            mem::swap(&mut e, &mut d);
            mem::swap(&mut d, &mut c);
            mem::swap(&mut c, &mut b);
            mem::swap(&mut b, &mut a);
        }

        for i in 16..80 {
            M!(i, x);
            R!(a, b, c, d, e, f, g, h, SHA512_ROUND_CONSTANTS[i], x[i & 15]);
            mem::swap(&mut h, &mut g);
            mem::swap(&mut g, &mut f);
            mem::swap(&mut f, &mut e);
            mem::swap(&mut e, &mut d);
            mem::swap(&mut d, &mut c);
            mem::swap(&mut c, &mut b);
            mem::swap(&mut b, &mut a);
        }

        ctx.state[0] = ctx.state[0].wrapping_add(a);
        ctx.state[1] = ctx.state[1].wrapping_add(b);
        ctx.state[2] = ctx.state[2].wrapping_add(c);
        ctx.state[3] = ctx.state[3].wrapping_add(d);
        ctx.state[4] = ctx.state[4].wrapping_add(e);
        ctx.state[5] = ctx.state[5].wrapping_add(f);
        ctx.state[6] = ctx.state[6].wrapping_add(g);
        ctx.state[7] = ctx.state[7].wrapping_add(h);
    }
}

fn sha512_stream(stream: &mut dyn Read, resblock: &mut [u8]) -> io::Result<()> {
    let mut ctx = Sha512Context::new();
    sha512_init_ctx(&mut ctx);

    let mut buffer = [0; 4096];
    loop {
        let bytes_read = stream.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        sha512_process_bytes(&buffer[..bytes_read], &mut ctx);
    }

    sha512_finish_ctx(&mut ctx, resblock);
    Ok(())
}

fn sha384_stream(stream: &mut dyn Read, resblock: &mut [u8]) -> io::Result<()> {
    let mut ctx = Sha512Context::new();
    sha384_init_ctx(&mut ctx