use std::mem;

#[derive(Copy, Clone)]
pub struct Aes128Ctx {
    pub keys: [u32; 44],
}

#[derive(Copy, Clone)]
pub struct Umac64Ctx {
    pub l1_key: [u32; 260],
    pub l2_key: [u32; 12],
    pub l3_key1: [u64; 16],
    pub l3_key2: [u32; 2],
    pub pdf_key: Aes128Ctx,
    pub l2_state: [u64; 6],
    pub nonce: [u8; 16],
    pub nonce_length: u16,
    pub nonce_low: u16,
    pub pad_cache: [u32; 4],
    pub index: u32,
    pub count: u64,
    pub block: [u8; 1024],
}

impl Umac64Ctx {
    pub fn set_key(&mut self, key: &[u8]) {
        unsafe {
            _nettle_umac_set_key(
                self.l1_key.as_mut_ptr(),
                self.l2_key.as_mut_ptr(),
                self.l3_key1.as_mut_ptr(),
                self.l3_key2.as_mut_ptr(),
                &mut self.pdf_key,
                key.as_ptr(),
                2,
            );
        }
        self.nonce.iter_mut().for_each(|x| *x = 0);
        self.nonce_low = 0;
        self.nonce_length = 16;
        self.index = 0;
        self.count = 0;
    }

    pub fn set_nonce(&mut self, nonce: &[u8]) {
        assert!(!nonce.is_empty(), "nonce_length > 0");
        assert!(nonce.len() <= 16, "nonce_length <= AES_BLOCK_SIZE");

        let nonce_length = nonce.len();
        self.nonce[..nonce_length].copy_from_slice(nonce);
        self.nonce[nonce_length..].fill(0);

        self.nonce_low = (self.nonce[nonce_length - 1] & 1) as u16;
        self.nonce[nonce_length - 1] &= !1;
        self.nonce_length = nonce_length as u16;
    }

    pub fn update(&mut self, data: &[u8]) {
        if data.is_empty() {
            return;
        }

        if self.index != 0 {
            let remaining = 1024 - self.index as usize;
            if data.len() < remaining {
                self.block[self.index as usize..self.index as usize + data.len()]
                    .copy_from_slice(data);
                self.index += data.len() as u32;
                return;
            } else {
                self.block[self.index as usize..].copy_from_slice(&data[..remaining]);
                let mut y = [0u64; 2];
                unsafe {
                    _nettle_umac_nh_n(
                        y.as_mut_ptr(),
                        2,
                        self.l1_key.as_ptr(),
                        1024,
                        self.block.as_ptr(),
                    );
                }
                y[0] += 8 * 1024;
                y[1] += 8 * 1024;
                let count = self.count;
                self.count += 1;
                unsafe {
                    _nettle_umac_l2(
                        self.l2_key.as_ptr(),
                        self.l2_state.as_mut_ptr(),
                        2,
                        count,
                        y.as_mut_ptr(),
                    );
                }
                self.process_blocks(&data[remaining..]);
            }
        } else {
            self.process_blocks(data);
        }
    }

    fn process_blocks(&mut self, data: &[u8]) {
        let mut chunks = data.chunks_exact(1024);
        for chunk in chunks.by_ref() {
            let mut y = [0u64; 2];
            unsafe {
                _nettle_umac_nh_n(
                    y.as_mut_ptr(),
                    2,
                    self.l1_key.as_ptr(),
                    1024,
                    chunk.as_ptr(),
                );
            }
            y[0] += 8 * 1024;
            y[1] += 8 * 1024;
            let count = self.count;
            self.count += 1;
            unsafe {
                _nettle_umac_l2(
                    self.l2_key.as_ptr(),
                    self.l2_state.as_mut_ptr(),
                    2,
                    count,
                    y.as_mut_ptr(),
                );
            }
        }

        let remainder = chunks.remainder();
        if !remainder.is_empty() {
            self.block[..remainder.len()].copy_from_slice(remainder);
            self.index = remainder.len() as u32;
        }
    }

    pub fn digest(&mut self, length: usize, digest: &mut [u8]) {
        assert!(length > 0, "length > 0");
        assert!(length <= 8, "length <= 8");

        if self.index > 0 || self.count == 0 {
            let pad = if self.index > 0 {
                (31 & (!self.index).wrapping_add(1)) as usize
            } else {
                32
            };

            self.block[self.index as usize..self.index as usize + pad].fill(0);
            let mut y = [0u64; 2];
            unsafe {
                _nettle_umac_nh_n(
                    y.as_mut_ptr(),
                    2,
                    self.l1_key.as_ptr(),
                    self.index + pad as u32,
                    self.block.as_ptr(),
                );
            }
            y[0] += 8 * self.index as u64;
            y[1] += 8 * self.index as u64;
            let count = self.count;
            self.count += 1;
            unsafe {
                _nettle_umac_l2(
                    self.l2_key.as_ptr(),
                    self.l2_state.as_mut_ptr(),
                    2,
                    count,
                    y.as_mut_ptr(),
                );
            }
        }

        assert!(self.count > 0, "ctx->count > 0");

        if self.nonce_low & 0x80 == 0 {
            unsafe {
                nettle_aes128_encrypt(
                    &self.pdf_key,
                    16,
                    self.pad_cache.as_mut_ptr() as *mut u8,
                    self.nonce.as_ptr(),
                );
            }
            self.nonce_low |= 0x80;
        }

        let pad_offset = (2 * (self.nonce_low & 1) as usize) as isize;
        let pad = unsafe { self.pad_cache.as_ptr().offset(pad_offset) };
        self.nonce_low += 1;

        if self.nonce_low & 1 == 0 {
            let mut i = (self.nonce_length - 1) as usize;
            self.nonce_low = 0;
            self.nonce[i] = self.nonce[i].wrapping_add(2);

            if self.nonce[i] == 0 && i > 0 {
                let mut increment_i = i - 1;
                self.nonce[increment_i] = self.nonce[increment_i].wrapping_add(1);
                if self.nonce[increment_i] == 0 {
                    while increment_i > 0 {
                        increment_i -= 1;
                        self.nonce[increment_i] = self.nonce[increment_i].wrapping_add(1);
                        if self.nonce[increment_i] != 0 {
                            break;
                        }
                    }
                }
            }
        }

        unsafe {
            _nettle_umac_l2_final(
                self.l2_key.as_ptr(),
                self.l2_state.as_mut_ptr(),
                2,
                self.count,
            );
        }

        let mut tag = [0u32; 2];
        tag[0] = unsafe { *pad } ^ self.l3_key2[0] ^ unsafe {
            _nettle_umac_l3(self.l3_key1.as_ptr(), self.l2_state.as_ptr())
        };
        tag[1] = unsafe { *pad.offset(1) } ^ self.l3_key2[1] ^ unsafe {
            _nettle_umac_l3(
                self.l3_key1.as_ptr().offset(8),
                self.l2_state.as_ptr().offset(2),
            )
        };

        digest[..length].copy_from_slice(unsafe {
            std::slice::from_raw_parts(tag.as_ptr() as *const u8, length)
        });

        self.index = 0;
        self.count = 0;
    }
}

extern "C" {
    fn _nettle_umac_set_key(
        l1_key: *mut u32,
        l2_key: *mut u32,
        l3_key1: *mut u64,
        l3_key2: *mut u32,
        pad: *mut Aes128Ctx,
        key: *const u8,
        n: u32,
    );
    fn _nettle_umac_nh_n(
        out: *mut u64,
        n: u32,
        key: *const u32,
        length: u32,
        msg: *const u8,
    );
    fn _nettle_umac_l2(
        key: *const u32,
        state: *mut u64,
        n: u32,
        count: u64,
        m: *const u64,
    );
    fn _nettle_umac_l2_final(key: *const u32, state: *mut u64, n: u32, count: u64);
    fn _nettle_umac_l3(key: *const u64, m: *const u64) -> u32;
    fn nettle_aes128_encrypt(
        ctx: *const Aes128Ctx,
        length: usize,
        dst: *mut u8,
        src: *const u8,
    );
}