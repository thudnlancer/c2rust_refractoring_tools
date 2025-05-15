use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
extern "C" {
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn strcpy(_: *mut i8, _: *const i8) -> *mut i8;
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strlen(_: *const i8) -> u64;
    fn quote(arg: *const i8) -> *const i8;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn dcgettext(
        __domainname: *const i8,
        __msgid: *const i8,
        __category: i32,
    ) -> *mut i8;
    fn error(__status: i32, __errnum: i32, __format: *const i8, _: ...);
}
pub type size_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub _gl_dummy: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tagRegexTypeMap {
    pub name: *const i8,
    pub context: i32,
    pub option_val: i32,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_0 {
    CONTEXT_GENERIC = 2,
    CONTEXT_ALL = 3,
    CONTEXT_FINDUTILS = 1,
}
impl C2RustUnnamed_0 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_0::CONTEXT_GENERIC => 2,
            C2RustUnnamed_0::CONTEXT_ALL => 3,
            C2RustUnnamed_0::CONTEXT_FINDUTILS => 1,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_0 {
        match value {
            2 => C2RustUnnamed_0::CONTEXT_GENERIC,
            3 => C2RustUnnamed_0::CONTEXT_ALL,
            1 => C2RustUnnamed_0::CONTEXT_FINDUTILS,
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
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed_1 {
    N_REGEX_MAP_ENTRIES = 13,
}
impl C2RustUnnamed_1 {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed_1::N_REGEX_MAP_ENTRIES => 13,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed_1 {
        match value {
            13 => C2RustUnnamed_1::N_REGEX_MAP_ENTRIES,
            _ => panic!("Invalid value for C2RustUnnamed_1: {}", value),
        }
    }
}
impl AddAssign<u32> for C2RustUnnamed_1 {
    fn add_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_1::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for C2RustUnnamed_1 {
    fn sub_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_1::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for C2RustUnnamed_1 {
    fn mul_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_1::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for C2RustUnnamed_1 {
    fn div_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_1::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for C2RustUnnamed_1 {
    fn rem_assign(&mut self, rhs: u32) {
        *self = C2RustUnnamed_1::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for C2RustUnnamed_1 {
    type Output = C2RustUnnamed_1;
    fn add(self, rhs: u32) -> C2RustUnnamed_1 {
        C2RustUnnamed_1::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for C2RustUnnamed_1 {
    type Output = C2RustUnnamed_1;
    fn sub(self, rhs: u32) -> C2RustUnnamed_1 {
        C2RustUnnamed_1::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for C2RustUnnamed_1 {
    type Output = C2RustUnnamed_1;
    fn mul(self, rhs: u32) -> C2RustUnnamed_1 {
        C2RustUnnamed_1::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for C2RustUnnamed_1 {
    type Output = C2RustUnnamed_1;
    fn div(self, rhs: u32) -> C2RustUnnamed_1 {
        C2RustUnnamed_1::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for C2RustUnnamed_1 {
    type Output = C2RustUnnamed_1;
    fn rem(self, rhs: u32) -> C2RustUnnamed_1 {
        C2RustUnnamed_1::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
static mut regex_map: [tagRegexTypeMap; 13] = [
    {
        let mut init = tagRegexTypeMap {
            name: b"findutils-default\0" as *const u8 as *const i8,
            context: C2RustUnnamed_0::CONTEXT_FINDUTILS as i32,
            option_val: (0 as i32 as u64
                | ((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) as i32,
        };
        init
    },
    {
        let mut init = tagRegexTypeMap {
            name: b"ed\0" as *const u8 as *const i8,
            context: C2RustUnnamed_0::CONTEXT_GENERIC as i32,
            option_val: (((1 as i32 as u64) << 1 as i32) << 1 as i32
                | ((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32
                | (((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32
                | (((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32
                | ((((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32 | (1 as i32 as u64) << 1 as i32
                | ((((((((((((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) as i32,
        };
        init
    },
    {
        let mut init = tagRegexTypeMap {
            name: b"emacs\0" as *const u8 as *const i8,
            context: C2RustUnnamed_0::CONTEXT_ALL as i32,
            option_val: 0 as i32,
        };
        init
    },
    {
        let mut init = tagRegexTypeMap {
            name: b"gnu-awk\0" as *const u8 as *const i8,
            context: C2RustUnnamed_0::CONTEXT_ALL as i32,
            option_val: ((((1 as i32 as u64) << 1 as i32) << 1 as i32
                | ((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32
                | (((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32
                | (((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32
                | ((((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32
                | (((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32
                | ((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32
                | ((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32
                | (((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32
                | (((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32
                | (((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32
                | (((((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32
                | 1 as i32 as u64
                | (((((((((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                & !((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32
                    | ((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32
                    | (((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32)) as i32,
        };
        init
    },
    {
        let mut init = tagRegexTypeMap {
            name: b"grep\0" as *const u8 as *const i8,
            context: C2RustUnnamed_0::CONTEXT_ALL as i32,
            option_val: ((((1 as i32 as u64) << 1 as i32) << 1 as i32
                | ((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32
                | (((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32
                | (((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32
                | ((((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32 | (1 as i32 as u64) << 1 as i32
                | ((((((((((((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32
                | (((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32)
                & !(((((((((((((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32
                    | (((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)) as i32,
        };
        init
    },
    {
        let mut init = tagRegexTypeMap {
            name: b"posix-awk\0" as *const u8 as *const i8,
            context: C2RustUnnamed_0::CONTEXT_ALL as i32,
            option_val: (((1 as i32 as u64) << 1 as i32) << 1 as i32
                | ((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32
                | (((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32
                | (((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32
                | ((((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32
                | (((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32
                | ((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32
                | ((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32
                | (((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32
                | (((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32
                | (((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32
                | (((((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32
                | 1 as i32 as u64
                | (((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32
                | (((((((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32
                | (((((((((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) as i32,
        };
        init
    },
    {
        let mut init = tagRegexTypeMap {
            name: b"awk\0" as *const u8 as *const i8,
            context: C2RustUnnamed_0::CONTEXT_ALL as i32,
            option_val: (1 as i32 as u64
                | (((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32
                | (((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32
                | ((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32
                | (((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32
                | ((((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32
                | ((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32
                | (((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32
                | ((1 as i32 as u64) << 1 as i32) << 1 as i32
                | (((((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32
                | (((((((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) as i32,
        };
        init
    },
    {
        let mut init = tagRegexTypeMap {
            name: b"posix-basic\0" as *const u8 as *const i8,
            context: C2RustUnnamed_0::CONTEXT_ALL as i32,
            option_val: (((1 as i32 as u64) << 1 as i32) << 1 as i32
                | ((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32
                | (((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32
                | (((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32
                | ((((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32 | (1 as i32 as u64) << 1 as i32
                | ((((((((((((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) as i32,
        };
        init
    },
    {
        let mut init = tagRegexTypeMap {
            name: b"posix-egrep\0" as *const u8 as *const i8,
            context: C2RustUnnamed_0::CONTEXT_ALL as i32,
            option_val: ((((1 as i32 as u64) << 1 as i32) << 1 as i32
                | ((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32
                | (((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32
                | (((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32
                | ((((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32
                | (((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32
                | ((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32
                | ((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32
                | (((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32
                | (((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32
                | (((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32
                | (((((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32
                | (((((((((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32
                | (((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32)
                & !((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32
                    | (((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)) as i32,
        };
        init
    },
    {
        let mut init = tagRegexTypeMap {
            name: b"egrep\0" as *const u8 as *const i8,
            context: C2RustUnnamed_0::CONTEXT_ALL as i32,
            option_val: ((((1 as i32 as u64) << 1 as i32) << 1 as i32
                | ((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32
                | (((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32
                | (((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32
                | ((((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32
                | (((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32
                | ((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32
                | ((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32
                | (((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32
                | (((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32
                | (((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32
                | (((((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32
                | (((((((((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32
                | (((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32)
                & !((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32
                    | (((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                        << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)) as i32,
        };
        init
    },
    {
        let mut init = tagRegexTypeMap {
            name: b"posix-extended\0" as *const u8 as *const i8,
            context: C2RustUnnamed_0::CONTEXT_ALL as i32,
            option_val: (((1 as i32 as u64) << 1 as i32) << 1 as i32
                | ((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32
                | (((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32
                | (((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32
                | ((((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32
                | (((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32
                | ((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32
                | ((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32
                | (((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32
                | (((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32
                | (((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32
                | (((((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                as i32,
        };
        init
    },
    {
        let mut init = tagRegexTypeMap {
            name: b"posix-minimal-basic\0" as *const u8 as *const i8,
            context: C2RustUnnamed_0::CONTEXT_GENERIC as i32,
            option_val: (((1 as i32 as u64) << 1 as i32) << 1 as i32
                | ((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32
                | (((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32
                | (((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32
                | ((((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32
                | ((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) as i32,
        };
        init
    },
    {
        let mut init = tagRegexTypeMap {
            name: b"sed\0" as *const u8 as *const i8,
            context: C2RustUnnamed_0::CONTEXT_GENERIC as i32,
            option_val: (((1 as i32 as u64) << 1 as i32) << 1 as i32
                | ((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32
                | (((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32
                | (((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32
                | ((((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32 | (1 as i32 as u64) << 1 as i32
                | ((((((((((((((((((((((((1 as i32 as u64) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32) << 1 as i32)
                    << 1 as i32) << 1 as i32) as i32,
        };
        init
    },
];
#[no_mangle]
pub unsafe extern "C" fn get_regex_type(mut s: *const i8) -> i32 {
    let mut i: u32 = 0;
    let mut msglen: size_t = 0;
    let mut buf: *mut i8 = 0 as *mut i8;
    let mut p: *mut i8 = 0 as *mut i8;
    msglen = 0 as u32 as size_t;
    i = 0 as u32;
    while i < C2RustUnnamed_1::N_REGEX_MAP_ENTRIES as i32 as u32 {
        if 0 as i32 == strcmp(regex_map[i as usize].name, s) {
            return regex_map[i as usize].option_val
        } else {
            msglen = (msglen as u64)
                .wrapping_add(
                    (strlen(quote(regex_map[i as usize].name)))
                        .wrapping_add(2 as u32 as u64),
                ) as size_t as size_t;
        }
        i = i.wrapping_add(1);
        i;
    }
    buf = xmalloc((1 as u32 as u64).wrapping_add(msglen)) as *mut i8;
    p = buf;
    i = 0 as u32;
    while i < C2RustUnnamed_1::N_REGEX_MAP_ENTRIES as i32 as u32 {
        if i > 0 as u32 {
            strcpy(p, b", \0" as *const u8 as *const i8);
            p = p.offset(2 as i32 as isize);
        }
        p = p
            .offset(
                sprintf(
                    p,
                    b"%s\0" as *const u8 as *const i8,
                    quote(regex_map[i as usize].name),
                ) as isize,
            );
        i = i.wrapping_add(1);
        i;
    }
    if ::core::mem::size_of::<C2RustUnnamed>() as u64 != 0 {
        error(
            1 as i32,
            0 as i32,
            dcgettext(
                0 as *const i8,
                b"Unknown regular expression type %s; valid types are %s.\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            quote(s),
            buf,
        );
        if 0 as i32 != 0 {} else {
            unreachable!();
        };
    } else {
        error(
            1 as i32,
            0 as i32,
            dcgettext(
                0 as *const i8,
                b"Unknown regular expression type %s; valid types are %s.\0" as *const u8
                    as *const i8,
                5 as i32,
            ),
            quote(s),
            buf,
        );
        if 0 as i32 != 0 {} else {
            unreachable!();
        };
    };
    return -(1 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn get_regex_type_name(mut ix: u32) -> *const i8 {
    if ix < C2RustUnnamed_1::N_REGEX_MAP_ENTRIES as i32 as u32 {
        return regex_map[ix as usize].name
    } else {
        return 0 as *const i8
    };
}
#[no_mangle]
pub unsafe extern "C" fn get_regex_type_flags(mut ix: u32) -> i32 {
    if ix < C2RustUnnamed_1::N_REGEX_MAP_ENTRIES as i32 as u32 {
        return regex_map[ix as usize].option_val
    } else {
        return -(1 as i32)
    };
}
#[no_mangle]
pub unsafe extern "C" fn get_regex_type_context(mut ix: u32) -> u32 {
    if ix < C2RustUnnamed_1::N_REGEX_MAP_ENTRIES as i32 as u32 {
        return regex_map[ix as usize].context as u32
    } else {
        return 0 as u32
    };
}
#[no_mangle]
pub unsafe extern "C" fn get_regex_type_synonym(mut ix: u32, mut context: u32) -> i32 {
    let mut i: u32 = 0;
    let mut flags: i32 = 0;
    if ix >= C2RustUnnamed_1::N_REGEX_MAP_ENTRIES as i32 as u32 {
        return -(1 as i32);
    }
    flags = regex_map[ix as usize].option_val;
    i = 0 as u32;
    while i < ix {
        if !(regex_map[i as usize].context as u32 & context == 0 as i32 as u32) {
            if flags == regex_map[i as usize].option_val {
                return i as i32;
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    return -(1 as i32);
}