use std::env;
use std::ffi::{CStr, CString};
use std::mem;
use std::os::raw::{c_char, c_int, c_uint, c_ulong, c_uchar, c_void};
use std::ptr;

type size_t = c_ulong;
type uint8_t = c_uchar;
type uint32_t = c_uint;
type uint64_t = c_ulong;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct nettle_block16 {
    pub b: [uint8_t; 16],
    pub w: [c_ulong; 2],
    pub u64_0: [uint64_t; 2],
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct aes128_ctx {
    pub keys: [uint32_t; 44],
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct aes192_ctx {
    pub keys: [uint32_t; 52],
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct aes256_ctx {
    pub keys: [uint32_t; 60],
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct gcm_key {
    pub h: [nettle_block16; 256],
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum x86_vendor {
    X86_OTHER = 0,
    X86_INTEL = 1,
    X86_AMD = 2,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct x86_features {
    pub vendor: x86_vendor,
    pub have_aesni: c_int,
    pub have_sha_ni: c_int,
    pub have_pclmul: c_int,
}

type aes128_crypt_func = fn(*const aes128_ctx, size_t, *mut uint8_t, *const uint8_t);
type aes192_crypt_func = fn(*const aes192_ctx, size_t, *mut uint8_t, *const uint8_t);
type aes256_crypt_func = fn(*const aes256_ctx, size_t, *mut uint8_t, *const uint8_t);
type cbc_aes128_encrypt_func = fn(*const aes128_ctx, *mut uint8_t, size_t, *mut uint8_t, *const uint8_t);
type cbc_aes192_encrypt_func = fn(*const aes192_ctx, *mut uint8_t, size_t, *mut uint8_t, *const uint8_t);
type cbc_aes256_encrypt_func = fn(*const aes256_ctx, *mut uint8_t, size_t, *mut uint8_t, *const uint8_t);
type memxor_func = fn(*mut c_void, *const c_void, size_t) -> *mut c_void;
type sha1_compress_func = fn(*mut uint32_t, *const uint8_t);
type sha256_compress_n_func = fn(*mut uint32_t, *const uint32_t, size_t, *const uint8_t) -> *const uint8_t;
type ghash_set_key_func = fn(*mut gcm_key, *const nettle_block16);
type ghash_update_func = fn(*const gcm_key, *mut nettle_block16, size_t, *const uint8_t) -> *const uint8_t;

static mut nettle_aes128_encrypt_vec: Option<aes128_crypt_func> = None;
static mut nettle_aes128_decrypt_vec: Option<aes128_crypt_func> = None;
static mut nettle_aes192_encrypt_vec: Option<aes192_crypt_func> = None;
static mut nettle_aes192_decrypt_vec: Option<aes192_crypt_func> = None;
static mut nettle_aes256_encrypt_vec: Option<aes256_crypt_func> = None;
static mut nettle_aes256_decrypt_vec: Option<aes256_crypt_func> = None;
static mut nettle_cbc_aes128_encrypt_vec: Option<cbc_aes128_encrypt_func> = None;
static mut nettle_cbc_aes192_encrypt_vec: Option<cbc_aes192_encrypt_func> = None;
static mut nettle_cbc_aes256_encrypt_vec: Option<cbc_aes256_encrypt_func> = None;
static mut nettle_memxor_vec: Option<memxor_func> = None;
static mut nettle_sha1_compress_vec: Option<sha1_compress_func> = None;
static mut _nettle_sha256_compress_n_vec: Option<sha256_compress_n_func> = None;
static mut _nettle_ghash_set_key_vec: Option<ghash_set_key_func> = None;
static mut _nettle_ghash_update_vec: Option<ghash_update_func> = None;

extern "C" {
    fn _nettle_cpuid(input: uint32_t, regs: *mut uint32_t);
    fn _nettle_aes128_encrypt_c(ctx: *const aes128_ctx, length: size_t, dst: *mut uint8_t, src: *const uint8_t);
    fn _nettle_aes128_encrypt_aesni(ctx: *const aes128_ctx, length: size_t, dst: *mut uint8_t, src: *const uint8_t);
    fn _nettle_aes128_decrypt_c(ctx: *const aes128_ctx, length: size_t, dst: *mut uint8_t, src: *const uint8_t);
    fn _nettle_aes128_decrypt_aesni(ctx: *const aes128_ctx, length: size_t, dst: *mut uint8_t, src: *const uint8_t);
    fn _nettle_aes192_encrypt_c(ctx: *const aes192_ctx, length: size_t, dst: *mut uint8_t, src: *const uint8_t);
    fn _nettle_aes192_encrypt_aesni(ctx: *const aes192_ctx, length: size_t, dst: *mut uint8_t, src: *const uint8_t);
    fn _nettle_aes192_decrypt_c(ctx: *const aes192_ctx, length: size_t, dst: *mut uint8_t, src: *const uint8_t);
    fn _nettle_aes192_decrypt_aesni(ctx: *const aes192_ctx, length: size_t, dst: *mut uint8_t, src: *const uint8_t);
    fn _nettle_aes256_encrypt_c(ctx: *const aes256_ctx, length: size_t, dst: *mut uint8_t, src: *const uint8_t);
    fn _nettle_aes256_encrypt_aesni(ctx: *const aes256_ctx, length: size_t, dst: *mut uint8_t, src: *const uint8_t);
    fn _nettle_aes256_decrypt_c(ctx: *const aes256_ctx, length: size_t, dst: *mut uint8_t, src: *const uint8_t);
    fn _nettle_aes256_decrypt_aesni(ctx: *const aes256_ctx, length: size_t, dst: *mut uint8_t, src: *const uint8_t);
    fn _nettle_cbc_aes128_encrypt_c(ctx: *const aes128_ctx, iv: *mut uint8_t, length: size_t, dst: *mut uint8_t, src: *const uint8_t);
    fn _nettle_cbc_aes128_encrypt_aesni(ctx: *const aes128_ctx, iv: *mut uint8_t, length: size_t, dst: *mut uint8_t, src: *const uint8_t);
    fn _nettle_cbc_aes192_encrypt_c(ctx: *const aes192_ctx, iv: *mut uint8_t, length: size_t, dst: *mut uint8_t, src: *const uint8_t);
    fn _nettle_cbc_aes192_encrypt_aesni(ctx: *const aes192_ctx, iv: *mut uint8_t, length: size_t, dst: *mut uint8_t, src: *const uint8_t);
    fn _nettle_cbc_aes256_encrypt_c(ctx: *const aes256_ctx, iv: *mut uint8_t, length: size_t, dst: *mut uint8_t, src: *const uint8_t);
    fn _nettle_cbc_aes256_encrypt_aesni(ctx: *const aes256_ctx, iv: *mut uint8_t, length: size_t, dst: *mut uint8_t, src: *const uint8_t);
    fn _nettle_memxor_x86_64(dst: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn _nettle_memxor_sse2(dst: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn _nettle_sha1_compress_x86_64(state: *mut uint32_t, input: *const uint8_t);
    fn _nettle_sha1_compress_sha_ni(state: *mut uint32_t, input: *const uint8_t);
    fn _nettle_sha256_compress_n_x86_64(state: *mut uint32_t, k: *const uint32_t, blocks: size_t, input: *const uint8_t) -> *const uint8_t;
    fn _nettle_sha256_compress_n_sha_ni(state: *mut uint32_t, k: *const uint32_t, blocks: size_t, input: *const uint8_t) -> *const uint8_t;
    fn _nettle_ghash_set_key_c(ctx: *mut gcm_key, key: *const nettle_block16);
    fn _nettle_ghash_set_key_pclmul(ctx: *mut gcm_key, key: *const nettle_block16);
    fn _nettle_ghash_update_table(ctx: *const gcm_key, state: *mut nettle_block16, blocks: size_t, data: *const uint8_t) -> *const uint8_t;
    fn _nettle_ghash_update_pclmul(ctx: *const gcm_key, state: *mut nettle_block16, blocks: size_t, data: *const uint8_t) -> *const uint8_t;
}

fn get_x86_features(features: &mut x86_features) {
    let override_var = env::var("NETTLE_FAT_OVERRIDE").ok();
    
    if let Some(override_val) = override_var {
        for part in override_val.split(',') {
            if part.starts_with("vendor:") {
                let vendor = &part[7..];
                features.vendor = match vendor {
                    "intel" => x86_vendor::X86_INTEL,
                    "amd" => x86_vendor::X86_AMD,
                    _ => x86_vendor::X86_OTHER,
                };
            } else if part == "aesni" {
                features.have_aesni = 1;
            } else if part == "sha_ni" {
                features.have_sha_ni = 1;
            } else if part == "pclmul" {
                features.have_pclmul = 1;
            }
        }
    } else {
        let mut cpuid_data: [uint32_t; 4] = [0; 4];
        
        unsafe {
            _nettle_cpuid(0, cpuid_data.as_mut_ptr());
            
            let vendor_str = &cpuid_data[1..4];
            if vendor_str == b"GenuineIntel" {
                features.vendor = x86_vendor::X86_INTEL;
            } else if vendor_str == b"AuthenticAMD" {
                features.vendor = x86_vendor::X86_AMD;
            }
            
            _nettle_cpuid(1, cpuid_data.as_mut_ptr());
            features.have_pclmul = if cpuid_data[2] & 0x2 != 0 { 1 } else { 0 };
            features.have_aesni = if cpuid_data[2] & 0x2000000 != 0 { 1 } else { 0 };
            
            _nettle_cpuid(7, cpuid_data.as_mut_ptr());
            features.have_sha_ni = if cpuid_data[1] & 0x20000000 != 0 { 1 } else { 0 };
        }
    }
}

fn fat_init() {
    let verbose = env::var("NETTLE_FAT_VERBOSE").is_ok();
    let mut features = x86_features {
        vendor: x86_vendor::X86_OTHER,
        have_aesni: 0,
        have_sha_ni: 0,
        have_pclmul: 0,
    };
    
    get_x86_features(&mut features);
    
    if verbose {
        let vendor_names = ["other", "intel", "amd"];
        println!(
            "libnettle: cpu features: vendor:{}{}{}{}",
            vendor_names[features.vendor as usize],
            if features.have_aesni != 0 { ",aesni" } else { "" },
            if features.have_sha_ni != 0 { ",sha_ni" } else { "" },
            if features.have_pclmul != 0 { ",pclmul" } else { "" }
        );
    }
    
    unsafe {
        if features.have_aesni != 0 {
            if verbose {
                println!("libnettle: using aes instructions.");
            }
            nettle_aes128_encrypt_vec = Some(_nettle_aes128_encrypt_aesni);
            nettle_aes128_decrypt_vec = Some(_nettle_aes128_decrypt_aesni);
            nettle_aes192_encrypt_vec = Some(_nettle_aes192_encrypt_aesni);
            nettle_aes192_decrypt_vec = Some(_nettle_aes192_decrypt_aesni);
            nettle_aes256_encrypt_vec = Some(_nettle_aes256_encrypt_aesni);
            nettle_aes256_decrypt_vec = Some(_nettle_aes256_decrypt_aesni);
            nettle_cbc_aes128_encrypt_vec = Some(_nettle_cbc_aes128_encrypt_aesni);
            nettle_cbc_aes192_encrypt_vec = Some(_nettle_cbc_aes192_encrypt_aesni);
            nettle_cbc_aes256_encrypt_vec = Some(_nettle_cbc_aes256_encrypt_aesni);
        } else {
            if verbose {
                println!("libnettle: not using aes instructions.");
            }
            nettle_aes128_encrypt_vec = Some(_nettle_aes128_encrypt_c);
            nettle_aes128_decrypt_vec = Some(_nettle_aes128_decrypt_c);
            nettle_aes192_encrypt_vec = Some(_nettle_aes192_encrypt_c);
            nettle_aes192_decrypt_vec = Some(_nettle_aes192_decrypt_c);
            nettle_aes256_encrypt_vec = Some(_nettle_aes256_encrypt_c);
            nettle_aes256_decrypt_vec = Some(_nettle_aes256_decrypt_c);
            nettle_cbc_aes128_encrypt_vec = Some(_nettle_cbc_aes128_encrypt_c);
            nettle_cbc_aes192_encrypt_vec = Some(_nettle_cbc_aes192_encrypt_c);
            nettle_cbc_aes256_encrypt_vec = Some(_nettle_cbc_aes256_encrypt_c);
        }
        
        if features.have_sha_ni != 0 {
            if verbose {
                println!("libnettle: using sha_ni instructions.");
            }
            nettle_sha1_compress_vec = Some(_nettle_sha1_compress_sha_ni);
            _nettle_sha256_compress_n_vec = Some(_nettle_sha256_compress_n_sha_ni);
        } else {
            if verbose {
                println!("libnettle: not using sha_ni instructions.");
            }
            nettle_sha1_compress_vec = Some(_nettle_sha1_compress_x86_64);
            _nettle_sha256_compress_n_vec = Some(_nettle_sha256_compress_n_x86_64);
        }
        
        if features.have_pclmul != 0 {
            if verbose {
                println!("libnettle: using pclmulqdq instructions.");
            }
            _nettle_ghash_set_key_vec = Some(_nettle_ghash_set_key_pclmul);
            _nettle_ghash_update_vec = Some(_nettle_ghash_update_pclmul);
        } else {
            if verbose {
                println!("libnettle: not using pclmulqdq instructions.");
            }
            _nettle_ghash_set_key_vec = Some(_nettle_ghash_set_key_c);
            _nettle_ghash_update_vec = Some(_nettle_ghash_update_table);
        }
        
        if features.vendor == x86_vendor::X86_INTEL {
            if verbose {
                println!("libnettle: intel SSE2 will be used for memxor.");
            }
            nettle_memxor_vec = Some(_nettle_memxor_sse2);
        } else {
            if verbose {
                println!("libnettle: intel SSE2 will not be used for memxor.");
            }
            nettle_memxor_vec = Some(_nettle_memxor_x86_64);
        }
    }
}

#[no_mangle]
pub extern "C" fn nettle_aes128_encrypt(
    ctx: *const aes128_ctx,
    length: size_t,
    dst: *mut uint8_t,
    src: *const uint8_t,
) {
    unsafe {
        if nettle_aes128_encrypt_vec.is_none() {
            fat_init();
        }
        nettle_aes128_encrypt_vec.unwrap()(ctx, length, dst, src);
    }
}

// Similar safe wrappers would be implemented for all other functions
// following the same pattern as above