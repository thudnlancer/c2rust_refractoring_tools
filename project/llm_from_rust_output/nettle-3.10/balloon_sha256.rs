use std::mem;

pub type SizeT = usize;
pub type Uint8T = u8;
pub type Uint32T = u32;
pub type Uint64T = u64;

pub struct Sha256Ctx {
    pub state: [Uint32T; 8],
    pub count: Uint64T,
    pub index: u32,
    pub block: [Uint8T; 64],
}

impl Sha256Ctx {
    pub fn new() -> Self {
        Sha256Ctx {
            state: [0; 8],
            count: 0,
            index: 0,
            block: [0; 64],
        }
    }

    pub fn init(&mut self) {
        unsafe { nettle_sha256_init(self) }
    }

    pub fn update(&mut self, data: &[Uint8T]) {
        unsafe { nettle_sha256_update(self, data.len(), data.as_ptr()) }
    }

    pub fn digest(&mut self, output: &mut [Uint8T]) {
        unsafe { nettle_sha256_digest(self, output.len(), output.as_mut_ptr()) }
    }
}

pub fn balloon_sha256(
    s_cost: SizeT,
    t_cost: SizeT,
    passwd: &[Uint8T],
    salt: &[Uint8T],
    scratch: &mut [Uint8T],
    dst: &mut [Uint8T],
) {
    let mut ctx = Sha256Ctx::new();
    ctx.init();

    unsafe {
        nettle_balloon(
            &mut ctx as *mut Sha256Ctx as *mut libc::c_void,
            Some(mem::transmute(nettle_sha256_update as unsafe extern "C" fn(*mut Sha256Ctx, SizeT, *const Uint8T))),
            Some(mem::transmute(nettle_sha256_digest as unsafe extern "C" fn(*mut Sha256Ctx, SizeT, *mut Uint8T))),
            32,
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

extern "C" {
    fn nettle_balloon(
        hash_ctx: *mut libc::c_void,
        update: Option<unsafe extern "C" fn(*mut libc::c_void, SizeT, *const Uint8T)>,
        digest: Option<unsafe extern "C" fn(*mut libc::c_void, SizeT, *mut Uint8T)>,
        digest_size: SizeT,
        s_cost: SizeT,
        t_cost: SizeT,
        passwd_length: SizeT,
        passwd: *const Uint8T,
        salt_length: SizeT,
        salt: *const Uint8T,
        scratch: *mut Uint8T,
        dst: *mut Uint8T,
    );
    fn nettle_sha256_init(ctx: *mut Sha256Ctx);
    fn nettle_sha256_update(ctx: *mut Sha256Ctx, length: SizeT, data: *const Uint8T);
    fn nettle_sha256_digest(ctx: *mut Sha256Ctx, length: SizeT, digest: *mut Uint8T);
}