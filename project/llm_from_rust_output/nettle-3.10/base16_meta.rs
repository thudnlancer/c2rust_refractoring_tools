use std::mem;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Base16DecodeCtx {
    pub word: u8,
    pub bits: u8,
}

#[repr(C)]
pub struct NettleArmor {
    pub name: &'static str,
    pub encode_context_size: u32,
    pub decode_context_size: u32,
    pub encode_final_length: u32,
    pub encode_init: Option<fn()>,
    pub encode_length: Option<fn(usize) -> usize>,
    pub encode_update: Option<fn(&mut [u8], &[u8]) -> usize>,
    pub encode_final: Option<fn() -> usize>,
    pub decode_init: Option<fn(&mut Base16DecodeCtx)>,
    pub decode_length: Option<fn(usize) -> usize>,
    pub decode_update: Option<fn(&mut Base16DecodeCtx, &mut usize, &mut [u8], &str) -> i32>,
    pub decode_final: Option<fn(&mut Base16DecodeCtx) -> i32>,
}

fn base16_encode_length(length: usize) -> usize {
    length * 2
}

fn base16_decode_length(length: usize) -> usize {
    (length + 1) / 2
}

fn base16_encode_init() {}

fn base16_encode_update_wrapper(dst: &mut [u8], src: &[u8]) -> usize {
    // Safe wrapper around the unsafe C function
    unsafe {
        nettle_base16_encode_update(
            dst.as_mut_ptr() as *mut libc::c_char,
            dst.len(),
            src.as_ptr(),
        );
    }
    src.len() * 2
}

fn base16_encode_final() -> usize {
    0
}

extern "C" {
    fn nettle_base16_encode_update(
        dst: *mut libc::c_char,
        length: libc::size_t,
        src: *const libc::uint8_t,
    );
    fn nettle_base16_decode_init(ctx: *mut Base16DecodeCtx);
    fn nettle_base16_decode_update(
        ctx: *mut Base16DecodeCtx,
        dst_length: *mut libc::size_t,
        dst: *mut libc::uint8_t,
        src_length: libc::size_t,
        src: *const libc::c_char,
    ) -> libc::c_int;
    fn nettle_base16_decode_final(ctx: *mut Base16DecodeCtx) -> libc::c_int;
}

pub static NETTLE_BASE16: NettleArmor = NettleArmor {
    name: "base16",
    encode_context_size: 0,
    decode_context_size: mem::size_of::<Base16DecodeCtx>() as u32,
    encode_final_length: 0,
    encode_init: Some(base16_encode_init),
    encode_length: Some(base16_encode_length),
    encode_update: Some(base16_encode_update_wrapper),
    encode_final: Some(base16_encode_final),
    decode_init: Some(|ctx| unsafe { nettle_base16_decode_init(ctx) }),
    decode_length: Some(base16_decode_length),
    decode_update: Some(|ctx, dst_len, dst, src| unsafe {
        nettle_base16_decode_update(
            ctx,
            dst_len,
            dst.as_mut_ptr(),
            src.len(),
            src.as_ptr() as *const libc::c_char,
        )
    }),
    decode_final: Some(|ctx| unsafe { nettle_base16_decode_final(ctx) }),
};