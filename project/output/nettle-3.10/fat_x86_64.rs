#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(label_break_value)]
extern "C" {
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
    static mut stderr: *mut _IO_FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn getenv(__name: *const i8) -> *mut i8;
    fn secure_getenv(__name: *const i8) -> *mut i8;
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: u64) -> i32;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn strlen(_: *const i8) -> u64;
    fn _nettle_cpuid(input: uint32_t, regs: *mut uint32_t);
    fn _nettle_aes128_encrypt_c(
        _: *const aes128_ctx,
        _: size_t,
        _: *mut uint8_t,
        _: *const uint8_t,
    );
    fn _nettle_aes128_encrypt_aesni(
        _: *const aes128_ctx,
        _: size_t,
        _: *mut uint8_t,
        _: *const uint8_t,
    );
    fn _nettle_aes128_decrypt_c(
        _: *const aes128_ctx,
        _: size_t,
        _: *mut uint8_t,
        _: *const uint8_t,
    );
    fn _nettle_aes128_decrypt_aesni(
        _: *const aes128_ctx,
        _: size_t,
        _: *mut uint8_t,
        _: *const uint8_t,
    );
    fn _nettle_aes192_encrypt_c(
        _: *const aes192_ctx,
        _: size_t,
        _: *mut uint8_t,
        _: *const uint8_t,
    );
    fn _nettle_aes192_encrypt_aesni(
        _: *const aes192_ctx,
        _: size_t,
        _: *mut uint8_t,
        _: *const uint8_t,
    );
    fn _nettle_aes192_decrypt_c(
        _: *const aes192_ctx,
        _: size_t,
        _: *mut uint8_t,
        _: *const uint8_t,
    );
    fn _nettle_aes192_decrypt_aesni(
        _: *const aes192_ctx,
        _: size_t,
        _: *mut uint8_t,
        _: *const uint8_t,
    );
    fn _nettle_aes256_encrypt_c(
        _: *const aes256_ctx,
        _: size_t,
        _: *mut uint8_t,
        _: *const uint8_t,
    );
    fn _nettle_aes256_encrypt_aesni(
        _: *const aes256_ctx,
        _: size_t,
        _: *mut uint8_t,
        _: *const uint8_t,
    );
    fn _nettle_aes256_decrypt_c(
        _: *const aes256_ctx,
        _: size_t,
        _: *mut uint8_t,
        _: *const uint8_t,
    );
    fn _nettle_aes256_decrypt_aesni(
        _: *const aes256_ctx,
        _: size_t,
        _: *mut uint8_t,
        _: *const uint8_t,
    );
    fn _nettle_cbc_aes128_encrypt_c(
        _: *const aes128_ctx,
        _: *mut uint8_t,
        _: size_t,
        _: *mut uint8_t,
        _: *const uint8_t,
    );
    fn _nettle_cbc_aes128_encrypt_aesni(
        _: *const aes128_ctx,
        _: *mut uint8_t,
        _: size_t,
        _: *mut uint8_t,
        _: *const uint8_t,
    );
    fn _nettle_cbc_aes192_encrypt_c(
        _: *const aes192_ctx,
        _: *mut uint8_t,
        _: size_t,
        _: *mut uint8_t,
        _: *const uint8_t,
    );
    fn _nettle_cbc_aes192_encrypt_aesni(
        _: *const aes192_ctx,
        _: *mut uint8_t,
        _: size_t,
        _: *mut uint8_t,
        _: *const uint8_t,
    );
    fn _nettle_cbc_aes256_encrypt_c(
        _: *const aes256_ctx,
        _: *mut uint8_t,
        _: size_t,
        _: *mut uint8_t,
        _: *const uint8_t,
    );
    fn _nettle_cbc_aes256_encrypt_aesni(
        _: *const aes256_ctx,
        _: *mut uint8_t,
        _: size_t,
        _: *mut uint8_t,
        _: *const uint8_t,
    );
    fn _nettle_memxor_x86_64(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: size_t,
    ) -> *mut libc::c_void;
    fn _nettle_memxor_sse2(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: size_t,
    ) -> *mut libc::c_void;
    fn _nettle_sha1_compress_x86_64(_: *mut uint32_t, _: *const uint8_t);
    fn _nettle_sha1_compress_sha_ni(_: *mut uint32_t, _: *const uint8_t);
    fn _nettle_sha256_compress_n_x86_64(
        _: *mut uint32_t,
        _: *const uint32_t,
        _: size_t,
        _: *const uint8_t,
    ) -> *const uint8_t;
    fn _nettle_sha256_compress_n_sha_ni(
        _: *mut uint32_t,
        _: *const uint32_t,
        _: size_t,
        _: *const uint8_t,
    ) -> *const uint8_t;
    fn _nettle_ghash_set_key_c(_: *mut gcm_key, _: *const nettle_block16);
    fn _nettle_ghash_set_key_pclmul(_: *mut gcm_key, _: *const nettle_block16);
    fn _nettle_ghash_update_table(
        _: *const gcm_key,
        _: *mut nettle_block16,
        _: size_t,
        _: *const uint8_t,
    ) -> *const uint8_t;
    fn _nettle_ghash_update_pclmul(
        _: *const gcm_key,
        _: *mut nettle_block16,
        _: size_t,
        _: *const uint8_t,
    ) -> *const uint8_t;
}
pub type size_t = u64;
pub type __uint8_t = u8;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type __off_t = i64;
pub type __off64_t = i64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: i32,
    pub _IO_read_ptr: *mut i8,
    pub _IO_read_end: *mut i8,
    pub _IO_read_base: *mut i8,
    pub _IO_write_base: *mut i8,
    pub _IO_write_ptr: *mut i8,
    pub _IO_write_end: *mut i8,
    pub _IO_buf_base: *mut i8,
    pub _IO_buf_end: *mut i8,
    pub _IO_save_base: *mut i8,
    pub _IO_backup_base: *mut i8,
    pub _IO_save_end: *mut i8,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: i32,
    pub _flags2: i32,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [i8; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub __pad1: *mut libc::c_void,
    pub __pad2: *mut libc::c_void,
    pub __pad3: *mut libc::c_void,
    pub __pad4: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: i32,
    pub _unused2: [i8; 20],
}
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: i32,
}
pub type FILE = _IO_FILE;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union nettle_block16 {
    pub b: [uint8_t; 16],
    pub w: [u64; 2],
    pub u64_0: [uint64_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aes128_ctx {
    pub keys: [uint32_t; 44],
}
pub type aes128_crypt_func = unsafe extern "C" fn(
    *const aes128_ctx,
    size_t,
    *mut uint8_t,
    *const uint8_t,
) -> ();
pub type memxor_func = unsafe extern "C" fn(
    *mut libc::c_void,
    *const libc::c_void,
    size_t,
) -> *mut libc::c_void;
pub const X86_INTEL: x86_vendor = 1;
pub type x86_vendor = u32;
pub const X86_AMD: x86_vendor = 2;
pub const X86_OTHER: x86_vendor = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x86_features {
    pub vendor: x86_vendor,
    pub have_aesni: i32,
    pub have_sha_ni: i32,
    pub have_pclmul: i32,
}
pub type ghash_update_func = unsafe extern "C" fn(
    *const gcm_key,
    *mut nettle_block16,
    size_t,
    *const uint8_t,
) -> *const uint8_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gcm_key {
    pub h: [nettle_block16; 256],
}
pub type ghash_set_key_func = unsafe extern "C" fn(
    *mut gcm_key,
    *const nettle_block16,
) -> ();
pub type sha256_compress_n_func = unsafe extern "C" fn(
    *mut uint32_t,
    *const uint32_t,
    size_t,
    *const uint8_t,
) -> *const uint8_t;
pub type sha1_compress_func = unsafe extern "C" fn(*mut uint32_t, *const uint8_t) -> ();
pub type cbc_aes256_encrypt_func = unsafe extern "C" fn(
    *const aes256_ctx,
    *mut uint8_t,
    size_t,
    *mut uint8_t,
    *const uint8_t,
) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aes256_ctx {
    pub keys: [uint32_t; 60],
}
pub type cbc_aes192_encrypt_func = unsafe extern "C" fn(
    *const aes192_ctx,
    *mut uint8_t,
    size_t,
    *mut uint8_t,
    *const uint8_t,
) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aes192_ctx {
    pub keys: [uint32_t; 52],
}
pub type cbc_aes128_encrypt_func = unsafe extern "C" fn(
    *const aes128_ctx,
    *mut uint8_t,
    size_t,
    *mut uint8_t,
    *const uint8_t,
) -> ();
pub type aes256_crypt_func = unsafe extern "C" fn(
    *const aes256_ctx,
    size_t,
    *mut uint8_t,
    *const uint8_t,
) -> ();
pub type aes192_crypt_func = unsafe extern "C" fn(
    *const aes192_ctx,
    size_t,
    *mut uint8_t,
    *const uint8_t,
) -> ();
unsafe extern "C" fn get_x86_features(mut features: *mut x86_features) {
    let mut s: *const i8 = 0 as *const i8;
    (*features).vendor = X86_OTHER;
    (*features).have_aesni = 0 as i32;
    (*features).have_sha_ni = 0 as i32;
    (*features).have_pclmul = 0 as i32;
    s = secure_getenv(b"NETTLE_FAT_OVERRIDE\0" as *const u8 as *const i8);
    if !s.is_null() {
        loop {
            let mut sep: *const i8 = strchr(s, ',' as i32);
            let mut length: size_t = if !sep.is_null() {
                sep.offset_from(s) as i64 as size_t
            } else {
                strlen(s)
            };
            if if length >= 7 as i32 as u64
                && memcmp(
                    s as *const libc::c_void,
                    b"vendor:\0" as *const u8 as *const i8 as *const libc::c_void,
                    7 as i32 as u64,
                ) == 0 as i32
            {
                length = (length as u64).wrapping_sub(7 as i32 as u64) as size_t
                    as size_t;
                s = s.offset(7 as i32 as isize);
                1 as i32
            } else {
                0 as i32
            } != 0
            {
                if length == 5 as i32 as u64
                    && memcmp(
                        s as *const libc::c_void,
                        b"intel\0" as *const u8 as *const i8 as *const libc::c_void,
                        5 as i32 as u64,
                    ) == 0 as i32
                {
                    (*features).vendor = X86_INTEL;
                } else if length == 3 as i32 as u64
                    && memcmp(
                        s as *const libc::c_void,
                        b"amd\0" as *const u8 as *const i8 as *const libc::c_void,
                        3 as i32 as u64,
                    ) == 0 as i32
                {
                    (*features).vendor = X86_AMD;
                }
            } else if length == 5 as i32 as u64
                && memcmp(
                    s as *const libc::c_void,
                    b"aesni\0" as *const u8 as *const i8 as *const libc::c_void,
                    5 as i32 as u64,
                ) == 0 as i32
            {
                (*features).have_aesni = 1 as i32;
            } else if length == 6 as i32 as u64
                && memcmp(
                    s as *const libc::c_void,
                    b"sha_ni\0" as *const u8 as *const i8 as *const libc::c_void,
                    6 as i32 as u64,
                ) == 0 as i32
            {
                (*features).have_sha_ni = 1 as i32;
            } else if length == 6 as i32 as u64
                && memcmp(
                    s as *const libc::c_void,
                    b"pclmul\0" as *const u8 as *const i8 as *const libc::c_void,
                    6 as i32 as u64,
                ) == 0 as i32
            {
                (*features).have_pclmul = 1 as i32;
            }
            if sep.is_null() {
                break;
            }
            s = sep.offset(1 as i32 as isize);
        }
    } else {
        let mut cpuid_data: [uint32_t; 4] = [0; 4];
        _nettle_cpuid(0 as i32 as uint32_t, cpuid_data.as_mut_ptr());
        if memcmp(
            cpuid_data.as_mut_ptr().offset(1 as i32 as isize) as *const libc::c_void,
            b"GenuntelineI\0" as *const u8 as *const i8 as *const libc::c_void,
            12 as i32 as u64,
        ) == 0 as i32
        {
            (*features).vendor = X86_INTEL;
        } else if memcmp(
            cpuid_data.as_mut_ptr().offset(1 as i32 as isize) as *const libc::c_void,
            b"AuthcAMDenti\0" as *const u8 as *const i8 as *const libc::c_void,
            12 as i32 as u64,
        ) == 0 as i32
        {
            (*features).vendor = X86_AMD;
        }
        _nettle_cpuid(1 as i32 as uint32_t, cpuid_data.as_mut_ptr());
        if cpuid_data[2 as i32 as usize] & 0x2 as i32 as u32 != 0 {
            (*features).have_pclmul = 1 as i32;
        }
        if cpuid_data[2 as i32 as usize] & 0x2000000 as i32 as u32 != 0 {
            (*features).have_aesni = 1 as i32;
        }
        _nettle_cpuid(7 as i32 as uint32_t, cpuid_data.as_mut_ptr());
        if cpuid_data[1 as i32 as usize] & 0x20000000 as i32 as u32 != 0 {
            (*features).have_sha_ni = 1 as i32;
        }
    };
}
static mut nettle_aes128_encrypt_vec: Option<aes128_crypt_func> = unsafe {
    Some(nettle_aes128_encrypt_init as aes128_crypt_func)
};
static mut nettle_aes128_decrypt_vec: Option<aes128_crypt_func> = unsafe {
    Some(nettle_aes128_decrypt_init as aes128_crypt_func)
};
static mut nettle_aes192_encrypt_vec: Option<aes192_crypt_func> = unsafe {
    Some(nettle_aes192_encrypt_init as aes192_crypt_func)
};
static mut nettle_aes192_decrypt_vec: Option<aes192_crypt_func> = unsafe {
    Some(nettle_aes192_decrypt_init as aes192_crypt_func)
};
static mut nettle_aes256_encrypt_vec: Option<aes256_crypt_func> = unsafe {
    Some(nettle_aes256_encrypt_init as aes256_crypt_func)
};
static mut nettle_aes256_decrypt_vec: Option<aes256_crypt_func> = unsafe {
    Some(nettle_aes256_decrypt_init as aes256_crypt_func)
};
static mut nettle_cbc_aes128_encrypt_vec: Option<cbc_aes128_encrypt_func> = unsafe {
    Some(nettle_cbc_aes128_encrypt_init as cbc_aes128_encrypt_func)
};
static mut nettle_cbc_aes192_encrypt_vec: Option<cbc_aes192_encrypt_func> = unsafe {
    Some(nettle_cbc_aes192_encrypt_init as cbc_aes192_encrypt_func)
};
static mut nettle_cbc_aes256_encrypt_vec: Option<cbc_aes256_encrypt_func> = unsafe {
    Some(nettle_cbc_aes256_encrypt_init as cbc_aes256_encrypt_func)
};
static mut nettle_memxor_vec: Option<memxor_func> = unsafe {
    Some(nettle_memxor_init as memxor_func)
};
static mut nettle_sha1_compress_vec: Option<sha1_compress_func> = unsafe {
    Some(nettle_sha1_compress_init as sha1_compress_func)
};
static mut _nettle_sha256_compress_n_vec: Option<sha256_compress_n_func> = unsafe {
    Some(_nettle_sha256_compress_n_init as sha256_compress_n_func)
};
static mut _nettle_ghash_set_key_vec: Option<ghash_set_key_func> = unsafe {
    Some(_nettle_ghash_set_key_init as ghash_set_key_func)
};
static mut _nettle_ghash_update_vec: Option<ghash_update_func> = unsafe {
    Some(_nettle_ghash_update_init as ghash_update_func)
};
unsafe extern "C" fn fat_init() {
    let mut features: x86_features = x86_features {
        vendor: X86_OTHER,
        have_aesni: 0,
        have_sha_ni: 0,
        have_pclmul: 0,
    };
    let mut verbose: i32 = 0;
    verbose = (getenv(b"NETTLE_FAT_VERBOSE\0" as *const u8 as *const i8)
        != 0 as *mut libc::c_void as *mut i8) as i32;
    if verbose != 0 {
        fprintf(
            stderr,
            b"libnettle: fat library initialization.\n\0" as *const u8 as *const i8,
        );
    }
    get_x86_features(&mut features);
    if verbose != 0 {
        let vendor_names: [*const i8; 3] = [
            b"other\0" as *const u8 as *const i8,
            b"intel\0" as *const u8 as *const i8,
            b"amd\0" as *const u8 as *const i8,
        ];
        fprintf(
            stderr,
            b"libnettle: cpu features: vendor:%s%s%s%s\n\0" as *const u8 as *const i8,
            vendor_names[features.vendor as usize],
            if features.have_aesni != 0 {
                b",aesni\0" as *const u8 as *const i8
            } else {
                b"\0" as *const u8 as *const i8
            },
            if features.have_sha_ni != 0 {
                b",sha_ni\0" as *const u8 as *const i8
            } else {
                b"\0" as *const u8 as *const i8
            },
            if features.have_pclmul != 0 {
                b",pclmul\0" as *const u8 as *const i8
            } else {
                b"\0" as *const u8 as *const i8
            },
        );
    }
    if features.have_aesni != 0 {
        if verbose != 0 {
            fprintf(
                stderr,
                b"libnettle: using aes instructions.\n\0" as *const u8 as *const i8,
            );
        }
        nettle_aes128_encrypt_vec = Some(
            _nettle_aes128_encrypt_aesni as aes128_crypt_func,
        );
        nettle_aes128_decrypt_vec = Some(
            _nettle_aes128_decrypt_aesni as aes128_crypt_func,
        );
        nettle_aes192_encrypt_vec = Some(
            _nettle_aes192_encrypt_aesni as aes192_crypt_func,
        );
        nettle_aes192_decrypt_vec = Some(
            _nettle_aes192_decrypt_aesni as aes192_crypt_func,
        );
        nettle_aes256_encrypt_vec = Some(
            _nettle_aes256_encrypt_aesni as aes256_crypt_func,
        );
        nettle_aes256_decrypt_vec = Some(
            _nettle_aes256_decrypt_aesni as aes256_crypt_func,
        );
        nettle_cbc_aes128_encrypt_vec = Some(
            _nettle_cbc_aes128_encrypt_aesni as cbc_aes128_encrypt_func,
        );
        nettle_cbc_aes192_encrypt_vec = Some(
            _nettle_cbc_aes192_encrypt_aesni as cbc_aes192_encrypt_func,
        );
        nettle_cbc_aes256_encrypt_vec = Some(
            _nettle_cbc_aes256_encrypt_aesni as cbc_aes256_encrypt_func,
        );
    } else {
        if verbose != 0 {
            fprintf(
                stderr,
                b"libnettle: not using aes instructions.\n\0" as *const u8 as *const i8,
            );
        }
        nettle_aes128_encrypt_vec = Some(_nettle_aes128_encrypt_c as aes128_crypt_func);
        nettle_aes128_decrypt_vec = Some(_nettle_aes128_decrypt_c as aes128_crypt_func);
        nettle_aes192_encrypt_vec = Some(_nettle_aes192_encrypt_c as aes192_crypt_func);
        nettle_aes192_decrypt_vec = Some(_nettle_aes192_decrypt_c as aes192_crypt_func);
        nettle_aes256_encrypt_vec = Some(_nettle_aes256_encrypt_c as aes256_crypt_func);
        nettle_aes256_decrypt_vec = Some(_nettle_aes256_decrypt_c as aes256_crypt_func);
        nettle_cbc_aes128_encrypt_vec = Some(
            _nettle_cbc_aes128_encrypt_c as cbc_aes128_encrypt_func,
        );
        nettle_cbc_aes192_encrypt_vec = Some(
            _nettle_cbc_aes192_encrypt_c as cbc_aes192_encrypt_func,
        );
        nettle_cbc_aes256_encrypt_vec = Some(
            _nettle_cbc_aes256_encrypt_c as cbc_aes256_encrypt_func,
        );
    }
    if features.have_sha_ni != 0 {
        if verbose != 0 {
            fprintf(
                stderr,
                b"libnettle: using sha_ni instructions.\n\0" as *const u8 as *const i8,
            );
        }
        nettle_sha1_compress_vec = Some(
            _nettle_sha1_compress_sha_ni as sha1_compress_func,
        );
        _nettle_sha256_compress_n_vec = Some(
            _nettle_sha256_compress_n_sha_ni as sha256_compress_n_func,
        );
    } else {
        if verbose != 0 {
            fprintf(
                stderr,
                b"libnettle: not using sha_ni instructions.\n\0" as *const u8
                    as *const i8,
            );
        }
        nettle_sha1_compress_vec = Some(
            _nettle_sha1_compress_x86_64 as sha1_compress_func,
        );
        _nettle_sha256_compress_n_vec = Some(
            _nettle_sha256_compress_n_x86_64 as sha256_compress_n_func,
        );
    }
    if features.have_pclmul != 0 {
        if verbose != 0 {
            fprintf(
                stderr,
                b"libnettle: using pclmulqdq instructions.\n\0" as *const u8 as *const i8,
            );
        }
        _nettle_ghash_set_key_vec = Some(
            _nettle_ghash_set_key_pclmul as ghash_set_key_func,
        );
        _nettle_ghash_update_vec = Some(
            _nettle_ghash_update_pclmul as ghash_update_func,
        );
    } else {
        if verbose != 0 {
            fprintf(
                stderr,
                b"libnettle: not using pclmulqdq instructions.\n\0" as *const u8
                    as *const i8,
            );
        }
        _nettle_ghash_set_key_vec = Some(_nettle_ghash_set_key_c as ghash_set_key_func);
        _nettle_ghash_update_vec = Some(_nettle_ghash_update_table as ghash_update_func);
    }
    if features.vendor as u32 == X86_INTEL as i32 as u32 {
        if verbose != 0 {
            fprintf(
                stderr,
                b"libnettle: intel SSE2 will be used for memxor.\n\0" as *const u8
                    as *const i8,
            );
        }
        nettle_memxor_vec = Some(_nettle_memxor_sse2 as memxor_func);
    } else {
        if verbose != 0 {
            fprintf(
                stderr,
                b"libnettle: intel SSE2 will not be used for memxor.\n\0" as *const u8
                    as *const i8,
            );
        }
        nettle_memxor_vec = Some(_nettle_memxor_x86_64 as memxor_func);
    };
}
unsafe extern "C" fn nettle_aes128_encrypt_init(
    mut ctx: *const aes128_ctx,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    if !(getenv(b"NETTLE_FAT_VERBOSE\0" as *const u8 as *const i8)).is_null() {
        fprintf(
            stderr,
            b"libnettle: nettle_aes128_encrypt_init\n\0" as *const u8 as *const i8,
        );
    }
    if nettle_aes128_encrypt_vec == Some(nettle_aes128_encrypt_init as aes128_crypt_func)
    {
        fat_init();
    }
    if nettle_aes128_encrypt_vec != Some(nettle_aes128_encrypt_init as aes128_crypt_func)
    {} else {
        __assert_fail(
            b"nettle_aes128_encrypt_vec != nettle_aes128_encrypt_init\0" as *const u8
                as *const i8,
            b"fat-x86_64.c\0" as *const u8 as *const i8,
            273 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 95],
                &[i8; 95],
            >(
                b"void nettle_aes128_encrypt_init(const struct aes128_ctx *, size_t, uint8_t *, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_2689: {
        if nettle_aes128_encrypt_vec
            != Some(nettle_aes128_encrypt_init as aes128_crypt_func)
        {} else {
            __assert_fail(
                b"nettle_aes128_encrypt_vec != nettle_aes128_encrypt_init\0" as *const u8
                    as *const i8,
                b"fat-x86_64.c\0" as *const u8 as *const i8,
                273 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 95],
                    &[i8; 95],
                >(
                    b"void nettle_aes128_encrypt_init(const struct aes128_ctx *, size_t, uint8_t *, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    return nettle_aes128_encrypt_vec
        .expect("non-null function pointer")(ctx, length, dst, src);
}
#[no_mangle]
pub unsafe extern "C" fn nettle_aes128_encrypt(
    mut ctx: *const aes128_ctx,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    return nettle_aes128_encrypt_vec
        .expect("non-null function pointer")(ctx, length, dst, src);
}
unsafe extern "C" fn nettle_aes128_decrypt_init(
    mut ctx: *const aes128_ctx,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    if !(getenv(b"NETTLE_FAT_VERBOSE\0" as *const u8 as *const i8)).is_null() {
        fprintf(
            stderr,
            b"libnettle: nettle_aes128_decrypt_init\n\0" as *const u8 as *const i8,
        );
    }
    if nettle_aes128_decrypt_vec == Some(nettle_aes128_decrypt_init as aes128_crypt_func)
    {
        fat_init();
    }
    if nettle_aes128_decrypt_vec != Some(nettle_aes128_decrypt_init as aes128_crypt_func)
    {} else {
        __assert_fail(
            b"nettle_aes128_decrypt_vec != nettle_aes128_decrypt_init\0" as *const u8
                as *const i8,
            b"fat-x86_64.c\0" as *const u8 as *const i8,
            277 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 95],
                &[i8; 95],
            >(
                b"void nettle_aes128_decrypt_init(const struct aes128_ctx *, size_t, uint8_t *, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_4142: {
        if nettle_aes128_decrypt_vec
            != Some(nettle_aes128_decrypt_init as aes128_crypt_func)
        {} else {
            __assert_fail(
                b"nettle_aes128_decrypt_vec != nettle_aes128_decrypt_init\0" as *const u8
                    as *const i8,
                b"fat-x86_64.c\0" as *const u8 as *const i8,
                277 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 95],
                    &[i8; 95],
                >(
                    b"void nettle_aes128_decrypt_init(const struct aes128_ctx *, size_t, uint8_t *, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    return nettle_aes128_decrypt_vec
        .expect("non-null function pointer")(ctx, length, dst, src);
}
#[no_mangle]
pub unsafe extern "C" fn nettle_aes128_decrypt(
    mut ctx: *const aes128_ctx,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    return nettle_aes128_decrypt_vec
        .expect("non-null function pointer")(ctx, length, dst, src);
}
unsafe extern "C" fn nettle_aes192_encrypt_init(
    mut ctx: *const aes192_ctx,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    if !(getenv(b"NETTLE_FAT_VERBOSE\0" as *const u8 as *const i8)).is_null() {
        fprintf(
            stderr,
            b"libnettle: nettle_aes192_encrypt_init\n\0" as *const u8 as *const i8,
        );
    }
    if nettle_aes192_encrypt_vec == Some(nettle_aes192_encrypt_init as aes192_crypt_func)
    {
        fat_init();
    }
    if nettle_aes192_encrypt_vec != Some(nettle_aes192_encrypt_init as aes192_crypt_func)
    {} else {
        __assert_fail(
            b"nettle_aes192_encrypt_vec != nettle_aes192_encrypt_init\0" as *const u8
                as *const i8,
            b"fat-x86_64.c\0" as *const u8 as *const i8,
            282 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 95],
                &[i8; 95],
            >(
                b"void nettle_aes192_encrypt_init(const struct aes192_ctx *, size_t, uint8_t *, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_4049: {
        if nettle_aes192_encrypt_vec
            != Some(nettle_aes192_encrypt_init as aes192_crypt_func)
        {} else {
            __assert_fail(
                b"nettle_aes192_encrypt_vec != nettle_aes192_encrypt_init\0" as *const u8
                    as *const i8,
                b"fat-x86_64.c\0" as *const u8 as *const i8,
                282 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 95],
                    &[i8; 95],
                >(
                    b"void nettle_aes192_encrypt_init(const struct aes192_ctx *, size_t, uint8_t *, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    return nettle_aes192_encrypt_vec
        .expect("non-null function pointer")(ctx, length, dst, src);
}
#[no_mangle]
pub unsafe extern "C" fn nettle_aes192_encrypt(
    mut ctx: *const aes192_ctx,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    return nettle_aes192_encrypt_vec
        .expect("non-null function pointer")(ctx, length, dst, src);
}
unsafe extern "C" fn nettle_aes192_decrypt_init(
    mut ctx: *const aes192_ctx,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    if !(getenv(b"NETTLE_FAT_VERBOSE\0" as *const u8 as *const i8)).is_null() {
        fprintf(
            stderr,
            b"libnettle: nettle_aes192_decrypt_init\n\0" as *const u8 as *const i8,
        );
    }
    if nettle_aes192_decrypt_vec == Some(nettle_aes192_decrypt_init as aes192_crypt_func)
    {
        fat_init();
    }
    if nettle_aes192_decrypt_vec != Some(nettle_aes192_decrypt_init as aes192_crypt_func)
    {} else {
        __assert_fail(
            b"nettle_aes192_decrypt_vec != nettle_aes192_decrypt_init\0" as *const u8
                as *const i8,
            b"fat-x86_64.c\0" as *const u8 as *const i8,
            286 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 95],
                &[i8; 95],
            >(
                b"void nettle_aes192_decrypt_init(const struct aes192_ctx *, size_t, uint8_t *, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_3956: {
        if nettle_aes192_decrypt_vec
            != Some(nettle_aes192_decrypt_init as aes192_crypt_func)
        {} else {
            __assert_fail(
                b"nettle_aes192_decrypt_vec != nettle_aes192_decrypt_init\0" as *const u8
                    as *const i8,
                b"fat-x86_64.c\0" as *const u8 as *const i8,
                286 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 95],
                    &[i8; 95],
                >(
                    b"void nettle_aes192_decrypt_init(const struct aes192_ctx *, size_t, uint8_t *, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    return nettle_aes192_decrypt_vec
        .expect("non-null function pointer")(ctx, length, dst, src);
}
#[no_mangle]
pub unsafe extern "C" fn nettle_aes192_decrypt(
    mut ctx: *const aes192_ctx,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    return nettle_aes192_decrypt_vec
        .expect("non-null function pointer")(ctx, length, dst, src);
}
unsafe extern "C" fn nettle_aes256_encrypt_init(
    mut ctx: *const aes256_ctx,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    if !(getenv(b"NETTLE_FAT_VERBOSE\0" as *const u8 as *const i8)).is_null() {
        fprintf(
            stderr,
            b"libnettle: nettle_aes256_encrypt_init\n\0" as *const u8 as *const i8,
        );
    }
    if nettle_aes256_encrypt_vec == Some(nettle_aes256_encrypt_init as aes256_crypt_func)
    {
        fat_init();
    }
    if nettle_aes256_encrypt_vec != Some(nettle_aes256_encrypt_init as aes256_crypt_func)
    {} else {
        __assert_fail(
            b"nettle_aes256_encrypt_vec != nettle_aes256_encrypt_init\0" as *const u8
                as *const i8,
            b"fat-x86_64.c\0" as *const u8 as *const i8,
            291 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 95],
                &[i8; 95],
            >(
                b"void nettle_aes256_encrypt_init(const struct aes256_ctx *, size_t, uint8_t *, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_3859: {
        if nettle_aes256_encrypt_vec
            != Some(nettle_aes256_encrypt_init as aes256_crypt_func)
        {} else {
            __assert_fail(
                b"nettle_aes256_encrypt_vec != nettle_aes256_encrypt_init\0" as *const u8
                    as *const i8,
                b"fat-x86_64.c\0" as *const u8 as *const i8,
                291 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 95],
                    &[i8; 95],
                >(
                    b"void nettle_aes256_encrypt_init(const struct aes256_ctx *, size_t, uint8_t *, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    return nettle_aes256_encrypt_vec
        .expect("non-null function pointer")(ctx, length, dst, src);
}
#[no_mangle]
pub unsafe extern "C" fn nettle_aes256_encrypt(
    mut ctx: *const aes256_ctx,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    return nettle_aes256_encrypt_vec
        .expect("non-null function pointer")(ctx, length, dst, src);
}
unsafe extern "C" fn nettle_aes256_decrypt_init(
    mut ctx: *const aes256_ctx,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    if !(getenv(b"NETTLE_FAT_VERBOSE\0" as *const u8 as *const i8)).is_null() {
        fprintf(
            stderr,
            b"libnettle: nettle_aes256_decrypt_init\n\0" as *const u8 as *const i8,
        );
    }
    if nettle_aes256_decrypt_vec == Some(nettle_aes256_decrypt_init as aes256_crypt_func)
    {
        fat_init();
    }
    if nettle_aes256_decrypt_vec != Some(nettle_aes256_decrypt_init as aes256_crypt_func)
    {} else {
        __assert_fail(
            b"nettle_aes256_decrypt_vec != nettle_aes256_decrypt_init\0" as *const u8
                as *const i8,
            b"fat-x86_64.c\0" as *const u8 as *const i8,
            295 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 95],
                &[i8; 95],
            >(
                b"void nettle_aes256_decrypt_init(const struct aes256_ctx *, size_t, uint8_t *, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_3766: {
        if nettle_aes256_decrypt_vec
            != Some(nettle_aes256_decrypt_init as aes256_crypt_func)
        {} else {
            __assert_fail(
                b"nettle_aes256_decrypt_vec != nettle_aes256_decrypt_init\0" as *const u8
                    as *const i8,
                b"fat-x86_64.c\0" as *const u8 as *const i8,
                295 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 95],
                    &[i8; 95],
                >(
                    b"void nettle_aes256_decrypt_init(const struct aes256_ctx *, size_t, uint8_t *, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    return nettle_aes256_decrypt_vec
        .expect("non-null function pointer")(ctx, length, dst, src);
}
#[no_mangle]
pub unsafe extern "C" fn nettle_aes256_decrypt(
    mut ctx: *const aes256_ctx,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    return nettle_aes256_decrypt_vec
        .expect("non-null function pointer")(ctx, length, dst, src);
}
unsafe extern "C" fn nettle_cbc_aes128_encrypt_init(
    mut ctx: *const aes128_ctx,
    mut iv: *mut uint8_t,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    if !(getenv(b"NETTLE_FAT_VERBOSE\0" as *const u8 as *const i8)).is_null() {
        fprintf(
            stderr,
            b"libnettle: nettle_cbc_aes128_encrypt_init\n\0" as *const u8 as *const i8,
        );
    }
    if nettle_cbc_aes128_encrypt_vec
        == Some(nettle_cbc_aes128_encrypt_init as cbc_aes128_encrypt_func)
    {
        fat_init();
    }
    if nettle_cbc_aes128_encrypt_vec
        != Some(nettle_cbc_aes128_encrypt_init as cbc_aes128_encrypt_func)
    {} else {
        __assert_fail(
            b"nettle_cbc_aes128_encrypt_vec != nettle_cbc_aes128_encrypt_init\0"
                as *const u8 as *const i8,
            b"fat-x86_64.c\0" as *const u8 as *const i8,
            300 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 110],
                &[i8; 110],
            >(
                b"void nettle_cbc_aes128_encrypt_init(const struct aes128_ctx *, uint8_t *, size_t, uint8_t *, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_3669: {
        if nettle_cbc_aes128_encrypt_vec
            != Some(nettle_cbc_aes128_encrypt_init as cbc_aes128_encrypt_func)
        {} else {
            __assert_fail(
                b"nettle_cbc_aes128_encrypt_vec != nettle_cbc_aes128_encrypt_init\0"
                    as *const u8 as *const i8,
                b"fat-x86_64.c\0" as *const u8 as *const i8,
                300 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 110],
                    &[i8; 110],
                >(
                    b"void nettle_cbc_aes128_encrypt_init(const struct aes128_ctx *, uint8_t *, size_t, uint8_t *, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    return nettle_cbc_aes128_encrypt_vec
        .expect("non-null function pointer")(ctx, iv, length, dst, src);
}
#[no_mangle]
pub unsafe extern "C" fn nettle_cbc_aes128_encrypt(
    mut ctx: *const aes128_ctx,
    mut iv: *mut uint8_t,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    return nettle_cbc_aes128_encrypt_vec
        .expect("non-null function pointer")(ctx, iv, length, dst, src);
}
unsafe extern "C" fn nettle_cbc_aes192_encrypt_init(
    mut ctx: *const aes192_ctx,
    mut iv: *mut uint8_t,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    if !(getenv(b"NETTLE_FAT_VERBOSE\0" as *const u8 as *const i8)).is_null() {
        fprintf(
            stderr,
            b"libnettle: nettle_cbc_aes192_encrypt_init\n\0" as *const u8 as *const i8,
        );
    }
    if nettle_cbc_aes192_encrypt_vec
        == Some(nettle_cbc_aes192_encrypt_init as cbc_aes192_encrypt_func)
    {
        fat_init();
    }
    if nettle_cbc_aes192_encrypt_vec
        != Some(nettle_cbc_aes192_encrypt_init as cbc_aes192_encrypt_func)
    {} else {
        __assert_fail(
            b"nettle_cbc_aes192_encrypt_vec != nettle_cbc_aes192_encrypt_init\0"
                as *const u8 as *const i8,
            b"fat-x86_64.c\0" as *const u8 as *const i8,
            304 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 110],
                &[i8; 110],
            >(
                b"void nettle_cbc_aes192_encrypt_init(const struct aes192_ctx *, uint8_t *, size_t, uint8_t *, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_3568: {
        if nettle_cbc_aes192_encrypt_vec
            != Some(nettle_cbc_aes192_encrypt_init as cbc_aes192_encrypt_func)
        {} else {
            __assert_fail(
                b"nettle_cbc_aes192_encrypt_vec != nettle_cbc_aes192_encrypt_init\0"
                    as *const u8 as *const i8,
                b"fat-x86_64.c\0" as *const u8 as *const i8,
                304 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 110],
                    &[i8; 110],
                >(
                    b"void nettle_cbc_aes192_encrypt_init(const struct aes192_ctx *, uint8_t *, size_t, uint8_t *, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    return nettle_cbc_aes192_encrypt_vec
        .expect("non-null function pointer")(ctx, iv, length, dst, src);
}
#[no_mangle]
pub unsafe extern "C" fn nettle_cbc_aes192_encrypt(
    mut ctx: *const aes192_ctx,
    mut iv: *mut uint8_t,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    return nettle_cbc_aes192_encrypt_vec
        .expect("non-null function pointer")(ctx, iv, length, dst, src);
}
unsafe extern "C" fn nettle_cbc_aes256_encrypt_init(
    mut ctx: *const aes256_ctx,
    mut iv: *mut uint8_t,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    if !(getenv(b"NETTLE_FAT_VERBOSE\0" as *const u8 as *const i8)).is_null() {
        fprintf(
            stderr,
            b"libnettle: nettle_cbc_aes256_encrypt_init\n\0" as *const u8 as *const i8,
        );
    }
    if nettle_cbc_aes256_encrypt_vec
        == Some(nettle_cbc_aes256_encrypt_init as cbc_aes256_encrypt_func)
    {
        fat_init();
    }
    if nettle_cbc_aes256_encrypt_vec
        != Some(nettle_cbc_aes256_encrypt_init as cbc_aes256_encrypt_func)
    {} else {
        __assert_fail(
            b"nettle_cbc_aes256_encrypt_vec != nettle_cbc_aes256_encrypt_init\0"
                as *const u8 as *const i8,
            b"fat-x86_64.c\0" as *const u8 as *const i8,
            308 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 110],
                &[i8; 110],
            >(
                b"void nettle_cbc_aes256_encrypt_init(const struct aes256_ctx *, uint8_t *, size_t, uint8_t *, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_3460: {
        if nettle_cbc_aes256_encrypt_vec
            != Some(nettle_cbc_aes256_encrypt_init as cbc_aes256_encrypt_func)
        {} else {
            __assert_fail(
                b"nettle_cbc_aes256_encrypt_vec != nettle_cbc_aes256_encrypt_init\0"
                    as *const u8 as *const i8,
                b"fat-x86_64.c\0" as *const u8 as *const i8,
                308 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 110],
                    &[i8; 110],
                >(
                    b"void nettle_cbc_aes256_encrypt_init(const struct aes256_ctx *, uint8_t *, size_t, uint8_t *, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    return nettle_cbc_aes256_encrypt_vec
        .expect("non-null function pointer")(ctx, iv, length, dst, src);
}
#[no_mangle]
pub unsafe extern "C" fn nettle_cbc_aes256_encrypt(
    mut ctx: *const aes256_ctx,
    mut iv: *mut uint8_t,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    return nettle_cbc_aes256_encrypt_vec
        .expect("non-null function pointer")(ctx, iv, length, dst, src);
}
unsafe extern "C" fn nettle_memxor_init(
    mut dst: *mut libc::c_void,
    mut src: *const libc::c_void,
    mut n: size_t,
) -> *mut libc::c_void {
    if !(getenv(b"NETTLE_FAT_VERBOSE\0" as *const u8 as *const i8)).is_null() {
        fprintf(stderr, b"libnettle: nettle_memxor_init\n\0" as *const u8 as *const i8);
    }
    if nettle_memxor_vec == Some(nettle_memxor_init as memxor_func) {
        fat_init();
    }
    if nettle_memxor_vec != Some(nettle_memxor_init as memxor_func) {} else {
        __assert_fail(
            b"nettle_memxor_vec != nettle_memxor_init\0" as *const u8 as *const i8,
            b"fat-x86_64.c\0" as *const u8 as *const i8,
            312 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 55],
                &[i8; 55],
            >(b"void *nettle_memxor_init(void *, const void *, size_t)\0"))
                .as_ptr(),
        );
    }
    'c_2789: {
        if nettle_memxor_vec != Some(nettle_memxor_init as memxor_func) {} else {
            __assert_fail(
                b"nettle_memxor_vec != nettle_memxor_init\0" as *const u8 as *const i8,
                b"fat-x86_64.c\0" as *const u8 as *const i8,
                312 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 55],
                    &[i8; 55],
                >(b"void *nettle_memxor_init(void *, const void *, size_t)\0"))
                    .as_ptr(),
            );
        }
    };
    return nettle_memxor_vec.expect("non-null function pointer")(dst, src, n);
}
#[no_mangle]
pub unsafe extern "C" fn nettle_memxor(
    mut dst: *mut libc::c_void,
    mut src: *const libc::c_void,
    mut n: size_t,
) -> *mut libc::c_void {
    return nettle_memxor_vec.expect("non-null function pointer")(dst, src, n);
}
unsafe extern "C" fn nettle_sha1_compress_init(
    mut state: *mut uint32_t,
    mut input: *const uint8_t,
) {
    if !(getenv(b"NETTLE_FAT_VERBOSE\0" as *const u8 as *const i8)).is_null() {
        fprintf(
            stderr,
            b"libnettle: nettle_sha1_compress_init\n\0" as *const u8 as *const i8,
        );
    }
    if nettle_sha1_compress_vec == Some(nettle_sha1_compress_init as sha1_compress_func)
    {
        fat_init();
    }
    if nettle_sha1_compress_vec != Some(nettle_sha1_compress_init as sha1_compress_func)
    {} else {
        __assert_fail(
            b"nettle_sha1_compress_vec != nettle_sha1_compress_init\0" as *const u8
                as *const i8,
            b"fat-x86_64.c\0" as *const u8 as *const i8,
            316 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 60],
                &[i8; 60],
            >(b"void nettle_sha1_compress_init(uint32_t *, const uint8_t *)\0"))
                .as_ptr(),
        );
    }
    'c_3298: {
        if nettle_sha1_compress_vec
            != Some(nettle_sha1_compress_init as sha1_compress_func)
        {} else {
            __assert_fail(
                b"nettle_sha1_compress_vec != nettle_sha1_compress_init\0" as *const u8
                    as *const i8,
                b"fat-x86_64.c\0" as *const u8 as *const i8,
                316 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 60],
                    &[i8; 60],
                >(b"void nettle_sha1_compress_init(uint32_t *, const uint8_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    return nettle_sha1_compress_vec.expect("non-null function pointer")(state, input);
}
#[no_mangle]
pub unsafe extern "C" fn nettle_sha1_compress(
    mut state: *mut uint32_t,
    mut input: *const uint8_t,
) {
    return nettle_sha1_compress_vec.expect("non-null function pointer")(state, input);
}
unsafe extern "C" fn _nettle_sha256_compress_n_init(
    mut state: *mut uint32_t,
    mut k: *const uint32_t,
    mut blocks: size_t,
    mut input: *const uint8_t,
) -> *const uint8_t {
    if !(getenv(b"NETTLE_FAT_VERBOSE\0" as *const u8 as *const i8)).is_null() {
        fprintf(
            stderr,
            b"libnettle: _nettle_sha256_compress_n_init\n\0" as *const u8 as *const i8,
        );
    }
    if _nettle_sha256_compress_n_vec
        == Some(_nettle_sha256_compress_n_init as sha256_compress_n_func)
    {
        fat_init();
    }
    if _nettle_sha256_compress_n_vec
        != Some(_nettle_sha256_compress_n_init as sha256_compress_n_func)
    {} else {
        __assert_fail(
            b"_nettle_sha256_compress_n_vec != _nettle_sha256_compress_n_init\0"
                as *const u8 as *const i8,
            b"fat-x86_64.c\0" as *const u8 as *const i8,
            321 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 101],
                &[i8; 101],
            >(
                b"const uint8_t *_nettle_sha256_compress_n_init(uint32_t *, const uint32_t *, size_t, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_3206: {
        if _nettle_sha256_compress_n_vec
            != Some(_nettle_sha256_compress_n_init as sha256_compress_n_func)
        {} else {
            __assert_fail(
                b"_nettle_sha256_compress_n_vec != _nettle_sha256_compress_n_init\0"
                    as *const u8 as *const i8,
                b"fat-x86_64.c\0" as *const u8 as *const i8,
                321 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 101],
                    &[i8; 101],
                >(
                    b"const uint8_t *_nettle_sha256_compress_n_init(uint32_t *, const uint32_t *, size_t, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    return _nettle_sha256_compress_n_vec
        .expect("non-null function pointer")(state, k, blocks, input);
}
#[no_mangle]
pub unsafe extern "C" fn _nettle_sha256_compress_n(
    mut state: *mut uint32_t,
    mut k: *const uint32_t,
    mut blocks: size_t,
    mut input: *const uint8_t,
) -> *const uint8_t {
    return _nettle_sha256_compress_n_vec
        .expect("non-null function pointer")(state, k, blocks, input);
}
unsafe extern "C" fn _nettle_ghash_set_key_init(
    mut ctx: *mut gcm_key,
    mut key: *const nettle_block16,
) {
    if !(getenv(b"NETTLE_FAT_VERBOSE\0" as *const u8 as *const i8)).is_null() {
        fprintf(
            stderr,
            b"libnettle: _nettle_ghash_set_key_init\n\0" as *const u8 as *const i8,
        );
    }
    if _nettle_ghash_set_key_vec
        == Some(_nettle_ghash_set_key_init as ghash_set_key_func)
    {
        fat_init();
    }
    if _nettle_ghash_set_key_vec
        != Some(_nettle_ghash_set_key_init as ghash_set_key_func)
    {} else {
        __assert_fail(
            b"_nettle_ghash_set_key_vec != _nettle_ghash_set_key_init\0" as *const u8
                as *const i8,
            b"fat-x86_64.c\0" as *const u8 as *const i8,
            325 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 80],
                &[i8; 80],
            >(
                b"void _nettle_ghash_set_key_init(struct gcm_key *, const union nettle_block16 *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_3056: {
        if _nettle_ghash_set_key_vec
            != Some(_nettle_ghash_set_key_init as ghash_set_key_func)
        {} else {
            __assert_fail(
                b"_nettle_ghash_set_key_vec != _nettle_ghash_set_key_init\0" as *const u8
                    as *const i8,
                b"fat-x86_64.c\0" as *const u8 as *const i8,
                325 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 80],
                    &[i8; 80],
                >(
                    b"void _nettle_ghash_set_key_init(struct gcm_key *, const union nettle_block16 *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    return _nettle_ghash_set_key_vec.expect("non-null function pointer")(ctx, key);
}
#[no_mangle]
pub unsafe extern "C" fn _nettle_ghash_set_key(
    mut ctx: *mut gcm_key,
    mut key: *const nettle_block16,
) {
    return _nettle_ghash_set_key_vec.expect("non-null function pointer")(ctx, key);
}
unsafe extern "C" fn _nettle_ghash_update_init(
    mut ctx: *const gcm_key,
    mut state: *mut nettle_block16,
    mut blocks: size_t,
    mut data: *const uint8_t,
) -> *const uint8_t {
    if !(getenv(b"NETTLE_FAT_VERBOSE\0" as *const u8 as *const i8)).is_null() {
        fprintf(
            stderr,
            b"libnettle: _nettle_ghash_update_init\n\0" as *const u8 as *const i8,
        );
    }
    if _nettle_ghash_update_vec == Some(_nettle_ghash_update_init as ghash_update_func) {
        fat_init();
    }
    if _nettle_ghash_update_vec != Some(_nettle_ghash_update_init as ghash_update_func)
    {} else {
        __assert_fail(
            b"_nettle_ghash_update_vec != _nettle_ghash_update_init\0" as *const u8
                as *const i8,
            b"fat-x86_64.c\0" as *const u8 as *const i8,
            329 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 114],
                &[i8; 114],
            >(
                b"const uint8_t *_nettle_ghash_update_init(const struct gcm_key *, union nettle_block16 *, size_t, const uint8_t *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_2962: {
        if _nettle_ghash_update_vec
            != Some(_nettle_ghash_update_init as ghash_update_func)
        {} else {
            __assert_fail(
                b"_nettle_ghash_update_vec != _nettle_ghash_update_init\0" as *const u8
                    as *const i8,
                b"fat-x86_64.c\0" as *const u8 as *const i8,
                329 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 114],
                    &[i8; 114],
                >(
                    b"const uint8_t *_nettle_ghash_update_init(const struct gcm_key *, union nettle_block16 *, size_t, const uint8_t *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    return _nettle_ghash_update_vec
        .expect("non-null function pointer")(ctx, state, blocks, data);
}
#[no_mangle]
pub unsafe extern "C" fn _nettle_ghash_update(
    mut ctx: *const gcm_key,
    mut state: *mut nettle_block16,
    mut blocks: size_t,
    mut data: *const uint8_t,
) -> *const uint8_t {
    return _nettle_ghash_update_vec
        .expect("non-null function pointer")(ctx, state, blocks, data);
}