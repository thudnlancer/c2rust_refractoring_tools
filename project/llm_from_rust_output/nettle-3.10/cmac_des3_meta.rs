use std::mem;
use std::os::raw::{c_uchar, c_uint, c_ulong, c_void, c_char};

pub type size_t = c_ulong;
pub type uint8_t = c_uchar;
pub type uint32_t = c_uint;
pub type uint64_t = c_ulong;

#[derive(Copy, Clone)]
#[repr(C)]
pub union NettleBlock8 {
    pub b: [uint8_t; 8],
    pub u64_0: uint64_t,
}

pub type NettleSetKeyFunc = fn(*mut c_void, *const uint8_t);
pub type NettleHashUpdateFunc = fn(*mut c_void, size_t, *const uint8_t);
pub type NettleHashDigestFunc = fn(*mut c_void, size_t, *mut uint8_t);

#[derive(Copy, Clone)]
#[repr(C)]
pub struct NettleMac {
    pub name: *const c_char,
    pub context_size: c_uint,
    pub digest_size: c_uint,
    pub key_size: c_uint,
    pub set_key: Option<NettleSetKeyFunc>,
    pub update: Option<NettleHashUpdateFunc>,
    pub digest: Option<NettleHashDigestFunc>,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct CmacDes3Ctx {
    pub key: Cmac64Key,
    pub ctx: Cmac64Ctx,
    pub cipher: Des3Ctx,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Des3Ctx {
    pub des: [DesCtx; 3],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct DesCtx {
    pub key: [uint32_t; 32],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Cmac64Ctx {
    pub X: NettleBlock8,
    pub block: NettleBlock8,
    pub index: size_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Cmac64Key {
    pub K1: NettleBlock8,
    pub K2: NettleBlock8,
}

pub static NETTLE_CMAC_DES3: NettleMac = NettleMac {
    name: b"cmac_des3\0".as_ptr() as *const c_char,
    context_size: mem::size_of::<CmacDes3Ctx>() as c_uint,
    digest_size: 8 as c_uint,
    key_size: 24 as c_uint,
    set_key: Some(nettle_cmac_des3_set_key as NettleSetKeyFunc),
    update: Some(nettle_cmac_des3_update as NettleHashUpdateFunc),
    digest: Some(nettle_cmac_des3_digest as NettleHashDigestFunc),
};

extern "C" {
    pub fn nettle_cmac_des3_set_key(ctx: *mut CmacDes3Ctx, key: *const uint8_t);
    pub fn nettle_cmac_des3_update(ctx: *mut CmacDes3Ctx, length: size_t, data: *const uint8_t);
    pub fn nettle_cmac_des3_digest(ctx: *mut CmacDes3Ctx, length: size_t, digest: *mut uint8_t);
}