use std::ffi::CStr;
use std::os::raw::{c_char, c_uint, c_uchar, c_ulong};
use std::ptr;

type size_t = c_ulong;
type uint8_t = c_uchar;

pub struct NettleHash {
    pub name: &'static str,
    pub context_size: c_uint,
    pub digest_size: c_uint,
    pub block_size: c_uint,
    pub init: Option<fn(*mut ())>,
    pub update: Option<fn(*mut (), size_t, *const uint8_t)>,
    pub digest: Option<fn(*mut (), size_t, *mut uint8_t)>,
}

static NETTLE_HASHES: &[Option<&NettleHash>] = &[];

pub fn nettle_lookup_hash(name: &str) -> Option<&'static NettleHash> {
    let c_name = std::ffi::CString::new(name).ok()?;
    
    NETTLE_HASHES.iter()
        .filter_map(|&opt_hash| opt_hash)
        .find(|hash| {
            let hash_name = unsafe { CStr::from_ptr(hash.name.as_ptr() as *const c_char) };
            hash_name.to_str().ok() == Some(name)
        })
        .copied()
}