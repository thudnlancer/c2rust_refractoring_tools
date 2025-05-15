use std::ffi::CStr;
use std::marker::PhantomData;

pub type size_t = usize;
pub type uint8_t = u8;

pub struct NettleMac {
    name: &'static str,
    context_size: u32,
    digest_size: u32,
    key_size: u32,
    set_key: Option<fn(context: &mut [u8], key: &[u8])>,
    update: Option<fn(context: &mut [u8], length: size_t, data: &[u8])>,
    digest: Option<fn(context: &mut [u8], length: size_t, digest: &mut [u8])>,
}

macro_rules! define_mac {
    ($name:ident, $context_size:expr, $digest_size:expr, $key_size:expr, $set_key:expr, $update:expr, $digest:expr) => {
        pub const $name: NettleMac = NettleMac {
            name: stringify!($name),
            context_size: $context_size,
            digest_size: $digest_size,
            key_size: $key_size,
            set_key: $set_key,
            update: $update,
            digest: $digest,
        };
    };
}

define_mac!(NETTLE_CMAC_AES128, 0, 0, 0, None, None, None);
define_mac!(NETTLE_CMAC_AES256, 0, 0, 0, None, None, None);
define_mac!(NETTLE_CMAC_DES3, 0, 0, 0, None, None, None);
define_mac!(NETTLE_HMAC_MD5, 0, 0, 0, None, None, None);
define_mac!(NETTLE_HMAC_RIPEMD160, 0, 0, 0, None, None, None);
define_mac!(NETTLE_HMAC_SHA1, 0, 0, 0, None, None, None);
define_mac!(NETTLE_HMAC_SHA224, 0, 0, 0, None, None, None);
define_mac!(NETTLE_HMAC_SHA256, 0, 0, 0, None, None, None);
define_mac!(NETTLE_HMAC_SHA384, 0, 0, 0, None, None, None);
define_mac!(NETTLE_HMAC_SHA512, 0, 0, 0, None, None, None);
define_mac!(NETTLE_HMAC_STREEBOG256, 0, 0, 0, None, None, None);
define_mac!(NETTLE_HMAC_STREEBOG512, 0, 0, 0, None, None, None);
define_mac!(NETTLE_HMAC_SM3, 0, 0, 0, None, None, None);

pub fn nettle_get_macs() -> &'static [&'static NettleMac] {
    &[
        &NETTLE_CMAC_AES128,
        &NETTLE_CMAC_AES256,
        &NETTLE_CMAC_DES3,
        &NETTLE_HMAC_MD5,
        &NETTLE_HMAC_RIPEMD160,
        &NETTLE_HMAC_SHA1,
        &NETTLE_HMAC_SHA224,
        &NETTLE_HMAC_SHA256,
        &NETTLE_HMAC_SHA384,
        &NETTLE_HMAC_SHA512,
        &NETTLE_HMAC_STREEBOG256,
        &NETTLE_HMAC_STREEBOG512,
        &NETTLE_HMAC_SM3,
    ]
}