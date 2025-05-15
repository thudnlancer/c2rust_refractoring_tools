use ::libc;
extern "C" {
    fn sha384_finish_ctx(
        ctx: *mut sha512_ctx,
        resbuf: *mut libc::c_void,
    ) -> *mut libc::c_void;
    fn sha512_finish_ctx(
        ctx: *mut sha512_ctx,
        resbuf: *mut libc::c_void,
    ) -> *mut libc::c_void;
    fn sha512_process_bytes(
        buffer: *const libc::c_void,
        len: size_t,
        ctx: *mut sha512_ctx,
    );
    fn sha512_process_block(
        buffer: *const libc::c_void,
        len: size_t,
        ctx: *mut sha512_ctx,
    );
    fn sha384_init_ctx(ctx: *mut sha512_ctx);
    fn sha512_init_ctx(ctx: *mut sha512_ctx);
    fn __uflow(_: *mut _IO_FILE) -> libc::c_int;
    fn fread_unlocked(
        __ptr: *mut libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn rpl_free(ptr: *mut libc::c_void);
}
pub type size_t = libc::c_ulong;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub __pad1: *mut libc::c_void,
    pub __pad2: *mut libc::c_void,
    pub __pad3: *mut libc::c_void,
    pub __pad4: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
pub type FILE = _IO_FILE;
pub type ssize_t = __ssize_t;
pub type uint64_t = __uint64_t;
pub type u64_0 = uint64_t;
pub type C2RustUnnamed = libc::c_uint;
pub const SHA384_DIGEST_SIZE: C2RustUnnamed = 48;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const SHA512_DIGEST_SIZE: C2RustUnnamed_0 = 64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sha512_ctx {
    pub state: [u64_0; 8],
    pub total: [u64_0; 2],
    pub buflen: size_t,
    pub buffer: [u64_0; 32],
}
#[inline]
unsafe extern "C" fn feof_unlocked(mut __stream: *mut FILE) -> libc::c_int {
    return ((*__stream)._flags & 0x10 as libc::c_int != 0 as libc::c_int) as libc::c_int;
}
#[inline]
unsafe extern "C" fn ferror_unlocked(mut __stream: *mut FILE) -> libc::c_int {
    return ((*__stream)._flags & 0x20 as libc::c_int != 0 as libc::c_int) as libc::c_int;
}
#[inline]
unsafe extern "C" fn afalg_stream(
    mut stream: *mut FILE,
    mut alg: *const libc::c_char,
    mut resblock: *mut libc::c_void,
    mut hashlen: ssize_t,
) -> libc::c_int {
    return -(97 as libc::c_int);
}
unsafe extern "C" fn shaxxx_stream(
    mut stream: *mut FILE,
    mut alg: *const libc::c_char,
    mut resblock: *mut libc::c_void,
    mut hashlen: ssize_t,
    mut init_ctx: Option::<unsafe extern "C" fn(*mut sha512_ctx) -> ()>,
    mut finish_ctx: Option::<
        unsafe extern "C" fn(*mut sha512_ctx, *mut libc::c_void) -> *mut libc::c_void,
    >,
) -> libc::c_int {
    match afalg_stream(stream, alg, resblock, hashlen) {
        0 => return 0 as libc::c_int,
        -5 => return 1 as libc::c_int,
        _ => {}
    }
    let mut buffer: *mut libc::c_char = malloc(
        (32768 as libc::c_int + 72 as libc::c_int) as libc::c_ulong,
    ) as *mut libc::c_char;
    if buffer.is_null() {
        return 1 as libc::c_int;
    }
    let mut ctx: sha512_ctx = sha512_ctx {
        state: [0; 8],
        total: [0; 2],
        buflen: 0,
        buffer: [0; 32],
    };
    init_ctx.expect("non-null function pointer")(&mut ctx);
    let mut sum: size_t = 0;
    's_34: loop {
        let mut n: size_t = 0;
        sum = 0 as libc::c_int as size_t;
        loop {
            if feof_unlocked(stream) != 0 {
                break 's_34;
            }
            n = if 0 != 0 && 0 != 0
                && (1 as libc::c_int as size_t)
                    .wrapping_mul(
                        (32768 as libc::c_int as libc::c_ulong).wrapping_sub(sum),
                    ) <= 8 as libc::c_int as libc::c_ulong
                && 1 as libc::c_int as size_t != 0 as libc::c_int as libc::c_ulong
            {
                ({
                    let mut __ptr: *mut libc::c_char = buffer.offset(sum as isize);
                    let mut __stream: *mut FILE = stream;
                    let mut __cnt: size_t = 0;
                    __cnt = (1 as libc::c_int as size_t)
                        .wrapping_mul(
                            (32768 as libc::c_int as libc::c_ulong).wrapping_sub(sum),
                        );
                    while __cnt > 0 as libc::c_int as libc::c_ulong {
                        let mut __c: libc::c_int = if ((*__stream)._IO_read_ptr
                            >= (*__stream)._IO_read_end) as libc::c_int as libc::c_long
                            != 0
                        {
                            __uflow(__stream)
                        } else {
                            let fresh0 = (*__stream)._IO_read_ptr;
                            (*__stream)
                                ._IO_read_ptr = ((*__stream)._IO_read_ptr).offset(1);
                            *(fresh0 as *mut libc::c_uchar) as libc::c_int
                        };
                        if __c == -(1 as libc::c_int) {
                            break;
                        }
                        let fresh1 = __ptr;
                        __ptr = __ptr.offset(1);
                        *fresh1 = __c as libc::c_char;
                        __cnt = __cnt.wrapping_sub(1);
                        __cnt;
                    }
                    (1 as libc::c_int as size_t)
                        .wrapping_mul(
                            (32768 as libc::c_int as libc::c_ulong).wrapping_sub(sum),
                        )
                        .wrapping_sub(__cnt)
                        .wrapping_div(1 as libc::c_int as size_t)
                })
            } else if 0 != 0
                && 1 as libc::c_int as size_t == 0 as libc::c_int as libc::c_ulong
                || 0 != 0
                    && (32768 as libc::c_int as libc::c_ulong).wrapping_sub(sum)
                        == 0 as libc::c_int as libc::c_ulong
            {
                0 as libc::c_int as size_t
            } else {
                fread_unlocked(
                    buffer.offset(sum as isize) as *mut libc::c_void,
                    1 as libc::c_int as size_t,
                    (32768 as libc::c_int as libc::c_ulong).wrapping_sub(sum),
                    stream,
                )
            };
            sum = (sum as libc::c_ulong).wrapping_add(n) as size_t as size_t;
            if sum == 32768 as libc::c_int as libc::c_ulong {
                break;
            }
            if !(n == 0 as libc::c_int as libc::c_ulong) {
                continue;
            }
            if ferror_unlocked(stream) != 0 {
                rpl_free(buffer as *mut libc::c_void);
                return 1 as libc::c_int;
            }
            break 's_34;
        }
        sha512_process_block(
            buffer as *const libc::c_void,
            32768 as libc::c_int as size_t,
            &mut ctx,
        );
    }
    if sum > 0 as libc::c_int as libc::c_ulong {
        sha512_process_bytes(buffer as *const libc::c_void, sum, &mut ctx);
    }
    finish_ctx.expect("non-null function pointer")(&mut ctx, resblock);
    rpl_free(buffer as *mut libc::c_void);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn sha512_stream(
    mut stream: *mut FILE,
    mut resblock: *mut libc::c_void,
) -> libc::c_int {
    return shaxxx_stream(
        stream,
        b"sha512\0" as *const u8 as *const libc::c_char,
        resblock,
        SHA512_DIGEST_SIZE as libc::c_int as ssize_t,
        Some(sha512_init_ctx as unsafe extern "C" fn(*mut sha512_ctx) -> ()),
        Some(
            sha512_finish_ctx
                as unsafe extern "C" fn(
                    *mut sha512_ctx,
                    *mut libc::c_void,
                ) -> *mut libc::c_void,
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn sha384_stream(
    mut stream: *mut FILE,
    mut resblock: *mut libc::c_void,
) -> libc::c_int {
    return shaxxx_stream(
        stream,
        b"sha384\0" as *const u8 as *const libc::c_char,
        resblock,
        SHA384_DIGEST_SIZE as libc::c_int as ssize_t,
        Some(sha384_init_ctx as unsafe extern "C" fn(*mut sha512_ctx) -> ()),
        Some(
            sha384_finish_ctx
                as unsafe extern "C" fn(
                    *mut sha512_ctx,
                    *mut libc::c_void,
                ) -> *mut libc::c_void,
        ),
    );
}
