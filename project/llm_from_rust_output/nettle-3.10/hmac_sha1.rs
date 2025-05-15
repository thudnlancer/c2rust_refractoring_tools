use std::ffi::CStr;

pub type size_t = usize;
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type uint64_t = u64;

#[derive(Clone, Copy)]
pub struct NettleHash {
    pub name: &'static CStr,
    pub context_size: u32,
    pub digest_size: u32,
    pub block_size: u32,
    pub init: fn(&mut [u8]),
    pub update: fn(&mut [u8], &[u8]),
    pub digest: fn(&mut [u8], &mut [u8]),
}

#[derive(Clone, Copy)]
pub struct Sha1Context {
    pub state: [uint32_t; 5],
    pub count: uint64_t,
    pub index: u32,
    pub block: [uint8_t; 64],
}

#[derive(Clone, Copy)]
pub struct HmacSha1Context {
    pub outer: Sha1Context,
    pub inner: Sha1Context,
    pub state: Sha1Context,
}

pub static NETTLE_SHA1: NettleHash = {
    extern "C" {
        fn nettle_sha1_init(ctx: *mut libc::c_void);
        fn nettle_sha1_update(ctx: *mut libc::c_void, length: size_t, data: *const uint8_t);
        fn nettle_sha1_digest(ctx: *mut libc::c_void, length: size_t, digest: *mut uint8_t);
    }

    NettleHash {
        name: unsafe { CStr::from_bytes_with_nul_unchecked(b"sha1\0") },
        context_size: std::mem::size_of::<Sha1Context>() as u32,
        digest_size: 20,
        block_size: 64,
        init: |ctx| unsafe {
            nettle_sha1_init(ctx.as_mut_ptr() as *mut libc::c_void)
        },
        update: |ctx, data| unsafe {
            nettle_sha1_update(
                ctx.as_mut_ptr() as *mut libc::c_void,
                data.len(),
                data.as_ptr(),
            )
        },
        digest: |ctx, digest| unsafe {
            nettle_sha1_digest(
                ctx.as_mut_ptr() as *mut libc::c_void,
                digest.len(),
                digest.as_mut_ptr(),
            )
        },
    }
};

impl HmacSha1Context {
    pub fn set_key(&mut self, key: &[u8]) {
        unsafe {
            nettle_hmac_set_key(
                &mut self.outer as *mut Sha1Context as *mut libc::c_void,
                &mut self.inner as *mut Sha1Context as *mut libc::c_void,
                &mut self.state as *mut Sha1Context as *mut libc::c_void,
                &NETTLE_SHA1,
                key.len(),
                key.as_ptr(),
            );
        }
    }

    pub fn update(&mut self, data: &[u8]) {
        unsafe {
            nettle_sha1_update(
                &mut self.state as *mut Sha1Context,
                data.len(),
                data.as_ptr(),
            );
        }
    }

    pub fn digest(&mut self, digest: &mut [u8]) {
        unsafe {
            nettle_hmac_digest(
                &mut self.outer as *mut Sha1Context as *const libc::c_void,
                &mut self.inner as *mut Sha1Context as *const libc::c_void,
                &mut self.state as *mut Sha1Context as *mut libc::c_void,
                &NETTLE_SHA1,
                digest.len(),
                digest.as_mut_ptr(),
            );
        }
    }
}

extern "C" {
    fn nettle_hmac_set_key(
        outer: *mut libc::c_void,
        inner: *mut libc::c_void,
        state: *mut libc::c_void,
        hash: *const NettleHash,
        length: size_t,
        key: *const uint8_t,
    );
    fn nettle_hmac_digest(
        outer: *const libc::c_void,
        inner: *const libc::c_void,
        state: *mut libc::c_void,
        hash: *const NettleHash,
        length: size_t,
        digest: *mut uint8_t,
    );
    fn nettle_sha1_update(ctx: *mut Sha1Context, length: size_t, data: *const uint8_t);
}