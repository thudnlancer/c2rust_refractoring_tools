#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(c_variadic, label_break_value)]
extern "C" {
    static mut stdout: *mut _IO_FILE;
    static mut stderr: *mut _IO_FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn printf(_: *const i8, _: ...) -> i32;
    fn vfprintf(_: *mut FILE, _: *const i8, _: ::core::ffi::VaList) -> i32;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: u64) -> *mut libc::c_void;
    fn fwrite(
        __ptr: *const libc::c_void,
        __size: size_t,
        __n: size_t,
        __s: *mut FILE,
    ) -> size_t;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: u64) -> i32;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn free(__ptr: *mut libc::c_void);
    fn strlen(_: *const i8) -> u64;
    fn test_main();
    fn abort() -> !;
    fn exit(_: i32) -> !;
    fn getenv(__name: *const i8) -> *mut i8;
    fn nettle_base16_decode_init(ctx: *mut base16_decode_ctx);
    fn nettle_base16_decode_update(
        ctx: *mut base16_decode_ctx,
        dst_length: *mut size_t,
        dst: *mut uint8_t,
        src_length: size_t,
        src: *const i8,
    ) -> i32;
    fn nettle_base16_decode_final(ctx: *mut base16_decode_ctx) -> i32;
    fn nettle_cbc_encrypt(
        ctx: *const libc::c_void,
        f: Option<nettle_cipher_func>,
        block_size: size_t,
        iv: *mut uint8_t,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_cbc_decrypt(
        ctx: *const libc::c_void,
        f: Option<nettle_cipher_func>,
        block_size: size_t,
        iv: *mut uint8_t,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_cfb_encrypt(
        ctx: *const libc::c_void,
        f: Option<nettle_cipher_func>,
        block_size: size_t,
        iv: *mut uint8_t,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_cfb_decrypt(
        ctx: *const libc::c_void,
        f: Option<nettle_cipher_func>,
        block_size: size_t,
        iv: *mut uint8_t,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_cfb8_encrypt(
        ctx: *const libc::c_void,
        f: Option<nettle_cipher_func>,
        block_size: size_t,
        iv: *mut uint8_t,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_cfb8_decrypt(
        ctx: *const libc::c_void,
        f: Option<nettle_cipher_func>,
        block_size: size_t,
        iv: *mut uint8_t,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_ctr_crypt(
        ctx: *const libc::c_void,
        f: Option<nettle_cipher_func>,
        block_size: size_t,
        ctr: *mut uint8_t,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn __assert_fail(
        __assertion: *const i8,
        __file: *const i8,
        __line: u32,
        __function: *const i8,
    ) -> !;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: u32,
    pub fp_offset: u32,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type size_t = u64;
pub type va_list = __builtin_va_list;
pub type __uint8_t = u8;
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
pub type ptrdiff_t = i64;
pub type uint8_t = __uint8_t;
pub type nettle_set_key_func = unsafe extern "C" fn(
    *mut libc::c_void,
    *const uint8_t,
) -> ();
pub type nettle_cipher_func = unsafe extern "C" fn(
    *const libc::c_void,
    size_t,
    *mut uint8_t,
    *const uint8_t,
) -> ();
pub type nettle_crypt_func = unsafe extern "C" fn(
    *mut libc::c_void,
    size_t,
    *mut uint8_t,
    *const uint8_t,
) -> ();
pub type nettle_hash_init_func = unsafe extern "C" fn(*mut libc::c_void) -> ();
pub type nettle_hash_update_func = unsafe extern "C" fn(
    *mut libc::c_void,
    size_t,
    *const uint8_t,
) -> ();
pub type nettle_hash_digest_func = unsafe extern "C" fn(
    *mut libc::c_void,
    size_t,
    *mut uint8_t,
) -> ();
pub type nettle_armor_length_func = unsafe extern "C" fn(size_t) -> size_t;
pub type nettle_armor_init_func = unsafe extern "C" fn(*mut libc::c_void) -> ();
pub type nettle_armor_encode_update_func = unsafe extern "C" fn(
    *mut libc::c_void,
    *mut i8,
    size_t,
    *const uint8_t,
) -> size_t;
pub type nettle_armor_encode_final_func = unsafe extern "C" fn(
    *mut libc::c_void,
    *mut i8,
) -> size_t;
pub type nettle_armor_decode_update_func = unsafe extern "C" fn(
    *mut libc::c_void,
    *mut size_t,
    *mut uint8_t,
    size_t,
    *const i8,
) -> i32;
pub type nettle_armor_decode_final_func = unsafe extern "C" fn(*mut libc::c_void) -> i32;
pub type mp_limb_t = u64;
pub type mp_size_t = i64;
pub type mp_bitcnt_t = u64;
pub type mp_ptr = *mut mp_limb_t;
pub type mp_srcptr = *const mp_limb_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __mpz_struct {
    pub _mp_alloc: i32,
    pub _mp_size: i32,
    pub _mp_d: *mut mp_limb_t,
}
pub type mpz_t = [__mpz_struct; 1];
pub type mpz_ptr = *mut __mpz_struct;
pub type mpz_srcptr = *const __mpz_struct;
pub type mpz_div_round_mode = u32;
pub const GMP_DIV_TRUNC: mpz_div_round_mode = 2;
pub const GMP_DIV_CEIL: mpz_div_round_mode = 1;
pub const GMP_DIV_FLOOR: mpz_div_round_mode = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gmp_div_inverse {
    pub shift: u32,
    pub d1: mp_limb_t,
    pub d0: mp_limb_t,
    pub di: mp_limb_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mpn_base_info {
    pub exp: u32,
    pub bb: mp_limb_t,
}
pub const _ISspace: C2RustUnnamed = 8192;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nettle_cipher {
    pub name: *const i8,
    pub context_size: u32,
    pub block_size: u32,
    pub key_size: u32,
    pub set_encrypt_key: Option<nettle_set_key_func>,
    pub set_decrypt_key: Option<nettle_set_key_func>,
    pub encrypt: Option<nettle_cipher_func>,
    pub decrypt: Option<nettle_cipher_func>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nettle_hash {
    pub name: *const i8,
    pub context_size: u32,
    pub digest_size: u32,
    pub block_size: u32,
    pub init: Option<nettle_hash_init_func>,
    pub update: Option<nettle_hash_update_func>,
    pub digest: Option<nettle_hash_digest_func>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nettle_mac {
    pub name: *const i8,
    pub context_size: u32,
    pub digest_size: u32,
    pub key_size: u32,
    pub set_key: Option<nettle_set_key_func>,
    pub update: Option<nettle_hash_update_func>,
    pub digest: Option<nettle_hash_digest_func>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nettle_aead {
    pub name: *const i8,
    pub context_size: u32,
    pub block_size: u32,
    pub key_size: u32,
    pub nonce_size: u32,
    pub digest_size: u32,
    pub set_encrypt_key: Option<nettle_set_key_func>,
    pub set_decrypt_key: Option<nettle_set_key_func>,
    pub set_nonce: Option<nettle_set_key_func>,
    pub update: Option<nettle_hash_update_func>,
    pub encrypt: Option<nettle_crypt_func>,
    pub decrypt: Option<nettle_crypt_func>,
    pub digest: Option<nettle_hash_digest_func>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nettle_armor {
    pub name: *const i8,
    pub encode_context_size: u32,
    pub decode_context_size: u32,
    pub encode_final_length: u32,
    pub encode_init: Option<nettle_armor_init_func>,
    pub encode_length: Option<nettle_armor_length_func>,
    pub encode_update: Option<nettle_armor_encode_update_func>,
    pub encode_final: Option<nettle_armor_encode_final_func>,
    pub decode_init: Option<nettle_armor_init_func>,
    pub decode_length: Option<nettle_armor_length_func>,
    pub decode_update: Option<nettle_armor_decode_update_func>,
    pub decode_final: Option<nettle_armor_decode_final_func>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tstring {
    pub next: *mut tstring,
    pub length: size_t,
    pub data: [uint8_t; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct base16_decode_ctx {
    pub word: u8,
    pub bits: u8,
}
pub type nettle_encrypt_message_func = unsafe extern "C" fn(
    *mut libc::c_void,
    size_t,
    *const uint8_t,
    size_t,
    *const uint8_t,
    size_t,
    *mut uint8_t,
    *const uint8_t,
) -> ();
pub type nettle_decrypt_message_func = unsafe extern "C" fn(
    *mut libc::c_void,
    size_t,
    *const uint8_t,
    size_t,
    *const uint8_t,
    size_t,
    *mut uint8_t,
    *const uint8_t,
) -> i32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nettle_aead_message {
    pub name: *const i8,
    pub context_size: u32,
    pub key_size: u32,
    pub digest_size: u32,
    pub supports_inplace: i32,
    pub set_encrypt_key: Option<nettle_set_key_func>,
    pub set_decrypt_key: Option<nettle_set_key_func>,
    pub encrypt: Option<nettle_encrypt_message_func>,
    pub decrypt: Option<nettle_decrypt_message_func>,
}
pub type C2RustUnnamed = u32;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
#[no_mangle]
pub unsafe extern "C" fn die(mut format: *const i8, mut args: ...) -> ! {
    let mut args_0: ::core::ffi::VaListImpl;
    args_0 = args.clone();
    vfprintf(stderr, format, args_0.as_va_list());
    abort();
}
#[no_mangle]
pub unsafe extern "C" fn xalloc(mut size: size_t) -> *mut libc::c_void {
    let mut p: *mut libc::c_void = malloc(size);
    if size != 0 && p.is_null() {
        fprintf(stderr, b"Virtual memory exhausted.\n\0" as *const u8 as *const i8);
        abort();
    }
    return p;
}
static mut tstring_first: *mut tstring = 0 as *const tstring as *mut tstring;
#[no_mangle]
pub unsafe extern "C" fn tstring_alloc(mut length: size_t) -> *mut tstring {
    let mut s: *mut tstring = xalloc(
        (::core::mem::size_of::<tstring>() as u64).wrapping_add(length),
    ) as *mut tstring;
    (*s).length = length;
    (*s).next = tstring_first;
    *((*s).data).as_mut_ptr().offset(length as isize) = '\0' as i32 as uint8_t;
    tstring_first = s;
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn tstring_clear() {
    while !tstring_first.is_null() {
        let mut s: *mut tstring = tstring_first;
        tstring_first = (*s).next;
        free(s as *mut libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn tstring_data(
    mut length: size_t,
    mut data: *const uint8_t,
) -> *mut tstring {
    let mut s: *mut tstring = tstring_alloc(length);
    memcpy(
        ((*s).data).as_mut_ptr() as *mut libc::c_void,
        data as *const libc::c_void,
        length,
    );
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn tstring_hex(mut hex: *const i8) -> *mut tstring {
    let mut ctx: base16_decode_ctx = base16_decode_ctx {
        word: 0,
        bits: 0,
    };
    let mut s: *mut tstring = 0 as *mut tstring;
    let mut length: size_t = strlen(hex);
    s = tstring_alloc(
        length.wrapping_add(1 as i32 as u64).wrapping_div(2 as i32 as u64),
    );
    nettle_base16_decode_init(&mut ctx);
    if nettle_base16_decode_update(
        &mut ctx,
        &mut (*s).length,
        ((*s).data).as_mut_ptr(),
        length,
        hex,
    ) == 0
    {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const i8,
            b"testutils.c\0" as *const u8 as *const i8,
            90 as i32,
            b"base16_decode_update (&ctx, &s->length, s->data, length, hex)\0"
                as *const u8 as *const i8,
        );
        abort();
    }
    if nettle_base16_decode_final(&mut ctx) == 0 {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const i8,
            b"testutils.c\0" as *const u8 as *const i8,
            91 as i32,
            b"base16_decode_final (&ctx)\0" as *const u8 as *const i8,
        );
        abort();
    }
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn tstring_print_hex(mut s: *const tstring) {
    print_hex((*s).length, ((*s).data).as_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn print_hex(mut length: size_t, mut data: *const uint8_t) {
    let mut i: size_t = 0;
    i = 0 as i32 as size_t;
    while i < length {
        match i.wrapping_rem(16 as i32 as u64) {
            0 => {
                printf(b"\n\0" as *const u8 as *const i8);
            }
            8 => {
                printf(b" \0" as *const u8 as *const i8);
            }
            _ => {}
        }
        printf(b"%02x\0" as *const u8 as *const i8, *data.offset(i as isize) as i32);
        i = i.wrapping_add(1);
        i;
    }
    printf(b"\n\0" as *const u8 as *const i8);
}
#[no_mangle]
pub static mut verbose: i32 = 0 as i32;
#[no_mangle]
pub static mut test_side_channel: i32 = 0 as i32;
#[no_mangle]
pub unsafe extern "C" fn mark_bytes_undefined(
    mut size: size_t,
    mut p: *const libc::c_void,
) {}
#[no_mangle]
pub unsafe extern "C" fn mark_bytes_defined(
    mut size: size_t,
    mut p: *const libc::c_void,
) {}
unsafe fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    if argc > 1 as i32 {
        if argc == 2 as i32
            && strcmp(*argv.offset(1 as i32 as isize), b"-v\0" as *const u8 as *const i8)
                == 0
        {
            verbose = 1 as i32;
        } else {
            fprintf(
                stderr,
                b"Invalid argument `%s', only accepted option is `-v'.\n\0" as *const u8
                    as *const i8,
                *argv.offset(1 as i32 as isize),
            );
            return 1 as i32;
        }
    }
    if !(getenv(b"NETTLE_TEST_SIDE_CHANNEL\0" as *const u8 as *const i8)).is_null() {
        exit(77 as i32);
    }
    test_main();
    tstring_clear();
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn test_cipher(
    mut cipher: *const nettle_cipher,
    mut key: *const tstring,
    mut cleartext: *const tstring,
    mut ciphertext: *const tstring,
) {
    let mut ctx: *mut libc::c_void = xalloc((*cipher).context_size as size_t);
    let mut data: *mut uint8_t = xalloc((*cleartext).length) as *mut uint8_t;
    let mut length: size_t = 0;
    if !((*cleartext).length == (*ciphertext).length) {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const i8,
            b"testutils.c\0" as *const u8 as *const i8,
            188 as i32,
            b"cleartext->length == ciphertext->length\0" as *const u8 as *const i8,
        );
        abort();
    }
    length = (*cleartext).length;
    if !((*key).length == (*cipher).key_size as u64) {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const i8,
            b"testutils.c\0" as *const u8 as *const i8,
            191 as i32,
            b"key->length == cipher->key_size\0" as *const u8 as *const i8,
        );
        abort();
    }
    ((*cipher).set_encrypt_key)
        .expect("non-null function pointer")(ctx, ((*key).data).as_ptr());
    ((*cipher).encrypt)
        .expect(
            "non-null function pointer",
        )(ctx, length, data, ((*cleartext).data).as_ptr());
    if memcmp(
        data as *const libc::c_void,
        ((*ciphertext).data).as_ptr() as *const libc::c_void,
        length,
    ) != 0
    {
        fprintf(stderr, b"Encrypt failed:\nInput:\0" as *const u8 as *const i8);
        tstring_print_hex(cleartext);
        fprintf(stderr, b"\nOutput: \0" as *const u8 as *const i8);
        print_hex(length, data);
        fprintf(stderr, b"\nExpected:\0" as *const u8 as *const i8);
        tstring_print_hex(ciphertext);
        fprintf(stderr, b"\n\0" as *const u8 as *const i8);
        abort();
    }
    ((*cipher).set_decrypt_key)
        .expect("non-null function pointer")(ctx, ((*key).data).as_ptr());
    ((*cipher).decrypt).expect("non-null function pointer")(ctx, length, data, data);
    if memcmp(
        data as *const libc::c_void,
        ((*cleartext).data).as_ptr() as *const libc::c_void,
        length,
    ) != 0
    {
        fprintf(stderr, b"Decrypt failed:\nInput:\0" as *const u8 as *const i8);
        tstring_print_hex(ciphertext);
        fprintf(stderr, b"\nOutput: \0" as *const u8 as *const i8);
        print_hex(length, data);
        fprintf(stderr, b"\nExpected:\0" as *const u8 as *const i8);
        tstring_print_hex(cleartext);
        fprintf(stderr, b"\n\0" as *const u8 as *const i8);
        abort();
    }
    free(ctx);
    free(data as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn test_cipher_cbc(
    mut cipher: *const nettle_cipher,
    mut key: *const tstring,
    mut cleartext: *const tstring,
    mut ciphertext: *const tstring,
    mut iiv: *const tstring,
) {
    let mut ctx: *mut libc::c_void = xalloc((*cipher).context_size as size_t);
    let mut data: *mut uint8_t = 0 as *mut uint8_t;
    let mut iv: *mut uint8_t = xalloc((*cipher).block_size as size_t) as *mut uint8_t;
    let mut length: size_t = 0;
    if !((*cleartext).length == (*ciphertext).length) {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const i8,
            b"testutils.c\0" as *const u8 as *const i8,
            237 as i32,
            b"cleartext->length == ciphertext->length\0" as *const u8 as *const i8,
        );
        abort();
    }
    length = (*cleartext).length;
    if !((*key).length == (*cipher).key_size as u64) {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const i8,
            b"testutils.c\0" as *const u8 as *const i8,
            240 as i32,
            b"key->length == cipher->key_size\0" as *const u8 as *const i8,
        );
        abort();
    }
    if !((*iiv).length == (*cipher).block_size as u64) {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const i8,
            b"testutils.c\0" as *const u8 as *const i8,
            241 as i32,
            b"iiv->length == cipher->block_size\0" as *const u8 as *const i8,
        );
        abort();
    }
    data = xalloc(length) as *mut uint8_t;
    ((*cipher).set_encrypt_key)
        .expect("non-null function pointer")(ctx, ((*key).data).as_ptr());
    memcpy(
        iv as *mut libc::c_void,
        ((*iiv).data).as_ptr() as *const libc::c_void,
        (*cipher).block_size as u64,
    );
    nettle_cbc_encrypt(
        ctx,
        (*cipher).encrypt,
        (*cipher).block_size as size_t,
        iv,
        length,
        data,
        ((*cleartext).data).as_ptr(),
    );
    if memcmp(
        data as *const libc::c_void,
        ((*ciphertext).data).as_ptr() as *const libc::c_void,
        length,
    ) != 0
    {
        fprintf(stderr, b"CBC encrypt failed:\nInput:\0" as *const u8 as *const i8);
        tstring_print_hex(cleartext);
        fprintf(stderr, b"\nOutput: \0" as *const u8 as *const i8);
        print_hex(length, data);
        fprintf(stderr, b"\nExpected:\0" as *const u8 as *const i8);
        tstring_print_hex(ciphertext);
        fprintf(stderr, b"\n\0" as *const u8 as *const i8);
        abort();
    }
    ((*cipher).set_decrypt_key)
        .expect("non-null function pointer")(ctx, ((*key).data).as_ptr());
    memcpy(
        iv as *mut libc::c_void,
        ((*iiv).data).as_ptr() as *const libc::c_void,
        (*cipher).block_size as u64,
    );
    nettle_cbc_decrypt(
        ctx,
        (*cipher).decrypt,
        (*cipher).block_size as size_t,
        iv,
        length,
        data,
        data,
    );
    if memcmp(
        data as *const libc::c_void,
        ((*cleartext).data).as_ptr() as *const libc::c_void,
        length,
    ) != 0
    {
        fprintf(stderr, b"CBC decrypt failed:\nInput:\0" as *const u8 as *const i8);
        tstring_print_hex(ciphertext);
        fprintf(stderr, b"\nOutput: \0" as *const u8 as *const i8);
        print_hex(length, data);
        fprintf(stderr, b"\nExpected:\0" as *const u8 as *const i8);
        tstring_print_hex(cleartext);
        fprintf(stderr, b"\n\0" as *const u8 as *const i8);
        abort();
    }
    free(ctx);
    free(data as *mut libc::c_void);
    free(iv as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn test_cipher_cfb(
    mut cipher: *const nettle_cipher,
    mut key: *const tstring,
    mut cleartext: *const tstring,
    mut ciphertext: *const tstring,
    mut iiv: *const tstring,
) {
    let mut ctx: *mut libc::c_void = xalloc((*cipher).context_size as size_t);
    let mut data: *mut uint8_t = 0 as *mut uint8_t;
    let mut data2: *mut uint8_t = 0 as *mut uint8_t;
    let mut iv: *mut uint8_t = xalloc((*cipher).block_size as size_t) as *mut uint8_t;
    let mut length: size_t = 0;
    if !((*cleartext).length == (*ciphertext).length) {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const i8,
            b"testutils.c\0" as *const u8 as *const i8,
            298 as i32,
            b"cleartext->length == ciphertext->length\0" as *const u8 as *const i8,
        );
        abort();
    }
    length = (*cleartext).length;
    if !((*key).length == (*cipher).key_size as u64) {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const i8,
            b"testutils.c\0" as *const u8 as *const i8,
            301 as i32,
            b"key->length == cipher->key_size\0" as *const u8 as *const i8,
        );
        abort();
    }
    if !((*iiv).length == (*cipher).block_size as u64) {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const i8,
            b"testutils.c\0" as *const u8 as *const i8,
            302 as i32,
            b"iiv->length == cipher->block_size\0" as *const u8 as *const i8,
        );
        abort();
    }
    data = xalloc(length) as *mut uint8_t;
    data2 = xalloc(length) as *mut uint8_t;
    ((*cipher).set_encrypt_key)
        .expect("non-null function pointer")(ctx, ((*key).data).as_ptr());
    memcpy(
        iv as *mut libc::c_void,
        ((*iiv).data).as_ptr() as *const libc::c_void,
        (*cipher).block_size as u64,
    );
    nettle_cfb_encrypt(
        ctx,
        (*cipher).encrypt,
        (*cipher).block_size as size_t,
        iv,
        length,
        data,
        ((*cleartext).data).as_ptr(),
    );
    if memcmp(
        data as *const libc::c_void,
        ((*ciphertext).data).as_ptr() as *const libc::c_void,
        length,
    ) != 0
    {
        fprintf(stderr, b"CFB encrypt failed:\nInput:\0" as *const u8 as *const i8);
        tstring_print_hex(cleartext);
        fprintf(stderr, b"\nOutput: \0" as *const u8 as *const i8);
        print_hex(length, data);
        fprintf(stderr, b"\nExpected:\0" as *const u8 as *const i8);
        tstring_print_hex(ciphertext);
        fprintf(stderr, b"\n\0" as *const u8 as *const i8);
        abort();
    }
    ((*cipher).set_encrypt_key)
        .expect("non-null function pointer")(ctx, ((*key).data).as_ptr());
    memcpy(
        iv as *mut libc::c_void,
        ((*iiv).data).as_ptr() as *const libc::c_void,
        (*cipher).block_size as u64,
    );
    nettle_cfb_decrypt(
        ctx,
        (*cipher).encrypt,
        (*cipher).block_size as size_t,
        iv,
        length,
        data2,
        data,
    );
    if memcmp(
        data2 as *const libc::c_void,
        ((*cleartext).data).as_ptr() as *const libc::c_void,
        length,
    ) != 0
    {
        fprintf(stderr, b"CFB decrypt failed:\nInput:\0" as *const u8 as *const i8);
        tstring_print_hex(ciphertext);
        fprintf(stderr, b"\nOutput: \0" as *const u8 as *const i8);
        print_hex(length, data2);
        fprintf(stderr, b"\nExpected:\0" as *const u8 as *const i8);
        tstring_print_hex(cleartext);
        fprintf(stderr, b"\n\0" as *const u8 as *const i8);
        abort();
    }
    ((*cipher).set_encrypt_key)
        .expect("non-null function pointer")(ctx, ((*key).data).as_ptr());
    memcpy(
        iv as *mut libc::c_void,
        ((*iiv).data).as_ptr() as *const libc::c_void,
        (*cipher).block_size as u64,
    );
    memcpy(
        data as *mut libc::c_void,
        ((*cleartext).data).as_ptr() as *const libc::c_void,
        length,
    );
    nettle_cfb_encrypt(
        ctx,
        (*cipher).encrypt,
        (*cipher).block_size as size_t,
        iv,
        length,
        data,
        data,
    );
    if memcmp(
        data as *const libc::c_void,
        ((*ciphertext).data).as_ptr() as *const libc::c_void,
        length,
    ) != 0
    {
        fprintf(
            stderr,
            b"CFB inplace encrypt failed:\nInput:\0" as *const u8 as *const i8,
        );
        tstring_print_hex(cleartext);
        fprintf(stderr, b"\nOutput: \0" as *const u8 as *const i8);
        print_hex(length, data);
        fprintf(stderr, b"\nExpected:\0" as *const u8 as *const i8);
        tstring_print_hex(ciphertext);
        fprintf(stderr, b"\n\0" as *const u8 as *const i8);
        abort();
    }
    ((*cipher).set_encrypt_key)
        .expect("non-null function pointer")(ctx, ((*key).data).as_ptr());
    memcpy(
        iv as *mut libc::c_void,
        ((*iiv).data).as_ptr() as *const libc::c_void,
        (*cipher).block_size as u64,
    );
    nettle_cfb_decrypt(
        ctx,
        (*cipher).encrypt,
        (*cipher).block_size as size_t,
        iv,
        length,
        data,
        data,
    );
    if memcmp(
        data as *const libc::c_void,
        ((*cleartext).data).as_ptr() as *const libc::c_void,
        length,
    ) != 0
    {
        fprintf(
            stderr,
            b"CFB inplace decrypt failed:\nInput:\0" as *const u8 as *const i8,
        );
        tstring_print_hex(ciphertext);
        fprintf(stderr, b"\nOutput: \0" as *const u8 as *const i8);
        print_hex(length, data);
        fprintf(stderr, b"\nExpected:\0" as *const u8 as *const i8);
        tstring_print_hex(cleartext);
        fprintf(stderr, b"\n\0" as *const u8 as *const i8);
        abort();
    }
    length = (length as u64).wrapping_sub(1 as i32 as u64) as size_t as size_t;
    ((*cipher).set_encrypt_key)
        .expect("non-null function pointer")(ctx, ((*key).data).as_ptr());
    memcpy(
        iv as *mut libc::c_void,
        ((*iiv).data).as_ptr() as *const libc::c_void,
        (*cipher).block_size as u64,
    );
    nettle_cfb_encrypt(
        ctx,
        (*cipher).encrypt,
        (*cipher).block_size as size_t,
        iv,
        length,
        data,
        ((*cleartext).data).as_ptr(),
    );
    if memcmp(
        data as *const libc::c_void,
        ((*ciphertext).data).as_ptr() as *const libc::c_void,
        length,
    ) != 0
    {
        fprintf(stderr, b"CFB encrypt failed:\nInput:\0" as *const u8 as *const i8);
        print_hex(length, ((*cleartext).data).as_ptr());
        fprintf(stderr, b"\nOutput: \0" as *const u8 as *const i8);
        print_hex(length, data);
        fprintf(stderr, b"\nExpected:\0" as *const u8 as *const i8);
        print_hex(length, ((*ciphertext).data).as_ptr());
        fprintf(stderr, b"\n\0" as *const u8 as *const i8);
        abort();
    }
    ((*cipher).set_encrypt_key)
        .expect("non-null function pointer")(ctx, ((*key).data).as_ptr());
    memcpy(
        iv as *mut libc::c_void,
        ((*iiv).data).as_ptr() as *const libc::c_void,
        (*cipher).block_size as u64,
    );
    nettle_cfb_decrypt(
        ctx,
        (*cipher).encrypt,
        (*cipher).block_size as size_t,
        iv,
        length,
        data2,
        data,
    );
    if memcmp(
        data2 as *const libc::c_void,
        ((*cleartext).data).as_ptr() as *const libc::c_void,
        length,
    ) != 0
    {
        fprintf(stderr, b"CFB decrypt failed:\nInput:\0" as *const u8 as *const i8);
        print_hex(length, ((*ciphertext).data).as_ptr());
        fprintf(stderr, b"\nOutput: \0" as *const u8 as *const i8);
        print_hex(length, data2);
        fprintf(stderr, b"\nExpected:\0" as *const u8 as *const i8);
        print_hex(length, ((*cleartext).data).as_ptr());
        fprintf(stderr, b"\n\0" as *const u8 as *const i8);
        abort();
    }
    ((*cipher).set_encrypt_key)
        .expect("non-null function pointer")(ctx, ((*key).data).as_ptr());
    memcpy(
        iv as *mut libc::c_void,
        ((*iiv).data).as_ptr() as *const libc::c_void,
        (*cipher).block_size as u64,
    );
    memcpy(
        data as *mut libc::c_void,
        ((*cleartext).data).as_ptr() as *const libc::c_void,
        length,
    );
    nettle_cfb_encrypt(
        ctx,
        (*cipher).encrypt,
        (*cipher).block_size as size_t,
        iv,
        length,
        data,
        data,
    );
    if memcmp(
        data as *const libc::c_void,
        ((*ciphertext).data).as_ptr() as *const libc::c_void,
        length,
    ) != 0
    {
        fprintf(
            stderr,
            b"CFB inplace encrypt failed:\nInput:\0" as *const u8 as *const i8,
        );
        print_hex(length, ((*cleartext).data).as_ptr());
        fprintf(stderr, b"\nOutput: \0" as *const u8 as *const i8);
        print_hex(length, data);
        fprintf(stderr, b"\nExpected:\0" as *const u8 as *const i8);
        print_hex(length, ((*ciphertext).data).as_ptr());
        fprintf(stderr, b"\n\0" as *const u8 as *const i8);
        abort();
    }
    ((*cipher).set_encrypt_key)
        .expect("non-null function pointer")(ctx, ((*key).data).as_ptr());
    memcpy(
        iv as *mut libc::c_void,
        ((*iiv).data).as_ptr() as *const libc::c_void,
        (*cipher).block_size as u64,
    );
    nettle_cfb_decrypt(
        ctx,
        (*cipher).encrypt,
        (*cipher).block_size as size_t,
        iv,
        length,
        data,
        data,
    );
    if memcmp(
        data as *const libc::c_void,
        ((*cleartext).data).as_ptr() as *const libc::c_void,
        length,
    ) != 0
    {
        fprintf(
            stderr,
            b"CFB inplace decrypt failed:\nInput:\0" as *const u8 as *const i8,
        );
        print_hex(length, ((*ciphertext).data).as_ptr());
        fprintf(stderr, b"\nOutput: \0" as *const u8 as *const i8);
        print_hex(length, data);
        fprintf(stderr, b"\nExpected:\0" as *const u8 as *const i8);
        print_hex(length, ((*cleartext).data).as_ptr());
        fprintf(stderr, b"\n\0" as *const u8 as *const i8);
        abort();
    }
    free(ctx);
    free(data as *mut libc::c_void);
    free(data2 as *mut libc::c_void);
    free(iv as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn test_cipher_cfb8(
    mut cipher: *const nettle_cipher,
    mut key: *const tstring,
    mut cleartext: *const tstring,
    mut ciphertext: *const tstring,
    mut iiv: *const tstring,
) {
    let mut ctx: *mut libc::c_void = xalloc((*cipher).context_size as size_t);
    let mut data: *mut uint8_t = 0 as *mut uint8_t;
    let mut data2: *mut uint8_t = 0 as *mut uint8_t;
    let mut iv: *mut uint8_t = xalloc((*cipher).block_size as size_t) as *mut uint8_t;
    let mut length: size_t = 0;
    let mut block: size_t = 0;
    if !((*cleartext).length == (*ciphertext).length) {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const i8,
            b"testutils.c\0" as *const u8 as *const i8,
            477 as i32,
            b"cleartext->length == ciphertext->length\0" as *const u8 as *const i8,
        );
        abort();
    }
    length = (*cleartext).length;
    if !((*key).length == (*cipher).key_size as u64) {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const i8,
            b"testutils.c\0" as *const u8 as *const i8,
            480 as i32,
            b"key->length == cipher->key_size\0" as *const u8 as *const i8,
        );
        abort();
    }
    if !((*iiv).length == (*cipher).block_size as u64) {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const i8,
            b"testutils.c\0" as *const u8 as *const i8,
            481 as i32,
            b"iiv->length == cipher->block_size\0" as *const u8 as *const i8,
        );
        abort();
    }
    data = xalloc(length.wrapping_add(1 as i32 as u64)) as *mut uint8_t;
    data2 = xalloc(length.wrapping_add(1 as i32 as u64)) as *mut uint8_t;
    block = 1 as i32 as size_t;
    while block <= length {
        let mut i: size_t = 0;
        ((*cipher).set_encrypt_key)
            .expect("non-null function pointer")(ctx, ((*key).data).as_ptr());
        memcpy(
            iv as *mut libc::c_void,
            ((*iiv).data).as_ptr() as *const libc::c_void,
            (*cipher).block_size as u64,
        );
        memset(
            data as *mut libc::c_void,
            0x17 as i32,
            length.wrapping_add(1 as i32 as u64),
        );
        i = 0 as i32 as size_t;
        while i.wrapping_add(block) <= length {
            nettle_cfb8_encrypt(
                ctx,
                (*cipher).encrypt,
                (*cipher).block_size as size_t,
                iv,
                block,
                data.offset(i as isize),
                ((*cleartext).data).as_ptr().offset(i as isize),
            );
            i = (i as u64).wrapping_add(block) as size_t as size_t;
        }
        nettle_cfb8_encrypt(
            ctx,
            (*cipher).encrypt,
            (*cipher).block_size as size_t,
            iv,
            length.wrapping_sub(i),
            data.offset(i as isize),
            ((*cleartext).data).as_ptr().offset(i as isize),
        );
        if memcmp(
            data as *const libc::c_void,
            ((*ciphertext).data).as_ptr() as *const libc::c_void,
            length,
        ) != 0
        {
            fprintf(
                stderr,
                b"CFB8 encrypt failed, block size %lu:\nInput:\0" as *const u8
                    as *const i8,
                block,
            );
            tstring_print_hex(cleartext);
            fprintf(stderr, b"\nOutput: \0" as *const u8 as *const i8);
            print_hex(length, data);
            fprintf(stderr, b"\nExpected:\0" as *const u8 as *const i8);
            tstring_print_hex(ciphertext);
            fprintf(stderr, b"\n\0" as *const u8 as *const i8);
            abort();
        }
        if !(*data.offset(length as isize) as i32 == 0x17 as i32) {
            fprintf(
                stderr,
                b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const i8,
                b"testutils.c\0" as *const u8 as *const i8,
                516 as i32,
                b"data[length] == 0x17\0" as *const u8 as *const i8,
            );
            abort();
        }
        ((*cipher).set_encrypt_key)
            .expect("non-null function pointer")(ctx, ((*key).data).as_ptr());
        memcpy(
            iv as *mut libc::c_void,
            ((*iiv).data).as_ptr() as *const libc::c_void,
            (*cipher).block_size as u64,
        );
        memset(
            data2 as *mut libc::c_void,
            0x17 as i32,
            length.wrapping_add(1 as i32 as u64),
        );
        i = 0 as i32 as size_t;
        while i.wrapping_add(block) <= length {
            nettle_cfb8_decrypt(
                ctx,
                (*cipher).encrypt,
                (*cipher).block_size as size_t,
                iv,
                block,
                data2.offset(i as isize),
                data.offset(i as isize),
            );
            i = (i as u64).wrapping_add(block) as size_t as size_t;
        }
        nettle_cfb8_decrypt(
            ctx,
            (*cipher).encrypt,
            (*cipher).block_size as size_t,
            iv,
            length.wrapping_sub(i),
            data2.offset(i as isize),
            data.offset(i as isize),
        );
        if memcmp(
            data2 as *const libc::c_void,
            ((*cleartext).data).as_ptr() as *const libc::c_void,
            length,
        ) != 0
        {
            fprintf(
                stderr,
                b"CFB8 decrypt failed, block size %lu:\nInput:\0" as *const u8
                    as *const i8,
                block,
            );
            tstring_print_hex(ciphertext);
            fprintf(stderr, b"\nOutput: \0" as *const u8 as *const i8);
            print_hex(length, data2);
            fprintf(stderr, b"\nExpected:\0" as *const u8 as *const i8);
            tstring_print_hex(cleartext);
            fprintf(stderr, b"\n\0" as *const u8 as *const i8);
            abort();
        }
        if !(*data.offset(length as isize) as i32 == 0x17 as i32) {
            fprintf(
                stderr,
                b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const i8,
                b"testutils.c\0" as *const u8 as *const i8,
                544 as i32,
                b"data[length] == 0x17\0" as *const u8 as *const i8,
            );
            abort();
        }
        block = block.wrapping_add(1);
        block;
    }
    ((*cipher).set_encrypt_key)
        .expect("non-null function pointer")(ctx, ((*key).data).as_ptr());
    memcpy(
        iv as *mut libc::c_void,
        ((*iiv).data).as_ptr() as *const libc::c_void,
        (*cipher).block_size as u64,
    );
    memcpy(
        data as *mut libc::c_void,
        ((*cleartext).data).as_ptr() as *const libc::c_void,
        length,
    );
    nettle_cfb8_encrypt(
        ctx,
        (*cipher).encrypt,
        (*cipher).block_size as size_t,
        iv,
        length,
        data,
        data,
    );
    if memcmp(
        data as *const libc::c_void,
        ((*ciphertext).data).as_ptr() as *const libc::c_void,
        length,
    ) != 0
    {
        fprintf(
            stderr,
            b"CFB8 inplace encrypt failed:\nInput:\0" as *const u8 as *const i8,
        );
        tstring_print_hex(cleartext);
        fprintf(stderr, b"\nOutput: \0" as *const u8 as *const i8);
        print_hex(length, data);
        fprintf(stderr, b"\nExpected:\0" as *const u8 as *const i8);
        tstring_print_hex(ciphertext);
        fprintf(stderr, b"\n\0" as *const u8 as *const i8);
        abort();
    }
    ((*cipher).set_encrypt_key)
        .expect("non-null function pointer")(ctx, ((*key).data).as_ptr());
    memcpy(
        iv as *mut libc::c_void,
        ((*iiv).data).as_ptr() as *const libc::c_void,
        (*cipher).block_size as u64,
    );
    nettle_cfb8_decrypt(
        ctx,
        (*cipher).encrypt,
        (*cipher).block_size as size_t,
        iv,
        length,
        data,
        data,
    );
    if memcmp(
        data as *const libc::c_void,
        ((*cleartext).data).as_ptr() as *const libc::c_void,
        length,
    ) != 0
    {
        fprintf(
            stderr,
            b"CFB8 inplace decrypt failed:\nInput:\0" as *const u8 as *const i8,
        );
        tstring_print_hex(ciphertext);
        fprintf(stderr, b"\nOutput: \0" as *const u8 as *const i8);
        print_hex(length, data);
        fprintf(stderr, b"\nExpected:\0" as *const u8 as *const i8);
        tstring_print_hex(cleartext);
        fprintf(stderr, b"\n\0" as *const u8 as *const i8);
        abort();
    }
    length = (length as u64).wrapping_sub(1 as i32 as u64) as size_t as size_t;
    ((*cipher).set_encrypt_key)
        .expect("non-null function pointer")(ctx, ((*key).data).as_ptr());
    memcpy(
        iv as *mut libc::c_void,
        ((*iiv).data).as_ptr() as *const libc::c_void,
        (*cipher).block_size as u64,
    );
    nettle_cfb8_encrypt(
        ctx,
        (*cipher).encrypt,
        (*cipher).block_size as size_t,
        iv,
        length,
        data,
        ((*cleartext).data).as_ptr(),
    );
    if memcmp(
        data as *const libc::c_void,
        ((*ciphertext).data).as_ptr() as *const libc::c_void,
        length,
    ) != 0
    {
        fprintf(stderr, b"CFB8 encrypt failed:\nInput:\0" as *const u8 as *const i8);
        print_hex(length, ((*cleartext).data).as_ptr());
        fprintf(stderr, b"\nOutput: \0" as *const u8 as *const i8);
        print_hex(length, data);
        fprintf(stderr, b"\nExpected:\0" as *const u8 as *const i8);
        print_hex(length, ((*ciphertext).data).as_ptr());
        fprintf(stderr, b"\n\0" as *const u8 as *const i8);
        abort();
    }
    ((*cipher).set_encrypt_key)
        .expect("non-null function pointer")(ctx, ((*key).data).as_ptr());
    memcpy(
        iv as *mut libc::c_void,
        ((*iiv).data).as_ptr() as *const libc::c_void,
        (*cipher).block_size as u64,
    );
    nettle_cfb8_decrypt(
        ctx,
        (*cipher).encrypt,
        (*cipher).block_size as size_t,
        iv,
        length,
        data2,
        data,
    );
    if memcmp(
        data2 as *const libc::c_void,
        ((*cleartext).data).as_ptr() as *const libc::c_void,
        length,
    ) != 0
    {
        fprintf(stderr, b"CFB8 decrypt failed:\nInput:\0" as *const u8 as *const i8);
        print_hex(length, ((*ciphertext).data).as_ptr());
        fprintf(stderr, b"\nOutput: \0" as *const u8 as *const i8);
        print_hex(length, data2);
        fprintf(stderr, b"\nExpected:\0" as *const u8 as *const i8);
        print_hex(length, ((*cleartext).data).as_ptr());
        fprintf(stderr, b"\n\0" as *const u8 as *const i8);
        abort();
    }
    ((*cipher).set_encrypt_key)
        .expect("non-null function pointer")(ctx, ((*key).data).as_ptr());
    memcpy(
        iv as *mut libc::c_void,
        ((*iiv).data).as_ptr() as *const libc::c_void,
        (*cipher).block_size as u64,
    );
    memcpy(
        data as *mut libc::c_void,
        ((*cleartext).data).as_ptr() as *const libc::c_void,
        length,
    );
    nettle_cfb8_encrypt(
        ctx,
        (*cipher).encrypt,
        (*cipher).block_size as size_t,
        iv,
        length,
        data,
        data,
    );
    if memcmp(
        data as *const libc::c_void,
        ((*ciphertext).data).as_ptr() as *const libc::c_void,
        length,
    ) != 0
    {
        fprintf(
            stderr,
            b"CFB8 inplace encrypt failed:\nInput:\0" as *const u8 as *const i8,
        );
        print_hex(length, ((*cleartext).data).as_ptr());
        fprintf(stderr, b"\nOutput: \0" as *const u8 as *const i8);
        print_hex(length, data);
        fprintf(stderr, b"\nExpected:\0" as *const u8 as *const i8);
        print_hex(length, ((*ciphertext).data).as_ptr());
        fprintf(stderr, b"\n\0" as *const u8 as *const i8);
        abort();
    }
    ((*cipher).set_encrypt_key)
        .expect("non-null function pointer")(ctx, ((*key).data).as_ptr());
    memcpy(
        iv as *mut libc::c_void,
        ((*iiv).data).as_ptr() as *const libc::c_void,
        (*cipher).block_size as u64,
    );
    nettle_cfb8_decrypt(
        ctx,
        (*cipher).encrypt,
        (*cipher).block_size as size_t,
        iv,
        length,
        data,
        data,
    );
    if memcmp(
        data as *const libc::c_void,
        ((*cleartext).data).as_ptr() as *const libc::c_void,
        length,
    ) != 0
    {
        fprintf(
            stderr,
            b"CFB8 inplace decrypt failed:\nInput:\0" as *const u8 as *const i8,
        );
        print_hex(length, ((*ciphertext).data).as_ptr());
        fprintf(stderr, b"\nOutput: \0" as *const u8 as *const i8);
        print_hex(length, data);
        fprintf(stderr, b"\nExpected:\0" as *const u8 as *const i8);
        print_hex(length, ((*cleartext).data).as_ptr());
        fprintf(stderr, b"\n\0" as *const u8 as *const i8);
        abort();
    }
    free(ctx);
    free(data as *mut libc::c_void);
    free(data2 as *mut libc::c_void);
    free(iv as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn test_cipher_ctr(
    mut cipher: *const nettle_cipher,
    mut key: *const tstring,
    mut cleartext: *const tstring,
    mut ciphertext: *const tstring,
    mut ictr: *const tstring,
) {
    let mut ctx: *mut libc::c_void = xalloc((*cipher).context_size as size_t);
    let mut data: *mut uint8_t = 0 as *mut uint8_t;
    let mut ctr: *mut uint8_t = xalloc((*cipher).block_size as size_t) as *mut uint8_t;
    let mut octr: *mut uint8_t = xalloc((*cipher).block_size as size_t) as *mut uint8_t;
    let mut length: size_t = 0;
    let mut nblocks: size_t = 0;
    let mut low: u32 = 0;
    let mut i: size_t = 0;
    if !((*cleartext).length == (*ciphertext).length) {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const i8,
            b"testutils.c\0" as *const u8 as *const i8,
            683 as i32,
            b"cleartext->length == ciphertext->length\0" as *const u8 as *const i8,
        );
        abort();
    }
    length = (*cleartext).length;
    if !((*key).length == (*cipher).key_size as u64) {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const i8,
            b"testutils.c\0" as *const u8 as *const i8,
            686 as i32,
            b"key->length == cipher->key_size\0" as *const u8 as *const i8,
        );
        abort();
    }
    if !((*ictr).length == (*cipher).block_size as u64) {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const i8,
            b"testutils.c\0" as *const u8 as *const i8,
            687 as i32,
            b"ictr->length == cipher->block_size\0" as *const u8 as *const i8,
        );
        abort();
    }
    nblocks = length
        .wrapping_add((*cipher).block_size as u64)
        .wrapping_sub(1 as i32 as u64)
        .wrapping_div((*cipher).block_size as u64);
    if !(nblocks < 0x100 as i32 as u64) {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const i8,
            b"testutils.c\0" as *const u8 as *const i8,
            691 as i32,
            b"nblocks < 0x100\0" as *const u8 as *const i8,
        );
        abort();
    }
    memcpy(
        octr as *mut libc::c_void,
        ((*ictr).data).as_ptr() as *const libc::c_void,
        ((*cipher).block_size).wrapping_sub(1 as i32 as u32) as u64,
    );
    low = (*((*ictr).data)
        .as_ptr()
        .offset(((*cipher).block_size).wrapping_sub(1 as i32 as u32) as isize) as u64)
        .wrapping_add(nblocks) as u32;
    *octr.offset(((*cipher).block_size).wrapping_sub(1 as i32 as u32) as isize) = low
        as uint8_t;
    if low >= 0x100 as i32 as u32 {
        let mut increment_i: u32 = ((*cipher).block_size)
            .wrapping_sub(1 as i32 as u32)
            .wrapping_sub(1 as i32 as u32);
        let ref mut fresh0 = *octr.offset(increment_i as isize);
        *fresh0 = (*fresh0).wrapping_add(1);
        if *fresh0 as i32 == 0 as i32 {
            while increment_i > 0 as i32 as u32
                && {
                    increment_i = increment_i.wrapping_sub(1);
                    let ref mut fresh1 = *octr.offset(increment_i as isize);
                    *fresh1 = (*fresh1).wrapping_add(1);
                    *fresh1 as i32 == 0 as i32
                }
            {}
        }
    }
    data = xalloc(length) as *mut uint8_t;
    ((*cipher).set_encrypt_key)
        .expect("non-null function pointer")(ctx, ((*key).data).as_ptr());
    i = 0 as i32 as size_t;
    while i <= length {
        memcpy(
            ctr as *mut libc::c_void,
            ((*ictr).data).as_ptr() as *const libc::c_void,
            (*cipher).block_size as u64,
        );
        memset(data as *mut libc::c_void, 17 as i32, length);
        nettle_ctr_crypt(
            ctx,
            (*cipher).encrypt,
            (*cipher).block_size as size_t,
            ctr,
            i,
            data,
            ((*cleartext).data).as_ptr(),
        );
        if memcmp(
            data as *const libc::c_void,
            ((*ciphertext).data).as_ptr() as *const libc::c_void,
            i,
        ) != 0 || i < length && *data.offset(i as isize) as i32 != 17 as i32
        {
            fprintf(
                stderr,
                b"CTR encrypt failed (length %d of %d):\nInput:\0" as *const u8
                    as *const i8,
                i as i32,
                length as i32,
            );
            tstring_print_hex(cleartext);
            fprintf(stderr, b"\nOutput: \0" as *const u8 as *const i8);
            print_hex(length, data);
            fprintf(stderr, b"\nExpected:\0" as *const u8 as *const i8);
            tstring_print_hex(ciphertext);
            fprintf(stderr, b"\n\0" as *const u8 as *const i8);
            abort();
        }
        i = i.wrapping_add(1);
        i;
    }
    if memcmp(
        ctr as *const libc::c_void,
        octr as *const libc::c_void,
        (*cipher).block_size as u64,
    ) != 0
    {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const i8,
            b"testutils.c\0" as *const u8 as *const i8,
            728 as i32,
            b"MEMEQ (cipher->block_size, ctr, octr)\0" as *const u8 as *const i8,
        );
        abort();
    }
    memcpy(
        ctr as *mut libc::c_void,
        ((*ictr).data).as_ptr() as *const libc::c_void,
        (*cipher).block_size as u64,
    );
    nettle_ctr_crypt(
        ctx,
        (*cipher).encrypt,
        (*cipher).block_size as size_t,
        ctr,
        length,
        data,
        data,
    );
    if memcmp(
        data as *const libc::c_void,
        ((*cleartext).data).as_ptr() as *const libc::c_void,
        length,
    ) != 0
    {
        fprintf(stderr, b"CTR decrypt failed:\nInput:\0" as *const u8 as *const i8);
        tstring_print_hex(ciphertext);
        fprintf(stderr, b"\nOutput: \0" as *const u8 as *const i8);
        print_hex(length, data);
        fprintf(stderr, b"\nExpected:\0" as *const u8 as *const i8);
        tstring_print_hex(cleartext);
        fprintf(stderr, b"\n\0" as *const u8 as *const i8);
        abort();
    }
    if memcmp(
        ctr as *const libc::c_void,
        octr as *const libc::c_void,
        (*cipher).block_size as u64,
    ) != 0
    {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const i8,
            b"testutils.c\0" as *const u8 as *const i8,
            748 as i32,
            b"MEMEQ (cipher->block_size, ctr, octr)\0" as *const u8 as *const i8,
        );
        abort();
    }
    free(ctx);
    free(data as *mut libc::c_void);
    free(octr as *mut libc::c_void);
    free(ctr as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn test_aead(
    mut aead: *const nettle_aead,
    mut set_nonce: Option<nettle_hash_update_func>,
    mut key: *const tstring,
    mut authtext: *const tstring,
    mut cleartext: *const tstring,
    mut ciphertext: *const tstring,
    mut nonce: *const tstring,
    mut digest: *const tstring,
) {
    let mut ctx: *mut libc::c_void = xalloc((*aead).context_size as size_t);
    let mut in_0: *mut uint8_t = 0 as *mut uint8_t;
    let mut out: *mut uint8_t = 0 as *mut uint8_t;
    let mut buffer: *mut uint8_t = 0 as *mut uint8_t;
    let mut in_align: u32 = 0;
    if !((*cleartext).length == (*ciphertext).length) {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const i8,
            b"testutils.c\0" as *const u8 as *const i8,
            841 as i32,
            b"cleartext->length == ciphertext->length\0" as *const u8 as *const i8,
        );
        abort();
    }
    if !((*key).length == (*aead).key_size as u64) {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const i8,
            b"testutils.c\0" as *const u8 as *const i8,
            842 as i32,
            b"key->length == aead->key_size\0" as *const u8 as *const i8,
        );
        abort();
    }
    if !((*aead).block_size > 0 as i32 as u32) {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const i8,
            b"testutils.c\0" as *const u8 as *const i8,
            843 as i32,
            b"aead->block_size > 0\0" as *const u8 as *const i8,
        );
        abort();
    }
    buffer = xalloc((*aead).digest_size as size_t) as *mut uint8_t;
    in_0 = xalloc(
        ((*cleartext).length)
            .wrapping_add((*aead).block_size as u64)
            .wrapping_sub(1 as i32 as u64),
    ) as *mut uint8_t;
    out = xalloc(
        ((*cleartext).length)
            .wrapping_add((*aead).block_size as u64)
            .wrapping_sub(1 as i32 as u64),
    ) as *mut uint8_t;
    in_align = 0 as i32 as u32;
    while in_align < (*aead).block_size {
        let mut out_align: u32 = (3 as i32 as u32)
            .wrapping_mul(in_align)
            .wrapping_rem((*aead).block_size);
        let mut offset: size_t = 0;
        memcpy(
            in_0.offset(in_align as isize) as *mut libc::c_void,
            ((*cleartext).data).as_ptr() as *const libc::c_void,
            (*cleartext).length,
        );
        offset = 0 as i32 as size_t;
        while offset <= (*cleartext).length {
            ((*aead).set_encrypt_key)
                .expect("non-null function pointer")(ctx, ((*key).data).as_ptr());
            if set_nonce.is_some() {
                set_nonce
                    .expect(
                        "non-null function pointer",
                    )(ctx, (*nonce).length, ((*nonce).data).as_ptr());
            } else {
                if (*nonce).length == (*aead).nonce_size as u64 {} else {
                    __assert_fail(
                        b"nonce->length == aead->nonce_size\0" as *const u8 as *const i8,
                        b"testutils.c\0" as *const u8 as *const i8,
                        864 as i32 as u32,
                        (*::core::mem::transmute::<
                            &[u8; 214],
                            &[i8; 214],
                        >(
                            b"void test_aead(const struct nettle_aead *, nettle_hash_update_func *, const struct tstring *, const struct tstring *, const struct tstring *, const struct tstring *, const struct tstring *, const struct tstring *)\0",
                        ))
                            .as_ptr(),
                    );
                }
                'c_36861: {
                    if (*nonce).length == (*aead).nonce_size as u64 {} else {
                        __assert_fail(
                            b"nonce->length == aead->nonce_size\0" as *const u8
                                as *const i8,
                            b"testutils.c\0" as *const u8 as *const i8,
                            864 as i32 as u32,
                            (*::core::mem::transmute::<
                                &[u8; 214],
                                &[i8; 214],
                            >(
                                b"void test_aead(const struct nettle_aead *, nettle_hash_update_func *, const struct tstring *, const struct tstring *, const struct tstring *, const struct tstring *, const struct tstring *, const struct tstring *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                };
                ((*aead).set_nonce)
                    .expect("non-null function pointer")(ctx, ((*nonce).data).as_ptr());
            }
            if ((*aead).update).is_some() {
                let mut a_offset: size_t = if offset <= (*authtext).length {
                    offset
                } else {
                    0 as i32 as u64
                };
                ((*aead).update)
                    .expect(
                        "non-null function pointer",
                    )(ctx, a_offset, ((*authtext).data).as_ptr());
                ((*aead).update)
                    .expect(
                        "non-null function pointer",
                    )(ctx, 0 as i32 as size_t, 0 as *const uint8_t);
                ((*aead).update)
                    .expect(
                        "non-null function pointer",
                    )(
                    ctx,
                    ((*authtext).length).wrapping_sub(a_offset),
                    ((*authtext).data).as_ptr().offset(a_offset as isize),
                );
            }
            ((*aead).encrypt)
                .expect(
                    "non-null function pointer",
                )(
                ctx,
                offset,
                out.offset(out_align as isize),
                in_0.offset(in_align as isize),
            );
            ((*aead).encrypt)
                .expect(
                    "non-null function pointer",
                )(
                ctx,
                0 as i32 as size_t,
                out.offset(out_align as isize),
                0 as *const uint8_t,
            );
            ((*aead).encrypt)
                .expect(
                    "non-null function pointer",
                )(
                ctx,
                ((*cleartext).length).wrapping_sub(offset),
                out.offset(out_align as isize).offset(offset as isize),
                in_0.offset(in_align as isize).offset(offset as isize),
            );
            if memcmp(
                out.offset(out_align as isize) as *const libc::c_void,
                ((*ciphertext).data).as_ptr() as *const libc::c_void,
                (*cleartext).length,
            ) != 0
            {
                fprintf(
                    stderr,
                    b"aead->encrypt failed (offset = %u):\nclear: \0" as *const u8
                        as *const i8,
                    offset as u32,
                );
                tstring_print_hex(cleartext);
                fprintf(stderr, b"  got: \0" as *const u8 as *const i8);
                print_hex((*cleartext).length, out.offset(out_align as isize));
                fprintf(stderr, b"  exp: \0" as *const u8 as *const i8);
                tstring_print_hex(ciphertext);
                abort();
            }
            if !digest.is_null() {
                if !((*digest).length <= (*aead).digest_size as u64) {
                    fprintf(
                        stderr,
                        b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const i8,
                        b"testutils.c\0" as *const u8 as *const i8,
                        892 as i32,
                        b"digest->length <= aead->digest_size\0" as *const u8
                            as *const i8,
                    );
                    abort();
                }
                memset(
                    buffer as *mut libc::c_void,
                    0 as i32,
                    (*aead).digest_size as u64,
                );
                ((*aead).digest)
                    .expect("non-null function pointer")(ctx, (*digest).length, buffer);
                if memcmp(
                    buffer as *const libc::c_void,
                    ((*digest).data).as_ptr() as *const libc::c_void,
                    (*digest).length,
                ) != 0
                {
                    fprintf(
                        stderr,
                        b"aead->digest failed (offset = %u):\n  got: \0" as *const u8
                            as *const i8,
                        offset as u32,
                    );
                    print_hex((*digest).length, buffer);
                    fprintf(stderr, b"  exp: \0" as *const u8 as *const i8);
                    tstring_print_hex(digest);
                    abort();
                }
            } else if ((*aead).digest).is_some() {
                fprintf(
                    stderr,
                    b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const i8,
                    b"testutils.c\0" as *const u8 as *const i8,
                    906 as i32,
                    b"!aead->digest\0" as *const u8 as *const i8,
                );
                abort();
            }
            if ((*aead).set_decrypt_key).is_some() {
                ((*aead).set_decrypt_key)
                    .expect("non-null function pointer")(ctx, ((*key).data).as_ptr());
                if set_nonce.is_some() {
                    set_nonce
                        .expect(
                            "non-null function pointer",
                        )(ctx, (*nonce).length, ((*nonce).data).as_ptr());
                } else {
                    if (*nonce).length == (*aead).nonce_size as u64 {} else {
                        __assert_fail(
                            b"nonce->length == aead->nonce_size\0" as *const u8
                                as *const i8,
                            b"testutils.c\0" as *const u8 as *const i8,
                            917 as i32 as u32,
                            (*::core::mem::transmute::<
                                &[u8; 214],
                                &[i8; 214],
                            >(
                                b"void test_aead(const struct nettle_aead *, nettle_hash_update_func *, const struct tstring *, const struct tstring *, const struct tstring *, const struct tstring *, const struct tstring *, const struct tstring *)\0",
                            ))
                                .as_ptr(),
                        );
                    }
                    'c_36390: {
                        if (*nonce).length == (*aead).nonce_size as u64 {} else {
                            __assert_fail(
                                b"nonce->length == aead->nonce_size\0" as *const u8
                                    as *const i8,
                                b"testutils.c\0" as *const u8 as *const i8,
                                917 as i32 as u32,
                                (*::core::mem::transmute::<
                                    &[u8; 214],
                                    &[i8; 214],
                                >(
                                    b"void test_aead(const struct nettle_aead *, nettle_hash_update_func *, const struct tstring *, const struct tstring *, const struct tstring *, const struct tstring *, const struct tstring *, const struct tstring *)\0",
                                ))
                                    .as_ptr(),
                            );
                        }
                    };
                    ((*aead).set_nonce)
                        .expect(
                            "non-null function pointer",
                        )(ctx, ((*nonce).data).as_ptr());
                }
                if ((*aead).update).is_some() && (*authtext).length != 0 {
                    ((*aead).update)
                        .expect(
                            "non-null function pointer",
                        )(ctx, (*authtext).length, ((*authtext).data).as_ptr());
                }
                ((*aead).decrypt)
                    .expect(
                        "non-null function pointer",
                    )(
                    ctx,
                    offset,
                    out.offset(out_align as isize),
                    out.offset(out_align as isize),
                );
                ((*aead).decrypt)
                    .expect(
                        "non-null function pointer",
                    )(
                    ctx,
                    0 as i32 as size_t,
                    out.offset(out_align as isize),
                    0 as *const uint8_t,
                );
                ((*aead).decrypt)
                    .expect(
                        "non-null function pointer",
                    )(
                    ctx,
                    ((*cleartext).length).wrapping_sub(offset),
                    out.offset(out_align as isize).offset(offset as isize),
                    out.offset(out_align as isize).offset(offset as isize),
                );
                if memcmp(
                    out.offset(out_align as isize) as *const libc::c_void,
                    ((*cleartext).data).as_ptr() as *const libc::c_void,
                    (*cleartext).length,
                ) != 0
                {
                    fprintf(
                        stderr,
                        b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const i8,
                        b"testutils.c\0" as *const u8 as *const i8,
                        929 as i32,
                        b"MEMEQ(cleartext->length, out + out_align, cleartext->data)\0"
                            as *const u8 as *const i8,
                    );
                    abort();
                }
                if !digest.is_null() {
                    memset(
                        buffer as *mut libc::c_void,
                        0 as i32,
                        (*aead).digest_size as u64,
                    );
                    ((*aead).digest)
                        .expect(
                            "non-null function pointer",
                        )(ctx, (*digest).length, buffer);
                    if memcmp(
                        buffer as *const libc::c_void,
                        ((*digest).data).as_ptr() as *const libc::c_void,
                        (*digest).length,
                    ) != 0
                    {
                        fprintf(
                            stderr,
                            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const i8,
                            b"testutils.c\0" as *const u8 as *const i8,
                            935 as i32,
                            b"MEMEQ(digest->length, buffer, digest->data)\0" as *const u8
                                as *const i8,
                        );
                        abort();
                    }
                }
            }
            offset = (offset as u64).wrapping_add((*aead).block_size as u64) as size_t
                as size_t;
        }
        in_align = in_align.wrapping_add(1);
        in_align;
    }
    free(ctx);
    free(in_0 as *mut libc::c_void);
    free(out as *mut libc::c_void);
    free(buffer as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn test_aead_message(
    mut aead: *const nettle_aead_message,
    mut key: *const tstring,
    mut nonce: *const tstring,
    mut adata: *const tstring,
    mut clear: *const tstring,
    mut cipher: *const tstring,
) {
    let mut ctx: *mut libc::c_void = xalloc((*aead).context_size as size_t);
    let mut buf: *mut uint8_t = xalloc(((*cipher).length).wrapping_add(1 as i32 as u64))
        as *mut uint8_t;
    let mut copy: *mut uint8_t = xalloc((*cipher).length) as *mut uint8_t;
    static mut nul: uint8_t = 0 as i32 as uint8_t;
    let mut res: i32 = 0;
    if !((*key).length == (*aead).key_size as u64) {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const i8,
            b"testutils.c\0" as *const u8 as *const i8,
            961 as i32,
            b"key->length == aead->key_size\0" as *const u8 as *const i8,
        );
        abort();
    }
    if !((*cipher).length > (*clear).length) {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const i8,
            b"testutils.c\0" as *const u8 as *const i8,
            962 as i32,
            b"cipher->length > clear->length\0" as *const u8 as *const i8,
        );
        abort();
    }
    if !(((*cipher).length).wrapping_sub((*clear).length) == (*aead).digest_size as u64)
    {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const i8,
            b"testutils.c\0" as *const u8 as *const i8,
            963 as i32,
            b"cipher->length - clear->length == aead->digest_size\0" as *const u8
                as *const i8,
        );
        abort();
    }
    ((*aead).set_encrypt_key)
        .expect("non-null function pointer")(ctx, ((*key).data).as_ptr());
    *buf.offset((*cipher).length as isize) = 0xae as i32 as uint8_t;
    ((*aead).encrypt)
        .expect(
            "non-null function pointer",
        )(
        ctx,
        (*nonce).length,
        ((*nonce).data).as_ptr(),
        (*adata).length,
        ((*adata).data).as_ptr(),
        (*cipher).length,
        buf,
        ((*clear).data).as_ptr(),
    );
    if memcmp(
        ((*cipher).data).as_ptr() as *const libc::c_void,
        buf as *const libc::c_void,
        (*cipher).length,
    ) != 0
    {
        fprintf(
            stderr,
            b"aead->encrypt (message) failed:\n  got: \0" as *const u8 as *const i8,
        );
        print_hex((*cipher).length, buf);
        fprintf(stderr, b"  exp: \0" as *const u8 as *const i8);
        tstring_print_hex(cipher);
        abort();
    }
    if *buf.offset((*cipher).length as isize) as i32 != 0xae as i32 {
        fprintf(
            stderr,
            b"aead->encrypt (message) wrote too much.\n \0" as *const u8 as *const i8,
        );
        abort();
    }
    ((*aead).set_decrypt_key)
        .expect("non-null function pointer")(ctx, ((*key).data).as_ptr());
    memset(
        buf as *mut libc::c_void,
        0xae as i32,
        ((*clear).length).wrapping_add(1 as i32 as u64),
    );
    res = ((*aead).decrypt)
        .expect(
            "non-null function pointer",
        )(
        ctx,
        (*nonce).length,
        ((*nonce).data).as_ptr(),
        (*adata).length,
        ((*adata).data).as_ptr(),
        (*clear).length,
        buf,
        ((*cipher).data).as_ptr(),
    );
    if res == 0 {
        fprintf(
            stderr,
            b"decrypting valid ciphertext failed:\n  \0" as *const u8 as *const i8,
        );
        tstring_print_hex(cipher);
    }
    if memcmp(
        ((*clear).data).as_ptr() as *const libc::c_void,
        buf as *const libc::c_void,
        (*clear).length,
    ) != 0
    {
        fprintf(
            stderr,
            b"aead->decrypt (message) failed:\n  got: \0" as *const u8 as *const i8,
        );
        print_hex((*clear).length, buf);
        fprintf(stderr, b"  exp: \0" as *const u8 as *const i8);
        tstring_print_hex(clear);
        abort();
    }
    if (*clear).length > 0 as i32 as u64
        && ((*aead).decrypt)
            .expect(
                "non-null function pointer",
            )(
            ctx,
            (*nonce).length,
            ((*nonce).data).as_ptr(),
            (*adata).length,
            ((*adata).data).as_ptr(),
            ((*clear).length).wrapping_sub(1 as i32 as u64),
            buf,
            ((*cipher).data).as_ptr(),
        ) != 0
    {
        fprintf(
            stderr,
            b"Invalid message (truncated) not rejected\n\0" as *const u8 as *const i8,
        );
        abort();
    }
    memcpy(
        copy as *mut libc::c_void,
        ((*cipher).data).as_ptr() as *const libc::c_void,
        (*cipher).length,
    );
    let ref mut fresh2 = *copy.offset(0 as i32 as isize);
    *fresh2 = (*fresh2 as i32 ^ 4 as i32) as uint8_t;
    if ((*aead).decrypt)
        .expect(
            "non-null function pointer",
        )(
        ctx,
        (*nonce).length,
        ((*nonce).data).as_ptr(),
        (*adata).length,
        ((*adata).data).as_ptr(),
        (*clear).length,
        buf,
        copy,
    ) != 0
    {
        fprintf(
            stderr,
            b"Invalid message (first byte modified) not rejected\n\0" as *const u8
                as *const i8,
        );
        abort();
    }
    memcpy(
        copy as *mut libc::c_void,
        ((*cipher).data).as_ptr() as *const libc::c_void,
        (*cipher).length,
    );
    let ref mut fresh3 = *copy
        .offset(((*cipher).length).wrapping_sub(1 as i32 as u64) as isize);
    *fresh3 = (*fresh3 as i32 ^ 4 as i32) as uint8_t;
    if ((*aead).decrypt)
        .expect(
            "non-null function pointer",
        )(
        ctx,
        (*nonce).length,
        ((*nonce).data).as_ptr(),
        (*adata).length,
        ((*adata).data).as_ptr(),
        (*clear).length,
        buf,
        copy,
    ) != 0
    {
        fprintf(
            stderr,
            b"Invalid message (last byte modified) not rejected\n\0" as *const u8
                as *const i8,
        );
        abort();
    }
    if ((*aead).decrypt)
        .expect(
            "non-null function pointer",
        )(
        ctx,
        (*nonce).length,
        ((*nonce).data).as_ptr(),
        if (*adata).length > 0 as i32 as u64 {
            ((*adata).length).wrapping_sub(1 as i32 as u64)
        } else {
            1 as i32 as u64
        },
        if (*adata).length > 0 as i32 as u64 { ((*adata).data).as_ptr() } else { &nul },
        (*clear).length,
        buf,
        ((*cipher).data).as_ptr(),
    ) != 0
    {
        fprintf(stderr, b"Invalid adata not rejected\n\0" as *const u8 as *const i8);
        abort();
    }
    if (*aead).supports_inplace != 0 {
        ((*aead).set_encrypt_key)
            .expect("non-null function pointer")(ctx, ((*key).data).as_ptr());
        *buf.offset((*cipher).length as isize) = 0xae as i32 as uint8_t;
        memcpy(
            buf as *mut libc::c_void,
            ((*clear).data).as_ptr() as *const libc::c_void,
            (*clear).length,
        );
        ((*aead).encrypt)
            .expect(
                "non-null function pointer",
            )(
            ctx,
            (*nonce).length,
            ((*nonce).data).as_ptr(),
            (*adata).length,
            ((*adata).data).as_ptr(),
            (*cipher).length,
            buf,
            buf,
        );
        if memcmp(
            ((*cipher).data).as_ptr() as *const libc::c_void,
            buf as *const libc::c_void,
            (*cipher).length,
        ) != 0
        {
            fprintf(
                stderr,
                b"aead->encrypt (in-place message) failed:\n  got: \0" as *const u8
                    as *const i8,
            );
            print_hex((*cipher).length, buf);
            fprintf(stderr, b"  exp: \0" as *const u8 as *const i8);
            tstring_print_hex(cipher);
            abort();
        }
        if *buf.offset((*cipher).length as isize) as i32 != 0xae as i32 {
            fprintf(
                stderr,
                b"aead->encrypt (in-place message) wrote too much.\n \0" as *const u8
                    as *const i8,
            );
            abort();
        }
        res = ((*aead).decrypt)
            .expect(
                "non-null function pointer",
            )(
            ctx,
            (*nonce).length,
            ((*nonce).data).as_ptr(),
            (*adata).length,
            ((*adata).data).as_ptr(),
            (*clear).length,
            buf,
            buf,
        );
        if res == 0 {
            fprintf(
                stderr,
                b"in-place decrypting valid ciphertext failed:\n  \0" as *const u8
                    as *const i8,
            );
            tstring_print_hex(cipher);
        }
        if memcmp(
            ((*clear).data).as_ptr() as *const libc::c_void,
            buf as *const libc::c_void,
            (*clear).length,
        ) != 0
        {
            fprintf(
                stderr,
                b"aead->decrypt (in-place message) failed:\n  got: \0" as *const u8
                    as *const i8,
            );
            print_hex((*clear).length, buf);
            fprintf(stderr, b"  exp: \0" as *const u8 as *const i8);
            tstring_print_hex(clear);
            abort();
        }
    }
    free(ctx);
    free(buf as *mut libc::c_void);
    free(copy as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn test_hash(
    mut hash: *const nettle_hash,
    mut msg: *const tstring,
    mut digest: *const tstring,
) {
    let mut ctx: *mut libc::c_void = xalloc((*hash).context_size as size_t);
    let mut buffer: *mut uint8_t = xalloc((*digest).length) as *mut uint8_t;
    let mut input: *mut uint8_t = 0 as *mut uint8_t;
    let mut offset: u32 = 0;
    if (*hash).digest_size != 0 {
        if !((*digest).length == (*hash).digest_size as u64) {
            fprintf(
                stderr,
                b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const i8,
                b"testutils.c\0" as *const u8 as *const i8,
                1108 as i32,
                b"digest->length == hash->digest_size\0" as *const u8 as *const i8,
            );
            abort();
        }
    }
    ((*hash).init).expect("non-null function pointer")(ctx);
    offset = 0 as i32 as u32;
    while offset as u64 <= (*msg).length && offset < 40 as i32 as u32 {
        ((*hash).update)
            .expect(
                "non-null function pointer",
            )(ctx, offset as size_t, ((*msg).data).as_ptr());
        ((*hash).update)
            .expect(
                "non-null function pointer",
            )(ctx, 0 as i32 as size_t, 0 as *const uint8_t);
        ((*hash).update)
            .expect(
                "non-null function pointer",
            )(
            ctx,
            ((*msg).length).wrapping_sub(offset as u64),
            ((*msg).data).as_ptr().offset(offset as isize),
        );
        ((*hash).digest)
            .expect("non-null function pointer")(ctx, (*digest).length, buffer);
        if (memcmp(
            ((*digest).data).as_ptr() as *const libc::c_void,
            buffer as *const libc::c_void,
            (*digest).length,
        ) == 0) as i32 == 0 as i32
        {
            fprintf(stdout, b"Offset %u\nGot:\n\0" as *const u8 as *const i8, offset);
            print_hex((*digest).length, buffer);
            fprintf(stdout, b"\nExpected:\n\0" as *const u8 as *const i8);
            print_hex((*digest).length, ((*digest).data).as_ptr());
            abort();
        }
        offset = offset.wrapping_add(1);
        offset;
    }
    memset(buffer as *mut libc::c_void, 0 as i32, (*digest).length);
    ((*hash).update)
        .expect("non-null function pointer")(ctx, (*msg).length, ((*msg).data).as_ptr());
    if !((*digest).length > 0 as i32 as u64) {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const i8,
            b"testutils.c\0" as *const u8 as *const i8,
            1132 as i32,
            b"digest->length > 0\0" as *const u8 as *const i8,
        );
        abort();
    }
    ((*hash).digest)
        .expect(
            "non-null function pointer",
        )(ctx, ((*digest).length).wrapping_sub(1 as i32 as u64), buffer);
    if memcmp(
        ((*digest).data).as_ptr() as *const libc::c_void,
        buffer as *const libc::c_void,
        ((*digest).length).wrapping_sub(1 as i32 as u64),
    ) != 0
    {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const i8,
            b"testutils.c\0" as *const u8 as *const i8,
            1135 as i32,
            b"MEMEQ(digest->length - 1, digest->data, buffer)\0" as *const u8
                as *const i8,
        );
        abort();
    }
    if !(*buffer.offset(((*digest).length).wrapping_sub(1 as i32 as u64) as isize) as i32
        == 0 as i32)
    {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const i8,
            b"testutils.c\0" as *const u8 as *const i8,
            1137 as i32,
            b"buffer[digest->length - 1] == 0\0" as *const u8 as *const i8,
        );
        abort();
    }
    input = xalloc(((*msg).length).wrapping_add(16 as i32 as u64)) as *mut uint8_t;
    offset = 0 as i32 as u32;
    while offset < 16 as i32 as u32 {
        memset(
            input as *mut libc::c_void,
            0 as i32,
            ((*msg).length).wrapping_add(16 as i32 as u64),
        );
        memcpy(
            input.offset(offset as isize) as *mut libc::c_void,
            ((*msg).data).as_ptr() as *const libc::c_void,
            (*msg).length,
        );
        ((*hash).update)
            .expect(
                "non-null function pointer",
            )(ctx, (*msg).length, input.offset(offset as isize));
        ((*hash).digest)
            .expect("non-null function pointer")(ctx, (*digest).length, buffer);
        if (memcmp(
            ((*digest).data).as_ptr() as *const libc::c_void,
            buffer as *const libc::c_void,
            (*digest).length,
        ) == 0) as i32 == 0 as i32
        {
            fprintf(
                stdout,
                b"hash input address: %p\nGot:\n\0" as *const u8 as *const i8,
                input.offset(offset as isize),
            );
            print_hex((*digest).length, buffer);
            fprintf(stdout, b"\nExpected:\n\0" as *const u8 as *const i8);
            print_hex((*digest).length, ((*digest).data).as_ptr());
            abort();
        }
        offset = offset.wrapping_add(1);
        offset;
    }
    free(ctx);
    free(buffer as *mut libc::c_void);
    free(input as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn test_hash_large(
    mut hash: *const nettle_hash,
    mut count: size_t,
    mut length: size_t,
    mut c: uint8_t,
    mut digest: *const tstring,
) {
    let mut ctx: *mut libc::c_void = xalloc((*hash).context_size as size_t);
    let mut buffer: *mut uint8_t = xalloc((*hash).digest_size as size_t) as *mut uint8_t;
    let mut data: *mut uint8_t = xalloc(length) as *mut uint8_t;
    let mut i: size_t = 0;
    if !((*digest).length == (*hash).digest_size as u64) {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const i8,
            b"testutils.c\0" as *const u8 as *const i8,
            1171 as i32,
            b"digest->length == hash->digest_size\0" as *const u8 as *const i8,
        );
        abort();
    }
    memset(data as *mut libc::c_void, c as i32, length);
    ((*hash).init).expect("non-null function pointer")(ctx);
    i = 0 as i32 as size_t;
    while i < count {
        ((*hash).update).expect("non-null function pointer")(ctx, length, data);
        if i.wrapping_rem(count.wrapping_div(50 as i32 as u64)) == 0 as i32 as u64 {
            fprintf(stderr, b".\0" as *const u8 as *const i8);
        }
        i = i.wrapping_add(1);
        i;
    }
    fprintf(stderr, b"\n\0" as *const u8 as *const i8);
    ((*hash).digest)
        .expect("non-null function pointer")(ctx, (*hash).digest_size as size_t, buffer);
    print_hex((*hash).digest_size as size_t, buffer);
    if memcmp(
        ((*digest).data).as_ptr() as *const libc::c_void,
        buffer as *const libc::c_void,
        (*hash).digest_size as u64,
    ) != 0
    {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const i8,
            b"testutils.c\0" as *const u8 as *const i8,
            1188 as i32,
            b"MEMEQ(hash->digest_size, digest->data, buffer)\0" as *const u8 as *const i8,
        );
        abort();
    }
    free(ctx);
    free(buffer as *mut libc::c_void);
    free(data as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn test_mac(
    mut mac: *const nettle_mac,
    mut key: *const tstring,
    mut msg: *const tstring,
    mut digest: *const tstring,
) {
    let mut ctx: *mut libc::c_void = xalloc((*mac).context_size as size_t);
    let mut hash: *mut uint8_t = xalloc((*mac).digest_size as size_t) as *mut uint8_t;
    let mut i: u32 = 0;
    if !((*digest).length <= (*mac).digest_size as u64) {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const i8,
            b"testutils.c\0" as *const u8 as *const i8,
            1205 as i32,
            b"digest->length <= mac->digest_size\0" as *const u8 as *const i8,
        );
        abort();
    }
    if !((*key).length == (*mac).key_size as u64) {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const i8,
            b"testutils.c\0" as *const u8 as *const i8,
            1206 as i32,
            b"key->length == mac->key_size\0" as *const u8 as *const i8,
        );
        abort();
    }
    ((*mac).set_key).expect("non-null function pointer")(ctx, ((*key).data).as_ptr());
    ((*mac).update)
        .expect("non-null function pointer")(ctx, (*msg).length, ((*msg).data).as_ptr());
    ((*mac).digest).expect("non-null function pointer")(ctx, (*digest).length, hash);
    if memcmp(
        ((*digest).data).as_ptr() as *const libc::c_void,
        hash as *const libc::c_void,
        (*digest).length,
    ) != 0
    {
        fprintf(stderr, b"test_mac failed, msg: \0" as *const u8 as *const i8);
        print_hex((*msg).length, ((*msg).data).as_ptr());
        fprintf(stderr, b"Output:\0" as *const u8 as *const i8);
        print_hex((*mac).digest_size as size_t, hash);
        fprintf(stderr, b"Expected:\0" as *const u8 as *const i8);
        tstring_print_hex(digest);
        fprintf(stderr, b"\n\0" as *const u8 as *const i8);
        abort();
    }
    ((*mac).update)
        .expect("non-null function pointer")(ctx, (*msg).length, ((*msg).data).as_ptr());
    ((*mac).digest).expect("non-null function pointer")(ctx, (*digest).length, hash);
    if memcmp(
        ((*digest).data).as_ptr() as *const libc::c_void,
        hash as *const libc::c_void,
        (*digest).length,
    ) != 0
    {
        fprintf(
            stderr,
            b"test_mac: failed on re-use, msg: \0" as *const u8 as *const i8,
        );
        print_hex((*msg).length, ((*msg).data).as_ptr());
        fprintf(stderr, b"Output:\0" as *const u8 as *const i8);
        print_hex((*mac).digest_size as size_t, hash);
        fprintf(stderr, b"Expected:\0" as *const u8 as *const i8);
        tstring_print_hex(digest);
        fprintf(stderr, b"\n\0" as *const u8 as *const i8);
        abort();
    }
    ((*mac).set_key).expect("non-null function pointer")(ctx, ((*key).data).as_ptr());
    i = 0 as i32 as u32;
    while (i as u64) < (*msg).length {
        ((*mac).update)
            .expect(
                "non-null function pointer",
            )(ctx, 1 as i32 as size_t, ((*msg).data).as_ptr().offset(i as isize));
        i = i.wrapping_add(1);
        i;
    }
    ((*mac).digest).expect("non-null function pointer")(ctx, (*digest).length, hash);
    if memcmp(
        ((*digest).data).as_ptr() as *const libc::c_void,
        hash as *const libc::c_void,
        (*digest).length,
    ) != 0
    {
        fprintf(
            stderr,
            b"cmac_hash failed on byte-by-byte, msg: \0" as *const u8 as *const i8,
        );
        print_hex((*msg).length, ((*msg).data).as_ptr());
        fprintf(stderr, b"Output:\0" as *const u8 as *const i8);
        print_hex(16 as i32 as size_t, hash);
        fprintf(stderr, b"Expected:\0" as *const u8 as *const i8);
        tstring_print_hex(digest);
        fprintf(stderr, b"\n\0" as *const u8 as *const i8);
        abort();
    }
    free(ctx);
    free(hash as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn test_armor(
    mut armor: *const nettle_armor,
    mut data_length: size_t,
    mut data: *const uint8_t,
    mut ascii: *const i8,
) {
    let mut ascii_length: size_t = strlen(ascii);
    let mut buffer: *mut i8 = xalloc((1 as i32 as u64).wrapping_add(ascii_length))
        as *mut i8;
    let mut check: *mut uint8_t = xalloc(
        (1 as i32 as u64)
            .wrapping_add(
                ((*armor).decode_length)
                    .expect("non-null function pointer")(ascii_length),
            ),
    ) as *mut uint8_t;
    let mut encode: *mut libc::c_void = xalloc((*armor).encode_context_size as size_t);
    let mut decode: *mut libc::c_void = xalloc((*armor).decode_context_size as size_t);
    let mut done: size_t = 0;
    if !(ascii_length
        <= (((*armor).encode_length).expect("non-null function pointer")(data_length))
            .wrapping_add((*armor).encode_final_length as u64))
    {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const i8,
            b"testutils.c\0" as *const u8 as *const i8,
            1272 as i32,
            b"ascii_length <= (armor->encode_length(data_length) + armor->encode_final_length)\0"
                as *const u8 as *const i8,
        );
        abort();
    }
    if !(data_length
        <= ((*armor).decode_length).expect("non-null function pointer")(ascii_length))
    {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const i8,
            b"testutils.c\0" as *const u8 as *const i8,
            1273 as i32,
            b"data_length <= armor->decode_length(ascii_length)\0" as *const u8
                as *const i8,
        );
        abort();
    }
    memset(
        buffer as *mut libc::c_void,
        0x33 as i32,
        (1 as i32 as u64).wrapping_add(ascii_length),
    );
    memset(
        check as *mut libc::c_void,
        0x55 as i32,
        (1 as i32 as u64).wrapping_add(data_length),
    );
    ((*armor).encode_init).expect("non-null function pointer")(encode);
    done = ((*armor).encode_update)
        .expect("non-null function pointer")(encode, buffer, data_length, data);
    done = (done as u64)
        .wrapping_add(
            ((*armor).encode_final)
                .expect(
                    "non-null function pointer",
                )(encode, buffer.offset(done as isize)),
        ) as size_t as size_t;
    if !(done == ascii_length) {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const i8,
            b"testutils.c\0" as *const u8 as *const i8,
            1282 as i32,
            b"done == ascii_length\0" as *const u8 as *const i8,
        );
        abort();
    }
    if memcmp(buffer as *const libc::c_void, ascii as *const libc::c_void, ascii_length)
        != 0
    {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const i8,
            b"testutils.c\0" as *const u8 as *const i8,
            1284 as i32,
            b"MEMEQ(ascii_length, buffer, ascii)\0" as *const u8 as *const i8,
        );
        abort();
    }
    if !(0x33 as i32 == *buffer.offset(strlen(ascii) as isize) as i32) {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const i8,
            b"testutils.c\0" as *const u8 as *const i8,
            1285 as i32,
            b"0x33 == buffer[strlen(ascii)]\0" as *const u8 as *const i8,
        );
        abort();
    }
    ((*armor).decode_init).expect("non-null function pointer")(decode);
    done = ((*armor).decode_length).expect("non-null function pointer")(ascii_length);
    if ((*armor).decode_update)
        .expect(
            "non-null function pointer",
        )(decode, &mut done, check, ascii_length, buffer) == 0
    {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const i8,
            b"testutils.c\0" as *const u8 as *const i8,
            1290 as i32,
            b"armor->decode_update(decode, &done, check, ascii_length, buffer)\0"
                as *const u8 as *const i8,
        );
        abort();
    }
    if !(done == data_length) {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const i8,
            b"testutils.c\0" as *const u8 as *const i8,
            1291 as i32,
            b"done == data_length\0" as *const u8 as *const i8,
        );
        abort();
    }
    if ((*armor).decode_final).expect("non-null function pointer")(decode) == 0 {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const i8,
            b"testutils.c\0" as *const u8 as *const i8,
            1292 as i32,
            b"armor->decode_final(decode)\0" as *const u8 as *const i8,
        );
        abort();
    }
    if memcmp(check as *const libc::c_void, data as *const libc::c_void, data_length)
        != 0
    {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const i8,
            b"testutils.c\0" as *const u8 as *const i8,
            1294 as i32,
            b"MEMEQ(data_length, check, data)\0" as *const u8 as *const i8,
        );
        abort();
    }
    if !(0x55 as i32 == *check.offset(data_length as isize) as i32) {
        fprintf(
            stderr,
            b"Assert failed: %s:%d: %s\n\0" as *const u8 as *const i8,
            b"testutils.c\0" as *const u8 as *const i8,
            1295 as i32,
            b"0x55 == check[data_length]\0" as *const u8 as *const i8,
        );
        abort();
    }
    free(buffer as *mut libc::c_void);
    free(check as *mut libc::c_void);
    free(encode);
    free(decode);
}
#[no_mangle]
pub static mut mp_bits_per_limb: i32 = 0;
unsafe extern "C" fn gmp_die(mut msg: *const i8) {
    fprintf(stderr, b"%s\n\0" as *const u8 as *const i8, msg);
    abort();
}
unsafe extern "C" fn gmp_default_alloc(mut size: size_t) -> *mut libc::c_void {
    let mut p: *mut libc::c_void = 0 as *mut libc::c_void;
    if size > 0 as i32 as u64 {} else {
        __assert_fail(
            b"size > 0\0" as *const u8 as *const i8,
            b"../mini-gmp.c\0" as *const u8 as *const i8,
            290 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 32],
                &[i8; 32],
            >(b"void *gmp_default_alloc(size_t)\0"))
                .as_ptr(),
        );
    }
    'c_2404: {
        if size > 0 as i32 as u64 {} else {
            __assert_fail(
                b"size > 0\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                290 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 32],
                    &[i8; 32],
                >(b"void *gmp_default_alloc(size_t)\0"))
                    .as_ptr(),
            );
        }
    };
    p = malloc(size);
    if p.is_null() {
        gmp_die(
            b"gmp_default_alloc: Virtual memory exhausted.\0" as *const u8 as *const i8,
        );
    }
    return p;
}
unsafe extern "C" fn gmp_default_realloc(
    mut old: *mut libc::c_void,
    mut unused_old_size: size_t,
    mut new_size: size_t,
) -> *mut libc::c_void {
    let mut p: *mut libc::c_void = 0 as *mut libc::c_void;
    p = realloc(old, new_size);
    if p.is_null() {
        gmp_die(
            b"gmp_default_realloc: Virtual memory exhausted.\0" as *const u8 as *const i8,
        );
    }
    return p;
}
unsafe extern "C" fn gmp_default_free(
    mut p: *mut libc::c_void,
    mut unused_size: size_t,
) {
    free(p);
}
static mut gmp_allocate_func: Option<
    unsafe extern "C" fn(size_t) -> *mut libc::c_void,
> = unsafe {
    Some(gmp_default_alloc as unsafe extern "C" fn(size_t) -> *mut libc::c_void)
};
static mut gmp_reallocate_func: Option<
    unsafe extern "C" fn(*mut libc::c_void, size_t, size_t) -> *mut libc::c_void,
> = unsafe {
    Some(
        gmp_default_realloc
            as unsafe extern "C" fn(
                *mut libc::c_void,
                size_t,
                size_t,
            ) -> *mut libc::c_void,
    )
};
static mut gmp_free_func: Option<
    unsafe extern "C" fn(*mut libc::c_void, size_t) -> (),
> = unsafe {
    Some(gmp_default_free as unsafe extern "C" fn(*mut libc::c_void, size_t) -> ())
};
#[no_mangle]
pub unsafe extern "C" fn mp_get_memory_functions(
    mut alloc_func: *mut Option<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
    mut realloc_func: *mut Option<
        unsafe extern "C" fn(*mut libc::c_void, size_t, size_t) -> *mut libc::c_void,
    >,
    mut free_func: *mut Option<unsafe extern "C" fn(*mut libc::c_void, size_t) -> ()>,
) {
    if !alloc_func.is_null() {
        *alloc_func = gmp_allocate_func;
    }
    if !realloc_func.is_null() {
        *realloc_func = gmp_reallocate_func;
    }
    if !free_func.is_null() {
        *free_func = gmp_free_func;
    }
}
#[no_mangle]
pub unsafe extern "C" fn mp_set_memory_functions(
    mut alloc_func: Option<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
    mut realloc_func: Option<
        unsafe extern "C" fn(*mut libc::c_void, size_t, size_t) -> *mut libc::c_void,
    >,
    mut free_func: Option<unsafe extern "C" fn(*mut libc::c_void, size_t) -> ()>,
) {
    if alloc_func.is_none() {
        alloc_func = Some(
            gmp_default_alloc as unsafe extern "C" fn(size_t) -> *mut libc::c_void,
        );
    }
    if realloc_func.is_none() {
        realloc_func = Some(
            gmp_default_realloc
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    size_t,
                    size_t,
                ) -> *mut libc::c_void,
        );
    }
    if free_func.is_none() {
        free_func = Some(
            gmp_default_free as unsafe extern "C" fn(*mut libc::c_void, size_t) -> (),
        );
    }
    gmp_allocate_func = alloc_func;
    gmp_reallocate_func = realloc_func;
    gmp_free_func = free_func;
}
unsafe extern "C" fn gmp_xalloc_limbs(mut size: mp_size_t) -> mp_ptr {
    return (Some(gmp_allocate_func.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )((size as u64).wrapping_mul(::core::mem::size_of::<mp_limb_t>() as u64))
        as mp_ptr;
}
unsafe extern "C" fn gmp_xrealloc_limbs(mut old: mp_ptr, mut size: mp_size_t) -> mp_ptr {
    if size > 0 as i32 as i64 {} else {
        __assert_fail(
            b"size > 0\0" as *const u8 as *const i8,
            b"../mini-gmp.c\0" as *const u8 as *const i8,
            366 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 45],
                &[i8; 45],
            >(b"mp_ptr gmp_xrealloc_limbs(mp_ptr, mp_size_t)\0"))
                .as_ptr(),
        );
    }
    'c_5590: {
        if size > 0 as i32 as i64 {} else {
            __assert_fail(
                b"size > 0\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                366 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 45],
                    &[i8; 45],
                >(b"mp_ptr gmp_xrealloc_limbs(mp_ptr, mp_size_t)\0"))
                    .as_ptr(),
            );
        }
    };
    return (Some(gmp_reallocate_func.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(
        old as *mut libc::c_void,
        0 as i32 as size_t,
        (size as u64).wrapping_mul(::core::mem::size_of::<mp_limb_t>() as u64),
    ) as mp_ptr;
}
#[no_mangle]
pub unsafe extern "C" fn mpn_copyi(mut d: mp_ptr, mut s: mp_srcptr, mut n: mp_size_t) {
    let mut i: mp_size_t = 0;
    i = 0 as i32 as mp_size_t;
    while i < n {
        *d.offset(i as isize) = *s.offset(i as isize);
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn mpn_copyd(mut d: mp_ptr, mut s: mp_srcptr, mut n: mp_size_t) {
    loop {
        n -= 1;
        if !(n >= 0 as i32 as i64) {
            break;
        }
        *d.offset(n as isize) = *s.offset(n as isize);
    };
}
#[no_mangle]
pub unsafe extern "C" fn mpn_cmp(
    mut ap: mp_srcptr,
    mut bp: mp_srcptr,
    mut n: mp_size_t,
) -> i32 {
    loop {
        n -= 1;
        if !(n >= 0 as i32 as i64) {
            break;
        }
        if *ap.offset(n as isize) != *bp.offset(n as isize) {
            return if *ap.offset(n as isize) > *bp.offset(n as isize) {
                1 as i32
            } else {
                -(1 as i32)
            };
        }
    }
    return 0 as i32;
}
unsafe extern "C" fn mpn_cmp4(
    mut ap: mp_srcptr,
    mut an: mp_size_t,
    mut bp: mp_srcptr,
    mut bn: mp_size_t,
) -> i32 {
    if an != bn {
        return if an < bn { -(1 as i32) } else { 1 as i32 }
    } else {
        return mpn_cmp(ap, bp, an)
    };
}
unsafe extern "C" fn mpn_normalized_size(
    mut xp: mp_srcptr,
    mut n: mp_size_t,
) -> mp_size_t {
    while n > 0 as i32 as i64
        && *xp.offset((n - 1 as i32 as i64) as isize) == 0 as i32 as u64
    {
        n -= 1;
        n;
    }
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn mpn_zero_p(mut rp: mp_srcptr, mut n: mp_size_t) -> i32 {
    return (mpn_normalized_size(rp, n) == 0 as i32 as i64) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn mpn_zero(mut rp: mp_ptr, mut n: mp_size_t) {
    loop {
        n -= 1;
        if !(n >= 0 as i32 as i64) {
            break;
        }
        *rp.offset(n as isize) = 0 as i32 as mp_limb_t;
    };
}
#[no_mangle]
pub unsafe extern "C" fn mpn_add_1(
    mut rp: mp_ptr,
    mut ap: mp_srcptr,
    mut n: mp_size_t,
    mut b: mp_limb_t,
) -> mp_limb_t {
    let mut i: mp_size_t = 0;
    if n > 0 as i32 as i64 {} else {
        __assert_fail(
            b"n > 0\0" as *const u8 as *const i8,
            b"../mini-gmp.c\0" as *const u8 as *const i8,
            434 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 61],
                &[i8; 61],
            >(b"mp_limb_t mpn_add_1(mp_ptr, mp_srcptr, mp_size_t, mp_limb_t)\0"))
                .as_ptr(),
        );
    }
    'c_2778: {
        if n > 0 as i32 as i64 {} else {
            __assert_fail(
                b"n > 0\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                434 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 61],
                    &[i8; 61],
                >(b"mp_limb_t mpn_add_1(mp_ptr, mp_srcptr, mp_size_t, mp_limb_t)\0"))
                    .as_ptr(),
            );
        }
    };
    i = 0 as i32 as mp_size_t;
    loop {
        let mut r: mp_limb_t = (*ap.offset(i as isize)).wrapping_add(b);
        b = (r < b) as i32 as mp_limb_t;
        *rp.offset(i as isize) = r;
        i += 1;
        if !(i < n) {
            break;
        }
    }
    return b;
}
#[no_mangle]
pub unsafe extern "C" fn mpn_add_n(
    mut rp: mp_ptr,
    mut ap: mp_srcptr,
    mut bp: mp_srcptr,
    mut n: mp_size_t,
) -> mp_limb_t {
    let mut i: mp_size_t = 0;
    let mut cy: mp_limb_t = 0;
    i = 0 as i32 as mp_size_t;
    cy = 0 as i32 as mp_limb_t;
    while i < n {
        let mut a: mp_limb_t = 0;
        let mut b: mp_limb_t = 0;
        let mut r: mp_limb_t = 0;
        a = *ap.offset(i as isize);
        b = *bp.offset(i as isize);
        r = a.wrapping_add(cy);
        cy = (r < cy) as i32 as mp_limb_t;
        r = (r as u64).wrapping_add(b) as mp_limb_t as mp_limb_t;
        cy = (cy as u64).wrapping_add((r < b) as i32 as u64) as mp_limb_t as mp_limb_t;
        *rp.offset(i as isize) = r;
        i += 1;
        i;
    }
    return cy;
}
#[no_mangle]
pub unsafe extern "C" fn mpn_add(
    mut rp: mp_ptr,
    mut ap: mp_srcptr,
    mut an: mp_size_t,
    mut bp: mp_srcptr,
    mut bn: mp_size_t,
) -> mp_limb_t {
    let mut cy: mp_limb_t = 0;
    if an >= bn {} else {
        __assert_fail(
            b"an >= bn\0" as *const u8 as *const i8,
            b"../mini-gmp.c\0" as *const u8 as *const i8,
            472 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 70],
                &[i8; 70],
            >(
                b"mp_limb_t mpn_add(mp_ptr, mp_srcptr, mp_size_t, mp_srcptr, mp_size_t)\0",
            ))
                .as_ptr(),
        );
    }
    'c_2974: {
        if an >= bn {} else {
            __assert_fail(
                b"an >= bn\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                472 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 70],
                    &[i8; 70],
                >(
                    b"mp_limb_t mpn_add(mp_ptr, mp_srcptr, mp_size_t, mp_srcptr, mp_size_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    cy = mpn_add_n(rp, ap, bp, bn);
    if an > bn {
        cy = mpn_add_1(rp.offset(bn as isize), ap.offset(bn as isize), an - bn, cy);
    }
    return cy;
}
#[no_mangle]
pub unsafe extern "C" fn mpn_sub_1(
    mut rp: mp_ptr,
    mut ap: mp_srcptr,
    mut n: mp_size_t,
    mut b: mp_limb_t,
) -> mp_limb_t {
    let mut i: mp_size_t = 0;
    if n > 0 as i32 as i64 {} else {
        __assert_fail(
            b"n > 0\0" as *const u8 as *const i8,
            b"../mini-gmp.c\0" as *const u8 as *const i8,
            485 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 61],
                &[i8; 61],
            >(b"mp_limb_t mpn_sub_1(mp_ptr, mp_srcptr, mp_size_t, mp_limb_t)\0"))
                .as_ptr(),
        );
    }
    'c_3070: {
        if n > 0 as i32 as i64 {} else {
            __assert_fail(
                b"n > 0\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                485 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 61],
                    &[i8; 61],
                >(b"mp_limb_t mpn_sub_1(mp_ptr, mp_srcptr, mp_size_t, mp_limb_t)\0"))
                    .as_ptr(),
            );
        }
    };
    i = 0 as i32 as mp_size_t;
    loop {
        let mut a: mp_limb_t = *ap.offset(i as isize);
        let mut cy: mp_limb_t = (a < b) as i32 as mp_limb_t;
        *rp.offset(i as isize) = a.wrapping_sub(b);
        b = cy;
        i += 1;
        if !(i < n) {
            break;
        }
    }
    return b;
}
#[no_mangle]
pub unsafe extern "C" fn mpn_sub_n(
    mut rp: mp_ptr,
    mut ap: mp_srcptr,
    mut bp: mp_srcptr,
    mut n: mp_size_t,
) -> mp_limb_t {
    let mut i: mp_size_t = 0;
    let mut cy: mp_limb_t = 0;
    i = 0 as i32 as mp_size_t;
    cy = 0 as i32 as mp_limb_t;
    while i < n {
        let mut a: mp_limb_t = 0;
        let mut b: mp_limb_t = 0;
        a = *ap.offset(i as isize);
        b = *bp.offset(i as isize);
        b = (b as u64).wrapping_add(cy) as mp_limb_t as mp_limb_t;
        cy = (b < cy) as i32 as mp_limb_t;
        cy = (cy as u64).wrapping_add((a < b) as i32 as u64) as mp_limb_t as mp_limb_t;
        *rp.offset(i as isize) = a.wrapping_sub(b);
        i += 1;
        i;
    }
    return cy;
}
#[no_mangle]
pub unsafe extern "C" fn mpn_sub(
    mut rp: mp_ptr,
    mut ap: mp_srcptr,
    mut an: mp_size_t,
    mut bp: mp_srcptr,
    mut bn: mp_size_t,
) -> mp_limb_t {
    let mut cy: mp_limb_t = 0;
    if an >= bn {} else {
        __assert_fail(
            b"an >= bn\0" as *const u8 as *const i8,
            b"../mini-gmp.c\0" as *const u8 as *const i8,
            524 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 70],
                &[i8; 70],
            >(
                b"mp_limb_t mpn_sub(mp_ptr, mp_srcptr, mp_size_t, mp_srcptr, mp_size_t)\0",
            ))
                .as_ptr(),
        );
    }
    'c_3254: {
        if an >= bn {} else {
            __assert_fail(
                b"an >= bn\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                524 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 70],
                    &[i8; 70],
                >(
                    b"mp_limb_t mpn_sub(mp_ptr, mp_srcptr, mp_size_t, mp_srcptr, mp_size_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    cy = mpn_sub_n(rp, ap, bp, bn);
    if an > bn {
        cy = mpn_sub_1(rp.offset(bn as isize), ap.offset(bn as isize), an - bn, cy);
    }
    return cy;
}
#[no_mangle]
pub unsafe extern "C" fn mpn_mul_1(
    mut rp: mp_ptr,
    mut up: mp_srcptr,
    mut n: mp_size_t,
    mut vl: mp_limb_t,
) -> mp_limb_t {
    let mut ul: mp_limb_t = 0;
    let mut cl: mp_limb_t = 0;
    let mut hpl: mp_limb_t = 0;
    let mut lpl: mp_limb_t = 0;
    if n >= 1 as i32 as i64 {} else {
        __assert_fail(
            b"n >= 1\0" as *const u8 as *const i8,
            b"../mini-gmp.c\0" as *const u8 as *const i8,
            537 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 61],
                &[i8; 61],
            >(b"mp_limb_t mpn_mul_1(mp_ptr, mp_srcptr, mp_size_t, mp_limb_t)\0"))
                .as_ptr(),
        );
    }
    'c_3691: {
        if n >= 1 as i32 as i64 {} else {
            __assert_fail(
                b"n >= 1\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                537 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 61],
                    &[i8; 61],
                >(b"mp_limb_t mpn_mul_1(mp_ptr, mp_srcptr, mp_size_t, mp_limb_t)\0"))
                    .as_ptr(),
            );
        }
    };
    cl = 0 as i32 as mp_limb_t;
    loop {
        let fresh4 = up;
        up = up.offset(1);
        ul = *fresh4;
        let mut LOCAL_GMP_LIMB_BITS: i32 = (::core::mem::size_of::<mp_limb_t>() as u64)
            .wrapping_mul(8 as i32 as u64) as i32;
        if (::core::mem::size_of::<u32>() as u64).wrapping_mul(8 as i32 as u64)
            >= (2 as i32 as u64)
                .wrapping_mul(
                    (::core::mem::size_of::<mp_limb_t>() as u64)
                        .wrapping_mul(8 as i32 as u64),
                )
        {
            let mut __ww: u32 = (ul as u32 as u64).wrapping_mul(vl) as u32;
            lpl = __ww as mp_limb_t;
            hpl = (__ww >> LOCAL_GMP_LIMB_BITS) as mp_limb_t;
        } else if (::core::mem::size_of::<u64>() as u64).wrapping_mul(8 as i32 as u64)
            >= (2 as i32 as u64)
                .wrapping_mul(
                    (::core::mem::size_of::<mp_limb_t>() as u64)
                        .wrapping_mul(8 as i32 as u64),
                )
        {
            let mut __ww_0: u64 = ul.wrapping_mul(vl);
            lpl = __ww_0;
            hpl = __ww_0 >> LOCAL_GMP_LIMB_BITS;
        } else {
            let mut __x0: mp_limb_t = 0;
            let mut __x1: mp_limb_t = 0;
            let mut __x2: mp_limb_t = 0;
            let mut __x3: mp_limb_t = 0;
            let mut __ul: u32 = 0;
            let mut __vl: u32 = 0;
            let mut __uh: u32 = 0;
            let mut __vh: u32 = 0;
            let mut __u: mp_limb_t = ul;
            let mut __v: mp_limb_t = vl;
            __ul = (__u
                & ((1 as i32 as mp_limb_t)
                    << (::core::mem::size_of::<mp_limb_t>() as u64)
                        .wrapping_mul(8 as i32 as u64)
                        .wrapping_div(2 as i32 as u64))
                    .wrapping_sub(1 as i32 as u64)) as u32;
            __uh = (__u
                >> (::core::mem::size_of::<mp_limb_t>() as u64)
                    .wrapping_mul(8 as i32 as u64)
                    .wrapping_div(2 as i32 as u64)) as u32;
            __vl = (__v
                & ((1 as i32 as mp_limb_t)
                    << (::core::mem::size_of::<mp_limb_t>() as u64)
                        .wrapping_mul(8 as i32 as u64)
                        .wrapping_div(2 as i32 as u64))
                    .wrapping_sub(1 as i32 as u64)) as u32;
            __vh = (__v
                >> (::core::mem::size_of::<mp_limb_t>() as u64)
                    .wrapping_mul(8 as i32 as u64)
                    .wrapping_div(2 as i32 as u64)) as u32;
            __x0 = (__ul as mp_limb_t).wrapping_mul(__vl as u64);
            __x1 = (__ul as mp_limb_t).wrapping_mul(__vh as u64);
            __x2 = (__uh as mp_limb_t).wrapping_mul(__vl as u64);
            __x3 = (__uh as mp_limb_t).wrapping_mul(__vh as u64);
            __x1 = (__x1 as u64)
                .wrapping_add(
                    __x0
                        >> (::core::mem::size_of::<mp_limb_t>() as u64)
                            .wrapping_mul(8 as i32 as u64)
                            .wrapping_div(2 as i32 as u64),
                ) as mp_limb_t as mp_limb_t;
            __x1 = (__x1 as u64).wrapping_add(__x2) as mp_limb_t as mp_limb_t;
            if __x1 < __x2 {
                __x3 = (__x3 as u64)
                    .wrapping_add(
                        (1 as i32 as mp_limb_t)
                            << (::core::mem::size_of::<mp_limb_t>() as u64)
                                .wrapping_mul(8 as i32 as u64)
                                .wrapping_div(2 as i32 as u64),
                    ) as mp_limb_t as mp_limb_t;
            }
            hpl = __x3
                .wrapping_add(
                    __x1
                        >> (::core::mem::size_of::<mp_limb_t>() as u64)
                            .wrapping_mul(8 as i32 as u64)
                            .wrapping_div(2 as i32 as u64),
                );
            lpl = (__x1
                << (::core::mem::size_of::<mp_limb_t>() as u64)
                    .wrapping_mul(8 as i32 as u64)
                    .wrapping_div(2 as i32 as u64))
                .wrapping_add(
                    __x0
                        & ((1 as i32 as mp_limb_t)
                            << (::core::mem::size_of::<mp_limb_t>() as u64)
                                .wrapping_mul(8 as i32 as u64)
                                .wrapping_div(2 as i32 as u64))
                            .wrapping_sub(1 as i32 as u64),
                );
        }
        lpl = (lpl as u64).wrapping_add(cl) as mp_limb_t as mp_limb_t;
        cl = ((lpl < cl) as i32 as u64).wrapping_add(hpl);
        let fresh5 = rp;
        rp = rp.offset(1);
        *fresh5 = lpl;
        n -= 1;
        if !(n != 0 as i32 as i64) {
            break;
        }
    }
    return cl;
}
#[no_mangle]
pub unsafe extern "C" fn mpn_addmul_1(
    mut rp: mp_ptr,
    mut up: mp_srcptr,
    mut n: mp_size_t,
    mut vl: mp_limb_t,
) -> mp_limb_t {
    let mut ul: mp_limb_t = 0;
    let mut cl: mp_limb_t = 0;
    let mut hpl: mp_limb_t = 0;
    let mut lpl: mp_limb_t = 0;
    let mut rl: mp_limb_t = 0;
    if n >= 1 as i32 as i64 {} else {
        __assert_fail(
            b"n >= 1\0" as *const u8 as *const i8,
            b"../mini-gmp.c\0" as *const u8 as *const i8,
            560 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 64],
                &[i8; 64],
            >(b"mp_limb_t mpn_addmul_1(mp_ptr, mp_srcptr, mp_size_t, mp_limb_t)\0"))
                .as_ptr(),
        );
    }
    'c_4151: {
        if n >= 1 as i32 as i64 {} else {
            __assert_fail(
                b"n >= 1\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                560 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 64],
                    &[i8; 64],
                >(b"mp_limb_t mpn_addmul_1(mp_ptr, mp_srcptr, mp_size_t, mp_limb_t)\0"))
                    .as_ptr(),
            );
        }
    };
    cl = 0 as i32 as mp_limb_t;
    loop {
        let fresh6 = up;
        up = up.offset(1);
        ul = *fresh6;
        let mut LOCAL_GMP_LIMB_BITS: i32 = (::core::mem::size_of::<mp_limb_t>() as u64)
            .wrapping_mul(8 as i32 as u64) as i32;
        if (::core::mem::size_of::<u32>() as u64).wrapping_mul(8 as i32 as u64)
            >= (2 as i32 as u64)
                .wrapping_mul(
                    (::core::mem::size_of::<mp_limb_t>() as u64)
                        .wrapping_mul(8 as i32 as u64),
                )
        {
            let mut __ww: u32 = (ul as u32 as u64).wrapping_mul(vl) as u32;
            lpl = __ww as mp_limb_t;
            hpl = (__ww >> LOCAL_GMP_LIMB_BITS) as mp_limb_t;
        } else if (::core::mem::size_of::<u64>() as u64).wrapping_mul(8 as i32 as u64)
            >= (2 as i32 as u64)
                .wrapping_mul(
                    (::core::mem::size_of::<mp_limb_t>() as u64)
                        .wrapping_mul(8 as i32 as u64),
                )
        {
            let mut __ww_0: u64 = ul.wrapping_mul(vl);
            lpl = __ww_0;
            hpl = __ww_0 >> LOCAL_GMP_LIMB_BITS;
        } else {
            let mut __x0: mp_limb_t = 0;
            let mut __x1: mp_limb_t = 0;
            let mut __x2: mp_limb_t = 0;
            let mut __x3: mp_limb_t = 0;
            let mut __ul: u32 = 0;
            let mut __vl: u32 = 0;
            let mut __uh: u32 = 0;
            let mut __vh: u32 = 0;
            let mut __u: mp_limb_t = ul;
            let mut __v: mp_limb_t = vl;
            __ul = (__u
                & ((1 as i32 as mp_limb_t)
                    << (::core::mem::size_of::<mp_limb_t>() as u64)
                        .wrapping_mul(8 as i32 as u64)
                        .wrapping_div(2 as i32 as u64))
                    .wrapping_sub(1 as i32 as u64)) as u32;
            __uh = (__u
                >> (::core::mem::size_of::<mp_limb_t>() as u64)
                    .wrapping_mul(8 as i32 as u64)
                    .wrapping_div(2 as i32 as u64)) as u32;
            __vl = (__v
                & ((1 as i32 as mp_limb_t)
                    << (::core::mem::size_of::<mp_limb_t>() as u64)
                        .wrapping_mul(8 as i32 as u64)
                        .wrapping_div(2 as i32 as u64))
                    .wrapping_sub(1 as i32 as u64)) as u32;
            __vh = (__v
                >> (::core::mem::size_of::<mp_limb_t>() as u64)
                    .wrapping_mul(8 as i32 as u64)
                    .wrapping_div(2 as i32 as u64)) as u32;
            __x0 = (__ul as mp_limb_t).wrapping_mul(__vl as u64);
            __x1 = (__ul as mp_limb_t).wrapping_mul(__vh as u64);
            __x2 = (__uh as mp_limb_t).wrapping_mul(__vl as u64);
            __x3 = (__uh as mp_limb_t).wrapping_mul(__vh as u64);
            __x1 = (__x1 as u64)
                .wrapping_add(
                    __x0
                        >> (::core::mem::size_of::<mp_limb_t>() as u64)
                            .wrapping_mul(8 as i32 as u64)
                            .wrapping_div(2 as i32 as u64),
                ) as mp_limb_t as mp_limb_t;
            __x1 = (__x1 as u64).wrapping_add(__x2) as mp_limb_t as mp_limb_t;
            if __x1 < __x2 {
                __x3 = (__x3 as u64)
                    .wrapping_add(
                        (1 as i32 as mp_limb_t)
                            << (::core::mem::size_of::<mp_limb_t>() as u64)
                                .wrapping_mul(8 as i32 as u64)
                                .wrapping_div(2 as i32 as u64),
                    ) as mp_limb_t as mp_limb_t;
            }
            hpl = __x3
                .wrapping_add(
                    __x1
                        >> (::core::mem::size_of::<mp_limb_t>() as u64)
                            .wrapping_mul(8 as i32 as u64)
                            .wrapping_div(2 as i32 as u64),
                );
            lpl = (__x1
                << (::core::mem::size_of::<mp_limb_t>() as u64)
                    .wrapping_mul(8 as i32 as u64)
                    .wrapping_div(2 as i32 as u64))
                .wrapping_add(
                    __x0
                        & ((1 as i32 as mp_limb_t)
                            << (::core::mem::size_of::<mp_limb_t>() as u64)
                                .wrapping_mul(8 as i32 as u64)
                                .wrapping_div(2 as i32 as u64))
                            .wrapping_sub(1 as i32 as u64),
                );
        }
        lpl = (lpl as u64).wrapping_add(cl) as mp_limb_t as mp_limb_t;
        cl = ((lpl < cl) as i32 as u64).wrapping_add(hpl);
        rl = *rp;
        lpl = rl.wrapping_add(lpl);
        cl = (cl as u64).wrapping_add((lpl < rl) as i32 as u64) as mp_limb_t
            as mp_limb_t;
        let fresh7 = rp;
        rp = rp.offset(1);
        *fresh7 = lpl;
        n -= 1;
        if !(n != 0 as i32 as i64) {
            break;
        }
    }
    return cl;
}
#[no_mangle]
pub unsafe extern "C" fn mpn_submul_1(
    mut rp: mp_ptr,
    mut up: mp_srcptr,
    mut n: mp_size_t,
    mut vl: mp_limb_t,
) -> mp_limb_t {
    let mut ul: mp_limb_t = 0;
    let mut cl: mp_limb_t = 0;
    let mut hpl: mp_limb_t = 0;
    let mut lpl: mp_limb_t = 0;
    let mut rl: mp_limb_t = 0;
    if n >= 1 as i32 as i64 {} else {
        __assert_fail(
            b"n >= 1\0" as *const u8 as *const i8,
            b"../mini-gmp.c\0" as *const u8 as *const i8,
            586 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 64],
                &[i8; 64],
            >(b"mp_limb_t mpn_submul_1(mp_ptr, mp_srcptr, mp_size_t, mp_limb_t)\0"))
                .as_ptr(),
        );
    }
    'c_4611: {
        if n >= 1 as i32 as i64 {} else {
            __assert_fail(
                b"n >= 1\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                586 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 64],
                    &[i8; 64],
                >(b"mp_limb_t mpn_submul_1(mp_ptr, mp_srcptr, mp_size_t, mp_limb_t)\0"))
                    .as_ptr(),
            );
        }
    };
    cl = 0 as i32 as mp_limb_t;
    loop {
        let fresh8 = up;
        up = up.offset(1);
        ul = *fresh8;
        let mut LOCAL_GMP_LIMB_BITS: i32 = (::core::mem::size_of::<mp_limb_t>() as u64)
            .wrapping_mul(8 as i32 as u64) as i32;
        if (::core::mem::size_of::<u32>() as u64).wrapping_mul(8 as i32 as u64)
            >= (2 as i32 as u64)
                .wrapping_mul(
                    (::core::mem::size_of::<mp_limb_t>() as u64)
                        .wrapping_mul(8 as i32 as u64),
                )
        {
            let mut __ww: u32 = (ul as u32 as u64).wrapping_mul(vl) as u32;
            lpl = __ww as mp_limb_t;
            hpl = (__ww >> LOCAL_GMP_LIMB_BITS) as mp_limb_t;
        } else if (::core::mem::size_of::<u64>() as u64).wrapping_mul(8 as i32 as u64)
            >= (2 as i32 as u64)
                .wrapping_mul(
                    (::core::mem::size_of::<mp_limb_t>() as u64)
                        .wrapping_mul(8 as i32 as u64),
                )
        {
            let mut __ww_0: u64 = ul.wrapping_mul(vl);
            lpl = __ww_0;
            hpl = __ww_0 >> LOCAL_GMP_LIMB_BITS;
        } else {
            let mut __x0: mp_limb_t = 0;
            let mut __x1: mp_limb_t = 0;
            let mut __x2: mp_limb_t = 0;
            let mut __x3: mp_limb_t = 0;
            let mut __ul: u32 = 0;
            let mut __vl: u32 = 0;
            let mut __uh: u32 = 0;
            let mut __vh: u32 = 0;
            let mut __u: mp_limb_t = ul;
            let mut __v: mp_limb_t = vl;
            __ul = (__u
                & ((1 as i32 as mp_limb_t)
                    << (::core::mem::size_of::<mp_limb_t>() as u64)
                        .wrapping_mul(8 as i32 as u64)
                        .wrapping_div(2 as i32 as u64))
                    .wrapping_sub(1 as i32 as u64)) as u32;
            __uh = (__u
                >> (::core::mem::size_of::<mp_limb_t>() as u64)
                    .wrapping_mul(8 as i32 as u64)
                    .wrapping_div(2 as i32 as u64)) as u32;
            __vl = (__v
                & ((1 as i32 as mp_limb_t)
                    << (::core::mem::size_of::<mp_limb_t>() as u64)
                        .wrapping_mul(8 as i32 as u64)
                        .wrapping_div(2 as i32 as u64))
                    .wrapping_sub(1 as i32 as u64)) as u32;
            __vh = (__v
                >> (::core::mem::size_of::<mp_limb_t>() as u64)
                    .wrapping_mul(8 as i32 as u64)
                    .wrapping_div(2 as i32 as u64)) as u32;
            __x0 = (__ul as mp_limb_t).wrapping_mul(__vl as u64);
            __x1 = (__ul as mp_limb_t).wrapping_mul(__vh as u64);
            __x2 = (__uh as mp_limb_t).wrapping_mul(__vl as u64);
            __x3 = (__uh as mp_limb_t).wrapping_mul(__vh as u64);
            __x1 = (__x1 as u64)
                .wrapping_add(
                    __x0
                        >> (::core::mem::size_of::<mp_limb_t>() as u64)
                            .wrapping_mul(8 as i32 as u64)
                            .wrapping_div(2 as i32 as u64),
                ) as mp_limb_t as mp_limb_t;
            __x1 = (__x1 as u64).wrapping_add(__x2) as mp_limb_t as mp_limb_t;
            if __x1 < __x2 {
                __x3 = (__x3 as u64)
                    .wrapping_add(
                        (1 as i32 as mp_limb_t)
                            << (::core::mem::size_of::<mp_limb_t>() as u64)
                                .wrapping_mul(8 as i32 as u64)
                                .wrapping_div(2 as i32 as u64),
                    ) as mp_limb_t as mp_limb_t;
            }
            hpl = __x3
                .wrapping_add(
                    __x1
                        >> (::core::mem::size_of::<mp_limb_t>() as u64)
                            .wrapping_mul(8 as i32 as u64)
                            .wrapping_div(2 as i32 as u64),
                );
            lpl = (__x1
                << (::core::mem::size_of::<mp_limb_t>() as u64)
                    .wrapping_mul(8 as i32 as u64)
                    .wrapping_div(2 as i32 as u64))
                .wrapping_add(
                    __x0
                        & ((1 as i32 as mp_limb_t)
                            << (::core::mem::size_of::<mp_limb_t>() as u64)
                                .wrapping_mul(8 as i32 as u64)
                                .wrapping_div(2 as i32 as u64))
                            .wrapping_sub(1 as i32 as u64),
                );
        }
        lpl = (lpl as u64).wrapping_add(cl) as mp_limb_t as mp_limb_t;
        cl = ((lpl < cl) as i32 as u64).wrapping_add(hpl);
        rl = *rp;
        lpl = rl.wrapping_sub(lpl);
        cl = (cl as u64).wrapping_add((lpl > rl) as i32 as u64) as mp_limb_t
            as mp_limb_t;
        let fresh9 = rp;
        rp = rp.offset(1);
        *fresh9 = lpl;
        n -= 1;
        if !(n != 0 as i32 as i64) {
            break;
        }
    }
    return cl;
}
#[no_mangle]
pub unsafe extern "C" fn mpn_mul(
    mut rp: mp_ptr,
    mut up: mp_srcptr,
    mut un: mp_size_t,
    mut vp: mp_srcptr,
    mut vn: mp_size_t,
) -> mp_limb_t {
    if un >= vn {} else {
        __assert_fail(
            b"un >= vn\0" as *const u8 as *const i8,
            b"../mini-gmp.c\0" as *const u8 as *const i8,
            610 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 70],
                &[i8; 70],
            >(
                b"mp_limb_t mpn_mul(mp_ptr, mp_srcptr, mp_size_t, mp_srcptr, mp_size_t)\0",
            ))
                .as_ptr(),
        );
    }
    'c_4940: {
        if un >= vn {} else {
            __assert_fail(
                b"un >= vn\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                610 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 70],
                    &[i8; 70],
                >(
                    b"mp_limb_t mpn_mul(mp_ptr, mp_srcptr, mp_size_t, mp_srcptr, mp_size_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if vn >= 1 as i32 as i64 {} else {
        __assert_fail(
            b"vn >= 1\0" as *const u8 as *const i8,
            b"../mini-gmp.c\0" as *const u8 as *const i8,
            611 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 70],
                &[i8; 70],
            >(
                b"mp_limb_t mpn_mul(mp_ptr, mp_srcptr, mp_size_t, mp_srcptr, mp_size_t)\0",
            ))
                .as_ptr(),
        );
    }
    'c_4901: {
        if vn >= 1 as i32 as i64 {} else {
            __assert_fail(
                b"vn >= 1\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                611 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 70],
                    &[i8; 70],
                >(
                    b"mp_limb_t mpn_mul(mp_ptr, mp_srcptr, mp_size_t, mp_srcptr, mp_size_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if !(rp.offset((un + vn) as isize) > up as mp_ptr
        && up.offset(un as isize) > rp as mp_srcptr)
    {} else {
        __assert_fail(
            b"!GMP_MPN_OVERLAP_P(rp, un + vn, up, un)\0" as *const u8 as *const i8,
            b"../mini-gmp.c\0" as *const u8 as *const i8,
            612 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 70],
                &[i8; 70],
            >(
                b"mp_limb_t mpn_mul(mp_ptr, mp_srcptr, mp_size_t, mp_srcptr, mp_size_t)\0",
            ))
                .as_ptr(),
        );
    }
    'c_4813: {
        if !(rp.offset((un + vn) as isize) > up as mp_ptr
            && up.offset(un as isize) > rp as mp_srcptr)
        {} else {
            __assert_fail(
                b"!GMP_MPN_OVERLAP_P(rp, un + vn, up, un)\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                612 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 70],
                    &[i8; 70],
                >(
                    b"mp_limb_t mpn_mul(mp_ptr, mp_srcptr, mp_size_t, mp_srcptr, mp_size_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if !(rp.offset((un + vn) as isize) > vp as mp_ptr
        && vp.offset(vn as isize) > rp as mp_srcptr)
    {} else {
        __assert_fail(
            b"!GMP_MPN_OVERLAP_P(rp, un + vn, vp, vn)\0" as *const u8 as *const i8,
            b"../mini-gmp.c\0" as *const u8 as *const i8,
            613 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 70],
                &[i8; 70],
            >(
                b"mp_limb_t mpn_mul(mp_ptr, mp_srcptr, mp_size_t, mp_srcptr, mp_size_t)\0",
            ))
                .as_ptr(),
        );
    }
    'c_4724: {
        if !(rp.offset((un + vn) as isize) > vp as mp_ptr
            && vp.offset(vn as isize) > rp as mp_srcptr)
        {} else {
            __assert_fail(
                b"!GMP_MPN_OVERLAP_P(rp, un + vn, vp, vn)\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                613 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 70],
                    &[i8; 70],
                >(
                    b"mp_limb_t mpn_mul(mp_ptr, mp_srcptr, mp_size_t, mp_srcptr, mp_size_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    *rp.offset(un as isize) = mpn_mul_1(rp, up, un, *vp.offset(0 as i32 as isize));
    loop {
        vn -= 1;
        if !(vn >= 1 as i32 as i64) {
            break;
        }
        rp = rp.offset(1 as i32 as isize);
        vp = vp.offset(1 as i32 as isize);
        *rp.offset(un as isize) = mpn_addmul_1(
            rp,
            up,
            un,
            *vp.offset(0 as i32 as isize),
        );
    }
    return *rp.offset(un as isize);
}
#[no_mangle]
pub unsafe extern "C" fn mpn_mul_n(
    mut rp: mp_ptr,
    mut ap: mp_srcptr,
    mut bp: mp_srcptr,
    mut n: mp_size_t,
) {
    mpn_mul(rp, ap, n, bp, n);
}
#[no_mangle]
pub unsafe extern "C" fn mpn_sqr(mut rp: mp_ptr, mut ap: mp_srcptr, mut n: mp_size_t) {
    mpn_mul(rp, ap, n, ap, n);
}
#[no_mangle]
pub unsafe extern "C" fn mpn_lshift(
    mut rp: mp_ptr,
    mut up: mp_srcptr,
    mut n: mp_size_t,
    mut cnt: u32,
) -> mp_limb_t {
    let mut high_limb: mp_limb_t = 0;
    let mut low_limb: mp_limb_t = 0;
    let mut tnc: u32 = 0;
    let mut retval: mp_limb_t = 0;
    if n >= 1 as i32 as i64 {} else {
        __assert_fail(
            b"n >= 1\0" as *const u8 as *const i8,
            b"../mini-gmp.c\0" as *const u8 as *const i8,
            651 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 65],
                &[i8; 65],
            >(b"mp_limb_t mpn_lshift(mp_ptr, mp_srcptr, mp_size_t, unsigned int)\0"))
                .as_ptr(),
        );
    }
    'c_9540: {
        if n >= 1 as i32 as i64 {} else {
            __assert_fail(
                b"n >= 1\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                651 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 65],
                    &[i8; 65],
                >(b"mp_limb_t mpn_lshift(mp_ptr, mp_srcptr, mp_size_t, unsigned int)\0"))
                    .as_ptr(),
            );
        }
    };
    if cnt >= 1 as i32 as u32 {} else {
        __assert_fail(
            b"cnt >= 1\0" as *const u8 as *const i8,
            b"../mini-gmp.c\0" as *const u8 as *const i8,
            652 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 65],
                &[i8; 65],
            >(b"mp_limb_t mpn_lshift(mp_ptr, mp_srcptr, mp_size_t, unsigned int)\0"))
                .as_ptr(),
        );
    }
    'c_9502: {
        if cnt >= 1 as i32 as u32 {} else {
            __assert_fail(
                b"cnt >= 1\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                652 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 65],
                    &[i8; 65],
                >(b"mp_limb_t mpn_lshift(mp_ptr, mp_srcptr, mp_size_t, unsigned int)\0"))
                    .as_ptr(),
            );
        }
    };
    if (cnt as u64)
        < (::core::mem::size_of::<mp_limb_t>() as u64).wrapping_mul(8 as i32 as u64)
    {} else {
        __assert_fail(
            b"cnt < GMP_LIMB_BITS\0" as *const u8 as *const i8,
            b"../mini-gmp.c\0" as *const u8 as *const i8,
            653 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 65],
                &[i8; 65],
            >(b"mp_limb_t mpn_lshift(mp_ptr, mp_srcptr, mp_size_t, unsigned int)\0"))
                .as_ptr(),
        );
    }
    'c_9456: {
        if (cnt as u64)
            < (::core::mem::size_of::<mp_limb_t>() as u64).wrapping_mul(8 as i32 as u64)
        {} else {
            __assert_fail(
                b"cnt < GMP_LIMB_BITS\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                653 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 65],
                    &[i8; 65],
                >(b"mp_limb_t mpn_lshift(mp_ptr, mp_srcptr, mp_size_t, unsigned int)\0"))
                    .as_ptr(),
            );
        }
    };
    up = up.offset(n as isize);
    rp = rp.offset(n as isize);
    tnc = (::core::mem::size_of::<mp_limb_t>() as u64)
        .wrapping_mul(8 as i32 as u64)
        .wrapping_sub(cnt as u64) as u32;
    up = up.offset(-1);
    low_limb = *up;
    retval = low_limb >> tnc;
    high_limb = low_limb << cnt;
    loop {
        n -= 1;
        if !(n != 0 as i32 as i64) {
            break;
        }
        up = up.offset(-1);
        low_limb = *up;
        rp = rp.offset(-1);
        *rp = high_limb | low_limb >> tnc;
        high_limb = low_limb << cnt;
    }
    rp = rp.offset(-1);
    *rp = high_limb;
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn mpn_rshift(
    mut rp: mp_ptr,
    mut up: mp_srcptr,
    mut n: mp_size_t,
    mut cnt: u32,
) -> mp_limb_t {
    let mut high_limb: mp_limb_t = 0;
    let mut low_limb: mp_limb_t = 0;
    let mut tnc: u32 = 0;
    let mut retval: mp_limb_t = 0;
    if n >= 1 as i32 as i64 {} else {
        __assert_fail(
            b"n >= 1\0" as *const u8 as *const i8,
            b"../mini-gmp.c\0" as *const u8 as *const i8,
            681 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 65],
                &[i8; 65],
            >(b"mp_limb_t mpn_rshift(mp_ptr, mp_srcptr, mp_size_t, unsigned int)\0"))
                .as_ptr(),
        );
    }
    'c_7817: {
        if n >= 1 as i32 as i64 {} else {
            __assert_fail(
                b"n >= 1\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                681 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 65],
                    &[i8; 65],
                >(b"mp_limb_t mpn_rshift(mp_ptr, mp_srcptr, mp_size_t, unsigned int)\0"))
                    .as_ptr(),
            );
        }
    };
    if cnt >= 1 as i32 as u32 {} else {
        __assert_fail(
            b"cnt >= 1\0" as *const u8 as *const i8,
            b"../mini-gmp.c\0" as *const u8 as *const i8,
            682 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 65],
                &[i8; 65],
            >(b"mp_limb_t mpn_rshift(mp_ptr, mp_srcptr, mp_size_t, unsigned int)\0"))
                .as_ptr(),
        );
    }
    'c_7779: {
        if cnt >= 1 as i32 as u32 {} else {
            __assert_fail(
                b"cnt >= 1\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                682 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 65],
                    &[i8; 65],
                >(b"mp_limb_t mpn_rshift(mp_ptr, mp_srcptr, mp_size_t, unsigned int)\0"))
                    .as_ptr(),
            );
        }
    };
    if (cnt as u64)
        < (::core::mem::size_of::<mp_limb_t>() as u64).wrapping_mul(8 as i32 as u64)
    {} else {
        __assert_fail(
            b"cnt < GMP_LIMB_BITS\0" as *const u8 as *const i8,
            b"../mini-gmp.c\0" as *const u8 as *const i8,
            683 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 65],
                &[i8; 65],
            >(b"mp_limb_t mpn_rshift(mp_ptr, mp_srcptr, mp_size_t, unsigned int)\0"))
                .as_ptr(),
        );
    }
    'c_7731: {
        if (cnt as u64)
            < (::core::mem::size_of::<mp_limb_t>() as u64).wrapping_mul(8 as i32 as u64)
        {} else {
            __assert_fail(
                b"cnt < GMP_LIMB_BITS\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                683 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 65],
                    &[i8; 65],
                >(b"mp_limb_t mpn_rshift(mp_ptr, mp_srcptr, mp_size_t, unsigned int)\0"))
                    .as_ptr(),
            );
        }
    };
    tnc = (::core::mem::size_of::<mp_limb_t>() as u64)
        .wrapping_mul(8 as i32 as u64)
        .wrapping_sub(cnt as u64) as u32;
    let fresh10 = up;
    up = up.offset(1);
    high_limb = *fresh10;
    retval = high_limb << tnc;
    low_limb = high_limb >> cnt;
    loop {
        n -= 1;
        if !(n != 0 as i32 as i64) {
            break;
        }
        let fresh11 = up;
        up = up.offset(1);
        high_limb = *fresh11;
        let fresh12 = rp;
        rp = rp.offset(1);
        *fresh12 = low_limb | high_limb << tnc;
        low_limb = high_limb >> cnt;
    }
    *rp = low_limb;
    return retval;
}
unsafe extern "C" fn mpn_common_scan(
    mut limb: mp_limb_t,
    mut i: mp_size_t,
    mut up: mp_srcptr,
    mut un: mp_size_t,
    mut ux: mp_limb_t,
) -> mp_bitcnt_t {
    let mut cnt: u32 = 0;
    if ux == 0 as i32 as u64 || ux == !(0 as i32 as mp_limb_t) {} else {
        __assert_fail(
            b"ux == 0 || ux == GMP_LIMB_MAX\0" as *const u8 as *const i8,
            b"../mini-gmp.c\0" as *const u8 as *const i8,
            707 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 83],
                &[i8; 83],
            >(
                b"mp_bitcnt_t mpn_common_scan(mp_limb_t, mp_size_t, mp_srcptr, mp_size_t, mp_limb_t)\0",
            ))
                .as_ptr(),
        );
    }
    'c_16327: {
        if ux == 0 as i32 as u64 || ux == !(0 as i32 as mp_limb_t) {} else {
            __assert_fail(
                b"ux == 0 || ux == GMP_LIMB_MAX\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                707 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 83],
                    &[i8; 83],
                >(
                    b"mp_bitcnt_t mpn_common_scan(mp_limb_t, mp_size_t, mp_srcptr, mp_size_t, mp_limb_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if 0 as i32 as i64 <= i && i <= un {} else {
        __assert_fail(
            b"0 <= i && i <= un\0" as *const u8 as *const i8,
            b"../mini-gmp.c\0" as *const u8 as *const i8,
            708 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 83],
                &[i8; 83],
            >(
                b"mp_bitcnt_t mpn_common_scan(mp_limb_t, mp_size_t, mp_srcptr, mp_size_t, mp_limb_t)\0",
            ))
                .as_ptr(),
        );
    }
    'c_16275: {
        if 0 as i32 as i64 <= i && i <= un {} else {
            __assert_fail(
                b"0 <= i && i <= un\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                708 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 83],
                    &[i8; 83],
                >(
                    b"mp_bitcnt_t mpn_common_scan(mp_limb_t, mp_size_t, mp_srcptr, mp_size_t, mp_limb_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    while limb == 0 as i32 as u64 {
        i += 1;
        i;
        if i == un {
            return if ux == 0 as i32 as u64 {
                !(0 as i32 as mp_bitcnt_t)
            } else {
                (un as u64)
                    .wrapping_mul(
                        (::core::mem::size_of::<mp_limb_t>() as u64)
                            .wrapping_mul(8 as i32 as u64),
                    )
            };
        }
        limb = ux ^ *up.offset(i as isize);
    }
    let mut __ctz_x: mp_limb_t = limb;
    let mut __ctz_c: u32 = 0 as i32 as u32;
    let mut __clz_x: mp_limb_t = __ctz_x & __ctz_x.wrapping_neg();
    let mut __clz_c: u32 = 0 as i32 as u32;
    let mut LOCAL_SHIFT_BITS: i32 = 8 as i32;
    if (::core::mem::size_of::<mp_limb_t>() as u64).wrapping_mul(8 as i32 as u64)
        > LOCAL_SHIFT_BITS as u64
    {
        while __clz_x
            & (0xff as i32 as mp_limb_t)
                << (::core::mem::size_of::<mp_limb_t>() as u64)
                    .wrapping_mul(8 as i32 as u64)
                    .wrapping_sub(8 as i32 as u64) == 0 as i32 as u64
        {
            __clz_x <<= LOCAL_SHIFT_BITS;
            __clz_c = __clz_c.wrapping_add(8 as i32 as u32);
        }
    }
    while __clz_x
        & (1 as i32 as mp_limb_t)
            << (::core::mem::size_of::<mp_limb_t>() as u64)
                .wrapping_mul(8 as i32 as u64)
                .wrapping_sub(1 as i32 as u64) == 0 as i32 as u64
    {
        __clz_x <<= 1 as i32;
        __clz_c = __clz_c.wrapping_add(1);
        __clz_c;
    }
    __ctz_c = __clz_c;
    cnt = (::core::mem::size_of::<mp_limb_t>() as u64)
        .wrapping_mul(8 as i32 as u64)
        .wrapping_sub(1 as i32 as u64)
        .wrapping_sub(__ctz_c as u64) as u32;
    return (i as mp_bitcnt_t)
        .wrapping_mul(
            (::core::mem::size_of::<mp_limb_t>() as u64).wrapping_mul(8 as i32 as u64),
        )
        .wrapping_add(cnt as u64);
}
#[no_mangle]
pub unsafe extern "C" fn mpn_scan1(
    mut ptr: mp_srcptr,
    mut bit: mp_bitcnt_t,
) -> mp_bitcnt_t {
    let mut i: mp_size_t = 0;
    i = bit
        .wrapping_div(
            (::core::mem::size_of::<mp_limb_t>() as u64).wrapping_mul(8 as i32 as u64),
        ) as mp_size_t;
    return mpn_common_scan(
        *ptr.offset(i as isize)
            & !(0 as i32 as mp_limb_t)
                << bit
                    .wrapping_rem(
                        (::core::mem::size_of::<mp_limb_t>() as u64)
                            .wrapping_mul(8 as i32 as u64),
                    ),
        i,
        ptr,
        i,
        0 as i32 as mp_limb_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn mpn_scan0(
    mut ptr: mp_srcptr,
    mut bit: mp_bitcnt_t,
) -> mp_bitcnt_t {
    let mut i: mp_size_t = 0;
    i = bit
        .wrapping_div(
            (::core::mem::size_of::<mp_limb_t>() as u64).wrapping_mul(8 as i32 as u64),
        ) as mp_size_t;
    return mpn_common_scan(
        !*ptr.offset(i as isize)
            & !(0 as i32 as mp_limb_t)
                << bit
                    .wrapping_rem(
                        (::core::mem::size_of::<mp_limb_t>() as u64)
                            .wrapping_mul(8 as i32 as u64),
                    ),
        i,
        ptr,
        i,
        !(0 as i32 as mp_limb_t),
    );
}
#[no_mangle]
pub unsafe extern "C" fn mpn_com(mut rp: mp_ptr, mut up: mp_srcptr, mut n: mp_size_t) {
    loop {
        n -= 1;
        if !(n >= 0 as i32 as i64) {
            break;
        }
        let fresh13 = up;
        up = up.offset(1);
        let fresh14 = rp;
        rp = rp.offset(1);
        *fresh14 = !*fresh13;
    };
}
#[no_mangle]
pub unsafe extern "C" fn mpn_neg(
    mut rp: mp_ptr,
    mut up: mp_srcptr,
    mut n: mp_size_t,
) -> mp_limb_t {
    while *up == 0 as i32 as u64 {
        *rp = 0 as i32 as mp_limb_t;
        n -= 1;
        if n == 0 {
            return 0 as i32 as mp_limb_t;
        }
        up = up.offset(1);
        up;
        rp = rp.offset(1);
        rp;
    }
    *rp = (*up).wrapping_neg();
    rp = rp.offset(1);
    up = up.offset(1);
    n -= 1;
    mpn_com(rp, up, n);
    return 1 as i32 as mp_limb_t;
}
#[no_mangle]
pub unsafe extern "C" fn mpn_invert_3by2(
    mut u1: mp_limb_t,
    mut u0: mp_limb_t,
) -> mp_limb_t {
    let mut r: mp_limb_t = 0;
    let mut m: mp_limb_t = 0;
    let mut p: mp_limb_t = 0;
    let mut ql: mp_limb_t = 0;
    let mut ul: u32 = 0;
    let mut uh: u32 = 0;
    let mut qh: u32 = 0;
    ul = (u1
        & ((1 as i32 as mp_limb_t)
            << (::core::mem::size_of::<mp_limb_t>() as u64)
                .wrapping_mul(8 as i32 as u64)
                .wrapping_div(2 as i32 as u64))
            .wrapping_sub(1 as i32 as u64)) as u32;
    uh = (u1
        >> (::core::mem::size_of::<mp_limb_t>() as u64)
            .wrapping_mul(8 as i32 as u64)
            .wrapping_div(2 as i32 as u64)) as u32;
    qh = (u1 ^ !(0 as i32 as mp_limb_t)).wrapping_div(uh as u64) as u32;
    r = (!u1).wrapping_sub((qh as mp_limb_t).wrapping_mul(uh as u64))
        << (::core::mem::size_of::<mp_limb_t>() as u64)
            .wrapping_mul(8 as i32 as u64)
            .wrapping_div(2 as i32 as u64)
        | ((1 as i32 as mp_limb_t)
            << (::core::mem::size_of::<mp_limb_t>() as u64)
                .wrapping_mul(8 as i32 as u64)
                .wrapping_div(2 as i32 as u64))
            .wrapping_sub(1 as i32 as u64);
    p = (qh as mp_limb_t).wrapping_mul(ul as u64);
    if r < p {
        qh = qh.wrapping_sub(1);
        qh;
        r = (r as u64).wrapping_add(u1) as mp_limb_t as mp_limb_t;
        if r >= u1 {
            if r < p {
                qh = qh.wrapping_sub(1);
                qh;
                r = (r as u64).wrapping_add(u1) as mp_limb_t as mp_limb_t;
            }
        }
    }
    r = (r as u64).wrapping_sub(p) as mp_limb_t as mp_limb_t;
    p = (r
        >> (::core::mem::size_of::<mp_limb_t>() as u64)
            .wrapping_mul(8 as i32 as u64)
            .wrapping_div(2 as i32 as u64))
        .wrapping_mul(qh as u64)
        .wrapping_add(r);
    ql = (p
        >> (::core::mem::size_of::<mp_limb_t>() as u64)
            .wrapping_mul(8 as i32 as u64)
            .wrapping_div(2 as i32 as u64))
        .wrapping_add(1 as i32 as u64);
    r = (r
        << (::core::mem::size_of::<mp_limb_t>() as u64)
            .wrapping_mul(8 as i32 as u64)
            .wrapping_div(2 as i32 as u64))
        .wrapping_add(
            ((1 as i32 as mp_limb_t)
                << (::core::mem::size_of::<mp_limb_t>() as u64)
                    .wrapping_mul(8 as i32 as u64)
                    .wrapping_div(2 as i32 as u64))
                .wrapping_sub(1 as i32 as u64),
        )
        .wrapping_sub(ql.wrapping_mul(u1));
    if r
        >= !(0 as i32 as mp_limb_t)
            & p
                << (::core::mem::size_of::<mp_limb_t>() as u64)
                    .wrapping_mul(8 as i32 as u64)
                    .wrapping_div(2 as i32 as u64)
    {
        ql = ql.wrapping_sub(1);
        ql;
        r = (r as u64).wrapping_add(u1) as mp_limb_t as mp_limb_t;
    }
    m = ((qh as mp_limb_t)
        << (::core::mem::size_of::<mp_limb_t>() as u64)
            .wrapping_mul(8 as i32 as u64)
            .wrapping_div(2 as i32 as u64))
        .wrapping_add(ql);
    if r >= u1 {
        m = m.wrapping_add(1);
        m;
        r = (r as u64).wrapping_sub(u1) as mp_limb_t as mp_limb_t;
    }
    if u0 > 0 as i32 as u64 {
        let mut th: mp_limb_t = 0;
        let mut tl: mp_limb_t = 0;
        r = !r;
        r = (r as u64).wrapping_add(u0) as mp_limb_t as mp_limb_t;
        if r < u0 {
            m = m.wrapping_sub(1);
            m;
            if r >= u1 {
                m = m.wrapping_sub(1);
                m;
                r = (r as u64).wrapping_sub(u1) as mp_limb_t as mp_limb_t;
            }
            r = (r as u64).wrapping_sub(u1) as mp_limb_t as mp_limb_t;
        }
        let mut LOCAL_GMP_LIMB_BITS: i32 = (::core::mem::size_of::<mp_limb_t>() as u64)
            .wrapping_mul(8 as i32 as u64) as i32;
        if (::core::mem::size_of::<u32>() as u64).wrapping_mul(8 as i32 as u64)
            >= (2 as i32 as u64)
                .wrapping_mul(
                    (::core::mem::size_of::<mp_limb_t>() as u64)
                        .wrapping_mul(8 as i32 as u64),
                )
        {
            let mut __ww: u32 = (u0 as u32 as u64).wrapping_mul(m) as u32;
            tl = __ww as mp_limb_t;
            th = (__ww >> LOCAL_GMP_LIMB_BITS) as mp_limb_t;
        } else if (::core::mem::size_of::<u64>() as u64).wrapping_mul(8 as i32 as u64)
            >= (2 as i32 as u64)
                .wrapping_mul(
                    (::core::mem::size_of::<mp_limb_t>() as u64)
                        .wrapping_mul(8 as i32 as u64),
                )
        {
            let mut __ww_0: u64 = u0.wrapping_mul(m);
            tl = __ww_0;
            th = __ww_0 >> LOCAL_GMP_LIMB_BITS;
        } else {
            let mut __x0: mp_limb_t = 0;
            let mut __x1: mp_limb_t = 0;
            let mut __x2: mp_limb_t = 0;
            let mut __x3: mp_limb_t = 0;
            let mut __ul: u32 = 0;
            let mut __vl: u32 = 0;
            let mut __uh: u32 = 0;
            let mut __vh: u32 = 0;
            let mut __u: mp_limb_t = u0;
            let mut __v: mp_limb_t = m;
            __ul = (__u
                & ((1 as i32 as mp_limb_t)
                    << (::core::mem::size_of::<mp_limb_t>() as u64)
                        .wrapping_mul(8 as i32 as u64)
                        .wrapping_div(2 as i32 as u64))
                    .wrapping_sub(1 as i32 as u64)) as u32;
            __uh = (__u
                >> (::core::mem::size_of::<mp_limb_t>() as u64)
                    .wrapping_mul(8 as i32 as u64)
                    .wrapping_div(2 as i32 as u64)) as u32;
            __vl = (__v
                & ((1 as i32 as mp_limb_t)
                    << (::core::mem::size_of::<mp_limb_t>() as u64)
                        .wrapping_mul(8 as i32 as u64)
                        .wrapping_div(2 as i32 as u64))
                    .wrapping_sub(1 as i32 as u64)) as u32;
            __vh = (__v
                >> (::core::mem::size_of::<mp_limb_t>() as u64)
                    .wrapping_mul(8 as i32 as u64)
                    .wrapping_div(2 as i32 as u64)) as u32;
            __x0 = (__ul as mp_limb_t).wrapping_mul(__vl as u64);
            __x1 = (__ul as mp_limb_t).wrapping_mul(__vh as u64);
            __x2 = (__uh as mp_limb_t).wrapping_mul(__vl as u64);
            __x3 = (__uh as mp_limb_t).wrapping_mul(__vh as u64);
            __x1 = (__x1 as u64)
                .wrapping_add(
                    __x0
                        >> (::core::mem::size_of::<mp_limb_t>() as u64)
                            .wrapping_mul(8 as i32 as u64)
                            .wrapping_div(2 as i32 as u64),
                ) as mp_limb_t as mp_limb_t;
            __x1 = (__x1 as u64).wrapping_add(__x2) as mp_limb_t as mp_limb_t;
            if __x1 < __x2 {
                __x3 = (__x3 as u64)
                    .wrapping_add(
                        (1 as i32 as mp_limb_t)
                            << (::core::mem::size_of::<mp_limb_t>() as u64)
                                .wrapping_mul(8 as i32 as u64)
                                .wrapping_div(2 as i32 as u64),
                    ) as mp_limb_t as mp_limb_t;
            }
            th = __x3
                .wrapping_add(
                    __x1
                        >> (::core::mem::size_of::<mp_limb_t>() as u64)
                            .wrapping_mul(8 as i32 as u64)
                            .wrapping_div(2 as i32 as u64),
                );
            tl = (__x1
                << (::core::mem::size_of::<mp_limb_t>() as u64)
                    .wrapping_mul(8 as i32 as u64)
                    .wrapping_div(2 as i32 as u64))
                .wrapping_add(
                    __x0
                        & ((1 as i32 as mp_limb_t)
                            << (::core::mem::size_of::<mp_limb_t>() as u64)
                                .wrapping_mul(8 as i32 as u64)
                                .wrapping_div(2 as i32 as u64))
                            .wrapping_sub(1 as i32 as u64),
                );
        }
        r = (r as u64).wrapping_add(th) as mp_limb_t as mp_limb_t;
        if r < th {
            m = m.wrapping_sub(1);
            m;
            m = (m as u64)
                .wrapping_sub(
                    ((r > u1) as i32 | (r == u1) as i32 & (tl > u0) as i32) as u64,
                ) as mp_limb_t as mp_limb_t;
        }
    }
    return m;
}
unsafe extern "C" fn mpn_div_qr_1_invert(
    mut inv: *mut gmp_div_inverse,
    mut d: mp_limb_t,
) {
    let mut shift: u32 = 0;
    if d > 0 as i32 as u64 {} else {
        __assert_fail(
            b"d > 0\0" as *const u8 as *const i8,
            b"../mini-gmp.c\0" as *const u8 as *const i8,
            893 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 62],
                &[i8; 62],
            >(b"void mpn_div_qr_1_invert(struct gmp_div_inverse *, mp_limb_t)\0"))
                .as_ptr(),
        );
    }
    'c_13281: {
        if d > 0 as i32 as u64 {} else {
            __assert_fail(
                b"d > 0\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                893 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 62],
                    &[i8; 62],
                >(b"void mpn_div_qr_1_invert(struct gmp_div_inverse *, mp_limb_t)\0"))
                    .as_ptr(),
            );
        }
    };
    let mut __clz_x: mp_limb_t = d;
    let mut __clz_c: u32 = 0 as i32 as u32;
    let mut LOCAL_SHIFT_BITS: i32 = 8 as i32;
    if (::core::mem::size_of::<mp_limb_t>() as u64).wrapping_mul(8 as i32 as u64)
        > LOCAL_SHIFT_BITS as u64
    {
        while __clz_x
            & (0xff as i32 as mp_limb_t)
                << (::core::mem::size_of::<mp_limb_t>() as u64)
                    .wrapping_mul(8 as i32 as u64)
                    .wrapping_sub(8 as i32 as u64) == 0 as i32 as u64
        {
            __clz_x <<= LOCAL_SHIFT_BITS;
            __clz_c = __clz_c.wrapping_add(8 as i32 as u32);
        }
    }
    while __clz_x
        & (1 as i32 as mp_limb_t)
            << (::core::mem::size_of::<mp_limb_t>() as u64)
                .wrapping_mul(8 as i32 as u64)
                .wrapping_sub(1 as i32 as u64) == 0 as i32 as u64
    {
        __clz_x <<= 1 as i32;
        __clz_c = __clz_c.wrapping_add(1);
        __clz_c;
    }
    shift = __clz_c;
    (*inv).shift = shift;
    (*inv).d1 = d << shift;
    (*inv).di = mpn_invert_3by2((*inv).d1, 0 as i32 as mp_limb_t);
}
unsafe extern "C" fn mpn_div_qr_2_invert(
    mut inv: *mut gmp_div_inverse,
    mut d1: mp_limb_t,
    mut d0: mp_limb_t,
) {
    let mut shift: u32 = 0;
    if d1 > 0 as i32 as u64 {} else {
        __assert_fail(
            b"d1 > 0\0" as *const u8 as *const i8,
            b"../mini-gmp.c\0" as *const u8 as *const i8,
            906 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 73],
                &[i8; 73],
            >(
                b"void mpn_div_qr_2_invert(struct gmp_div_inverse *, mp_limb_t, mp_limb_t)\0",
            ))
                .as_ptr(),
        );
    }
    'c_13097: {
        if d1 > 0 as i32 as u64 {} else {
            __assert_fail(
                b"d1 > 0\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                906 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 73],
                    &[i8; 73],
                >(
                    b"void mpn_div_qr_2_invert(struct gmp_div_inverse *, mp_limb_t, mp_limb_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    let mut __clz_x: mp_limb_t = d1;
    let mut __clz_c: u32 = 0 as i32 as u32;
    let mut LOCAL_SHIFT_BITS: i32 = 8 as i32;
    if (::core::mem::size_of::<mp_limb_t>() as u64).wrapping_mul(8 as i32 as u64)
        > LOCAL_SHIFT_BITS as u64
    {
        while __clz_x
            & (0xff as i32 as mp_limb_t)
                << (::core::mem::size_of::<mp_limb_t>() as u64)
                    .wrapping_mul(8 as i32 as u64)
                    .wrapping_sub(8 as i32 as u64) == 0 as i32 as u64
        {
            __clz_x <<= LOCAL_SHIFT_BITS;
            __clz_c = __clz_c.wrapping_add(8 as i32 as u32);
        }
    }
    while __clz_x
        & (1 as i32 as mp_limb_t)
            << (::core::mem::size_of::<mp_limb_t>() as u64)
                .wrapping_mul(8 as i32 as u64)
                .wrapping_sub(1 as i32 as u64) == 0 as i32 as u64
    {
        __clz_x <<= 1 as i32;
        __clz_c = __clz_c.wrapping_add(1);
        __clz_c;
    }
    shift = __clz_c;
    (*inv).shift = shift;
    if shift > 0 as i32 as u32 {
        d1 = d1 << shift
            | d0
                >> (::core::mem::size_of::<mp_limb_t>() as u64)
                    .wrapping_mul(8 as i32 as u64)
                    .wrapping_sub(shift as u64);
        d0 <<= shift;
    }
    (*inv).d1 = d1;
    (*inv).d0 = d0;
    (*inv).di = mpn_invert_3by2(d1, d0);
}
unsafe extern "C" fn mpn_div_qr_invert(
    mut inv: *mut gmp_div_inverse,
    mut dp: mp_srcptr,
    mut dn: mp_size_t,
) {
    if dn > 0 as i32 as i64 {} else {
        __assert_fail(
            b"dn > 0\0" as *const u8 as *const i8,
            b"../mini-gmp.c\0" as *const u8 as *const i8,
            923 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 71],
                &[i8; 71],
            >(
                b"void mpn_div_qr_invert(struct gmp_div_inverse *, mp_srcptr, mp_size_t)\0",
            ))
                .as_ptr(),
        );
    }
    'c_13324: {
        if dn > 0 as i32 as i64 {} else {
            __assert_fail(
                b"dn > 0\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                923 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 71],
                    &[i8; 71],
                >(
                    b"void mpn_div_qr_invert(struct gmp_div_inverse *, mp_srcptr, mp_size_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if dn == 1 as i32 as i64 {
        mpn_div_qr_1_invert(inv, *dp.offset(0 as i32 as isize));
    } else if dn == 2 as i32 as i64 {
        mpn_div_qr_2_invert(
            inv,
            *dp.offset(1 as i32 as isize),
            *dp.offset(0 as i32 as isize),
        );
    } else {
        let mut shift: u32 = 0;
        let mut d1: mp_limb_t = 0;
        let mut d0: mp_limb_t = 0;
        d1 = *dp.offset((dn - 1 as i32 as i64) as isize);
        d0 = *dp.offset((dn - 2 as i32 as i64) as isize);
        if d1 > 0 as i32 as u64 {} else {
            __assert_fail(
                b"d1 > 0\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                936 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 71],
                    &[i8; 71],
                >(
                    b"void mpn_div_qr_invert(struct gmp_div_inverse *, mp_srcptr, mp_size_t)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_12853: {
            if d1 > 0 as i32 as u64 {} else {
                __assert_fail(
                    b"d1 > 0\0" as *const u8 as *const i8,
                    b"../mini-gmp.c\0" as *const u8 as *const i8,
                    936 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 71],
                        &[i8; 71],
                    >(
                        b"void mpn_div_qr_invert(struct gmp_div_inverse *, mp_srcptr, mp_size_t)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        let mut __clz_x: mp_limb_t = d1;
        let mut __clz_c: u32 = 0 as i32 as u32;
        let mut LOCAL_SHIFT_BITS: i32 = 8 as i32;
        if (::core::mem::size_of::<mp_limb_t>() as u64).wrapping_mul(8 as i32 as u64)
            > LOCAL_SHIFT_BITS as u64
        {
            while __clz_x
                & (0xff as i32 as mp_limb_t)
                    << (::core::mem::size_of::<mp_limb_t>() as u64)
                        .wrapping_mul(8 as i32 as u64)
                        .wrapping_sub(8 as i32 as u64) == 0 as i32 as u64
            {
                __clz_x <<= LOCAL_SHIFT_BITS;
                __clz_c = __clz_c.wrapping_add(8 as i32 as u32);
            }
        }
        while __clz_x
            & (1 as i32 as mp_limb_t)
                << (::core::mem::size_of::<mp_limb_t>() as u64)
                    .wrapping_mul(8 as i32 as u64)
                    .wrapping_sub(1 as i32 as u64) == 0 as i32 as u64
        {
            __clz_x <<= 1 as i32;
            __clz_c = __clz_c.wrapping_add(1);
            __clz_c;
        }
        shift = __clz_c;
        (*inv).shift = shift;
        if shift > 0 as i32 as u32 {
            d1 = d1 << shift
                | d0
                    >> (::core::mem::size_of::<mp_limb_t>() as u64)
                        .wrapping_mul(8 as i32 as u64)
                        .wrapping_sub(shift as u64);
            d0 = d0 << shift
                | *dp.offset((dn - 3 as i32 as i64) as isize)
                    >> (::core::mem::size_of::<mp_limb_t>() as u64)
                        .wrapping_mul(8 as i32 as u64)
                        .wrapping_sub(shift as u64);
        }
        (*inv).d1 = d1;
        (*inv).d0 = d0;
        (*inv).di = mpn_invert_3by2(d1, d0);
    };
}
unsafe extern "C" fn mpn_div_qr_1_preinv(
    mut qp: mp_ptr,
    mut np: mp_srcptr,
    mut nn: mp_size_t,
    mut inv: *const gmp_div_inverse,
) -> mp_limb_t {
    let mut d: mp_limb_t = 0;
    let mut di: mp_limb_t = 0;
    let mut r: mp_limb_t = 0;
    let mut tp: mp_ptr = 0 as mp_ptr;
    if (*inv).shift > 0 as i32 as u32 {
        tp = if !qp.is_null() { qp } else { gmp_xalloc_limbs(nn) };
        r = mpn_lshift(tp, np, nn, (*inv).shift);
        np = tp as mp_srcptr;
    } else {
        r = 0 as i32 as mp_limb_t;
    }
    d = (*inv).d1;
    di = (*inv).di;
    loop {
        nn -= 1;
        if !(nn >= 0 as i32 as i64) {
            break;
        }
        let mut q: mp_limb_t = 0;
        let mut _qh: mp_limb_t = 0;
        let mut _ql: mp_limb_t = 0;
        let mut _r: mp_limb_t = 0;
        let mut _mask: mp_limb_t = 0;
        let mut LOCAL_GMP_LIMB_BITS: i32 = (::core::mem::size_of::<mp_limb_t>() as u64)
            .wrapping_mul(8 as i32 as u64) as i32;
        if (::core::mem::size_of::<u32>() as u64).wrapping_mul(8 as i32 as u64)
            >= (2 as i32 as u64)
                .wrapping_mul(
                    (::core::mem::size_of::<mp_limb_t>() as u64)
                        .wrapping_mul(8 as i32 as u64),
                )
        {
            let mut __ww: u32 = (r as u32 as u64).wrapping_mul(di) as u32;
            _ql = __ww as mp_limb_t;
            _qh = (__ww >> LOCAL_GMP_LIMB_BITS) as mp_limb_t;
        } else if (::core::mem::size_of::<u64>() as u64).wrapping_mul(8 as i32 as u64)
            >= (2 as i32 as u64)
                .wrapping_mul(
                    (::core::mem::size_of::<mp_limb_t>() as u64)
                        .wrapping_mul(8 as i32 as u64),
                )
        {
            let mut __ww_0: u64 = r.wrapping_mul(di);
            _ql = __ww_0;
            _qh = __ww_0 >> LOCAL_GMP_LIMB_BITS;
        } else {
            let mut __x0: mp_limb_t = 0;
            let mut __x1: mp_limb_t = 0;
            let mut __x2: mp_limb_t = 0;
            let mut __x3: mp_limb_t = 0;
            let mut __ul: u32 = 0;
            let mut __vl: u32 = 0;
            let mut __uh: u32 = 0;
            let mut __vh: u32 = 0;
            let mut __u: mp_limb_t = r;
            let mut __v: mp_limb_t = di;
            __ul = (__u
                & ((1 as i32 as mp_limb_t)
                    << (::core::mem::size_of::<mp_limb_t>() as u64)
                        .wrapping_mul(8 as i32 as u64)
                        .wrapping_div(2 as i32 as u64))
                    .wrapping_sub(1 as i32 as u64)) as u32;
            __uh = (__u
                >> (::core::mem::size_of::<mp_limb_t>() as u64)
                    .wrapping_mul(8 as i32 as u64)
                    .wrapping_div(2 as i32 as u64)) as u32;
            __vl = (__v
                & ((1 as i32 as mp_limb_t)
                    << (::core::mem::size_of::<mp_limb_t>() as u64)
                        .wrapping_mul(8 as i32 as u64)
                        .wrapping_div(2 as i32 as u64))
                    .wrapping_sub(1 as i32 as u64)) as u32;
            __vh = (__v
                >> (::core::mem::size_of::<mp_limb_t>() as u64)
                    .wrapping_mul(8 as i32 as u64)
                    .wrapping_div(2 as i32 as u64)) as u32;
            __x0 = (__ul as mp_limb_t).wrapping_mul(__vl as u64);
            __x1 = (__ul as mp_limb_t).wrapping_mul(__vh as u64);
            __x2 = (__uh as mp_limb_t).wrapping_mul(__vl as u64);
            __x3 = (__uh as mp_limb_t).wrapping_mul(__vh as u64);
            __x1 = (__x1 as u64)
                .wrapping_add(
                    __x0
                        >> (::core::mem::size_of::<mp_limb_t>() as u64)
                            .wrapping_mul(8 as i32 as u64)
                            .wrapping_div(2 as i32 as u64),
                ) as mp_limb_t as mp_limb_t;
            __x1 = (__x1 as u64).wrapping_add(__x2) as mp_limb_t as mp_limb_t;
            if __x1 < __x2 {
                __x3 = (__x3 as u64)
                    .wrapping_add(
                        (1 as i32 as mp_limb_t)
                            << (::core::mem::size_of::<mp_limb_t>() as u64)
                                .wrapping_mul(8 as i32 as u64)
                                .wrapping_div(2 as i32 as u64),
                    ) as mp_limb_t as mp_limb_t;
            }
            _qh = __x3
                .wrapping_add(
                    __x1
                        >> (::core::mem::size_of::<mp_limb_t>() as u64)
                            .wrapping_mul(8 as i32 as u64)
                            .wrapping_div(2 as i32 as u64),
                );
            _ql = (__x1
                << (::core::mem::size_of::<mp_limb_t>() as u64)
                    .wrapping_mul(8 as i32 as u64)
                    .wrapping_div(2 as i32 as u64))
                .wrapping_add(
                    __x0
                        & ((1 as i32 as mp_limb_t)
                            << (::core::mem::size_of::<mp_limb_t>() as u64)
                                .wrapping_mul(8 as i32 as u64)
                                .wrapping_div(2 as i32 as u64))
                            .wrapping_sub(1 as i32 as u64),
                );
        }
        let mut __x: mp_limb_t = 0;
        __x = _ql.wrapping_add(*np.offset(nn as isize));
        _qh = _qh
            .wrapping_add(r.wrapping_add(1 as i32 as u64))
            .wrapping_add((__x < _ql) as i32 as u64);
        _ql = __x;
        _r = (*np.offset(nn as isize)).wrapping_sub(_qh.wrapping_mul(d));
        _mask = ((_r > _ql) as i32 as mp_limb_t).wrapping_neg();
        _qh = (_qh as u64).wrapping_add(_mask) as mp_limb_t as mp_limb_t;
        _r = (_r as u64).wrapping_add(_mask & d) as mp_limb_t as mp_limb_t;
        if _r >= d {
            _r = (_r as u64).wrapping_sub(d) as mp_limb_t as mp_limb_t;
            _qh = _qh.wrapping_add(1);
            _qh;
        }
        r = _r;
        q = _qh;
        if !qp.is_null() {
            *qp.offset(nn as isize) = q;
        }
    }
    if (*inv).shift > 0 as i32 as u32 && tp != qp {
        (Some(gmp_free_func.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(tp as *mut libc::c_void, 0 as i32 as size_t);
    }
    return r >> (*inv).shift;
}
unsafe extern "C" fn mpn_div_qr_2_preinv(
    mut qp: mp_ptr,
    mut np: mp_ptr,
    mut nn: mp_size_t,
    mut inv: *const gmp_div_inverse,
) {
    let mut shift: u32 = 0;
    let mut i: mp_size_t = 0;
    let mut d1: mp_limb_t = 0;
    let mut d0: mp_limb_t = 0;
    let mut di: mp_limb_t = 0;
    let mut r1: mp_limb_t = 0;
    let mut r0: mp_limb_t = 0;
    if nn >= 2 as i32 as i64 {} else {
        __assert_fail(
            b"nn >= 2\0" as *const u8 as *const i8,
            b"../mini-gmp.c\0" as *const u8 as *const i8,
            994 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 84],
                &[i8; 84],
            >(
                b"void mpn_div_qr_2_preinv(mp_ptr, mp_ptr, mp_size_t, const struct gmp_div_inverse *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_11049: {
        if nn >= 2 as i32 as i64 {} else {
            __assert_fail(
                b"nn >= 2\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                994 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 84],
                    &[i8; 84],
                >(
                    b"void mpn_div_qr_2_preinv(mp_ptr, mp_ptr, mp_size_t, const struct gmp_div_inverse *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    shift = (*inv).shift;
    d1 = (*inv).d1;
    d0 = (*inv).d0;
    di = (*inv).di;
    if shift > 0 as i32 as u32 {
        r1 = mpn_lshift(np, np as mp_srcptr, nn, shift);
    } else {
        r1 = 0 as i32 as mp_limb_t;
    }
    r0 = *np.offset((nn - 1 as i32 as i64) as isize);
    i = nn - 2 as i32 as i64;
    loop {
        let mut n0: mp_limb_t = 0;
        let mut q: mp_limb_t = 0;
        n0 = *np.offset(i as isize);
        let mut _q0: mp_limb_t = 0;
        let mut _t1: mp_limb_t = 0;
        let mut _t0: mp_limb_t = 0;
        let mut _mask: mp_limb_t = 0;
        let mut LOCAL_GMP_LIMB_BITS: i32 = (::core::mem::size_of::<mp_limb_t>() as u64)
            .wrapping_mul(8 as i32 as u64) as i32;
        if (::core::mem::size_of::<u32>() as u64).wrapping_mul(8 as i32 as u64)
            >= (2 as i32 as u64)
                .wrapping_mul(
                    (::core::mem::size_of::<mp_limb_t>() as u64)
                        .wrapping_mul(8 as i32 as u64),
                )
        {
            let mut __ww: u32 = (r1 as u32 as u64).wrapping_mul(di) as u32;
            _q0 = __ww as mp_limb_t;
            q = (__ww >> LOCAL_GMP_LIMB_BITS) as mp_limb_t;
        } else if (::core::mem::size_of::<u64>() as u64).wrapping_mul(8 as i32 as u64)
            >= (2 as i32 as u64)
                .wrapping_mul(
                    (::core::mem::size_of::<mp_limb_t>() as u64)
                        .wrapping_mul(8 as i32 as u64),
                )
        {
            let mut __ww_0: u64 = r1.wrapping_mul(di);
            _q0 = __ww_0;
            q = __ww_0 >> LOCAL_GMP_LIMB_BITS;
        } else {
            let mut __x0: mp_limb_t = 0;
            let mut __x1: mp_limb_t = 0;
            let mut __x2: mp_limb_t = 0;
            let mut __x3: mp_limb_t = 0;
            let mut __ul: u32 = 0;
            let mut __vl: u32 = 0;
            let mut __uh: u32 = 0;
            let mut __vh: u32 = 0;
            let mut __u: mp_limb_t = r1;
            let mut __v: mp_limb_t = di;
            __ul = (__u
                & ((1 as i32 as mp_limb_t)
                    << (::core::mem::size_of::<mp_limb_t>() as u64)
                        .wrapping_mul(8 as i32 as u64)
                        .wrapping_div(2 as i32 as u64))
                    .wrapping_sub(1 as i32 as u64)) as u32;
            __uh = (__u
                >> (::core::mem::size_of::<mp_limb_t>() as u64)
                    .wrapping_mul(8 as i32 as u64)
                    .wrapping_div(2 as i32 as u64)) as u32;
            __vl = (__v
                & ((1 as i32 as mp_limb_t)
                    << (::core::mem::size_of::<mp_limb_t>() as u64)
                        .wrapping_mul(8 as i32 as u64)
                        .wrapping_div(2 as i32 as u64))
                    .wrapping_sub(1 as i32 as u64)) as u32;
            __vh = (__v
                >> (::core::mem::size_of::<mp_limb_t>() as u64)
                    .wrapping_mul(8 as i32 as u64)
                    .wrapping_div(2 as i32 as u64)) as u32;
            __x0 = (__ul as mp_limb_t).wrapping_mul(__vl as u64);
            __x1 = (__ul as mp_limb_t).wrapping_mul(__vh as u64);
            __x2 = (__uh as mp_limb_t).wrapping_mul(__vl as u64);
            __x3 = (__uh as mp_limb_t).wrapping_mul(__vh as u64);
            __x1 = (__x1 as u64)
                .wrapping_add(
                    __x0
                        >> (::core::mem::size_of::<mp_limb_t>() as u64)
                            .wrapping_mul(8 as i32 as u64)
                            .wrapping_div(2 as i32 as u64),
                ) as mp_limb_t as mp_limb_t;
            __x1 = (__x1 as u64).wrapping_add(__x2) as mp_limb_t as mp_limb_t;
            if __x1 < __x2 {
                __x3 = (__x3 as u64)
                    .wrapping_add(
                        (1 as i32 as mp_limb_t)
                            << (::core::mem::size_of::<mp_limb_t>() as u64)
                                .wrapping_mul(8 as i32 as u64)
                                .wrapping_div(2 as i32 as u64),
                    ) as mp_limb_t as mp_limb_t;
            }
            q = __x3
                .wrapping_add(
                    __x1
                        >> (::core::mem::size_of::<mp_limb_t>() as u64)
                            .wrapping_mul(8 as i32 as u64)
                            .wrapping_div(2 as i32 as u64),
                );
            _q0 = (__x1
                << (::core::mem::size_of::<mp_limb_t>() as u64)
                    .wrapping_mul(8 as i32 as u64)
                    .wrapping_div(2 as i32 as u64))
                .wrapping_add(
                    __x0
                        & ((1 as i32 as mp_limb_t)
                            << (::core::mem::size_of::<mp_limb_t>() as u64)
                                .wrapping_mul(8 as i32 as u64)
                                .wrapping_div(2 as i32 as u64))
                            .wrapping_sub(1 as i32 as u64),
                );
        }
        let mut __x: mp_limb_t = 0;
        __x = _q0.wrapping_add(r0);
        q = q.wrapping_add(r1).wrapping_add((__x < _q0) as i32 as u64);
        _q0 = __x;
        r1 = r0.wrapping_sub(d1.wrapping_mul(q));
        let mut __x_0: mp_limb_t = 0;
        __x_0 = n0.wrapping_sub(d0);
        r1 = r1.wrapping_sub(d1).wrapping_sub((n0 < d0) as i32 as u64);
        r0 = __x_0;
        let mut LOCAL_GMP_LIMB_BITS_0: i32 = (::core::mem::size_of::<mp_limb_t>() as u64)
            .wrapping_mul(8 as i32 as u64) as i32;
        if (::core::mem::size_of::<u32>() as u64).wrapping_mul(8 as i32 as u64)
            >= (2 as i32 as u64)
                .wrapping_mul(
                    (::core::mem::size_of::<mp_limb_t>() as u64)
                        .wrapping_mul(8 as i32 as u64),
                )
        {
            let mut __ww_1: u32 = (d0 as u32 as u64).wrapping_mul(q) as u32;
            _t0 = __ww_1 as mp_limb_t;
            _t1 = (__ww_1 >> LOCAL_GMP_LIMB_BITS_0) as mp_limb_t;
        } else if (::core::mem::size_of::<u64>() as u64).wrapping_mul(8 as i32 as u64)
            >= (2 as i32 as u64)
                .wrapping_mul(
                    (::core::mem::size_of::<mp_limb_t>() as u64)
                        .wrapping_mul(8 as i32 as u64),
                )
        {
            let mut __ww_2: u64 = d0.wrapping_mul(q);
            _t0 = __ww_2;
            _t1 = __ww_2 >> LOCAL_GMP_LIMB_BITS_0;
        } else {
            let mut __x0_0: mp_limb_t = 0;
            let mut __x1_0: mp_limb_t = 0;
            let mut __x2_0: mp_limb_t = 0;
            let mut __x3_0: mp_limb_t = 0;
            let mut __ul_0: u32 = 0;
            let mut __vl_0: u32 = 0;
            let mut __uh_0: u32 = 0;
            let mut __vh_0: u32 = 0;
            let mut __u_0: mp_limb_t = d0;
            let mut __v_0: mp_limb_t = q;
            __ul_0 = (__u_0
                & ((1 as i32 as mp_limb_t)
                    << (::core::mem::size_of::<mp_limb_t>() as u64)
                        .wrapping_mul(8 as i32 as u64)
                        .wrapping_div(2 as i32 as u64))
                    .wrapping_sub(1 as i32 as u64)) as u32;
            __uh_0 = (__u_0
                >> (::core::mem::size_of::<mp_limb_t>() as u64)
                    .wrapping_mul(8 as i32 as u64)
                    .wrapping_div(2 as i32 as u64)) as u32;
            __vl_0 = (__v_0
                & ((1 as i32 as mp_limb_t)
                    << (::core::mem::size_of::<mp_limb_t>() as u64)
                        .wrapping_mul(8 as i32 as u64)
                        .wrapping_div(2 as i32 as u64))
                    .wrapping_sub(1 as i32 as u64)) as u32;
            __vh_0 = (__v_0
                >> (::core::mem::size_of::<mp_limb_t>() as u64)
                    .wrapping_mul(8 as i32 as u64)
                    .wrapping_div(2 as i32 as u64)) as u32;
            __x0_0 = (__ul_0 as mp_limb_t).wrapping_mul(__vl_0 as u64);
            __x1_0 = (__ul_0 as mp_limb_t).wrapping_mul(__vh_0 as u64);
            __x2_0 = (__uh_0 as mp_limb_t).wrapping_mul(__vl_0 as u64);
            __x3_0 = (__uh_0 as mp_limb_t).wrapping_mul(__vh_0 as u64);
            __x1_0 = (__x1_0 as u64)
                .wrapping_add(
                    __x0_0
                        >> (::core::mem::size_of::<mp_limb_t>() as u64)
                            .wrapping_mul(8 as i32 as u64)
                            .wrapping_div(2 as i32 as u64),
                ) as mp_limb_t as mp_limb_t;
            __x1_0 = (__x1_0 as u64).wrapping_add(__x2_0) as mp_limb_t as mp_limb_t;
            if __x1_0 < __x2_0 {
                __x3_0 = (__x3_0 as u64)
                    .wrapping_add(
                        (1 as i32 as mp_limb_t)
                            << (::core::mem::size_of::<mp_limb_t>() as u64)
                                .wrapping_mul(8 as i32 as u64)
                                .wrapping_div(2 as i32 as u64),
                    ) as mp_limb_t as mp_limb_t;
            }
            _t1 = __x3_0
                .wrapping_add(
                    __x1_0
                        >> (::core::mem::size_of::<mp_limb_t>() as u64)
                            .wrapping_mul(8 as i32 as u64)
                            .wrapping_div(2 as i32 as u64),
                );
            _t0 = (__x1_0
                << (::core::mem::size_of::<mp_limb_t>() as u64)
                    .wrapping_mul(8 as i32 as u64)
                    .wrapping_div(2 as i32 as u64))
                .wrapping_add(
                    __x0_0
                        & ((1 as i32 as mp_limb_t)
                            << (::core::mem::size_of::<mp_limb_t>() as u64)
                                .wrapping_mul(8 as i32 as u64)
                                .wrapping_div(2 as i32 as u64))
                            .wrapping_sub(1 as i32 as u64),
                );
        }
        let mut __x_1: mp_limb_t = 0;
        __x_1 = r0.wrapping_sub(_t0);
        r1 = r1.wrapping_sub(_t1).wrapping_sub((r0 < _t0) as i32 as u64);
        r0 = __x_1;
        q = q.wrapping_add(1);
        q;
        _mask = ((r1 >= _q0) as i32 as mp_limb_t).wrapping_neg();
        q = (q as u64).wrapping_add(_mask) as mp_limb_t as mp_limb_t;
        let mut __x_2: mp_limb_t = 0;
        __x_2 = r0.wrapping_add(_mask & d0);
        r1 = r1.wrapping_add(_mask & d1).wrapping_add((__x_2 < r0) as i32 as u64);
        r0 = __x_2;
        if r1 >= d1 {
            if r1 > d1 || r0 >= d0 {
                q = q.wrapping_add(1);
                q;
                let mut __x_3: mp_limb_t = 0;
                __x_3 = r0.wrapping_sub(d0);
                r1 = r1.wrapping_sub(d1).wrapping_sub((r0 < d0) as i32 as u64);
                r0 = __x_3;
            }
        }
        if !qp.is_null() {
            *qp.offset(i as isize) = q;
        }
        i -= 1;
        if !(i >= 0 as i32 as i64) {
            break;
        }
    }
    if shift > 0 as i32 as u32 {
        if r0
            & !(0 as i32 as mp_limb_t)
                >> (::core::mem::size_of::<mp_limb_t>() as u64)
                    .wrapping_mul(8 as i32 as u64)
                    .wrapping_sub(shift as u64) == 0 as i32 as u64
        {} else {
            __assert_fail(
                b"(r0 & (GMP_LIMB_MAX >> (GMP_LIMB_BITS - shift))) == 0\0" as *const u8
                    as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                1021 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 84],
                    &[i8; 84],
                >(
                    b"void mpn_div_qr_2_preinv(mp_ptr, mp_ptr, mp_size_t, const struct gmp_div_inverse *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_9856: {
            if r0
                & !(0 as i32 as mp_limb_t)
                    >> (::core::mem::size_of::<mp_limb_t>() as u64)
                        .wrapping_mul(8 as i32 as u64)
                        .wrapping_sub(shift as u64) == 0 as i32 as u64
            {} else {
                __assert_fail(
                    b"(r0 & (GMP_LIMB_MAX >> (GMP_LIMB_BITS - shift))) == 0\0"
                        as *const u8 as *const i8,
                    b"../mini-gmp.c\0" as *const u8 as *const i8,
                    1021 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 84],
                        &[i8; 84],
                    >(
                        b"void mpn_div_qr_2_preinv(mp_ptr, mp_ptr, mp_size_t, const struct gmp_div_inverse *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        r0 = r0 >> shift
            | r1
                << (::core::mem::size_of::<mp_limb_t>() as u64)
                    .wrapping_mul(8 as i32 as u64)
                    .wrapping_sub(shift as u64);
        r1 >>= shift;
    }
    *np.offset(1 as i32 as isize) = r1;
    *np.offset(0 as i32 as isize) = r0;
}
unsafe extern "C" fn mpn_div_qr_pi1(
    mut qp: mp_ptr,
    mut np: mp_ptr,
    mut nn: mp_size_t,
    mut n1: mp_limb_t,
    mut dp: mp_srcptr,
    mut dn: mp_size_t,
    mut dinv: mp_limb_t,
) {
    let mut i: mp_size_t = 0;
    let mut d1: mp_limb_t = 0;
    let mut d0: mp_limb_t = 0;
    let mut cy: mp_limb_t = 0;
    let mut cy1: mp_limb_t = 0;
    let mut q: mp_limb_t = 0;
    if dn > 2 as i32 as i64 {} else {
        __assert_fail(
            b"dn > 2\0" as *const u8 as *const i8,
            b"../mini-gmp.c\0" as *const u8 as *const i8,
            1042 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 91],
                &[i8; 91],
            >(
                b"void mpn_div_qr_pi1(mp_ptr, mp_ptr, mp_size_t, mp_limb_t, mp_srcptr, mp_size_t, mp_limb_t)\0",
            ))
                .as_ptr(),
        );
    }
    'c_9287: {
        if dn > 2 as i32 as i64 {} else {
            __assert_fail(
                b"dn > 2\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                1042 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 91],
                    &[i8; 91],
                >(
                    b"void mpn_div_qr_pi1(mp_ptr, mp_ptr, mp_size_t, mp_limb_t, mp_srcptr, mp_size_t, mp_limb_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if nn >= dn {} else {
        __assert_fail(
            b"nn >= dn\0" as *const u8 as *const i8,
            b"../mini-gmp.c\0" as *const u8 as *const i8,
            1043 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 91],
                &[i8; 91],
            >(
                b"void mpn_div_qr_pi1(mp_ptr, mp_ptr, mp_size_t, mp_limb_t, mp_srcptr, mp_size_t, mp_limb_t)\0",
            ))
                .as_ptr(),
        );
    }
    'c_9249: {
        if nn >= dn {} else {
            __assert_fail(
                b"nn >= dn\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                1043 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 91],
                    &[i8; 91],
                >(
                    b"void mpn_div_qr_pi1(mp_ptr, mp_ptr, mp_size_t, mp_limb_t, mp_srcptr, mp_size_t, mp_limb_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    d1 = *dp.offset((dn - 1 as i32 as i64) as isize);
    d0 = *dp.offset((dn - 2 as i32 as i64) as isize);
    if d1
        & (1 as i32 as mp_limb_t)
            << (::core::mem::size_of::<mp_limb_t>() as u64)
                .wrapping_mul(8 as i32 as u64)
                .wrapping_sub(1 as i32 as u64) != 0 as i32 as u64
    {} else {
        __assert_fail(
            b"(d1 & GMP_LIMB_HIGHBIT) != 0\0" as *const u8 as *const i8,
            b"../mini-gmp.c\0" as *const u8 as *const i8,
            1048 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 91],
                &[i8; 91],
            >(
                b"void mpn_div_qr_pi1(mp_ptr, mp_ptr, mp_size_t, mp_limb_t, mp_srcptr, mp_size_t, mp_limb_t)\0",
            ))
                .as_ptr(),
        );
    }
    'c_9157: {
        if d1
            & (1 as i32 as mp_limb_t)
                << (::core::mem::size_of::<mp_limb_t>() as u64)
                    .wrapping_mul(8 as i32 as u64)
                    .wrapping_sub(1 as i32 as u64) != 0 as i32 as u64
        {} else {
            __assert_fail(
                b"(d1 & GMP_LIMB_HIGHBIT) != 0\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                1048 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 91],
                    &[i8; 91],
                >(
                    b"void mpn_div_qr_pi1(mp_ptr, mp_ptr, mp_size_t, mp_limb_t, mp_srcptr, mp_size_t, mp_limb_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    i = nn - dn;
    loop {
        let mut n0: mp_limb_t = *np.offset((dn - 1 as i32 as i64 + i) as isize);
        if n1 == d1 && n0 == d0 {
            q = !(0 as i32 as mp_limb_t);
            mpn_submul_1(np.offset(i as isize), dp, dn, q);
            n1 = *np.offset((dn - 1 as i32 as i64 + i) as isize);
        } else {
            let mut _q0: mp_limb_t = 0;
            let mut _t1: mp_limb_t = 0;
            let mut _t0: mp_limb_t = 0;
            let mut _mask: mp_limb_t = 0;
            let mut LOCAL_GMP_LIMB_BITS: i32 = (::core::mem::size_of::<mp_limb_t>()
                as u64)
                .wrapping_mul(8 as i32 as u64) as i32;
            if (::core::mem::size_of::<u32>() as u64).wrapping_mul(8 as i32 as u64)
                >= (2 as i32 as u64)
                    .wrapping_mul(
                        (::core::mem::size_of::<mp_limb_t>() as u64)
                            .wrapping_mul(8 as i32 as u64),
                    )
            {
                let mut __ww: u32 = (n1 as u32 as u64).wrapping_mul(dinv) as u32;
                _q0 = __ww as mp_limb_t;
                q = (__ww >> LOCAL_GMP_LIMB_BITS) as mp_limb_t;
            } else if (::core::mem::size_of::<u64>() as u64)
                .wrapping_mul(8 as i32 as u64)
                >= (2 as i32 as u64)
                    .wrapping_mul(
                        (::core::mem::size_of::<mp_limb_t>() as u64)
                            .wrapping_mul(8 as i32 as u64),
                    )
            {
                let mut __ww_0: u64 = n1.wrapping_mul(dinv);
                _q0 = __ww_0;
                q = __ww_0 >> LOCAL_GMP_LIMB_BITS;
            } else {
                let mut __x0: mp_limb_t = 0;
                let mut __x1: mp_limb_t = 0;
                let mut __x2: mp_limb_t = 0;
                let mut __x3: mp_limb_t = 0;
                let mut __ul: u32 = 0;
                let mut __vl: u32 = 0;
                let mut __uh: u32 = 0;
                let mut __vh: u32 = 0;
                let mut __u: mp_limb_t = n1;
                let mut __v: mp_limb_t = dinv;
                __ul = (__u
                    & ((1 as i32 as mp_limb_t)
                        << (::core::mem::size_of::<mp_limb_t>() as u64)
                            .wrapping_mul(8 as i32 as u64)
                            .wrapping_div(2 as i32 as u64))
                        .wrapping_sub(1 as i32 as u64)) as u32;
                __uh = (__u
                    >> (::core::mem::size_of::<mp_limb_t>() as u64)
                        .wrapping_mul(8 as i32 as u64)
                        .wrapping_div(2 as i32 as u64)) as u32;
                __vl = (__v
                    & ((1 as i32 as mp_limb_t)
                        << (::core::mem::size_of::<mp_limb_t>() as u64)
                            .wrapping_mul(8 as i32 as u64)
                            .wrapping_div(2 as i32 as u64))
                        .wrapping_sub(1 as i32 as u64)) as u32;
                __vh = (__v
                    >> (::core::mem::size_of::<mp_limb_t>() as u64)
                        .wrapping_mul(8 as i32 as u64)
                        .wrapping_div(2 as i32 as u64)) as u32;
                __x0 = (__ul as mp_limb_t).wrapping_mul(__vl as u64);
                __x1 = (__ul as mp_limb_t).wrapping_mul(__vh as u64);
                __x2 = (__uh as mp_limb_t).wrapping_mul(__vl as u64);
                __x3 = (__uh as mp_limb_t).wrapping_mul(__vh as u64);
                __x1 = (__x1 as u64)
                    .wrapping_add(
                        __x0
                            >> (::core::mem::size_of::<mp_limb_t>() as u64)
                                .wrapping_mul(8 as i32 as u64)
                                .wrapping_div(2 as i32 as u64),
                    ) as mp_limb_t as mp_limb_t;
                __x1 = (__x1 as u64).wrapping_add(__x2) as mp_limb_t as mp_limb_t;
                if __x1 < __x2 {
                    __x3 = (__x3 as u64)
                        .wrapping_add(
                            (1 as i32 as mp_limb_t)
                                << (::core::mem::size_of::<mp_limb_t>() as u64)
                                    .wrapping_mul(8 as i32 as u64)
                                    .wrapping_div(2 as i32 as u64),
                        ) as mp_limb_t as mp_limb_t;
                }
                q = __x3
                    .wrapping_add(
                        __x1
                            >> (::core::mem::size_of::<mp_limb_t>() as u64)
                                .wrapping_mul(8 as i32 as u64)
                                .wrapping_div(2 as i32 as u64),
                    );
                _q0 = (__x1
                    << (::core::mem::size_of::<mp_limb_t>() as u64)
                        .wrapping_mul(8 as i32 as u64)
                        .wrapping_div(2 as i32 as u64))
                    .wrapping_add(
                        __x0
                            & ((1 as i32 as mp_limb_t)
                                << (::core::mem::size_of::<mp_limb_t>() as u64)
                                    .wrapping_mul(8 as i32 as u64)
                                    .wrapping_div(2 as i32 as u64))
                                .wrapping_sub(1 as i32 as u64),
                    );
            }
            let mut __x: mp_limb_t = 0;
            __x = _q0.wrapping_add(n0);
            q = q.wrapping_add(n1).wrapping_add((__x < _q0) as i32 as u64);
            _q0 = __x;
            n1 = n0.wrapping_sub(d1.wrapping_mul(q));
            let mut __x_0: mp_limb_t = 0;
            __x_0 = (*np.offset((dn - 2 as i32 as i64 + i) as isize)).wrapping_sub(d0);
            n1 = n1
                .wrapping_sub(d1)
                .wrapping_sub(
                    (*np.offset((dn - 2 as i32 as i64 + i) as isize) < d0) as i32 as u64,
                );
            n0 = __x_0;
            let mut LOCAL_GMP_LIMB_BITS_0: i32 = (::core::mem::size_of::<mp_limb_t>()
                as u64)
                .wrapping_mul(8 as i32 as u64) as i32;
            if (::core::mem::size_of::<u32>() as u64).wrapping_mul(8 as i32 as u64)
                >= (2 as i32 as u64)
                    .wrapping_mul(
                        (::core::mem::size_of::<mp_limb_t>() as u64)
                            .wrapping_mul(8 as i32 as u64),
                    )
            {
                let mut __ww_1: u32 = (d0 as u32 as u64).wrapping_mul(q) as u32;
                _t0 = __ww_1 as mp_limb_t;
                _t1 = (__ww_1 >> LOCAL_GMP_LIMB_BITS_0) as mp_limb_t;
            } else if (::core::mem::size_of::<u64>() as u64)
                .wrapping_mul(8 as i32 as u64)
                >= (2 as i32 as u64)
                    .wrapping_mul(
                        (::core::mem::size_of::<mp_limb_t>() as u64)
                            .wrapping_mul(8 as i32 as u64),
                    )
            {
                let mut __ww_2: u64 = d0.wrapping_mul(q);
                _t0 = __ww_2;
                _t1 = __ww_2 >> LOCAL_GMP_LIMB_BITS_0;
            } else {
                let mut __x0_0: mp_limb_t = 0;
                let mut __x1_0: mp_limb_t = 0;
                let mut __x2_0: mp_limb_t = 0;
                let mut __x3_0: mp_limb_t = 0;
                let mut __ul_0: u32 = 0;
                let mut __vl_0: u32 = 0;
                let mut __uh_0: u32 = 0;
                let mut __vh_0: u32 = 0;
                let mut __u_0: mp_limb_t = d0;
                let mut __v_0: mp_limb_t = q;
                __ul_0 = (__u_0
                    & ((1 as i32 as mp_limb_t)
                        << (::core::mem::size_of::<mp_limb_t>() as u64)
                            .wrapping_mul(8 as i32 as u64)
                            .wrapping_div(2 as i32 as u64))
                        .wrapping_sub(1 as i32 as u64)) as u32;
                __uh_0 = (__u_0
                    >> (::core::mem::size_of::<mp_limb_t>() as u64)
                        .wrapping_mul(8 as i32 as u64)
                        .wrapping_div(2 as i32 as u64)) as u32;
                __vl_0 = (__v_0
                    & ((1 as i32 as mp_limb_t)
                        << (::core::mem::size_of::<mp_limb_t>() as u64)
                            .wrapping_mul(8 as i32 as u64)
                            .wrapping_div(2 as i32 as u64))
                        .wrapping_sub(1 as i32 as u64)) as u32;
                __vh_0 = (__v_0
                    >> (::core::mem::size_of::<mp_limb_t>() as u64)
                        .wrapping_mul(8 as i32 as u64)
                        .wrapping_div(2 as i32 as u64)) as u32;
                __x0_0 = (__ul_0 as mp_limb_t).wrapping_mul(__vl_0 as u64);
                __x1_0 = (__ul_0 as mp_limb_t).wrapping_mul(__vh_0 as u64);
                __x2_0 = (__uh_0 as mp_limb_t).wrapping_mul(__vl_0 as u64);
                __x3_0 = (__uh_0 as mp_limb_t).wrapping_mul(__vh_0 as u64);
                __x1_0 = (__x1_0 as u64)
                    .wrapping_add(
                        __x0_0
                            >> (::core::mem::size_of::<mp_limb_t>() as u64)
                                .wrapping_mul(8 as i32 as u64)
                                .wrapping_div(2 as i32 as u64),
                    ) as mp_limb_t as mp_limb_t;
                __x1_0 = (__x1_0 as u64).wrapping_add(__x2_0) as mp_limb_t as mp_limb_t;
                if __x1_0 < __x2_0 {
                    __x3_0 = (__x3_0 as u64)
                        .wrapping_add(
                            (1 as i32 as mp_limb_t)
                                << (::core::mem::size_of::<mp_limb_t>() as u64)
                                    .wrapping_mul(8 as i32 as u64)
                                    .wrapping_div(2 as i32 as u64),
                        ) as mp_limb_t as mp_limb_t;
                }
                _t1 = __x3_0
                    .wrapping_add(
                        __x1_0
                            >> (::core::mem::size_of::<mp_limb_t>() as u64)
                                .wrapping_mul(8 as i32 as u64)
                                .wrapping_div(2 as i32 as u64),
                    );
                _t0 = (__x1_0
                    << (::core::mem::size_of::<mp_limb_t>() as u64)
                        .wrapping_mul(8 as i32 as u64)
                        .wrapping_div(2 as i32 as u64))
                    .wrapping_add(
                        __x0_0
                            & ((1 as i32 as mp_limb_t)
                                << (::core::mem::size_of::<mp_limb_t>() as u64)
                                    .wrapping_mul(8 as i32 as u64)
                                    .wrapping_div(2 as i32 as u64))
                                .wrapping_sub(1 as i32 as u64),
                    );
            }
            let mut __x_1: mp_limb_t = 0;
            __x_1 = n0.wrapping_sub(_t0);
            n1 = n1.wrapping_sub(_t1).wrapping_sub((n0 < _t0) as i32 as u64);
            n0 = __x_1;
            q = q.wrapping_add(1);
            q;
            _mask = ((n1 >= _q0) as i32 as mp_limb_t).wrapping_neg();
            q = (q as u64).wrapping_add(_mask) as mp_limb_t as mp_limb_t;
            let mut __x_2: mp_limb_t = 0;
            __x_2 = n0.wrapping_add(_mask & d0);
            n1 = n1.wrapping_add(_mask & d1).wrapping_add((__x_2 < n0) as i32 as u64);
            n0 = __x_2;
            if n1 >= d1 {
                if n1 > d1 || n0 >= d0 {
                    q = q.wrapping_add(1);
                    q;
                    let mut __x_3: mp_limb_t = 0;
                    __x_3 = n0.wrapping_sub(d0);
                    n1 = n1.wrapping_sub(d1).wrapping_sub((n0 < d0) as i32 as u64);
                    n0 = __x_3;
                }
            }
            cy = mpn_submul_1(np.offset(i as isize), dp, dn - 2 as i32 as i64, q);
            cy1 = (n0 < cy) as i32 as mp_limb_t;
            n0 = n0.wrapping_sub(cy);
            cy = (n1 < cy1) as i32 as mp_limb_t;
            n1 = n1.wrapping_sub(cy1);
            *np.offset((dn - 2 as i32 as i64 + i) as isize) = n0;
            if cy != 0 as i32 as u64 {
                n1 = (n1 as u64)
                    .wrapping_add(
                        d1
                            .wrapping_add(
                                mpn_add_n(
                                    np.offset(i as isize),
                                    np.offset(i as isize) as mp_srcptr,
                                    dp,
                                    dn - 1 as i32 as i64,
                                ),
                            ),
                    ) as mp_limb_t as mp_limb_t;
                q = q.wrapping_sub(1);
                q;
            }
        }
        if !qp.is_null() {
            *qp.offset(i as isize) = q;
        }
        i -= 1;
        if !(i >= 0 as i32 as i64) {
            break;
        }
    }
    *np.offset((dn - 1 as i32 as i64) as isize) = n1;
}
unsafe extern "C" fn mpn_div_qr_preinv(
    mut qp: mp_ptr,
    mut np: mp_ptr,
    mut nn: mp_size_t,
    mut dp: mp_srcptr,
    mut dn: mp_size_t,
    mut inv: *const gmp_div_inverse,
) {
    if dn > 0 as i32 as i64 {} else {
        __assert_fail(
            b"dn > 0\0" as *const u8 as *const i8,
            b"../mini-gmp.c\0" as *const u8 as *const i8,
            1098 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 104],
                &[i8; 104],
            >(
                b"void mpn_div_qr_preinv(mp_ptr, mp_ptr, mp_size_t, mp_srcptr, mp_size_t, const struct gmp_div_inverse *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_11763: {
        if dn > 0 as i32 as i64 {} else {
            __assert_fail(
                b"dn > 0\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                1098 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 104],
                    &[i8; 104],
                >(
                    b"void mpn_div_qr_preinv(mp_ptr, mp_ptr, mp_size_t, mp_srcptr, mp_size_t, const struct gmp_div_inverse *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if nn >= dn {} else {
        __assert_fail(
            b"nn >= dn\0" as *const u8 as *const i8,
            b"../mini-gmp.c\0" as *const u8 as *const i8,
            1099 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 104],
                &[i8; 104],
            >(
                b"void mpn_div_qr_preinv(mp_ptr, mp_ptr, mp_size_t, mp_srcptr, mp_size_t, const struct gmp_div_inverse *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_11725: {
        if nn >= dn {} else {
            __assert_fail(
                b"nn >= dn\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                1099 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 104],
                    &[i8; 104],
                >(
                    b"void mpn_div_qr_preinv(mp_ptr, mp_ptr, mp_size_t, mp_srcptr, mp_size_t, const struct gmp_div_inverse *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if dn == 1 as i32 as i64 {
        *np.offset(0 as i32 as isize) = mpn_div_qr_1_preinv(
            qp,
            np as mp_srcptr,
            nn,
            inv,
        );
    } else if dn == 2 as i32 as i64 {
        mpn_div_qr_2_preinv(qp, np, nn, inv);
    } else {
        let mut nh: mp_limb_t = 0;
        let mut shift: u32 = 0;
        if (*inv).d1 == *dp.offset((dn - 1 as i32 as i64) as isize) {} else {
            __assert_fail(
                b"inv->d1 == dp[dn-1]\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                1110 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 104],
                    &[i8; 104],
                >(
                    b"void mpn_div_qr_preinv(mp_ptr, mp_ptr, mp_size_t, mp_srcptr, mp_size_t, const struct gmp_div_inverse *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_9717: {
            if (*inv).d1 == *dp.offset((dn - 1 as i32 as i64) as isize) {} else {
                __assert_fail(
                    b"inv->d1 == dp[dn-1]\0" as *const u8 as *const i8,
                    b"../mini-gmp.c\0" as *const u8 as *const i8,
                    1110 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 104],
                        &[i8; 104],
                    >(
                        b"void mpn_div_qr_preinv(mp_ptr, mp_ptr, mp_size_t, mp_srcptr, mp_size_t, const struct gmp_div_inverse *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        if (*inv).d0 == *dp.offset((dn - 2 as i32 as i64) as isize) {} else {
            __assert_fail(
                b"inv->d0 == dp[dn-2]\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                1111 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 104],
                    &[i8; 104],
                >(
                    b"void mpn_div_qr_preinv(mp_ptr, mp_ptr, mp_size_t, mp_srcptr, mp_size_t, const struct gmp_div_inverse *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_9661: {
            if (*inv).d0 == *dp.offset((dn - 2 as i32 as i64) as isize) {} else {
                __assert_fail(
                    b"inv->d0 == dp[dn-2]\0" as *const u8 as *const i8,
                    b"../mini-gmp.c\0" as *const u8 as *const i8,
                    1111 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 104],
                        &[i8; 104],
                    >(
                        b"void mpn_div_qr_preinv(mp_ptr, mp_ptr, mp_size_t, mp_srcptr, mp_size_t, const struct gmp_div_inverse *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        if (*inv).d1
            & (1 as i32 as mp_limb_t)
                << (::core::mem::size_of::<mp_limb_t>() as u64)
                    .wrapping_mul(8 as i32 as u64)
                    .wrapping_sub(1 as i32 as u64) != 0 as i32 as u64
        {} else {
            __assert_fail(
                b"(inv->d1 & GMP_LIMB_HIGHBIT) != 0\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                1112 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 104],
                    &[i8; 104],
                >(
                    b"void mpn_div_qr_preinv(mp_ptr, mp_ptr, mp_size_t, mp_srcptr, mp_size_t, const struct gmp_div_inverse *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_9588: {
            if (*inv).d1
                & (1 as i32 as mp_limb_t)
                    << (::core::mem::size_of::<mp_limb_t>() as u64)
                        .wrapping_mul(8 as i32 as u64)
                        .wrapping_sub(1 as i32 as u64) != 0 as i32 as u64
            {} else {
                __assert_fail(
                    b"(inv->d1 & GMP_LIMB_HIGHBIT) != 0\0" as *const u8 as *const i8,
                    b"../mini-gmp.c\0" as *const u8 as *const i8,
                    1112 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 104],
                        &[i8; 104],
                    >(
                        b"void mpn_div_qr_preinv(mp_ptr, mp_ptr, mp_size_t, mp_srcptr, mp_size_t, const struct gmp_div_inverse *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        shift = (*inv).shift;
        if shift > 0 as i32 as u32 {
            nh = mpn_lshift(np, np as mp_srcptr, nn, shift);
        } else {
            nh = 0 as i32 as mp_limb_t;
        }
        mpn_div_qr_pi1(qp, np, nn, nh, dp, dn, (*inv).di);
        if shift > 0 as i32 as u32 {
            let mut __cy: mp_limb_t = mpn_rshift(np, np as mp_srcptr, dn, shift);
            if __cy == 0 as i32 as u64 {} else {
                __assert_fail(
                    b"__cy == 0\0" as *const u8 as *const i8,
                    b"../mini-gmp.c\0" as *const u8 as *const i8,
                    1123 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 104],
                        &[i8; 104],
                    >(
                        b"void mpn_div_qr_preinv(mp_ptr, mp_ptr, mp_size_t, mp_srcptr, mp_size_t, const struct gmp_div_inverse *)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_7587: {
                if __cy == 0 as i32 as u64 {} else {
                    __assert_fail(
                        b"__cy == 0\0" as *const u8 as *const i8,
                        b"../mini-gmp.c\0" as *const u8 as *const i8,
                        1123 as i32 as u32,
                        (*::core::mem::transmute::<
                            &[u8; 104],
                            &[i8; 104],
                        >(
                            b"void mpn_div_qr_preinv(mp_ptr, mp_ptr, mp_size_t, mp_srcptr, mp_size_t, const struct gmp_div_inverse *)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
        }
    };
}
unsafe extern "C" fn mpn_div_qr(
    mut qp: mp_ptr,
    mut np: mp_ptr,
    mut nn: mp_size_t,
    mut dp: mp_srcptr,
    mut dn: mp_size_t,
) {
    let mut inv: gmp_div_inverse = gmp_div_inverse {
        shift: 0,
        d1: 0,
        d0: 0,
        di: 0,
    };
    let mut tp: mp_ptr = 0 as mp_ptr;
    if dn > 0 as i32 as i64 {} else {
        __assert_fail(
            b"dn > 0\0" as *const u8 as *const i8,
            b"../mini-gmp.c\0" as *const u8 as *const i8,
            1133 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 65],
                &[i8; 65],
            >(b"void mpn_div_qr(mp_ptr, mp_ptr, mp_size_t, mp_srcptr, mp_size_t)\0"))
                .as_ptr(),
        );
    }
    'c_13400: {
        if dn > 0 as i32 as i64 {} else {
            __assert_fail(
                b"dn > 0\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                1133 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 65],
                    &[i8; 65],
                >(b"void mpn_div_qr(mp_ptr, mp_ptr, mp_size_t, mp_srcptr, mp_size_t)\0"))
                    .as_ptr(),
            );
        }
    };
    if nn >= dn {} else {
        __assert_fail(
            b"nn >= dn\0" as *const u8 as *const i8,
            b"../mini-gmp.c\0" as *const u8 as *const i8,
            1134 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 65],
                &[i8; 65],
            >(b"void mpn_div_qr(mp_ptr, mp_ptr, mp_size_t, mp_srcptr, mp_size_t)\0"))
                .as_ptr(),
        );
    }
    'c_13362: {
        if nn >= dn {} else {
            __assert_fail(
                b"nn >= dn\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                1134 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 65],
                    &[i8; 65],
                >(b"void mpn_div_qr(mp_ptr, mp_ptr, mp_size_t, mp_srcptr, mp_size_t)\0"))
                    .as_ptr(),
            );
        }
    };
    mpn_div_qr_invert(&mut inv, dp, dn);
    if dn > 2 as i32 as i64 && inv.shift > 0 as i32 as u32 {
        tp = gmp_xalloc_limbs(dn);
        let mut __cy: mp_limb_t = mpn_lshift(tp, dp, dn, inv.shift);
        if __cy == 0 as i32 as u64 {} else {
            __assert_fail(
                b"__cy == 0\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                1140 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 65],
                    &[i8; 65],
                >(b"void mpn_div_qr(mp_ptr, mp_ptr, mp_size_t, mp_srcptr, mp_size_t)\0"))
                    .as_ptr(),
            );
        }
        'c_11815: {
            if __cy == 0 as i32 as u64 {} else {
                __assert_fail(
                    b"__cy == 0\0" as *const u8 as *const i8,
                    b"../mini-gmp.c\0" as *const u8 as *const i8,
                    1140 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 65],
                        &[i8; 65],
                    >(
                        b"void mpn_div_qr(mp_ptr, mp_ptr, mp_size_t, mp_srcptr, mp_size_t)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        dp = tp as mp_srcptr;
    }
    mpn_div_qr_preinv(qp, np, nn, dp, dn, &mut inv);
    if !tp.is_null() {
        (Some(gmp_free_func.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(tp as *mut libc::c_void, 0 as i32 as size_t);
    }
}
unsafe extern "C" fn mpn_base_power_of_two_p(mut b: u32) -> u32 {
    match b {
        2 => return 1 as i32 as u32,
        4 => return 2 as i32 as u32,
        8 => return 3 as i32 as u32,
        16 => return 4 as i32 as u32,
        32 => return 5 as i32 as u32,
        64 => return 6 as i32 as u32,
        128 => return 7 as i32 as u32,
        256 => return 8 as i32 as u32,
        _ => return 0 as i32 as u32,
    };
}
unsafe extern "C" fn mpn_get_base_info(mut info: *mut mpn_base_info, mut b: mp_limb_t) {
    let mut m: mp_limb_t = 0;
    let mut p: mp_limb_t = 0;
    let mut exp: u32 = 0;
    m = (!(0 as i32 as mp_limb_t)).wrapping_div(b);
    exp = 1 as i32 as u32;
    p = b;
    while p <= m {
        p = (p as u64).wrapping_mul(b) as mp_limb_t as mp_limb_t;
        exp = exp.wrapping_add(1);
        exp;
    }
    (*info).exp = exp;
    (*info).bb = p;
}
unsafe extern "C" fn mpn_limb_size_in_base_2(mut u: mp_limb_t) -> mp_bitcnt_t {
    let mut shift: u32 = 0;
    if u > 0 as i32 as u64 {} else {
        __assert_fail(
            b"u > 0\0" as *const u8 as *const i8,
            b"../mini-gmp.c\0" as *const u8 as *const i8,
            1195 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 47],
                &[i8; 47],
            >(b"mp_bitcnt_t mpn_limb_size_in_base_2(mp_limb_t)\0"))
                .as_ptr(),
        );
    }
    'c_14622: {
        if u > 0 as i32 as u64 {} else {
            __assert_fail(
                b"u > 0\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                1195 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 47],
                    &[i8; 47],
                >(b"mp_bitcnt_t mpn_limb_size_in_base_2(mp_limb_t)\0"))
                    .as_ptr(),
            );
        }
    };
    let mut __clz_x: mp_limb_t = u;
    let mut __clz_c: u32 = 0 as i32 as u32;
    let mut LOCAL_SHIFT_BITS: i32 = 8 as i32;
    if (::core::mem::size_of::<mp_limb_t>() as u64).wrapping_mul(8 as i32 as u64)
        > LOCAL_SHIFT_BITS as u64
    {
        while __clz_x
            & (0xff as i32 as mp_limb_t)
                << (::core::mem::size_of::<mp_limb_t>() as u64)
                    .wrapping_mul(8 as i32 as u64)
                    .wrapping_sub(8 as i32 as u64) == 0 as i32 as u64
        {
            __clz_x <<= LOCAL_SHIFT_BITS;
            __clz_c = __clz_c.wrapping_add(8 as i32 as u32);
        }
    }
    while __clz_x
        & (1 as i32 as mp_limb_t)
            << (::core::mem::size_of::<mp_limb_t>() as u64)
                .wrapping_mul(8 as i32 as u64)
                .wrapping_sub(1 as i32 as u64) == 0 as i32 as u64
    {
        __clz_x <<= 1 as i32;
        __clz_c = __clz_c.wrapping_add(1);
        __clz_c;
    }
    shift = __clz_c;
    return (::core::mem::size_of::<mp_limb_t>() as u64)
        .wrapping_mul(8 as i32 as u64)
        .wrapping_sub(shift as u64);
}
unsafe extern "C" fn mpn_get_str_bits(
    mut sp: *mut u8,
    mut bits: u32,
    mut up: mp_srcptr,
    mut un: mp_size_t,
) -> size_t {
    let mut mask: u8 = 0;
    let mut sn: size_t = 0;
    let mut j: size_t = 0;
    let mut i: mp_size_t = 0;
    let mut shift: u32 = 0;
    sn = ((un - 1 as i32 as i64) as u64)
        .wrapping_mul(
            (::core::mem::size_of::<mp_limb_t>() as u64).wrapping_mul(8 as i32 as u64),
        )
        .wrapping_add(
            mpn_limb_size_in_base_2(*up.offset((un - 1 as i32 as i64) as isize)),
        )
        .wrapping_add(bits as u64)
        .wrapping_sub(1 as i32 as u64)
        .wrapping_div(bits as u64);
    mask = ((1 as u32) << bits).wrapping_sub(1 as i32 as u32) as u8;
    i = 0 as i32 as mp_size_t;
    j = sn;
    shift = 0 as i32 as u32;
    loop {
        let fresh15 = j;
        j = j.wrapping_sub(1);
        if !(fresh15 > 0 as i32 as u64) {
            break;
        }
        let mut digit: u8 = (*up.offset(i as isize) >> shift) as u8;
        shift = shift.wrapping_add(bits);
        if shift as u64
            >= (::core::mem::size_of::<mp_limb_t>() as u64).wrapping_mul(8 as i32 as u64)
            && {
                i += 1;
                i < un
            }
        {
            shift = (shift as u64)
                .wrapping_sub(
                    (::core::mem::size_of::<mp_limb_t>() as u64)
                        .wrapping_mul(8 as i32 as u64),
                ) as u32 as u32;
            digit = (digit as u64 | *up.offset(i as isize) << bits.wrapping_sub(shift))
                as u8;
        }
        *sp.offset(j as isize) = (digit as i32 & mask as i32) as u8;
    }
    return sn;
}
unsafe extern "C" fn mpn_limb_get_str(
    mut sp: *mut u8,
    mut w: mp_limb_t,
    mut binv: *const gmp_div_inverse,
) -> size_t {
    let mut i: mp_size_t = 0;
    i = 0 as i32 as mp_size_t;
    while w > 0 as i32 as u64 {
        let mut h: mp_limb_t = 0;
        let mut l: mp_limb_t = 0;
        let mut r: mp_limb_t = 0;
        h = w
            >> (::core::mem::size_of::<mp_limb_t>() as u64)
                .wrapping_mul(8 as i32 as u64)
                .wrapping_sub((*binv).shift as u64);
        l = w << (*binv).shift;
        let mut _qh: mp_limb_t = 0;
        let mut _ql: mp_limb_t = 0;
        let mut _r: mp_limb_t = 0;
        let mut _mask: mp_limb_t = 0;
        let mut LOCAL_GMP_LIMB_BITS: i32 = (::core::mem::size_of::<mp_limb_t>() as u64)
            .wrapping_mul(8 as i32 as u64) as i32;
        if (::core::mem::size_of::<u32>() as u64).wrapping_mul(8 as i32 as u64)
            >= (2 as i32 as u64)
                .wrapping_mul(
                    (::core::mem::size_of::<mp_limb_t>() as u64)
                        .wrapping_mul(8 as i32 as u64),
                )
        {
            let mut __ww: u32 = (h as u32 as u64).wrapping_mul((*binv).di) as u32;
            _ql = __ww as mp_limb_t;
            _qh = (__ww >> LOCAL_GMP_LIMB_BITS) as mp_limb_t;
        } else if (::core::mem::size_of::<u64>() as u64).wrapping_mul(8 as i32 as u64)
            >= (2 as i32 as u64)
                .wrapping_mul(
                    (::core::mem::size_of::<mp_limb_t>() as u64)
                        .wrapping_mul(8 as i32 as u64),
                )
        {
            let mut __ww_0: u64 = h.wrapping_mul((*binv).di);
            _ql = __ww_0;
            _qh = __ww_0 >> LOCAL_GMP_LIMB_BITS;
        } else {
            let mut __x0: mp_limb_t = 0;
            let mut __x1: mp_limb_t = 0;
            let mut __x2: mp_limb_t = 0;
            let mut __x3: mp_limb_t = 0;
            let mut __ul: u32 = 0;
            let mut __vl: u32 = 0;
            let mut __uh: u32 = 0;
            let mut __vh: u32 = 0;
            let mut __u: mp_limb_t = h;
            let mut __v: mp_limb_t = (*binv).di;
            __ul = (__u
                & ((1 as i32 as mp_limb_t)
                    << (::core::mem::size_of::<mp_limb_t>() as u64)
                        .wrapping_mul(8 as i32 as u64)
                        .wrapping_div(2 as i32 as u64))
                    .wrapping_sub(1 as i32 as u64)) as u32;
            __uh = (__u
                >> (::core::mem::size_of::<mp_limb_t>() as u64)
                    .wrapping_mul(8 as i32 as u64)
                    .wrapping_div(2 as i32 as u64)) as u32;
            __vl = (__v
                & ((1 as i32 as mp_limb_t)
                    << (::core::mem::size_of::<mp_limb_t>() as u64)
                        .wrapping_mul(8 as i32 as u64)
                        .wrapping_div(2 as i32 as u64))
                    .wrapping_sub(1 as i32 as u64)) as u32;
            __vh = (__v
                >> (::core::mem::size_of::<mp_limb_t>() as u64)
                    .wrapping_mul(8 as i32 as u64)
                    .wrapping_div(2 as i32 as u64)) as u32;
            __x0 = (__ul as mp_limb_t).wrapping_mul(__vl as u64);
            __x1 = (__ul as mp_limb_t).wrapping_mul(__vh as u64);
            __x2 = (__uh as mp_limb_t).wrapping_mul(__vl as u64);
            __x3 = (__uh as mp_limb_t).wrapping_mul(__vh as u64);
            __x1 = (__x1 as u64)
                .wrapping_add(
                    __x0
                        >> (::core::mem::size_of::<mp_limb_t>() as u64)
                            .wrapping_mul(8 as i32 as u64)
                            .wrapping_div(2 as i32 as u64),
                ) as mp_limb_t as mp_limb_t;
            __x1 = (__x1 as u64).wrapping_add(__x2) as mp_limb_t as mp_limb_t;
            if __x1 < __x2 {
                __x3 = (__x3 as u64)
                    .wrapping_add(
                        (1 as i32 as mp_limb_t)
                            << (::core::mem::size_of::<mp_limb_t>() as u64)
                                .wrapping_mul(8 as i32 as u64)
                                .wrapping_div(2 as i32 as u64),
                    ) as mp_limb_t as mp_limb_t;
            }
            _qh = __x3
                .wrapping_add(
                    __x1
                        >> (::core::mem::size_of::<mp_limb_t>() as u64)
                            .wrapping_mul(8 as i32 as u64)
                            .wrapping_div(2 as i32 as u64),
                );
            _ql = (__x1
                << (::core::mem::size_of::<mp_limb_t>() as u64)
                    .wrapping_mul(8 as i32 as u64)
                    .wrapping_div(2 as i32 as u64))
                .wrapping_add(
                    __x0
                        & ((1 as i32 as mp_limb_t)
                            << (::core::mem::size_of::<mp_limb_t>() as u64)
                                .wrapping_mul(8 as i32 as u64)
                                .wrapping_div(2 as i32 as u64))
                            .wrapping_sub(1 as i32 as u64),
                );
        }
        let mut __x: mp_limb_t = 0;
        __x = _ql.wrapping_add(l);
        _qh = _qh
            .wrapping_add(h.wrapping_add(1 as i32 as u64))
            .wrapping_add((__x < _ql) as i32 as u64);
        _ql = __x;
        _r = l.wrapping_sub(_qh.wrapping_mul((*binv).d1));
        _mask = ((_r > _ql) as i32 as mp_limb_t).wrapping_neg();
        _qh = (_qh as u64).wrapping_add(_mask) as mp_limb_t as mp_limb_t;
        _r = (_r as u64).wrapping_add(_mask & (*binv).d1) as mp_limb_t as mp_limb_t;
        if _r >= (*binv).d1 {
            _r = (_r as u64).wrapping_sub((*binv).d1) as mp_limb_t as mp_limb_t;
            _qh = _qh.wrapping_add(1);
            _qh;
        }
        r = _r;
        w = _qh;
        if r
            & !(0 as i32 as mp_limb_t)
                >> (::core::mem::size_of::<mp_limb_t>() as u64)
                    .wrapping_mul(8 as i32 as u64)
                    .wrapping_sub((*binv).shift as u64) == 0 as i32 as u64
        {} else {
            __assert_fail(
                b"(r & (GMP_LIMB_MAX >> (GMP_LIMB_BITS - binv->shift))) == 0\0"
                    as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                1244 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 84],
                    &[i8; 84],
                >(
                    b"size_t mpn_limb_get_str(unsigned char *, mp_limb_t, const struct gmp_div_inverse *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_16867: {
            if r
                & !(0 as i32 as mp_limb_t)
                    >> (::core::mem::size_of::<mp_limb_t>() as u64)
                        .wrapping_mul(8 as i32 as u64)
                        .wrapping_sub((*binv).shift as u64) == 0 as i32 as u64
            {} else {
                __assert_fail(
                    b"(r & (GMP_LIMB_MAX >> (GMP_LIMB_BITS - binv->shift))) == 0\0"
                        as *const u8 as *const i8,
                    b"../mini-gmp.c\0" as *const u8 as *const i8,
                    1244 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 84],
                        &[i8; 84],
                    >(
                        b"size_t mpn_limb_get_str(unsigned char *, mp_limb_t, const struct gmp_div_inverse *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        r >>= (*binv).shift;
        *sp.offset(i as isize) = r as u8;
        i += 1;
        i;
    }
    return i as size_t;
}
unsafe extern "C" fn mpn_get_str_other(
    mut sp: *mut u8,
    mut base: i32,
    mut info: *const mpn_base_info,
    mut up: mp_ptr,
    mut un: mp_size_t,
) -> size_t {
    let mut binv: gmp_div_inverse = gmp_div_inverse {
        shift: 0,
        d1: 0,
        d0: 0,
        di: 0,
    };
    let mut sn: size_t = 0;
    let mut i: size_t = 0;
    mpn_div_qr_1_invert(&mut binv, base as mp_limb_t);
    sn = 0 as i32 as size_t;
    if un > 1 as i32 as i64 {
        let mut bbinv: gmp_div_inverse = gmp_div_inverse {
            shift: 0,
            d1: 0,
            d0: 0,
            di: 0,
        };
        mpn_div_qr_1_invert(&mut bbinv, (*info).bb);
        loop {
            let mut w: mp_limb_t = 0;
            let mut done: size_t = 0;
            w = mpn_div_qr_1_preinv(up, up as mp_srcptr, un, &mut bbinv);
            un
                -= (*up.offset((un - 1 as i32 as i64) as isize) == 0 as i32 as u64)
                    as i32 as i64;
            done = mpn_limb_get_str(sp.offset(sn as isize), w, &mut binv);
            sn = (sn as u64).wrapping_add(done) as size_t as size_t;
            while done < (*info).exp as u64 {
                let fresh16 = sn;
                sn = sn.wrapping_add(1);
                *sp.offset(fresh16 as isize) = 0 as i32 as u8;
                done = done.wrapping_add(1);
                done;
            }
            if !(un > 1 as i32 as i64) {
                break;
            }
        }
    }
    sn = (sn as u64)
        .wrapping_add(
            mpn_limb_get_str(
                sp.offset(sn as isize),
                *up.offset(0 as i32 as isize),
                &mut binv,
            ),
        ) as size_t as size_t;
    i = 0 as i32 as size_t;
    while (2 as i32 as u64).wrapping_mul(i).wrapping_add(1 as i32 as u64) < sn {
        let mut t: u8 = *sp.offset(i as isize);
        *sp.offset(i as isize) = *sp
            .offset(sn.wrapping_sub(i).wrapping_sub(1 as i32 as u64) as isize);
        *sp.offset(sn.wrapping_sub(i).wrapping_sub(1 as i32 as u64) as isize) = t;
        i = i.wrapping_add(1);
        i;
    }
    return sn;
}
#[no_mangle]
pub unsafe extern "C" fn mpn_get_str(
    mut sp: *mut u8,
    mut base: i32,
    mut up: mp_ptr,
    mut un: mp_size_t,
) -> size_t {
    let mut bits: u32 = 0;
    if un > 0 as i32 as i64 {} else {
        __assert_fail(
            b"un > 0\0" as *const u8 as *const i8,
            b"../mini-gmp.c\0" as *const u8 as *const i8,
            1301 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 60],
                &[i8; 60],
            >(b"size_t mpn_get_str(unsigned char *, int, mp_ptr, mp_size_t)\0"))
                .as_ptr(),
        );
    }
    'c_17948: {
        if un > 0 as i32 as i64 {} else {
            __assert_fail(
                b"un > 0\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                1301 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 60],
                    &[i8; 60],
                >(b"size_t mpn_get_str(unsigned char *, int, mp_ptr, mp_size_t)\0"))
                    .as_ptr(),
            );
        }
    };
    if *up.offset((un - 1 as i32 as i64) as isize) > 0 as i32 as u64 {} else {
        __assert_fail(
            b"up[un-1] > 0\0" as *const u8 as *const i8,
            b"../mini-gmp.c\0" as *const u8 as *const i8,
            1302 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 60],
                &[i8; 60],
            >(b"size_t mpn_get_str(unsigned char *, int, mp_ptr, mp_size_t)\0"))
                .as_ptr(),
        );
    }
    'c_17896: {
        if *up.offset((un - 1 as i32 as i64) as isize) > 0 as i32 as u64 {} else {
            __assert_fail(
                b"up[un-1] > 0\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                1302 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 60],
                    &[i8; 60],
                >(b"size_t mpn_get_str(unsigned char *, int, mp_ptr, mp_size_t)\0"))
                    .as_ptr(),
            );
        }
    };
    bits = mpn_base_power_of_two_p(base as u32);
    if bits != 0 {
        return mpn_get_str_bits(sp, bits, up as mp_srcptr, un)
    } else {
        let mut info: mpn_base_info = mpn_base_info { exp: 0, bb: 0 };
        mpn_get_base_info(&mut info, base as mp_limb_t);
        return mpn_get_str_other(sp, base, &mut info, up, un);
    };
}
unsafe extern "C" fn mpn_set_str_bits(
    mut rp: mp_ptr,
    mut sp: *const u8,
    mut sn: size_t,
    mut bits: u32,
) -> mp_size_t {
    let mut rn: mp_size_t = 0;
    let mut j: size_t = 0;
    let mut shift: u32 = 0;
    j = sn;
    rn = 0 as i32 as mp_size_t;
    shift = 0 as i32 as u32;
    loop {
        let fresh17 = j;
        j = j.wrapping_sub(1);
        if !(fresh17 > 0 as i32 as u64) {
            break;
        }
        if shift == 0 as i32 as u32 {
            let fresh18 = rn;
            rn = rn + 1;
            *rp.offset(fresh18 as isize) = *sp.offset(j as isize) as mp_limb_t;
            shift = shift.wrapping_add(bits);
        } else {
            let ref mut fresh19 = *rp.offset((rn - 1 as i32 as i64) as isize);
            *fresh19 |= (*sp.offset(j as isize) as mp_limb_t) << shift;
            shift = shift.wrapping_add(bits);
            if shift as u64
                >= (::core::mem::size_of::<mp_limb_t>() as u64)
                    .wrapping_mul(8 as i32 as u64)
            {
                shift = (shift as u64)
                    .wrapping_sub(
                        (::core::mem::size_of::<mp_limb_t>() as u64)
                            .wrapping_mul(8 as i32 as u64),
                    ) as u32 as u32;
                if shift > 0 as i32 as u32 {
                    let fresh20 = rn;
                    rn = rn + 1;
                    *rp.offset(fresh20 as isize) = *sp.offset(j as isize) as mp_limb_t
                        >> bits.wrapping_sub(shift);
                }
            }
        }
    }
    rn = mpn_normalized_size(rp as mp_srcptr, rn);
    return rn;
}
unsafe extern "C" fn mpn_set_str_other(
    mut rp: mp_ptr,
    mut sp: *const u8,
    mut sn: size_t,
    mut b: mp_limb_t,
    mut info: *const mpn_base_info,
) -> mp_size_t {
    let mut rn: mp_size_t = 0;
    let mut w: mp_limb_t = 0;
    let mut k: u32 = 0;
    let mut j: size_t = 0;
    if sn > 0 as i32 as u64 {} else {
        __assert_fail(
            b"sn > 0\0" as *const u8 as *const i8,
            b"../mini-gmp.c\0" as *const u8 as *const i8,
            1358 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 108],
                &[i8; 108],
            >(
                b"mp_size_t mpn_set_str_other(mp_ptr, const unsigned char *, size_t, mp_limb_t, const struct mpn_base_info *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_18247: {
        if sn > 0 as i32 as u64 {} else {
            __assert_fail(
                b"sn > 0\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                1358 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 108],
                    &[i8; 108],
                >(
                    b"mp_size_t mpn_set_str_other(mp_ptr, const unsigned char *, size_t, mp_limb_t, const struct mpn_base_info *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    k = (1 as i32 as u64)
        .wrapping_add(sn.wrapping_sub(1 as i32 as u64).wrapping_rem((*info).exp as u64))
        as u32;
    j = 0 as i32 as size_t;
    let fresh21 = j;
    j = j.wrapping_add(1);
    w = *sp.offset(fresh21 as isize) as mp_limb_t;
    loop {
        k = k.wrapping_sub(1);
        if !(k != 0 as i32 as u32) {
            break;
        }
        let fresh22 = j;
        j = j.wrapping_add(1);
        w = w.wrapping_mul(b).wrapping_add(*sp.offset(fresh22 as isize) as u64);
    }
    *rp.offset(0 as i32 as isize) = w;
    rn = 1 as i32 as mp_size_t;
    while j < sn {
        let mut cy: mp_limb_t = 0;
        let fresh23 = j;
        j = j.wrapping_add(1);
        w = *sp.offset(fresh23 as isize) as mp_limb_t;
        k = 1 as i32 as u32;
        while k < (*info).exp {
            let fresh24 = j;
            j = j.wrapping_add(1);
            w = w.wrapping_mul(b).wrapping_add(*sp.offset(fresh24 as isize) as u64);
            k = k.wrapping_add(1);
            k;
        }
        cy = mpn_mul_1(rp, rp as mp_srcptr, rn, (*info).bb);
        cy = (cy as u64).wrapping_add(mpn_add_1(rp, rp as mp_srcptr, rn, w)) as mp_limb_t
            as mp_limb_t;
        if cy > 0 as i32 as u64 {
            let fresh25 = rn;
            rn = rn + 1;
            *rp.offset(fresh25 as isize) = cy;
        }
    }
    if j == sn {} else {
        __assert_fail(
            b"j == sn\0" as *const u8 as *const i8,
            b"../mini-gmp.c\0" as *const u8 as *const i8,
            1382 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 108],
                &[i8; 108],
            >(
                b"mp_size_t mpn_set_str_other(mp_ptr, const unsigned char *, size_t, mp_limb_t, const struct mpn_base_info *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_18045: {
        if j == sn {} else {
            __assert_fail(
                b"j == sn\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                1382 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 108],
                    &[i8; 108],
                >(
                    b"mp_size_t mpn_set_str_other(mp_ptr, const unsigned char *, size_t, mp_limb_t, const struct mpn_base_info *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    return rn;
}
#[no_mangle]
pub unsafe extern "C" fn mpn_set_str(
    mut rp: mp_ptr,
    mut sp: *const u8,
    mut sn: size_t,
    mut base: i32,
) -> mp_size_t {
    let mut bits: u32 = 0;
    if sn == 0 as i32 as u64 {
        return 0 as i32 as mp_size_t;
    }
    bits = mpn_base_power_of_two_p(base as u32);
    if bits != 0 {
        return mpn_set_str_bits(rp, sp, sn, bits)
    } else {
        let mut info: mpn_base_info = mpn_base_info { exp: 0, bb: 0 };
        mpn_get_base_info(&mut info, base as mp_limb_t);
        return mpn_set_str_other(rp, sp, sn, base as mp_limb_t, &mut info);
    };
}
#[no_mangle]
pub unsafe extern "C" fn mpz_init(mut r: *mut __mpz_struct) {
    static mut dummy_limb: mp_limb_t = !(0 as i32 as mp_limb_t) & 0xc1a0 as i32 as u64;
    (*r)._mp_alloc = 0 as i32;
    (*r)._mp_size = 0 as i32;
    (*r)._mp_d = &dummy_limb as *const mp_limb_t as mp_ptr;
}
#[no_mangle]
pub unsafe extern "C" fn mpz_init2(mut r: *mut __mpz_struct, mut bits: mp_bitcnt_t) {
    let mut rn: mp_size_t = 0;
    bits = (bits as u64).wrapping_sub((bits != 0 as i32 as u64) as i32 as u64)
        as mp_bitcnt_t as mp_bitcnt_t;
    rn = (1 as i32 as u64)
        .wrapping_add(
            bits
                .wrapping_div(
                    (::core::mem::size_of::<mp_limb_t>() as u64)
                        .wrapping_mul(8 as i32 as u64),
                ),
        ) as mp_size_t;
    (*r)._mp_alloc = rn as i32;
    (*r)._mp_size = 0 as i32;
    (*r)._mp_d = gmp_xalloc_limbs(rn);
}
#[no_mangle]
pub unsafe extern "C" fn mpz_clear(mut r: *mut __mpz_struct) {
    if (*r)._mp_alloc != 0 {
        (Some(gmp_free_func.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )((*r)._mp_d as *mut libc::c_void, 0 as i32 as size_t);
    }
}
unsafe extern "C" fn mpz_realloc(
    mut r: *mut __mpz_struct,
    mut size: mp_size_t,
) -> mp_ptr {
    size = if size > 1 as i32 as i64 { size } else { 1 as i32 as i64 };
    if (*r)._mp_alloc != 0 {
        (*r)._mp_d = gmp_xrealloc_limbs((*r)._mp_d, size);
    } else {
        (*r)._mp_d = gmp_xalloc_limbs(size);
    }
    (*r)._mp_alloc = size as i32;
    if (if (*r)._mp_size >= 0 as i32 { (*r)._mp_size } else { -(*r)._mp_size }) as i64
        > size
    {
        (*r)._mp_size = 0 as i32;
    }
    return (*r)._mp_d;
}
#[no_mangle]
pub unsafe extern "C" fn mpz_set_si(mut r: *mut __mpz_struct, mut x: i64) {
    if x >= 0 as i32 as i64 {
        mpz_set_ui(r, x as u64);
    } else if (::core::mem::size_of::<mp_limb_t>() as u64).wrapping_mul(8 as i32 as u64)
        < (::core::mem::size_of::<u64>() as u64).wrapping_mul(8 as i32 as u64)
    {
        mpz_set_ui(
            r,
            ((x + 1 as i32 as i64) as u64).wrapping_sub(1 as i32 as u64).wrapping_neg(),
        );
        mpz_neg(r, r as *const __mpz_struct);
    } else {
        (*r)._mp_size = -(1 as i32);
        *if 1 as i32 > (*r)._mp_alloc {
            mpz_realloc(r, 1 as i32 as mp_size_t)
        } else {
            (*r)._mp_d
        }
            .offset(0 as i32 as isize) = ((x + 1 as i32 as i64) as u64)
            .wrapping_sub(1 as i32 as u64)
            .wrapping_neg();
    };
}
#[no_mangle]
pub unsafe extern "C" fn mpz_set_ui(mut r: *mut __mpz_struct, mut x: u64) {
    if x > 0 as i32 as u64 {
        (*r)._mp_size = 1 as i32;
        *if 1 as i32 > (*r)._mp_alloc {
            mpz_realloc(r, 1 as i32 as mp_size_t)
        } else {
            (*r)._mp_d
        }
            .offset(0 as i32 as isize) = x;
        if (::core::mem::size_of::<mp_limb_t>() as u64).wrapping_mul(8 as i32 as u64)
            < (::core::mem::size_of::<u64>() as u64).wrapping_mul(8 as i32 as u64)
        {
            let mut LOCAL_GMP_LIMB_BITS: i32 = (::core::mem::size_of::<mp_limb_t>()
                as u64)
                .wrapping_mul(8 as i32 as u64) as i32;
            loop {
                x >>= LOCAL_GMP_LIMB_BITS;
                if !(x != 0) {
                    break;
                }
                (*r)._mp_size += 1;
                (*r)._mp_size;
                *if (*r)._mp_size > (*r)._mp_alloc {
                    mpz_realloc(r, (*r)._mp_size as mp_size_t)
                } else {
                    (*r)._mp_d
                }
                    .offset(((*r)._mp_size - 1 as i32) as isize) = x;
            }
        }
    } else {
        (*r)._mp_size = 0 as i32;
    };
}
#[no_mangle]
pub unsafe extern "C" fn mpz_set(mut r: *mut __mpz_struct, mut x: *const __mpz_struct) {
    if r != x as *mut __mpz_struct {
        let mut n: mp_size_t = 0;
        let mut rp: mp_ptr = 0 as *mut mp_limb_t;
        n = (if (*x)._mp_size >= 0 as i32 { (*x)._mp_size } else { -(*x)._mp_size })
            as mp_size_t;
        rp = if n > (*r)._mp_alloc as i64 { mpz_realloc(r, n) } else { (*r)._mp_d };
        mpn_copyi(rp, (*x)._mp_d as mp_srcptr, n);
        (*r)._mp_size = (*x)._mp_size;
    }
}
#[no_mangle]
pub unsafe extern "C" fn mpz_init_set_si(mut r: *mut __mpz_struct, mut x: i64) {
    mpz_init(r);
    mpz_set_si(r, x);
}
#[no_mangle]
pub unsafe extern "C" fn mpz_init_set_ui(mut r: *mut __mpz_struct, mut x: u64) {
    mpz_init(r);
    mpz_set_ui(r, x);
}
#[no_mangle]
pub unsafe extern "C" fn mpz_init_set(
    mut r: *mut __mpz_struct,
    mut x: *const __mpz_struct,
) {
    mpz_init(r);
    mpz_set(r, x);
}
#[no_mangle]
pub unsafe extern "C" fn mpz_fits_slong_p(mut u: *const __mpz_struct) -> i32 {
    return (mpz_cmp_si(u, 9223372036854775807 as i64) <= 0 as i32
        && mpz_cmp_si(u, -(9223372036854775807 as i64) - 1 as i64) >= 0 as i32) as i32;
}
unsafe extern "C" fn mpn_absfits_ulong_p(mut up: mp_srcptr, mut un: mp_size_t) -> i32 {
    let mut ulongsize: i32 = (::core::mem::size_of::<u64>() as u64)
        .wrapping_mul(8 as i32 as u64)
        .wrapping_div(
            (::core::mem::size_of::<mp_limb_t>() as u64).wrapping_mul(8 as i32 as u64),
        ) as i32;
    let mut ulongrem: mp_limb_t = 0 as i32 as mp_limb_t;
    if (::core::mem::size_of::<u64>() as u64)
        .wrapping_mul(8 as i32 as u64)
        .wrapping_rem(
            (::core::mem::size_of::<mp_limb_t>() as u64).wrapping_mul(8 as i32 as u64),
        ) != 0 as i32 as u64
    {
        ulongrem = ((9223372036854775807 as i64 as u64)
            .wrapping_mul(2 as u64)
            .wrapping_add(1 as u64)
            >> (::core::mem::size_of::<mp_limb_t>() as u64)
                .wrapping_mul(8 as i32 as u64)
                .wrapping_mul(ulongsize as u64))
            .wrapping_add(1 as i32 as u64);
    }
    return (un <= ulongsize as i64
        || *up.offset(ulongsize as isize) < ulongrem
            && un == (ulongsize + 1 as i32) as i64) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn mpz_fits_ulong_p(mut u: *const __mpz_struct) -> i32 {
    let mut us: mp_size_t = (*u)._mp_size as mp_size_t;
    return (us >= 0 as i32 as i64
        && mpn_absfits_ulong_p((*u)._mp_d as mp_srcptr, us) != 0) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn mpz_fits_sint_p(mut u: *const __mpz_struct) -> i32 {
    return (mpz_cmp_si(u, 2147483647 as i32 as i64) <= 0 as i32
        && mpz_cmp_si(u, (-(2147483647 as i32) - 1 as i32) as i64) >= 0 as i32) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn mpz_fits_uint_p(mut u: *const __mpz_struct) -> i32 {
    return ((*u)._mp_size >= 0 as i32
        && mpz_cmpabs_ui(
            u,
            (2147483647 as i32 as u32).wrapping_mul(2 as u32).wrapping_add(1 as u32)
                as u64,
        ) <= 0 as i32) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn mpz_fits_sshort_p(mut u: *const __mpz_struct) -> i32 {
    return (mpz_cmp_si(u, 32767 as i32 as i64) <= 0 as i32
        && mpz_cmp_si(u, (-(32767 as i32) - 1 as i32) as i64) >= 0 as i32) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn mpz_fits_ushort_p(mut u: *const __mpz_struct) -> i32 {
    return ((*u)._mp_size >= 0 as i32
        && mpz_cmpabs_ui(u, (32767 as i32 * 2 as i32 + 1 as i32) as u64) <= 0 as i32)
        as i32;
}
#[no_mangle]
pub unsafe extern "C" fn mpz_get_si(mut u: *const __mpz_struct) -> i64 {
    let mut r: u64 = mpz_get_ui(u);
    let mut c: u64 = (-(9223372036854775807 as i64)
        - (-(9223372036854775807 as i64) - 1 as i64)) as u64;
    if (*u)._mp_size < 0 as i32 {
        return -(c as i64)
            - (r.wrapping_sub(c) & 9223372036854775807 as i64 as u64) as i64
    } else {
        return (r & 9223372036854775807 as i64 as u64) as i64
    };
}
#[no_mangle]
pub unsafe extern "C" fn mpz_get_ui(mut u: *const __mpz_struct) -> u64 {
    if (::core::mem::size_of::<mp_limb_t>() as u64).wrapping_mul(8 as i32 as u64)
        < (::core::mem::size_of::<u64>() as u64).wrapping_mul(8 as i32 as u64)
    {
        let mut LOCAL_GMP_LIMB_BITS: i32 = (::core::mem::size_of::<mp_limb_t>() as u64)
            .wrapping_mul(8 as i32 as u64) as i32;
        let mut r: u64 = 0 as i32 as u64;
        let mut n: mp_size_t = (if (*u)._mp_size >= 0 as i32 {
            (*u)._mp_size
        } else {
            -(*u)._mp_size
        }) as mp_size_t;
        n = (if (n as u64)
            < (1 as i32 as u64)
                .wrapping_add(
                    ((::core::mem::size_of::<u64>() as u64)
                        .wrapping_mul(8 as i32 as u64)
                        .wrapping_sub(1 as i32 as u64) as mp_size_t as u64)
                        .wrapping_div(
                            (::core::mem::size_of::<mp_limb_t>() as u64)
                                .wrapping_mul(8 as i32 as u64),
                        ),
                )
        {
            n as u64
        } else {
            (1 as i32 as u64)
                .wrapping_add(
                    ((::core::mem::size_of::<u64>() as u64)
                        .wrapping_mul(8 as i32 as u64)
                        .wrapping_sub(1 as i32 as u64) as mp_size_t as u64)
                        .wrapping_div(
                            (::core::mem::size_of::<mp_limb_t>() as u64)
                                .wrapping_mul(8 as i32 as u64),
                        ),
                )
        }) as mp_size_t;
        loop {
            n -= 1;
            if !(n >= 0 as i32 as i64) {
                break;
            }
            r = (r << LOCAL_GMP_LIMB_BITS)
                .wrapping_add(*((*u)._mp_d).offset(n as isize));
        }
        return r;
    }
    return if (*u)._mp_size == 0 as i32 {
        0 as i32 as u64
    } else {
        *((*u)._mp_d).offset(0 as i32 as isize)
    };
}
#[no_mangle]
pub unsafe extern "C" fn mpz_size(mut u: *const __mpz_struct) -> size_t {
    return (if (*u)._mp_size >= 0 as i32 { (*u)._mp_size } else { -(*u)._mp_size })
        as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn mpz_getlimbn(
    mut u: *const __mpz_struct,
    mut n: mp_size_t,
) -> mp_limb_t {
    if n >= 0 as i32 as i64
        && n
            < (if (*u)._mp_size >= 0 as i32 { (*u)._mp_size } else { -(*u)._mp_size })
                as i64
    {
        return *((*u)._mp_d).offset(n as isize)
    } else {
        return 0 as i32 as mp_limb_t
    };
}
#[no_mangle]
pub unsafe extern "C" fn mpz_realloc2(mut x: *mut __mpz_struct, mut n: mp_bitcnt_t) {
    mpz_realloc(
        x,
        (1 as i32 as u64)
            .wrapping_add(
                n
                    .wrapping_sub((n != 0 as i32 as u64) as i32 as u64)
                    .wrapping_div(
                        (::core::mem::size_of::<mp_limb_t>() as u64)
                            .wrapping_mul(8 as i32 as u64),
                    ),
            ) as mp_size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn mpz_limbs_read(mut x: mpz_srcptr) -> mp_srcptr {
    return (*x)._mp_d as mp_srcptr;
}
#[no_mangle]
pub unsafe extern "C" fn mpz_limbs_modify(
    mut x: *mut __mpz_struct,
    mut n: mp_size_t,
) -> mp_ptr {
    if n > 0 as i32 as i64 {} else {
        __assert_fail(
            b"n > 0\0" as *const u8 as *const i8,
            b"../mini-gmp.c\0" as *const u8 as *const i8,
            1651 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 51],
                &[i8; 51],
            >(b"mp_ptr mpz_limbs_modify(__mpz_struct *, mp_size_t)\0"))
                .as_ptr(),
        );
    }
    'c_28272: {
        if n > 0 as i32 as i64 {} else {
            __assert_fail(
                b"n > 0\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                1651 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 51],
                    &[i8; 51],
                >(b"mp_ptr mpz_limbs_modify(__mpz_struct *, mp_size_t)\0"))
                    .as_ptr(),
            );
        }
    };
    return if n > (*x)._mp_alloc as i64 { mpz_realloc(x, n) } else { (*x)._mp_d };
}
#[no_mangle]
pub unsafe extern "C" fn mpz_limbs_write(
    mut x: *mut __mpz_struct,
    mut n: mp_size_t,
) -> mp_ptr {
    return mpz_limbs_modify(x, n);
}
#[no_mangle]
pub unsafe extern "C" fn mpz_limbs_finish(mut x: *mut __mpz_struct, mut xs: mp_size_t) {
    let mut xn: mp_size_t = 0;
    xn = mpn_normalized_size(
        (*x)._mp_d as mp_srcptr,
        if xs >= 0 as i32 as i64 { xs } else { -xs },
    );
    (*x)._mp_size = (if xs < 0 as i32 as i64 { -xn } else { xn }) as i32;
}
unsafe extern "C" fn mpz_roinit_normal_n(
    mut x: *mut __mpz_struct,
    mut xp: mp_srcptr,
    mut xs: mp_size_t,
) -> mpz_srcptr {
    (*x)._mp_alloc = 0 as i32;
    (*x)._mp_d = xp as mp_ptr;
    (*x)._mp_size = xs as i32;
    return x as mpz_srcptr;
}
#[no_mangle]
pub unsafe extern "C" fn mpz_roinit_n(
    mut x: *mut __mpz_struct,
    mut xp: mp_srcptr,
    mut xs: mp_size_t,
) -> mpz_srcptr {
    mpz_roinit_normal_n(x, xp, xs);
    mpz_limbs_finish(x, xs);
    return x as mpz_srcptr;
}
#[no_mangle]
pub unsafe extern "C" fn mpz_set_d(mut r: *mut __mpz_struct, mut x: libc::c_double) {
    let mut sign: i32 = 0;
    let mut rp: mp_ptr = 0 as *mut mp_limb_t;
    let mut rn: mp_size_t = 0;
    let mut i: mp_size_t = 0;
    let mut B: libc::c_double = 0.;
    let mut Bi: libc::c_double = 0.;
    let mut f: mp_limb_t = 0;
    if x != x || x == x * 0.5f64 {
        (*r)._mp_size = 0 as i32;
        return;
    }
    sign = (x < 0.0f64) as i32;
    if sign != 0 {
        x = -x;
    }
    if x < 1.0f64 {
        (*r)._mp_size = 0 as i32;
        return;
    }
    B = 4.0f64
        * ((1 as i32 as mp_limb_t)
            << (::core::mem::size_of::<mp_limb_t>() as u64)
                .wrapping_mul(8 as i32 as u64)
                .wrapping_sub(1 as i32 as u64) >> 1 as i32) as libc::c_double;
    Bi = 1.0f64 / B;
    rn = 1 as i32 as mp_size_t;
    while x >= B {
        x *= Bi;
        rn += 1;
        rn;
    }
    rp = if rn > (*r)._mp_alloc as i64 { mpz_realloc(r, rn) } else { (*r)._mp_d };
    f = x as mp_limb_t;
    x -= f as libc::c_double;
    if x < 1.0f64 {} else {
        __assert_fail(
            b"x < 1.0\0" as *const u8 as *const i8,
            b"../mini-gmp.c\0" as *const u8 as *const i8,
            1724 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 39],
                &[i8; 39],
            >(b"void mpz_set_d(__mpz_struct *, double)\0"))
                .as_ptr(),
        );
    }
    'c_28534: {
        if x < 1.0f64 {} else {
            __assert_fail(
                b"x < 1.0\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                1724 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 39],
                    &[i8; 39],
                >(b"void mpz_set_d(__mpz_struct *, double)\0"))
                    .as_ptr(),
            );
        }
    };
    i = rn - 1 as i32 as i64;
    *rp.offset(i as isize) = f;
    loop {
        i -= 1;
        if !(i >= 0 as i32 as i64) {
            break;
        }
        x = B * x;
        f = x as mp_limb_t;
        x -= f as libc::c_double;
        if x < 1.0f64 {} else {
            __assert_fail(
                b"x < 1.0\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                1732 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 39],
                    &[i8; 39],
                >(b"void mpz_set_d(__mpz_struct *, double)\0"))
                    .as_ptr(),
            );
        }
        'c_28460: {
            if x < 1.0f64 {} else {
                __assert_fail(
                    b"x < 1.0\0" as *const u8 as *const i8,
                    b"../mini-gmp.c\0" as *const u8 as *const i8,
                    1732 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 39],
                        &[i8; 39],
                    >(b"void mpz_set_d(__mpz_struct *, double)\0"))
                        .as_ptr(),
                );
            }
        };
        *rp.offset(i as isize) = f;
    }
    (*r)._mp_size = (if sign != 0 { -rn } else { rn }) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn mpz_init_set_d(
    mut r: *mut __mpz_struct,
    mut x: libc::c_double,
) {
    mpz_init(r);
    mpz_set_d(r, x);
}
#[no_mangle]
pub unsafe extern "C" fn mpz_get_d(mut u: *const __mpz_struct) -> libc::c_double {
    let mut m: i32 = 0;
    let mut l: mp_limb_t = 0;
    let mut un: mp_size_t = 0;
    let mut x: libc::c_double = 0.;
    let mut B: libc::c_double = 4.0f64
        * ((1 as i32 as mp_limb_t)
            << (::core::mem::size_of::<mp_limb_t>() as u64)
                .wrapping_mul(8 as i32 as u64)
                .wrapping_sub(1 as i32 as u64) >> 1 as i32) as libc::c_double;
    un = (if (*u)._mp_size >= 0 as i32 { (*u)._mp_size } else { -(*u)._mp_size })
        as mp_size_t;
    if un == 0 as i32 as i64 {
        return 0.0f64;
    }
    un -= 1;
    l = *((*u)._mp_d).offset(un as isize);
    let mut __clz_x: mp_limb_t = l;
    let mut __clz_c: u32 = 0 as i32 as u32;
    let mut LOCAL_SHIFT_BITS: i32 = 8 as i32;
    if (::core::mem::size_of::<mp_limb_t>() as u64).wrapping_mul(8 as i32 as u64)
        > LOCAL_SHIFT_BITS as u64
    {
        while __clz_x
            & (0xff as i32 as mp_limb_t)
                << (::core::mem::size_of::<mp_limb_t>() as u64)
                    .wrapping_mul(8 as i32 as u64)
                    .wrapping_sub(8 as i32 as u64) == 0 as i32 as u64
        {
            __clz_x <<= LOCAL_SHIFT_BITS;
            __clz_c = __clz_c.wrapping_add(8 as i32 as u32);
        }
    }
    while __clz_x
        & (1 as i32 as mp_limb_t)
            << (::core::mem::size_of::<mp_limb_t>() as u64)
                .wrapping_mul(8 as i32 as u64)
                .wrapping_sub(1 as i32 as u64) == 0 as i32 as u64
    {
        __clz_x <<= 1 as i32;
        __clz_c = __clz_c.wrapping_add(1);
        __clz_c;
    }
    m = __clz_c as i32;
    m = ((m + 53 as i32) as u64)
        .wrapping_sub(
            (::core::mem::size_of::<mp_limb_t>() as u64).wrapping_mul(8 as i32 as u64),
        ) as i32;
    if m < 0 as i32 {
        l &= !(0 as i32 as mp_limb_t) << -m;
    }
    x = l as libc::c_double;
    loop {
        un -= 1;
        if !(un >= 0 as i32 as i64) {
            break;
        }
        x = B * x;
        if m > 0 as i32 {
            l = *((*u)._mp_d).offset(un as isize);
            m = (m as u64)
                .wrapping_sub(
                    (::core::mem::size_of::<mp_limb_t>() as u64)
                        .wrapping_mul(8 as i32 as u64),
                ) as i32 as i32;
            if m < 0 as i32 {
                l &= !(0 as i32 as mp_limb_t) << -m;
            }
            x += l as libc::c_double;
        }
    }
    if (*u)._mp_size < 0 as i32 {
        x = -x;
    }
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn mpz_cmpabs_d(
    mut x: *const __mpz_struct,
    mut d: libc::c_double,
) -> i32 {
    let mut xn: mp_size_t = 0;
    let mut B: libc::c_double = 0.;
    let mut Bi: libc::c_double = 0.;
    let mut i: mp_size_t = 0;
    xn = (*x)._mp_size as mp_size_t;
    d = if d >= 0 as i32 as libc::c_double { d } else { -d };
    if xn != 0 as i32 as i64 {
        xn = if xn >= 0 as i32 as i64 { xn } else { -xn };
        B = 4.0f64
            * ((1 as i32 as mp_limb_t)
                << (::core::mem::size_of::<mp_limb_t>() as u64)
                    .wrapping_mul(8 as i32 as u64)
                    .wrapping_sub(1 as i32 as u64) >> 1 as i32) as libc::c_double;
        Bi = 1.0f64 / B;
        i = 1 as i32 as mp_size_t;
        while i < xn {
            d *= Bi;
            i += 1;
            i;
        }
        if d >= B {
            return -(1 as i32);
        }
        i = xn;
        loop {
            let fresh26 = i;
            i = i - 1;
            if !(fresh26 > 0 as i32 as i64) {
                break;
            }
            let mut f: mp_limb_t = 0;
            let mut xl: mp_limb_t = 0;
            f = d as mp_limb_t;
            xl = *((*x)._mp_d).offset(i as isize);
            if xl > f { return 1 as i32 } else if xl < f { return -(1 as i32) }
            d = B * (d - f as libc::c_double);
        }
    }
    return -((d > 0.0f64) as i32);
}
#[no_mangle]
pub unsafe extern "C" fn mpz_cmp_d(
    mut x: *const __mpz_struct,
    mut d: libc::c_double,
) -> i32 {
    if (*x)._mp_size < 0 as i32 {
        if d >= 0.0f64 { return -(1 as i32) } else { return -mpz_cmpabs_d(x, d) }
    } else if d < 0.0f64 {
        return 1 as i32
    } else {
        return mpz_cmpabs_d(x, d)
    };
}
#[no_mangle]
pub unsafe extern "C" fn mpz_sgn(mut u: *const __mpz_struct) -> i32 {
    return ((*u)._mp_size > 0 as i32) as i32 - ((*u)._mp_size < 0 as i32) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn mpz_cmp_si(mut u: *const __mpz_struct, mut v: i64) -> i32 {
    let mut usize: mp_size_t = (*u)._mp_size as mp_size_t;
    if v >= 0 as i32 as i64 {
        return mpz_cmp_ui(u, v as u64)
    } else if usize >= 0 as i32 as i64 {
        return 1 as i32
    } else {
        return -mpz_cmpabs_ui(
            u,
            ((v + 1 as i32 as i64) as u64).wrapping_sub(1 as i32 as u64).wrapping_neg(),
        )
    };
}
#[no_mangle]
pub unsafe extern "C" fn mpz_cmp_ui(mut u: *const __mpz_struct, mut v: u64) -> i32 {
    let mut usize: mp_size_t = (*u)._mp_size as mp_size_t;
    if usize < 0 as i32 as i64 {
        return -(1 as i32)
    } else {
        return mpz_cmpabs_ui(u, v)
    };
}
#[no_mangle]
pub unsafe extern "C" fn mpz_cmp(
    mut a: *const __mpz_struct,
    mut b: *const __mpz_struct,
) -> i32 {
    let mut asize: mp_size_t = (*a)._mp_size as mp_size_t;
    let mut bsize: mp_size_t = (*b)._mp_size as mp_size_t;
    if asize != bsize {
        return if asize < bsize { -(1 as i32) } else { 1 as i32 }
    } else if asize >= 0 as i32 as i64 {
        return mpn_cmp((*a)._mp_d as mp_srcptr, (*b)._mp_d as mp_srcptr, asize)
    } else {
        return mpn_cmp((*b)._mp_d as mp_srcptr, (*a)._mp_d as mp_srcptr, -asize)
    };
}
#[no_mangle]
pub unsafe extern "C" fn mpz_cmpabs_ui(mut u: *const __mpz_struct, mut v: u64) -> i32 {
    let mut un: mp_size_t = (if (*u)._mp_size >= 0 as i32 {
        (*u)._mp_size
    } else {
        -(*u)._mp_size
    }) as mp_size_t;
    if mpn_absfits_ulong_p((*u)._mp_d as mp_srcptr, un) == 0 {
        return 1 as i32
    } else {
        let mut uu: u64 = mpz_get_ui(u);
        return (uu > v) as i32 - (uu < v) as i32;
    };
}
#[no_mangle]
pub unsafe extern "C" fn mpz_cmpabs(
    mut u: *const __mpz_struct,
    mut v: *const __mpz_struct,
) -> i32 {
    return mpn_cmp4(
        (*u)._mp_d as mp_srcptr,
        (if (*u)._mp_size >= 0 as i32 { (*u)._mp_size } else { -(*u)._mp_size })
            as mp_size_t,
        (*v)._mp_d as mp_srcptr,
        (if (*v)._mp_size >= 0 as i32 { (*v)._mp_size } else { -(*v)._mp_size })
            as mp_size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn mpz_abs(mut r: *mut __mpz_struct, mut u: *const __mpz_struct) {
    mpz_set(r, u);
    (*r)._mp_size = if (*r)._mp_size >= 0 as i32 {
        (*r)._mp_size
    } else {
        -(*r)._mp_size
    };
}
#[no_mangle]
pub unsafe extern "C" fn mpz_neg(mut r: *mut __mpz_struct, mut u: *const __mpz_struct) {
    mpz_set(r, u);
    (*r)._mp_size = -(*r)._mp_size;
}
#[no_mangle]
pub unsafe extern "C" fn mpz_swap(mut u: *mut __mpz_struct, mut v: *mut __mpz_struct) {
    let mut __mp_size_t_swap__tmp: mp_size_t = (*u)._mp_size as mp_size_t;
    (*u)._mp_size = (*v)._mp_size;
    (*v)._mp_size = __mp_size_t_swap__tmp as i32;
    let mut __mp_size_t_swap__tmp_0: mp_size_t = (*u)._mp_alloc as mp_size_t;
    (*u)._mp_alloc = (*v)._mp_alloc;
    (*v)._mp_alloc = __mp_size_t_swap__tmp_0 as i32;
    let mut __mp_ptr_swap__tmp: mp_ptr = (*u)._mp_d;
    (*u)._mp_d = (*v)._mp_d;
    (*v)._mp_d = __mp_ptr_swap__tmp;
}
#[no_mangle]
pub unsafe extern "C" fn mpz_add_ui(
    mut r: *mut __mpz_struct,
    mut a: *const __mpz_struct,
    mut b: u64,
) {
    let mut bb: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    mpz_init_set_ui(bb.as_mut_ptr(), b);
    mpz_add(r, a, bb.as_mut_ptr() as *const __mpz_struct);
    mpz_clear(bb.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn mpz_sub_ui(
    mut r: *mut __mpz_struct,
    mut a: *const __mpz_struct,
    mut b: u64,
) {
    mpz_ui_sub(r, b, a);
    mpz_neg(r, r as *const __mpz_struct);
}
#[no_mangle]
pub unsafe extern "C" fn mpz_ui_sub(
    mut r: *mut __mpz_struct,
    mut a: u64,
    mut b: *const __mpz_struct,
) {
    mpz_neg(r, b);
    mpz_add_ui(r, r as *const __mpz_struct, a);
}
unsafe extern "C" fn mpz_abs_add(
    mut r: *mut __mpz_struct,
    mut a: *const __mpz_struct,
    mut b: *const __mpz_struct,
) -> mp_size_t {
    let mut an: mp_size_t = (if (*a)._mp_size >= 0 as i32 {
        (*a)._mp_size
    } else {
        -(*a)._mp_size
    }) as mp_size_t;
    let mut bn: mp_size_t = (if (*b)._mp_size >= 0 as i32 {
        (*b)._mp_size
    } else {
        -(*b)._mp_size
    }) as mp_size_t;
    let mut rp: mp_ptr = 0 as *mut mp_limb_t;
    let mut cy: mp_limb_t = 0;
    if an < bn {
        let mut __mpz_srcptr_swap__tmp: mpz_srcptr = a;
        a = b;
        b = __mpz_srcptr_swap__tmp;
        let mut __mp_size_t_swap__tmp: mp_size_t = an;
        an = bn;
        bn = __mp_size_t_swap__tmp;
    }
    rp = if an + 1 as i32 as i64 > (*r)._mp_alloc as i64 {
        mpz_realloc(r, an + 1 as i32 as i64)
    } else {
        (*r)._mp_d
    };
    cy = mpn_add(rp, (*a)._mp_d as mp_srcptr, an, (*b)._mp_d as mp_srcptr, bn);
    *rp.offset(an as isize) = cy;
    return (an as u64).wrapping_add(cy) as mp_size_t;
}
unsafe extern "C" fn mpz_abs_sub(
    mut r: *mut __mpz_struct,
    mut a: *const __mpz_struct,
    mut b: *const __mpz_struct,
) -> mp_size_t {
    let mut an: mp_size_t = (if (*a)._mp_size >= 0 as i32 {
        (*a)._mp_size
    } else {
        -(*a)._mp_size
    }) as mp_size_t;
    let mut bn: mp_size_t = (if (*b)._mp_size >= 0 as i32 {
        (*b)._mp_size
    } else {
        -(*b)._mp_size
    }) as mp_size_t;
    let mut cmp: i32 = 0;
    let mut rp: mp_ptr = 0 as *mut mp_limb_t;
    cmp = mpn_cmp4((*a)._mp_d as mp_srcptr, an, (*b)._mp_d as mp_srcptr, bn);
    if cmp > 0 as i32 {
        rp = if an > (*r)._mp_alloc as i64 { mpz_realloc(r, an) } else { (*r)._mp_d };
        let mut __cy: mp_limb_t = mpn_sub(
            rp,
            (*a)._mp_d as mp_srcptr,
            an,
            (*b)._mp_d as mp_srcptr,
            bn,
        );
        if __cy == 0 as i32 as u64 {} else {
            __assert_fail(
                b"__cy == 0\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                1994 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 82],
                    &[i8; 82],
                >(
                    b"mp_size_t mpz_abs_sub(__mpz_struct *, const __mpz_struct *, const __mpz_struct *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_5908: {
            if __cy == 0 as i32 as u64 {} else {
                __assert_fail(
                    b"__cy == 0\0" as *const u8 as *const i8,
                    b"../mini-gmp.c\0" as *const u8 as *const i8,
                    1994 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 82],
                        &[i8; 82],
                    >(
                        b"mp_size_t mpz_abs_sub(__mpz_struct *, const __mpz_struct *, const __mpz_struct *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        return mpn_normalized_size(rp as mp_srcptr, an);
    } else if cmp < 0 as i32 {
        rp = if bn > (*r)._mp_alloc as i64 { mpz_realloc(r, bn) } else { (*r)._mp_d };
        let mut __cy_0: mp_limb_t = mpn_sub(
            rp,
            (*b)._mp_d as mp_srcptr,
            bn,
            (*a)._mp_d as mp_srcptr,
            an,
        );
        if __cy_0 == 0 as i32 as u64 {} else {
            __assert_fail(
                b"__cy == 0\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                2000 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 82],
                    &[i8; 82],
                >(
                    b"mp_size_t mpz_abs_sub(__mpz_struct *, const __mpz_struct *, const __mpz_struct *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_5780: {
            if __cy_0 == 0 as i32 as u64 {} else {
                __assert_fail(
                    b"__cy == 0\0" as *const u8 as *const i8,
                    b"../mini-gmp.c\0" as *const u8 as *const i8,
                    2000 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 82],
                        &[i8; 82],
                    >(
                        b"mp_size_t mpz_abs_sub(__mpz_struct *, const __mpz_struct *, const __mpz_struct *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        return -mpn_normalized_size(rp as mp_srcptr, bn);
    } else {
        return 0 as i32 as mp_size_t
    };
}
#[no_mangle]
pub unsafe extern "C" fn mpz_add(
    mut r: *mut __mpz_struct,
    mut a: *const __mpz_struct,
    mut b: *const __mpz_struct,
) {
    let mut rn: mp_size_t = 0;
    if (*a)._mp_size ^ (*b)._mp_size >= 0 as i32 {
        rn = mpz_abs_add(r, a, b);
    } else {
        rn = mpz_abs_sub(r, a, b);
    }
    (*r)._mp_size = (if (*a)._mp_size >= 0 as i32 { rn } else { -rn }) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn mpz_sub(
    mut r: *mut __mpz_struct,
    mut a: *const __mpz_struct,
    mut b: *const __mpz_struct,
) {
    let mut rn: mp_size_t = 0;
    if (*a)._mp_size ^ (*b)._mp_size >= 0 as i32 {
        rn = mpz_abs_sub(r, a, b);
    } else {
        rn = mpz_abs_add(r, a, b);
    }
    (*r)._mp_size = (if (*a)._mp_size >= 0 as i32 { rn } else { -rn }) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn mpz_mul_si(
    mut r: *mut __mpz_struct,
    mut u: *const __mpz_struct,
    mut v: i64,
) {
    if v < 0 as i32 as i64 {
        mpz_mul_ui(
            r,
            u,
            ((v + 1 as i32 as i64) as u64).wrapping_sub(1 as i32 as u64).wrapping_neg(),
        );
        mpz_neg(r, r as *const __mpz_struct);
    } else {
        mpz_mul_ui(r, u, v as u64);
    };
}
#[no_mangle]
pub unsafe extern "C" fn mpz_mul_ui(
    mut r: *mut __mpz_struct,
    mut u: *const __mpz_struct,
    mut v: u64,
) {
    let mut vv: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    mpz_init_set_ui(vv.as_mut_ptr(), v);
    mpz_mul(r, u, vv.as_mut_ptr() as *const __mpz_struct);
    mpz_clear(vv.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn mpz_mul(
    mut r: *mut __mpz_struct,
    mut u: *const __mpz_struct,
    mut v: *const __mpz_struct,
) {
    let mut sign: i32 = 0;
    let mut un: mp_size_t = 0;
    let mut vn: mp_size_t = 0;
    let mut rn: mp_size_t = 0;
    let mut t: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut tp: mp_ptr = 0 as *mut mp_limb_t;
    un = (*u)._mp_size as mp_size_t;
    vn = (*v)._mp_size as mp_size_t;
    if un == 0 as i32 as i64 || vn == 0 as i32 as i64 {
        (*r)._mp_size = 0 as i32;
        return;
    }
    sign = (un ^ vn < 0 as i32 as i64) as i32;
    un = if un >= 0 as i32 as i64 { un } else { -un };
    vn = if vn >= 0 as i32 as i64 { vn } else { -vn };
    mpz_init2(
        t.as_mut_ptr(),
        ((un + vn) as u64)
            .wrapping_mul(
                (::core::mem::size_of::<mp_limb_t>() as u64)
                    .wrapping_mul(8 as i32 as u64),
            ),
    );
    tp = (*t.as_mut_ptr())._mp_d;
    if un >= vn {
        mpn_mul(tp, (*u)._mp_d as mp_srcptr, un, (*v)._mp_d as mp_srcptr, vn);
    } else {
        mpn_mul(tp, (*v)._mp_d as mp_srcptr, vn, (*u)._mp_d as mp_srcptr, un);
    }
    rn = un + vn;
    rn -= (*tp.offset((rn - 1 as i32 as i64) as isize) == 0 as i32 as u64) as i32 as i64;
    (*t.as_mut_ptr())._mp_size = (if sign != 0 { -rn } else { rn }) as i32;
    mpz_swap(r, t.as_mut_ptr());
    mpz_clear(t.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn mpz_mul_2exp(
    mut r: *mut __mpz_struct,
    mut u: *const __mpz_struct,
    mut bits: mp_bitcnt_t,
) {
    let mut un: mp_size_t = 0;
    let mut rn: mp_size_t = 0;
    let mut limbs: mp_size_t = 0;
    let mut shift: u32 = 0;
    let mut rp: mp_ptr = 0 as *mut mp_limb_t;
    un = (if (*u)._mp_size >= 0 as i32 { (*u)._mp_size } else { -(*u)._mp_size })
        as mp_size_t;
    if un == 0 as i32 as i64 {
        (*r)._mp_size = 0 as i32;
        return;
    }
    limbs = bits
        .wrapping_div(
            (::core::mem::size_of::<mp_limb_t>() as u64).wrapping_mul(8 as i32 as u64),
        ) as mp_size_t;
    shift = bits
        .wrapping_rem(
            (::core::mem::size_of::<mp_limb_t>() as u64).wrapping_mul(8 as i32 as u64),
        ) as u32;
    rn = un + limbs + (shift > 0 as i32 as u32) as i32 as i64;
    rp = if rn > (*r)._mp_alloc as i64 { mpz_realloc(r, rn) } else { (*r)._mp_d };
    if shift > 0 as i32 as u32 {
        let mut cy: mp_limb_t = mpn_lshift(
            rp.offset(limbs as isize),
            (*u)._mp_d as mp_srcptr,
            un,
            shift,
        );
        *rp.offset((rn - 1 as i32 as i64) as isize) = cy;
        rn -= (cy == 0 as i32 as u64) as i32 as i64;
    } else {
        mpn_copyd(rp.offset(limbs as isize), (*u)._mp_d as mp_srcptr, un);
    }
    mpn_zero(rp, limbs);
    (*r)._mp_size = (if (*u)._mp_size < 0 as i32 { -rn } else { rn }) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn mpz_addmul_ui(
    mut r: *mut __mpz_struct,
    mut u: *const __mpz_struct,
    mut v: u64,
) {
    let mut t: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    mpz_init_set_ui(t.as_mut_ptr(), v);
    mpz_mul(t.as_mut_ptr(), u, t.as_mut_ptr() as *const __mpz_struct);
    mpz_add(r, r as *const __mpz_struct, t.as_mut_ptr() as *const __mpz_struct);
    mpz_clear(t.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn mpz_submul_ui(
    mut r: *mut __mpz_struct,
    mut u: *const __mpz_struct,
    mut v: u64,
) {
    let mut t: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    mpz_init_set_ui(t.as_mut_ptr(), v);
    mpz_mul(t.as_mut_ptr(), u, t.as_mut_ptr() as *const __mpz_struct);
    mpz_sub(r, r as *const __mpz_struct, t.as_mut_ptr() as *const __mpz_struct);
    mpz_clear(t.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn mpz_addmul(
    mut r: *mut __mpz_struct,
    mut u: *const __mpz_struct,
    mut v: *const __mpz_struct,
) {
    let mut t: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    mpz_init(t.as_mut_ptr());
    mpz_mul(t.as_mut_ptr(), u, v);
    mpz_add(r, r as *const __mpz_struct, t.as_mut_ptr() as *const __mpz_struct);
    mpz_clear(t.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn mpz_submul(
    mut r: *mut __mpz_struct,
    mut u: *const __mpz_struct,
    mut v: *const __mpz_struct,
) {
    let mut t: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    mpz_init(t.as_mut_ptr());
    mpz_mul(t.as_mut_ptr(), u, v);
    mpz_sub(r, r as *const __mpz_struct, t.as_mut_ptr() as *const __mpz_struct);
    mpz_clear(t.as_mut_ptr());
}
unsafe extern "C" fn mpz_div_qr(
    mut q: *mut __mpz_struct,
    mut r: *mut __mpz_struct,
    mut n: *const __mpz_struct,
    mut d: *const __mpz_struct,
    mut mode: mpz_div_round_mode,
) -> i32 {
    let mut ns: mp_size_t = 0;
    let mut ds: mp_size_t = 0;
    let mut nn: mp_size_t = 0;
    let mut dn: mp_size_t = 0;
    let mut qs: mp_size_t = 0;
    ns = (*n)._mp_size as mp_size_t;
    ds = (*d)._mp_size as mp_size_t;
    if ds == 0 as i32 as i64 {
        gmp_die(b"mpz_div_qr: Divide by zero.\0" as *const u8 as *const i8);
    }
    if ns == 0 as i32 as i64 {
        if !q.is_null() {
            (*q)._mp_size = 0 as i32;
        }
        if !r.is_null() {
            (*r)._mp_size = 0 as i32;
        }
        return 0 as i32;
    }
    nn = if ns >= 0 as i32 as i64 { ns } else { -ns };
    dn = if ds >= 0 as i32 as i64 { ds } else { -ds };
    qs = ds ^ ns;
    if nn < dn {
        if mode as u32 == GMP_DIV_CEIL as i32 as u32 && qs >= 0 as i32 as i64 {
            if !r.is_null() {
                mpz_sub(r, n, d);
            }
            if !q.is_null() {
                mpz_set_ui(q, 1 as i32 as u64);
            }
        } else if mode as u32 == GMP_DIV_FLOOR as i32 as u32 && qs < 0 as i32 as i64 {
            if !r.is_null() {
                mpz_add(r, n, d);
            }
            if !q.is_null() {
                mpz_set_si(q, -(1 as i32) as i64);
            }
        } else {
            if !r.is_null() {
                mpz_set(r, n);
            }
            if !q.is_null() {
                (*q)._mp_size = 0 as i32;
            }
        }
        return 1 as i32;
    } else {
        let mut np: mp_ptr = 0 as *mut mp_limb_t;
        let mut qp: mp_ptr = 0 as *mut mp_limb_t;
        let mut qn: mp_size_t = 0;
        let mut rn: mp_size_t = 0;
        let mut tq: mpz_t = [__mpz_struct {
            _mp_alloc: 0,
            _mp_size: 0,
            _mp_d: 0 as *mut mp_limb_t,
        }; 1];
        let mut tr: mpz_t = [__mpz_struct {
            _mp_alloc: 0,
            _mp_size: 0,
            _mp_d: 0 as *mut mp_limb_t,
        }; 1];
        mpz_init_set(tr.as_mut_ptr(), n);
        np = (*tr.as_mut_ptr())._mp_d;
        qn = nn - dn + 1 as i32 as i64;
        if !q.is_null() {
            mpz_init2(
                tq.as_mut_ptr(),
                (qn as u64)
                    .wrapping_mul(
                        (::core::mem::size_of::<mp_limb_t>() as u64)
                            .wrapping_mul(8 as i32 as u64),
                    ),
            );
            qp = (*tq.as_mut_ptr())._mp_d;
        } else {
            qp = 0 as mp_ptr;
        }
        mpn_div_qr(qp, np, nn, (*d)._mp_d as mp_srcptr, dn);
        if !qp.is_null() {
            qn
                -= (*qp.offset((qn - 1 as i32 as i64) as isize) == 0 as i32 as u64)
                    as i32 as i64;
            (*tq.as_mut_ptr())._mp_size = (if qs < 0 as i32 as i64 { -qn } else { qn })
                as i32;
        }
        rn = mpn_normalized_size(np as mp_srcptr, dn);
        (*tr.as_mut_ptr())._mp_size = (if ns < 0 as i32 as i64 { -rn } else { rn })
            as i32;
        if mode as u32 == GMP_DIV_FLOOR as i32 as u32 && qs < 0 as i32 as i64
            && rn != 0 as i32 as i64
        {
            if !q.is_null() {
                mpz_sub_ui(
                    tq.as_mut_ptr(),
                    tq.as_mut_ptr() as *const __mpz_struct,
                    1 as i32 as u64,
                );
            }
            if !r.is_null() {
                mpz_add(tr.as_mut_ptr(), tr.as_mut_ptr() as *const __mpz_struct, d);
            }
        } else if mode as u32 == GMP_DIV_CEIL as i32 as u32 && qs >= 0 as i32 as i64
            && rn != 0 as i32 as i64
        {
            if !q.is_null() {
                mpz_add_ui(
                    tq.as_mut_ptr(),
                    tq.as_mut_ptr() as *const __mpz_struct,
                    1 as i32 as u64,
                );
            }
            if !r.is_null() {
                mpz_sub(tr.as_mut_ptr(), tr.as_mut_ptr() as *const __mpz_struct, d);
            }
        }
        if !q.is_null() {
            mpz_swap(tq.as_mut_ptr(), q);
            mpz_clear(tq.as_mut_ptr());
        }
        if !r.is_null() {
            mpz_swap(tr.as_mut_ptr(), r);
        }
        mpz_clear(tr.as_mut_ptr());
        return (rn != 0 as i32 as i64) as i32;
    };
}
#[no_mangle]
pub unsafe extern "C" fn mpz_cdiv_qr(
    mut q: *mut __mpz_struct,
    mut r: *mut __mpz_struct,
    mut n: *const __mpz_struct,
    mut d: *const __mpz_struct,
) {
    mpz_div_qr(q, r, n, d, GMP_DIV_CEIL);
}
#[no_mangle]
pub unsafe extern "C" fn mpz_fdiv_qr(
    mut q: *mut __mpz_struct,
    mut r: *mut __mpz_struct,
    mut n: *const __mpz_struct,
    mut d: *const __mpz_struct,
) {
    mpz_div_qr(q, r, n, d, GMP_DIV_FLOOR);
}
#[no_mangle]
pub unsafe extern "C" fn mpz_tdiv_qr(
    mut q: *mut __mpz_struct,
    mut r: *mut __mpz_struct,
    mut n: *const __mpz_struct,
    mut d: *const __mpz_struct,
) {
    mpz_div_qr(q, r, n, d, GMP_DIV_TRUNC);
}
#[no_mangle]
pub unsafe extern "C" fn mpz_cdiv_q(
    mut q: *mut __mpz_struct,
    mut n: *const __mpz_struct,
    mut d: *const __mpz_struct,
) {
    mpz_div_qr(q, 0 as *mut __mpz_struct, n, d, GMP_DIV_CEIL);
}
#[no_mangle]
pub unsafe extern "C" fn mpz_fdiv_q(
    mut q: *mut __mpz_struct,
    mut n: *const __mpz_struct,
    mut d: *const __mpz_struct,
) {
    mpz_div_qr(q, 0 as *mut __mpz_struct, n, d, GMP_DIV_FLOOR);
}
#[no_mangle]
pub unsafe extern "C" fn mpz_tdiv_q(
    mut q: *mut __mpz_struct,
    mut n: *const __mpz_struct,
    mut d: *const __mpz_struct,
) {
    mpz_div_qr(q, 0 as *mut __mpz_struct, n, d, GMP_DIV_TRUNC);
}
#[no_mangle]
pub unsafe extern "C" fn mpz_cdiv_r(
    mut r: *mut __mpz_struct,
    mut n: *const __mpz_struct,
    mut d: *const __mpz_struct,
) {
    mpz_div_qr(0 as *mut __mpz_struct, r, n, d, GMP_DIV_CEIL);
}
#[no_mangle]
pub unsafe extern "C" fn mpz_fdiv_r(
    mut r: *mut __mpz_struct,
    mut n: *const __mpz_struct,
    mut d: *const __mpz_struct,
) {
    mpz_div_qr(0 as *mut __mpz_struct, r, n, d, GMP_DIV_FLOOR);
}
#[no_mangle]
pub unsafe extern "C" fn mpz_tdiv_r(
    mut r: *mut __mpz_struct,
    mut n: *const __mpz_struct,
    mut d: *const __mpz_struct,
) {
    mpz_div_qr(0 as *mut __mpz_struct, r, n, d, GMP_DIV_TRUNC);
}
#[no_mangle]
pub unsafe extern "C" fn mpz_mod(
    mut r: *mut __mpz_struct,
    mut n: *const __mpz_struct,
    mut d: *const __mpz_struct,
) {
    mpz_div_qr(
        0 as *mut __mpz_struct,
        r,
        n,
        d,
        (if (*d)._mp_size >= 0 as i32 {
            GMP_DIV_FLOOR as i32
        } else {
            GMP_DIV_CEIL as i32
        }) as mpz_div_round_mode,
    );
}
unsafe extern "C" fn mpz_div_q_2exp(
    mut q: *mut __mpz_struct,
    mut u: *const __mpz_struct,
    mut bit_index: mp_bitcnt_t,
    mut mode: mpz_div_round_mode,
) {
    let mut un: mp_size_t = 0;
    let mut qn: mp_size_t = 0;
    let mut limb_cnt: mp_size_t = 0;
    let mut qp: mp_ptr = 0 as *mut mp_limb_t;
    let mut adjust: i32 = 0;
    un = (*u)._mp_size as mp_size_t;
    if un == 0 as i32 as i64 {
        (*q)._mp_size = 0 as i32;
        return;
    }
    limb_cnt = bit_index
        .wrapping_div(
            (::core::mem::size_of::<mp_limb_t>() as u64).wrapping_mul(8 as i32 as u64),
        ) as mp_size_t;
    qn = (if un >= 0 as i32 as i64 { un } else { -un }) - limb_cnt;
    bit_index = (bit_index as u64)
        .wrapping_rem(
            (::core::mem::size_of::<mp_limb_t>() as u64).wrapping_mul(8 as i32 as u64),
        ) as mp_bitcnt_t as mp_bitcnt_t;
    if mode as u32
        == (if un > 0 as i32 as i64 {
            GMP_DIV_CEIL as i32
        } else {
            GMP_DIV_FLOOR as i32
        }) as u32
    {
        adjust = (qn <= 0 as i32 as i64
            || mpn_zero_p((*u)._mp_d as mp_srcptr, limb_cnt) == 0
            || *((*u)._mp_d).offset(limb_cnt as isize)
                & ((1 as i32 as mp_limb_t) << bit_index).wrapping_sub(1 as i32 as u64)
                != 0) as i32;
    } else {
        adjust = 0 as i32;
    }
    if qn <= 0 as i32 as i64 {
        qn = 0 as i32 as mp_size_t;
    } else {
        qp = if qn > (*q)._mp_alloc as i64 { mpz_realloc(q, qn) } else { (*q)._mp_d };
        if bit_index != 0 as i32 as u64 {
            mpn_rshift(
                qp,
                ((*u)._mp_d).offset(limb_cnt as isize) as mp_srcptr,
                qn,
                bit_index as u32,
            );
            qn
                -= (*qp.offset((qn - 1 as i32 as i64) as isize) == 0 as i32 as u64)
                    as i32 as i64;
        } else {
            mpn_copyi(qp, ((*u)._mp_d).offset(limb_cnt as isize) as mp_srcptr, qn);
        }
    }
    (*q)._mp_size = qn as i32;
    if adjust != 0 {
        mpz_add_ui(q, q as *const __mpz_struct, 1 as i32 as u64);
    }
    if un < 0 as i32 as i64 {
        mpz_neg(q, q as *const __mpz_struct);
    }
}
unsafe extern "C" fn mpz_div_r_2exp(
    mut r: *mut __mpz_struct,
    mut u: *const __mpz_struct,
    mut bit_index: mp_bitcnt_t,
    mut mode: mpz_div_round_mode,
) {
    let mut us: mp_size_t = 0;
    let mut un: mp_size_t = 0;
    let mut rn: mp_size_t = 0;
    let mut rp: mp_ptr = 0 as *mut mp_limb_t;
    let mut mask: mp_limb_t = 0;
    us = (*u)._mp_size as mp_size_t;
    if us == 0 as i32 as i64 || bit_index == 0 as i32 as u64 {
        (*r)._mp_size = 0 as i32;
        return;
    }
    rn = bit_index
        .wrapping_add(
            (::core::mem::size_of::<mp_limb_t>() as u64).wrapping_mul(8 as i32 as u64),
        )
        .wrapping_sub(1 as i32 as u64)
        .wrapping_div(
            (::core::mem::size_of::<mp_limb_t>() as u64).wrapping_mul(8 as i32 as u64),
        ) as mp_size_t;
    if rn > 0 as i32 as i64 {} else {
        __assert_fail(
            b"rn > 0\0" as *const u8 as *const i8,
            b"../mini-gmp.c\0" as *const u8 as *const i8,
            2415 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 96],
                &[i8; 96],
            >(
                b"void mpz_div_r_2exp(__mpz_struct *, const __mpz_struct *, mp_bitcnt_t, enum mpz_div_round_mode)\0",
            ))
                .as_ptr(),
        );
    }
    'c_19943: {
        if rn > 0 as i32 as i64 {} else {
            __assert_fail(
                b"rn > 0\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                2415 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 96],
                    &[i8; 96],
                >(
                    b"void mpz_div_r_2exp(__mpz_struct *, const __mpz_struct *, mp_bitcnt_t, enum mpz_div_round_mode)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    rp = if rn > (*r)._mp_alloc as i64 { mpz_realloc(r, rn) } else { (*r)._mp_d };
    un = if us >= 0 as i32 as i64 { us } else { -us };
    mask = !(0 as i32 as mp_limb_t)
        >> (rn as u64)
            .wrapping_mul(
                (::core::mem::size_of::<mp_limb_t>() as u64)
                    .wrapping_mul(8 as i32 as u64),
            )
            .wrapping_sub(bit_index);
    if rn > un {
        if mode as u32
            == (if us > 0 as i32 as i64 {
                GMP_DIV_CEIL as i32
            } else {
                GMP_DIV_FLOOR as i32
            }) as u32
        {
            let mut i: mp_size_t = 0;
            let mut __cy: mp_limb_t = (mpn_neg(rp, (*u)._mp_d as mp_srcptr, un) == 0)
                as i32 as mp_limb_t;
            if __cy == 0 as i32 as u64 {} else {
                __assert_fail(
                    b"__cy == 0\0" as *const u8 as *const i8,
                    b"../mini-gmp.c\0" as *const u8 as *const i8,
                    2431 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 96],
                        &[i8; 96],
                    >(
                        b"void mpz_div_r_2exp(__mpz_struct *, const __mpz_struct *, mp_bitcnt_t, enum mpz_div_round_mode)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_19806: {
                if __cy == 0 as i32 as u64 {} else {
                    __assert_fail(
                        b"__cy == 0\0" as *const u8 as *const i8,
                        b"../mini-gmp.c\0" as *const u8 as *const i8,
                        2431 as i32 as u32,
                        (*::core::mem::transmute::<
                            &[u8; 96],
                            &[i8; 96],
                        >(
                            b"void mpz_div_r_2exp(__mpz_struct *, const __mpz_struct *, mp_bitcnt_t, enum mpz_div_round_mode)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            i = un;
            while i < rn - 1 as i32 as i64 {
                *rp.offset(i as isize) = !(0 as i32 as mp_limb_t);
                i += 1;
                i;
            }
            *rp.offset((rn - 1 as i32 as i64) as isize) = mask;
            us = -us;
        } else {
            if r != u as *mut __mpz_struct {
                mpn_copyi(rp, (*u)._mp_d as mp_srcptr, un);
            }
            rn = un;
        }
    } else {
        if r != u as *mut __mpz_struct {
            mpn_copyi(rp, (*u)._mp_d as mp_srcptr, rn - 1 as i32 as i64);
        }
        *rp.offset((rn - 1 as i32 as i64) as isize) = *((*u)._mp_d)
            .offset((rn - 1 as i32 as i64) as isize) & mask;
        if mode as u32
            == (if us > 0 as i32 as i64 {
                GMP_DIV_CEIL as i32
            } else {
                GMP_DIV_FLOOR as i32
            }) as u32
        {
            mpn_neg(rp, rp as mp_srcptr, rn);
            let ref mut fresh27 = *rp.offset((rn - 1 as i32 as i64) as isize);
            *fresh27 &= mask;
            us = -us;
        }
    }
    rn = mpn_normalized_size(rp as mp_srcptr, rn);
    (*r)._mp_size = (if us < 0 as i32 as i64 { -rn } else { rn }) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn mpz_cdiv_q_2exp(
    mut r: *mut __mpz_struct,
    mut u: *const __mpz_struct,
    mut cnt: mp_bitcnt_t,
) {
    mpz_div_q_2exp(r, u, cnt, GMP_DIV_CEIL);
}
#[no_mangle]
pub unsafe extern "C" fn mpz_fdiv_q_2exp(
    mut r: *mut __mpz_struct,
    mut u: *const __mpz_struct,
    mut cnt: mp_bitcnt_t,
) {
    mpz_div_q_2exp(r, u, cnt, GMP_DIV_FLOOR);
}
#[no_mangle]
pub unsafe extern "C" fn mpz_tdiv_q_2exp(
    mut r: *mut __mpz_struct,
    mut u: *const __mpz_struct,
    mut cnt: mp_bitcnt_t,
) {
    mpz_div_q_2exp(r, u, cnt, GMP_DIV_TRUNC);
}
#[no_mangle]
pub unsafe extern "C" fn mpz_cdiv_r_2exp(
    mut r: *mut __mpz_struct,
    mut u: *const __mpz_struct,
    mut cnt: mp_bitcnt_t,
) {
    mpz_div_r_2exp(r, u, cnt, GMP_DIV_CEIL);
}
#[no_mangle]
pub unsafe extern "C" fn mpz_fdiv_r_2exp(
    mut r: *mut __mpz_struct,
    mut u: *const __mpz_struct,
    mut cnt: mp_bitcnt_t,
) {
    mpz_div_r_2exp(r, u, cnt, GMP_DIV_FLOOR);
}
#[no_mangle]
pub unsafe extern "C" fn mpz_tdiv_r_2exp(
    mut r: *mut __mpz_struct,
    mut u: *const __mpz_struct,
    mut cnt: mp_bitcnt_t,
) {
    mpz_div_r_2exp(r, u, cnt, GMP_DIV_TRUNC);
}
#[no_mangle]
pub unsafe extern "C" fn mpz_divexact(
    mut q: *mut __mpz_struct,
    mut n: *const __mpz_struct,
    mut d: *const __mpz_struct,
) {
    let mut __cy: mp_limb_t = mpz_div_qr(q, 0 as *mut __mpz_struct, n, d, GMP_DIV_TRUNC)
        as mp_limb_t;
    if __cy == 0 as i32 as u64 {} else {
        __assert_fail(
            b"__cy == 0\0" as *const u8 as *const i8,
            b"../mini-gmp.c\0" as *const u8 as *const i8,
            2509 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 78],
                &[i8; 78],
            >(
                b"void mpz_divexact(__mpz_struct *, const __mpz_struct *, const __mpz_struct *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_20101: {
        if __cy == 0 as i32 as u64 {} else {
            __assert_fail(
                b"__cy == 0\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                2509 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 78],
                    &[i8; 78],
                >(
                    b"void mpz_divexact(__mpz_struct *, const __mpz_struct *, const __mpz_struct *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn mpz_divisible_p(
    mut n: *const __mpz_struct,
    mut d: *const __mpz_struct,
) -> i32 {
    return (mpz_div_qr(
        0 as *mut __mpz_struct,
        0 as *mut __mpz_struct,
        n,
        d,
        GMP_DIV_TRUNC,
    ) == 0 as i32) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn mpz_congruent_p(
    mut a: *const __mpz_struct,
    mut b: *const __mpz_struct,
    mut m: *const __mpz_struct,
) -> i32 {
    let mut t: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut res: i32 = 0;
    if mpz_sgn(m) == 0 as i32 {
        return (mpz_cmp(a, b) == 0 as i32) as i32;
    }
    mpz_init(t.as_mut_ptr());
    mpz_sub(t.as_mut_ptr(), a, b);
    res = mpz_divisible_p(t.as_mut_ptr() as *const __mpz_struct, m);
    mpz_clear(t.as_mut_ptr());
    return res;
}
unsafe extern "C" fn mpz_div_qr_ui(
    mut q: *mut __mpz_struct,
    mut r: *mut __mpz_struct,
    mut n: *const __mpz_struct,
    mut d: u64,
    mut mode: mpz_div_round_mode,
) -> u64 {
    let mut ret: u64 = 0;
    let mut rr: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut dd: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    mpz_init(rr.as_mut_ptr());
    mpz_init_set_ui(dd.as_mut_ptr(), d);
    mpz_div_qr(q, rr.as_mut_ptr(), n, dd.as_mut_ptr() as *const __mpz_struct, mode);
    mpz_clear(dd.as_mut_ptr());
    ret = mpz_get_ui(rr.as_mut_ptr() as *const __mpz_struct);
    if !r.is_null() {
        mpz_swap(r, rr.as_mut_ptr());
    }
    mpz_clear(rr.as_mut_ptr());
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn mpz_cdiv_qr_ui(
    mut q: *mut __mpz_struct,
    mut r: *mut __mpz_struct,
    mut n: *const __mpz_struct,
    mut d: u64,
) -> u64 {
    return mpz_div_qr_ui(q, r, n, d, GMP_DIV_CEIL);
}
#[no_mangle]
pub unsafe extern "C" fn mpz_fdiv_qr_ui(
    mut q: *mut __mpz_struct,
    mut r: *mut __mpz_struct,
    mut n: *const __mpz_struct,
    mut d: u64,
) -> u64 {
    return mpz_div_qr_ui(q, r, n, d, GMP_DIV_FLOOR);
}
#[no_mangle]
pub unsafe extern "C" fn mpz_tdiv_qr_ui(
    mut q: *mut __mpz_struct,
    mut r: *mut __mpz_struct,
    mut n: *const __mpz_struct,
    mut d: u64,
) -> u64 {
    return mpz_div_qr_ui(q, r, n, d, GMP_DIV_TRUNC);
}
#[no_mangle]
pub unsafe extern "C" fn mpz_cdiv_q_ui(
    mut q: *mut __mpz_struct,
    mut n: *const __mpz_struct,
    mut d: u64,
) -> u64 {
    return mpz_div_qr_ui(q, 0 as *mut __mpz_struct, n, d, GMP_DIV_CEIL);
}
#[no_mangle]
pub unsafe extern "C" fn mpz_fdiv_q_ui(
    mut q: *mut __mpz_struct,
    mut n: *const __mpz_struct,
    mut d: u64,
) -> u64 {
    return mpz_div_qr_ui(q, 0 as *mut __mpz_struct, n, d, GMP_DIV_FLOOR);
}
#[no_mangle]
pub unsafe extern "C" fn mpz_tdiv_q_ui(
    mut q: *mut __mpz_struct,
    mut n: *const __mpz_struct,
    mut d: u64,
) -> u64 {
    return mpz_div_qr_ui(q, 0 as *mut __mpz_struct, n, d, GMP_DIV_TRUNC);
}
#[no_mangle]
pub unsafe extern "C" fn mpz_cdiv_r_ui(
    mut r: *mut __mpz_struct,
    mut n: *const __mpz_struct,
    mut d: u64,
) -> u64 {
    return mpz_div_qr_ui(0 as *mut __mpz_struct, r, n, d, GMP_DIV_CEIL);
}
#[no_mangle]
pub unsafe extern "C" fn mpz_fdiv_r_ui(
    mut r: *mut __mpz_struct,
    mut n: *const __mpz_struct,
    mut d: u64,
) -> u64 {
    return mpz_div_qr_ui(0 as *mut __mpz_struct, r, n, d, GMP_DIV_FLOOR);
}
#[no_mangle]
pub unsafe extern "C" fn mpz_tdiv_r_ui(
    mut r: *mut __mpz_struct,
    mut n: *const __mpz_struct,
    mut d: u64,
) -> u64 {
    return mpz_div_qr_ui(0 as *mut __mpz_struct, r, n, d, GMP_DIV_TRUNC);
}
#[no_mangle]
pub unsafe extern "C" fn mpz_cdiv_ui(mut n: *const __mpz_struct, mut d: u64) -> u64 {
    return mpz_div_qr_ui(
        0 as *mut __mpz_struct,
        0 as *mut __mpz_struct,
        n,
        d,
        GMP_DIV_CEIL,
    );
}
#[no_mangle]
pub unsafe extern "C" fn mpz_fdiv_ui(mut n: *const __mpz_struct, mut d: u64) -> u64 {
    return mpz_div_qr_ui(
        0 as *mut __mpz_struct,
        0 as *mut __mpz_struct,
        n,
        d,
        GMP_DIV_FLOOR,
    );
}
#[no_mangle]
pub unsafe extern "C" fn mpz_tdiv_ui(mut n: *const __mpz_struct, mut d: u64) -> u64 {
    return mpz_div_qr_ui(
        0 as *mut __mpz_struct,
        0 as *mut __mpz_struct,
        n,
        d,
        GMP_DIV_TRUNC,
    );
}
#[no_mangle]
pub unsafe extern "C" fn mpz_mod_ui(
    mut r: *mut __mpz_struct,
    mut n: *const __mpz_struct,
    mut d: u64,
) -> u64 {
    return mpz_div_qr_ui(0 as *mut __mpz_struct, r, n, d, GMP_DIV_FLOOR);
}
#[no_mangle]
pub unsafe extern "C" fn mpz_divexact_ui(
    mut q: *mut __mpz_struct,
    mut n: *const __mpz_struct,
    mut d: u64,
) {
    let mut __cy: mp_limb_t = mpz_div_qr_ui(
        q,
        0 as *mut __mpz_struct,
        n,
        d,
        GMP_DIV_TRUNC,
    );
    if __cy == 0 as i32 as u64 {} else {
        __assert_fail(
            b"__cy == 0\0" as *const u8 as *const i8,
            b"../mini-gmp.c\0" as *const u8 as *const i8,
            2635 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 74],
                &[i8; 74],
            >(
                b"void mpz_divexact_ui(__mpz_struct *, const __mpz_struct *, unsigned long)\0",
            ))
                .as_ptr(),
        );
    }
    'c_20512: {
        if __cy == 0 as i32 as u64 {} else {
            __assert_fail(
                b"__cy == 0\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                2635 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 74],
                    &[i8; 74],
                >(
                    b"void mpz_divexact_ui(__mpz_struct *, const __mpz_struct *, unsigned long)\0",
                ))
                    .as_ptr(),
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn mpz_divisible_ui_p(
    mut n: *const __mpz_struct,
    mut d: u64,
) -> i32 {
    return (mpz_div_qr_ui(
        0 as *mut __mpz_struct,
        0 as *mut __mpz_struct,
        n,
        d,
        GMP_DIV_TRUNC,
    ) == 0 as i32 as u64) as i32;
}
unsafe extern "C" fn mpn_gcd_11(mut u: mp_limb_t, mut v: mp_limb_t) -> mp_limb_t {
    let mut shift: u32 = 0;
    if u | v > 0 as i32 as u64 {} else {
        __assert_fail(
            b"(u | v) > 0\0" as *const u8 as *const i8,
            b"../mini-gmp.c\0" as *const u8 as *const i8,
            2651 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 43],
                &[i8; 43],
            >(b"mp_limb_t mpn_gcd_11(mp_limb_t, mp_limb_t)\0"))
                .as_ptr(),
        );
    }
    'c_21012: {
        if u | v > 0 as i32 as u64 {} else {
            __assert_fail(
                b"(u | v) > 0\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                2651 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 43],
                    &[i8; 43],
                >(b"mp_limb_t mpn_gcd_11(mp_limb_t, mp_limb_t)\0"))
                    .as_ptr(),
            );
        }
    };
    if u == 0 as i32 as u64 { return v } else if v == 0 as i32 as u64 { return u }
    let mut __ctz_x: mp_limb_t = u | v;
    let mut __ctz_c: u32 = 0 as i32 as u32;
    let mut __clz_x: mp_limb_t = __ctz_x & __ctz_x.wrapping_neg();
    let mut __clz_c: u32 = 0 as i32 as u32;
    let mut LOCAL_SHIFT_BITS: i32 = 8 as i32;
    if (::core::mem::size_of::<mp_limb_t>() as u64).wrapping_mul(8 as i32 as u64)
        > LOCAL_SHIFT_BITS as u64
    {
        while __clz_x
            & (0xff as i32 as mp_limb_t)
                << (::core::mem::size_of::<mp_limb_t>() as u64)
                    .wrapping_mul(8 as i32 as u64)
                    .wrapping_sub(8 as i32 as u64) == 0 as i32 as u64
        {
            __clz_x <<= LOCAL_SHIFT_BITS;
            __clz_c = __clz_c.wrapping_add(8 as i32 as u32);
        }
    }
    while __clz_x
        & (1 as i32 as mp_limb_t)
            << (::core::mem::size_of::<mp_limb_t>() as u64)
                .wrapping_mul(8 as i32 as u64)
                .wrapping_sub(1 as i32 as u64) == 0 as i32 as u64
    {
        __clz_x <<= 1 as i32;
        __clz_c = __clz_c.wrapping_add(1);
        __clz_c;
    }
    __ctz_c = __clz_c;
    shift = (::core::mem::size_of::<mp_limb_t>() as u64)
        .wrapping_mul(8 as i32 as u64)
        .wrapping_sub(1 as i32 as u64)
        .wrapping_sub(__ctz_c as u64) as u32;
    u >>= shift;
    v >>= shift;
    if u & 1 as i32 as u64 == 0 as i32 as u64 {
        let mut __mp_limb_t_swap__tmp: mp_limb_t = u;
        u = v;
        v = __mp_limb_t_swap__tmp;
    }
    while v & 1 as i32 as u64 == 0 as i32 as u64 {
        v >>= 1 as i32;
    }
    while u != v {
        if u > v {
            u = (u as u64).wrapping_sub(v) as mp_limb_t as mp_limb_t;
            loop {
                u >>= 1 as i32;
                if !(u & 1 as i32 as u64 == 0 as i32 as u64) {
                    break;
                }
            }
        } else {
            v = (v as u64).wrapping_sub(u) as mp_limb_t as mp_limb_t;
            loop {
                v >>= 1 as i32;
                if !(v & 1 as i32 as u64 == 0 as i32 as u64) {
                    break;
                }
            }
        }
    }
    return u << shift;
}
#[no_mangle]
pub unsafe extern "C" fn mpz_gcd_ui(
    mut g: *mut __mpz_struct,
    mut u: *const __mpz_struct,
    mut v: u64,
) -> u64 {
    let mut t: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    mpz_init_set_ui(t.as_mut_ptr(), v);
    mpz_gcd(t.as_mut_ptr(), u, t.as_mut_ptr() as *const __mpz_struct);
    if v > 0 as i32 as u64 {
        v = mpz_get_ui(t.as_mut_ptr() as *const __mpz_struct);
    }
    if !g.is_null() {
        mpz_swap(t.as_mut_ptr(), g);
    }
    mpz_clear(t.as_mut_ptr());
    return v;
}
unsafe extern "C" fn mpz_make_odd(mut r: *mut __mpz_struct) -> mp_bitcnt_t {
    let mut shift: mp_bitcnt_t = 0;
    if (*r)._mp_size > 0 as i32 {} else {
        __assert_fail(
            b"r->_mp_size > 0\0" as *const u8 as *const i8,
            b"../mini-gmp.c\0" as *const u8 as *const i8,
            2711 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 41],
                &[i8; 41],
            >(b"mp_bitcnt_t mpz_make_odd(__mpz_struct *)\0"))
                .as_ptr(),
        );
    }
    'c_21156: {
        if (*r)._mp_size > 0 as i32 {} else {
            __assert_fail(
                b"r->_mp_size > 0\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                2711 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 41],
                    &[i8; 41],
                >(b"mp_bitcnt_t mpz_make_odd(__mpz_struct *)\0"))
                    .as_ptr(),
            );
        }
    };
    shift = mpn_common_scan(
        *((*r)._mp_d).offset(0 as i32 as isize),
        0 as i32 as mp_size_t,
        (*r)._mp_d as mp_srcptr,
        0 as i32 as mp_size_t,
        0 as i32 as mp_limb_t,
    );
    mpz_tdiv_q_2exp(r, r as *const __mpz_struct, shift);
    return shift;
}
#[no_mangle]
pub unsafe extern "C" fn mpz_gcd(
    mut g: *mut __mpz_struct,
    mut u: *const __mpz_struct,
    mut v: *const __mpz_struct,
) {
    let mut tu: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut tv: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut uz: mp_bitcnt_t = 0;
    let mut vz: mp_bitcnt_t = 0;
    let mut gz: mp_bitcnt_t = 0;
    if (*u)._mp_size == 0 as i32 {
        mpz_abs(g, v);
        return;
    }
    if (*v)._mp_size == 0 as i32 {
        mpz_abs(g, u);
        return;
    }
    mpz_init(tu.as_mut_ptr());
    mpz_init(tv.as_mut_ptr());
    mpz_abs(tu.as_mut_ptr(), u);
    uz = mpz_make_odd(tu.as_mut_ptr());
    mpz_abs(tv.as_mut_ptr(), v);
    vz = mpz_make_odd(tv.as_mut_ptr());
    gz = if uz < vz { uz } else { vz };
    if (*tu.as_mut_ptr())._mp_size < (*tv.as_mut_ptr())._mp_size {
        mpz_swap(tu.as_mut_ptr(), tv.as_mut_ptr());
    }
    mpz_tdiv_r(
        tu.as_mut_ptr(),
        tu.as_mut_ptr() as *const __mpz_struct,
        tv.as_mut_ptr() as *const __mpz_struct,
    );
    if (*tu.as_mut_ptr())._mp_size == 0 as i32 {
        mpz_swap(g, tv.as_mut_ptr());
    } else {
        loop {
            let mut c: i32 = 0;
            mpz_make_odd(tu.as_mut_ptr());
            c = mpz_cmp(
                tu.as_mut_ptr() as *const __mpz_struct,
                tv.as_mut_ptr() as *const __mpz_struct,
            );
            if c == 0 as i32 {
                mpz_swap(g, tu.as_mut_ptr());
                break;
            } else {
                if c < 0 as i32 {
                    mpz_swap(tu.as_mut_ptr(), tv.as_mut_ptr());
                }
                if (*tv.as_mut_ptr())._mp_size == 1 as i32 {
                    let mut vl: mp_limb_t = *((*tv.as_mut_ptr())._mp_d)
                        .offset(0 as i32 as isize);
                    let mut ul: mp_limb_t = mpz_tdiv_ui(
                        tu.as_mut_ptr() as *const __mpz_struct,
                        vl,
                    );
                    mpz_set_ui(g, mpn_gcd_11(ul, vl));
                    break;
                } else {
                    mpz_sub(
                        tu.as_mut_ptr(),
                        tu.as_mut_ptr() as *const __mpz_struct,
                        tv.as_mut_ptr() as *const __mpz_struct,
                    );
                }
            }
        }
    }
    mpz_clear(tu.as_mut_ptr());
    mpz_clear(tv.as_mut_ptr());
    mpz_mul_2exp(g, g as *const __mpz_struct, gz);
}
#[no_mangle]
pub unsafe extern "C" fn mpz_gcdext(
    mut g: *mut __mpz_struct,
    mut s: *mut __mpz_struct,
    mut t: *mut __mpz_struct,
    mut u: *const __mpz_struct,
    mut v: *const __mpz_struct,
) {
    let mut tu: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut tv: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut s0: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut s1: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut t0: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut t1: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut uz: mp_bitcnt_t = 0;
    let mut vz: mp_bitcnt_t = 0;
    let mut gz: mp_bitcnt_t = 0;
    let mut power: mp_bitcnt_t = 0;
    if (*u)._mp_size == 0 as i32 {
        let mut sign: i64 = mpz_sgn(v) as i64;
        mpz_abs(g, v);
        if !s.is_null() {
            (*s)._mp_size = 0 as i32;
        }
        if !t.is_null() {
            mpz_set_si(t, sign);
        }
        return;
    }
    if (*v)._mp_size == 0 as i32 {
        let mut sign_0: i64 = mpz_sgn(u) as i64;
        mpz_abs(g, u);
        if !s.is_null() {
            mpz_set_si(s, sign_0);
        }
        if !t.is_null() {
            (*t)._mp_size = 0 as i32;
        }
        return;
    }
    mpz_init(tu.as_mut_ptr());
    mpz_init(tv.as_mut_ptr());
    mpz_init(s0.as_mut_ptr());
    mpz_init(s1.as_mut_ptr());
    mpz_init(t0.as_mut_ptr());
    mpz_init(t1.as_mut_ptr());
    mpz_abs(tu.as_mut_ptr(), u);
    uz = mpz_make_odd(tu.as_mut_ptr());
    mpz_abs(tv.as_mut_ptr(), v);
    vz = mpz_make_odd(tv.as_mut_ptr());
    gz = if uz < vz { uz } else { vz };
    uz = (uz as u64).wrapping_sub(gz) as mp_bitcnt_t as mp_bitcnt_t;
    vz = (vz as u64).wrapping_sub(gz) as mp_bitcnt_t as mp_bitcnt_t;
    if (*tu.as_mut_ptr())._mp_size < (*tv.as_mut_ptr())._mp_size {
        mpz_swap(tu.as_mut_ptr(), tv.as_mut_ptr());
        let mut __mpz_srcptr_swap__tmp: mpz_srcptr = u;
        u = v;
        v = __mpz_srcptr_swap__tmp;
        let mut __mpz_ptr_swap__tmp: mpz_ptr = s;
        s = t;
        t = __mpz_ptr_swap__tmp;
        let mut __mp_bitcnt_t_swap__tmp: mp_bitcnt_t = uz;
        uz = vz;
        vz = __mp_bitcnt_t_swap__tmp;
    }
    mpz_setbit(t0.as_mut_ptr(), uz);
    mpz_tdiv_qr(
        t1.as_mut_ptr(),
        tu.as_mut_ptr(),
        tu.as_mut_ptr() as *const __mpz_struct,
        tv.as_mut_ptr() as *const __mpz_struct,
    );
    mpz_mul_2exp(t1.as_mut_ptr(), t1.as_mut_ptr() as *const __mpz_struct, uz);
    mpz_setbit(s1.as_mut_ptr(), vz);
    power = uz.wrapping_add(vz);
    if (*tu.as_mut_ptr())._mp_size > 0 as i32 {
        let mut shift: mp_bitcnt_t = 0;
        shift = mpz_make_odd(tu.as_mut_ptr());
        mpz_mul_2exp(t0.as_mut_ptr(), t0.as_mut_ptr() as *const __mpz_struct, shift);
        mpz_mul_2exp(s0.as_mut_ptr(), s0.as_mut_ptr() as *const __mpz_struct, shift);
        power = (power as u64).wrapping_add(shift) as mp_bitcnt_t as mp_bitcnt_t;
        loop {
            let mut c: i32 = 0;
            c = mpz_cmp(
                tu.as_mut_ptr() as *const __mpz_struct,
                tv.as_mut_ptr() as *const __mpz_struct,
            );
            if c == 0 as i32 {
                break;
            }
            if c < 0 as i32 {
                mpz_sub(
                    tv.as_mut_ptr(),
                    tv.as_mut_ptr() as *const __mpz_struct,
                    tu.as_mut_ptr() as *const __mpz_struct,
                );
                mpz_add(
                    t0.as_mut_ptr(),
                    t0.as_mut_ptr() as *const __mpz_struct,
                    t1.as_mut_ptr() as *const __mpz_struct,
                );
                mpz_add(
                    s0.as_mut_ptr(),
                    s0.as_mut_ptr() as *const __mpz_struct,
                    s1.as_mut_ptr() as *const __mpz_struct,
                );
                shift = mpz_make_odd(tv.as_mut_ptr());
                mpz_mul_2exp(
                    t1.as_mut_ptr(),
                    t1.as_mut_ptr() as *const __mpz_struct,
                    shift,
                );
                mpz_mul_2exp(
                    s1.as_mut_ptr(),
                    s1.as_mut_ptr() as *const __mpz_struct,
                    shift,
                );
            } else {
                mpz_sub(
                    tu.as_mut_ptr(),
                    tu.as_mut_ptr() as *const __mpz_struct,
                    tv.as_mut_ptr() as *const __mpz_struct,
                );
                mpz_add(
                    t1.as_mut_ptr(),
                    t0.as_mut_ptr() as *const __mpz_struct,
                    t1.as_mut_ptr() as *const __mpz_struct,
                );
                mpz_add(
                    s1.as_mut_ptr(),
                    s0.as_mut_ptr() as *const __mpz_struct,
                    s1.as_mut_ptr() as *const __mpz_struct,
                );
                shift = mpz_make_odd(tu.as_mut_ptr());
                mpz_mul_2exp(
                    t0.as_mut_ptr(),
                    t0.as_mut_ptr() as *const __mpz_struct,
                    shift,
                );
                mpz_mul_2exp(
                    s0.as_mut_ptr(),
                    s0.as_mut_ptr() as *const __mpz_struct,
                    shift,
                );
            }
            power = (power as u64).wrapping_add(shift) as mp_bitcnt_t as mp_bitcnt_t;
        }
    }
    mpz_mul_2exp(tv.as_mut_ptr(), tv.as_mut_ptr() as *const __mpz_struct, gz);
    mpz_neg(s0.as_mut_ptr(), s0.as_mut_ptr() as *const __mpz_struct);
    mpz_divexact(s1.as_mut_ptr(), v, tv.as_mut_ptr() as *const __mpz_struct);
    mpz_abs(s1.as_mut_ptr(), s1.as_mut_ptr() as *const __mpz_struct);
    mpz_divexact(t1.as_mut_ptr(), u, tv.as_mut_ptr() as *const __mpz_struct);
    mpz_abs(t1.as_mut_ptr(), t1.as_mut_ptr() as *const __mpz_struct);
    loop {
        let fresh28 = power;
        power = power.wrapping_sub(1);
        if !(fresh28 > 0 as i32 as u64) {
            break;
        }
        if ((*s0.as_mut_ptr())._mp_size != 0 as i32) as i32
            & *((*s0.as_mut_ptr())._mp_d).offset(0 as i32 as isize) as i32 != 0
            || ((*t0.as_mut_ptr())._mp_size != 0 as i32) as i32
                & *((*t0.as_mut_ptr())._mp_d).offset(0 as i32 as isize) as i32 != 0
        {
            mpz_sub(
                s0.as_mut_ptr(),
                s0.as_mut_ptr() as *const __mpz_struct,
                s1.as_mut_ptr() as *const __mpz_struct,
            );
            mpz_add(
                t0.as_mut_ptr(),
                t0.as_mut_ptr() as *const __mpz_struct,
                t1.as_mut_ptr() as *const __mpz_struct,
            );
        }
        if ((*t0.as_mut_ptr())._mp_size != 0 as i32) as i32
            & *((*t0.as_mut_ptr())._mp_d).offset(0 as i32 as isize) as i32 == 0
            && ((*s0.as_mut_ptr())._mp_size != 0 as i32) as i32
                & *((*s0.as_mut_ptr())._mp_d).offset(0 as i32 as isize) as i32 == 0
        {} else {
            __assert_fail(
                b"mpz_even_p (t0) && mpz_even_p (s0)\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                2934 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 108],
                    &[i8; 108],
                >(
                    b"void mpz_gcdext(__mpz_struct *, __mpz_struct *, __mpz_struct *, const __mpz_struct *, const __mpz_struct *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_21549: {
            if ((*t0.as_mut_ptr())._mp_size != 0 as i32) as i32
                & *((*t0.as_mut_ptr())._mp_d).offset(0 as i32 as isize) as i32 == 0
                && ((*s0.as_mut_ptr())._mp_size != 0 as i32) as i32
                    & *((*s0.as_mut_ptr())._mp_d).offset(0 as i32 as isize) as i32 == 0
            {} else {
                __assert_fail(
                    b"mpz_even_p (t0) && mpz_even_p (s0)\0" as *const u8 as *const i8,
                    b"../mini-gmp.c\0" as *const u8 as *const i8,
                    2934 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 108],
                        &[i8; 108],
                    >(
                        b"void mpz_gcdext(__mpz_struct *, __mpz_struct *, __mpz_struct *, const __mpz_struct *, const __mpz_struct *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        mpz_tdiv_q_2exp(
            s0.as_mut_ptr(),
            s0.as_mut_ptr() as *const __mpz_struct,
            1 as i32 as mp_bitcnt_t,
        );
        mpz_tdiv_q_2exp(
            t0.as_mut_ptr(),
            t0.as_mut_ptr() as *const __mpz_struct,
            1 as i32 as mp_bitcnt_t,
        );
    }
    mpz_add(
        s1.as_mut_ptr(),
        s0.as_mut_ptr() as *const __mpz_struct,
        s1.as_mut_ptr() as *const __mpz_struct,
    );
    if mpz_cmpabs(
        s0.as_mut_ptr() as *const __mpz_struct,
        s1.as_mut_ptr() as *const __mpz_struct,
    ) > 0 as i32
    {
        mpz_swap(s0.as_mut_ptr(), s1.as_mut_ptr());
        mpz_sub(
            t0.as_mut_ptr(),
            t0.as_mut_ptr() as *const __mpz_struct,
            t1.as_mut_ptr() as *const __mpz_struct,
        );
    }
    if (*u)._mp_size < 0 as i32 {
        mpz_neg(s0.as_mut_ptr(), s0.as_mut_ptr() as *const __mpz_struct);
    }
    if (*v)._mp_size < 0 as i32 {
        mpz_neg(t0.as_mut_ptr(), t0.as_mut_ptr() as *const __mpz_struct);
    }
    mpz_swap(g, tv.as_mut_ptr());
    if !s.is_null() {
        mpz_swap(s, s0.as_mut_ptr());
    }
    if !t.is_null() {
        mpz_swap(t, t0.as_mut_ptr());
    }
    mpz_clear(tu.as_mut_ptr());
    mpz_clear(tv.as_mut_ptr());
    mpz_clear(s0.as_mut_ptr());
    mpz_clear(s1.as_mut_ptr());
    mpz_clear(t0.as_mut_ptr());
    mpz_clear(t1.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn mpz_lcm(
    mut r: *mut __mpz_struct,
    mut u: *const __mpz_struct,
    mut v: *const __mpz_struct,
) {
    let mut g: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    if (*u)._mp_size == 0 as i32 || (*v)._mp_size == 0 as i32 {
        (*r)._mp_size = 0 as i32;
        return;
    }
    mpz_init(g.as_mut_ptr());
    mpz_gcd(g.as_mut_ptr(), u, v);
    mpz_divexact(g.as_mut_ptr(), u, g.as_mut_ptr() as *const __mpz_struct);
    mpz_mul(r, g.as_mut_ptr() as *const __mpz_struct, v);
    mpz_clear(g.as_mut_ptr());
    mpz_abs(r, r as *const __mpz_struct);
}
#[no_mangle]
pub unsafe extern "C" fn mpz_lcm_ui(
    mut r: *mut __mpz_struct,
    mut u: *const __mpz_struct,
    mut v: u64,
) {
    if v == 0 as i32 as u64 || (*u)._mp_size == 0 as i32 {
        (*r)._mp_size = 0 as i32;
        return;
    }
    v = v.wrapping_div(mpz_gcd_ui(0 as *mut __mpz_struct, u, v));
    mpz_mul_ui(r, u, v);
    mpz_abs(r, r as *const __mpz_struct);
}
#[no_mangle]
pub unsafe extern "C" fn mpz_invert(
    mut r: *mut __mpz_struct,
    mut u: *const __mpz_struct,
    mut m: *const __mpz_struct,
) -> i32 {
    let mut g: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut tr: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut invertible: i32 = 0;
    if (*u)._mp_size == 0 as i32 || mpz_cmpabs_ui(m, 1 as i32 as u64) <= 0 as i32 {
        return 0 as i32;
    }
    mpz_init(g.as_mut_ptr());
    mpz_init(tr.as_mut_ptr());
    mpz_gcdext(g.as_mut_ptr(), tr.as_mut_ptr(), 0 as *mut __mpz_struct, u, m);
    invertible = (mpz_cmp_ui(g.as_mut_ptr() as *const __mpz_struct, 1 as i32 as u64)
        == 0 as i32) as i32;
    if invertible != 0 {
        if (*tr.as_mut_ptr())._mp_size < 0 as i32 {
            if (*m)._mp_size >= 0 as i32 {
                mpz_add(tr.as_mut_ptr(), tr.as_mut_ptr() as *const __mpz_struct, m);
            } else {
                mpz_sub(tr.as_mut_ptr(), tr.as_mut_ptr() as *const __mpz_struct, m);
            }
        }
        mpz_swap(r, tr.as_mut_ptr());
    }
    mpz_clear(g.as_mut_ptr());
    mpz_clear(tr.as_mut_ptr());
    return invertible;
}
#[no_mangle]
pub unsafe extern "C" fn mpz_pow_ui(
    mut r: *mut __mpz_struct,
    mut b: *const __mpz_struct,
    mut e: u64,
) {
    let mut bit: u64 = 0;
    let mut tr: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    mpz_init_set_ui(tr.as_mut_ptr(), 1 as i32 as u64);
    bit = (1 as i32 as u64)
        << (::core::mem::size_of::<u64>() as u64)
            .wrapping_mul(8 as i32 as u64)
            .wrapping_sub(1 as i32 as u64);
    loop {
        mpz_mul(
            tr.as_mut_ptr(),
            tr.as_mut_ptr() as *const __mpz_struct,
            tr.as_mut_ptr() as *const __mpz_struct,
        );
        if e & bit != 0 {
            mpz_mul(tr.as_mut_ptr(), tr.as_mut_ptr() as *const __mpz_struct, b);
        }
        bit >>= 1 as i32;
        if !(bit > 0 as i32 as u64) {
            break;
        }
    }
    mpz_swap(r, tr.as_mut_ptr());
    mpz_clear(tr.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn mpz_ui_pow_ui(
    mut r: *mut __mpz_struct,
    mut blimb: u64,
    mut e: u64,
) {
    let mut b: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    mpz_init_set_ui(b.as_mut_ptr(), blimb);
    mpz_pow_ui(r, b.as_mut_ptr() as *const __mpz_struct, e);
    mpz_clear(b.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn mpz_powm(
    mut r: *mut __mpz_struct,
    mut b: *const __mpz_struct,
    mut e: *const __mpz_struct,
    mut m: *const __mpz_struct,
) {
    let mut tr: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut base: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut en: mp_size_t = 0;
    let mut mn: mp_size_t = 0;
    let mut mp: mp_srcptr = 0 as *const mp_limb_t;
    let mut minv: gmp_div_inverse = gmp_div_inverse {
        shift: 0,
        d1: 0,
        d0: 0,
        di: 0,
    };
    let mut shift: u32 = 0;
    let mut tp: mp_ptr = 0 as mp_ptr;
    en = (if (*e)._mp_size >= 0 as i32 { (*e)._mp_size } else { -(*e)._mp_size })
        as mp_size_t;
    mn = (if (*m)._mp_size >= 0 as i32 { (*m)._mp_size } else { -(*m)._mp_size })
        as mp_size_t;
    if mn == 0 as i32 as i64 {
        gmp_die(b"mpz_powm: Zero modulo.\0" as *const u8 as *const i8);
    }
    if en == 0 as i32 as i64 {
        mpz_set_ui(r, 1 as i32 as u64);
        return;
    }
    mp = (*m)._mp_d as mp_srcptr;
    mpn_div_qr_invert(&mut minv, mp, mn);
    shift = minv.shift;
    if shift > 0 as i32 as u32 {
        minv.shift = 0 as i32 as u32;
        tp = gmp_xalloc_limbs(mn);
        let mut __cy: mp_limb_t = mpn_lshift(tp, mp, mn, shift);
        if __cy == 0 as i32 as u64 {} else {
            __assert_fail(
                b"__cy == 0\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                3100 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 96],
                    &[i8; 96],
                >(
                    b"void mpz_powm(__mpz_struct *, const __mpz_struct *, const __mpz_struct *, const __mpz_struct *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_23154: {
            if __cy == 0 as i32 as u64 {} else {
                __assert_fail(
                    b"__cy == 0\0" as *const u8 as *const i8,
                    b"../mini-gmp.c\0" as *const u8 as *const i8,
                    3100 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 96],
                        &[i8; 96],
                    >(
                        b"void mpz_powm(__mpz_struct *, const __mpz_struct *, const __mpz_struct *, const __mpz_struct *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        mp = tp as mp_srcptr;
    }
    mpz_init(base.as_mut_ptr());
    if (*e)._mp_size < 0 as i32 {
        if mpz_invert(base.as_mut_ptr(), b, m) == 0 {
            gmp_die(
                b"mpz_powm: Negative exponent and non-invertible base.\0" as *const u8
                    as *const i8,
            );
        }
    } else {
        let mut bn: mp_size_t = 0;
        mpz_abs(base.as_mut_ptr(), b);
        bn = (*base.as_mut_ptr())._mp_size as mp_size_t;
        if bn >= mn {
            mpn_div_qr_preinv(
                0 as mp_ptr,
                (*base.as_mut_ptr())._mp_d,
                (*base.as_mut_ptr())._mp_size as mp_size_t,
                mp,
                mn,
                &mut minv,
            );
            bn = mn;
        }
        if (*b)._mp_size < 0 as i32 {
            let mut bp: mp_ptr = if mn > (*base.as_mut_ptr())._mp_alloc as i64 {
                mpz_realloc(base.as_mut_ptr(), mn)
            } else {
                (*base.as_mut_ptr())._mp_d
            };
            let mut __cy_0: mp_limb_t = mpn_sub(bp, mp, mn, bp as mp_srcptr, bn);
            if __cy_0 == 0 as i32 as u64 {} else {
                __assert_fail(
                    b"__cy == 0\0" as *const u8 as *const i8,
                    b"../mini-gmp.c\0" as *const u8 as *const i8,
                    3129 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 96],
                        &[i8; 96],
                    >(
                        b"void mpz_powm(__mpz_struct *, const __mpz_struct *, const __mpz_struct *, const __mpz_struct *)\0",
                    ))
                        .as_ptr(),
                );
            }
            'c_22977: {
                if __cy_0 == 0 as i32 as u64 {} else {
                    __assert_fail(
                        b"__cy == 0\0" as *const u8 as *const i8,
                        b"../mini-gmp.c\0" as *const u8 as *const i8,
                        3129 as i32 as u32,
                        (*::core::mem::transmute::<
                            &[u8; 96],
                            &[i8; 96],
                        >(
                            b"void mpz_powm(__mpz_struct *, const __mpz_struct *, const __mpz_struct *, const __mpz_struct *)\0",
                        ))
                            .as_ptr(),
                    );
                }
            };
            bn = mn;
        }
        (*base.as_mut_ptr())._mp_size = mpn_normalized_size(
            (*base.as_mut_ptr())._mp_d as mp_srcptr,
            bn,
        ) as i32;
    }
    mpz_init_set_ui(tr.as_mut_ptr(), 1 as i32 as u64);
    loop {
        en -= 1;
        if !(en >= 0 as i32 as i64) {
            break;
        }
        let mut w: mp_limb_t = *((*e)._mp_d).offset(en as isize);
        let mut bit: mp_limb_t = 0;
        bit = (1 as i32 as mp_limb_t)
            << (::core::mem::size_of::<mp_limb_t>() as u64)
                .wrapping_mul(8 as i32 as u64)
                .wrapping_sub(1 as i32 as u64);
        loop {
            mpz_mul(
                tr.as_mut_ptr(),
                tr.as_mut_ptr() as *const __mpz_struct,
                tr.as_mut_ptr() as *const __mpz_struct,
            );
            if w & bit != 0 {
                mpz_mul(
                    tr.as_mut_ptr(),
                    tr.as_mut_ptr() as *const __mpz_struct,
                    base.as_mut_ptr() as *const __mpz_struct,
                );
            }
            if (*tr.as_mut_ptr())._mp_size as i64 > mn {
                mpn_div_qr_preinv(
                    0 as mp_ptr,
                    (*tr.as_mut_ptr())._mp_d,
                    (*tr.as_mut_ptr())._mp_size as mp_size_t,
                    mp,
                    mn,
                    &mut minv,
                );
                (*tr.as_mut_ptr())._mp_size = mpn_normalized_size(
                    (*tr.as_mut_ptr())._mp_d as mp_srcptr,
                    mn,
                ) as i32;
            }
            bit >>= 1 as i32;
            if !(bit > 0 as i32 as u64) {
                break;
            }
        }
    }
    if (*tr.as_mut_ptr())._mp_size as i64 >= mn {
        minv.shift = shift;
        mpn_div_qr_preinv(
            0 as mp_ptr,
            (*tr.as_mut_ptr())._mp_d,
            (*tr.as_mut_ptr())._mp_size as mp_size_t,
            mp,
            mn,
            &mut minv,
        );
        (*tr.as_mut_ptr())._mp_size = mpn_normalized_size(
            (*tr.as_mut_ptr())._mp_d as mp_srcptr,
            mn,
        ) as i32;
    }
    if !tp.is_null() {
        (Some(gmp_free_func.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(tp as *mut libc::c_void, 0 as i32 as size_t);
    }
    mpz_swap(r, tr.as_mut_ptr());
    mpz_clear(tr.as_mut_ptr());
    mpz_clear(base.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn mpz_powm_ui(
    mut r: *mut __mpz_struct,
    mut b: *const __mpz_struct,
    mut elimb: u64,
    mut m: *const __mpz_struct,
) {
    let mut e: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    mpz_init_set_ui(e.as_mut_ptr(), elimb);
    mpz_powm(r, b, e.as_mut_ptr() as *const __mpz_struct, m);
    mpz_clear(e.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn mpz_rootrem(
    mut x: *mut __mpz_struct,
    mut r: *mut __mpz_struct,
    mut y: *const __mpz_struct,
    mut z: u64,
) {
    let mut sgn: i32 = 0;
    let mut t: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut u: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    sgn = ((*y)._mp_size < 0 as i32) as i32;
    if !z & sgn as u64 != 0 as i32 as u64 {
        gmp_die(
            b"mpz_rootrem: Negative argument, with even root.\0" as *const u8
                as *const i8,
        );
    }
    if z == 0 as i32 as u64 {
        gmp_die(b"mpz_rootrem: Zeroth root.\0" as *const u8 as *const i8);
    }
    if mpz_cmpabs_ui(y, 1 as i32 as u64) <= 0 as i32 {
        if !x.is_null() {
            mpz_set(x, y);
        }
        if !r.is_null() {
            (*r)._mp_size = 0 as i32;
        }
        return;
    }
    mpz_init(u.as_mut_ptr());
    mpz_init(t.as_mut_ptr());
    mpz_setbit(
        t.as_mut_ptr(),
        (mpz_sizeinbase(y, 2 as i32)).wrapping_div(z).wrapping_add(1 as i32 as u64),
    );
    if z == 2 as i32 as u64 {
        loop {
            mpz_swap(u.as_mut_ptr(), t.as_mut_ptr());
            mpz_tdiv_q(t.as_mut_ptr(), y, u.as_mut_ptr() as *const __mpz_struct);
            mpz_add(
                t.as_mut_ptr(),
                t.as_mut_ptr() as *const __mpz_struct,
                u.as_mut_ptr() as *const __mpz_struct,
            );
            mpz_tdiv_q_2exp(
                t.as_mut_ptr(),
                t.as_mut_ptr() as *const __mpz_struct,
                1 as i32 as mp_bitcnt_t,
            );
            if !(mpz_cmpabs(
                t.as_mut_ptr() as *const __mpz_struct,
                u.as_mut_ptr() as *const __mpz_struct,
            ) < 0 as i32)
            {
                break;
            }
        }
    } else {
        let mut v: mpz_t = [__mpz_struct {
            _mp_alloc: 0,
            _mp_size: 0,
            _mp_d: 0 as *mut mp_limb_t,
        }; 1];
        mpz_init(v.as_mut_ptr());
        if sgn != 0 {
            mpz_neg(t.as_mut_ptr(), t.as_mut_ptr() as *const __mpz_struct);
        }
        loop {
            mpz_swap(u.as_mut_ptr(), t.as_mut_ptr());
            mpz_pow_ui(
                t.as_mut_ptr(),
                u.as_mut_ptr() as *const __mpz_struct,
                z.wrapping_sub(1 as i32 as u64),
            );
            mpz_tdiv_q(t.as_mut_ptr(), y, t.as_mut_ptr() as *const __mpz_struct);
            mpz_mul_ui(
                v.as_mut_ptr(),
                u.as_mut_ptr() as *const __mpz_struct,
                z.wrapping_sub(1 as i32 as u64),
            );
            mpz_add(
                t.as_mut_ptr(),
                t.as_mut_ptr() as *const __mpz_struct,
                v.as_mut_ptr() as *const __mpz_struct,
            );
            mpz_tdiv_q_ui(t.as_mut_ptr(), t.as_mut_ptr() as *const __mpz_struct, z);
            if !(mpz_cmpabs(
                t.as_mut_ptr() as *const __mpz_struct,
                u.as_mut_ptr() as *const __mpz_struct,
            ) < 0 as i32)
            {
                break;
            }
        }
        mpz_clear(v.as_mut_ptr());
    }
    if !r.is_null() {
        mpz_pow_ui(t.as_mut_ptr(), u.as_mut_ptr() as *const __mpz_struct, z);
        mpz_sub(r, y, t.as_mut_ptr() as *const __mpz_struct);
    }
    if !x.is_null() {
        mpz_swap(x, u.as_mut_ptr());
    }
    mpz_clear(u.as_mut_ptr());
    mpz_clear(t.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn mpz_root(
    mut x: *mut __mpz_struct,
    mut y: *const __mpz_struct,
    mut z: u64,
) -> i32 {
    let mut res: i32 = 0;
    let mut r: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    mpz_init(r.as_mut_ptr());
    mpz_rootrem(x, r.as_mut_ptr(), y, z);
    res = ((*r.as_mut_ptr())._mp_size == 0 as i32) as i32;
    mpz_clear(r.as_mut_ptr());
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn mpz_sqrtrem(
    mut s: *mut __mpz_struct,
    mut r: *mut __mpz_struct,
    mut u: *const __mpz_struct,
) {
    mpz_rootrem(s, r, u, 2 as i32 as u64);
}
#[no_mangle]
pub unsafe extern "C" fn mpz_sqrt(mut s: *mut __mpz_struct, mut u: *const __mpz_struct) {
    mpz_rootrem(s, 0 as *mut __mpz_struct, u, 2 as i32 as u64);
}
#[no_mangle]
pub unsafe extern "C" fn mpz_perfect_square_p(mut u: *const __mpz_struct) -> i32 {
    if (*u)._mp_size <= 0 as i32 {
        return ((*u)._mp_size == 0 as i32) as i32
    } else {
        return mpz_root(0 as *mut __mpz_struct, u, 2 as i32 as u64)
    };
}
#[no_mangle]
pub unsafe extern "C" fn mpn_perfect_square_p(
    mut p: mp_srcptr,
    mut n: mp_size_t,
) -> i32 {
    let mut t: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    if n > 0 as i32 as i64 {} else {
        __assert_fail(
            b"n > 0\0" as *const u8 as *const i8,
            b"../mini-gmp.c\0" as *const u8 as *const i8,
            3284 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 47],
                &[i8; 47],
            >(b"int mpn_perfect_square_p(mp_srcptr, mp_size_t)\0"))
                .as_ptr(),
        );
    }
    'c_15741: {
        if n > 0 as i32 as i64 {} else {
            __assert_fail(
                b"n > 0\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                3284 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 47],
                    &[i8; 47],
                >(b"int mpn_perfect_square_p(mp_srcptr, mp_size_t)\0"))
                    .as_ptr(),
            );
        }
    };
    if *p.offset((n - 1 as i32 as i64) as isize) != 0 as i32 as u64 {} else {
        __assert_fail(
            b"p [n-1] != 0\0" as *const u8 as *const i8,
            b"../mini-gmp.c\0" as *const u8 as *const i8,
            3285 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 47],
                &[i8; 47],
            >(b"int mpn_perfect_square_p(mp_srcptr, mp_size_t)\0"))
                .as_ptr(),
        );
    }
    'c_15688: {
        if *p.offset((n - 1 as i32 as i64) as isize) != 0 as i32 as u64 {} else {
            __assert_fail(
                b"p [n-1] != 0\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                3285 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 47],
                    &[i8; 47],
                >(b"int mpn_perfect_square_p(mp_srcptr, mp_size_t)\0"))
                    .as_ptr(),
            );
        }
    };
    return mpz_root(
        0 as *mut __mpz_struct,
        mpz_roinit_normal_n(t.as_mut_ptr(), p, n),
        2 as i32 as u64,
    );
}
#[no_mangle]
pub unsafe extern "C" fn mpn_sqrtrem(
    mut sp: mp_ptr,
    mut rp: mp_ptr,
    mut p: mp_srcptr,
    mut n: mp_size_t,
) -> mp_size_t {
    let mut s: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut r: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut u: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut res: mp_size_t = 0;
    if n > 0 as i32 as i64 {} else {
        __assert_fail(
            b"n > 0\0" as *const u8 as *const i8,
            b"../mini-gmp.c\0" as *const u8 as *const i8,
            3295 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 60],
                &[i8; 60],
            >(b"mp_size_t mpn_sqrtrem(mp_ptr, mp_ptr, mp_srcptr, mp_size_t)\0"))
                .as_ptr(),
        );
    }
    'c_15993: {
        if n > 0 as i32 as i64 {} else {
            __assert_fail(
                b"n > 0\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                3295 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 60],
                    &[i8; 60],
                >(b"mp_size_t mpn_sqrtrem(mp_ptr, mp_ptr, mp_srcptr, mp_size_t)\0"))
                    .as_ptr(),
            );
        }
    };
    if *p.offset((n - 1 as i32 as i64) as isize) != 0 as i32 as u64 {} else {
        __assert_fail(
            b"p [n-1] != 0\0" as *const u8 as *const i8,
            b"../mini-gmp.c\0" as *const u8 as *const i8,
            3296 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 60],
                &[i8; 60],
            >(b"mp_size_t mpn_sqrtrem(mp_ptr, mp_ptr, mp_srcptr, mp_size_t)\0"))
                .as_ptr(),
        );
    }
    'c_15941: {
        if *p.offset((n - 1 as i32 as i64) as isize) != 0 as i32 as u64 {} else {
            __assert_fail(
                b"p [n-1] != 0\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                3296 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 60],
                    &[i8; 60],
                >(b"mp_size_t mpn_sqrtrem(mp_ptr, mp_ptr, mp_srcptr, mp_size_t)\0"))
                    .as_ptr(),
            );
        }
    };
    mpz_init(r.as_mut_ptr());
    mpz_init(s.as_mut_ptr());
    mpz_rootrem(
        s.as_mut_ptr(),
        r.as_mut_ptr(),
        mpz_roinit_normal_n(u.as_mut_ptr(), p, n),
        2 as i32 as u64,
    );
    if (*s.as_mut_ptr())._mp_size as i64 == (n + 1 as i32 as i64) / 2 as i32 as i64
    {} else {
        __assert_fail(
            b"s->_mp_size == (n+1)/2\0" as *const u8 as *const i8,
            b"../mini-gmp.c\0" as *const u8 as *const i8,
            3302 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 60],
                &[i8; 60],
            >(b"mp_size_t mpn_sqrtrem(mp_ptr, mp_ptr, mp_srcptr, mp_size_t)\0"))
                .as_ptr(),
        );
    }
    'c_15852: {
        if (*s.as_mut_ptr())._mp_size as i64 == (n + 1 as i32 as i64) / 2 as i32 as i64
        {} else {
            __assert_fail(
                b"s->_mp_size == (n+1)/2\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                3302 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 60],
                    &[i8; 60],
                >(b"mp_size_t mpn_sqrtrem(mp_ptr, mp_ptr, mp_srcptr, mp_size_t)\0"))
                    .as_ptr(),
            );
        }
    };
    mpn_copyd(
        sp,
        (*s.as_mut_ptr())._mp_d as mp_srcptr,
        (*s.as_mut_ptr())._mp_size as mp_size_t,
    );
    mpz_clear(s.as_mut_ptr());
    res = (*r.as_mut_ptr())._mp_size as mp_size_t;
    if !rp.is_null() {
        mpn_copyd(rp, (*r.as_mut_ptr())._mp_d as mp_srcptr, res);
    }
    mpz_clear(r.as_mut_ptr());
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn mpz_mfac_uiui(
    mut x: *mut __mpz_struct,
    mut n: u64,
    mut m: u64,
) {
    mpz_set_ui(x, n.wrapping_add((n == 0 as i32 as u64) as i32 as u64));
    if m.wrapping_add(1 as i32 as u64) < 2 as i32 as u64 {
        return;
    }
    while n > m.wrapping_add(1 as i32 as u64) {
        n = n.wrapping_sub(m);
        mpz_mul_ui(x, x as *const __mpz_struct, n);
    }
}
#[no_mangle]
pub unsafe extern "C" fn mpz_2fac_ui(mut x: *mut __mpz_struct, mut n: u64) {
    mpz_mfac_uiui(x, n, 2 as i32 as u64);
}
#[no_mangle]
pub unsafe extern "C" fn mpz_fac_ui(mut x: *mut __mpz_struct, mut n: u64) {
    mpz_mfac_uiui(x, n, 1 as i32 as u64);
}
#[no_mangle]
pub unsafe extern "C" fn mpz_bin_uiui(mut r: *mut __mpz_struct, mut n: u64, mut k: u64) {
    let mut t: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    mpz_set_ui(r, (k <= n) as i32 as u64);
    if k > n >> 1 as i32 {
        k = if k <= n { n.wrapping_sub(k) } else { 0 as i32 as u64 };
    }
    mpz_init(t.as_mut_ptr());
    mpz_fac_ui(t.as_mut_ptr(), k);
    while k > 0 as i32 as u64 {
        let fresh29 = n;
        n = n.wrapping_sub(1);
        mpz_mul_ui(r, r as *const __mpz_struct, fresh29);
        k = k.wrapping_sub(1);
        k;
    }
    mpz_divexact(r, r as *const __mpz_struct, t.as_mut_ptr() as *const __mpz_struct);
    mpz_clear(t.as_mut_ptr());
}
unsafe extern "C" fn gmp_jacobi_coprime(mut a: mp_limb_t, mut b: mp_limb_t) -> i32 {
    let mut c: i32 = 0;
    let mut bit: i32 = 0 as i32;
    if b & 1 as i32 as u64 != 0 {} else {
        __assert_fail(
            b"b & 1\0" as *const u8 as *const i8,
            b"../mini-gmp.c\0" as *const u8 as *const i8,
            3365 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 45],
                &[i8; 45],
            >(b"int gmp_jacobi_coprime(mp_limb_t, mp_limb_t)\0"))
                .as_ptr(),
        );
    }
    'c_25086: {
        if b & 1 as i32 as u64 != 0 {} else {
            __assert_fail(
                b"b & 1\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                3365 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 45],
                    &[i8; 45],
                >(b"int gmp_jacobi_coprime(mp_limb_t, mp_limb_t)\0"))
                    .as_ptr(),
            );
        }
    };
    if a != 0 as i32 as u64 {} else {
        __assert_fail(
            b"a != 0\0" as *const u8 as *const i8,
            b"../mini-gmp.c\0" as *const u8 as *const i8,
            3366 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 45],
                &[i8; 45],
            >(b"int gmp_jacobi_coprime(mp_limb_t, mp_limb_t)\0"))
                .as_ptr(),
        );
    }
    'c_25048: {
        if a != 0 as i32 as u64 {} else {
            __assert_fail(
                b"a != 0\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                3366 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 45],
                    &[i8; 45],
                >(b"int gmp_jacobi_coprime(mp_limb_t, mp_limb_t)\0"))
                    .as_ptr(),
            );
        }
    };
    b >>= 1 as i32;
    let mut __ctz_x: mp_limb_t = a;
    let mut __ctz_c: u32 = 0 as i32 as u32;
    let mut __clz_x: mp_limb_t = __ctz_x & __ctz_x.wrapping_neg();
    let mut __clz_c: u32 = 0 as i32 as u32;
    let mut LOCAL_SHIFT_BITS: i32 = 8 as i32;
    if (::core::mem::size_of::<mp_limb_t>() as u64).wrapping_mul(8 as i32 as u64)
        > LOCAL_SHIFT_BITS as u64
    {
        while __clz_x
            & (0xff as i32 as mp_limb_t)
                << (::core::mem::size_of::<mp_limb_t>() as u64)
                    .wrapping_mul(8 as i32 as u64)
                    .wrapping_sub(8 as i32 as u64) == 0 as i32 as u64
        {
            __clz_x <<= LOCAL_SHIFT_BITS;
            __clz_c = __clz_c.wrapping_add(8 as i32 as u32);
        }
    }
    while __clz_x
        & (1 as i32 as mp_limb_t)
            << (::core::mem::size_of::<mp_limb_t>() as u64)
                .wrapping_mul(8 as i32 as u64)
                .wrapping_sub(1 as i32 as u64) == 0 as i32 as u64
    {
        __clz_x <<= 1 as i32;
        __clz_c = __clz_c.wrapping_add(1);
        __clz_c;
    }
    __ctz_c = __clz_c;
    c = (::core::mem::size_of::<mp_limb_t>() as u64)
        .wrapping_mul(8 as i32 as u64)
        .wrapping_sub(1 as i32 as u64)
        .wrapping_sub(__ctz_c as u64) as i32;
    a >>= 1 as i32;
    loop {
        a >>= c;
        bit = (bit as u64 ^ c as u64 & (b ^ b >> 1 as i32)) as i32;
        if a < b {
            if a == 0 as i32 as u64 {
                return if bit & 1 as i32 != 0 { -(1 as i32) } else { 1 as i32 };
            }
            bit = (bit as u64 ^ a & b) as i32;
            a = b.wrapping_sub(a);
            b = (b as u64).wrapping_sub(a) as mp_limb_t as mp_limb_t;
        } else {
            a = (a as u64).wrapping_sub(b) as mp_limb_t as mp_limb_t;
            if a != 0 as i32 as u64 {} else {
                __assert_fail(
                    b"a != 0\0" as *const u8 as *const i8,
                    b"../mini-gmp.c\0" as *const u8 as *const i8,
                    3392 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 45],
                        &[i8; 45],
                    >(b"int gmp_jacobi_coprime(mp_limb_t, mp_limb_t)\0"))
                        .as_ptr(),
                );
            }
            'c_24818: {
                if a != 0 as i32 as u64 {} else {
                    __assert_fail(
                        b"a != 0\0" as *const u8 as *const i8,
                        b"../mini-gmp.c\0" as *const u8 as *const i8,
                        3392 as i32 as u32,
                        (*::core::mem::transmute::<
                            &[u8; 45],
                            &[i8; 45],
                        >(b"int gmp_jacobi_coprime(mp_limb_t, mp_limb_t)\0"))
                            .as_ptr(),
                    );
                }
            };
        }
        let mut __ctz_x_0: mp_limb_t = a;
        let mut __ctz_c_0: u32 = 0 as i32 as u32;
        let mut __clz_x_0: mp_limb_t = __ctz_x_0 & __ctz_x_0.wrapping_neg();
        let mut __clz_c_0: u32 = 0 as i32 as u32;
        let mut LOCAL_SHIFT_BITS_0: i32 = 8 as i32;
        if (::core::mem::size_of::<mp_limb_t>() as u64).wrapping_mul(8 as i32 as u64)
            > LOCAL_SHIFT_BITS_0 as u64
        {
            while __clz_x_0
                & (0xff as i32 as mp_limb_t)
                    << (::core::mem::size_of::<mp_limb_t>() as u64)
                        .wrapping_mul(8 as i32 as u64)
                        .wrapping_sub(8 as i32 as u64) == 0 as i32 as u64
            {
                __clz_x_0 <<= LOCAL_SHIFT_BITS_0;
                __clz_c_0 = __clz_c_0.wrapping_add(8 as i32 as u32);
            }
        }
        while __clz_x_0
            & (1 as i32 as mp_limb_t)
                << (::core::mem::size_of::<mp_limb_t>() as u64)
                    .wrapping_mul(8 as i32 as u64)
                    .wrapping_sub(1 as i32 as u64) == 0 as i32 as u64
        {
            __clz_x_0 <<= 1 as i32;
            __clz_c_0 = __clz_c_0.wrapping_add(1);
            __clz_c_0;
        }
        __ctz_c_0 = __clz_c_0;
        c = (::core::mem::size_of::<mp_limb_t>() as u64)
            .wrapping_mul(8 as i32 as u64)
            .wrapping_sub(1 as i32 as u64)
            .wrapping_sub(__ctz_c_0 as u64) as i32;
        c += 1;
        c;
    };
}
unsafe extern "C" fn gmp_lucas_step_k_2k(
    mut V: *mut __mpz_struct,
    mut Qk: *mut __mpz_struct,
    mut n: *const __mpz_struct,
) {
    mpz_mod(Qk, Qk as *const __mpz_struct, n);
    mpz_mul(V, V as *const __mpz_struct, V as *const __mpz_struct);
    mpz_submul_ui(V, Qk as *const __mpz_struct, 2 as i32 as u64);
    mpz_tdiv_r(V, V as *const __mpz_struct, n);
    mpz_mul(Qk, Qk as *const __mpz_struct, Qk as *const __mpz_struct);
}
unsafe extern "C" fn gmp_lucas_mod(
    mut V: *mut __mpz_struct,
    mut Qk: *mut __mpz_struct,
    mut Q: i64,
    mut b0: mp_bitcnt_t,
    mut n: *const __mpz_struct,
) -> i32 {
    let mut bs: mp_bitcnt_t = 0;
    let mut U: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut res: i32 = 0;
    if b0 > 0 as i32 as u64 {} else {
        __assert_fail(
            b"b0 > 0\0" as *const u8 as *const i8,
            b"../mini-gmp.c\0" as *const u8 as *const i8,
            3424 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 91],
                &[i8; 91],
            >(
                b"int gmp_lucas_mod(__mpz_struct *, __mpz_struct *, long, mp_bitcnt_t, const __mpz_struct *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_24430: {
        if b0 > 0 as i32 as u64 {} else {
            __assert_fail(
                b"b0 > 0\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                3424 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 91],
                    &[i8; 91],
                >(
                    b"int gmp_lucas_mod(__mpz_struct *, __mpz_struct *, long, mp_bitcnt_t, const __mpz_struct *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if Q <= -((-(9223372036854775807 as i64) - 1 as i64) / 2 as i32 as i64) {} else {
        __assert_fail(
            b"Q <= - (LONG_MIN / 2)\0" as *const u8 as *const i8,
            b"../mini-gmp.c\0" as *const u8 as *const i8,
            3425 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 91],
                &[i8; 91],
            >(
                b"int gmp_lucas_mod(__mpz_struct *, __mpz_struct *, long, mp_bitcnt_t, const __mpz_struct *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_24376: {
        if Q <= -((-(9223372036854775807 as i64) - 1 as i64) / 2 as i32 as i64) {} else {
            __assert_fail(
                b"Q <= - (LONG_MIN / 2)\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                3425 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 91],
                    &[i8; 91],
                >(
                    b"int gmp_lucas_mod(__mpz_struct *, __mpz_struct *, long, mp_bitcnt_t, const __mpz_struct *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if Q >= -(9223372036854775807 as i64 / 2 as i32 as i64) {} else {
        __assert_fail(
            b"Q >= - (LONG_MAX / 2)\0" as *const u8 as *const i8,
            b"../mini-gmp.c\0" as *const u8 as *const i8,
            3426 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 91],
                &[i8; 91],
            >(
                b"int gmp_lucas_mod(__mpz_struct *, __mpz_struct *, long, mp_bitcnt_t, const __mpz_struct *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_24330: {
        if Q >= -(9223372036854775807 as i64 / 2 as i32 as i64) {} else {
            __assert_fail(
                b"Q >= - (LONG_MAX / 2)\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                3426 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 91],
                    &[i8; 91],
                >(
                    b"int gmp_lucas_mod(__mpz_struct *, __mpz_struct *, long, mp_bitcnt_t, const __mpz_struct *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if mpz_cmp_ui(n, 4 as i32 as u64) > 0 as i32 {} else {
        __assert_fail(
            b"mpz_cmp_ui (n, 4) > 0\0" as *const u8 as *const i8,
            b"../mini-gmp.c\0" as *const u8 as *const i8,
            3427 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 91],
                &[i8; 91],
            >(
                b"int gmp_lucas_mod(__mpz_struct *, __mpz_struct *, long, mp_bitcnt_t, const __mpz_struct *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_24283: {
        if mpz_cmp_ui(n, 4 as i32 as u64) > 0 as i32 {} else {
            __assert_fail(
                b"mpz_cmp_ui (n, 4) > 0\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                3427 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 91],
                    &[i8; 91],
                >(
                    b"int gmp_lucas_mod(__mpz_struct *, __mpz_struct *, long, mp_bitcnt_t, const __mpz_struct *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if ((*n)._mp_size != 0 as i32) as i32
        & *((*n)._mp_d).offset(0 as i32 as isize) as i32 != 0
    {} else {
        __assert_fail(
            b"mpz_odd_p (n)\0" as *const u8 as *const i8,
            b"../mini-gmp.c\0" as *const u8 as *const i8,
            3428 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 91],
                &[i8; 91],
            >(
                b"int gmp_lucas_mod(__mpz_struct *, __mpz_struct *, long, mp_bitcnt_t, const __mpz_struct *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_24217: {
        if ((*n)._mp_size != 0 as i32) as i32
            & *((*n)._mp_d).offset(0 as i32 as isize) as i32 != 0
        {} else {
            __assert_fail(
                b"mpz_odd_p (n)\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                3428 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 91],
                    &[i8; 91],
                >(
                    b"int gmp_lucas_mod(__mpz_struct *, __mpz_struct *, long, mp_bitcnt_t, const __mpz_struct *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    mpz_init_set_ui(U.as_mut_ptr(), 1 as i32 as u64);
    mpz_set_ui(V, 1 as i32 as u64);
    mpz_set_si(Qk, Q);
    bs = (mpz_sizeinbase(n, 2 as i32)).wrapping_sub(1 as i32 as u64);
    loop {
        bs = bs.wrapping_sub(1);
        if !(bs >= b0) {
            break;
        }
        mpz_mul(
            U.as_mut_ptr(),
            U.as_mut_ptr() as *const __mpz_struct,
            V as *const __mpz_struct,
        );
        gmp_lucas_step_k_2k(V, Qk, n);
        if b0 == bs || mpz_tstbit(n, bs) != 0 {
            mpz_mul_si(Qk, Qk as *const __mpz_struct, Q);
            mpz_swap(U.as_mut_ptr(), V);
            mpz_add(
                U.as_mut_ptr(),
                U.as_mut_ptr() as *const __mpz_struct,
                V as *const __mpz_struct,
            );
            if ((*U.as_mut_ptr())._mp_size != 0 as i32) as i32
                & *((*U.as_mut_ptr())._mp_d).offset(0 as i32 as isize) as i32 != 0
            {
                mpz_add(U.as_mut_ptr(), U.as_mut_ptr() as *const __mpz_struct, n);
            }
            mpz_tdiv_q_2exp(
                U.as_mut_ptr(),
                U.as_mut_ptr() as *const __mpz_struct,
                1 as i32 as mp_bitcnt_t,
            );
            mpz_mul_si(V, V as *const __mpz_struct, -(2 as i32) as i64 * Q);
            mpz_add(V, U.as_mut_ptr() as *const __mpz_struct, V as *const __mpz_struct);
            mpz_tdiv_r(V, V as *const __mpz_struct, n);
        }
        mpz_tdiv_r(U.as_mut_ptr(), U.as_mut_ptr() as *const __mpz_struct, n);
    }
    res = ((*U.as_mut_ptr())._mp_size == 0 as i32) as i32;
    mpz_clear(U.as_mut_ptr());
    return res;
}
unsafe extern "C" fn gmp_stronglucas(
    mut x: *const __mpz_struct,
    mut Qk: *mut __mpz_struct,
) -> i32 {
    let mut b0: mp_bitcnt_t = 0;
    let mut V: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut n: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut maxD: mp_limb_t = 0;
    let mut D: mp_limb_t = 0;
    let mut Q: i64 = 0;
    let mut tl: mp_limb_t = 0;
    mpz_roinit_normal_n(
        n.as_mut_ptr(),
        (*x)._mp_d as mp_srcptr,
        (if (*x)._mp_size >= 0 as i32 { (*x)._mp_size } else { -(*x)._mp_size })
            as mp_size_t,
    );
    if ((*n.as_mut_ptr())._mp_size != 0 as i32) as i32
        & *((*n.as_mut_ptr())._mp_d).offset(0 as i32 as isize) as i32 != 0
    {} else {
        __assert_fail(
            b"mpz_odd_p (n)\0" as *const u8 as *const i8,
            b"../mini-gmp.c\0" as *const u8 as *const i8,
            3486 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 58],
                &[i8; 58],
            >(b"int gmp_stronglucas(const __mpz_struct *, __mpz_struct *)\0"))
                .as_ptr(),
        );
    }
    'c_25208: {
        if ((*n.as_mut_ptr())._mp_size != 0 as i32) as i32
            & *((*n.as_mut_ptr())._mp_d).offset(0 as i32 as isize) as i32 != 0
        {} else {
            __assert_fail(
                b"mpz_odd_p (n)\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                3486 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 58],
                    &[i8; 58],
                >(b"int gmp_stronglucas(const __mpz_struct *, __mpz_struct *)\0"))
                    .as_ptr(),
            );
        }
    };
    if mpz_root(Qk, n.as_mut_ptr() as *const __mpz_struct, 2 as i32 as u64) != 0 {
        return 0 as i32;
    }
    maxD = if (*Qk)._mp_size == 1 as i32 {
        (*((*Qk)._mp_d).offset(0 as i32 as isize)).wrapping_sub(1 as i32 as u64)
    } else {
        !(0 as i32 as mp_limb_t)
    };
    D = 3 as i32 as mp_limb_t;
    loop {
        if D >= maxD {
            return 1 as i32 + (D != !(0 as i32 as mp_limb_t)) as i32;
        }
        D = (D as u64).wrapping_add(2 as i32 as u64) as mp_limb_t as mp_limb_t;
        tl = mpz_tdiv_ui(n.as_mut_ptr() as *const __mpz_struct, D);
        if tl == 0 as i32 as u64 {
            return 0 as i32;
        }
        if !(gmp_jacobi_coprime(tl, D) == 1 as i32) {
            break;
        }
    }
    mpz_init(V.as_mut_ptr());
    b0 = mpz_scan0(n.as_mut_ptr() as *const __mpz_struct, 0 as i32 as mp_bitcnt_t);
    Q = if D & 2 as i32 as u64 != 0 {
        (D >> 2 as i32) as i64 + 1 as i32 as i64
    } else {
        -((D >> 2 as i32) as i64)
    };
    if gmp_lucas_mod(V.as_mut_ptr(), Qk, Q, b0, n.as_mut_ptr() as *const __mpz_struct)
        == 0
    {
        while (*V.as_mut_ptr())._mp_size != 0 as i32
            && {
                b0 = b0.wrapping_sub(1);
                b0 != 0 as i32 as u64
            }
        {
            gmp_lucas_step_k_2k(
                V.as_mut_ptr(),
                Qk,
                n.as_mut_ptr() as *const __mpz_struct,
            );
        }
    }
    mpz_clear(V.as_mut_ptr());
    return (b0 != 0 as i32 as u64) as i32;
}
unsafe extern "C" fn gmp_millerrabin(
    mut n: *const __mpz_struct,
    mut nm1: *const __mpz_struct,
    mut y: *mut __mpz_struct,
    mut q: *const __mpz_struct,
    mut k: mp_bitcnt_t,
) -> i32 {
    if k > 0 as i32 as u64 {} else {
        __assert_fail(
            b"k > 0\0" as *const u8 as *const i8,
            b"../mini-gmp.c\0" as *const u8 as *const i8,
            3531 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 115],
                &[i8; 115],
            >(
                b"int gmp_millerrabin(const __mpz_struct *, const __mpz_struct *, __mpz_struct *, const __mpz_struct *, mp_bitcnt_t)\0",
            ))
                .as_ptr(),
        );
    }
    'c_23701: {
        if k > 0 as i32 as u64 {} else {
            __assert_fail(
                b"k > 0\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                3531 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 115],
                    &[i8; 115],
                >(
                    b"int gmp_millerrabin(const __mpz_struct *, const __mpz_struct *, __mpz_struct *, const __mpz_struct *, mp_bitcnt_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    mpz_powm(y, y as *const __mpz_struct, q, n);
    if mpz_cmp_ui(y as *const __mpz_struct, 1 as i32 as u64) == 0 as i32
        || mpz_cmp(y as *const __mpz_struct, nm1) == 0 as i32
    {
        return 1 as i32;
    }
    loop {
        k = k.wrapping_sub(1);
        if !(k > 0 as i32 as u64) {
            break;
        }
        mpz_powm_ui(y, y as *const __mpz_struct, 2 as i32 as u64, n);
        if mpz_cmp(y as *const __mpz_struct, nm1) == 0 as i32 {
            return 1 as i32;
        }
        if mpz_cmp_ui(y as *const __mpz_struct, 1 as i32 as u64) <= 0 as i32 {
            return 0 as i32;
        }
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn mpz_probab_prime_p(
    mut n: *const __mpz_struct,
    mut reps: i32,
) -> i32 {
    let mut nm1: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut q: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut y: mpz_t = [__mpz_struct {
        _mp_alloc: 0,
        _mp_size: 0,
        _mp_d: 0 as *mut mp_limb_t,
    }; 1];
    let mut k: mp_bitcnt_t = 0;
    let mut is_prime: i32 = 0;
    let mut j: i32 = 0;
    if ((*n)._mp_size != 0 as i32) as i32
        & *((*n)._mp_d).offset(0 as i32 as isize) as i32 == 0
    {
        return if mpz_cmpabs_ui(n, 2 as i32 as u64) == 0 as i32 {
            2 as i32
        } else {
            0 as i32
        };
    }
    if (*n)._mp_size != 0 as i32 {} else {
        __assert_fail(
            b"n->_mp_size != 0\0" as *const u8 as *const i8,
            b"../mini-gmp.c\0" as *const u8 as *const i8,
            3576 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 50],
                &[i8; 50],
            >(b"int mpz_probab_prime_p(const __mpz_struct *, int)\0"))
                .as_ptr(),
        );
    }
    'c_25617: {
        if (*n)._mp_size != 0 as i32 {} else {
            __assert_fail(
                b"n->_mp_size != 0\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                3576 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 50],
                    &[i8; 50],
                >(b"int mpz_probab_prime_p(const __mpz_struct *, int)\0"))
                    .as_ptr(),
            );
        }
    };
    if mpz_cmpabs_ui(n, 64 as i32 as u64) < 0 as i32 {
        return (0xc96996dc as u64
            >> (*((*n)._mp_d).offset(0 as i32 as isize) >> 1 as i32) & 2 as i32 as u64)
            as i32;
    }
    if mpz_gcd_ui(
        0 as *mut __mpz_struct,
        n,
        (3 as u64)
            .wrapping_mul(5 as u64)
            .wrapping_mul(7 as u64)
            .wrapping_mul(11 as u64)
            .wrapping_mul(13 as u64)
            .wrapping_mul(17 as u64)
            .wrapping_mul(19 as u64)
            .wrapping_mul(23 as u64)
            .wrapping_mul(29 as u64),
    ) != 1 as i32 as u64
    {
        return 0 as i32;
    }
    if mpz_cmpabs_ui(n, (31 as i32 * 31 as i32) as u64) < 0 as i32 {
        return 2 as i32;
    }
    mpz_init(nm1.as_mut_ptr());
    mpz_init(q.as_mut_ptr());
    mpz_abs(nm1.as_mut_ptr(), n);
    let ref mut fresh30 = *((*nm1.as_mut_ptr())._mp_d).offset(0 as i32 as isize);
    *fresh30 = (*fresh30 as u64).wrapping_sub(1 as i32 as u64) as mp_limb_t as mp_limb_t;
    k = mpz_scan1(nm1.as_mut_ptr() as *const __mpz_struct, 0 as i32 as mp_bitcnt_t);
    mpz_tdiv_q_2exp(q.as_mut_ptr(), nm1.as_mut_ptr() as *const __mpz_struct, k);
    mpz_init_set_ui(y.as_mut_ptr(), 2 as i32 as u64);
    is_prime = (gmp_millerrabin(
        n,
        nm1.as_mut_ptr() as *const __mpz_struct,
        y.as_mut_ptr(),
        q.as_mut_ptr() as *const __mpz_struct,
        k,
    ) != 0 && gmp_stronglucas(n, y.as_mut_ptr()) != 0) as i32;
    reps -= 24 as i32;
    j = 0 as i32;
    while is_prime & (j < reps) as i32 != 0 {
        mpz_set_ui(
            y.as_mut_ptr(),
            (j as u64)
                .wrapping_mul(j as u64)
                .wrapping_add(j as u64)
                .wrapping_add(41 as i32 as u64),
        );
        if mpz_cmp(
            y.as_mut_ptr() as *const __mpz_struct,
            nm1.as_mut_ptr() as *const __mpz_struct,
        ) >= 0 as i32
        {
            if j >= 30 as i32 {} else {
                __assert_fail(
                    b"j >= 30\0" as *const u8 as *const i8,
                    b"../mini-gmp.c\0" as *const u8 as *const i8,
                    3614 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 50],
                        &[i8; 50],
                    >(b"int mpz_probab_prime_p(const __mpz_struct *, int)\0"))
                        .as_ptr(),
                );
            }
            'c_23744: {
                if j >= 30 as i32 {} else {
                    __assert_fail(
                        b"j >= 30\0" as *const u8 as *const i8,
                        b"../mini-gmp.c\0" as *const u8 as *const i8,
                        3614 as i32 as u32,
                        (*::core::mem::transmute::<
                            &[u8; 50],
                            &[i8; 50],
                        >(b"int mpz_probab_prime_p(const __mpz_struct *, int)\0"))
                            .as_ptr(),
                    );
                }
            };
            break;
        } else {
            is_prime = gmp_millerrabin(
                n,
                nm1.as_mut_ptr() as *const __mpz_struct,
                y.as_mut_ptr(),
                q.as_mut_ptr() as *const __mpz_struct,
                k,
            );
            j += 1;
            j;
        }
    }
    mpz_clear(nm1.as_mut_ptr());
    mpz_clear(q.as_mut_ptr());
    mpz_clear(y.as_mut_ptr());
    return is_prime;
}
#[no_mangle]
pub unsafe extern "C" fn mpz_tstbit(
    mut d: *const __mpz_struct,
    mut bit_index: mp_bitcnt_t,
) -> i32 {
    let mut limb_index: mp_size_t = 0;
    let mut shift: u32 = 0;
    let mut ds: mp_size_t = 0;
    let mut dn: mp_size_t = 0;
    let mut w: mp_limb_t = 0;
    let mut bit: i32 = 0;
    ds = (*d)._mp_size as mp_size_t;
    dn = if ds >= 0 as i32 as i64 { ds } else { -ds };
    limb_index = bit_index
        .wrapping_div(
            (::core::mem::size_of::<mp_limb_t>() as u64).wrapping_mul(8 as i32 as u64),
        ) as mp_size_t;
    if limb_index >= dn {
        return (ds < 0 as i32 as i64) as i32;
    }
    shift = bit_index
        .wrapping_rem(
            (::core::mem::size_of::<mp_limb_t>() as u64).wrapping_mul(8 as i32 as u64),
        ) as u32;
    w = *((*d)._mp_d).offset(limb_index as isize);
    bit = (w >> shift & 1 as i32 as u64) as i32;
    if ds < 0 as i32 as i64 {
        if shift > 0 as i32 as u32
            && w
                << (::core::mem::size_of::<mp_limb_t>() as u64)
                    .wrapping_mul(8 as i32 as u64)
                    .wrapping_sub(shift as u64) > 0 as i32 as u64
        {
            return bit ^ 1 as i32;
        }
        loop {
            limb_index -= 1;
            if !(limb_index >= 0 as i32 as i64) {
                break;
            }
            if *((*d)._mp_d).offset(limb_index as isize) > 0 as i32 as u64 {
                return bit ^ 1 as i32;
            }
        }
    }
    return bit;
}
unsafe extern "C" fn mpz_abs_add_bit(
    mut d: *mut __mpz_struct,
    mut bit_index: mp_bitcnt_t,
) {
    let mut dn: mp_size_t = 0;
    let mut limb_index: mp_size_t = 0;
    let mut bit: mp_limb_t = 0;
    let mut dp: mp_ptr = 0 as *mut mp_limb_t;
    dn = (if (*d)._mp_size >= 0 as i32 { (*d)._mp_size } else { -(*d)._mp_size })
        as mp_size_t;
    limb_index = bit_index
        .wrapping_div(
            (::core::mem::size_of::<mp_limb_t>() as u64).wrapping_mul(8 as i32 as u64),
        ) as mp_size_t;
    bit = (1 as i32 as mp_limb_t)
        << bit_index
            .wrapping_rem(
                (::core::mem::size_of::<mp_limb_t>() as u64)
                    .wrapping_mul(8 as i32 as u64),
            );
    if limb_index >= dn {
        let mut i: mp_size_t = 0;
        dp = if limb_index + 1 as i32 as i64 > (*d)._mp_alloc as i64 {
            mpz_realloc(d, limb_index + 1 as i32 as i64)
        } else {
            (*d)._mp_d
        };
        *dp.offset(limb_index as isize) = bit;
        i = dn;
        while i < limb_index {
            *dp.offset(i as isize) = 0 as i32 as mp_limb_t;
            i += 1;
            i;
        }
        dn = limb_index + 1 as i32 as i64;
    } else {
        let mut cy: mp_limb_t = 0;
        dp = (*d)._mp_d;
        cy = mpn_add_1(
            dp.offset(limb_index as isize),
            dp.offset(limb_index as isize) as mp_srcptr,
            dn - limb_index,
            bit,
        );
        if cy > 0 as i32 as u64 {
            dp = if dn + 1 as i32 as i64 > (*d)._mp_alloc as i64 {
                mpz_realloc(d, dn + 1 as i32 as i64)
            } else {
                (*d)._mp_d
            };
            let fresh31 = dn;
            dn = dn + 1;
            *dp.offset(fresh31 as isize) = cy;
        }
    }
    (*d)._mp_size = (if (*d)._mp_size < 0 as i32 { -dn } else { dn }) as i32;
}
unsafe extern "C" fn mpz_abs_sub_bit(
    mut d: *mut __mpz_struct,
    mut bit_index: mp_bitcnt_t,
) {
    let mut dn: mp_size_t = 0;
    let mut limb_index: mp_size_t = 0;
    let mut dp: mp_ptr = 0 as *mut mp_limb_t;
    let mut bit: mp_limb_t = 0;
    dn = (if (*d)._mp_size >= 0 as i32 { (*d)._mp_size } else { -(*d)._mp_size })
        as mp_size_t;
    dp = (*d)._mp_d;
    limb_index = bit_index
        .wrapping_div(
            (::core::mem::size_of::<mp_limb_t>() as u64).wrapping_mul(8 as i32 as u64),
        ) as mp_size_t;
    bit = (1 as i32 as mp_limb_t)
        << bit_index
            .wrapping_rem(
                (::core::mem::size_of::<mp_limb_t>() as u64)
                    .wrapping_mul(8 as i32 as u64),
            );
    if limb_index < dn {} else {
        __assert_fail(
            b"limb_index < dn\0" as *const u8 as *const i8,
            b"../mini-gmp.c\0" as *const u8 as *const i8,
            3738 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 50],
                &[i8; 50],
            >(b"void mpz_abs_sub_bit(__mpz_struct *, mp_bitcnt_t)\0"))
                .as_ptr(),
        );
    }
    'c_14917: {
        if limb_index < dn {} else {
            __assert_fail(
                b"limb_index < dn\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                3738 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 50],
                    &[i8; 50],
                >(b"void mpz_abs_sub_bit(__mpz_struct *, mp_bitcnt_t)\0"))
                    .as_ptr(),
            );
        }
    };
    let mut __cy: mp_limb_t = mpn_sub_1(
        dp.offset(limb_index as isize),
        dp.offset(limb_index as isize) as mp_srcptr,
        dn - limb_index,
        bit,
    );
    if __cy == 0 as i32 as u64 {} else {
        __assert_fail(
            b"__cy == 0\0" as *const u8 as *const i8,
            b"../mini-gmp.c\0" as *const u8 as *const i8,
            3741 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 50],
                &[i8; 50],
            >(b"void mpz_abs_sub_bit(__mpz_struct *, mp_bitcnt_t)\0"))
                .as_ptr(),
        );
    }
    'c_14853: {
        if __cy == 0 as i32 as u64 {} else {
            __assert_fail(
                b"__cy == 0\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                3741 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 50],
                    &[i8; 50],
                >(b"void mpz_abs_sub_bit(__mpz_struct *, mp_bitcnt_t)\0"))
                    .as_ptr(),
            );
        }
    };
    dn = mpn_normalized_size(dp as mp_srcptr, dn);
    (*d)._mp_size = (if (*d)._mp_size < 0 as i32 { -dn } else { dn }) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn mpz_setbit(
    mut d: *mut __mpz_struct,
    mut bit_index: mp_bitcnt_t,
) {
    if mpz_tstbit(d as *const __mpz_struct, bit_index) == 0 {
        if (*d)._mp_size >= 0 as i32 {
            mpz_abs_add_bit(d, bit_index);
        } else {
            mpz_abs_sub_bit(d, bit_index);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn mpz_clrbit(
    mut d: *mut __mpz_struct,
    mut bit_index: mp_bitcnt_t,
) {
    if mpz_tstbit(d as *const __mpz_struct, bit_index) != 0 {
        if (*d)._mp_size >= 0 as i32 {
            mpz_abs_sub_bit(d, bit_index);
        } else {
            mpz_abs_add_bit(d, bit_index);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn mpz_combit(
    mut d: *mut __mpz_struct,
    mut bit_index: mp_bitcnt_t,
) {
    if mpz_tstbit(d as *const __mpz_struct, bit_index)
        ^ ((*d)._mp_size < 0 as i32) as i32 != 0
    {
        mpz_abs_sub_bit(d, bit_index);
    } else {
        mpz_abs_add_bit(d, bit_index);
    };
}
#[no_mangle]
pub unsafe extern "C" fn mpz_com(mut r: *mut __mpz_struct, mut u: *const __mpz_struct) {
    mpz_add_ui(r, u, 1 as i32 as u64);
    mpz_neg(r, r as *const __mpz_struct);
}
#[no_mangle]
pub unsafe extern "C" fn mpz_and(
    mut r: *mut __mpz_struct,
    mut u: *const __mpz_struct,
    mut v: *const __mpz_struct,
) {
    let mut un: mp_size_t = 0;
    let mut vn: mp_size_t = 0;
    let mut rn: mp_size_t = 0;
    let mut i: mp_size_t = 0;
    let mut up: mp_ptr = 0 as *mut mp_limb_t;
    let mut vp: mp_ptr = 0 as *mut mp_limb_t;
    let mut rp: mp_ptr = 0 as *mut mp_limb_t;
    let mut ux: mp_limb_t = 0;
    let mut vx: mp_limb_t = 0;
    let mut rx: mp_limb_t = 0;
    let mut uc: mp_limb_t = 0;
    let mut vc: mp_limb_t = 0;
    let mut rc: mp_limb_t = 0;
    let mut ul: mp_limb_t = 0;
    let mut vl: mp_limb_t = 0;
    let mut rl: mp_limb_t = 0;
    un = (if (*u)._mp_size >= 0 as i32 { (*u)._mp_size } else { -(*u)._mp_size })
        as mp_size_t;
    vn = (if (*v)._mp_size >= 0 as i32 { (*v)._mp_size } else { -(*v)._mp_size })
        as mp_size_t;
    if un < vn {
        let mut __mpz_srcptr_swap__tmp: mpz_srcptr = u;
        u = v;
        v = __mpz_srcptr_swap__tmp;
        let mut __mp_size_t_swap__tmp: mp_size_t = un;
        un = vn;
        vn = __mp_size_t_swap__tmp;
    }
    if vn == 0 as i32 as i64 {
        (*r)._mp_size = 0 as i32;
        return;
    }
    uc = ((*u)._mp_size < 0 as i32) as i32 as mp_limb_t;
    vc = ((*v)._mp_size < 0 as i32) as i32 as mp_limb_t;
    rc = uc & vc;
    ux = uc.wrapping_neg();
    vx = vc.wrapping_neg();
    rx = rc.wrapping_neg();
    rn = if vx != 0 { un } else { vn };
    rp = if rn + rc as mp_size_t > (*r)._mp_alloc as i64 {
        mpz_realloc(r, rn + rc as mp_size_t)
    } else {
        (*r)._mp_d
    };
    up = (*u)._mp_d;
    vp = (*v)._mp_d;
    i = 0 as i32 as mp_size_t;
    loop {
        ul = (*up.offset(i as isize) ^ ux).wrapping_add(uc);
        uc = (ul < uc) as i32 as mp_limb_t;
        vl = (*vp.offset(i as isize) ^ vx).wrapping_add(vc);
        vc = (vl < vc) as i32 as mp_limb_t;
        rl = (ul & vl ^ rx).wrapping_add(rc);
        rc = (rl < rc) as i32 as mp_limb_t;
        *rp.offset(i as isize) = rl;
        i += 1;
        if !(i < vn) {
            break;
        }
    }
    if vc == 0 as i32 as u64 {} else {
        __assert_fail(
            b"vc == 0\0" as *const u8 as *const i8,
            b"../mini-gmp.c\0" as *const u8 as *const i8,
            3839 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 73],
                &[i8; 73],
            >(
                b"void mpz_and(__mpz_struct *, const __mpz_struct *, const __mpz_struct *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_25934: {
        if vc == 0 as i32 as u64 {} else {
            __assert_fail(
                b"vc == 0\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                3839 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 73],
                    &[i8; 73],
                >(
                    b"void mpz_and(__mpz_struct *, const __mpz_struct *, const __mpz_struct *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    while i < rn {
        ul = (*up.offset(i as isize) ^ ux).wrapping_add(uc);
        uc = (ul < uc) as i32 as mp_limb_t;
        rl = (ul & vx ^ rx).wrapping_add(rc);
        rc = (rl < rc) as i32 as mp_limb_t;
        *rp.offset(i as isize) = rl;
        i += 1;
        i;
    }
    if rc != 0 {
        let fresh32 = rn;
        rn = rn + 1;
        *rp.offset(fresh32 as isize) = rc;
    } else {
        rn = mpn_normalized_size(rp as mp_srcptr, rn);
    }
    (*r)._mp_size = (if rx != 0 { -rn } else { rn }) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn mpz_ior(
    mut r: *mut __mpz_struct,
    mut u: *const __mpz_struct,
    mut v: *const __mpz_struct,
) {
    let mut un: mp_size_t = 0;
    let mut vn: mp_size_t = 0;
    let mut rn: mp_size_t = 0;
    let mut i: mp_size_t = 0;
    let mut up: mp_ptr = 0 as *mut mp_limb_t;
    let mut vp: mp_ptr = 0 as *mut mp_limb_t;
    let mut rp: mp_ptr = 0 as *mut mp_limb_t;
    let mut ux: mp_limb_t = 0;
    let mut vx: mp_limb_t = 0;
    let mut rx: mp_limb_t = 0;
    let mut uc: mp_limb_t = 0;
    let mut vc: mp_limb_t = 0;
    let mut rc: mp_limb_t = 0;
    let mut ul: mp_limb_t = 0;
    let mut vl: mp_limb_t = 0;
    let mut rl: mp_limb_t = 0;
    un = (if (*u)._mp_size >= 0 as i32 { (*u)._mp_size } else { -(*u)._mp_size })
        as mp_size_t;
    vn = (if (*v)._mp_size >= 0 as i32 { (*v)._mp_size } else { -(*v)._mp_size })
        as mp_size_t;
    if un < vn {
        let mut __mpz_srcptr_swap__tmp: mpz_srcptr = u;
        u = v;
        v = __mpz_srcptr_swap__tmp;
        let mut __mp_size_t_swap__tmp: mp_size_t = un;
        un = vn;
        vn = __mp_size_t_swap__tmp;
    }
    if vn == 0 as i32 as i64 {
        mpz_set(r, u);
        return;
    }
    uc = ((*u)._mp_size < 0 as i32) as i32 as mp_limb_t;
    vc = ((*v)._mp_size < 0 as i32) as i32 as mp_limb_t;
    rc = uc | vc;
    ux = uc.wrapping_neg();
    vx = vc.wrapping_neg();
    rx = rc.wrapping_neg();
    rn = if vx != 0 { vn } else { un };
    rp = if rn + rc as mp_size_t > (*r)._mp_alloc as i64 {
        mpz_realloc(r, rn + rc as mp_size_t)
    } else {
        (*r)._mp_d
    };
    up = (*u)._mp_d;
    vp = (*v)._mp_d;
    i = 0 as i32 as mp_size_t;
    loop {
        ul = (*up.offset(i as isize) ^ ux).wrapping_add(uc);
        uc = (ul < uc) as i32 as mp_limb_t;
        vl = (*vp.offset(i as isize) ^ vx).wrapping_add(vc);
        vc = (vl < vc) as i32 as mp_limb_t;
        rl = ((ul | vl) ^ rx).wrapping_add(rc);
        rc = (rl < rc) as i32 as mp_limb_t;
        *rp.offset(i as isize) = rl;
        i += 1;
        if !(i < vn) {
            break;
        }
    }
    if vc == 0 as i32 as u64 {} else {
        __assert_fail(
            b"vc == 0\0" as *const u8 as *const i8,
            b"../mini-gmp.c\0" as *const u8 as *const i8,
            3912 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 73],
                &[i8; 73],
            >(
                b"void mpz_ior(__mpz_struct *, const __mpz_struct *, const __mpz_struct *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_26416: {
        if vc == 0 as i32 as u64 {} else {
            __assert_fail(
                b"vc == 0\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                3912 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 73],
                    &[i8; 73],
                >(
                    b"void mpz_ior(__mpz_struct *, const __mpz_struct *, const __mpz_struct *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    while i < rn {
        ul = (*up.offset(i as isize) ^ ux).wrapping_add(uc);
        uc = (ul < uc) as i32 as mp_limb_t;
        rl = ((ul | vx) ^ rx).wrapping_add(rc);
        rc = (rl < rc) as i32 as mp_limb_t;
        *rp.offset(i as isize) = rl;
        i += 1;
        i;
    }
    if rc != 0 {
        let fresh33 = rn;
        rn = rn + 1;
        *rp.offset(fresh33 as isize) = rc;
    } else {
        rn = mpn_normalized_size(rp as mp_srcptr, rn);
    }
    (*r)._mp_size = (if rx != 0 { -rn } else { rn }) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn mpz_xor(
    mut r: *mut __mpz_struct,
    mut u: *const __mpz_struct,
    mut v: *const __mpz_struct,
) {
    let mut un: mp_size_t = 0;
    let mut vn: mp_size_t = 0;
    let mut i: mp_size_t = 0;
    let mut up: mp_ptr = 0 as *mut mp_limb_t;
    let mut vp: mp_ptr = 0 as *mut mp_limb_t;
    let mut rp: mp_ptr = 0 as *mut mp_limb_t;
    let mut ux: mp_limb_t = 0;
    let mut vx: mp_limb_t = 0;
    let mut rx: mp_limb_t = 0;
    let mut uc: mp_limb_t = 0;
    let mut vc: mp_limb_t = 0;
    let mut rc: mp_limb_t = 0;
    let mut ul: mp_limb_t = 0;
    let mut vl: mp_limb_t = 0;
    let mut rl: mp_limb_t = 0;
    un = (if (*u)._mp_size >= 0 as i32 { (*u)._mp_size } else { -(*u)._mp_size })
        as mp_size_t;
    vn = (if (*v)._mp_size >= 0 as i32 { (*v)._mp_size } else { -(*v)._mp_size })
        as mp_size_t;
    if un < vn {
        let mut __mpz_srcptr_swap__tmp: mpz_srcptr = u;
        u = v;
        v = __mpz_srcptr_swap__tmp;
        let mut __mp_size_t_swap__tmp: mp_size_t = un;
        un = vn;
        vn = __mp_size_t_swap__tmp;
    }
    if vn == 0 as i32 as i64 {
        mpz_set(r, u);
        return;
    }
    uc = ((*u)._mp_size < 0 as i32) as i32 as mp_limb_t;
    vc = ((*v)._mp_size < 0 as i32) as i32 as mp_limb_t;
    rc = uc ^ vc;
    ux = uc.wrapping_neg();
    vx = vc.wrapping_neg();
    rx = rc.wrapping_neg();
    rp = if un + rc as mp_size_t > (*r)._mp_alloc as i64 {
        mpz_realloc(r, un + rc as mp_size_t)
    } else {
        (*r)._mp_d
    };
    up = (*u)._mp_d;
    vp = (*v)._mp_d;
    i = 0 as i32 as mp_size_t;
    loop {
        ul = (*up.offset(i as isize) ^ ux).wrapping_add(uc);
        uc = (ul < uc) as i32 as mp_limb_t;
        vl = (*vp.offset(i as isize) ^ vx).wrapping_add(vc);
        vc = (vl < vc) as i32 as mp_limb_t;
        rl = (ul ^ vl ^ rx).wrapping_add(rc);
        rc = (rl < rc) as i32 as mp_limb_t;
        *rp.offset(i as isize) = rl;
        i += 1;
        if !(i < vn) {
            break;
        }
    }
    if vc == 0 as i32 as u64 {} else {
        __assert_fail(
            b"vc == 0\0" as *const u8 as *const i8,
            b"../mini-gmp.c\0" as *const u8 as *const i8,
            3981 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 73],
                &[i8; 73],
            >(
                b"void mpz_xor(__mpz_struct *, const __mpz_struct *, const __mpz_struct *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_26894: {
        if vc == 0 as i32 as u64 {} else {
            __assert_fail(
                b"vc == 0\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                3981 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 73],
                    &[i8; 73],
                >(
                    b"void mpz_xor(__mpz_struct *, const __mpz_struct *, const __mpz_struct *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    while i < un {
        ul = (*up.offset(i as isize) ^ ux).wrapping_add(uc);
        uc = (ul < uc) as i32 as mp_limb_t;
        rl = (ul ^ ux).wrapping_add(rc);
        rc = (rl < rc) as i32 as mp_limb_t;
        *rp.offset(i as isize) = rl;
        i += 1;
        i;
    }
    if rc != 0 {
        let fresh34 = un;
        un = un + 1;
        *rp.offset(fresh34 as isize) = rc;
    } else {
        un = mpn_normalized_size(rp as mp_srcptr, un);
    }
    (*r)._mp_size = (if rx != 0 { -un } else { un }) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn mpz_sizeinbase(
    mut u: *const __mpz_struct,
    mut base: i32,
) -> size_t {
    let mut un: mp_size_t = 0;
    let mut up: mp_srcptr = 0 as *const mp_limb_t;
    let mut tp: mp_ptr = 0 as *mut mp_limb_t;
    let mut bits: mp_bitcnt_t = 0;
    let mut bi: gmp_div_inverse = gmp_div_inverse {
        shift: 0,
        d1: 0,
        d0: 0,
        di: 0,
    };
    let mut ndigits: size_t = 0;
    if base >= 2 as i32 {} else {
        __assert_fail(
            b"base >= 2\0" as *const u8 as *const i8,
            b"../mini-gmp.c\0" as *const u8 as *const i8,
            4176 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 49],
                &[i8; 49],
            >(b"size_t mpz_sizeinbase(const __mpz_struct *, int)\0"))
                .as_ptr(),
        );
    }
    'c_14748: {
        if base >= 2 as i32 {} else {
            __assert_fail(
                b"base >= 2\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                4176 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 49],
                    &[i8; 49],
                >(b"size_t mpz_sizeinbase(const __mpz_struct *, int)\0"))
                    .as_ptr(),
            );
        }
    };
    if base <= 62 as i32 {} else {
        __assert_fail(
            b"base <= 62\0" as *const u8 as *const i8,
            b"../mini-gmp.c\0" as *const u8 as *const i8,
            4177 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 49],
                &[i8; 49],
            >(b"size_t mpz_sizeinbase(const __mpz_struct *, int)\0"))
                .as_ptr(),
        );
    }
    'c_14710: {
        if base <= 62 as i32 {} else {
            __assert_fail(
                b"base <= 62\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                4177 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 49],
                    &[i8; 49],
                >(b"size_t mpz_sizeinbase(const __mpz_struct *, int)\0"))
                    .as_ptr(),
            );
        }
    };
    un = (if (*u)._mp_size >= 0 as i32 { (*u)._mp_size } else { -(*u)._mp_size })
        as mp_size_t;
    if un == 0 as i32 as i64 {
        return 1 as i32 as size_t;
    }
    up = (*u)._mp_d as mp_srcptr;
    bits = ((un - 1 as i32 as i64) as u64)
        .wrapping_mul(
            (::core::mem::size_of::<mp_limb_t>() as u64).wrapping_mul(8 as i32 as u64),
        )
        .wrapping_add(
            mpn_limb_size_in_base_2(*up.offset((un - 1 as i32 as i64) as isize)),
        );
    match base {
        2 => return bits,
        4 => {
            return bits.wrapping_add(1 as i32 as u64).wrapping_div(2 as i32 as u64);
        }
        8 => {
            return bits.wrapping_add(2 as i32 as u64).wrapping_div(3 as i32 as u64);
        }
        16 => {
            return bits.wrapping_add(3 as i32 as u64).wrapping_div(4 as i32 as u64);
        }
        32 => {
            return bits.wrapping_add(4 as i32 as u64).wrapping_div(5 as i32 as u64);
        }
        _ => {}
    }
    tp = gmp_xalloc_limbs(un);
    mpn_copyi(tp, up, un);
    mpn_div_qr_1_invert(&mut bi, base as mp_limb_t);
    ndigits = 0 as i32 as size_t;
    loop {
        ndigits = ndigits.wrapping_add(1);
        ndigits;
        mpn_div_qr_1_preinv(tp, tp as mp_srcptr, un, &mut bi);
        un
            -= (*tp.offset((un - 1 as i32 as i64) as isize) == 0 as i32 as u64) as i32
                as i64;
        if !(un > 0 as i32 as i64) {
            break;
        }
    }
    (Some(gmp_free_func.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(tp as *mut libc::c_void, 0 as i32 as size_t);
    return ndigits;
}
#[no_mangle]
pub unsafe extern "C" fn mpz_scan1(
    mut u: *const __mpz_struct,
    mut starting_bit: mp_bitcnt_t,
) -> mp_bitcnt_t {
    let mut up: mp_ptr = 0 as *mut mp_limb_t;
    let mut us: mp_size_t = 0;
    let mut un: mp_size_t = 0;
    let mut i: mp_size_t = 0;
    let mut limb: mp_limb_t = 0;
    let mut ux: mp_limb_t = 0;
    us = (*u)._mp_size as mp_size_t;
    un = if us >= 0 as i32 as i64 { us } else { -us };
    i = starting_bit
        .wrapping_div(
            (::core::mem::size_of::<mp_limb_t>() as u64).wrapping_mul(8 as i32 as u64),
        ) as mp_size_t;
    if i >= un {
        return if us >= 0 as i32 as i64 {
            !(0 as i32 as mp_bitcnt_t)
        } else {
            starting_bit
        };
    }
    up = (*u)._mp_d;
    ux = 0 as i32 as mp_limb_t;
    limb = *up.offset(i as isize);
    if starting_bit != 0 as i32 as u64 {
        if us < 0 as i32 as i64 {
            ux = mpn_zero_p(up as mp_srcptr, i) as mp_limb_t;
            limb = (!limb).wrapping_add(ux);
            ux = ((limb >= ux) as i32 as mp_limb_t).wrapping_neg();
        }
        limb
            &= !(0 as i32 as mp_limb_t)
                << starting_bit
                    .wrapping_rem(
                        (::core::mem::size_of::<mp_limb_t>() as u64)
                            .wrapping_mul(8 as i32 as u64),
                    );
    }
    return mpn_common_scan(limb, i, up as mp_srcptr, un, ux);
}
unsafe extern "C" fn gmp_detect_endian() -> i32 {
    static mut i: i32 = 2 as i32;
    let mut p: *const u8 = &i as *const i32 as *const u8;
    return 1 as i32 - *p as i32;
}
#[no_mangle]
pub unsafe extern "C" fn mpz_scan0(
    mut u: *const __mpz_struct,
    mut starting_bit: mp_bitcnt_t,
) -> mp_bitcnt_t {
    let mut up: mp_ptr = 0 as *mut mp_limb_t;
    let mut us: mp_size_t = 0;
    let mut un: mp_size_t = 0;
    let mut i: mp_size_t = 0;
    let mut limb: mp_limb_t = 0;
    let mut ux: mp_limb_t = 0;
    us = (*u)._mp_size as mp_size_t;
    ux = ((us >= 0 as i32 as i64) as i32 as mp_limb_t).wrapping_neg();
    un = if us >= 0 as i32 as i64 { us } else { -us };
    i = starting_bit
        .wrapping_div(
            (::core::mem::size_of::<mp_limb_t>() as u64).wrapping_mul(8 as i32 as u64),
        ) as mp_size_t;
    if i >= un {
        return if ux != 0 { starting_bit } else { !(0 as i32 as mp_bitcnt_t) };
    }
    up = (*u)._mp_d;
    limb = *up.offset(i as isize) ^ ux;
    if ux == 0 as i32 as u64 {
        limb = (limb as u64).wrapping_sub(mpn_zero_p(up as mp_srcptr, i) as u64)
            as mp_limb_t as mp_limb_t;
    }
    limb
        &= !(0 as i32 as mp_limb_t)
            << starting_bit
                .wrapping_rem(
                    (::core::mem::size_of::<mp_limb_t>() as u64)
                        .wrapping_mul(8 as i32 as u64),
                );
    return mpn_common_scan(limb, i, up as mp_srcptr, un, ux);
}
#[no_mangle]
pub unsafe extern "C" fn mpn_popcount(
    mut p: mp_srcptr,
    mut n: mp_size_t,
) -> mp_bitcnt_t {
    let mut i: mp_size_t = 0;
    let mut c: mp_bitcnt_t = 0;
    c = 0 as i32 as mp_bitcnt_t;
    i = 0 as i32 as mp_size_t;
    while i < n {
        c = (c as u64).wrapping_add(gmp_popcount_limb(*p.offset(i as isize)) as u64)
            as mp_bitcnt_t as mp_bitcnt_t;
        i += 1;
        i;
    }
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn mpz_export(
    mut r: *mut libc::c_void,
    mut countp: *mut size_t,
    mut order: i32,
    mut size: size_t,
    mut endian: i32,
    mut nails: size_t,
    mut u: *const __mpz_struct,
) -> *mut libc::c_void {
    let mut count: size_t = 0;
    let mut un: mp_size_t = 0;
    if nails != 0 as i32 as u64 {
        gmp_die(b"mpz_import: Nails not supported.\0" as *const u8 as *const i8);
    }
    if order == 1 as i32 || order == -(1 as i32) {} else {
        __assert_fail(
            b"order == 1 || order == -1\0" as *const u8 as *const i8,
            b"../mini-gmp.c\0" as *const u8 as *const i8,
            4500 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 83],
                &[i8; 83],
            >(
                b"void *mpz_export(void *, size_t *, int, size_t, int, size_t, const __mpz_struct *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_30941: {
        if order == 1 as i32 || order == -(1 as i32) {} else {
            __assert_fail(
                b"order == 1 || order == -1\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                4500 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 83],
                    &[i8; 83],
                >(
                    b"void *mpz_export(void *, size_t *, int, size_t, int, size_t, const __mpz_struct *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if endian >= -(1 as i32) && endian <= 1 as i32 {} else {
        __assert_fail(
            b"endian >= -1 && endian <= 1\0" as *const u8 as *const i8,
            b"../mini-gmp.c\0" as *const u8 as *const i8,
            4501 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 83],
                &[i8; 83],
            >(
                b"void *mpz_export(void *, size_t *, int, size_t, int, size_t, const __mpz_struct *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_30893: {
        if endian >= -(1 as i32) && endian <= 1 as i32 {} else {
            __assert_fail(
                b"endian >= -1 && endian <= 1\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                4501 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 83],
                    &[i8; 83],
                >(
                    b"void *mpz_export(void *, size_t *, int, size_t, int, size_t, const __mpz_struct *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if size > 0 as i32 as u64 || (*u)._mp_size == 0 as i32 {} else {
        __assert_fail(
            b"size > 0 || u->_mp_size == 0\0" as *const u8 as *const i8,
            b"../mini-gmp.c\0" as *const u8 as *const i8,
            4502 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 83],
                &[i8; 83],
            >(
                b"void *mpz_export(void *, size_t *, int, size_t, int, size_t, const __mpz_struct *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_30841: {
        if size > 0 as i32 as u64 || (*u)._mp_size == 0 as i32 {} else {
            __assert_fail(
                b"size > 0 || u->_mp_size == 0\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                4502 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 83],
                    &[i8; 83],
                >(
                    b"void *mpz_export(void *, size_t *, int, size_t, int, size_t, const __mpz_struct *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    un = (*u)._mp_size as mp_size_t;
    count = 0 as i32 as size_t;
    if un != 0 as i32 as i64 {
        let mut k: size_t = 0;
        let mut p: *mut u8 = 0 as *mut u8;
        let mut word_step: ptrdiff_t = 0;
        let mut limb: mp_limb_t = 0;
        let mut bytes: size_t = 0;
        let mut i: mp_size_t = 0;
        un = if un >= 0 as i32 as i64 { un } else { -un };
        limb = *((*u)._mp_d).offset((un - 1 as i32 as i64) as isize);
        if limb != 0 as i32 as u64 {} else {
            __assert_fail(
                b"limb != 0\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                4522 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 83],
                    &[i8; 83],
                >(
                    b"void *mpz_export(void *, size_t *, int, size_t, int, size_t, const __mpz_struct *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_30758: {
            if limb != 0 as i32 as u64 {} else {
                __assert_fail(
                    b"limb != 0\0" as *const u8 as *const i8,
                    b"../mini-gmp.c\0" as *const u8 as *const i8,
                    4522 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 83],
                        &[i8; 83],
                    >(
                        b"void *mpz_export(void *, size_t *, int, size_t, int, size_t, const __mpz_struct *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        k = ((::core::mem::size_of::<mp_limb_t>() as u64).wrapping_mul(8 as i32 as u64)
            <= 8 as i32 as u64) as i32 as size_t;
        if k == 0 {
            loop {
                let mut LOCAL_CHAR_BIT: i32 = 8 as i32;
                k = k.wrapping_add(1);
                k;
                limb >>= LOCAL_CHAR_BIT;
                if !(limb != 0 as i32 as u64) {
                    break;
                }
            }
        }
        count = k
            .wrapping_add(
                ((un - 1 as i32 as i64) as u64)
                    .wrapping_mul(::core::mem::size_of::<mp_limb_t>() as u64),
            )
            .wrapping_add(size)
            .wrapping_sub(1 as i32 as u64)
            .wrapping_div(size);
        if r.is_null() {
            r = (Some(gmp_allocate_func.expect("non-null function pointer")))
                .expect("non-null function pointer")(count.wrapping_mul(size));
        }
        if endian == 0 as i32 {
            endian = gmp_detect_endian();
        }
        p = r as *mut u8;
        word_step = (if order != endian {
            (2 as i32 as u64).wrapping_mul(size)
        } else {
            0 as i32 as u64
        }) as ptrdiff_t;
        if order == 1 as i32 {
            p = p
                .offset(size.wrapping_mul(count.wrapping_sub(1 as i32 as u64)) as isize);
            word_step = -word_step;
        }
        if endian == 1 as i32 {
            p = p.offset(size.wrapping_sub(1 as i32 as u64) as isize);
        }
        bytes = 0 as i32 as size_t;
        i = 0 as i32 as mp_size_t;
        k = 0 as i32 as size_t;
        while k < count {
            let mut j: size_t = 0;
            j = 0 as i32 as size_t;
            while j < size {
                if ::core::mem::size_of::<mp_limb_t>() as u64 == 1 as i32 as u64 {
                    if i < un {
                        let fresh35 = i;
                        i = i + 1;
                        *p = *((*u)._mp_d).offset(fresh35 as isize) as u8;
                    } else {
                        *p = 0 as i32 as u8;
                    }
                } else {
                    let mut LOCAL_CHAR_BIT_0: i32 = 8 as i32;
                    if bytes == 0 as i32 as u64 {
                        if i < un {
                            let fresh36 = i;
                            i = i + 1;
                            limb = *((*u)._mp_d).offset(fresh36 as isize);
                        }
                        bytes = ::core::mem::size_of::<mp_limb_t>() as u64;
                    }
                    *p = limb as u8;
                    limb >>= LOCAL_CHAR_BIT_0;
                    bytes = bytes.wrapping_sub(1);
                    bytes;
                }
                j = j.wrapping_add(1);
                j;
                p = p.offset(-(endian as ptrdiff_t as isize));
            }
            k = k.wrapping_add(1);
            k;
            p = p.offset(word_step as isize);
        }
        if i == un {} else {
            __assert_fail(
                b"i == un\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                4585 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 83],
                    &[i8; 83],
                >(
                    b"void *mpz_export(void *, size_t *, int, size_t, int, size_t, const __mpz_struct *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_30438: {
            if i == un {} else {
                __assert_fail(
                    b"i == un\0" as *const u8 as *const i8,
                    b"../mini-gmp.c\0" as *const u8 as *const i8,
                    4585 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 83],
                        &[i8; 83],
                    >(
                        b"void *mpz_export(void *, size_t *, int, size_t, int, size_t, const __mpz_struct *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        if k == count {} else {
            __assert_fail(
                b"k == count\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                4586 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 83],
                    &[i8; 83],
                >(
                    b"void *mpz_export(void *, size_t *, int, size_t, int, size_t, const __mpz_struct *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_30399: {
            if k == count {} else {
                __assert_fail(
                    b"k == count\0" as *const u8 as *const i8,
                    b"../mini-gmp.c\0" as *const u8 as *const i8,
                    4586 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 83],
                        &[i8; 83],
                    >(
                        b"void *mpz_export(void *, size_t *, int, size_t, int, size_t, const __mpz_struct *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
    }
    if !countp.is_null() {
        *countp = count;
    }
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn mpz_init_set_str(
    mut r: *mut __mpz_struct,
    mut sp: *const i8,
    mut base: i32,
) -> i32 {
    mpz_init(r);
    return mpz_set_str(r, sp, base);
}
#[no_mangle]
pub unsafe extern "C" fn mpz_popcount(mut u: *const __mpz_struct) -> mp_bitcnt_t {
    let mut un: mp_size_t = 0;
    un = (*u)._mp_size as mp_size_t;
    if un < 0 as i32 as i64 {
        return !(0 as i32 as mp_bitcnt_t);
    }
    return mpn_popcount((*u)._mp_d as mp_srcptr, un);
}
#[no_mangle]
pub unsafe extern "C" fn mpz_hamdist(
    mut u: *const __mpz_struct,
    mut v: *const __mpz_struct,
) -> mp_bitcnt_t {
    let mut un: mp_size_t = 0;
    let mut vn: mp_size_t = 0;
    let mut i: mp_size_t = 0;
    let mut uc: mp_limb_t = 0;
    let mut vc: mp_limb_t = 0;
    let mut ul: mp_limb_t = 0;
    let mut vl: mp_limb_t = 0;
    let mut comp: mp_limb_t = 0;
    let mut up: mp_srcptr = 0 as *const mp_limb_t;
    let mut vp: mp_srcptr = 0 as *const mp_limb_t;
    let mut c: mp_bitcnt_t = 0;
    un = (*u)._mp_size as mp_size_t;
    vn = (*v)._mp_size as mp_size_t;
    if un ^ vn < 0 as i32 as i64 {
        return !(0 as i32 as mp_bitcnt_t);
    }
    vc = (un < 0 as i32 as i64) as i32 as mp_limb_t;
    uc = vc;
    comp = uc.wrapping_neg();
    if uc != 0 {
        if vn < 0 as i32 as i64 {} else {
            __assert_fail(
                b"vn < 0\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                4064 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 68],
                    &[i8; 68],
                >(
                    b"mp_bitcnt_t mpz_hamdist(const __mpz_struct *, const __mpz_struct *)\0",
                ))
                    .as_ptr(),
            );
        }
        'c_27538: {
            if vn < 0 as i32 as i64 {} else {
                __assert_fail(
                    b"vn < 0\0" as *const u8 as *const i8,
                    b"../mini-gmp.c\0" as *const u8 as *const i8,
                    4064 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 68],
                        &[i8; 68],
                    >(
                        b"mp_bitcnt_t mpz_hamdist(const __mpz_struct *, const __mpz_struct *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        un = -un;
        vn = -vn;
    }
    up = (*u)._mp_d as mp_srcptr;
    vp = (*v)._mp_d as mp_srcptr;
    if un < vn {
        let mut __mp_srcptr_swap__tmp: mp_srcptr = up;
        up = vp;
        vp = __mp_srcptr_swap__tmp;
        let mut __mp_size_t_swap__tmp: mp_size_t = un;
        un = vn;
        vn = __mp_size_t_swap__tmp;
    }
    i = 0 as i32 as mp_size_t;
    c = 0 as i32 as mp_bitcnt_t;
    while i < vn {
        ul = (*up.offset(i as isize) ^ comp).wrapping_add(uc);
        uc = (ul < uc) as i32 as mp_limb_t;
        vl = (*vp.offset(i as isize) ^ comp).wrapping_add(vc);
        vc = (vl < vc) as i32 as mp_limb_t;
        c = (c as u64).wrapping_add(gmp_popcount_limb(ul ^ vl) as u64) as mp_bitcnt_t
            as mp_bitcnt_t;
        i += 1;
        i;
    }
    if vc == 0 as i32 as u64 {} else {
        __assert_fail(
            b"vc == 0\0" as *const u8 as *const i8,
            b"../mini-gmp.c\0" as *const u8 as *const i8,
            4085 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 68],
                &[i8; 68],
            >(b"mp_bitcnt_t mpz_hamdist(const __mpz_struct *, const __mpz_struct *)\0"))
                .as_ptr(),
        );
    }
    'c_27336: {
        if vc == 0 as i32 as u64 {} else {
            __assert_fail(
                b"vc == 0\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                4085 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 68],
                    &[i8; 68],
                >(
                    b"mp_bitcnt_t mpz_hamdist(const __mpz_struct *, const __mpz_struct *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    while i < un {
        ul = (*up.offset(i as isize) ^ comp).wrapping_add(uc);
        uc = (ul < uc) as i32 as mp_limb_t;
        c = (c as u64).wrapping_add(gmp_popcount_limb(ul ^ comp) as u64) as mp_bitcnt_t
            as mp_bitcnt_t;
        i += 1;
        i;
    }
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn mpz_import(
    mut r: *mut __mpz_struct,
    mut count: size_t,
    mut order: i32,
    mut size: size_t,
    mut endian: i32,
    mut nails: size_t,
    mut src: *const libc::c_void,
) {
    let mut p: *const u8 = 0 as *const u8;
    let mut word_step: ptrdiff_t = 0;
    let mut rp: mp_ptr = 0 as *mut mp_limb_t;
    let mut rn: mp_size_t = 0;
    let mut limb: mp_limb_t = 0;
    let mut bytes: size_t = 0;
    let mut i: mp_size_t = 0;
    if nails != 0 as i32 as u64 {
        gmp_die(b"mpz_import: Nails not supported.\0" as *const u8 as *const i8);
    }
    if order == 1 as i32 || order == -(1 as i32) {} else {
        __assert_fail(
            b"order == 1 || order == -1\0" as *const u8 as *const i8,
            b"../mini-gmp.c\0" as *const u8 as *const i8,
            4442 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 80],
                &[i8; 80],
            >(
                b"void mpz_import(__mpz_struct *, size_t, int, size_t, int, size_t, const void *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_30283: {
        if order == 1 as i32 || order == -(1 as i32) {} else {
            __assert_fail(
                b"order == 1 || order == -1\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                4442 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 80],
                    &[i8; 80],
                >(
                    b"void mpz_import(__mpz_struct *, size_t, int, size_t, int, size_t, const void *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if endian >= -(1 as i32) && endian <= 1 as i32 {} else {
        __assert_fail(
            b"endian >= -1 && endian <= 1\0" as *const u8 as *const i8,
            b"../mini-gmp.c\0" as *const u8 as *const i8,
            4443 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 80],
                &[i8; 80],
            >(
                b"void mpz_import(__mpz_struct *, size_t, int, size_t, int, size_t, const void *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_30235: {
        if endian >= -(1 as i32) && endian <= 1 as i32 {} else {
            __assert_fail(
                b"endian >= -1 && endian <= 1\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                4443 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 80],
                    &[i8; 80],
                >(
                    b"void mpz_import(__mpz_struct *, size_t, int, size_t, int, size_t, const void *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if endian == 0 as i32 {
        endian = gmp_detect_endian();
    }
    p = src as *mut u8;
    word_step = (if order != endian {
        (2 as i32 as u64).wrapping_mul(size)
    } else {
        0 as i32 as u64
    }) as ptrdiff_t;
    if order == 1 as i32 {
        p = p.offset(size.wrapping_mul(count.wrapping_sub(1 as i32 as u64)) as isize);
        word_step = -word_step;
    }
    if endian == 1 as i32 {
        p = p.offset(size.wrapping_sub(1 as i32 as u64) as isize);
    }
    rn = size
        .wrapping_mul(count)
        .wrapping_add(::core::mem::size_of::<mp_limb_t>() as u64)
        .wrapping_sub(1 as i32 as u64)
        .wrapping_div(::core::mem::size_of::<mp_limb_t>() as u64) as mp_size_t;
    rp = if rn > (*r)._mp_alloc as i64 { mpz_realloc(r, rn) } else { (*r)._mp_d };
    limb = 0 as i32 as mp_limb_t;
    bytes = 0 as i32 as size_t;
    i = 0 as i32 as mp_size_t;
    while count > 0 as i32 as u64 {
        let mut j: size_t = 0;
        j = 0 as i32 as size_t;
        while j < size {
            let fresh37 = bytes;
            bytes = bytes.wrapping_add(1);
            limb |= (*p as mp_limb_t) << fresh37.wrapping_mul(8 as i32 as u64);
            if bytes == ::core::mem::size_of::<mp_limb_t>() as u64 {
                let fresh38 = i;
                i = i + 1;
                *rp.offset(fresh38 as isize) = limb;
                bytes = 0 as i32 as size_t;
                limb = 0 as i32 as mp_limb_t;
            }
            j = j.wrapping_add(1);
            j;
            p = p.offset(-(endian as ptrdiff_t as isize));
        }
        count = count.wrapping_sub(1);
        count;
        p = p.offset(word_step as isize);
    }
    if i + (bytes > 0 as i32 as u64) as i32 as i64 == rn {} else {
        __assert_fail(
            b"i + (bytes > 0) == rn\0" as *const u8 as *const i8,
            b"../mini-gmp.c\0" as *const u8 as *const i8,
            4481 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 80],
                &[i8; 80],
            >(
                b"void mpz_import(__mpz_struct *, size_t, int, size_t, int, size_t, const void *)\0",
            ))
                .as_ptr(),
        );
    }
    'c_29955: {
        if i + (bytes > 0 as i32 as u64) as i32 as i64 == rn {} else {
            __assert_fail(
                b"i + (bytes > 0) == rn\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                4481 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 80],
                    &[i8; 80],
                >(
                    b"void mpz_import(__mpz_struct *, size_t, int, size_t, int, size_t, const void *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if limb != 0 as i32 as u64 {
        let fresh39 = i;
        i = i + 1;
        *rp.offset(fresh39 as isize) = limb;
    } else {
        i = mpn_normalized_size(rp as mp_srcptr, i);
    }
    (*r)._mp_size = i as i32;
}
#[no_mangle]
pub unsafe extern "C" fn mpz_out_str(
    mut stream: *mut FILE,
    mut base: i32,
    mut x: *const __mpz_struct,
) -> size_t {
    let mut str: *mut i8 = 0 as *mut i8;
    let mut len: size_t = 0;
    str = mpz_get_str(0 as *mut i8, base, x);
    len = strlen(str);
    len = fwrite(str as *const libc::c_void, 1 as i32 as size_t, len, stream);
    (Some(gmp_free_func.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(str as *mut libc::c_void, 0 as i32 as size_t);
    return len;
}
unsafe extern "C" fn gmp_popcount_limb(mut x: mp_limb_t) -> u32 {
    let mut c: u32 = 0;
    let mut LOCAL_SHIFT_BITS: i32 = 16 as i32;
    c = 0 as i32 as u32;
    while x > 0 as i32 as u64 {
        let mut w: u32 = x.wrapping_sub(x >> 1 as i32 & 0x5555 as i32 as u64) as u32;
        w = (w >> 2 as i32 & 0x3333 as i32 as u32)
            .wrapping_add(w & 0x3333 as i32 as u32);
        w = (w >> 4 as i32).wrapping_add(w);
        w = (w >> 8 as i32 & 0xf as i32 as u32).wrapping_add(w & 0xf as i32 as u32);
        c = c.wrapping_add(w);
        if (::core::mem::size_of::<mp_limb_t>() as u64).wrapping_mul(8 as i32 as u64)
            > LOCAL_SHIFT_BITS as u64
        {
            x >>= LOCAL_SHIFT_BITS;
        } else {
            x = 0 as i32 as mp_limb_t;
        }
    }
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn mpz_get_str(
    mut sp: *mut i8,
    mut base: i32,
    mut u: *const __mpz_struct,
) -> *mut i8 {
    let mut bits: u32 = 0;
    let mut digits: *const i8 = 0 as *const i8;
    let mut un: mp_size_t = 0;
    let mut i: size_t = 0;
    let mut sn: size_t = 0;
    digits = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz\0"
        as *const u8 as *const i8;
    if base > 1 as i32 {
        if base <= 36 as i32 {
            digits = b"0123456789abcdefghijklmnopqrstuvwxyz\0" as *const u8 as *const i8;
        } else if base > 62 as i32 {
            return 0 as *mut i8
        }
    } else if base >= -(1 as i32) {
        base = 10 as i32;
    } else {
        base = -base;
        if base > 36 as i32 {
            return 0 as *mut i8;
        }
    }
    sn = (1 as i32 as u64).wrapping_add(mpz_sizeinbase(u, base));
    if sp.is_null() {
        sp = (Some(gmp_allocate_func.expect("non-null function pointer")))
            .expect("non-null function pointer")((1 as i32 as u64).wrapping_add(sn))
            as *mut i8;
    }
    un = (if (*u)._mp_size >= 0 as i32 { (*u)._mp_size } else { -(*u)._mp_size })
        as mp_size_t;
    if un == 0 as i32 as i64 {
        *sp.offset(0 as i32 as isize) = '0' as i32 as i8;
        *sp.offset(1 as i32 as isize) = '\0' as i32 as i8;
        return sp;
    }
    i = 0 as i32 as size_t;
    if (*u)._mp_size < 0 as i32 {
        let fresh40 = i;
        i = i.wrapping_add(1);
        *sp.offset(fresh40 as isize) = '-' as i32 as i8;
    }
    bits = mpn_base_power_of_two_p(base as u32);
    if bits != 0 {
        sn = i
            .wrapping_add(
                mpn_get_str_bits(
                    (sp as *mut u8).offset(i as isize),
                    bits,
                    (*u)._mp_d as mp_srcptr,
                    un,
                ),
            );
    } else {
        let mut info: mpn_base_info = mpn_base_info { exp: 0, bb: 0 };
        let mut tp: mp_ptr = 0 as *mut mp_limb_t;
        mpn_get_base_info(&mut info, base as mp_limb_t);
        tp = gmp_xalloc_limbs(un);
        mpn_copyi(tp, (*u)._mp_d as mp_srcptr, un);
        sn = i
            .wrapping_add(
                mpn_get_str_other(
                    (sp as *mut u8).offset(i as isize),
                    base,
                    &mut info,
                    tp,
                    un,
                ),
            );
        (Some(gmp_free_func.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(tp as *mut libc::c_void, 0 as i32 as size_t);
    }
    while i < sn {
        *sp.offset(i as isize) = *digits.offset(*sp.offset(i as isize) as u8 as isize);
        i = i.wrapping_add(1);
        i;
    }
    *sp.offset(sn as isize) = '\0' as i32 as i8;
    return sp;
}
#[no_mangle]
pub unsafe extern "C" fn mpz_set_str(
    mut r: *mut __mpz_struct,
    mut sp: *const i8,
    mut base: i32,
) -> i32 {
    let mut bits: u32 = 0;
    let mut value_of_a: u32 = 0;
    let mut rn: mp_size_t = 0;
    let mut alloc: mp_size_t = 0;
    let mut rp: mp_ptr = 0 as *mut mp_limb_t;
    let mut dn: size_t = 0;
    let mut sign: i32 = 0;
    let mut dp: *mut u8 = 0 as *mut u8;
    if base == 0 as i32 || base >= 2 as i32 && base <= 62 as i32 {} else {
        __assert_fail(
            b"base == 0 || (base >= 2 && base <= 62)\0" as *const u8 as *const i8,
            b"../mini-gmp.c\0" as *const u8 as *const i8,
            4297 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 51],
                &[i8; 51],
            >(b"int mpz_set_str(__mpz_struct *, const char *, int)\0"))
                .as_ptr(),
        );
    }
    'c_29742: {
        if base == 0 as i32 || base >= 2 as i32 && base <= 62 as i32 {} else {
            __assert_fail(
                b"base == 0 || (base >= 2 && base <= 62)\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                4297 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 51],
                    &[i8; 51],
                >(b"int mpz_set_str(__mpz_struct *, const char *, int)\0"))
                    .as_ptr(),
            );
        }
    };
    while *(*__ctype_b_loc()).offset(*sp as u8 as i32 as isize) as i32
        & _ISspace as i32 as libc::c_ushort as i32 != 0
    {
        sp = sp.offset(1);
        sp;
    }
    sign = (*sp as i32 == '-' as i32) as i32;
    sp = sp.offset(sign as isize);
    if base == 0 as i32 {
        if *sp.offset(0 as i32 as isize) as i32 == '0' as i32 {
            if *sp.offset(1 as i32 as isize) as i32 == 'x' as i32
                || *sp.offset(1 as i32 as isize) as i32 == 'X' as i32
            {
                base = 16 as i32;
                sp = sp.offset(2 as i32 as isize);
            } else if *sp.offset(1 as i32 as isize) as i32 == 'b' as i32
                || *sp.offset(1 as i32 as isize) as i32 == 'B' as i32
            {
                base = 2 as i32;
                sp = sp.offset(2 as i32 as isize);
            } else {
                base = 8 as i32;
            }
        } else {
            base = 10 as i32;
        }
    }
    if *sp == 0 {
        (*r)._mp_size = 0 as i32;
        return -(1 as i32);
    }
    dp = (Some(gmp_allocate_func.expect("non-null function pointer")))
        .expect("non-null function pointer")(strlen(sp)) as *mut u8;
    value_of_a = (if base > 36 as i32 { 36 as i32 } else { 10 as i32 }) as u32;
    dn = 0 as i32 as size_t;
    while *sp != 0 {
        let mut digit: u32 = 0;
        if !(*(*__ctype_b_loc()).offset(*sp as u8 as i32 as isize) as i32
            & _ISspace as i32 as libc::c_ushort as i32 != 0)
        {
            if *sp as i32 >= '0' as i32 && *sp as i32 <= '9' as i32 {
                digit = (*sp as i32 - '0' as i32) as u32;
            } else if *sp as i32 >= 'a' as i32 && *sp as i32 <= 'z' as i32 {
                digit = ((*sp as i32 - 'a' as i32) as u32).wrapping_add(value_of_a);
            } else if *sp as i32 >= 'A' as i32 && *sp as i32 <= 'Z' as i32 {
                digit = (*sp as i32 - 'A' as i32 + 10 as i32) as u32;
            } else {
                digit = base as u32;
            }
            if digit >= base as u32 {
                (Some(gmp_free_func.expect("non-null function pointer")))
                    .expect(
                        "non-null function pointer",
                    )(dp as *mut libc::c_void, 0 as i32 as size_t);
                (*r)._mp_size = 0 as i32;
                return -(1 as i32);
            }
            let fresh41 = dn;
            dn = dn.wrapping_add(1);
            *dp.offset(fresh41 as isize) = digit as u8;
        }
        sp = sp.offset(1);
        sp;
    }
    if dn == 0 {
        (Some(gmp_free_func.expect("non-null function pointer")))
            .expect(
                "non-null function pointer",
            )(dp as *mut libc::c_void, 0 as i32 as size_t);
        (*r)._mp_size = 0 as i32;
        return -(1 as i32);
    }
    bits = mpn_base_power_of_two_p(base as u32);
    if bits > 0 as i32 as u32 {
        alloc = dn
            .wrapping_mul(bits as u64)
            .wrapping_add(
                (::core::mem::size_of::<mp_limb_t>() as u64)
                    .wrapping_mul(8 as i32 as u64),
            )
            .wrapping_sub(1 as i32 as u64)
            .wrapping_div(
                (::core::mem::size_of::<mp_limb_t>() as u64)
                    .wrapping_mul(8 as i32 as u64),
            ) as mp_size_t;
        rp = if alloc > (*r)._mp_alloc as i64 {
            mpz_realloc(r, alloc)
        } else {
            (*r)._mp_d
        };
        rn = mpn_set_str_bits(rp, dp, dn, bits);
    } else {
        let mut info: mpn_base_info = mpn_base_info { exp: 0, bb: 0 };
        mpn_get_base_info(&mut info, base as mp_limb_t);
        alloc = dn
            .wrapping_add(info.exp as u64)
            .wrapping_sub(1 as i32 as u64)
            .wrapping_div(info.exp as u64) as mp_size_t;
        rp = if alloc > (*r)._mp_alloc as i64 {
            mpz_realloc(r, alloc)
        } else {
            (*r)._mp_d
        };
        rn = mpn_set_str_other(rp, dp, dn, base as mp_limb_t, &mut info);
        if rn > 0 as i32 as i64 {} else {
            __assert_fail(
                b"rn > 0\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                4381 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 51],
                    &[i8; 51],
                >(b"int mpz_set_str(__mpz_struct *, const char *, int)\0"))
                    .as_ptr(),
            );
        }
        'c_29185: {
            if rn > 0 as i32 as i64 {} else {
                __assert_fail(
                    b"rn > 0\0" as *const u8 as *const i8,
                    b"../mini-gmp.c\0" as *const u8 as *const i8,
                    4381 as i32 as u32,
                    (*::core::mem::transmute::<
                        &[u8; 51],
                        &[i8; 51],
                    >(b"int mpz_set_str(__mpz_struct *, const char *, int)\0"))
                        .as_ptr(),
                );
            }
        };
        rn
            -= (*rp.offset((rn - 1 as i32 as i64) as isize) == 0 as i32 as u64) as i32
                as i64;
    }
    if rn <= alloc {} else {
        __assert_fail(
            b"rn <= alloc\0" as *const u8 as *const i8,
            b"../mini-gmp.c\0" as *const u8 as *const i8,
            4384 as i32 as u32,
            (*::core::mem::transmute::<
                &[u8; 51],
                &[i8; 51],
            >(b"int mpz_set_str(__mpz_struct *, const char *, int)\0"))
                .as_ptr(),
        );
    }
    'c_29120: {
        if rn <= alloc {} else {
            __assert_fail(
                b"rn <= alloc\0" as *const u8 as *const i8,
                b"../mini-gmp.c\0" as *const u8 as *const i8,
                4384 as i32 as u32,
                (*::core::mem::transmute::<
                    &[u8; 51],
                    &[i8; 51],
                >(b"int mpz_set_str(__mpz_struct *, const char *, int)\0"))
                    .as_ptr(),
            );
        }
    };
    (Some(gmp_free_func.expect("non-null function pointer")))
        .expect(
            "non-null function pointer",
        )(dp as *mut libc::c_void, 0 as i32 as size_t);
    (*r)._mp_size = (if sign != 0 { -rn } else { rn }) as i32;
    return 0 as i32;
}
pub fn main() {
    let mut args: Vec<*mut i8> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0((args.len() - 1) as i32, args.as_mut_ptr() as *mut *mut i8) as i32,
        )
    }
}
unsafe extern "C" fn run_static_initializers() {
    mp_bits_per_limb = (::core::mem::size_of::<mp_limb_t>() as u64)
        .wrapping_mul(8 as i32 as u64) as i32;
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];