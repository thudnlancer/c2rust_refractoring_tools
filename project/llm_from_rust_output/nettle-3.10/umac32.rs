use std::mem;

#[derive(Copy, Clone)]
pub struct Aes128Ctx {
    keys: [u32; 44],
}

#[derive(Copy, Clone)]
pub struct Umac32Ctx {
    l1_key: [u32; 256],
    l2_key: [u32; 6],
    l3_key1: [u64; 8],
    l3_key2: [u32; 1],
    pdf_key: Aes128Ctx,
    l2_state: [u64; 3],
    nonce: [u8; 16],
    nonce_length: u16,
    nonce_low: u16,
    pad_cache: [u32; 4],
    index: u32,
    count: u64,
    block: [u8; 1024],
}

impl Umac32Ctx {
    pub fn set_key(&mut self, key: &[u8]) {
        unsafe {
            _nettle_umac_set_key(
                self.l1_key.as_mut_ptr(),
                self.l2_key.as_mut_ptr(),
                self.l3_key1.as_mut_ptr(),
                self.l3_key2.as_mut_ptr(),
                &mut self.pdf_key,
                key.as_ptr(),
                1,
            );
        }
        self.nonce.fill(0);
        self.nonce_low = 0;
        self.nonce_length = 16;
        self.index = 0;
        self.count = self.index as u64;
    }

    pub fn set_nonce(&mut self, nonce: &[u8]) {
        assert!(!nonce.is_empty(), "nonce_length > 0");
        assert!(nonce.len() <= 16, "nonce_length <= AES_BLOCK_SIZE");

        let nonce_length = nonce.len();
        self.nonce[..nonce_length].copy_from_slice(nonce);
        self.nonce[nonce_length..].fill(0);

        self.nonce_low = (self.nonce[nonce_length - 1] & 3) as u16;
        self.nonce[nonce_length - 1] &= !3;
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
                let (left, right) = data.split_at(remaining);
                self.block[self.index as usize..].copy_from_slice(left);
                let y = unsafe {
                    _nettle_umac_nh(
                        self.l1_key.as_ptr(),
                        1024,
                        self.block.as_ptr(),
                    )
                    .wrapping_add(8 * 1024)
                };
                let count = self.count;
                self.count += 1;
                unsafe {
                    _nettle_umac_l2(
                        self.l2_key.as_ptr(),
                        self.l2_state.as_mut_ptr(),
                        1,
                        count,
                        &y,
                    );
                }
                self.process_blocks(right);
            }
        } else {
            self.process_blocks(data);
        }
    }

    fn process_blocks(&mut self, data: &[u8]) {
        let mut chunks = data.chunks_exact(1024);
        for chunk in chunks.by_ref() {
            let y = unsafe {
                _nettle_umac_nh(self.l1_key.as_ptr(), 1024, chunk.as_ptr())
                    .wrapping_add(8 * 1024)
            };
            let count = self.count;
            self.count += 1;
            unsafe {
                _nettle_umac_l2(
                    self.l2_key.as_ptr(),
                    self.l2_state.as_mut_ptr(),
                    1,
                    count,
                    &y,
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
        assert!(length <= 4, "length <= 4");

        if self.index > 0 || self.count == 0 {
            let pad = if self.index > 0 {
                31 & (!self.index).wrapping_add(1)
            } else {
                32
            };
            self.block[self.index as usize..self.index as usize + pad as usize]
                .fill(0);
            let y = unsafe {
                _nettle_umac_nh(
                    self.l1_key.as_ptr(),
                    self.index + pad,
                    self.block.as_ptr(),
                )
                .wrapping_add(8 * self.index as u64)
            };
            let count = self.count;
            self.count += 1;
            unsafe {
                _nettle_umac_l2(
                    self.l2_key.as_ptr(),
                    self.l2_state.as_mut_ptr(),
                    1,
                    count,
                    &y,
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

        let mut pad = self.pad_cache[(self.nonce_low & 3) as usize];
        self.nonce_low += 1;

        if self.nonce_low & 3 == 0 {
            let mut i = (self.nonce_length - 1) as usize;
            self.nonce_low = 0;
            self.nonce[i] = self.nonce[i].wrapping_add(4);

            if self.nonce[i] == 0 && i > 0 {
                i -= 1;
                self.nonce[i] = self.nonce[i].wrapping_add(1);
                if self.nonce[i] == 0 {
                    while i > 0 && {
                        i -= 1;
                        self.nonce[i] = self.nonce[i].wrapping_add(1);
                        self.nonce[i] == 0
                    } {}
                }
            }
        }

        unsafe {
            _nettle_umac_l2_final(
                self.l2_key.as_ptr(),
                self.l2_state.as_mut_ptr(),
                1,
                self.count,
            );
        }

        pad ^= self.l3_key2[0]
            ^ unsafe {
                _nettle_umac_l3(self.l3_key1.as_ptr(), self.l2_state.as_ptr())
            };

        digest[..length].copy_from_slice(&pad.to_le_bytes()[..length]);
        self.index = 0;
        self.count = self.index as u64;
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
    fn _nettle_umac_nh(key: *const u32, length: u32, msg: *const u8) -> u64;
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