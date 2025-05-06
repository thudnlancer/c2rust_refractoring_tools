#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(c_variadic)]
extern "C" {
    fn malloc(_: u64) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: u64) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn abort() -> !;
    fn __errno_location() -> *mut i32;
    fn strerror(_: i32) -> *mut i8;
    fn nettle_yarrow256_seed(
        ctx: *mut yarrow256_ctx,
        length: size_t,
        seed_file: *const uint8_t,
    );
    static mut stderr: *mut _IO_FILE;
    fn fclose(__stream: *mut FILE) -> i32;
    fn fopen(__filename: *const i8, __modes: *const i8) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const i8, _: ...) -> i32;
    fn vfprintf(_: *mut FILE, _: *const i8, _: ::core::ffi::VaList) -> i32;
    fn fread(
        __ptr: *mut libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn fwrite(
        __ptr: *const libc::c_void,
        __size: size_t,
        __n: size_t,
        __s: *mut FILE,
    ) -> size_t;
    fn feof(__stream: *mut FILE) -> i32;
    fn ferror(__stream: *mut FILE) -> i32;
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
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
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
pub struct aes256_ctx {
    pub keys: [uint32_t; 60],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sha256_ctx {
    pub state: [uint32_t; 8],
    pub count: uint64_t,
    pub index: u32,
    pub block: [uint8_t; 64],
}
pub type yarrow_pool_id = u32;
pub const YARROW_SLOW: yarrow_pool_id = 1;
pub const YARROW_FAST: yarrow_pool_id = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yarrow_source {
    pub estimate: [uint32_t; 2],
    pub next: yarrow_pool_id,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yarrow256_ctx {
    pub pools: [sha256_ctx; 2],
    pub seeded: i32,
    pub key: aes256_ctx,
    pub counter: [uint8_t; 16],
    pub nsources: u32,
    pub sources: *mut yarrow_source,
}
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
#[no_mangle]
pub static mut quiet_flag: i32 = 0 as i32;
#[no_mangle]
pub unsafe extern "C" fn xalloc(mut size: size_t) -> *mut libc::c_void {
    let mut p: *mut libc::c_void = malloc(size);
    if p.is_null() {
        fprintf(stderr, b"Virtual memory exhausted.\n\0" as *const u8 as *const i8);
        abort();
    }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn werror(mut format: *const i8, mut args: ...) {
    if quiet_flag == 0 {
        let mut args_0: ::core::ffi::VaListImpl;
        args_0 = args.clone();
        vfprintf(stderr, format, args_0.as_va_list());
    }
}
#[no_mangle]
pub unsafe extern "C" fn read_file(
    mut name: *const i8,
    mut max_size: size_t,
    mut contents: *mut *mut uint8_t,
) -> size_t {
    let mut size: size_t = 0;
    let mut done: size_t = 0;
    let mut buffer: *mut uint8_t = 0 as *mut uint8_t;
    let mut f: *mut FILE = 0 as *mut FILE;
    f = fopen(name, b"rb\0" as *const u8 as *const i8);
    if f.is_null() {
        werror(
            b"Opening `%s' failed: %s\n\0" as *const u8 as *const i8,
            name,
            strerror(*__errno_location()),
        );
        return 0 as i32 as size_t;
    }
    size = 100 as i32 as size_t;
    buffer = 0 as *mut uint8_t;
    done = 0 as i32 as size_t;
    loop {
        let mut p: *mut uint8_t = 0 as *mut uint8_t;
        if max_size != 0 && size > max_size {
            size = max_size;
        }
        p = realloc(buffer as *mut libc::c_void, size.wrapping_add(1 as i32 as u64))
            as *mut uint8_t;
        if !p.is_null() {
            buffer = p;
            done = (done as u64)
                .wrapping_add(
                    fread(
                        buffer.offset(done as isize) as *mut libc::c_void,
                        1 as i32 as size_t,
                        size.wrapping_sub(done),
                        f,
                    ),
                ) as size_t as size_t;
            if done < size {
                if ferror(f) != 0 {
                    fprintf(
                        stderr,
                        b"Reading `%s' failed: %s\n\0" as *const u8 as *const i8,
                        name,
                        strerror(*__errno_location()),
                    );
                } else if !(done == 0 as i32 as u64) {
                    break;
                }
            } else {
                if size == max_size {
                    break;
                }
                size = (size as u64).wrapping_mul(2 as i32 as u64) as size_t as size_t;
                continue;
            }
        }
        fclose(f);
        free(buffer as *mut libc::c_void);
        *contents = 0 as *mut uint8_t;
        return 0 as i32 as size_t;
    }
    fclose(f);
    *buffer.offset(done as isize) = '\0' as i32 as uint8_t;
    *contents = buffer;
    return done;
}
#[no_mangle]
pub unsafe extern "C" fn write_data(
    mut f: *mut FILE,
    mut size: size_t,
    mut buffer: *const libc::c_void,
) -> i32 {
    let mut res: size_t = fwrite(buffer, 1 as i32 as size_t, size, f);
    return (res == size) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn write_file(
    mut name: *const i8,
    mut size: size_t,
    mut buffer: *const libc::c_void,
) -> i32 {
    let mut f: *mut FILE = fopen(name, b"wb\0" as *const u8 as *const i8);
    let mut res: i32 = 0;
    if f.is_null() {
        return 0 as i32;
    }
    res = write_data(f, size, buffer);
    return (fclose(f) == 0 as i32 && res != 0) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn simple_random(
    mut ctx: *mut yarrow256_ctx,
    mut name: *const i8,
) -> i32 {
    let mut length: u32 = 0;
    let mut buffer: *mut uint8_t = 0 as *mut uint8_t;
    if !name.is_null() {
        length = read_file(name, 0 as i32 as size_t, &mut buffer) as u32;
    } else {
        length = read_file(
            b"/dev/urandom\0" as *const u8 as *const i8,
            20 as i32 as size_t,
            &mut buffer,
        ) as u32;
    }
    if length == 0 {
        return 0 as i32;
    }
    nettle_yarrow256_seed(ctx, length as size_t, buffer);
    free(buffer as *mut libc::c_void);
    return 1 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn hash_file(
    mut hash: *const nettle_hash,
    mut ctx: *mut libc::c_void,
    mut f: *mut FILE,
) -> i32 {
    loop {
        let mut buffer: [uint8_t; 1000] = [0; 1000];
        let mut res: size_t = fread(
            buffer.as_mut_ptr() as *mut libc::c_void,
            1 as i32 as size_t,
            ::core::mem::size_of::<[uint8_t; 1000]>() as u64,
            f,
        );
        if ferror(f) != 0 {
            return 0 as i32;
        }
        ((*hash).update)
            .expect("non-null function pointer")(ctx, res, buffer.as_mut_ptr());
        if feof(f) != 0 {
            return 1 as i32;
        }
    };
}