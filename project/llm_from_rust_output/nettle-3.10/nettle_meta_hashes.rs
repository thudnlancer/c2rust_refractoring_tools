use std::ffi::CStr;
use std::marker::PhantomData;

pub type size_t = usize;
pub type uint8_t = u8;

pub struct NettleHash {
    name: &'static CStr,
    context_size: u32,
    digest_size: u32,
    block_size: u32,
    init: Option<unsafe extern "C" fn(*mut std::ffi::c_void)>,
    update: Option<unsafe extern "C" fn(*mut std::ffi::c_void, size_t, *const uint8_t)>,
    digest: Option<unsafe extern "C" fn(*mut std::ffi::c_void, size_t, *mut uint8_t)>,
}

extern "C" {
    static nettle_sm3: NettleHash;
    static nettle_streebog512: NettleHash;
    static nettle_streebog256: NettleHash;
    static nettle_sha3_512: NettleHash;
    static nettle_sha3_384: NettleHash;
    static nettle_sha3_256: NettleHash;
    static nettle_sha3_224: NettleHash;
    static nettle_sha512_256: NettleHash;
    static nettle_sha512_224: NettleHash;
    static nettle_sha512: NettleHash;
    static nettle_sha384: NettleHash;
    static nettle_sha256: NettleHash;
    static nettle_sha224: NettleHash;
    static nettle_sha1: NettleHash;
    static nettle_ripemd160: NettleHash;
    static nettle_md5: NettleHash;
    static nettle_md4: NettleHash;
    static nettle_md2: NettleHash;
    static nettle_gosthash94cp: NettleHash;
    static nettle_gosthash94: NettleHash;
}

pub struct NettleHashes {
    hashes: [&'static NettleHash; 20],
    _marker: PhantomData<*const NettleHash>,
}

impl NettleHashes {
    pub fn new() -> Self {
        Self {
            hashes: [
                unsafe { &nettle_gosthash94 },
                unsafe { &nettle_gosthash94cp },
                unsafe { &nettle_md2 },
                unsafe { &nettle_md4 },
                unsafe { &nettle_md5 },
                unsafe { &nettle_ripemd160 },
                unsafe { &nettle_sha1 },
                unsafe { &nettle_sha224 },
                unsafe { &nettle_sha256 },
                unsafe { &nettle_sha384 },
                unsafe { &nettle_sha512 },
                unsafe { &nettle_sha512_224 },
                unsafe { &nettle_sha512_256 },
                unsafe { &nettle_sha3_224 },
                unsafe { &nettle_sha3_256 },
                unsafe { &nettle_sha3_384 },
                unsafe { &nettle_sha3_512 },
                unsafe { &nettle_streebog256 },
                unsafe { &nettle_streebog512 },
                unsafe { &nettle_sm3 },
            ],
            _marker: PhantomData,
        }
    }

    pub fn get_hashes(&self) -> &[&'static NettleHash] {
        &self.hashes
    }
}

pub fn nettle_get_hashes() -> &'static [&'static NettleHash] {
    static HASHES: NettleHashes = NettleHashes::new();
    HASHES.get_hashes()
}