use std::mem;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct NettleBlock16 {
    pub b: [u8; 16],
    pub w: [u64; 2],
    pub u64_0: [u64; 2],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Aes128Ctx {
    pub keys: [u32; 44],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Poly1305Ctx {
    pub r: Poly1305R,
    pub s32: [u32; 3],
    pub hh: u32,
    pub h: Poly1305H,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union Poly1305H {
    pub h32: [u32; 4],
    pub h64: [u64; 2],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union Poly1305R {
    pub r32: [u32; 6],
    pub r64: [u64; 3],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Poly1305AesCtx {
    pub pctx: Poly1305Ctx,
    pub block: [u8; 16],
    pub index: u32,
    pub nonce: [u8; 16],
    pub aes: Aes128Ctx,
}

pub fn nettle_poly1305_aes_set_key(ctx: &mut Poly1305AesCtx, key: &[u8; 32]) {
    unsafe {
        nettle_aes128_set_encrypt_key(&mut ctx.aes, key.as_ptr());
        _nettle_poly1305_set_key(&mut ctx.pctx, key[16..].as_ptr());
    }
    ctx.index = 0;
}

pub fn nettle_poly1305_aes_set_nonce(ctx: &mut Poly1305AesCtx, nonce: &[u8; 16]) {
    ctx.nonce.copy_from_slice(nonce);
}

pub fn nettle_poly1305_aes_update(ctx: &mut Poly1305AesCtx, data: &[u8]) {
    let length = data.len();
    unsafe {
        ctx.index = _nettle_poly1305_update(
            &mut ctx.pctx,
            ctx.block.as_mut_ptr(),
            ctx.index,
            length,
            data.as_ptr(),
        );
    }
}

pub fn nettle_poly1305_aes_digest(ctx: &mut Poly1305AesCtx, length: usize, digest: &mut [u8]) {
    let mut s = NettleBlock16 { b: [0; 16] };

    if ctx.index > 0 {
        assert!(ctx.index < 16, "ctx->index < POLY1305_BLOCK_SIZE");
        
        ctx.block[ctx.index as usize] = 1;
        let start = ctx.index as usize + 1;
        let end = 16;
        ctx.block[start..end].fill(0);
        
        unsafe {
            _nettle_poly1305_block(
                &mut ctx.pctx,
                ctx.block.as_ptr(),
                0,
            );
        }
    }

    unsafe {
        nettle_aes128_encrypt(
            &ctx.aes,
            16,
            s.b.as_mut_ptr(),
            ctx.nonce.as_ptr(),
        );
        _nettle_poly1305_digest(&mut ctx.pctx, &mut s);
    }

    let copy_len = length.min(16);
    digest[..copy_len].copy_from_slice(&s.b[..copy_len]);

    let mut increment_i = 15;
    ctx.nonce[increment_i] = ctx.nonce[increment_i].wrapping_add(1);
    
    if ctx.nonce[increment_i] == 0 {
        while increment_i > 0 {
            increment_i -= 1;
            ctx.nonce[increment_i] = ctx.nonce[increment_i].wrapping_add(1);
            if ctx.nonce[increment_i] != 0 {
                break;
            }
        }
    }
    
    ctx.index = 0;
}

// External FFI functions - these would need to be implemented or linked
extern "C" {
    fn nettle_aes128_set_encrypt_key(ctx: *mut Aes128Ctx, key: *const u8);
    fn nettle_aes128_encrypt(ctx: *const Aes128Ctx, length: usize, dst: *mut u8, src: *const u8);
    fn _nettle_poly1305_set_key(ctx: *mut Poly1305Ctx, key: *const u8);
    fn _nettle_poly1305_digest(ctx: *mut Poly1305Ctx, s: *mut NettleBlock16);
    fn _nettle_poly1305_block(ctx: *mut Poly1305Ctx, m: *const u8, high: u32);
    fn _nettle_poly1305_update(
        ctx: *mut Poly1305Ctx,
        buffer: *mut u8,
        index: u32,
        length: usize,
        m: *const u8,
    ) -> u32;
}