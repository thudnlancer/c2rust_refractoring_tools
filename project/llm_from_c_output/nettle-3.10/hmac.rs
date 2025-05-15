// hmac.rs

use std::mem;
use std::ops::BitXorAssign;

const IPAD: u8 = 0x36;
const OPAD: u8 = 0x5c;

pub trait Hash {
    fn block_size(&self) -> usize;
    fn digest_size(&self) -> usize;
    fn context_size(&self) -> usize;
    fn init(&mut self);
    fn update(&mut self, data: &[u8]);
    fn digest(&mut self, output: &mut [u8]);
}

pub struct HmacContext<H> {
    outer: H,
    inner: H,
    state: H,
}

impl<H: Hash + Clone> HmacContext<H> {
    pub fn new(hash: H) -> Self {
        HmacContext {
            outer: hash.clone(),
            inner: hash.clone(),
            state: hash,
        }
    }

    pub fn set_key(&mut self, key: &[u8]) {
        let hash = &self.state;
        let block_size = hash.block_size();
        let digest_size = hash.digest_size();
        
        let mut pad = vec![0u8; block_size];
        
        self.outer.init();
        self.inner.init();

        let mut key_data = key.to_vec();
        if key_data.len() > block_size {
            let mut digest = vec![0u8; digest_size];
            self.state.init();
            self.state.update(&key_data);
            self.state.digest(&mut digest);
            key_data = digest;
        }

        assert!(key_data.len() <= block_size);
        
        pad.iter_mut().for_each(|x| *x = OPAD);
        xor_bytes(&mut pad, &key_data);
        self.outer.update(&pad);

        pad.iter_mut().for_each(|x| *x = IPAD);
        xor_bytes(&mut pad, &key_data);
        self.inner.update(&pad);

        self.state = self.inner.clone();
    }

    pub fn update(&mut self, data: &[u8]) {
        self.state.update(data);
    }

    pub fn digest(&mut self, output: &mut [u8]) {
        let hash = &self.state;
        let digest_size = hash.digest_size();
        let mut digest = vec![0u8; digest_size];
        
        self.state.digest(&mut digest);
        
        self.state = self.outer.clone();
        self.state.update(&digest);
        self.state.digest(output);
        
        self.state = self.inner.clone();
    }
}

fn xor_bytes(a: &mut [u8], b: &[u8]) {
    for (a, b) in a.iter_mut().zip(b.iter()) {
        *a ^= *b;
    }
}

// Example implementations for specific hash functions would go here
// For example:
// pub struct HmacMd5(HmacContext<Md5>);
// impl HmacMd5 {
//     pub fn new() -> Self { ... }
//     pub fn set_key(&mut self, key: &[u8]) { ... }
//     // etc.
// }