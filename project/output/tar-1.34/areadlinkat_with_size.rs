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
    fn __errno_location() -> *mut i32;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: u64) -> *mut libc::c_void;
    fn rpl_free(ptr: *mut libc::c_void);
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn readlinkat(
        __fd: i32,
        __path: *const i8,
        __buf: *mut i8,
        __len: size_t,
    ) -> ssize_t;
}
pub type size_t = u64;
pub const stackbuf_size: C2RustUnnamed = 128;
pub type ssize_t = __ssize_t;
pub type __ssize_t = i64;
pub type C2RustUnnamed = u32;
#[no_mangle]
pub unsafe extern "C" fn areadlinkat_with_size(
    mut fd: i32,
    mut file: *const i8,
    mut size: size_t,
) -> *mut i8 {
    let mut symlink_max: size_t = 1024 as i32 as size_t;
    let mut INITIAL_LIMIT_BOUND: size_t = (8 as i32 * 1024 as i32) as size_t;
    let mut initial_limit: size_t = if symlink_max < INITIAL_LIMIT_BOUND {
        symlink_max.wrapping_add(1 as i32 as u64)
    } else {
        INITIAL_LIMIT_BOUND
    };
    let mut buf_size: size_t = if size == 0 as i32 as u64 {
        stackbuf_size as i32 as u64
    } else if size < initial_limit {
        size.wrapping_add(1 as i32 as u64)
    } else {
        initial_limit
    };
    loop {
        let mut r: ssize_t = 0;
        let mut link_length: size_t = 0;
        let mut stackbuf: [i8; 128] = [0; 128];
        let mut buf: *mut i8 = stackbuf.as_mut_ptr();
        let mut buffer: *mut i8 = 0 as *mut i8;
        if !(size == 0 as i32 as u64 && buf_size == stackbuf_size as i32 as u64) {
            buffer = malloc(buf_size) as *mut i8;
            buf = buffer;
            if buffer.is_null() {
                return 0 as *mut i8;
            }
        }
        r = readlinkat(fd, file, buf, buf_size);
        link_length = r as size_t;
        if r < 0 as i32 as i64 {
            let mut saved_errno: i32 = *__errno_location();
            rpl_free(buffer as *mut libc::c_void);
            *__errno_location() = saved_errno;
            return 0 as *mut i8;
        }
        if link_length < buf_size {
            *buf.offset(link_length as isize) = 0 as i32 as i8;
            if buffer.is_null() {
                buffer = malloc(link_length.wrapping_add(1 as i32 as u64)) as *mut i8;
                if !buffer.is_null() {
                    return memcpy(
                        buffer as *mut libc::c_void,
                        buf as *const libc::c_void,
                        link_length.wrapping_add(1 as i32 as u64),
                    ) as *mut i8;
                }
            } else if link_length.wrapping_add(1 as i32 as u64) < buf_size {
                let mut shrinked_buffer: *mut i8 = realloc(
                    buffer as *mut libc::c_void,
                    link_length.wrapping_add(1 as i32 as u64),
                ) as *mut i8;
                if !shrinked_buffer.is_null() {
                    buffer = shrinked_buffer;
                }
            }
            return buffer;
        }
        rpl_free(buffer as *mut libc::c_void);
        if buf_size
            <= (if (18446744073709551615 as u64) < 9223372036854775807 as i64 as u64 {
                18446744073709551615 as u64
            } else {
                9223372036854775807 as i64 as u64
            })
                .wrapping_div(2 as i32 as u64)
        {
            buf_size = (buf_size as u64).wrapping_mul(2 as i32 as u64) as size_t
                as size_t;
        } else if buf_size
            < (if (18446744073709551615 as u64) < 9223372036854775807 as i64 as u64 {
                18446744073709551615 as u64
            } else {
                9223372036854775807 as i64 as u64
            })
        {
            buf_size = if (18446744073709551615 as u64)
                < 9223372036854775807 as i64 as u64
            {
                18446744073709551615 as u64
            } else {
                9223372036854775807 as i64 as u64
            };
        } else {
            *__errno_location() = 12 as i32;
            return 0 as *mut i8;
        }
    };
}