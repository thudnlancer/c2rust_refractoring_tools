use std::mem;

#[derive(Copy, Clone, Default)]
pub struct Md5Context {
    pub A: u32,
    pub B: u32,
    pub C: u32,
    pub D: u32,
    pub total: [u32; 2],
    pub buflen: u32,
    pub buffer: [u32; 32],
}

const FILLBUF: [u8; 64] = [
    0x80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

impl Md5Context {
    pub fn new() -> Self {
        let mut ctx = Md5Context::default();
        ctx.init();
        ctx
    }

    pub fn init(&mut self) {
        self.A = 0x67452301;
        self.B = 0xefcdab89;
        self.C = 0x98badcfe;
        self.D = 0x10325476;
        self.total = [0, 0];
        self.buflen = 0;
    }

    pub fn read_ctx(&self, resbuf: &mut [u8; 16]) {
        resbuf[0..4].copy_from_slice(&self.A.to_le_bytes());
        resbuf[4..8].copy_from_slice(&self.B.to_le_bytes());
        resbuf[8..12].copy_from_slice(&self.C.to_le_bytes());
        resbuf[12..16].copy_from_slice(&self.D.to_le_bytes());
    }

    pub fn finish(&mut self, resbuf: &mut [u8; 16]) {
        let bytes = self.buflen;
        let size = if bytes < 56 { 16 } else { 32 };

        self.total[0] = self.total[0].wrapping_add(bytes);
        if self.total[0] < bytes {
            self.total[1] = self.total[1].wrapping_add(1);
        }

        self.buffer[size - 2] = self.total[0] << 3;
        self.buffer[size - 1] = (self.total[1] << 3) | (self.total[0] >> 29);

        let fill_len = (size - 2) * 4 - bytes as usize;
        let buffer_ptr = self.buffer.as_mut_ptr() as *mut u8;
        unsafe {
            let buffer_bytes = std::slice::from_raw_parts_mut(
                buffer_ptr.offset(bytes as isize),
                fill_len,
            );
            buffer_bytes.copy_from_slice(&FILLBUF[..fill_len]);
        }

        self.process_block(size * 4);
        self.read_ctx(resbuf);
    }

    pub fn process_bytes(&mut self, buffer: &[u8]) {
        let mut buffer = buffer;
        let mut len = buffer.len();

        if self.buflen != 0 {
            let left_over = self.buflen as usize;
            let add = std::cmp::min(128 - left_over, len);

            let buffer_ptr = self.buffer.as_mut_ptr() as *mut u8;
            unsafe {
                let dst = std::slice::from_raw_parts_mut(
                    buffer_ptr.offset(left_over as isize),
                    add,
                );
                dst.copy_from_slice(&buffer[..add]);
            }

            self.buflen += add as u32;
            if self.buflen > 64 {
                let process_len = (self.buflen & !63) as usize;
                self.process_block(process_len);
                self.buflen &= 63;

                unsafe {
                    let src = buffer_ptr.add(left_over + add - process_len);
                    std::ptr::copy_nonoverlapping(
                        src,
                        buffer_ptr,
                        self.buflen as usize,
                    );
                }
            }

            buffer = &buffer[add..];
            len -= add;
        }

        if len >= 64 {
            if (buffer.as_ptr() as usize) % 4 != 0 {
                while len > 64 {
                    let buffer_ptr = self.buffer.as_mut_ptr() as *mut u8;
                    unsafe {
                        std::ptr::copy_nonoverlapping(
                            buffer.as_ptr(),
                            buffer_ptr,
                            64,
                        );
                    }
                    self.process_block(64);
                    buffer = &buffer[64..];
                    len -= 64;
                }
            } else {
                let process_len = len & !63;
                self.process_block(process_len);
                buffer = &buffer[process_len..];
                len &= 63;
            }
        }

        if len > 0 {
            let left_over = self.buflen as usize;
            let buffer_ptr = self.buffer.as_mut_ptr() as *mut u8;
            unsafe {
                let dst = std::slice::from_raw_parts_mut(
                    buffer_ptr.offset(left_over as isize),
                    len,
                );
                dst.copy_from_slice(&buffer[..len]);
            }
            self.buflen += len as u32;
        }
    }

    fn process_block(&mut self, len: usize) {
        let mut words = unsafe {
            std::slice::from_raw_parts(
                self.buffer.as_ptr() as *const u32,
                len / 4,
            )
        };

        let mut A = self.A;
        let mut B = self.B;
        let mut C = self.C;
        let mut D = self.D;
        let lolen = len as u32;

        self.total[0] = self.total[0].wrapping_add(lolen);
        self.total[1] = self.total[1].wrapping_add(
            (len >> 31 >> 1) as u32 + ((self.total[0] < lolen) as u32,
        );

        for chunk in words.chunks(16) {
            let mut correct_words = [0u32; 16];
            correct_words[..chunk.len()].copy_from_slice(chunk);

            let A_save = A;
            let B_save = B;
            let C_save = C;
            let D_save = D;

            // Round 1
            macro_rules! FF {
                ($a:expr, $b:expr, $c:expr, $d:expr, $x:expr, $s:expr, $ac:expr) => {
                    $a = $a.wrapping_add(($d ^ ($b & ($c ^ $d)))
                        .wrapping_add($x)
                        .wrapping_add($ac);
                    $a = $a.rotate_left($s).wrapping_add($b);
                };
            }

            FF!(A, B, C, D, correct_words[0], 7, 0xd76aa478);
            FF!(D, A, B, C, correct_words[1], 12, 0xe8c7b756);
            FF!(C, D, A, B, correct_words[2], 17, 0x242070db);
            FF!(B, C, D, A, correct_words[3], 22, 0xc1bdceee);
            FF!(A, B, C, D, correct_words[4], 7, 0xf57c0faf);
            FF!(D, A, B, C, correct_words[5], 12, 0x4787c62a);
            FF!(C, D, A, B, correct_words[6], 17, 0xa8304613);
            FF!(B, C, D, A, correct_words[7], 22, 0xfd469501);
            FF!(A, B, C, D, correct_words[8], 7, 0x698098d8);
            FF!(D, A, B, C, correct_words[9], 12, 0x8b44f7af);
            FF!(C, D, A, B, correct_words[10], 17, 0xffff5bb1);
            FF!(B, C, D, A, correct_words[11], 22, 0x895cd7be);
            FF!(A, B, C, D, correct_words[12], 7, 0x6b901122);
            FF!(D, A, B, C, correct_words[13], 12, 0xfd987193);
            FF!(C, D, A, B, correct_words[14], 17, 0xa679438e);
            FF!(B, C, D, A, correct_words[15], 22, 0x49b40821);

            // Round 2
            macro_rules! GG {
                ($a:expr, $b:expr, $c:expr, $d:expr, $x:expr, $s:expr, $ac:expr) => {
                    $a = $a.wrapping_add(($c ^ ($d & ($b ^ $c)))
                        .wrapping_add($x)
                        .wrapping_add($ac));
                    $a = $a.rotate_left($s).wrapping_add($b);
                };
            }

            GG!(A, B, C, D, correct_words[1], 5, 0xf61e2562);
            GG!(D, A, B, C, correct_words[6], 9, 0xc040b340);
            GG!(C, D, A, B, correct_words[11], 14, 0x265e5a51);
            GG!(B, C, D, A, correct_words[0], 20, 0xe9b6c7aa);
            GG!(A, B, C, D, correct_words[5], 5, 0xd62f105d);
            GG!(D, A, B, C, correct_words[10], 9, 0x02441453);
            GG!(C, D, A, B, correct_words[15], 14, 0xd8a1e681);
            GG!(B, C, D, A, correct_words[4], 20, 0xe7d3fbc8);
            GG!(A, B, C, D, correct_words[9], 5, 0x21e1cde6);
            GG!(D, A, B, C, correct_words[14], 9, 0xc33707d6);
            GG!(C, D, A, B, correct_words[3], 14, 0xf4d50d87);
            GG!(B, C, D, A, correct_words[8], 20, 0x455a14ed);
            GG!(A, B, C, D, correct_words[13], 5, 0xa9e3e905);
            GG!(D, A, B, C, correct_words[2], 9, 0xfcefa3f8);
            GG!(C, D, A, B, correct_words[7], 14, 0x676f02d9);
            GG!(B, C, D, A, correct_words[12], 20, 0x8d2a4c8a);

            // Round 3
            macro_rules! HH {
                ($a:expr, $b:expr, $c:expr, $d:expr, $x:expr, $s:expr, $ac:expr) => {
                    $a = $a.wrapping_add(($b ^ $c ^ $d)
                        .wrapping_add($x)
                        .wrapping_add($ac));
                    $a = $a.rotate_left($s).wrapping_add($b);
                };
            }

            HH!(A, B, C, D, correct_words[5], 4, 0xfffa3942);
            HH!(D, A, B, C, correct_words[8], 11, 0x8771f681);
            HH!(C, D, A, B, correct_words[11], 16, 0x6d9d6122);
            HH!(B, C, D, A, correct_words[14], 23, 0xfde5380c);
            HH!(A, B, C, D, correct_words[1], 4, 0xa4beea44);
            HH!(D, A, B, C, correct_words[4], 11, 0x4bdecfa9);
            HH!(C, D, A, B, correct_words[7], 16, 0xf6bb4b60);
            HH!(B, C, D, A, correct_words[10], 23, 0xbebfbc70);
            HH!(A, B, C, D, correct_words[13], 4, 0x289b7ec6);
            HH!(D, A, B, C, correct_words[0], 11, 0xeaa127fa);
            HH!(C, D, A, B, correct_words[3], 16, 0xd4ef3085);
            HH!(B, C, D, A, correct_words[6], 23, 0x04881d05);
            HH!(A, B, C, D, correct_words[9], 4, 0xd9d4d039);
            HH!(D, A, B, C, correct_words[12], 11, 0xe6db99e5);
            HH!(C, D, A, B, correct_words[15], 16, 0x1fa27cf8);
            HH!(B, C, D, A, correct_words[2], 23, 0xc4ac5665);

            // Round 4
            macro_rules! II {
                ($a:expr, $b:expr, $c:expr, $d:expr, $x:expr, $s:expr, $ac:expr) => {
                    $a = $a.wrapping_add(($c ^ ($b | !$d))
                        .wrapping_add($x)
                        .wrapping_add($ac));
                    $a = $a.rotate_left($s).wrapping_add($b);
                };
            }

            II!(A, B, C, D, correct_words[0], 6, 0xf4292244);
            II!(D, A, B, C, correct_words[7], 10, 0x432aff97);
            II!(C, D, A, B, correct_words[14], 15, 0xab9423a7);
            II!(B, C, D, A, correct_words[5], 21, 0xfc93a039);
            II!(A, B, C, D, correct_words[12], 6, 0x655b59c3);
            II!(D, A, B, C, correct_words[3], 10, 0x8f0ccc92);
            II!(C, D, A, B, correct_words[10], 15, 0xffeff47d);
            II!(B, C, D, A, correct_words[1], 21, 0x85845dd1);
            II!(A, B, C, D, correct_words[8], 6, 0x6fa87e4f);
            II!(D, A, B, C, correct_words[15], 10, 0xfe2ce6e0);
            II!(C, D, A, B, correct_words[6], 15, 0xa3014314);
            II!(B, C, D, A, correct_words[13], 21, 0x4e0811a1);
            II!(A, B, C, D, correct_words[4], 6, 0xf7537e82);
            II!(D, A, B, C, correct_words[11], 10, 0xbd3af235);
            II!(C, D, A, B, correct_words[2], 15, 0x2ad7d2bb);
            II!(B, C, D, A, correct_words[9], 21, 0xeb86d391);

            A = A.wrapping_add(A_save);
            B = B.wrapping_add(B_save);
            C = C.wrapping_add(C_save);
            D = D.wrapping_add(D_save);
        }

        self.A = A;
        self.B = B;
        self.C = C;
        self.D = D;
    }
}

pub fn md5_buffer(buffer: &[u8], resblock: &mut [u8; 16]) {
    let mut ctx = Md5Context::new();
    ctx.process_bytes(buffer);
    ctx.finish(resblock);
}