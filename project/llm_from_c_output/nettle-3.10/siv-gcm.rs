use std::cmp::min;
use std::mem;
use std::ptr;

const SIV_GCM_BLOCK_SIZE: usize = 16;
const SIV_GCM_DIGEST_SIZE: usize = 16;
const SIV_GCM_NONCE_SIZE: usize = 12;

#[derive(Clone, Copy)]
union Block16 {
    b: [u8; 16],
    u64: [u64; 2],
}

impl Block16 {
    fn zero() -> Self {
        Block16 { u64: [0, 0] }
    }

    fn bswap(&mut self) {
        let bytes = unsafe { &mut self.b };
        bytes.reverse();
    }
}

struct GcmKey {
    key: [u64; 2],
}

impl GcmKey {
    fn set_key(&mut self, key: &Block16) {
        self.key = unsafe { key.u64 };
    }
}

fn siv_gcm_derive_keys(
    ctx: &dyn AesCipher,
    f: fn(&dyn AesCipher, &[u8], &mut [u8]),
    key_size: usize,
    nlength: usize,
    nonce: &[u8],
    auth_key: &mut Block16,
    encryption_key: &mut [u8],
) {
    let mut block = Block16::zero();
    let mut out = Block16::zero();

    unsafe {
        ptr::copy_nonoverlapping(
            nonce.as_ptr(),
            block.b[4..].as_mut_ptr(),
            min(nlength, SIV_GCM_NONCE_SIZE),
        );
    }

    f(ctx, &block.b, &mut out.b);
    auth_key.u64[0] = out.u64[0];

    block.b[0] = 1;
    f(ctx, &block.b, &mut out.b);
    auth_key.u64[1] = out.u64[0];

    assert!(key_size % 8 == 0 && key_size / 8 + 2 <= u8::MAX as usize);

    for i in (0..key_size).step_by(8) {
        block.b[0] += 1;
        f(ctx, &block.b, &mut out.b);
        unsafe {
            ptr::copy_nonoverlapping(out.b.as_ptr(), encryption_key.as_mut_ptr().add(i), 8);
        }
    }
}

fn siv_gcm_fill(ctr: &mut [u8], blocks: usize, buffer: &mut [Block16]) {
    let mut c = u32::from_le_bytes([ctr[0], ctr[1], ctr[2], ctr[3]]);

    for i in 0..blocks {
        unsafe {
            ptr::copy_nonoverlapping(
                ctr.as_ptr().add(4),
                buffer[i].b.as_mut_ptr().add(4),
                SIV_GCM_BLOCK_SIZE - 4,
            );
            ptr::write(buffer[i].b.as_mut_ptr() as *mut u32, c.to_le());
        }
        c += 1;
    }

    ctr[..4].copy_from_slice(&c.to_le_bytes());
}

fn siv_ghash_pad_update(
    ctx: &GcmKey,
    state: &mut Block16,
    length: usize,
    data: &[u8],
) {
    let blocks = length / SIV_GCM_BLOCK_SIZE;
    let mut offset = 0;

    if blocks > 0 {
        offset = blocks * SIV_GCM_BLOCK_SIZE;
        _siv_ghash_update(ctx, state, blocks, data);
    }

    let remaining = length % SIV_GCM_BLOCK_SIZE;
    if remaining > 0 {
        let mut block = [0u8; SIV_GCM_BLOCK_SIZE];
        block[..remaining].copy_from_slice(&data[offset..offset + remaining]);
        _siv_ghash_update(ctx, state, 1, &block);
    }
}

fn siv_gcm_authenticate(
    ctx: &dyn AesCipher,
    nc: &dyn AesCipher,
    authentication_key: &Block16,
    nonce: &[u8],
    alength: usize,
    adata: &[u8],
    mlength: usize,
    mdata: &[u8],
    tag: &mut [u8],
) {
    let mut state = Block16::zero();
    let mut siv_ghash_key = GcmKey { key: [0, 0] };
    let mut block = Block16::zero();

    siv_ghash_key.set_key(authentication_key);

    siv_ghash_pad_update(&siv_ghash_key, &mut state, alength, adata);
    siv_ghash_pad_update(&siv_ghash_key, &mut state, mlength, mdata);

    block.u64[0] = (alength as u64 * 8).to_be();
    block.u64[1] = (mlength as u64 * 8).to_be();

    _siv_ghash_update(&siv_ghash_key, &mut state, 1, &block.b);
    state.bswap();

    for i in 0..SIV_GCM_NONCE_SIZE {
        state.b[i] ^= nonce[i];
    }
    state.b[15] &= 0x7f;

    nc.encrypt(ctx, &state.b, tag);
}

pub fn siv_gcm_encrypt_message(
    nc: &dyn AesCipher,
    ctx: &dyn AesCipher,
    ctr_ctx: &mut dyn AesCipher,
    nlength: usize,
    nonce: &[u8],
    alength: usize,
    adata: &[u8],
    clength: usize,
    dst: &mut [u8],
    src: &[u8],
) {
    let mut authentication_key = Block16::zero();
    let mut encryption_key = vec![0u8; nc.key_size()];
    let mut ctr = [0u8; SIV_GCM_DIGEST_SIZE];
    let tag = &mut dst[clength - SIV_GCM_BLOCK_SIZE..];

    assert!(clength >= SIV_GCM_DIGEST_SIZE);
    assert!(nlength == SIV_GCM_NONCE_SIZE);

    siv_gcm_derive_keys(
        ctx,
        AesCipher::encrypt,
        nc.key_size(),
        nlength,
        nonce,
        &mut authentication_key,
        &mut encryption_key,
    );

    nc.set_encrypt_key(ctr_ctx, &encryption_key);

    siv_gcm_authenticate(
        ctr_ctx,
        nc,
        &authentication_key,
        nonce,
        alength,
        adata,
        clength - SIV_GCM_BLOCK_SIZE,
        src,
        tag,
    );

    ctr.copy_from_slice(tag);
    ctr[15] |= 0x80;

    _nettle_ctr_crypt16(
        ctr_ctx,
        AesCipher::encrypt,
        siv_gcm_fill,
        &mut ctr,
        clength - SIV_GCM_BLOCK_SIZE,
        dst,
        src,
    );
}

pub fn siv_gcm_decrypt_message(
    nc: &dyn AesCipher,
    ctx: &dyn AesCipher,
    ctr_ctx: &mut dyn AesCipher,
    nlength: usize,
    nonce: &[u8],
    alength: usize,
    adata: &[u8],
    mlength: usize,
    dst: &mut [u8],
    src: &[u8],
) -> bool {
    let mut authentication_key = Block16::zero();
    let mut encryption_key = vec![0u8; nc.key_size()];
    let mut state = Block16::zero();
    let mut tag = [0u8; SIV_GCM_DIGEST_SIZE];

    assert!(nlength == SIV_GCM_NONCE_SIZE);

    siv_gcm_derive_keys(
        ctx,
        AesCipher::encrypt,
        nc.key_size(),
        nlength,
        nonce,
        &mut authentication_key,
        &mut encryption_key,
    );

    state.b.copy_from_slice(&src[mlength..mlength + SIV_GCM_DIGEST_SIZE]);
    state.b[15] |= 0x80;

    nc.set_encrypt_key(ctr_ctx, &encryption_key);

    _nettle_ctr_crypt16(
        ctr_ctx,
        AesCipher::encrypt,
        siv_gcm_fill,
        &mut state.b,
        mlength,
        dst,
        src,
    );

    siv_gcm_authenticate(
        ctr_ctx,
        nc,
        &authentication_key,
        nonce,
        alength,
        adata,
        mlength,
        dst,
        &mut tag,
    );

    tag == src[mlength..mlength + SIV_GCM_DIGEST_SIZE]
}

trait AesCipher {
    fn key_size(&self) -> usize;
    fn encrypt(&self, src: &[u8], dst: &mut [u8]);
    fn set_encrypt_key(&mut self, key: &[u8]);
}

fn _siv_ghash_update(_ctx: &GcmKey, _state: &mut Block16, _blocks: usize, _data: &[u8]) {}
fn _nettle_ctr_crypt16(
    _ctx: &dyn AesCipher,
    _f: fn(&dyn AesCipher, &[u8], &mut [u8]),
    _fill: fn(&mut [u8], usize, &mut [Block16]),
    _ctr: &mut [u8],
    _length: usize,
    _dst: &mut [u8],
    _src: &[u8],
) {
}