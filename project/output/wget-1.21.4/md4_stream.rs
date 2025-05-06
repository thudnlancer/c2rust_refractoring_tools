#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn __uflow(_: *mut _IO_FILE) -> i32;
    fn fread_unlocked(
        __ptr: *mut libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn md4_finish_ctx(ctx: *mut md4_ctx, resbuf: *mut libc::c_void) -> *mut libc::c_void;
    fn md4_process_bytes(buffer: *const libc::c_void, len: size_t, ctx: *mut md4_ctx);
    fn md4_process_block(buffer: *const libc::c_void, len: size_t, ctx: *mut md4_ctx);
    fn md4_init_ctx(ctx: *mut md4_ctx);
    fn malloc(_: u64) -> *mut libc::c_void;
    fn rpl_free(ptr: *mut libc::c_void);
}
pub type size_t = u64;
pub type __uint32_t = u32;
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
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct md4_ctx {
    pub A: uint32_t,
    pub B: uint32_t,
    pub C: uint32_t,
    pub D: uint32_t,
    pub total: [uint32_t; 2],
    pub buflen: uint32_t,
    pub buffer: [uint32_t; 32],
}
#[inline]
unsafe extern "C" fn feof_unlocked(mut __stream: *mut FILE) -> i32 {
    return ((*__stream)._flags & 0x10 as i32 != 0 as i32) as i32;
}
#[inline]
unsafe extern "C" fn ferror_unlocked(mut __stream: *mut FILE) -> i32 {
    return ((*__stream)._flags & 0x20 as i32 != 0 as i32) as i32;
}
#[no_mangle]
pub unsafe extern "C" fn md4_stream(
    mut stream: *mut FILE,
    mut resblock: *mut libc::c_void,
) -> i32 {
    let mut ctx: md4_ctx = md4_ctx {
        A: 0,
        B: 0,
        C: 0,
        D: 0,
        total: [0; 2],
        buflen: 0,
        buffer: [0; 32],
    };
    let mut sum: size_t = 0;
    let mut buffer: *mut i8 = malloc((32768 as i32 + 72 as i32) as u64) as *mut i8;
    if buffer.is_null() {
        return 1 as i32;
    }
    md4_init_ctx(&mut ctx);
    's_20: loop {
        let mut n: size_t = 0;
        sum = 0 as i32 as size_t;
        loop {
            n = if 0 != 0 && 0 != 0
                && (1 as i32 as size_t)
                    .wrapping_mul((32768 as i32 as u64).wrapping_sub(sum))
                    <= 8 as i32 as u64 && 1 as i32 as size_t != 0 as i32 as u64
            {
                ({
                    let mut __ptr: *mut i8 = buffer.offset(sum as isize);
                    let mut __stream: *mut FILE = stream;
                    let mut __cnt: size_t = 0;
                    __cnt = (1 as i32 as size_t)
                        .wrapping_mul((32768 as i32 as u64).wrapping_sub(sum));
                    while __cnt > 0 as i32 as u64 {
                        let mut __c: i32 = if ((*__stream)._IO_read_ptr
                            >= (*__stream)._IO_read_end) as i32 as i64 != 0
                        {
                            __uflow(__stream)
                        } else {
                            let fresh0 = (*__stream)._IO_read_ptr;
                            (*__stream)._IO_read_ptr = ((*__stream)._IO_read_ptr)
                                .offset(1);
                            *(fresh0 as *mut u8) as i32
                        };
                        if __c == -(1 as i32) {
                            break;
                        }
                        let fresh1 = __ptr;
                        __ptr = __ptr.offset(1);
                        *fresh1 = __c as i8;
                        __cnt = __cnt.wrapping_sub(1);
                        __cnt;
                    }
                    (1 as i32 as size_t)
                        .wrapping_mul((32768 as i32 as u64).wrapping_sub(sum))
                        .wrapping_sub(__cnt)
                        .wrapping_div(1 as i32 as size_t)
                })
            } else if 0 != 0 && 1 as i32 as size_t == 0 as i32 as u64
                || 0 != 0 && (32768 as i32 as u64).wrapping_sub(sum) == 0 as i32 as u64
            {
                0 as i32 as size_t
            } else {
                fread_unlocked(
                    buffer.offset(sum as isize) as *mut libc::c_void,
                    1 as i32 as size_t,
                    (32768 as i32 as u64).wrapping_sub(sum),
                    stream,
                )
            };
            sum = (sum as u64).wrapping_add(n) as size_t as size_t;
            if sum == 32768 as i32 as u64 {
                break;
            }
            if n == 0 as i32 as u64 {
                if ferror_unlocked(stream) != 0 {
                    rpl_free(buffer as *mut libc::c_void);
                    return 1 as i32;
                }
                break 's_20;
            } else if feof_unlocked(stream) != 0 {
                break 's_20;
            }
        }
        md4_process_block(
            buffer as *const libc::c_void,
            32768 as i32 as size_t,
            &mut ctx,
        );
    }
    if sum > 0 as i32 as u64 {
        md4_process_bytes(buffer as *const libc::c_void, sum, &mut ctx);
    }
    md4_finish_ctx(&mut ctx, resblock);
    rpl_free(buffer as *mut libc::c_void);
    return 0 as i32;
}