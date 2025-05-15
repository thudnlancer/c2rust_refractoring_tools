use std::mem;
use std::ptr;
use std::slice;

const GCM_BLOCK_SIZE: usize = 16;

type size_t = usize;
type uint8_t = u8;
type uint32_t = u32;
type uint64_t = u64;

#[derive(Copy, Clone)]
#[repr(C)]
pub union NettleBlock16 {
    b: [uint8_t; 16],
    w: [u64; 2],
    u64_0: [uint64_t; 2],
}

impl Default for NettleBlock16 {
    fn default() -> Self {
        NettleBlock16 { b: [0; 16] }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GcmKey {
    h: [NettleBlock16; 256],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GcmCtx {
    iv: NettleBlock16,
    ctr: NettleBlock16,
    x: NettleBlock16,
    auth_size: uint64_t,
    data_size: uint64_t,
}

pub type NettleCipherFunc = fn(key: &[u8], length: usize, dst: &mut [u8], src: &[u8]);
pub type NettleFill16Func = fn(ctr: &mut [u8], blocks: usize, buffer: &mut [NettleBlock16]);

fn block16_zero(r: &mut NettleBlock16) {
    *r = NettleBlock16::default();
}

fn block16_xor(r: &mut NettleBlock16, x: &NettleBlock16) {
    unsafe {
        r.u64_0[0] ^= x.u64_0[0];
        r.u64_0[1] ^= x.u64_0[1];
    }
}

pub fn nettle_gcm_set_key(key: &mut GcmKey, cipher: &[u8], f: NettleCipherFunc) {
    let zero_block = NettleBlock16::default();
    let mut key_block = NettleBlock16::default();
    
    let mut dst = unsafe { slice::from_raw_parts_mut(key_block.b.as_mut_ptr(), 16) };
    f(cipher, 16, &mut dst, &zero_block.b);
    
    unsafe {
        _nettle_ghash_set_key(key, &key_block);
    }
}

fn gcm_hash(key: &GcmKey, x: &mut NettleBlock16, length: usize, data: &[u8]) {
    let (blocks, remainder) = (length / GCM_BLOCK_SIZE, length % GCM_BLOCK_SIZE);
    let mut data_ptr = data.as_ptr();
    
    unsafe {
        data_ptr = _nettle_ghash_update(key, x, blocks, data_ptr);
    }
    
    if remainder > 0 {
        let mut block = NettleBlock16::default();
        unsafe {
            ptr::copy_nonoverlapping(data_ptr, block.b.as_mut_ptr(), remainder);
            _nettle_ghash_update(key, x, 1, block.b.as_ptr());
        }
    }
}

fn gcm_hash_sizes(key: &GcmKey, x: &mut NettleBlock16, auth_size: uint64_t, data_size: uint64_t) {
    let mut buffer = NettleBlock16::default();
    let data_size = data_size.wrapping_mul(8);
    let auth_size = auth_size.wrapping_mul(8);
    
    unsafe {
        buffer.u64_0[0] = auth_size.swap_bytes();
        buffer.u64_0[1] = data_size.swap_bytes();
        _nettle_ghash_update(key, x, 1, buffer.b.as_ptr());
    }
}

pub fn nettle_gcm_set_iv(ctx: &mut GcmCtx, key: &GcmKey, length: usize, iv: &[u8]) {
    if length == 12 {
        unsafe {
            ptr::copy_nonoverlapping(iv.as_ptr(), ctx.iv.b.as_mut_ptr(), 12);
            ctx.iv.b[12..16].copy_from_slice(&[0, 0, 0, 1]);
        }
    } else {
        block16_zero(&mut ctx.iv);
        gcm_hash(key, &mut ctx.iv, length, iv);
        gcm_hash_sizes(key, &mut ctx.iv, 0, length as u64);
    }
    
    ctx.ctr = ctx.iv;
    
    // Increment counter
    let mut increment_i = 3;
    unsafe {
        let ctr_ptr = ctx.ctr.b.as_mut_ptr().add(12);
        *ctr_ptr.add(increment_i) = ctr_ptr.add(increment_i).read().wrapping_add(1);
        
        while *ctr_ptr.add(increment_i) == 0 && increment_i > 0 {
            increment_i -= 1;
            *ctr_ptr.add(increment_i) = ctr_ptr.add(increment_i).read().wrapping_add(1);
        }
    }
    
    block16_zero(&mut ctx.x);
    ctx.data_size = 0;
    ctx.auth_size = ctx.data_size;
}

pub fn nettle_gcm_update(ctx: &mut GcmCtx, key: &GcmKey, length: usize, data: &[u8]) {
    assert!(ctx.auth_size % GCM_BLOCK_SIZE == 0);
    assert!(ctx.data_size == 0);
    
    gcm_hash(key, &mut ctx.x, length, data);
    ctx.auth_size += length as u64;
}

fn gcm_fill(ctr: &mut [u8], blocks: usize, buffer: &mut [NettleBlock16]) {
    let hi = u64::from_be_bytes([ctr[0], ctr[1], ctr[2], ctr[3], ctr[4], ctr[5], ctr[6], ctr[7]]);
    let mid = u32::from_be_bytes([ctr[8], ctr[9], ctr[10], ctr[11]]) as u64;
    let mut lo = u32::from_be_bytes([ctr[12], ctr[13], ctr[14], ctr[15]]);
    
    for i in 0..blocks {
        unsafe {
            buffer[i].u64_0[0] = hi;
            buffer[i].u64_0[1] = mid | (lo.swap_bytes() as u64) << 32;
        }
        lo = lo.wrapping_add(1);
    }
    
    let lo_be = lo.to_be_bytes();
    ctr[12..16].copy_from_slice(&lo_be);
}

pub fn nettle_gcm_encrypt(
    ctx: &mut GcmCtx,
    key: &GcmKey,
    cipher: &[u8],
    f: NettleCipherFunc,
    length: usize,
    dst: &mut [u8],
    src: &[u8],
) {
    assert!(ctx.data_size % GCM_BLOCK_SIZE == 0);
    
    unsafe {
        _nettle_ctr_crypt16(
            cipher.as_ptr(),
            mem::transmute(f),
            Some(mem::transmute(gcm_fill as NettleFill16Func)),
            ctx.ctr.b.as_mut_ptr(),
            length,
            dst.as_mut_ptr(),
            src.as_ptr(),
        );
    }
    
    gcm_hash(key, &mut ctx.x, length, dst);
    ctx.data_size += length as u64;
}

pub fn nettle_gcm_decrypt(
    ctx: &mut GcmCtx,
    key: &GcmKey,
    cipher: &[u8],
    f: NettleCipherFunc,
    length: usize,
    dst: &mut [u8],
    src: &[u8],
) {
    assert!(ctx.data_size % GCM_BLOCK_SIZE == 0);
    
    gcm_hash(key, &mut ctx.x, length, src);
    
    unsafe {
        _nettle_ctr_crypt16(
            cipher.as_ptr(),
            mem::transmute(f),
            Some(mem::transmute(gcm_fill as NettleFill16Func)),
            ctx.ctr.b.as_mut_ptr(),
            length,
            dst.as_mut_ptr(),
            src.as_ptr(),
        );
    }
    
    ctx.data_size += length as u64;
}

pub fn nettle_gcm_digest(
    ctx: &mut GcmCtx,
    key: &GcmKey,
    cipher: &[u8],
    f: NettleCipherFunc,
    length: usize,
    digest: &mut [u8],
) {
    assert!(length <= GCM_BLOCK_SIZE);
    
    let mut buffer = NettleBlock16::default();
    gcm_hash_sizes(key, &mut ctx.x, ctx.auth_size, ctx.data_size);
    
    let mut enc_result = [0u8; 16];
    f(cipher, 16, &mut enc_result, &ctx.iv.b);
    
    unsafe {
        ptr::copy_nonoverlapping(enc_result.as_ptr(), buffer.b.as_mut_ptr(), 16);
        block16_xor(&mut buffer, &ctx.x);
        ptr::copy_nonoverlapping(buffer.b.as_ptr(), digest.as_mut_ptr(), length);
    }
}

// These would need to be implemented or marked as extern
extern "C" {
    fn _nettle_ghash_set_key(ctx: *mut GcmKey, key: *const NettleBlock16);
    fn _nettle_ghash_update(
        ctx: *const GcmKey,
        state: *mut NettleBlock16,
        blocks: size_t,
        data: *const uint8_t,
    ) -> *const uint8_t;
    fn _nettle_ctr_crypt16(
        ctx: *const libc::c_void,
        f: NettleCipherFunc,
        fill: Option<NettleFill16Func>,
        ctr: *mut uint8_t,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
}