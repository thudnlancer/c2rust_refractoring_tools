use std::mem;
use std::ptr;
use std::slice;

pub type size_t = usize;
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type uint64_t = u64;

#[derive(Copy, Clone)]
pub struct aes256_ctx {
    pub keys: [uint32_t; 60],
}

#[derive(Copy, Clone)]
pub struct sha256_ctx {
    pub state: [uint32_t; 8],
    pub count: uint64_t,
    pub index: u32,
    pub block: [uint8_t; 64],
}

#[derive(Copy, Clone, PartialEq)]
pub enum yarrow_pool_id {
    YARROW_FAST = 0,
    YARROW_SLOW = 1,
}

#[derive(Copy, Clone)]
pub struct yarrow_source {
    pub estimate: [uint32_t; 2],
    pub next: yarrow_pool_id,
}

#[derive(Copy, Clone)]
pub struct yarrow256_ctx {
    pub pools: [sha256_ctx; 2],
    pub seeded: i32,
    pub key: aes256_ctx,
    pub counter: [uint8_t; 16],
    pub nsources: u32,
    pub sources: *mut yarrow_source,
}

impl yarrow256_ctx {
    pub fn init(&mut self, n: u32, s: *mut yarrow_source) {
        unsafe {
            nettle_sha256_init(&mut self.pools[yarrow_pool_id::YARROW_FAST as usize]);
            nettle_sha256_init(&mut self.pools[yarrow_pool_id::YARROW_SLOW as usize]);
        }
        self.seeded = 0;
        self.counter = [0; 16];
        self.nsources = n;
        self.sources = s;

        for i in 0..n {
            unsafe {
                let source = &mut *self.sources.offset(i as isize);
                source.estimate[yarrow_pool_id::YARROW_FAST as usize] = 0;
                source.estimate[yarrow_pool_id::YARROW_SLOW as usize] = 0;
                source.next = yarrow_pool_id::YARROW_FAST;
            }
        }
    }

    pub fn seed(&mut self, length: size_t, seed_file: &[uint8_t]) {
        assert!(length > 0);
        unsafe {
            nettle_sha256_update(
                &mut self.pools[yarrow_pool_id::YARROW_FAST as usize],
                length,
                seed_file.as_ptr(),
            );
        }
        self.fast_reseed();
    }

    fn generate_block(&mut self, block: &mut [uint8_t; 16]) {
        unsafe {
            nettle_aes256_encrypt(
                &mut self.key,
                mem::size_of::<[uint8_t; 16]>() as size_t,
                block.as_mut_ptr(),
                self.counter.as_ptr(),
            );
        }

        for i in (0..16).rev() {
            self.counter[i] = self.counter[i].wrapping_add(1);
            if self.counter[i] != 0 {
                break;
            }
        }
    }

    fn fast_reseed(&mut self) {
        let mut digest = [0u8; 32];
        unsafe {
            if self.seeded != 0 {
                let mut blocks = [0u8; 32];
                self.generate_block(&mut blocks[0..16].try_into().unwrap());
                self.generate_block(&mut blocks[16..32].try_into().unwrap());
                nettle_sha256_update(
                    &mut self.pools[yarrow_pool_id::YARROW_FAST as usize],
                    mem::size_of::<[uint8_t; 32]>() as size_t,
                    blocks.as_ptr(),
                );
            }

            nettle_sha256_digest(
                &mut self.pools[yarrow_pool_id::YARROW_FAST as usize],
                mem::size_of::<[uint8_t; 32]>() as size_t,
                digest.as_mut_ptr(),
            );
        }

        yarrow_iterate(&mut digest);
        unsafe {
            nettle_aes256_set_encrypt_key(&mut self.key, digest.as_ptr());
        }
        self.seeded = 1;
        self.counter = [0; 16];
        unsafe {
            nettle_aes256_encrypt(
                &mut self.key,
                mem::size_of::<[uint8_t; 16]>() as size_t,
                self.counter.as_mut_ptr(),
                self.counter.as_mut_ptr(),
            );
        }

        for i in 0..self.nsources {
            unsafe {
                (*self.sources.offset(i as isize)).estimate[yarrow_pool_id::YARROW_FAST as usize] = 0;
            }
        }
    }

    fn slow_reseed(&mut self) {
        let mut digest = [0u8; 32];
        unsafe {
            nettle_sha256_digest(
                &mut self.pools[yarrow_pool_id::YARROW_SLOW as usize],
                mem::size_of::<[uint8_t; 32]>() as size_t,
                digest.as_mut_ptr(),
            );
            nettle_sha256_update(
                &mut self.pools[yarrow_pool_id::YARROW_FAST as usize],
                mem::size_of::<[uint8_t; 32]>() as size_t,
                digest.as_ptr(),
            );
        }
        self.fast_reseed();

        for i in 0..self.nsources {
            unsafe {
                (*self.sources.offset(i as isize)).estimate[yarrow_pool_id::YARROW_SLOW as usize] = 0;
            }
        }
    }

    pub fn update(
        &mut self,
        source_index: u32,
        entropy: u32,
        length: size_t,
        data: &[uint8_t],
    ) -> i32 {
        assert!(source_index < self.nsources);
        if length == 0 {
            return 0;
        }

        let current = if self.seeded == 0 {
            yarrow_pool_id::YARROW_SLOW
        } else {
            unsafe {
                let source = &mut *self.sources.offset(source_index as isize);
                let current = source.next;
                source.next = if source.next == yarrow_pool_id::YARROW_FAST {
                    yarrow_pool_id::YARROW_SLOW
                } else {
                    yarrow_pool_id::YARROW_FAST
                };
                current
            }
        };

        unsafe {
            nettle_sha256_update(
                &mut self.pools[current as usize],
                length,
                data.as_ptr(),
            );
        }

        unsafe {
            let source = &mut *self.sources.offset(source_index as isize);
            if source.estimate[current as usize] < 0x100000 {
                let mut adjusted_entropy = entropy.min(0x100000);
                if length < (0x100000 / 4) as size_t && entropy as size_t > 4 * length {
                    adjusted_entropy = (4 * length) as u32;
                }
                adjusted_entropy = adjusted_entropy.saturating_add(source.estimate[current as usize]);
                source.estimate[current as usize] = adjusted_entropy.min(0x100000);
            }
        }

        match current {
            yarrow_pool_id::YARROW_FAST => {
                unsafe {
                    if (*self.sources.offset(source_index as isize)).estimate[yarrow_pool_id::YARROW_FAST as usize] >= 100 {
                        self.fast_reseed();
                        1
                    } else {
                        0
                    }
                }
            }
            yarrow_pool_id::YARROW_SLOW => {
                if self.needed_sources() == 0 {
                    self.slow_reseed();
                    1
                } else {
                    0
                }
            }
        }
    }

    fn gate(&mut self) {
        let mut key = [0u8; 32];
        for i in (0..32).step_by(16) {
            self.generate_block(&mut key[i..i+16].try_into().unwrap());
        }
        unsafe {
            nettle_aes256_set_encrypt_key(&mut self.key, key.as_ptr());
        }
    }

    pub fn random(&mut self, length: size_t, dst: &mut [uint8_t]) {
        assert!(self.seeded != 0);
        let mut remaining = length;
        let mut offset = 0;

        while remaining >= 16 {
            let block = &mut dst[offset..offset+16];
            self.generate_block(block.try_into().unwrap());
            offset += 16;
            remaining -= 16;
        }

        if remaining > 0 {
            let mut buffer = [0u8; 16];
            self.generate_block(&mut buffer);
            dst[offset..].copy_from_slice(&buffer[..remaining]);
        }

        self.gate();
    }

    pub fn is_seeded(&self) -> i32 {
        self.seeded
    }

    pub fn needed_sources(&self) -> u32 {
        let mut k = 0;
        for i in 0..self.nsources {
            unsafe {
                if (*self.sources.offset(i as isize)).estimate[yarrow_pool_id::YARROW_SLOW as usize] >= 160 {
                    k += 1;
                }
            }
        }
        if k < 2 { 2 - k } else { 0 }
    }
}

fn yarrow_iterate(digest: &mut [uint8_t; 32]) {
    let mut v0 = [0u8; 32];
    v0.copy_from_slice(digest);

    for i in 1..=1500 {
        let mut hash = sha256_ctx {
            state: [0; 8],
            count: 0,
            index: 0,
            block: [0; 64],
        };
        unsafe {
            nettle_sha256_init(&mut hash);
        }

        let count = [
            ((i >> 24) & 0xff) as u8,
            ((i >> 16) & 0xff) as u8,
            ((i >> 8) & 0xff) as u8,
            (i & 0xff) as u8,
        ];

        unsafe {
            nettle_sha256_update(&mut hash, 32, digest.as_ptr());
            nettle_sha256_update(&mut hash, 32, v0.as_ptr());
            nettle_sha256_update(&mut hash, 4, count.as_ptr());
            nettle_sha256_digest(&mut hash, 32, digest.as_mut_ptr());
        }
    }
}

// External C functions (would be linked from nettle)
extern "C" {
    fn nettle_aes256_set_encrypt_key(ctx: *mut aes256_ctx, key: *const uint8_t);
    fn nettle_aes256_encrypt(
        ctx: *const aes256_ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_sha256_init(ctx: *mut sha256_ctx);
    fn nettle_sha256_update(ctx: *mut sha256_ctx, length: size_t, data: *const uint8_t);
    fn nettle_sha256_digest(ctx: *mut sha256_ctx, length: size_t, digest: *mut uint8_t);
}