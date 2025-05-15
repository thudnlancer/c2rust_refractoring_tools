use std::mem;
use std::ptr;
use std::slice;

const UMAC_BLOCK_SIZE: usize = 1024;
const AES_BLOCK_SIZE: usize = 16;

pub struct Umac96Context {
    l1_key: [u64; (UMAC_BLOCK_SIZE / 8) + 2],
    l2_key: [u32; 6],
    l3_key1: [u64; 24],
    l3_key2: [u32; 3],
    pdf_key: [u32; 4],
    nonce: [u8; AES_BLOCK_SIZE],
    nonce_length: usize,
    count: u64,
    index: usize,
    block: [u8; UMAC_BLOCK_SIZE],
    l2_state: [u64; 6],
}

impl Umac96Context {
    pub fn set_key(&mut self, key: &[u8]) {
        unsafe {
            _nettle_umac_set_key(
                &mut self.l1_key,
                &mut self.l2_key,
                &mut self.l3_key1,
                &mut self.l3_key2,
                &mut self.pdf_key,
                key,
                3,
            );
        }

        self.nonce.fill(0);
        self.nonce_length = self.nonce.len();
        self.count = 0;
        self.index = 0;
    }

    pub fn set_nonce(&mut self, nonce: &[u8]) {
        assert!(!nonce.is_empty());
        assert!(nonce.len() <= AES_BLOCK_SIZE);

        self.nonce[..nonce.len()].copy_from_slice(nonce);
        self.nonce[nonce.len()..].fill(0);
        self.nonce_length = nonce.len();
    }

    pub fn update(&mut self, data: &[u8]) {
        let mut remaining = data.len();
        let mut offset = 0;

        if self.index > 0 {
            let space = UMAC_BLOCK_SIZE - self.index;
            let take = space.min(remaining);
            self.block[self.index..self.index + take].copy_from_slice(&data[..take]);
            self.index += take;
            offset += take;
            remaining -= take;

            if self.index == UMAC_BLOCK_SIZE {
                self.process_block();
                self.index = 0;
            }
        }

        while remaining >= UMAC_BLOCK_SIZE {
            self.process_block_slice(&data[offset..offset + UMAC_BLOCK_SIZE]);
            offset += UMAC_BLOCK_SIZE;
            remaining -= UMAC_BLOCK_SIZE;
        }

        if remaining > 0 {
            self.block[..remaining].copy_from_slice(&data[offset..]);
            self.index = remaining;
        }
    }

    fn process_block(&mut self) {
        let mut y = [0u64; 3];
        unsafe {
            _nettle_umac_nh_n(&mut y, 3, &self.l1_key, UMAC_BLOCK_SIZE, &self.block);
        }
        y[0] += 8 * UMAC_BLOCK_SIZE as u64;
        y[1] += 8 * UMAC_BLOCK_SIZE as u64;
        y[2] += 8 * UMAC_BLOCK_SIZE as u64;
        unsafe {
            _nettle_umac_l2(&self.l2_key, &mut self.l2_state, 3, self.count, &y);
        }
        self.count += 1;
    }

    fn process_block_slice(&mut self, block: &[u8]) {
        let mut y = [0u64; 3];
        unsafe {
            _nettle_umac_nh_n(&mut y, 3, &self.l1_key, UMAC_BLOCK_SIZE, block);
        }
        y[0] += 8 * UMAC_BLOCK_SIZE as u64;
        y[1] += 8 * UMAC_BLOCK_SIZE as u64;
        y[2] += 8 * UMAC_BLOCK_SIZE as u64;
        unsafe {
            _nettle_umac_l2(&self.l2_key, &mut self.l2_state, 3, self.count, &y);
        }
        self.count += 1;
    }

    pub fn digest(&mut self, length: usize) -> Vec<u8> {
        assert!(length > 0 && length <= 12);

        if self.index > 0 || self.count == 0 {
            let pad = if self.index > 0 {
                (32 - (self.index % 32)) % 32
            } else {
                32
            };
            self.block[self.index..self.index + pad].fill(0);

            let mut y = [0u64; 3];
            unsafe {
                _nettle_umac_nh_n(
                    &mut y,
                    3,
                    &self.l1_key,
                    self.index + pad,
                    &self.block,
                );
            }
            y[0] += 8 * self.index as u64;
            y[1] += 8 * self.index as u64;
            y[2] += 8 * self.index as u64;
            unsafe {
                _nettle_umac_l2(&self.l2_key, &mut self.l2_state, 3, self.count, &y);
            }
            self.count += 1;
        }

        let mut tag = [0u32; 4];
        unsafe {
            aes128_encrypt(&self.pdf_key, AES_BLOCK_SIZE, &mut tag, &self.nonce);
        }

        increment_nonce(&mut self.nonce, self.nonce_length);

        unsafe {
            _nettle_umac_l2_final(&self.l2_key, &mut self.l2_state, 3, self.count);
        }

        for i in 0..3 {
            tag[i] ^= self.l3_key2[i] ^ unsafe {
                _nettle_umac_l3(
                    &self.l3_key1[8 * i..8 * i + 8],
                    &self.l2_state[2 * i..2 * i + 2],
                )
            };
        }

        self.count = 0;
        self.index = 0;

        let tag_bytes = unsafe {
            slice::from_raw_parts(tag.as_ptr() as *const u8, mem::size_of_val(&tag))
        };
        tag_bytes[..length].to_vec()
    }
}

fn increment_nonce(nonce: &mut [u8], nonce_length: usize) {
    for i in (0..nonce_length).rev() {
        nonce[i] = nonce[i].wrapping_add(1);
        if nonce[i] != 0 {
            break;
        }
    }
}

// These would be implemented as external unsafe functions in practice
extern "C" {
    fn _nettle_umac_set_key(
        l1_key: *mut u64,
        l2_key: *mut u32,
        l3_key1: *mut u64,
        l3_key2: *mut u32,
        pdf_key: *mut u32,
        key: *const u8,
        count: usize,
    );

    fn _nettle_umac_nh_n(
        y: *mut u64,
        n: usize,
        key: *const u64,
        length: usize,
        data: *const u8,
    );

    fn _nettle_umac_l2(
        key: *const u32,
        state: *mut u64,
        n: usize,
        count: u64,
        y: *const u64,
    );

    fn _nettle_umac_l2_final(key: *const u32, state: *mut u64, n: usize, count: u64);

    fn _nettle_umac_l3(key: *const u64, data: *const u64) -> u32;

    fn aes128_encrypt(key: *const u32, length: usize, dst: *mut u32, src: *const u8);
}