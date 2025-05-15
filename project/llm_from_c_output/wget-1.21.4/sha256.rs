use std::convert::TryInto;
use std::io::{self, Read};
use std::mem;

const SHA224_DIGEST_SIZE: usize = 224 / 8;
const SHA256_DIGEST_SIZE: usize = 256 / 8;

#[derive(Debug, Clone)]
pub struct Sha256Ctx {
    state: [u32; 8],
    total: [u32; 2],
    buflen: usize,
    buffer: [u32; 32],
}

impl Sha256Ctx {
    pub fn new() -> Self {
        Self {
            state: [0; 8],
            total: [0; 2],
            buflen: 0,
            buffer: [0; 32],
        }
    }

    pub fn sha256_init(&mut self) {
        self.state[0] = 0x6a09e667;
        self.state[1] = 0xbb67ae85;
        self.state[2] = 0x3c6ef372;
        self.state[3] = 0xa54ff53a;
        self.state[4] = 0x510e527f;
        self.state[5] = 0x9b05688c;
        self.state[6] = 0x1f83d9ab;
        self.state[7] = 0x5be0cd19;

        self.total = [0, 0];
        self.buflen = 0;
    }

    pub fn sha224_init(&mut self) {
        self.state[0] = 0xc1059ed8;
        self.state[1] = 0x367cd507;
        self.state[2] = 0x3070dd17;
        self.state[3] = 0xf70e5939;
        self.state[4] = 0xffc00b31;
        self.state[5] = 0x68581511;
        self.state[6] = 0x64f98fa7;
        self.state[7] = 0xbefa4fa4;

        self.total = [0, 0];
        self.buflen = 0;
    }

    pub fn sha256_read(&self, resbuf: &mut [u8]) {
        for (i, &word) in self.state.iter().enumerate() {
            let bytes = word.to_be_bytes();
            resbuf[i * 4..(i + 1) * 4].copy_from_slice(&bytes);
        }
    }

    pub fn sha224_read(&self, resbuf: &mut [u8]) {
        for (i, &word) in self.state[..7].iter().enumerate() {
            let bytes = word.to_be_bytes();
            resbuf[i * 4..(i + 1) * 4].copy_from_slice(&bytes);
        }
    }

    pub fn sha256_finish(&mut self, resbuf: &mut [u8]) {
        self.sha256_conclude();
        self.sha256_read(resbuf);
    }

    pub fn sha224_finish(&mut self, resbuf: &mut [u8]) {
        self.sha256_conclude();
        self.sha224_read(resbuf);
    }

    fn sha256_conclude(&mut self) {
        let bytes = self.buflen;
        let size = if bytes < 56 { 16 } else { 32 };

        self.total[0] = self.total[0].wrapping_add(bytes as u32);
        if self.total[0] < bytes as u32 {
            self.total[1] = self.total[1].wrapping_add(1);
        }

        let len_bits = (self.total[1] << 3) | (self.total[0] >> 29);
        self.buffer[size - 2] = len_bits.to_be();
        self.buffer[size - 1] = (self.total[0] << 3).to_be();

        let fillbuf = [0x80u8, 0];
        let buffer_bytes = unsafe {
            &mut *(&mut self.buffer as *mut [u32; 32] as *mut [u8; 128])
        };
        buffer_bytes[bytes..].copy_from_slice(&fillbuf[..(size * 4 - 2 * 4 - bytes)]);

        self.process_block(&self.buffer, size * 4);
    }

    pub fn sha256_process_bytes(&mut self, buffer: &[u8]) {
        let mut len = buffer.len();
        let mut offset = 0;

        if self.buflen != 0 {
            let left_over = self.buflen;
            let add = (128 - left_over).min(len);

            let buffer_bytes = unsafe {
                &mut *(&mut self.buffer as *mut [u32; 32] as *mut [u8; 128])
            };
            buffer_bytes[left_over..left_over + add].copy_from_slice(&buffer[..add]);

            self.buflen += add;
            offset += add;
            len -= add;

            if self.buflen > 64 {
                self.process_block(&self.buffer, self.buflen & !63);
                self.buflen &= 63;

                let copy_len = self.buflen;
                buffer_bytes[..copy_len].copy_from_slice(
                    &buffer_bytes[(left_over + add) & !63..(left_over + add) & !63 + copy_len],
                );
            }
        }

        if len >= 64 {
            let blocks = len & !63;
            self.process_block(&buffer[offset..offset + blocks], blocks);
            offset += blocks;
            len -= blocks;
        }

        if len > 0 {
            let buffer_bytes = unsafe {
                &mut *(&mut self.buffer as *mut [u32; 32] as *mut [u8; 128])
            };
            buffer_bytes[self.buflen..self.buflen + len].copy_from_slice(&buffer[offset..]);
            self.buflen += len;
        }
    }

    fn process_block(&mut self, block: &[u8], len: usize) {
        const K: [u32; 64] = [
            0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4,
            0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe,
            0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f,
            0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
            0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc,
            0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
            0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116,
            0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
            0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7,
            0xc67178f2,
        ];

        let mut a = self.state[0];
        let mut b = self.state[1];
        let mut c = self.state[2];
        let mut d = self.state[3];
        let mut e = self.state[4];
        let mut f = self.state[5];
        let mut g = self.state[6];
        let mut h = self.state[7];

        let words = unsafe {
            std::slice::from_raw_parts(
                block.as_ptr() as *const u32,
                len / mem::size_of::<u32>(),
            )
        };

        for chunk in words.chunks(16) {
            let mut w = [0u32; 64];
            for (i, &word) in chunk.iter().enumerate() {
                w[i] = u32::from_be(word);
            }

            for i in 16..64 {
                let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
                let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
                w[i] = w[i - 16]
                    .wrapping_add(s0)
                    .wrapping_add(w[i - 7])
                    .wrapping_add(s1);
            }

            let mut t0;
            let mut t1;

            for i in 0..64 {
                t0 = h
                    .wrapping_add(e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25))
                    .wrapping_add((e & f) ^ (!e & g))
                    .wrapping_add(K[i])
                    .wrapping_add(w[i]);
                t1 = (a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22))
                    .wrapping_add((a & b) ^ (a & c) ^ (b & c));
                h = g;
                g = f;
                f = e;
                e = d.wrapping_add(t0);
                d = c;
                c = b;
                b = a;
                a = t0.wrapping_add(t1);
            }

            a = a.wrapping_add(self.state[0]);
            b = b.wrapping_add(self.state[1]);
            c = c.wrapping_add(self.state[2]);
            d = d.wrapping_add(self.state[3]);
            e = e.wrapping_add(self.state[4]);
            f = f.wrapping_add(self.state[5]);
            g = g.wrapping_add(self.state[6]);
            h = h.wrapping_add(self.state[7]);

            self.state[0] = a;
            self.state[1] = b;
            self.state[2] = c;
            self.state[3] = d;
            self.state[4] = e;
            self.state[5] = f;
            self.state[6] = g;
            self.state[7] = h;
        }

        self.total[0] = self.total[0].wrapping_add(len as u32);
        if self.total[0] < len as u32 {
            self.total[1] = self.total[1].wrapping_add(1);
        }
    }
}

pub fn sha256_buffer(buffer: &[u8], resblock: &mut [u8]) {
    let mut ctx = Sha256Ctx::new();
    ctx.sha256_init();
    ctx.sha256_process_bytes(buffer);
    ctx.sha256_finish(resblock);
}

pub fn sha224_buffer(buffer: &[u8], resblock: &mut [u8]) {
    let mut ctx = Sha256Ctx::new();
    ctx.sha224_init();
    ctx.sha256_process_bytes(buffer);
    ctx.sha224_finish(resblock);
}

pub fn sha256_stream<R: Read>(mut stream: R, resblock: &mut [u8]) -> io::Result<()> {
    let mut ctx = Sha256Ctx::new();
    ctx.sha256_init();

    let mut buffer = [0; 4096];
    loop {
        let bytes_read = stream.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        ctx.sha256_process_bytes(&buffer[..bytes_read]);
    }

    ctx.sha256_finish(resblock);
    Ok(())
}

pub fn sha224_stream<R: Read>(mut stream: R, resblock: &mut [u8]) -> io::Result<()> {
    let mut ctx = Sha256Ctx::new();
    ctx.sha224_init();

    let mut buffer = [0; 4096];
    loop {
        let bytes_read = stream.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        ctx.sha256_process_bytes(&buffer[..bytes_read]);
    }

    ctx.sha224_finish(resblock);
    Ok(())
}