use std::convert::TryInto;
use std::mem;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Sha256Ctx {
    pub state: [u32; 8],
    pub total: [u32; 2],
    pub buflen: usize,
    pub buffer: [u32; 32],
}

const SHA256_ROUND_CONSTANTS: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

const FILLBUF: [u8; 64] = [
    0x80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

impl Sha256Ctx {
    pub fn new() -> Self {
        Sha256Ctx {
            state: [0; 8],
            total: [0; 2],
            buflen: 0,
            buffer: [0; 32],
        }
    }

    pub fn sha256_init(&mut self) {
        self.state = [
            0x6a09e667,
            0xbb67ae85,
            0x3c6ef372,
            0xa54ff53a,
            0x510e527f,
            0x9b05688c,
            0x1f83d9ab,
            0x5be0cd19,
        ];
        self.total = [0, 0];
        self.buflen = 0;
    }

    pub fn sha224_init(&mut self) {
        self.state = [
            0xc1059ed8,
            0x367cd507,
            0x3070dd17,
            0xf70e5939,
            0xffc00b31,
            0x68581511,
            0x64f98fa7,
            0xbefa4fa4,
        ];
        self.total = [0, 0];
        self.buflen = 0;
    }

    fn set_uint32(&mut self, offset: usize, value: u32) {
        let bytes = value.to_be_bytes();
        let buf = &mut self.buffer;
        for i in 0..4 {
            buf[offset + i] = bytes[i] as u32;
        }
    }

    pub fn sha256_read(&self, resbuf: &mut [u8]) {
        for i in 0..8 {
            let bytes = self.state[i].to_be_bytes();
            resbuf[i*4..(i+1)*4].copy_from_slice(&bytes);
        }
    }

    pub fn sha224_read(&self, resbuf: &mut [u8]) {
        for i in 0..7 {
            let bytes = self.state[i].to_be_bytes();
            resbuf[i*4..(i+1)*4].copy_from_slice(&bytes);
        }
    }

    fn conclude(&mut self) {
        let bytes = self.buflen;
        let size = if bytes < 56 { 16 } else { 32 };

        self.total[0] = self.total[0].wrapping_add(bytes as u32);
        if self.total[0] < bytes as u32 {
            self.total[1] = self.total[1].wrapping_add(1);
        }

        self.set_uint32(
            size - 2,
            (self.total[1] << 3 | self.total[0] >> 29).to_be(),
        );
        self.set_uint32(size - 1, (self.total[0] << 3).to_be());

        let fill_start = bytes;
        let fill_len = (size - 2) * 4 - bytes;
        self.buffer[fill_start..fill_start + fill_len].copy_from_slice(&FILLBUF[..fill_len]);

        self.process_block(&self.buffer[..size * 4]);
    }

    pub fn sha256_finish(&mut self, resbuf: &mut [u8]) {
        self.conclude();
        self.sha256_read(resbuf);
    }

    pub fn sha224_finish(&mut self, resbuf: &mut [u8]) {
        self.conclude();
        self.sha224_read(resbuf);
    }

    pub fn sha256_process_bytes(&mut self, buffer: &[u8]) {
        let mut len = buffer.len();
        let mut offset = 0;

        if self.buflen != 0 {
            let left_over = self.buflen;
            let add = if 64 - left_over > len {
                len
            } else {
                64 - left_over
            };

            self.buffer[left_over..left_over + add].copy_from_slice(&buffer[..add]);
            self.buflen += add;

            if self.buflen > 64 {
                let process_len = self.buflen & !63;
                self.process_block(&self.buffer[..process_len]);
                self.buflen &= 63;
                self.buffer[..self.buflen].copy_from_slice(
                    &self.buffer[left_over + add - process_len..left_over + add - process_len + self.buflen]
                );
            }

            offset += add;
            len -= add;
        }

        if len >= 64 {
            if buffer.as_ptr() as usize % 4 != 0 {
                while len > 64 {
                    let mut temp = [0u32; 16];
                    for i in 0..16 {
                        temp[i] = u32::from_be_bytes([
                            buffer[offset + i*4],
                            buffer[offset + i*4 + 1],
                            buffer[offset + i*4 + 2],
                            buffer[offset + i*4 + 3],
                        ]);
                    }
                    self.process_block(&temp);
                    offset += 64;
                    len -= 64;
                }
            } else {
                let process_len = len & !63;
                self.process_block(&buffer[offset..offset + process_len]);
                offset += process_len;
                len &= 63;
            }
        }

        if len > 0 {
            let left_over = self.buflen;
            self.buffer[left_over..left_over + len].copy_from_slice(&buffer[offset..offset + len]);
            self.buflen += len;
        }
    }

    fn process_block(&mut self, block: &[u8]) {
        let mut a = self.state[0];
        let mut b = self.state[1];
        let mut c = self.state[2];
        let mut d = self.state[3];
        let mut e = self.state[4];
        let mut f = self.state[5];
        let mut g = self.state[6];
        let mut h = self.state[7];

        let mut x = [0u32; 16];
        for i in 0..16 {
            x[i] = u32::from_be_bytes([
                block[i*4],
                block[i*4 + 1],
                block[i*4 + 2],
                block[i*4 + 3],
            ]);
        }

        for i in 0..64 {
            let s0 = x[(i + 1) % 16].rotate_right(7) ^ x[(i + 1) % 16].rotate_right(18) ^ (x[(i + 1) % 16] >> 3);
            let s1 = x[(i + 14) % 16].rotate_right(17) ^ x[(i + 14) % 16].rotate_right(19) ^ (x[(i + 14) % 16] >> 10);
            x[i % 16] = x[i % 16].wrapping_add(s0.wrapping_add(x[(i + 9) % 16]).wrapping_add(s1));

            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ (!e & g);
            let temp1 = h.wrapping_add(s1).wrapping_add(ch).wrapping_add(SHA256_ROUND_CONSTANTS[i]).wrapping_add(x[i % 16]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = s0.wrapping_add(maj);

            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);
        }

        self.state[0] = self.state[0].wrapping_add(a);
        self.state[1] = self.state[1].wrapping_add(b);
        self.state[2] = self.state[2].wrapping_add(c);
        self.state[3] = self.state[3].wrapping_add(d);
        self.state[4] = self.state[4].wrapping_add(e);
        self.state[5] = self.state[5].wrapping_add(f);
        self.state[6] = self.state[6].wrapping_add(g);
        self.state[7] = self.state[7].wrapping_add(h);
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