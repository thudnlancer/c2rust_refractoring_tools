use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
extern "C" {
    fn __errno_location() -> *mut i32;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    static stdlib_allocator: allocator;
}
pub type __ssize_t = i64;
pub type ssize_t = __ssize_t;
pub type size_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct allocator {
    pub allocate: Option<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
    pub reallocate: Option<
        unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void,
    >,
    pub free: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub die: Option<unsafe extern "C" fn(size_t) -> ()>,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed {
    STACK_BUF_SIZE = 1024,
}
impl C2RustUnnamed {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed::STACK_BUF_SIZE => 1024,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed {
        match value {
            1024 => C2RustUnnamed::STACK_BUF_SIZE,
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
unsafe extern "C" fn readlink_stk(
    mut fd: i32,
    mut filename: *const i8,
    mut buffer: *mut i8,
    mut buffer_size: size_t,
    mut alloc: *const allocator,
    mut preadlinkat: Option<
        unsafe extern "C" fn(i32, *const i8, *mut i8, size_t) -> ssize_t,
    >,
    mut stack_buf: *mut i8,
) -> *mut i8 {
    let mut buf: *mut i8 = 0 as *mut i8;
    let mut buf_size: size_t = 0;
    let mut buf_size_max: size_t = if (9223372036854775807 as i64 as u64)
        < -(1 as i32) as size_t
    {
        (9223372036854775807 as i64 as size_t).wrapping_add(1 as i32 as u64)
    } else {
        -(1 as i32) as size_t
    };
    if alloc.is_null() {
        alloc = &stdlib_allocator;
    }
    if buffer.is_null() {
        buffer = stack_buf;
        buffer_size = C2RustUnnamed::STACK_BUF_SIZE as i32 as size_t;
    }
    buf = buffer;
    buf_size = buffer_size;
    while !buf.is_null() {
        let mut link_length: ssize_t = preadlinkat
            .expect("non-null function pointer")(fd, filename, buf, buf_size);
        let mut link_size: size_t = 0;
        if link_length < 0 as i32 as i64 {
            if buf != buffer {
                let mut readlinkat_errno: i32 = *__errno_location();
                ((*alloc).free)
                    .expect("non-null function pointer")(buf as *mut libc::c_void);
                *__errno_location() = readlinkat_errno;
            }
            return 0 as *mut i8;
        }
        link_size = link_length as size_t;
        if link_size < buf_size {
            let fresh0 = link_size;
            link_size = link_size.wrapping_add(1);
            *buf.offset(fresh0 as isize) = '\0' as i32 as i8;
            if buf == stack_buf {
                let mut b: *mut i8 = ((*alloc).allocate)
                    .expect("non-null function pointer")(link_size) as *mut i8;
                buf_size = link_size;
                if b.is_null() {
                    break;
                }
                return memcpy(
                    b as *mut libc::c_void,
                    buf as *const libc::c_void,
                    link_size,
                ) as *mut i8;
            } else {
                if link_size < buf_size && buf != buffer
                    && ((*alloc).reallocate).is_some()
                {
                    let mut b_0: *mut i8 = ((*alloc).reallocate)
                        .expect(
                            "non-null function pointer",
                        )(buf as *mut libc::c_void, link_size) as *mut i8;
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
            if buf_size < buf_size_max.wrapping_div(2 as i32 as u64) {
                buf_size = (2 as i32 as u64)
                    .wrapping_mul(buf_size)
                    .wrapping_add(1 as i32 as u64);
            } else if buf_size < buf_size_max {
                buf_size = buf_size_max;
            } else {
                if !(buf_size_max < -(1 as i32) as size_t) {
                    break;
                }
                *__errno_location() = 36 as i32;
                return 0 as *mut i8;
            }
            buf = ((*alloc).allocate).expect("non-null function pointer")(buf_size)
                as *mut i8;
        }
    }
    if ((*alloc).die).is_some() {
        ((*alloc).die).expect("non-null function pointer")(buf_size);
    }
    *__errno_location() = 12 as i32;
    return 0 as *mut i8;
}
#[no_mangle]
pub unsafe extern "C" fn careadlinkat(
    mut fd: i32,
    mut filename: *const i8,
    mut buffer: *mut i8,
    mut buffer_size: size_t,
    mut alloc: *const allocator,
    mut preadlinkat: Option<
        unsafe extern "C" fn(i32, *const i8, *mut i8, size_t) -> ssize_t,
    >,
) -> *mut i8 {
    let mut stack_buf: [i8; 1024] = [0; 1024];
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