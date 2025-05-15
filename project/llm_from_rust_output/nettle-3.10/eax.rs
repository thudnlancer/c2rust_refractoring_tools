use std::mem;

const EAX_BLOCK_SIZE: usize = 16;

#[derive(Copy, Clone)]
#[repr(C)]
pub union NettleBlock16 {
    pub b: [u8; 16],
    pub w: [u64; 2],
    pub u64_0: [u64; 2],
}

impl Default for NettleBlock16 {
    fn default() -> Self {
        NettleBlock16 { b: [0; 16] }
    }
}

#[derive(Default, Copy, Clone)]
#[repr(C)]
pub struct EaxKey {
    pub pad_block: NettleBlock16,
    pub pad_partial: NettleBlock16,
}

#[derive(Default, Copy, Clone)]
#[repr(C)]
pub struct EaxContext {
    pub omac_nonce: NettleBlock16,
    pub omac_data: NettleBlock16,
    pub omac_message: NettleBlock16,
    pub ctr: NettleBlock16,
}

pub type NettleCipherFunc = fn(cipher: &[u8], length: usize, dst: &mut [u8], src: &[u8]);

fn block16_xor(r: &mut NettleBlock16, x: &NettleBlock16) {
    unsafe {
        r.u64_0[0] ^= x.u64_0[0];
        r.u64_0[1] ^= x.u64_0[1];
    }
}

fn block16_mulx_be(dst: &mut NettleBlock16, src: &NettleBlock16) {
    unsafe {
        let carry = (src.u64_0[0] & 0x80) >> 7;
        dst.u64_0[0] = (src.u64_0[0] & 0x7f7f7f7f7f7f7f7f) << 1
            | (src.u64_0[0] & 0x8080808080808080) >> 15
            | (src.u64_0[1] & 0x80) << 49;
        dst.u64_0[1] = ((src.u64_0[1] & 0x7f7f7f7f7f7f7f7f) << 1
            | (src.u64_0[1] & 0x8080808080808080) >> 15)
            ^ (0x87 << 56) & (0u64.wrapping_sub(carry));
    }
}

fn omac_init(state: &mut NettleBlock16, t: u32) {
    unsafe {
        state.b[..15].fill(0);
        state.b[15] = t as u8;
    }
}

fn omac_update(
    state: &mut NettleBlock16,
    key: &EaxKey,
    cipher: &[u8],
    f: NettleCipherFunc,
    mut length: usize,
    data: &[u8],
) {
    let mut offset = 0;
    while length >= EAX_BLOCK_SIZE {
        f(cipher, EAX_BLOCK_SIZE, &mut state.b, &state.b);
        for i in 0..EAX_BLOCK_SIZE {
            state.b[i] ^= data[offset + i];
        }
        length -= EAX_BLOCK_SIZE;
        offset += EAX_BLOCK_SIZE;
    }

    if length > 0 {
        f(cipher, EAX_BLOCK_SIZE, &mut state.b, &state.b);
        for i in 0..length {
            state.b[i] ^= data[offset + i];
        }
        unsafe {
            state.b[length] ^= 0x80;
        }
        block16_xor(state, &key.pad_partial);
    }
}

fn omac_final(state: &mut NettleBlock16, key: &EaxKey, cipher: &[u8], f: NettleCipherFunc) {
    block16_xor(state, &key.pad_block);
    f(cipher, EAX_BLOCK_SIZE, &mut state.b, &state.b);
}

pub fn nettle_eax_set_key(key: &mut EaxKey, cipher: &[u8], f: NettleCipherFunc) {
    let zero_block = NettleBlock16::default();
    f(cipher, EAX_BLOCK_SIZE, &mut key.pad_block.b, &zero_block.b);
    block16_mulx_be(&mut key.pad_block, &key.pad_block);
    block16_mulx_be(&mut key.pad_partial, &key.pad_block);
    block16_xor(&mut key.pad_partial, &key.pad_block);
}

pub fn nettle_eax_set_nonce(
    eax: &mut EaxContext,
    key: &EaxKey,
    cipher: &[u8],
    f: NettleCipherFunc,
    nonce_length: usize,
    nonce: &[u8],
) {
    omac_init(&mut eax.omac_nonce, 0);
    omac_update(&mut eax.omac_nonce, key, cipher, f, nonce_length, nonce);
    omac_final(&mut eax.omac_nonce, key, cipher, f);
    eax.ctr.b.copy_from_slice(&eax.omac_nonce.b);
    omac_init(&mut eax.omac_data, 1);
    omac_init(&mut eax.omac_message, 2);
}

pub fn nettle_eax_update(
    eax: &mut EaxContext,
    key: &EaxKey,
    cipher: &[u8],
    f: NettleCipherFunc,
    data_length: usize,
    data: &[u8],
) {
    omac_update(&mut eax.omac_data, key, cipher, f, data_length, data);
}

pub fn nettle_eax_encrypt(
    eax: &mut EaxContext,
    key: &EaxKey,
    cipher: &[u8],
    f: NettleCipherFunc,
    length: usize,
    dst: &mut [u8],
    src: &[u8],
) {
    // TODO: Implement CTR mode encryption
    omac_update(&mut eax.omac_message, key, cipher, f, length, dst);
}

pub fn nettle_eax_decrypt(
    eax: &mut EaxContext,
    key: &EaxKey,
    cipher: &[u8],
    f: NettleCipherFunc,
    length: usize,
    dst: &mut [u8],
    src: &[u8],
) {
    omac_update(&mut eax.omac_message, key, cipher, f, length, src);
    // TODO: Implement CTR mode decryption
}

pub fn nettle_eax_digest(
    eax: &mut EaxContext,
    key: &EaxKey,
    cipher: &[u8],
    f: NettleCipherFunc,
    length: usize,
    digest: &mut [u8],
) {
    assert!(length > 0 && length <= EAX_BLOCK_SIZE);

    omac_final(&mut eax.omac_data, key, cipher, f);
    omac_final(&mut eax.omac_message, key, cipher, f);
    block16_xor(&mut eax.omac_nonce, &mut eax.omac_data);

    for i in 0..length {
        digest[i] = eax.omac_nonce.b[i] ^ eax.omac_message.b[i];
    }
}