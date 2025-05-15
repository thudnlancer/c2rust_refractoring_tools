use std::convert::TryInto;
use std::io::{self, Read};
use std::mem;

pub const MD5_DIGEST_SIZE: usize = 16;
pub const MD5_BLOCK_SIZE: usize = 64;

#[derive(Clone, Copy, Debug, Default)]
struct Md5Context {
    a: u32,
    b: u32,
    c: u32,
    d: u32,
    total: [u32; 2],
    buffer: [u32; 16],
    buflen: usize,
}

impl Md5Context {
    fn new() -> Self {
        Self {
            a: 0x67452301,
            b: 0xefcdab89,
            c: 0x98badcfe,
            d: 0x10325476,
            total: [0, 0],
            buffer: [0; 16],
            buflen: 0,
        }
    }

    fn process_block(&mut self, input: &[u8]) {
        let mut a = self.a;
        let mut b = self.b;
        let mut c = self.c;
        let mut d = self.d;

        let mut words = [0u32; 16];
        for (i, word) in words.iter_mut().enumerate() {
            let offset = i * 4;
            *word = u32::from_le_bytes([
                input[offset],
                input[offset + 1],
                input[offset + 2],
                input[offset + 3],
            ]);
        }

        macro_rules! round1 {
            ($a:expr, $b:expr, $c:expr, $d:expr, $k:expr, $s:expr, $t:expr) => {
                $a = $b.wrapping_add(
                    ($a.wrapping_add(($b & $c) | (!$b & $d))
                        .wrapping_add($k)
                        .wrapping_add($t)
                        .rotate_left($s),
                );
            };
        }

        macro_rules! round2 {
            ($a:expr, $b:expr, $c:expr, $d:expr, $k:expr, $s:expr, $t:expr) => {
                $a = $b.wrapping_add(
                    ($a.wrapping_add(($b & $d) | ($c & !$d))
                        .wrapping_add($k)
                        .wrapping_add($t)
                        .rotate_left($s)),
                );
            };
        }

        macro_rules! round3 {
            ($a:expr, $b:expr, $c:expr, $d:expr, $k:expr, $s:expr, $t:expr) => {
                $a = $b.wrapping_add(
                    ($a.wrapping_add($b ^ $c ^ $d)
                        .wrapping_add($k)
                        .wrapping_add($t)
                        .rotate_left($s),
                );
            };
        }

        macro_rules! round4 {
            ($a:expr, $b:expr, $c:expr, $d:expr, $k:expr, $s:expr, $t:expr) => {
                $a = $b.wrapping_add(
                    ($a.wrapping_add($c ^ ($b | !$d))
                        .wrapping_add($k)
                        .wrapping_add($t)
                        .rotate_left($s),
                );
            };
        }

        // Round 1
        round1!(a, b, c, d, words[0], 7, 0xd76aa478);
        round1!(d, a, b, c, words[1], 12, 0xe8c7b756);
        round1!(c, d, a, b, words[2], 17, 0x242070db);
        round1!(b, c, d, a, words[3], 22, 0xc1bdceee);
        round1!(a, b, c, d, words[4], 7, 0xf57c0faf);
        round1!(d, a, b, c, words[5], 12, 0x4787c62a);
        round1!(c, d, a, b, words[6], 17, 0xa8304613);
        round1!(b, c, d, a, words[7], 22, 0xfd469501);
        round1!(a, b, c, d, words[8], 7, 0x698098d8);
        round1!(d, a, b, c, words[9], 12, 0x8b44f7af);
        round1!(c, d, a, b, words[10], 17, 0xffff5bb1);
        round1!(b, c, d, a, words[11], 22, 0x895cd7be);
        round1!(a, b, c, d, words[12], 7, 0x6b901122);
        round1!(d, a, b, c, words[13], 12, 0xfd987193);
        round1!(c, d, a, b, words[14], 17, 0xa679438e);
        round1!(b, c, d, a, words[15], 22, 0x49b40821);

        // Round 2
        round2!(a, b, c, d, words[1], 5, 0xf61e2562);
        round2!(d, a, b, c, words[6], 9, 0xc040b340);
        round2!(c, d, a, b, words[11], 14, 0x265e5a51);
        round2!(b, c, d, a, words[0], 20, 0xe9b6c7aa);
        round2!(a, b, c, d, words[5], 5, 0xd62f105d);
        round2!(d, a, b, c, words[10], 9, 0x02441453);
        round2!(c, d, a, b, words[15], 14, 0xd8a1e681);
        round2!(b, c, d, a, words[4], 20, 0xe7d3fbc8);
        round2!(a, b, c, d, words[9], 5, 0x21e1cde6);
        round2!(d, a, b, c, words[14], 9, 0xc33707d6);
        round2!(c, d, a, b, words[3], 14, 0xf4d50d87);
        round2!(b, c, d, a, words[8], 20, 0x455a14ed);
        round2!(a, b, c, d, words[13], 5, 0xa9e3e905);
        round2!(d, a, b, c, words[2], 9, 0xfcefa3f8);
        round2!(c, d, a, b, words[7], 14, 0x676f02d9);
        round2!(b, c, d, a, words[12], 20, 0x8d2a4c8a);

        // Round 3
        round3!(a, b, c, d, words[5], 4, 0xfffa3942);
        round3!(d, a, b, c, words[8], 11, 0x8771f681);
        round3!(c, d, a, b, words[11], 16, 0x6d9d6122);
        round3!(b, c, d, a, words[14], 23, 0xfde5380c);
        round3!(a, b, c, d, words[1], 4, 0xa4beea44);
        round3!(d, a, b, c, words[4], 11, 0x4bdecfa9);
        round3!(c, d, a, b, words[7], 16, 0xf6bb4b60);
        round3!(b, c, d, a, words[10], 23, 0xbebfbc70);
        round3!(a, b, c, d, words[13], 4, 0x289b7ec6);
        round3!(d, a, b, c, words[0], 11, 0xeaa127fa);
        round3!(c, d, a, b, words[3], 16, 0xd4ef3085);
        round3!(b, c, d, a, words[6], 23, 0x04881d05);
        round3!(a, b, c, d, words[9], 4, 0xd9d4d039);
        round3!(d, a, b, c, words[12], 11, 0xe6db99e5);
        round3!(c, d, a, b, words[15], 16, 0x1fa27cf8);
        round3!(b, c, d, a, words[2], 23, 0xc4ac5665);

        // Round 4
        round4!(a, b, c, d, words[0], 6, 0xf4292244);
        round4!(d, a, b, c, words[7], 10, 0x432aff97);
        round4!(c, d, a, b, words[14], 15, 0xab9423a7);
        round4!(b, c, d, a, words[5], 21, 0xfc93a039);
        round4!(a, b, c, d, words[12], 6, 0x655b59c3);
        round4!(d, a, b, c, words[3], 10, 0x8f0ccc92);
        round4!(c, d, a, b, words[10], 15, 0xffeff47d);
        round4!(b, c, d, a, words[1], 21, 0x85845dd1);
        round4!(a, b, c, d, words[8], 6, 0x6fa87e4f);
        round4!(d, a, b, c, words[15], 10, 0xfe2ce6e0);
        round4!(c, d, a, b, words[6], 15, 0xa3014314);
        round4!(b, c, d, a, words[13], 21, 0x4e0811a1);
        round4!(a, b, c, d, words[4], 6, 0xf7537e82);
        round4!(d, a, b, c, words[11], 10, 0xbd3af235);
        round4!(c, d, a, b, words[2], 15, 0x2ad7d2bb);
        round4!(b, c, d, a, words[9], 21, 0xeb86d391);

        self.a = self.a.wrapping_add(a);
        self.b = self.b.wrapping_add(b);
        self.c = self.c.wrapping_add(c);
        self.d = self.d.wrapping_add(d);
    }

    fn process_bytes(&mut self, input: &[u8]) {
        let mut offset = 0;
        let mut remaining = input.len();
        let mut block = [0u8; 64];

        // Process any buffered data first
        if self.buflen > 0 {
            let needed = 64 - self.buflen;
            let take = if remaining < needed {
                remaining
            } else {
                needed
            };

            block[..self.buflen].copy_from_slice(&self.buffer[..self.buflen]);
            block[self.buflen..self.buflen + take].copy_from_slice(&input[..take]);

            if self.buflen + take == 64 {
                self.process_block(&block);
                self.total[0] = self.total[0].wrapping_add(64);
                if self.total[0] < 64 {
                    self.total[1] = self.total[1].wrapping_add(1);
                }
                offset += take;
                remaining -= take;
                self.buflen = 0;
            } else {
                self.buflen += take;
                return;
            }
        }

        // Process complete blocks
        while remaining >= 64 {
            self.process_block(&input[offset..offset + 64]);
            self.total[0] = self.total[0].wrapping_add(64);
            if self.total[0] < 64 {
                self.total[1] = self.total[1].wrapping_add(1);
            }
            offset += 64;
            remaining -= 64;
        }

        // Buffer any remaining data
        if remaining > 0 {
            self.buffer[..remaining].copy_from_slice(&input[offset..offset + remaining]);
            self.buflen = remaining;
        }
    }

    fn finish(mut self) -> [u8; 16] {
        let mut bits = [0u8; 8];
        bits[0..4].copy_from_slice(&(self.total[0] << 3).to_le_bytes());
        bits[4..8].copy_from_slice(&((self.total[1] << 3) | (self.total[0] >> 29)).to_le_bytes());

        let pad_len = if self.buflen < 56 {
            56 - self.buflen
        } else {
            120 - self.buflen
        };

        let padding = [0x80u8];
        self.process_bytes(&padding);
        self.process_bytes(&[0u8; 64][..pad_len]);
        self.process_bytes(&bits);

        let mut result = [0u8; 16];
        result[0..4].copy_from_slice(&self.a.to_le_bytes());
        result[4..8].copy_from_slice(&self.b.to_le_bytes());
        result[8..12].copy_from_slice(&self.c.to_le_bytes());
        result[12..16].copy_from_slice(&self.d.to_le_bytes());
        result
    }
}

pub fn md5(input: &[u8]) -> [u8; 16] {
    let mut ctx = Md5Context::new();
    ctx.process_bytes(input);
    ctx.finish()
}

pub fn md5_stream<R: Read>(mut reader: R) -> io::Result<[u8; 16]> {
    let mut ctx = Md5Context::new();
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

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::hex;

    #[test]
    fn test_md5() {
        assert_eq!(
            md5(b""),
            hex!("d41d8cd98f00b204e9800998ecf8427e")
        );
        assert_eq!(
            md5(b"a"),
            hex!("0cc175b9c0f1b6a831c399e269772661")
        );
        assert_eq!(
            md5(b"abc"),
            hex!("900150983cd24fb0d6963f7d28e17f72")
        );
        assert_eq!(
            md5(b"message digest"),
            hex!("f96b697d7cb7938d525a2f31aaf161d0")
        );
        assert_eq!(
            md5(b"abcdefghijklmnopqrstuvwxyz"),
            hex!("c3fcd3d76192e4007dfb496cca67e13b")
        );
        assert_eq!(
            md5(b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"),
            hex!("d174ab98d277d9f5a5611c2c9f419d9f")
        );
        assert_eq!(
            md5(b"12345678901234567890123456789012345678901234567890123456789012345678901234567890"),
            hex!("57edf4a22be3c955ac49da2e2107b67a")
        );
    }

    #[test]
    fn test_md5_stream() {
        let data = b"hello world";
        let hash = md5_stream(&mut &data[..]).unwrap();
        assert_eq!(hash, hex!("5eb63bbbe01eeed093cb22bb8f5acdc3"));
    }
}