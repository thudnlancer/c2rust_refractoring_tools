use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
extern "C" {
    fn __uflow(_: *mut _IO_FILE) -> i32;
    fn fread_unlocked(
        __ptr: *mut libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn sha224_finish_ctx(
        ctx: *mut sha256_ctx,
        resbuf: *mut libc::c_void,
    ) -> *mut libc::c_void;
    fn sha256_finish_ctx(
        ctx: *mut sha256_ctx,
        resbuf: *mut libc::c_void,
    ) -> *mut libc::c_void;
    fn sha256_process_bytes(
        buffer: *const libc::c_void,
        len: size_t,
        ctx: *mut sha256_ctx,
    );
    fn sha256_process_block(
        buffer: *const libc::c_void,
        len: size_t,
        ctx: *mut sha256_ctx,
    );
    fn sha224_init_ctx(ctx: *mut sha256_ctx);
    fn sha256_init_ctx(ctx: *mut sha256_ctx);
    fn malloc(_: u64) -> *mut libc::c_void;
    fn rpl_free(ptr: *mut libc::c_void);
}
pub type size_t = u64;
pub type __uint32_t = u32;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type __ssize_t = i64;
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
pub type ssize_t = __ssize_t;
pub type uint32_t = __uint32_t;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed {
    SHA224_DIGEST_SIZE = 28,
}
impl C2RustUnnamed {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed::SHA224_DIGEST_SIZE => 28,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed {
        match value {
            28 => C2RustUnnamed::SHA224_DIGEST_SIZE,
            _ => panic!("Invalid value for C2RustUnnamed: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn add(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn sub(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn mul(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn div(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed {
    type Output = C2RustUnnamed;
    fn rem(self, rhs: u32) -> C2RustUnnamed {
        C2RustUnnamed::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_0 {
    SHA256_DIGEST_SIZE = 32,
}
impl C2RustUnnamed_0 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_0::SHA256_DIGEST_SIZE => 32,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_0 {
        match value {
            32 => C2RustUnnamed_0::SHA256_DIGEST_SIZE,
            _ => panic!("Invalid value for C2RustUnnamed_0: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed_0 {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed_0 {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed_0 {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed_0 {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed_0 {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed_0 {
    type Output = C2RustUnnamed_0;
    fn add(self, rhs: u32) -> C2RustUnnamed_0 {
        C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed_0 {
    type Output = C2RustUnnamed_0;
    fn sub(self, rhs: u32) -> C2RustUnnamed_0 {
        C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed_0 {
    type Output = C2RustUnnamed_0;
    fn mul(self, rhs: u32) -> C2RustUnnamed_0 {
        C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed_0 {
    type Output = C2RustUnnamed_0;
    fn div(self, rhs: u32) -> C2RustUnnamed_0 {
        C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed_0 {
    type Output = C2RustUnnamed_0;
    fn rem(self, rhs: u32) -> C2RustUnnamed_0 {
        C2RustUnnamed_0::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sha256_ctx {
    pub state: [uint32_t; 8],
    pub total: [uint32_t; 2],
    pub buflen: size_t,
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
#[inline]
unsafe extern "C" fn afalg_stream(
    mut stream: *mut FILE,
    mut alg: *const i8,
    mut resblock: *mut libc::c_void,
    mut hashlen: ssize_t,
) -> i32 {
    return -(97 as i32);
}
unsafe extern "C" fn shaxxx_stream(
    mut stream: *mut FILE,
    mut alg: *const i8,
    mut resblock: *mut libc::c_void,
    mut hashlen: ssize_t,
    mut init_ctx: Option<unsafe extern "C" fn(*mut sha256_ctx) -> ()>,
    mut finish_ctx: Option<
        unsafe extern "C" fn(*mut sha256_ctx, *mut libc::c_void) -> *mut libc::c_void,
    >,
) -> i32 {
    match afalg_stream(stream, alg, resblock, hashlen) {
        0 => return 0 as i32,
        -5 => return 1 as i32,
        _ => {}
    }
    let mut buffer: *mut i8 = malloc((32768 as i32 + 72 as i32) as u64) as *mut i8;
    if buffer.is_null() {
        return 1 as i32;
    }
    let mut ctx: sha256_ctx = sha256_ctx {
        state: [0; 8],
        total: [0; 2],
        buflen: 0,
        buffer: [0; 32],
    };
    init_ctx.expect("non-null function pointer")(&mut ctx);
    let mut sum: size_t = 0;
    's_34: loop {
        let mut n: size_t = 0;
        sum = 0 as i32 as size_t;
        loop {
            if feof_unlocked(stream) != 0 {
                break 's_34;
            }
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
            if !(n == 0 as i32 as u64) {
                continue;
            }
            if ferror_unlocked(stream) != 0 {
                rpl_free(buffer as *mut libc::c_void);
                return 1 as i32;
            }
            break 's_34;
        }
        sha256_process_block(
            buffer as *const libc::c_void,
            32768 as i32 as size_t,
            &mut ctx,
        );
    }
    if sum > 0 as i32 as u64 {
        sha256_process_bytes(buffer as *const libc::c_void, sum, &mut ctx);
    }
    finish_ctx.expect("non-null function pointer")(&mut ctx, resblock);
    rpl_free(buffer as *mut libc::c_void);
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn sha256_stream(
    mut stream: *mut FILE,
    mut resblock: *mut libc::c_void,
) -> i32 {
    return shaxxx_stream(
        stream,
        b"sha256\0" as *const u8 as *const i8,
        resblock,
        C2RustUnnamed_0::SHA256_DIGEST_SIZE as i32 as ssize_t,
        Some(sha256_init_ctx as unsafe extern "C" fn(*mut sha256_ctx) -> ()),
        Some(
            sha256_finish_ctx
                as unsafe extern "C" fn(
                    *mut sha256_ctx,
                    *mut libc::c_void,
                ) -> *mut libc::c_void,
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn sha224_stream(
    mut stream: *mut FILE,
    mut resblock: *mut libc::c_void,
) -> i32 {
    return shaxxx_stream(
        stream,
        b"sha224\0" as *const u8 as *const i8,
        resblock,
        C2RustUnnamed::SHA224_DIGEST_SIZE as i32 as ssize_t,
        Some(sha224_init_ctx as unsafe extern "C" fn(*mut sha256_ctx) -> ()),
        Some(
            sha224_finish_ctx
                as unsafe extern "C" fn(
                    *mut sha256_ctx,
                    *mut libc::c_void,
                ) -> *mut libc::c_void,
        ),
    );
}