use std::mem;
use sha2::{Sha256, Digest};
use aes::Aes256;
use aes::cipher::{BlockEncrypt, KeyInit};
use aes::cipher::generic_array::GenericArray;

const YARROW_MULTIPLIER: u32 = 4;
const YARROW_FAST_THRESHOLD: u32 = 100;
const YARROW_SLOW_THRESHOLD: u32 = 160;
const YARROW_SLOW_K: usize = 2;
const YARROW_RESEED_ITERATIONS: usize = 1500;
const YARROW_MAX_ENTROPY: u32 = 0x100000;
const SHA256_DIGEST_SIZE: usize = 32;
const AES_BLOCK_SIZE: usize = 16;
const AES256_KEY_SIZE: usize = 32;

#[derive(Debug, Clone, Copy)]
enum YarrowPoolId {
    Fast,
    Slow,
}

#[derive(Debug)]
struct YarrowSource {
    estimate: [u32; 2], // [Fast, Slow]
    next: YarrowPoolId,
}

#[derive(Debug)]
pub struct Yarrow256Ctx {
    pools: [Sha256; 2], // [Fast, Slow]
    seeded: bool,
    counter: [u8; AES_BLOCK_SIZE],
    key: Aes256,
    nsources: usize,
    sources: Vec<YarrowSource>,
}

impl Yarrow256Ctx {
    pub fn new(n: usize, sources: Vec<YarrowSource>) -> Self {
        Yarrow256Ctx {
            pools: [Sha256::new(), Sha256::new()],
            seeded: false,
            counter: [0; AES_BLOCK_SIZE],
            key: Aes256::new(&GenericArray::from_slice(&[0; AES256_KEY_SIZE])),
            nsources: n,
            sources,
        }
    }

    pub fn init(&mut self, n: usize, sources: Vec<YarrowSource>) {
        self.pools = [Sha256::new(), Sha256::new()];
        self.seeded = false;
        self.counter = [0; AES_BLOCK_SIZE];
        self.key = Aes256::new(&GenericArray::from_slice(&[0; AES256_KEY_SIZE]));
        self.nsources = n;
        self.sources = sources;

        for source in &mut self.sources {
            source.estimate = [0, 0];
            source.next = YarrowPoolId::Fast;
        }
    }

    pub fn seed(&mut self, seed_file: &[u8]) {
        assert!(!seed_file.is_empty());
        self.pools[YarrowPoolId::Fast as usize].update(seed_file);
        self.fast_reseed();
    }

    fn generate_block(&mut self, block: &mut [u8]) {
        let mut generic_block = GenericArray::clone_from_slice(&self.counter);
        self.key.encrypt_block(&mut generic_block);
        block.copy_from_slice(&generic_block);

        // Increment counter (big-endian)
        for i in (0..self.counter.len()).rev() {
            self.counter[i] = self.counter[i].wrapping_add(1);
            if self.counter[i] != 0 {
                break;
            }
        }
    }

    fn iterate(digest: &mut [u8]) {
        let mut v0 = [0u8; SHA256_DIGEST_SIZE];
        v0.copy_from_slice(digest);

        for i in 1..YARROW_RESEED_ITERATIONS {
            let mut hash = Sha256::new();
            let count = i.to_be_bytes();

            hash.update(digest);
            hash.update(&v0);
            hash.update(&count);

            *digest = hash.finalize_reset().into();
        }
    }

    pub fn fast_reseed(&mut self) {
        let mut digest = [0u8; SHA256_DIGEST_SIZE];

        if self.seeded {
            let mut blocks = [0u8; AES_BLOCK_SIZE * 2];
            self.generate_block(&mut blocks[..AES_BLOCK_SIZE]);
            self.generate_block(&mut blocks[AES_BLOCK_SIZE..]);
            self.pools[YarrowPoolId::Fast as usize].update(&blocks);
        }

        digest.copy_from_slice(&self.pools[YarrowPoolId::Fast as usize].finalize_reset());
        Self::iterate(&mut digest);

        self.key = Aes256::new(&GenericArray::from_slice(&digest));
        self.seeded = true;

        self.counter = [0; AES_BLOCK_SIZE];
        let mut generic_block = GenericArray::clone_from_slice(&self.counter);
        self.key.encrypt_block(&mut generic_block);
        self.counter.copy_from_slice(&generic_block);

        for source in &mut self.sources {
            source.estimate[YarrowPoolId::Fast as usize] = 0;
        }
    }

    pub fn slow_reseed(&mut self) {
        let mut digest = [0u8; SHA256_DIGEST_SIZE];
        digest.copy_from_slice(&self.pools[YarrowPoolId::Slow as usize].finalize_reset());

        self.pools[YarrowPoolId::Fast as usize].update(&digest);
        self.fast_reseed();

        for source in &mut self.sources {
            source.estimate[YarrowPoolId::Slow as usize] = 0;
        }
    }

    pub fn update(&mut self, source_index: usize, entropy: u32, data: &[u8]) -> bool {
        assert!(source_index < self.nsources);

        if data.is_empty() {
            return false;
        }

        let source = &mut self.sources[source_index];
        let current = if !self.seeded {
            YarrowPoolId::Slow
        } else {
            let current = source.next;
            source.next = match source.next {
                YarrowPoolId::Fast => YarrowPoolId::Slow,
                YarrowPoolId::Slow => YarrowPoolId::Fast,
            };
            current
        };

        self.pools[current as usize].update(data);

        if source.estimate[current as usize] < YARROW_MAX_ENTROPY {
            let mut adjusted_entropy = entropy.min(YARROW_MAX_ENTROPY);

            if data.len() < (YARROW_MAX_ENTROPY / YARROW_MULTIPLIER) as usize
                && adjusted_entropy > YARROW_MULTIPLIER * data.len() as u32
            {
                adjusted_entropy = YARROW_MULTIPLIER * data.len() as u32;
            }

            adjusted_entropy = adjusted_entropy.saturating_add(source.estimate[current as usize]);
            source.estimate[current as usize] = adjusted_entropy.min(YARROW_MAX_ENTROPY);
        }

        match current {
            YarrowPoolId::Fast => {
                if source.estimate[YarrowPoolId::Fast as usize] >= YARROW_FAST_THRESHOLD {
                    self.fast_reseed();
                    true
                } else {
                    false
                }
            }
            YarrowPoolId::Slow => {
                if self.needed_sources() == 0 {
                    self.slow_reseed();
                    true
                } else {
                    false
                }
            }
        }
    }

    fn gate(&mut self) {
        let mut key = [0u8; AES256_KEY_SIZE];
        for chunk in key.chunks_mut(AES_BLOCK_SIZE) {
            self.generate_block(chunk);
        }
        self.key = Aes256::new(&GenericArray::from_slice(&key));
    }

    pub fn random(&mut self, dst: &mut [u8]) {
        assert!(self.seeded);

        let mut remaining = dst.len();
        let mut offset = 0;

        while remaining >= AES_BLOCK_SIZE {
            self.generate_block(&mut dst[offset..offset + AES_BLOCK_SIZE]);
            offset += AES_BLOCK_SIZE;
            remaining -= AES_BLOCK_SIZE;
        }

        if remaining > 0 {
            let mut buffer = [0u8; AES_BLOCK_SIZE];
            self.generate_block(&mut buffer);
            dst[offset..].copy_from_slice(&buffer[..remaining]);
        }

        self.gate();
    }

    pub fn is_seeded(&self) -> bool {
        self.seeded
    }

    pub fn needed_sources(&self) -> usize {
        let k = self.sources.iter()
            .filter(|s| s.estimate[YarrowPoolId::Slow as usize] >= YARROW_SLOW_THRESHOLD)
            .count();

        if k < YARROW_SLOW_K {
            YARROW_SLOW_K - k
        } else {
            0
        }
    }
}