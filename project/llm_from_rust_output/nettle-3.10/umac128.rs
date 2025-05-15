use std::mem;

const AES_BLOCK_SIZE: usize = 16;
const UMAC_BLOCK_SIZE: usize = 1024;

#[derive(Clone, Copy)]
pub struct Aes128Ctx {
    keys: [u32; 44],
}

#[derive(Clone, Copy)]
pub struct Umac128Ctx {
    l1_key: [u32; 268],
    l2_key: [u32; 24],
    l3_key1: [u64; 32],
    l3_key2: [u32; 4],
    pdf_key: Aes128Ctx,
    l2_state: [u64; 12],
    nonce: [u8; AES_BLOCK_SIZE],
    nonce_length: u16,
    index: u32,
    count: u64,
    block: [u8; UMAC_BLOCK_SIZE],
}

impl Umac128Ctx {
    pub fn set_key(&mut self, key: &[u8]) {
        unsafe {
            _nettle_umac_set_key(
                self.l1_key.as_mut_ptr(),
                self.l2_key.as_mut_ptr(),
                self.l3_key1.as_mut_ptr(),
                self.l3_key2.as_mut_ptr(),
                &mut self.pdf_key,
                key.as_ptr(),
                4,
            );
        }
        self.nonce.fill(0);
        self.nonce_length = AES_BLOCK_SIZE as u16;
        self.index = 0;
        self.count = 0;
    }

    pub fn set_nonce(&mut self, nonce: &[u8]) {
        assert!(!nonce.is_empty(), "nonce_length > 0");
        assert!(
            nonce.len() <= AES_BLOCK_SIZE,
            "nonce_length <= AES_BLOCK_SIZE"
        );

        let nonce_length = nonce.len();
        self.nonce[..nonce_length].copy_from_slice(nonce);
        self.nonce[nonce_length..].fill(0);
        self.nonce_length = nonce_length as u16;
    }

    pub fn update(&mut self, data: &[u8]) {
        if data.is_empty() {
            return;
        }

        let mut data = data;
        let mut length = data.len();

        if self.index != 0 {
            let remaining = UMAC_BLOCK_SIZE - self.index as usize;
            if length < remaining {
                self.block[self.index as usize..self.index as usize + length]
                    .copy_from_slice(data);
                self.index += length as u32;
                return;
            }

            let (left, right) = data.split_at(remaining);
            self.block[self.index as usize..].copy_from_slice(left);
            self.process_block(&self.block);
            data = right;
            length -= remaining;
        }

        while length >= UMAC_BLOCK_SIZE {
            let (block, rest) = data.split_at(UMAC_BLOCK_SIZE);
            self.process_block(block);
            data = rest;
            length -= UMAC_BLOCK_SIZE;
        }

        if length > 0 {
            self.block[..length].copy_from_slice(data);
            self.index = length as u32;
        }
    }

    pub fn digest(&mut self, length: usize, digest: &mut [u8]) {
        assert!(length > 0, "length > 0");
        assert!(length <= AES_BLOCK_SIZE, "length <= 16");

        if self.index > 0 || self.count == 0 {
            let pad = if self.index > 0 {
                (31 & (!self.index).wrapping_add(1)) as usize
            } else {
                32
            };

            self.block[self.index as usize..self.index as usize + pad].fill(0);
            let mut y = [0u64; 4];
            unsafe {
                _nettle_umac_nh_n(
                    y.as_mut_ptr(),
                    4,
                    self.l1_key.as_ptr(),
                    (self.index as usize + pad) as u32,
                    self.block.as_ptr(),
                );
            }

            for i in 0..4 {
                y[i] = y[i].wrapping_add(8 * self.index as u64);
            }

            self.count += 1;
            unsafe {
                _nettle_umac_l2(
                    self.l2_key.as_ptr(),
                    self.l2_state.as_mut_ptr(),
                    4,
                    self.count - 1,
                    y.as_ptr(),
                );
            }
        }

        assert!(self.count > 0, "ctx->count > 0");

        let mut tag = [0u32; 4];
        unsafe {
            nettle_aes128_encrypt(
                &self.pdf_key,
                AES_BLOCK_SIZE,
                tag.as_mut_ptr() as *mut u8,
                self.nonce.as_ptr(),
            );
        }

        self.increment_nonce();

        unsafe {
            _nettle_umac_l2_final(
                self.l2_key.as_ptr(),
                self.l2_state.as_mut_ptr(),
                4,
                self.count,
            );
        }

        for i in 0..4 {
            tag[i] ^= self.l3_key2[i]
                ^ unsafe {
                    _nettle_umac_l3(
                        self.l3_key1.as_ptr().add(8 * i),
                        self.l2_state.as_ptr().add(2 * i),
                    )
                };
        }

        digest[..length].copy_from_slice(&tag[..length]);
        self.index = 0;
        self.count = 0;
    }

    fn process_block(&mut self, block: &[u8]) {
        let mut y = [0u64; 4];
        unsafe {
            _nettle_umac_nh_n(
                y.as_mut_ptr(),
                4,
                self.l1_key.as_ptr(),
                UMAC_BLOCK_SIZE as u32,
                block.as_ptr(),
            );
        }

        for i in 0..4 {
            y[i] = y[i].wrapping_add(8 * UMAC_BLOCK_SIZE as u64);
        }

        let current_count = self.count;
        self.count += 1;
        unsafe {
            _nettle_umac_l2(
                self.l2_key.as_ptr(),
                self.l2_state.as_mut_ptr(),
                4,
                current_count,
                y.as_ptr(),
            );
        }
    }

    fn increment_nonce(&mut self) {
        let mut i = (self.nonce_length - 1) as usize;
        self.nonce[i] = self.nonce[i].wrapping_add(1);

        while self.nonce[i] == 0 && i > 0 {
            i -= 1;
            self.nonce[i] = self.nonce[i].wrapping_add(1);
        }
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