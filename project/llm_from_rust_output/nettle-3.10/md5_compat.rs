use std::mem::MaybeUninit;

pub type size_t = usize;
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type uint64_t = u64;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Md5Ctx {
    pub state: [uint32_t; 4],
    pub count: uint64_t,
    pub index: u32,
    pub block: [uint8_t; 64],
}

pub struct Md5 {
    ctx: Md5Ctx,
}

impl Md5 {
    pub fn new() -> Self {
        let mut ctx = MaybeUninit::<Md5Ctx>::uninit();
        unsafe {
            nettle_md5_init(ctx.as_mut_ptr());
            Self {
                ctx: ctx.assume_init(),
            }
        }
    }

    pub fn update(&mut self, data: &[u8]) {
        unsafe {
            nettle_md5_update(
                &mut self.ctx as *mut Md5Ctx,
                data.len() as size_t,
                data.as_ptr(),
            );
        }
    }

    pub fn digest(mut self, out: &mut [u8; 16]) {
        unsafe {
            nettle_md5_digest(
                &mut self.ctx as *mut Md5Ctx,
                16 as size_t,
                out.as_mut_ptr(),
            );
        }
    }
}

extern "C" {
    fn nettle_md5_digest(ctx: *mut Md5Ctx, length: size_t, digest: *mut uint8_t);
    fn nettle_md5_update(ctx: *mut Md5Ctx, length: size_t, data: *const uint8_t);
    fn nettle_md5_init(ctx: *mut Md5Ctx);
}