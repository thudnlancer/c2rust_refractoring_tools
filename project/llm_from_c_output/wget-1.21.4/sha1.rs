use std::convert::TryInto;
use std::io::{self, Read};
use std::mem;

const SHA1_DIGEST_SIZE: usize = 20;

#[derive(Debug, Clone)]
struct Sha1Context {
    a: u32,
    b: u32,
    c: u32,
    d: u32,
    e: u32,
    total: [u32; 2],
    buflen: usize,
    buffer: [u32; 32],
}

impl Sha1Context {
    fn new() -> Self {
        Sha1Context {
            a: 0x67452301,
            b: 0xEFCDAB89,
            c: 0x98BADCFE,
            d: 0x10325476,
            e: 0xC3D2E1F0,
            total: [0, 0],
            buflen: 0,
            buffer: [0; 32],
        }
    }

    fn process_block(&mut self, block: &[u8]) {
        assert!(block.len() % 64 == 0);

        let mut words = [0u32; 16];
        for (i, chunk) in block.chunks_exact(4).enumerate() {
            words[i] = u32::from_be_bytes(chunk.try_into().unwrap());
        }

        let mut a = self.a;
        let mut b = self.b;
        let mut c = self.c;
        let mut d = self.d;
        let mut e = self.e;

        macro_rules! rol {
            ($x:expr, $n:expr) => {
                ($x << $n) | ($x >> (32 - $n))
            };
        }

        macro_rules! f1 {
            ($b:expr, $c:expr, $d:expr) => {
                $d ^ ($b & ($c ^ $d))
            };
        }
        macro_rules! f2 {
            ($b:expr, $c:expr, $d:expr) => {
                $b ^ $c ^ $d
            };
        }
        macro_rules! f3 {
            ($b:expr, $c:expr, $d:expr) => {
                ($b & $c) | ($d & ($b | $c))
            };
        }

        macro_rules! round {
            ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:ident, $k:expr, $m:expr) => {
                $e = $e.wrapping_add(rol!($a, 5)).wrapping_add($f!($b, $c, $d)).wrapping_add($k).wrapping_add($m);
                $b = rol!($b, 30);
            };
        }

        for i in 0..16 {
            let t = i;
            round!(a, b, c, d, e, f1, 0x5A827999, words[t]);
            round!(e, a, b, c, d, f1, 0x5A827999, words[t + 1]);
            round!(d, e, a, b, c, f1, 0x5A827999, words[t + 2]);
            round!(c, d, e, a, b, f1, 0x5A827999, words[t + 3]);
            round!(b, c, d, e, a, f1, 0x5A827999, words[t + 4]);
        }

        for i in 16..20 {
            let t = i % 16;
            let word = words[t] ^ words[(t + 2) % 16] ^ words[(t + 8) % 16] ^ words[(t + 13) % 16];
            let word = rol!(word, 1);
            words[t] = word;

            round!(a, b, c, d, e, f1, 0x5A827999, word);
            round!(e, a, b, c, d, f1, 0x5A827999, words[(t + 1) % 16]);
            round!(d, e, a, b, c, f1, 0x5A827999, words[(t + 2) % 16]);
            round!(c, d, e, a, b, f1, 0x5A827999, words[(t + 3) % 16]);
            round!(b, c, d, e, a, f1, 0x5A827999, words[(t + 4) % 16]);
        }

        for i in 20..40 {
            let t = i % 16;
            let word = words[t] ^ words[(t + 2) % 16] ^ words[(t + 8) % 16] ^ words[(t + 13) % 16];
            let word = rol!(word, 1);
            words[t] = word;

            round!(a, b, c, d, e, f2, 0x6ED9EBA1, word);
            round!(e, a, b, c, d, f2, 0x6ED9EBA1, words[(t + 1) % 16]);
            round!(d, e, a, b, c, f2, 0x6ED9EBA1, words[(t + 2) % 16]);
            round!(c, d, e, a, b, f2, 0x6ED9EBA1, words[(t + 3) % 16]);
            round!(b, c, d, e, a, f2, 0x6ED9EBA1, words[(t + 4) % 16]);
        }

        for i in 40..60 {
            let t = i % 16;
            let word = words[t] ^ words[(t + 2) % 16] ^ words[(t + 8) % 16] ^ words[(t + 13) % 16];
            let word = rol!(word, 1);
            words[t] = word;

            round!(a, b, c, d, e, f3, 0x8F1BBCDC, word);
            round!(e, a, b, c, d, f3, 0x8F1BBCDC, words[(t + 1) % 16]);
            round!(d, e, a, b, c, f3, 0x8F1BBCDC, words[(t + 2) % 16]);
            round!(c, d, e, a, b, f3, 0x8F1BBCDC, words[(t + 3) % 16]);
            round!(b, c, d, e, a, f3, 0x8F1BBCDC, words[(t + 4) % 16]);
        }

        for i in 60..80 {
            let t = i % 16;
            let word = words[t] ^ words[(t + 2) % 16] ^ words[(t + 8) % 16] ^ words[(t + 13) % 16];
            let word = rol!(word, 1);
            words[t] = word;

            round!(a, b, c, d, e, f2, 0xCA62C1D6, word);
            round!(e, a, b, c, d, f2, 0xCA62C1D6, words[(t + 1) % 16]);
            round!(d, e, a, b, c, f2, 0xCA62C1D6, words[(t + 2) % 16]);
            round!(c, d, e, a, b, f2, 0xCA62C1D6, words[(t + 3) % 16]);
            round!(b, c, d, e, a, f2, 0xCA62C1D6, words[(t + 4) % 16]);
        }

        self.a = self.a.wrapping_add(a);
        self.b = self.b.wrapping_add(b);
        self.c = self.c.wrapping_add(c);
        self.d = self.d.wrapping_add(d);
        self.e = self.e.wrapping_add(e);
    }

    fn process_bytes(&mut self, data: &[u8]) {
        let mut offset = 0;
        let len = data.len();

        if self.buflen > 0 {
            let to_copy = std::cmp::min(64 - self.buflen, len);
            let buf_start = self.buflen / 4;
            let buf_remain = self.buflen % 4;

            if buf_remain > 0 {
                let mut tmp = [0u8; 4];
                let copy_len = std::cmp::min(4 - buf_remain, to_copy);
                tmp[buf_remain..buf_remain + copy_len].copy_from_slice(&data[..copy_len]);
                self.buffer[buf_start] |= u32::from_be_bytes(tmp);
                offset += copy_len;
                self.buflen += copy_len;
            }

            while offset < to_copy {
                let chunk = &data[offset..offset + 4];
                self.buffer[buf_start + offset / 4] = u32::from_be_bytes(chunk.try_into().unwrap());
                offset += 4;
                self.buflen += 4;
            }

            if self.buflen == 64 {
                let block = unsafe {
                    std::slice::from_raw_parts(
                        self.buffer.as_ptr() as *const u8,
                        64,
                    )
                };
                self.process_block(block);
                self.buflen = 0;
            }
        }

        while offset + 64 <= len {
            let block = &data[offset..offset + 64];
            self.process_block(block);
            offset += 64;
        }

        if offset < len {
            let remaining = &data[offset..];
            let buf_start = self.buflen / 4;
            let buf_remain = self.buflen % 4;

            if buf_remain > 0 {
                let mut tmp = [0u8; 4];
                let copy_len = std::cmp::min(4 - buf_remain, remaining.len());
                tmp[buf_remain..buf_remain + copy_len].copy_from_slice(&remaining[..copy_len]);
                self.buffer[buf_start] |= u32::from_be_bytes(tmp);
                self.buflen += copy_len;
                offset += copy_len;
            }

            while offset < len {
                let chunk = &data[offset..std::cmp::min(offset + 4, len)];
                let mut tmp = [0u8; 4];
                tmp[..chunk.len()].copy_from_slice(chunk);
                self.buffer[buf_start + (offset - (self.buflen % 4)) / 4] = u32::from_be_bytes(tmp);
                offset += chunk.len();
                self.buflen += chunk.len();
            }
        }

        self.total[0] = self.total[0].wrapping_add(len as u32);
        if self.total[0] < len as u32 {
            self.total[1] = self.total[1].wrapping_add(1);
        }
    }

    fn finish(mut self) -> [u8; SHA1_DIGEST_SIZE] {
        let len_bits = (self.total[1] << 3) | (self.total[0] >> 29);
        let len_bits_lo = self.total[0] << 3;

        let pad_len = if self.buflen < 56 {
            56 - self.buflen
        } else {
            120 - self.buflen
        };

        let mut padding = vec![0u8; pad_len + 8];
        padding[0] = 0x80;
        padding[pad_len..pad_len + 4].copy_from_slice(&len_bits.to_be_bytes());
        padding[pad_len + 4..pad_len + 8].copy_from_slice(&len_bits_lo.to_be_bytes());

        self.process_bytes(&padding);

        let mut digest = [0u8; SHA1_DIGEST_SIZE];
        digest[..4].copy_from_slice(&self.a.to_be_bytes());
        digest[4..8].copy_from_slice(&self.b.to_be_bytes());
        digest[8..12].copy_from_slice(&self.c.to_be_bytes());
        digest[12..16].copy_from_slice(&self.d.to_be_bytes());
        digest[16..20].copy_from_slice(&self.e.to_be_bytes());

        digest
    }
}

pub fn sha1(data: &[u8]) -> [u8; SHA1_DIGEST_SIZE] {
    let mut ctx = Sha1Context::new();
    ctx.process_bytes(data);
    ctx.finish()
}

pub fn sha1_stream<R: Read>(mut reader: R) -> io::Result<[u8; SHA1_DIGEST_SIZE]> {
    let mut ctx = Sha1Context::new();
    let mut buffer = [0u8; 4096];

    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        ctx.process_bytes(&buffer[..bytes_read]);
    }

    Ok(ctx.finish())
}