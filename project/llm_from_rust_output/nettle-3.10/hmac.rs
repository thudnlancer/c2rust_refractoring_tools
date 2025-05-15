use std::ptr;

pub type size_t = usize;
pub type uint8_t = u8;

pub type NettleHashInitFunc = fn(context: &mut [u8]);
pub type NettleHashUpdateFunc = fn(context: &mut [u8], length: size_t, data: &[u8]);
pub type NettleHashDigestFunc = fn(context: &mut [u8], length: size_t, digest: &mut [u8]);

pub struct NettleHash {
    pub name: &'static str,
    pub context_size: usize,
    pub digest_size: usize,
    pub block_size: usize,
    pub init: NettleHashInitFunc,
    pub update: NettleHashUpdateFunc,
    pub digest: NettleHashDigestFunc,
}

pub fn hmac_set_key(
    outer: &mut [u8],
    inner: &mut [u8],
    state: &mut [u8],
    hash: &NettleHash,
    key: &[u8],
) {
    let mut pad = vec![0u8; hash.block_size];
    
    (hash.init)(outer);
    (hash.init)(inner);

    let (key, key_length) = if key.len() > hash.block_size {
        let mut digest = vec![0u8; hash.digest_size];
        (hash.init)(state);
        (hash.update)(state, key.len(), key);
        (hash.digest)(state, hash.digest_size, &mut digest);
        (digest.as_slice(), hash.digest_size)
    } else {
        (key, key.len())
    };

    assert!(key_length <= hash.block_size);

    // Outer padding
    pad.iter_mut().for_each(|b| *b = 0x5c);
    xor_bytes(&mut pad, key);
    (hash.update)(outer, hash.block_size, &pad);

    // Inner padding
    pad.iter_mut().for_each(|b| *b = 0x36);
    xor_bytes(&mut pad, key);
    (hash.update)(inner, hash.block_size, &pad);

    state.copy_from_slice(inner);
}

pub fn hmac_update(state: &mut [u8], hash: &NettleHash, data: &[u8]) {
    (hash.update)(state, data.len(), data);
}

pub fn hmac_digest(
    outer: &[u8],
    inner: &[u8],
    state: &mut [u8],
    hash: &NettleHash,
    dst: &mut [u8],
) {
    let mut digest = vec![0u8; hash.digest_size];
    
    (hash.digest)(state, hash.digest_size, &mut digest);
    state.copy_from_slice(outer);
    (hash.update)(state, hash.digest_size, &digest);
    (hash.digest)(state, dst.len(), dst);
    state.copy_from_slice(inner);
}

fn xor_bytes(dst: &mut [u8], src: &[u8]) {
    assert!(dst.len() >= src.len());
    for (d, s) in dst.iter_mut().zip(src.iter()) {
        *d ^= *s;
    }
}