use ::libc;
extern "C" {
    fn __errno_location() -> *mut libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    static stdlib_allocator: allocator;
}
pub type __ssize_t = libc::c_long;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct allocator {
    pub allocate: Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
    pub reallocate: Option::<
        unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void,
    >,
    pub free: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub die: Option::<unsafe extern "C" fn(size_t) -> ()>,
}
pub const STACK_BUF_SIZE: C2RustUnnamed = 1024;
pub type C2RustUnnamed = libc::c_uint;
unsafe extern "C" fn readlink_stk(
    mut fd: libc::c_int,
    mut filename: *const libc::c_char,
    mut buffer: *mut libc::c_char,
    mut buffer_size: size_t,
    mut alloc: *const allocator,
    mut preadlinkat: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            *const libc::c_char,
            *mut libc::c_char,
            size_t,
        ) -> ssize_t,
    >,
    mut stack_buf: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf_size: size_t = 0;
    let mut buf_size_max: size_t = if (9223372036854775807 as libc::c_long
        as libc::c_ulong) < -(1 as libc::c_int) as size_t
    {
        (9223372036854775807 as libc::c_long as size_t)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
    } else {
        -(1 as libc::c_int) as size_t
    };
    if alloc.is_null() {
        alloc = &stdlib_allocator;
    }
    if buffer.is_null() {
        buffer = stack_buf;
        buffer_size = STACK_BUF_SIZE as libc::c_int as size_t;
    }
    buf = buffer;
    buf_size = buffer_size;
    while !buf.is_null() {
        let mut link_length: ssize_t = preadlinkat
            .expect("non-null function pointer")(fd, filename, buf, buf_size);
        let mut link_size: size_t = 0;
        if link_length < 0 as libc::c_int as libc::c_long {
            if buf != buffer {
                let mut readlinkat_errno: libc::c_int = *__errno_location();
                ((*alloc).free)
                    .expect("non-null function pointer")(buf as *mut libc::c_void);
                *__errno_location() = readlinkat_errno;
            }
            return 0 as *mut libc::c_char;
        }
        link_size = link_length as size_t;
        if link_size < buf_size {
            let fresh0 = link_size;
            link_size = link_size.wrapping_add(1);
            *buf.offset(fresh0 as isize) = '\0' as i32 as libc::c_char;
            if buf == stack_buf {
                let mut b: *mut libc::c_char = ((*alloc).allocate)
                    .expect("non-null function pointer")(link_size) as *mut libc::c_char;
                buf_size = link_size;
                if b.is_null() {
                    break;
                }
                return memcpy(
                    b as *mut libc::c_void,
                    buf as *const libc::c_void,
                    link_size,
                ) as *mut libc::c_char;
            } else {
                if link_size < buf_size && buf != buffer
                    && ((*alloc).reallocate).is_some()
                {
                    let mut b_0: *mut libc::c_char = ((*alloc).reallocate)
                        .expect(
                            "non-null function pointer",
                        )(buf as *mut libc::c_void, link_size) as *mut libc::c_char;
                    if !b_0.is_null() {
                        return b_0;
                    }
                }
                return buf;
            }
        } else {
            if buf != buffer {
                ((*alloc).free)
                    .expect("non-null function pointer")(buf as *mut libc::c_void);
            }
            if buf_size < buf_size_max.wrapping_div(2 as libc::c_int as libc::c_ulong) {
                buf_size = (2 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(buf_size)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong);
            } else if buf_size < buf_size_max {
                buf_size = buf_size_max;
            } else {
                if !(buf_size_max < -(1 as libc::c_int) as size_t) {
                    break;
                }
                *__errno_location() = 36 as libc::c_int;
                return 0 as *mut libc::c_char;
            }
            buf = ((*alloc).allocate).expect("non-null function pointer")(buf_size)
                as *mut libc::c_char;
        }
    }
    if ((*alloc).die).is_some() {
        ((*alloc).die).expect("non-null function pointer")(buf_size);
    }
    *__errno_location() = 12 as libc::c_int;
    return 0 as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn careadlinkat(
    mut fd: libc::c_int,
    mut filename: *const libc::c_char,
    mut buffer: *mut libc::c_char,
    mut buffer_size: size_t,
    mut alloc: *const allocator,
    mut preadlinkat: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            *const libc::c_char,
            *mut libc::c_char,
            size_t,
        ) -> ssize_t,
    >,
) -> *mut libc::c_char {
    let mut stack_buf: [libc::c_char; 1024] = [0; 1024];
    return readlink_stk(
        fd,
        filename,
        buffer,
        buffer_size,
        alloc,
        preadlinkat,
        stack_buf.as_mut_ptr(),
    );
}
