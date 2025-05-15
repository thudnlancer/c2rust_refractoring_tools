/*
 * twemproxy - A fast and lightweight proxy for memcached protocol.
 * Copyright (C) 2011 Twitter, Inc.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use std::convert::TryInto;

type MD5u32 = u32;

#[derive(Default)]
struct MD5Context {
    lo: MD5u32,
    hi: MD5u32,
    a: MD5u32,
    b: MD5u32,
    c: MD5u32,
    d: MD5u32,
    buffer: [u8; 64],
    block: [MD5u32; 16],
}

/*
 * The basic MD5 functions.
 *
 * F and G are optimized compared to their RFC 1321 definitions for
 * architectures that lack an AND-NOT instruction, just like in Colin Plumb's
 * implementation.
 */
fn f(x: MD5u32, y: MD5u32, z: MD5u32) -> MD5u32 {
    z ^ (x & (y ^ z))
}

fn g(x: MD5u32, y: MD5u32, z: MD5u32) -> MD5u32 {
    y ^ (z & (x ^ y))
}

fn h(x: MD5u32, y: MD5u32, z: MD5u32) -> MD5u32 {
    x ^ y ^ z
}

fn i(x: MD5u32, y: MD5u32, z: MD5u32) -> MD5u32 {
    y ^ (x | !z)
}

/*
 * The MD5 transformation for all four rounds.
 */
fn step(
    f: fn(MD5u32, MD5u32, MD5u32) -> MD5u32,
    a: &mut MD5u32,
    b: MD5u32,
    c: MD5u32,
    d: MD5u32,
    x: MD5u32,
    t: MD5u32,
    s: u32,
) {
    *a = a.wrapping_add(f(b, c, d)).wrapping_add(x).wrapping_add(t);
    *a = a.rotate_left(s);
    *a = a.wrapping_add(b);
}

/*
 * This processes one or more 64-byte data blocks, but does NOT update
 * the bit counters.  There are no alignment requirements.
 */
fn body(ctx: &mut MD5Context, data: &[u8]) -> usize {
    let mut a = ctx.a;
    let mut b = ctx.b;
    let mut c = ctx.c;
    let mut d = ctx.d;

    let mut processed = 0;
    let chunks = data.chunks_exact(64);

    for chunk in chunks {
        processed += 64;
        let saved_a = a;
        let saved_b = b;
        let saved_c = c;
        let saved_d = d;

        // Prepare the block
        let mut block = [0u32; 16];
        for (i, word) in chunk.chunks_exact(4).enumerate() {
            block[i] = u32::from_le_bytes(word.try_into().unwrap());
        }

        /* Round 1 */
        step(f, &mut a, b, c, d, block[0], 0xd76aa478, 7);
        step(f, &mut d, a, b, c, block[1], 0xe8c7b756, 12);
        step(f, &mut c, d, a, b, block[2], 0x242070db, 17);
        step(f, &mut b, c, d, a, block[3], 0xc1bdceee, 22);
        step(f, &mut a, b, c, d, block[4], 0xf57c0faf, 7);
        step(f, &mut d, a, b, c, block[5], 0x4787c62a, 12);
        step(f, &mut c, d, a, b, block[6], 0xa8304613, 17);
        step(f, &mut b, c, d, a, block[7], 0xfd469501, 22);
        step(f, &mut a, b, c, d, block[8], 0x698098d8, 7);
        step(f, &mut d, a, b, c, block[9], 0x8b44f7af, 12);
        step(f, &mut c, d, a, b, block[10], 0xffff5bb1, 17);
        step(f, &mut b, c, d, a, block[11], 0x895cd7be, 22);
        step(f, &mut a, b, c, d, block[12], 0x6b901122, 7);
        step(f, &mut d, a, b, c, block[13], 0xfd987193, 12);
        step(f, &mut c, d, a, b, block[14], 0xa679438e, 17);
        step(f, &mut b, c, d, a, block[15], 0x49b40821, 22);

        /* Round 2 */
        step(g, &mut a, b, c, d, block[1], 0xf61e2562, 5);
        step(g, &mut d, a, b, c, block[6], 0xc040b340, 9);
        step(g, &mut c, d, a, b, block[11], 0x265e5a51, 14);
        step(g, &mut b, c, d, a, block[0], 0xe9b6c7aa, 20);
        step(g, &mut a, b, c, d, block[5], 0xd62f105d, 5);
        step(g, &mut d, a, b, c, block[10], 0x02441453, 9);
        step(g, &mut c, d, a, b, block[15], 0xd8a1e681, 14);
        step(g, &mut b, c, d, a, block[4], 0xe7d3fbc8, 20);
        step(g, &mut a, b, c, d, block[9], 0x21e1cde6, 5);
        step(g, &mut d, a, b, c, block[14], 0xc33707d6, 9);
        step(g, &mut c, d, a, b, block[3], 0xf4d50d87, 14);
        step(g, &mut b, c, d, a, block[8], 0x455a14ed, 20);
        step(g, &mut a, b, c, d, block[13], 0xa9e3e905, 5);
        step(g, &mut d, a, b, c, block[2], 0xfcefa3f8, 9);
        step(g, &mut c, d, a, b, block[7], 0x676f02d9, 14);
        step(g, &mut b, c, d, a, block[12], 0x8d2a4c8a, 20);

        /* Round 3 */
        step(h, &mut a, b, c, d, block[5], 0xfffa3942, 4);
        step(h, &mut d, a, b, c, block[8], 0x8771f681, 11);
        step(h, &mut c, d, a, b, block[11], 0x6d9d6122, 16);
        step(h, &mut b, c, d, a, block[14], 0xfde5380c, 23);
        step(h, &mut a, b, c, d, block[1], 0xa4beea44, 4);
        step(h, &mut d, a, b, c, block[4], 0x4bdecfa9, 11);
        step(h, &mut c, d, a, b, block[7], 0xf6bb4b60, 16);
        step(h, &mut b, c, d, a, block[10], 0xbebfbc70, 23);
        step(h, &mut a, b, c, d, block[13], 0x289b7ec6, 4);
        step(h, &mut d, a, b, c, block[0], 0xeaa127fa, 11);
        step(h, &mut c, d, a, b, block[3], 0xd4ef3085, 16);
        step(h, &mut b, c, d, a, block[6], 0x04881d05, 23);
        step(h, &mut a, b, c, d, block[9], 0xd9d4d039, 4);
        step(h, &mut d, a, b, c, block[12], 0xe6db99e5, 11);
        step(h, &mut c, d, a, b, block[15], 0x1fa27cf8, 16);
        step(h, &mut b, c, d, a, block[2], 0xc4ac5665, 23);

        /* Round 4 */
        step(i, &mut a, b, c, d, block[0], 0xf4292244, 6);
        step(i, &mut d, a, b, c, block[7], 0x432aff97, 10);
        step(i, &mut c, d, a, b, block[14], 0xab9423a7, 15);
        step(i, &mut b, c, d, a, block[5], 0xfc93a039, 21);
        step(i, &mut a, b, c, d, block[12], 0x655b59c3, 6);
        step(i, &mut d, a, b, c, block[3], 0x8f0ccc92, 10);
        step(i, &mut c, d, a, b, block[10], 0xffeff47d, 15);
        step(i, &mut b, c, d, a, block[1], 0x85845dd1, 21);
        step(i, &mut a, b, c, d, block[8], 0x6fa87e4f, 6);
        step(i, &mut d, a, b, c, block[15], 0xfe2ce6e0, 10);
        step(i, &mut c, d, a, b, block[6], 0xa3014314, 15);
        step(i, &mut b, c, d, a, block[13], 0x4e0811a1, 21);
        step(i, &mut a, b, c, d, block[4], 0xf7537e82, 6);
        step(i, &mut d, a, b, c, block[11], 0xbd3af235, 10);
        step(i, &mut c, d, a, b, block[2], 0x2ad7d2bb, 15);
        step(i, &mut b, c, d, a, block[9], 0xeb86d391, 21);

        a = a.wrapping_add(saved_a);
        b = b.wrapping_add(saved_b);
        c = c.wrapping_add(saved_c);
        d = d.wrapping_add(saved_d);
    }

    ctx.a = a;
    ctx.b = b;
    ctx.c = c;
    ctx.d = d;

    processed
}

fn md5_init(ctx: &mut MD5Context) {
    ctx.a = 0x67452301;
    ctx.b = 0xefcdab89;
    ctx.c = 0x98badcfe;
    ctx.d = 0x10325476;

    ctx.lo = 0;
    ctx.hi = 0;
}

fn md5_update(ctx: &mut MD5Context, data: &[u8]) {
    let saved_lo = ctx.lo;
    ctx.lo = saved_lo.wrapping_add(data.len() as u32) & 0x1fffffff;
    if ctx.lo < saved_lo {
        ctx.hi += 1;
    }
    ctx.hi += (data.len() >> 29) as u32;

    let used = (saved_lo & 0x3f) as usize;

    if used > 0 {
        let free = 64 - used;

        if data.len() < free {
            ctx.buffer[used..used + data.len()].copy_from_slice(data);
            return;
        }

        ctx.buffer[used..used + free].copy_from_slice(&data[..free]);
        body(ctx, &ctx.buffer);
    }

    let remaining = data.len() - used.min(data.len());
    if remaining >= 64 {
        let processed = body(ctx, &data[free..]);
        let remaining = remaining - processed;
        if remaining > 0 {
            ctx.buffer[..remaining].copy_from_slice(&data[free + processed..]);
        }
    } else if remaining > 0 {
        ctx.buffer[..remaining].copy_from_slice(&data[free..]);
    }
}

fn md5_final(ctx: &mut MD5Context) -> [u8; 16] {
    let used = (ctx.lo & 0x3f) as usize;
    ctx.buffer[used] = 0x80;
    let free = 64 - used;

    if free < 8 {
        ctx.buffer[used + 1..used + free].fill(0);
        body(ctx, &ctx.buffer);
        ctx.buffer.fill(0);
    } else {
        ctx.buffer[used + 1..used + free - 8].fill(0);
    }

    let bits = (ctx.lo << 3) as u64 | ((ctx.hi as u64) << 32);
    ctx.buffer[56..64].copy_from_slice(&bits.to_le_bytes());

    body(ctx, &ctx.buffer);

    let mut result = [0u8; 16];
    result[0..4].copy_from_slice(&ctx.a.to_le_bytes());
    result[4..8].copy_from_slice(&ctx.b.to_le_bytes());
    result[8..12].copy_from_slice(&ctx.c.to_le_bytes());
    result[12..16].copy_from_slice(&ctx.d.to_le_bytes());

    *ctx = MD5Context::default();
    result
}

/*
 * Just a simple method for getting the signature
 */
pub fn md5_signature(key: &[u8]) -> [u8; 16] {
    let mut ctx = MD5Context::default();
    md5_init(&mut ctx);
    md5_update(&mut ctx, key);
    md5_final(&mut ctx)
}

pub fn hash_md5(key: &[u8]) -> u32 {
    let result = md5_signature(key);
    u32::from_le_bytes([result[0], result[1], result[2], result[3]])
}