use std::mem;

type SizeT = usize;
type UInt8T = u8;
type UInt64T = u64;

struct Sha512Ctx {
    state: [UInt64T; 8],
    count_low: UInt64T,
    count_high: UInt64T,
    index: u32,
    block: [UInt8T; 128],
}

impl Sha512Ctx {
    fn new() -> Self {
        Sha512Ctx {
            state: [0; 8],
            count_low: 0,
            count_high: 0,
            index: 0,
            block: [0; 128],
        }
    }

    fn init(&mut self) {
        unsafe {
            nettle_sha512_init(self);
        }
    }

    fn update(&mut self, data: &[UInt8T]) {
        unsafe {
            nettle_sha512_update(self, data.len(), data.as_ptr());
        }
    }

    fn digest(&mut self, output: &mut [UInt8T]) {
        unsafe {
            nettle_sha512_digest(self, output.len(), output.as_mut_ptr());
        }
    }
}

extern "C" {
    fn nettle_sha512_init(ctx: *mut Sha512Ctx);
    fn nettle_sha512_update(ctx: *mut Sha512Ctx, length: SizeT, data: *const UInt8T);
    fn nettle_sha512_digest(ctx: *mut Sha512Ctx, length: SizeT, digest: *mut UInt8T);
    fn nettle_balloon(
        hash_ctx: *mut std::ffi::c_void,
        update: Option<extern "C" fn(*mut std::ffi::c_void, SizeT, *const UInt8T)>,
        digest: Option<extern "C" fn(*mut std::ffi::c_void, SizeT, *mut UInt8T)>,
        digest_size: SizeT,
        s_cost: SizeT,
        t_cost: SizeT,
        passwd_length: SizeT,
        passwd: *const UInt8T,
        salt_length: SizeT,
        salt: *const UInt8T,
        scratch: *mut UInt8T,
        dst: *mut UInt8T,
    );
}

pub fn balloon_sha512(
    s_cost: SizeT,
    t_cost: SizeT,
    passwd: &[UInt8T],
    salt: &[UInt8T],
    scratch: &mut [UInt8T],
    dst: &mut [UInt8T],
) {
    let mut ctx = Sha512Ctx::new();
    ctx.init();

    unsafe {
        nettle_balloon(
            &mut ctx as *mut Sha512Ctx as *mut std::ffi::c_void,
            Some(mem::transmute(nettle_sha512_update as extern "C" fn(*mut Sha512Ctx, SizeT, *const UInt8T))),
            Some(mem::transmute(nettle_sha512_digest as extern "C" fn(*mut Sha512Ctx, SizeT, *mut UInt8T))),
            64,
            s_cost,
            t_cost,
            passwd.len(),
            passwd.as_ptr(),
            salt.len(),
            salt.as_ptr(),
            scratch.as_mut_ptr(),
            dst.as_mut_ptr(),
        );
    }
}