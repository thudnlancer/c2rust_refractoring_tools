use std::mem;
use std::os::raw::{c_char, c_uchar, c_ulong, c_uint, c_void};

pub type size_t = c_ulong;
pub type uint8_t = c_uchar;
pub type uint64_t = c_ulong;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Sha3State {
    pub a: [uint64_t; 25],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Sha3384Ctx {
    pub state: Sha3State,
    pub index: c_uint,
    pub block: [uint8_t; 104],
}

pub type NettleHashInitFunc = unsafe extern "C" fn(*mut c_void);
pub type NettleHashUpdateFunc = unsafe extern "C" fn(*mut c_void, size_t, *const uint8_t);
pub type NettleHashDigestFunc = unsafe extern "C" fn(*mut c_void, size_t, *mut uint8_t);

#[derive(Copy, Clone)]
#[repr(C)]
pub struct NettleHash {
    pub name: *const c_char,
    pub context_size: c_uint,
    pub digest_size: c_uint,
    pub block_size: c_uint,
    pub init: Option<NettleHashInitFunc>,
    pub update: Option<NettleHashUpdateFunc>,
    pub digest: Option<NettleHashDigestFunc>,
}

extern "C" {
    fn nettle_sha3_384_init(ctx: *mut Sha3384Ctx);
    fn nettle_sha3_384_update(ctx: *mut Sha3384Ctx, length: size_t, data: *const uint8_t);
    fn nettle_sha3_384_digest(ctx: *mut Sha3384Ctx, length: size_t, digest: *mut uint8_t);
}

pub static NETTLE_SHA3_384: NettleHash = NettleHash {
    name: b"sha3_384\0".as_ptr() as *const c_char,
    context_size: mem::size_of::<Sha3384Ctx>() as c_uint,
    digest_size: 48,
    block_size: 104,
    init: Some(nettle_sha3_384_init as NettleHashInitFunc),
    update: Some(nettle_sha3_384_update as NettleHashUpdateFunc),
    digest: Some(nettle_sha3_384_digest as NettleHashDigestFunc),
};